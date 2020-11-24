#[cfg(feature = "num-traits")]
use num_traits::Float;

use super::{Vec2, Vec3AMask, Vec3Mask, Vec4};
use crate::vector_traits::*;
use crate::XYZ;
use core::{cmp::Ordering, f32};
use core::{fmt, ops::*};

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

#[cfg(feature = "std")]
use std::iter::{Product, Sum};

type XYZF32 = XYZ<f32>;

#[cfg(not(doc))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec3(pub(crate) XYZF32);

/// A 3-dimensional vector without SIMD support.
#[cfg(doc)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[cfg(not(doc))]
#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
#[derive(Clone, Copy)]
#[repr(align(16), C)]
pub struct Vec3A(pub(crate) __m128);

#[cfg(not(doc))]
#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
#[derive(Clone, Copy)]
#[repr(align(16), C)]
pub struct Vec3A(pub(crate) XYZF32);

/// A 3-dimensional vector with SIMD support.
///
/// This type uses 16 byte aligned SIMD vector4 types for storage on supported platforms for better
/// performance than the `Vec3` type.
///
/// It is possible to convert between `Vec3` and `Vec3A` types using `From` trait implementations.
#[cfg(doc)]
#[derive(Clone, Copy)]
#[repr(align(16), C)]
pub struct Vec3A {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

macro_rules! impl_vec3 {
    ($new:ident, $vec3:ident, $t:ty, $mask:ident, $inner:ident) => {
        impl Default for $vec3 {
            #[inline]
            fn default() -> Self {
                Self(VectorConsts::ZERO)
            }
        }

        impl PartialEq for $vec3 {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.cmpeq(*other).all()
            }
        }

        impl PartialOrd for $vec3 {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.as_ref().partial_cmp(other.as_ref())
            }
        }

        impl From<$vec3> for $inner {
            // TODO: write test
            #[inline]
            fn from(t: $vec3) -> Self {
                t.0
            }
        }

        impl From<$inner> for $vec3 {
            #[inline]
            fn from(t: $inner) -> Self {
                Self(t)
            }
        }

        /// Creates a `Vec3`.
        #[inline]
        pub fn $new(x: $t, y: $t, z: $t) -> $vec3 {
            $vec3::new(x, y, z)
        }

        impl $vec3 {
            /// Creates a new `$vec3`.
            #[inline]
            pub fn new(x: $t, y: $t, z: $t) -> Self {
                Self(Vector3::new(x, y, z))
            }

            /// Creates a `$vec3` with all elements set to `0.0`.
            #[inline]
            pub const fn zero() -> Self {
                Self($inner::ZERO)
            }

            /// Creates a `$vec3` with all elements set to `1.0`.
            #[inline]
            pub const fn one() -> Self {
                Self($inner::ONE)
            }

            /// Creates a `$vec3` with values `[x: 1.0, y: 0.0, z: 0.0]`.
            #[inline]
            pub const fn unit_x() -> Self {
                Self(Vector3Consts::UNIT_X)
            }

            /// Creates a `$vec3` with values `[x: 0.0, y: 1.0, z: 0.0]`.
            #[inline]
            pub const fn unit_y() -> Self {
                Self(Vector3Consts::UNIT_Y)
            }

            /// Creates a `$vec3` with values `[x: 0.0, y: 0.0, z: 1.0]`.
            #[inline]
            pub const fn unit_z() -> Self {
                Self(Vector3Consts::UNIT_Z)
            }

            /// Creates a `$vec3` with all elements set to `v`.
            #[inline]
            pub fn splat(v: $t) -> Self {
                Self($inner::splat(v))
            }

            /// Creates a new `$vec3` from the elements in `if_true` and `if_false`, selecting which to use
            /// for each element of `self`.
            ///
            /// A true element in the mask uses the corresponding element from `if_true`, and false uses
            /// the element from `if_false`.
            #[inline]
            pub fn select(mask: $mask, if_true: $vec3, if_false: $vec3) -> $vec3 {
                Self($inner::select(mask.0, if_true.0, if_false.0))
            }

