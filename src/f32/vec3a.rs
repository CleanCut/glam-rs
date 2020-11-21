#[cfg(feature = "num-traits")]
use num_traits::Float;

use crate::vector_traits::*;

use super::{Vec2, Vec3, Vec3AMask, Vec4};
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

use core::{cmp::Ordering, f32};

#[cfg(feature = "std")]
use std::iter::{Product, Sum};

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
type Inner = __m128;

#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
type Inner = crate::XYZ<f32>;

#[cfg(not(doc))]
#[derive(Clone, Copy)]
// if compiling with simd enabled assume alignment needs to match the simd type
// #[cfg_attr(any(vec4_sse2, vec4_f32_align16), repr(align(16)))]
#[repr(align(16), C)]
pub struct Vec3A(pub(crate) Inner);

/// A 3-dimensional vector with SIMD support.
///
/// This type uses 16 byte aligned SIMD vector4 types for storage on supported platforms for better
/// performance than the `Vec3` type.
///
/// It is possible to convert between `Vec3` and `Vec3A` types using `From` trait implementations.
#[cfg(doc)]
#[derive(Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(align(16), C)]
pub struct Vec3A {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Vec3A {
    #[inline]
    fn default() -> Self {
        Self(Inner::ZERO)
    }
}

impl PartialEq for Vec3A {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.cmpeq(*other).all()
    }
}

