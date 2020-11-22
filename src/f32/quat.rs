#[cfg(feature = "num-traits")]
use num_traits::Float;

use super::{scalar_acos, scalar_sin_cos, Mat3, Mat4, Vec3, Vec3A, Vec4, Vec4Swizzles};
use crate::vector_traits::*;

#[cfg(all(
    target_arch = "x86",
    target_feature = "sse2",
    not(feature = "scalar-math")
))]
use core::arch::x86::*;
#[cfg(all(
    target_arch = "x86_64",
    target_feature = "sse2",
    not(feature = "scalar-math")
))]
use core::arch::x86_64::*;

use core::{
    cmp::Ordering,
    fmt,
    ops::{Add, Deref, Div, Mul, MulAssign, Neg, Sub},
};

#[cfg(feature = "std")]
use std::iter::{Product, Sum};

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
type Inner = __m128;

#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
type Inner = crate::XYZW<f32>;

#[cfg(feature = "std")]
const ZERO: Quat = const_quat!([0.0, 0.0, 0.0, 0.0]);

/// A quaternion representing an orientation.
///
/// This quaternion is intended to be of unit length but may denormalize due to
/// floating point "error creep" which can occur when successive quaternion
/// operations are applied.
///
/// This type is 16 byte aligned.
#[cfg(doc)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[cfg(not(doc))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Quat(pub(crate) Inner);

/// Creates a `Quat` from `x`, `y`, `z` and `w` values.
///
/// This should generally not be called manually unless you know what you are doing. Use one of
/// the other constructors instead such as `identity` or `from_axis_angle`.
#[inline]
pub fn quat(x: f32, y: f32, z: f32, w: f32) -> Quat {
    Quat::from_xyzw(x, y, z, w)
}