            /// Creates a `Vec4` from `self` and the given `w` value.
            #[inline]
            pub fn extend(self, w: $t) -> Vec4 {
                // TODO: Optimize?
                Vec4(Vector4::new(self.x, self.y, self.z, w))
            }

            /// Creates a `Vec2` from the `x` and `y` elements of `self`, discarding `z`.
            ///
            /// Truncation may also be performed by using `self.xy()` or `Vec2::from()`.
            #[inline]
            pub fn truncate(self) -> Vec2 {
                Vector3::into_xy(self.0).into()
            }

            /// Computes the dot product of `self` and `other`.
            #[inline]
            pub fn dot(self, other: Self) -> $t {
                FloatVector3::dot(self.0, other.0)
            }

            /// Returns $vec3 dot in all lanes of $vec3
            #[inline]
            #[allow(dead_code)]
            pub(crate) fn dot_as_vec3(self, other: Self) -> Self {
                Self(FloatVector3::dot_into_vec(self.0, other.0))
            }

            /// Computes the cross product of `self` and `other`.
            #[inline]
            pub fn cross(self, other: Self) -> Self {
                Self(self.0.cross(other.0))
            }

            /// Computes the length of `self`.
            #[inline]
            pub fn length(self) -> $t {
                FloatVector3::length(self.0)
            }

            /// Computes the squared length of `self`.
            ///
            /// This is generally faster than `$vec3::length()` as it avoids a square
            /// root operation.
            #[inline]
            pub fn length_squared(self) -> $t {
                FloatVector3::length_squared(self.0)
            }

            /// Computes `1.0 / $vec3::length()`.
            ///
            /// For valid results, `self` must _not_ be of length zero.
            #[inline]
            pub fn length_recip(self) -> $t {
                FloatVector3::length_recip(self.0)
            }

            /// Computes the Euclidean distance between two points in space.
            #[inline]
            pub fn distance(self, other: $vec3) -> $t {
                (self - other).length()
            }

            /// Compute the squared euclidean distance between two points in space.
            #[inline]
            pub fn distance_squared(self, other: $vec3) -> $t {
                (self - other).length_squared()
            }

            /// Returns `self` normalized to length 1.0.
            ///
            /// For valid results, `self` must _not_ be of length zero.
            #[inline]
            pub fn normalize(self) -> Self {
                Self(FloatVector3::normalize(self.0))
            }

            /// Returns the vertical minimum of `self` and `other`.
            ///
            /// In other words, this computes
            /// `[x: min(x1, x2), y: min(y1, y2), z: min(z1, z2)]`,
            /// taking the minimum of each element individually.
            #[inline]
            pub fn min(self, other: Self) -> Self {
                Self(self.0.min(other.0))
            }

            /// Returns the vertical maximum of `self` and `other`.
            ///
            /// In other words, this computes
            /// `[x: max(x1, x2), y: max(y1, y2), z: max(z1, z2)]`,
            /// taking the maximum of each element individually.
            #[inline]
            pub fn max(self, other: Self) -> Self {
                Self(self.0.max(other.0))
            }

            /// Returns the horizontal minimum of `self`'s elements.
            ///
            /// In other words, this computes `min(x, y, z)`.
            #[inline]
            pub fn min_element(self) -> $t {
                self.0.min_element()
            }

            /// Returns the horizontal maximum of `self`'s elements.
            ///
            /// In other words, this computes `max(x, y, z)`.
            #[inline]
            pub fn max_element(self) -> $t {
                self.0.max_element()
            }

            /// Performs a vertical `==` comparison between `self` and `other`,
            /// returning a `$mask` of the results.
            ///
            /// In other words, this computes `[x1 == x2, y1 == y2, z1 == z2, w1 == w2]`.
            #[inline]
            pub fn cmpeq(self, other: Self) -> $mask {
                $mask(self.0.cmpeq(other.0))
            }

            /// Performs a vertical `!=` comparison between `self` and `other`,
            /// returning a `$mask` of the results.
            ///
            /// In other words, this computes `[x1 != x2, y1 != y2, z1 != z2, w1 != w2]`.
            #[inline]
            pub fn cmpne(self, other: Self) -> $mask {
                $mask(self.0.cmpne(other.0))
            }