impl PartialOrd for Vec3A {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl From<Vec3A> for Inner {
    // TODO: write test
    #[inline]
    fn from(t: Vec3A) -> Self {
        t.0
    }
}

impl From<Inner> for Vec3A {
    #[inline]
    fn from(t: Inner) -> Self {
        Self(t)
    }
}

/// Creates a `Vec3`.
#[inline]
pub fn vec3a(x: f32, y: f32, z: f32) -> Vec3A {
    Vec3A::new(x, y, z)
}

impl Vec3A {
    /// Creates a new `Vec3A`.
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vector3::new(x, y, z))
    }

    /// Creates a `Vec3A` with all elements set to `0.0`.
    #[inline]
    pub const fn zero() -> Self {
        Self(Inner::ZERO)
    }

    /// Creates a `Vec3A` with all elements set to `1.0`.
    #[inline]
    pub const fn one() -> Self {
        Self(Inner::ONE)
    }

    /// Creates a `Vec3A` with values `[x: 1.0, y: 0.0, z: 0.0]`.
    #[inline]
    pub const fn unit_x() -> Self {
        Self(Vector3Consts::UNIT_X)
    }

    /// Creates a `Vec3A` with values `[x: 0.0, y: 1.0, z: 0.0]`.
    #[inline]
    pub const fn unit_y() -> Self {
        Self(Vector3Consts::UNIT_Y)
    }

    /// Creates a `Vec3A` with values `[x: 0.0, y: 0.0, z: 1.0]`.
    #[inline]
    pub const fn unit_z() -> Self {
        Self(Vector3Consts::UNIT_Z)
    }

    /// Creates a `Vec3A` with all elements set to `v`.
    #[inline]
    pub fn splat(v: f32) -> Self {
        Self(Inner::splat(v))
    }

    /// Creates a new `Vec3A` from the elements in `if_true` and `if_false`, selecting which to use
    /// for each element of `self`.
    ///
    /// A true element in the mask uses the corresponding element from `if_true`, and false uses
    /// the element from `if_false`.
    #[inline]
    pub fn select(mask: Vec3AMask, if_true: Vec3A, if_false: Vec3A) -> Vec3A {
        Self(Inner::select(mask.0, if_true.0, if_false.0))
    }

    /// Creates a `Vec4` from `self` and the given `w` value.
    #[inline]
    pub fn extend(self, w: f32) -> Vec4 {
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
    pub fn dot(self, other: Self) -> f32 {
        FloatVector3::dot(self.0, other.0)
    }

    /// Returns Vec3A dot in all lanes of Vec3A
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
    pub fn length(self) -> f32 {
        FloatVector3::length(self.0)
    }

    /// Computes the squared length of `self`.
    ///
    /// This is generally faster than `Vec3A::length()` as it avoids a square
    /// root operation.
    #[inline]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    /// Computes `1.0 / Vec3A::length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn length_recip(self) -> f32 {
        FloatVector3::length_recip(self.0)
    }

    /// Computes the Euclidean distance between two points in space.
    #[inline]
    pub fn distance(self, other: Vec3A) -> f32 {
        (self - other).length()
    }

    /// Compute the squared euclidean distance between two points in space.
    #[inline]
    pub fn distance_squared(self, other: Vec3A) -> f32 {
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
    pub fn min_element(self) -> f32 {
        self.0.min_element()
    }

    /// Returns the horizontal maximum of `self`'s elements.
    ///
    /// In other words, this computes `max(x, y, z)`.
    #[inline]
    pub fn max_element(self) -> f32 {
        self.0.max_element()
    }

    /// Performs a vertical `==` comparison between `self` and `other`,
    /// returning a `Vec3AMask` of the results.
    ///
    /// In other words, this computes `[x1 == x2, y1 == y2, z1 == z2, w1 == w2]`.
    #[inline]
    pub fn cmpeq(self, other: Self) -> Vec3AMask {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3AMask(_mm_cmpeq_ps(self.0, other.0))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3AMask(self.0.cmpeq(other.0))
        }
    }

    /// Performs a vertical `!=` comparison between `self` and `other`,
    /// returning a `Vec3AMask` of the results.
    ///
    /// In other words, this computes `[x1 != x2, y1 != y2, z1 != z2, w1 != w2]`.
    #[inline]
    pub fn cmpne(self, other: Self) -> Vec3AMask {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3AMask(_mm_cmpneq_ps(self.0, other.0))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3AMask(self.0.cmpne(other.0))
        }
    }

    /// Performs a vertical `>=` comparison between `self` and `other`,
    /// returning a `Vec3AMask` of the results.
    ///
    /// In other words, this computes `[x1 >= x2, y1 >= y2, z1 >= z2, w1 >= w2]`.
    #[inline]
    pub fn cmpge(self, other: Self) -> Vec3AMask {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3AMask(_mm_cmpge_ps(self.0, other.0))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3AMask(self.0.cmpge(other.0))
        }
    }

    /// Performs a vertical `>` comparison between `self` and `other`,
    /// returning a `Vec3AMask` of the results.
    ///
    /// In other words, this computes `[x1 > x2, y1 > y2, z1 > z2, w1 > w2]`.
    #[inline]
    pub fn cmpgt(self, other: Self) -> Vec3AMask {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3AMask(_mm_cmpgt_ps(self.0, other.0))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3AMask(self.0.cmpgt(other.0))
        }
    }

    /// Performs a vertical `<=` comparison between `self` and `other`,
    /// returning a `Vec3AMask` of the results.
    ///
    /// In other words, this computes `[x1 <= x2, y1 <= y2, z1 <= z2, w1 <= w2]`.
    #[inline]
    pub fn cmple(self, other: Self) -> Vec3AMask {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3AMask(_mm_cmple_ps(self.0, other.0))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3AMask(self.0.cmple(other.0))
        }
    }

    /// Performs a vertical `<` comparison between `self` and `other`,
    /// returning a `Vec3AMask` of the results.
    ///
    /// In other words, this computes `[x1 < x2, y1 < y2, z1 < z2, w1 < w2]`.
    #[inline]
    pub fn cmplt(self, other: Self) -> Vec3AMask {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3AMask(_mm_cmplt_ps(self.0, other.0))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3AMask(self.0.cmplt(other.0))
        }
    }

    /// Creates a `Vec3A` from the first four values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than three elements long.
    #[inline]
    pub fn from_slice_unaligned(slice: &[f32]) -> Self {
        Self(Vector3::from_slice_unaligned(slice))
    }

    /// Writes the elements of `self` to the first three elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than three elements long.
    #[inline]
    pub fn write_to_slice_unaligned(self, slice: &mut [f32]) {
        Vector3::write_to_slice_unaligned(self.0, slice)
    }

    /// Per element multiplication/addition of the three inputs: b + (self * a)
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn mul_add(self, a: Self, b: Self) -> Self {
        Self(self.0.mul_add(a.0, b.0))
    }

    /// Returns a `Vec3A` containing the absolute value of each element of `self`.
    #[inline]
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// Returns a `Vec3A` containing the nearest integer to a number for each element of `self`.
    /// Round half-way cases away from 0.0.
    #[inline]
    pub fn round(self) -> Self {
        Self(self.0.round())
    }

    /// Returns a `Vec3A` containing the largest integer less than or equal to a number for each
    /// element of `self`.
    #[inline]
    pub fn floor(self) -> Self {
        Self(self.0.floor())
    }

    /// Returns a `Vec3A` containing the smallest integer greater than or equal to a number for each
    /// element of `self`.
    #[inline]
    pub fn ceil(self) -> Self {
        Self(self.0.ceil())
    }

    /// Returns a `Vec3A` containing `e^self` (the exponential function) for each element of `self`.
    #[inline]
    pub fn exp(self) -> Self {
        Self::new(self.x.exp(), self.y.exp(), self.z.exp())
    }

    /// Returns a `Vec3A` containing each element of `self` raised to the power of `n`.
    #[inline]
    pub fn powf(self, n: f32) -> Self {
        Self::new(self.x.powf(n), self.y.powf(n), self.z.powf(n))
    }

    /// Performs `is_nan()` on each element of self, returning a `Vec3AMask` of the results.
    ///
    /// In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan()]`.
    #[inline]
    pub fn is_nan_mask(self) -> Vec3AMask {
        Vec3AMask(self.0.is_nan())
    }

    /// Returns a `Vec3A` with elements representing the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    #[inline]
    pub fn signum(self) -> Self {
        Self(self.0.signum())
    }

    /// Returns a `Vec3A` containing the reciprocal `1.0/n` of each element of `self`.
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
    pub fn lerp(self, other: Self, s: f32) -> Self {
        self + ((other - self) * s)
    }

    /// Returns whether `self` of length `1.0` or not.
    ///
    /// Uses a precision threshold of `1e-6`.
    #[inline]
    pub fn is_normalized(self) -> bool {
        is_normalized!(self)
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
    /// This can be used to compare if two `Vec3A`'s contain similar elements. It
    /// works best when comparing with a known value. The `max_abs_diff` that
    /// should be used used depends on the values being compared against.
    ///
    /// For more on floating point comparisons see
    /// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
    #[inline]
    pub fn abs_diff_eq(self, other: Self, max_abs_diff: f32) -> bool {
        abs_diff_eq!(self, other, max_abs_diff)
    }

    /// Returns the angle between two vectors, in radians.
    ///
    /// The vectors do not need to be unit length, but this function does
    /// perform a `sqrt`.
    #[inline]
    pub fn angle_between(self, other: Self) -> f32 {
        crate::f32::funcs::scalar_acos(self.dot(other) / (self.dot(self) * other.dot(other)).sqrt())
    }
}