impl Quat {
    /// Creates a new rotation quaternion.
    ///
    /// This should generally not be called manually unless you know what you are doing. Use one of
    /// the other constructors instead such as `identity` or `from_axis_angle`.
    ///
    /// `from_xyzw` is mostly used by unit tests and `serde` deserialization.
    #[inline]
    pub fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(Vector4::new(x, y, z, w))
    }

    #[inline]
    pub const fn identity() -> Self {
        Self(Inner::UNIT_W)
    }

    /// Creates a rotation quaternion from an unaligned `&[f32]`.
    ///
    /// # Preconditions
    ///
    /// The resulting quaternion is expected to be of unit length.
    ///
    /// # Panics
    ///
    /// Panics if `slice` length is less than 4.
    #[inline]
    pub fn from_slice_unaligned(slice: &[f32]) -> Self {
        #[allow(clippy::let_and_return)]
        let q = Vector4::from_slice_unaligned(slice);
        glam_assert!(q.is_normalized());
        Self(q)
    }

    /// Writes the quaternion to an unaligned `&mut [f32]`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` length is less than 4.
    #[inline]
    pub fn write_to_slice_unaligned(self, slice: &mut [f32]) {
        Vector4::write_to_slice_unaligned(self.0, slice)
    }

    /// Create a quaterion for a normalized rotation axis and angle (in radians).
    #[inline]
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        glam_assert!(axis.is_normalized());
        let (s, c) = scalar_sin_cos(angle * 0.5);
        let v = axis * s;
        Self::from_xyzw(v.x, v.y, v.z, c)
    }

    /// Creates a quaternion from the angle (in radians) around the x axis.
    #[inline]
    pub fn from_rotation_x(angle: f32) -> Self {
        let (s, c) = scalar_sin_cos(angle * 0.5);
        Self::from_xyzw(s, 0.0, 0.0, c)
    }

    /// Creates a quaternion from the angle (in radians) around the y axis.
    #[inline]
    pub fn from_rotation_y(angle: f32) -> Self {
        let (s, c) = scalar_sin_cos(angle * 0.5);
        Self::from_xyzw(0.0, s, 0.0, c)
    }

    /// Creates a quaternion from the angle (in radians) around the z axis.
    #[inline]
    pub fn from_rotation_z(angle: f32) -> Self {
        let (s, c) = scalar_sin_cos(angle * 0.5);
        Self::from_xyzw(0.0, 0.0, s, c)
    }

    #[inline]
    /// Create a quaternion from the given yaw (around y), pitch (around x) and roll (around z)
    /// in radians.
    pub fn from_rotation_ypr(yaw: f32, pitch: f32, roll: f32) -> Self {
        // Self::from_rotation_y(yaw) * Self::from_rotation_x(pitch) * Self::from_rotation_z(roll)
        let (y0, w0) = scalar_sin_cos(yaw * 0.5);
        let (x1, w1) = scalar_sin_cos(pitch * 0.5);
        let (z2, w2) = scalar_sin_cos(roll * 0.5);

        let x3 = w0 * x1;
        let y3 = y0 * w1;
        let z3 = -y0 * x1;
        let w3 = w0 * w1;

        let x4 = x3 * w2 + y3 * z2;
        let y4 = -x3 * z2 + y3 * w2;
        let z4 = w3 * z2 + z3 * w2;
        let w4 = w3 * w2 - z3 * z2;

        Self::from_xyzw(x4, y4, z4, w4)
    }

    #[inline]
    fn from_rotation_axes(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3) -> Self {
        // Based on https://github.com/microsoft/DirectXMath `XMQuaternionRotationMatrix`
        // TODO: sse2 version
        let (m00, m01, m02) = x_axis.into();
        let (m10, m11, m12) = y_axis.into();
        let (m20, m21, m22) = z_axis.into();
        if m22 <= 0.0 {
            // x^2 + y^2 >= z^2 + w^2
            let dif10 = m11 - m00;
            let omm22 = 1.0 - m22;
            if dif10 <= 0.0 {
                // x^2 >= y^2
                let four_xsq = omm22 - dif10;
                let inv4x = 0.5 / four_xsq.sqrt();
                Self::from_xyzw(
                    four_xsq * inv4x,
                    (m01 + m10) * inv4x,
                    (m02 + m20) * inv4x,
                    (m12 - m21) * inv4x,
                )
            } else {
                // y^2 >= x^2
                let four_ysq = omm22 + dif10;
                let inv4y = 0.5 / four_ysq.sqrt();
                Self::from_xyzw(
                    (m01 + m10) * inv4y,
                    four_ysq * inv4y,
                    (m12 + m21) * inv4y,
                    (m20 - m02) * inv4y,
                )
            }
        } else {
            // z^2 + w^2 >= x^2 + y^2
            let sum10 = m11 + m00;
            let opm22 = 1.0 + m22;
            if sum10 <= 0.0 {
                // z^2 >= w^2
                let four_zsq = opm22 - sum10;
                let inv4z = 0.5 / four_zsq.sqrt();
                Self::from_xyzw(
                    (m02 + m20) * inv4z,
                    (m12 + m21) * inv4z,
                    four_zsq * inv4z,
                    (m01 - m10) * inv4z,
                )
            } else {
                // w^2 >= z^2
                let four_wsq = opm22 + sum10;
                let inv4w = 0.5 / four_wsq.sqrt();
                Self::from_xyzw(
                    (m12 - m21) * inv4w,
                    (m20 - m02) * inv4w,
                    (m01 - m10) * inv4w,
                    four_wsq * inv4w,
                )
            }
        }
    }

    /// Creates a quaternion from a 3x3 rotation matrix.
    #[inline]
    pub fn from_rotation_mat3(mat: &Mat3) -> Self {
        Self::from_rotation_axes(mat.x_axis, mat.y_axis, mat.z_axis)
    }

    /// Creates a quaternion from a 3x3 rotation matrix inside a homogeneous 4x4 matrix.
    #[inline]
    pub fn from_rotation_mat4(mat: &Mat4) -> Self {
        Self::from_rotation_axes(mat.x_axis.xyz(), mat.y_axis.xyz(), mat.z_axis.xyz())
    }

    /// Returns the rotation axis and angle of `self`.
    #[inline]
    pub fn to_axis_angle(self) -> (Vec3, f32) {
        const EPSILON: f32 = 1.0e-8;
        const EPSILON_SQUARED: f32 = EPSILON * EPSILON;
        let (x, y, z, w) = Vector4::into_tuple(self.0);
        let angle = scalar_acos(w) * 2.0;
        let scale_sq = (1.0 - w * w).max(0.0);
        if scale_sq >= EPSILON_SQUARED {
            (Vec3::new(x, y, z) / scale_sq.sqrt(), angle)
        } else {
            (Vec3::unit_x(), angle)
        }
    }

    /// Returns the quaternion conjugate of `self`. For a unit quaternion the
    /// conjugate is also the inverse.
    #[inline]
    pub fn conjugate(self) -> Self {
        Self(self.0.conjugate())
    }

    /// Computes the dot product of `self` and `other`. The dot product is
    /// equal to the the cosine of the angle between two quaterion rotations.
    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        FloatVector4::dot(self.0, other.0)
    }

    /// Computes the length of `self`.
    #[inline]
    pub fn length(self) -> f32 {
        FloatVector4::length(self.0)
    }

    /// Computes the squared length of `self`.
    ///
    /// This is generally faster than `Quat::length()` as it avoids a square
    /// root operation.
    #[inline]
    pub fn length_squared(self) -> f32 {
        FloatVector4::dot(self.0, self.0)
    }

    /// Computes `1.0 / Quat::length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn length_recip(self) -> f32 {
        FloatVector4::length_recip(self.0)
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn normalize(self) -> Self {
        Self(FloatVector4::normalize(self.0))
    }

    /// Returns `true` if, and only if, all elements are finite.
    /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    pub fn is_finite(self) -> bool {
        // TODO: SIMD implementation
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }

    pub fn is_nan(self) -> bool {
        MaskVector4::any(FloatVector::is_nan(self.0))
    }

    /// Returns whether `self` of length `1.0` or not.
    ///
    /// Uses a precision threshold of `1e-6`.
    #[inline]
    pub fn is_normalized(self) -> bool {
        is_normalized!(self)
    }

    #[inline]
    pub fn is_near_identity(self) -> bool {
        use crate::scalar_traits::Float;
        // Based on https://github.com/nfrechette/rtm `rtm::quat_near_identity`
        const THRESHOLD_ANGLE: f32 = 0.002_847_144_6;
        // Because of floating point precision, we cannot represent very small rotations.
        // The closest f32 to 1.0 that is not 1.0 itself yields:
        // 0.99999994.acos() * 2.0  = 0.000690533954 rad
        //
        // An error threshold of 1.e-6 is used by default.
        // (1.0 - 1.e-6).acos() * 2.0 = 0.00284714461 rad
        // (1.0 - 1.e-7).acos() * 2.0 = 0.00097656250 rad
        //
        // We don't really care about the angle value itself, only if it's close to 0.
        // This will happen whenever quat.w is close to 1.0.
        // If the quat.w is close to -1.0, the angle will be near 2*PI which is close to
        // a negative 0 rotation. By forcing quat.w to be positive, we'll end up with
        // the shortest path.
        let positive_w_angle = self.w.abs().acos_approx() * 2.0;
        positive_w_angle < THRESHOLD_ANGLE
    }

    /// Returns true if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two `Quat`'s contain similar elements. It
    /// works best when comparing with a known value. The `max_abs_diff` that
    /// should be used used depends on the values being compared against.
    ///
    /// For more on floating point comparisons see
    /// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
    #[inline]
    pub fn abs_diff_eq(self, other: Self, max_abs_diff: f32) -> bool {
        // TODO: common function somehow
        MaskVector4::all(self.0.sub(other.0).abs().cmple(Vector::splat(max_abs_diff)))
        // abs_diff_eq!(self, other, max_abs_diff)
    }

    /// Performs a linear interpolation between `self` and `other` based on
    /// the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s`
    /// is `1.0`, the result will be equal to `other`.
    #[inline]
    pub fn lerp(self, end: Self, s: f32) -> Self {
        Self(self.0.lerp(end.0, s))
    }

    /// Performs a spherical linear interpolation between `self` and `end`
    /// based on the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s`
    /// is `1.0`, the result will be equal to `end`.
    ///
    /// Note that a rotation can be represented by two quaternions: `q` and
    /// `-q`. The slerp path between `q` and `end` will be different from the
    /// path between `-q` and `end`. One path will take the long way around and
    /// one will take the short way. In order to correct for this, the `dot`
    /// product between `self` and `end` should be positive. If the `dot`
    /// product is negative, slerp between `-self` and `end`.
    #[inline]
    pub fn slerp(self, end: Self, s: f32) -> Self {
        Self(self.0.slerp(end.0, s))
    }

    #[inline]
    /// Multiplies a quaternion and a 3D vector, rotating it.
    pub fn mul_vec3a(self, other: Vec3A) -> Vec3A {
        glam_assert!(self.is_normalized());
        let v = Vec4(self.0);
        let w = Vec3A::from(v.wwww());
        let two = Vec3A::splat(2.0);
        let b = Vec3A::from(v);
        let b2 = b.dot_as_vec3(b);
        other * (w * w - b2) + b * (other.dot_as_vec3(b) * two) + b.cross(other) * (w * two)
    }

    #[inline]
    /// Multiplies a quaternion and a 3D vector, rotating it.
    pub fn mul_vec3(self, other: Vec3) -> Vec3 {
        glam_assert!(self.is_normalized());
        self.mul_vec3a(Vec3A::from(other)).into()
    }

    #[inline]
    /// Multiplies two quaternions.
    /// If they each represent a rotation, the result will represent the combined rotation.
    /// Note that due to floating point rounding the result may not be perfectly normalized.
    pub fn mul_quat(self, other: Self) -> Self {
        Self(self.0.mul_quaternion(other.0))
    }
}