            /// Performs a vertical `>=` comparison between `self` and `other`,
            /// returning a `$mask` of the results.
            ///
            /// In other words, this computes `[x1 >= x2, y1 >= y2, z1 >= z2, w1 >= w2]`.
            #[inline]
            pub fn cmpge(self, other: Self) -> $mask {
                $mask(self.0.cmpge(other.0))
            }

            /// Performs a vertical `>` comparison between `self` and `other`,
            /// returning a `$mask` of the results.
            ///
            /// In other words, this computes `[x1 > x2, y1 > y2, z1 > z2, w1 > w2]`.
            #[inline]
            pub fn cmpgt(self, other: Self) -> $mask {
                $mask(self.0.cmpgt(other.0))
            }

            /// Performs a vertical `<=` comparison between `self` and `other`,
            /// returning a `$mask` of the results.
            ///
            /// In other words, this computes `[x1 <= x2, y1 <= y2, z1 <= z2, w1 <= w2]`.
            #[inline]
            pub fn cmple(self, other: Self) -> $mask {
                $mask(self.0.cmple(other.0))
            }

            /// Performs a vertical `<` comparison between `self` and `other`,
            /// returning a `$mask` of the results.
            ///
            /// In other words, this computes `[x1 < x2, y1 < y2, z1 < z2, w1 < w2]`.
            #[inline]
            pub fn cmplt(self, other: Self) -> $mask {
                $mask(self.0.cmplt(other.0))
            }

            /// Creates a `$vec3` from the first four values in `slice`.
            ///
            /// # Panics
            ///
            /// Panics if `slice` is less than three elements long.
            #[inline]
            pub fn from_slice_unaligned(slice: &[$t]) -> Self {
                Self(Vector3::from_slice_unaligned(slice))
            }

            /// Writes the elements of `self` to the first three elements in `slice`.
            ///
            /// # Panics
            ///
            /// Panics if `slice` is less than three elements long.
            #[inline]
            pub fn write_to_slice_unaligned(self, slice: &mut [$t]) {
                Vector3::write_to_slice_unaligned(self.0, slice)
            }

            /// Per element multiplication/addition of the three inputs: b + (self * a)
            #[inline]
            #[allow(dead_code)]
            pub(crate) fn mul_add(self, a: Self, b: Self) -> Self {
                Self(self.0.mul_add(a.0, b.0))
            }

            /// Returns a `$vec3` containing the absolute value of each element of `self`.
            #[inline]
            pub fn abs(self) -> Self {
                Self(self.0.abs())
            }

            /// Returns a `$vec3` containing the nearest integer to a number for each element of `self`.
            /// Round half-way cases away from 0.0.
            #[inline]
            pub fn round(self) -> Self {
                Self(self.0.round())
            }

            /// Returns a `$vec3` containing the largest integer less than or equal to a number for each
            /// element of `self`.
            #[inline]
            pub fn floor(self) -> Self {
                Self(self.0.floor())
            }

            /// Returns a `$vec3` containing the smallest integer greater than or equal to a number for each
            /// element of `self`.
            #[inline]
            pub fn ceil(self) -> Self {
                Self(self.0.ceil())
            }

            /// Returns a `$vec3` containing `e^self` (the exponential function) for each element of `self`.
            #[inline]
            pub fn exp(self) -> Self {
                Self::new(self.x.exp(), self.y.exp(), self.z.exp())
            }

            /// Returns a `$vec3` containing each element of `self` raised to the power of `n`.
            #[inline]
            pub fn powf(self, n: $t) -> Self {
                Self::new(self.x.powf(n), self.y.powf(n), self.z.powf(n))
            }

            /// Performs `is_nan()` on each element of self, returning a `$mask` of the results.
            ///
            /// In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan()]`.
            #[inline]
            pub fn is_nan_mask(self) -> $mask {
                $mask(self.0.is_nan())
            }

            /// Returns a `$vec3` with elements representing the sign of `self`.
            ///
            /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
            /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
            /// - `NAN` if the number is `NAN`
            #[inline]
            pub fn signum(self) -> Self {
                Self(self.0.signum())
            }