impl AsRef<[f32; 3]> for Vec3A {
    #[inline]
    fn as_ref(&self) -> &[f32; 3] {
        unsafe { &*(self as *const Vec3A as *const [f32; 3]) }
    }
}

impl AsMut<[f32; 3]> for Vec3A {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 3] {
        unsafe { &mut *(self as *mut Vec3A as *mut [f32; 3]) }
    }
}

impl fmt::Debug for Vec3A {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let a = self.as_ref();
        fmt.debug_tuple("Vec3A")
            .field(&a[0])
            .field(&a[1])
            .field(&a[2])
            .finish()
    }
}

impl fmt::Display for Vec3A {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl Div<Vec3A> for Vec3A {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        Self(self.0.div(other.0))
    }
}

impl DivAssign<Vec3A> for Vec3A {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        self.0 = self.0.div(other.0);
    }
}

impl Div<f32> for Vec3A {
    type Output = Self;
    #[inline]
    fn div(self, other: f32) -> Self {
        // TODO: add div by scalar to inner?
        Self(self.0.div(Inner::splat(other)))
    }
}

impl DivAssign<f32> for Vec3A {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        self.0 = self.0.div(Inner::splat(other));
    }
}

impl Div<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn div(self, other: Vec3A) -> Vec3A {
        Vec3A(Inner::splat(self).div(other.0))
    }
}