impl fmt::Debug for Quat {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_tuple("Quat")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

impl fmt::Display for Quat {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}

impl Add<Quat> for Quat {
    type Output = Self;
    #[inline]
    /// Adds two quaternions.
    /// The sum is not guaranteed to be normalized.
    ///
    /// NB: Addition is not the same as combining the rotations represented by the two quaternions!
    /// That corresponds to multiplication.
    fn add(self, other: Self) -> Self {
        Self(self.0.add(other.0))
    }
}

impl Sub<Quat> for Quat {
    type Output = Self;
    #[inline]
    /// Subtracts the other quaternion from self.
    /// The difference is not guaranteed to be normalized.
    fn sub(self, other: Self) -> Self {
        Self(self.0.sub(other.0))
    }
}

impl Mul<f32> for Quat {
    type Output = Self;
    #[inline]
    /// Multiplies a quaternion with an f32.
    /// The product is not guaranteed to be normalized.
    fn mul(self, other: f32) -> Self {
        Self(self.0.scale(other))
    }
}

impl Div<f32> for Quat {
    type Output = Self;
    #[inline]
    /// Divides a quaternion by an f32.
    /// The quotient is not guaranteed to be normalized.
    fn div(self, other: f32) -> Self {
        Self(self.0.scale(other.recip()))
    }
}

impl Mul<Quat> for Quat {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        Self(self.0.mul_quaternion(other.0))
    }
}