            /// Returns a `$vec3` containing the reciprocal `1.0/n` of each element of `self`.
            #[inline]
            pub fn recip(self) -> Self {
                Self(self.0.recip())
            }

            /// Performs a linear interpolation between `self` and `other` based on
            /// the value `s`.
            ///
            /// When `s` is `0.0`, the result will be equal to `self`.  When `s`
            /// is `1.0`, the result will be equal to `other`.
            #[inline]
            pub fn lerp(self, other: Self, s: $t) -> Self {
                self + ((other - self) * s)
            }

            /// Returns whether `self` of length `1.0` or not.
            ///
            /// Uses a precision threshold of `1e-6`.
            #[inline]
            pub fn is_normalized(self) -> bool {
                FloatVector3::is_normalized(self.0)
            }

            /// Returns `true` if, and only if, all elements are finite.
            /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
            #[inline]
            pub fn is_finite(self) -> bool {
                self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
            }

            /// Returns `true` if any elements are `NaN`.
            #[inline]
            pub fn is_nan(self) -> bool {
                MaskVector3::all(self.0.is_nan())
            }

            /// Returns true if the absolute difference of all elements between `self`
            /// and `other` is less than or equal to `max_abs_diff`.
            ///
            /// This can be used to compare if two `$vec3`'s contain similar elements. It
            /// works best when comparing with a known value. The `max_abs_diff` that
            /// should be used used depends on the values being compared against.
            ///
            /// For more on floating point comparisons see
            /// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
            #[inline]
            pub fn abs_diff_eq(self, other: Self, max_abs_diff: $t) -> bool {
                FloatVector3::abs_diff_eq(self.0, other.0, max_abs_diff)
            }

            /// Returns the angle between two vectors, in radians.
            ///
            /// The vectors do not need to be unit length, but this function does
            /// perform a `sqrt`.
            #[inline]
            pub fn angle_between(self, other: Self) -> $t {
                self.0.angle_between(other.0)
            }
        }

        impl AsRef<[$t; 3]> for $vec3 {
            #[inline]
            fn as_ref(&self) -> &[$t; 3] {
                unsafe { &*(self as *const $vec3 as *const [$t; 3]) }
            }
        }

        impl AsMut<[$t; 3]> for $vec3 {
            #[inline]
            fn as_mut(&mut self) -> &mut [$t; 3] {
                unsafe { &mut *(self as *mut $vec3 as *mut [$t; 3]) }
            }
        }