impl Mul<Vec3A> for Vec3A {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        Self(self.0.mul(other.0))
    }
}

impl MulAssign<Vec3A> for Vec3A {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        self.0 = self.0.mul(other.0);
    }
}

impl Mul<f32> for Vec3A {
    type Output = Self;
    #[inline]
    fn mul(self, other: f32) -> Self {
        Self(self.0.mul(Inner::splat(other)))
    }
}

impl MulAssign<f32> for Vec3A {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        self.0 = self.0.mul(Inner::splat(other));
    }
}

impl Mul<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn mul(self, other: Vec3A) -> Vec3A {
        Vec3A(Inner::splat(self).mul(other.0))
    }
}

impl Add for Vec3A {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self(self.0.add(other.0))
    }
}

impl AddAssign for Vec3A {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0.add(other.0);
    }
}

impl Sub for Vec3A {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self(self.0.sub(other.0))
    }
}

impl SubAssign for Vec3A {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.0 = self.0.sub(other.0);
    }
}

impl Neg for Vec3A {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(self.0.neg())
    }
}

impl Index<usize> for Vec3A {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for Vec3A {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl From<(f32, f32, f32)> for Vec3A {
    #[inline]
    fn from(t: (f32, f32, f32)) -> Self {
        Self(Vector3::from_tuple(t))
    }
}

impl From<Vec3A> for (f32, f32, f32) {
    #[inline]
    fn from(v: Vec3A) -> Self {
        v.into_tuple()
    }
}

impl From<[f32; 3]> for Vec3A {
    #[inline]
    fn from(a: [f32; 3]) -> Self {
        Self(Vector3::from_array(a))
    }
}

impl From<Vec3A> for [f32; 3] {
    #[inline]
    fn from(v: Vec3A) -> Self {
        v.into_array()
    }
}

impl From<Vec3> for Vec3A {
    #[inline]
    fn from(v: Vec3) -> Self {
        Vec3A::new(v.x, v.y, v.z)
    }
}

impl From<Vec3A> for Vec2 {
    /// Creates a `Vec2` from the `x` and `y` elements of the `Vec3A`, discarding `z`.
    #[inline]
    fn from(v: Vec3A) -> Self {
        v.into_xy().into()
    }
}

impl Deref for Vec3A {
    type Target = crate::XYZ<f32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}

impl DerefMut for Vec3A {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self as *mut Self::Target) }
    }
}

#[cfg(feature = "std")]
impl<'a> Sum<&'a Self> for Vec3A {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::zero(), |a, &b| Self::add(a, b))
    }
}

#[cfg(feature = "std")]
impl<'a> Product<&'a Self> for Vec3A {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::one(), |a, &b| Self::mul(a, b))
    }
}

#[test]
fn test_vec3_private() {
    assert_eq!(
        vec3a(1.0, 1.0, 1.0).mul_add(vec3a(0.5, 2.0, -4.0), vec3a(-1.0, -1.0, -1.0)),
        vec3a(-0.5, 1.0, -5.0)
    );
}