impl MulAssign<Quat> for Quat {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        self.0 = self.0.mul_quaternion(other.0);
    }
}

impl Mul<Vec3> for Quat {
    type Output = Vec3;
    #[inline]
    fn mul(self, other: Vec3) -> Self::Output {
        self.mul_vec3(other)
    }
}

impl Mul<Vec3A> for Quat {
    type Output = Vec3A;
    #[inline]
    fn mul(self, other: Vec3A) -> Self::Output {
        self.mul_vec3a(other)
    }
}

impl Neg for Quat {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(self.0.scale(-1.0))
    }
}

impl Default for Quat {
    #[inline]
    fn default() -> Self {
        Self::identity()
    }
}

impl PartialEq for Quat {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        MaskVector4::all(self.0.cmpeq(other.0))
    }
}

impl PartialOrd for Quat {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl AsRef<[f32; 4]> for Quat {
    #[inline]
    fn as_ref(&self) -> &[f32; 4] {
        unsafe { &*(self as *const Self as *const [f32; 4]) }
    }
}

impl AsMut<[f32; 4]> for Quat {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 4] {
        unsafe { &mut *(self as *mut Self as *mut [f32; 4]) }
    }
}

impl From<Vec4> for Quat {
    #[inline]
    fn from(v: Vec4) -> Self {
        Self(v.0)
    }
}

impl From<Quat> for Vec4 {
    #[inline]
    fn from(q: Quat) -> Self {
        Vec4(q.0)
    }
}

impl From<(f32, f32, f32, f32)> for Quat {
    #[inline]
    fn from(t: (f32, f32, f32, f32)) -> Self {
        Self(Vector4::from_tuple(t))
    }
}

impl From<Quat> for (f32, f32, f32, f32) {
    #[inline]
    fn from(q: Quat) -> Self {
        Vector4::into_tuple(q.0)
    }
}

impl From<[f32; 4]> for Quat {
    #[inline]
    fn from(a: [f32; 4]) -> Self {
        Self(Vector4::from_array(a))
    }
}

impl From<Quat> for [f32; 4] {
    #[inline]
    fn from(q: Quat) -> Self {
        Vector4::into_array(q.0)
    }
}

impl From<Quat> for Inner {
    // TODO: write test
    #[inline]
    fn from(q: Quat) -> Self {
        q.0
    }
}

impl From<Inner> for Quat {
    #[inline]
    fn from(inner: Inner) -> Self {
        Self(inner)
    }
}

impl Deref for Quat {
    type Target = crate::XYZW<f32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}

#[cfg(feature = "std")]
impl<'a> Sum<&'a Self> for Quat {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(ZERO, |a, &b| Self::add(a, b))
    }
}

#[cfg(feature = "std")]
impl<'a> Product<&'a Self> for Quat {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::identity(), |a, &b| Self::mul(a, b))
    }
}