        impl fmt::Debug for $vec3 {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                let a = self.as_ref();
                fmt.debug_tuple(stringify!($vec3))
                    .field(&a[0])
                    .field(&a[1])
                    .field(&a[2])
                    .finish()
            }
        }

        impl fmt::Display for $vec3 {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
            }
        }

        impl Div<$vec3> for $vec3 {
            type Output = Self;
            #[inline]
            fn div(self, other: Self) -> Self {
                Self(self.0.div(other.0))
            }
        }

        impl DivAssign<$vec3> for $vec3 {
            #[inline]
            fn div_assign(&mut self, other: Self) {
                self.0 = self.0.div(other.0);
            }
        }

        impl Div<$t> for $vec3 {
            type Output = Self;
            #[inline]
            fn div(self, other: $t) -> Self {
                // TODO: add div by scalar to inner?
                Self(self.0.div($inner::splat(other)))
            }
        }

        impl DivAssign<$t> for $vec3 {
            #[inline]
            fn div_assign(&mut self, other: $t) {
                self.0 = self.0.div($inner::splat(other));
            }
        }

        impl Div<$vec3> for $t {
            type Output = $vec3;
            #[inline]
            fn div(self, other: $vec3) -> $vec3 {
                $vec3($inner::splat(self).div(other.0))
            }
        }

        impl Mul<$vec3> for $vec3 {
            type Output = Self;
            #[inline]
            fn mul(self, other: Self) -> Self {
                Self(self.0.mul(other.0))
            }
        }

        impl MulAssign<$vec3> for $vec3 {
            #[inline]
            fn mul_assign(&mut self, other: Self) {
                self.0 = self.0.mul(other.0);
            }
        }

        impl Mul<$t> for $vec3 {
            type Output = Self;
            #[inline]
            fn mul(self, other: $t) -> Self {
                Self(self.0.mul($inner::splat(other)))
            }
        }

        impl MulAssign<$t> for $vec3 {
            #[inline]
            fn mul_assign(&mut self, other: $t) {
                self.0 = self.0.mul($inner::splat(other));
            }
        }

        impl Mul<$vec3> for $t {
            type Output = $vec3;
            #[inline]
            fn mul(self, other: $vec3) -> $vec3 {
                $vec3($inner::splat(self).mul(other.0))
            }
        }

        impl Add for $vec3 {
            type Output = Self;
            #[inline]
            fn add(self, other: Self) -> Self {
                Self(self.0.add(other.0))
            }
        }

        impl AddAssign for $vec3 {
            #[inline]
            fn add_assign(&mut self, other: Self) {
                self.0 = self.0.add(other.0);
            }
        }

        impl Sub for $vec3 {
            type Output = Self;
            #[inline]
            fn sub(self, other: Self) -> Self {
                Self(self.0.sub(other.0))
            }
        }

        impl SubAssign for $vec3 {
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                self.0 = self.0.sub(other.0);
            }
        }

        impl Neg for $vec3 {
            type Output = Self;
            #[inline]
            fn neg(self) -> Self {
                Self(self.0.neg())
            }
        }

        impl Index<usize> for $vec3 {
            type Output = $t;
            #[inline]
            fn index(&self, index: usize) -> &Self::Output {
                &self.as_ref()[index]
            }
        }

        impl IndexMut<usize> for $vec3 {
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.as_mut()[index]
            }
        }

        impl From<($t, $t, $t)> for $vec3 {
            #[inline]
            fn from(t: ($t, $t, $t)) -> Self {
                Self(Vector3::from_tuple(t))
            }
        }

        impl From<$vec3> for ($t, $t, $t) {
            #[inline]
            fn from(v: $vec3) -> Self {
                v.into_tuple()
            }
        }

        impl From<[$t; 3]> for $vec3 {
            #[inline]
            fn from(a: [$t; 3]) -> Self {
                Self(Vector3::from_array(a))
            }
        }

        impl From<$vec3> for [$t; 3] {
            #[inline]
            fn from(v: $vec3) -> Self {
                v.into_array()
            }
        }

        impl From<$vec3> for Vec2 {
            /// Creates a `Vec2` from the `x` and `y` elements of the `$vec3`, discarding `z`.
            #[inline]
            fn from(v: $vec3) -> Self {
                v.into_xy().into()
            }
        }

        impl Deref for $vec3 {
            type Target = XYZ<$t>;
            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                unsafe { &*(self as *const Self as *const Self::Target) }
            }
        }

        impl DerefMut for $vec3 {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { &mut *(self as *mut Self as *mut Self::Target) }
            }
        }

        #[cfg(feature = "std")]
        impl<'a> Sum<&'a Self> for $vec3 {
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'a Self>,
            {
                iter.fold(Self::zero(), |a, &b| Self::add(a, b))
            }
        }

        #[cfg(feature = "std")]
        impl<'a> Product<&'a Self> for $vec3 {
            fn product<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'a Self>,
            {
                iter.fold(Self::one(), |a, &b| Self::mul(a, b))
            }
        }
    };
}

impl_vec3!(vec3, Vec3, f32, Vec3Mask, XYZF32);

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
impl_vec3!(vec3a, Vec3A, f32, Vec3AMask, __m128);
#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
impl_vec3!(vec3a, Vec3A, f32, Vec3AMask, XYZF32);

impl From<Vec3> for Vec3A {
    #[inline]
    fn from(v: Vec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl From<Vec3A> for Vec3 {
    #[inline]
    fn from(v: Vec3A) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

#[test]
fn test_vec3_private() {
    assert_eq!(
        vec3a(1.0, 1.0, 1.0).mul_add(vec3a(0.5, 2.0, -4.0), vec3a(-1.0, -1.0, -1.0)),
        vec3a(-0.5, 1.0, -5.0)
    );
}
