#[cfg(feature = "num-traits")]
use num_traits::Float;

use crate::vector_traits::*;

use super::{Vec2, Vec3, Vec3A, Vec4Mask};
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

#[cfg(vec4_sse2)]
use crate::Align16;
#[cfg(vec4_sse2)]
use core::mem::MaybeUninit;

use core::{cmp::Ordering, f32};

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
type Inner = __m128;

#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
type Inner = crate::XYZW<f32>;

#[cfg(not(doc))]
#[derive(Clone, Copy)]
// if compiling with simd enabled assume alignment needs to match the simd type
// #[cfg_attr(any(vec4_sse2, vec4_f32_align16), repr(align(16)))]
#[repr(C)]
pub struct Vec4(pub(crate) Inner);

/// A 4-dimensional vector.
///
/// This type is 16 byte aligned unless the `scalar-math` feature is enabed.
#[cfg(doc)]
#[derive(Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(C)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Default for Vec4 {
    #[inline]
    fn default() -> Self {
        Self(Inner::ZERO)
    }
}

impl PartialEq for Vec4 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.cmpeq(*other).all()
    }
}

impl PartialOrd for Vec4 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl From<Vec4> for Inner {
    #[inline]
    fn from(t: Vec4) -> Self {
        t.0
    }
}

impl From<Inner> for Vec4 {
    #[inline]
    fn from(t: Inner) -> Self {
        Self(t)
    }
}

/// Creates a `Vec4`.
#[inline]
pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4::new(x, y, z, w)
}

impl Vec4 {
    /// Creates a new `Vec4`.
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(Vector4::new(x, y, z, w))
    }

    /// Creates a `Vec4` with all elements set to `0.0`.
    #[inline]
    pub const fn zero() -> Self {
        Self(Inner::ZERO)
    }

    /// Creates a `Vec4` with all elements set to `1.0`.
    #[inline]
    pub const fn one() -> Self {
        Self(Inner::ONE)
    }

    /// Creates a `Vec4` with values `[x: 1.0, y: 0.0, z: 0.0, w: 0.0]`.
    #[inline]
    pub const fn unit_x() -> Self {
        Self(Vector4Consts::UNIT_X)
    }

    /// Creates a `Vec4` with values `[x: 0.0, y: 1.0, z: 0.0, w: 0.0]`.
    #[inline]
    pub const fn unit_y() -> Self {
        Self(Vector4Consts::UNIT_Y)
    }

    /// Creates a `Vec4` with values `[x: 0.0, y: 0.0, z: 1.0, w: 0.0]`.
    #[inline]
    pub const fn unit_z() -> Self {
        Self(Vector4Consts::UNIT_Z)
    }

    /// Creates a `Vec4` with values `[x: 0.0, y: 0.0, z: 0.0, w: 1.0]`.
    #[inline]
    pub const fn unit_w() -> Self {
        Self(Vector4Consts::UNIT_W)
    }

    /// Creates a `Vec4` with all elements set to `v`.
    #[inline]
    pub fn splat(v: f32) -> Self {
        Self(Inner::splat(v))
    }

    /// Creates a `Vec4` from the elements in `if_true` and `if_false`, selecting which to use for
    /// each element of `self`.
    ///
    /// A true element in the mask uses the corresponding element from `if_true`, and false uses
    /// the element from `if_false`.
    #[inline]
    pub fn select(mask: Vec4Mask, if_true: Vec4, if_false: Vec4) -> Vec4 {
        Self(Inner::select(mask.0, if_true.0, if_false.0))
    }

    /// Creates a `Vec3` from the `x`, `y` and `z` elements of `self`, discarding `w`.
    ///
    /// Truncation to `Vec3` may also be performed by using `self.xyz()` or `Vec3::from()`.
    ///
    /// To truncate to `Vec3A` use `Vec3A::from()`.
    #[inline]
    pub fn truncate(self) -> Vec3 {
        // TODO: Inner truncate
        Vec3::new(self.x, self.y, self.z)
    }

    /// Calculates the Vec4 dot product and returns answer in x lane of __m128.
    #[cfg(vec4_sse2)]
    #[inline]
    unsafe fn dot_as_m128(self, other: Self) -> __m128 {
        let x2_y2_z2_w2 = _mm_mul_ps(self.0, other.0);
        let z2_w2_0_0 = _mm_shuffle_ps(x2_y2_z2_w2, x2_y2_z2_w2, 0b00_00_11_10);
        let x2z2_y2w2_0_0 = _mm_add_ps(x2_y2_z2_w2, z2_w2_0_0);
        let y2w2_0_0_0 = _mm_shuffle_ps(x2z2_y2w2_0_0, x2z2_y2w2_0_0, 0b00_00_00_01);
        _mm_add_ps(x2z2_y2w2_0_0, y2w2_0_0_0)
    }

    /// Returns Vec4 dot in all lanes of Vec4
    #[cfg(vec4_sse2)]
    #[inline]
    pub(crate) fn dot_as_vec4(self, other: Self) -> Self {
        unsafe {
            let dot_in_x = self.dot_as_m128(other);
            Self(_mm_shuffle_ps(dot_in_x, dot_in_x, 0b00_00_00_00))
        }
    }

    /// Computes the 4D dot product of `self` and `other`.
    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        FloatVector4::dot(self.0, other.0)
    }

    /// Computes the 4D length of `self`.
    #[inline]
    pub fn length(self) -> f32 {
        FloatVector4::length(self.0)
    }

    /// Computes the squared 4D length of `self`.
    ///
    /// This is generally faster than `Vec4::length()` as it avoids a square
    /// root operation.
    #[inline]
    pub fn length_squared(self) -> f32 {
        FloatVector4::dot(self.0, self.0)
    }

    /// Computes `1.0 / Vec4::length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn length_recip(self) -> f32 {
        FloatVector4::length_recip(self.0)
    }

    /// Computes the Euclidean distance between two points in space.
    #[inline]
    pub fn distance(self, other: Vec4) -> f32 {
        (self - other).length()
    }

    /// Compute the squared euclidean distance between two points in space.
    #[inline]
    pub fn distance_squared(self, other: Vec4) -> f32 {
        (self - other).length_squared()
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn normalize(self) -> Self {
        Self(FloatVector4::normalize(self.0))
    }

    /// Returns the vertical minimum of `self` and `other`.
    ///
    /// In other words, this computes
    /// `[x: min(x1, x2), y: min(y1, y2), z: min(z1, z2), w: min(w1, w2)]`,
    /// taking the minimum of each element individually.
    #[inline]
    pub fn min(self, other: Self) -> Self {
        Self(self.0.min(other.0))
    }

    /// Returns the vertical maximum of `self` and `other`.
    ///
    /// In other words, this computes
    /// `[x: max(x1, x2), y: max(y1, y2), z: max(z1, z2), w: max(w1, w2)]`,
    /// taking the maximum of each element individually.
    #[inline]
    pub fn max(self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }

    /// Returns the horizontal minimum of `self`'s elements.
    ///
    /// In other words, this computes `min(x, y, z, w)`.
    #[inline]
    pub fn min_element(self) -> f32 {
        self.0.min_element()
    }

    /// Returns the horizontal maximum of `self`'s elements.
    ///
    /// In other words, this computes `max(x, y, z, w)`.
    #[inline]
    pub fn max_element(self) -> f32 {
        self.0.max_element()
    }

    /// Performs a vertical `==` comparison between `self` and `other`,
    /// returning a `Vec4Mask` of the results.
    ///
    /// In other words, this computes `[x1 == x2, y1 == y2, z1 == z2, w1 == w2]`.
    #[inline]
    pub fn cmpeq(self, other: Self) -> Vec4Mask {
        Vec4Mask(self.0.cmpeq(other.0))
    }

    /// Performs a vertical `!=` comparison between `self` and `other`,
    /// returning a `Vec4Mask` of the results.
    ///
    /// In other words, this computes `[x1 != x2, y1 != y2, z1 != z2, w1 != w2]`.
    #[inline]
    pub fn cmpne(self, other: Self) -> Vec4Mask {
        Vec4Mask(self.0.cmpne(other.0))
    }

    /// Performs a vertical `>=` comparison between `self` and `other`,
    /// returning a `Vec4Mask` of the results.
    ///
    /// In other words, this computes `[x1 >= x2, y1 >= y2, z1 >= z2, w1 >= w2]`.
    #[inline]
    pub fn cmpge(self, other: Self) -> Vec4Mask {
        Vec4Mask(self.0.cmpge(other.0))
    }

    /// Performs a vertical `>` comparison between `self` and `other`,
    /// returning a `Vec4Mask` of the results.
    ///
    /// In other words, this computes `[x1 > x2, y1 > y2, z1 > z2, w1 > w2]`.
    #[inline]
    pub fn cmpgt(self, other: Self) -> Vec4Mask {
        Vec4Mask(self.0.cmpgt(other.0))
    }

    /// Performs a vertical `<=` comparison between `self` and `other`,
    /// returning a `Vec4Mask` of the results.
    ///
    /// In other words, this computes `[x1 <= x2, y1 <= y2, z1 <= z2, w1 <= w2]`.
    #[inline]
    pub fn cmple(self, other: Self) -> Vec4Mask {
        Vec4Mask(self.0.cmple(other.0))
    }

    /// Performs a vertical `<` comparison between `self` and `other`,
    /// returning a `Vec4Mask` of the results.
    ///
    /// In other words, this computes `[x1 < x2, y1 < y2, z1 < z2, w1 < w2]`.
    #[inline]
    pub fn cmplt(self, other: Self) -> Vec4Mask {
        Vec4Mask(self.0.cmplt(other.0))
    }

    /// Creates a `Vec4` from the first four values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than four elements long.
    #[inline]
    pub fn from_slice_unaligned(slice: &[f32]) -> Self {
        Self(Vector4::from_slice_unaligned(slice))
    }

    /// Writes the elements of `self` to the first four elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than four elements long.
    #[inline]
    pub fn write_to_slice_unaligned(self, slice: &mut [f32]) {
        Vector4::write_to_slice_unaligned(self.0, slice)
    }

    /// Per element multiplication/addition of the three inputs: b + (self * a)
    #[inline]
    pub(crate) fn mul_add(self, a: Self, b: Self) -> Self {
        Self(self.0.mul_add(a.0, b.0))
    }

    /// Returns a `Vec4` containing the absolute value of each element of `self`.
    #[inline]
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// Returns a `Vec4` containing the nearest integer to a number for each element of `self`.
    /// Round half-way cases away from 0.0.
    #[inline]
    pub fn round(self) -> Self {
        Self(self.0.round())
    }

    /// Returns a `Vec4` containing the largest integer less than or equal to a number for each
    /// element of `self`.
    #[inline]
    pub fn floor(self) -> Self {
        Self(self.0.floor())
    }

    /// Returns a `Vec4` containing the smallest integer greater than or equal to a number for each
    /// element of `self`.
    #[inline]
    pub fn ceil(self) -> Self {
        Self(self.0.ceil())
    }

    /// Returns a `Vec4` containing `e^self` (the exponential function) for each element of `self`.
    #[inline]
    pub fn exp(self) -> Self {
        Self::new(self.x.exp(), self.y.exp(), self.z.exp(), self.w.exp())
    }

    /// Returns a `Vec4` containing each element of `self` raised to the power of `n`.
    #[inline]
    pub fn powf(self, n: f32) -> Self {
        Self::new(
            self.x.powf(n),
            self.y.powf(n),
            self.z.powf(n),
            self.w.powf(n),
        )
    }

    /// Performs `is_nan` on each element of self, returning a `Vec4Mask` of the results.
    ///
    /// In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
    #[inline]
    pub fn is_nan_mask(self) -> Vec4Mask {
        Vec4Mask(self.0.is_nan())
    }

    /// Returns a `Vec4` with elements representing the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    #[inline]
    pub fn signum(self) -> Self {
        Self(self.0.signum())
    }

    /// Returns a `Vec4` containing the reciprocal `1.0/n` of each element of `self`.
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

    /// Returns `true` if, and only if, all elements are finite.
    /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline]
    pub fn is_nan(self) -> bool {
        #[cfg(vec4_sse2)]
        {
            self.is_nan_mask().any()
        }

        #[cfg(vec4_f32)]
        {
            self.x.is_nan() || self.y.is_nan() || self.z.is_nan() || self.w.is_nan()
        }
    }

    /// Returns whether `self` is length `1.0` or not.
    ///
    /// Uses a precision threshold of `1e-6`.
    #[inline]
    pub fn is_normalized(self) -> bool {
        is_normalized!(self)
    }

    /// Returns true if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two `Vec4`'s contain similar elements. It
    /// works best when comparing with a known value. The `max_abs_diff` that
    /// should be used used depends on the values being compared against.
    ///
    /// For more on floating point comparisons see
    /// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
    #[inline]
    pub fn abs_diff_eq(self, other: Self, max_abs_diff: f32) -> bool {
        abs_diff_eq!(self, other, max_abs_diff)
    }
}

impl AsRef<[f32; 4]> for Vec4 {
    #[inline]
    fn as_ref(&self) -> &[f32; 4] {
        unsafe { &*(self as *const Self as *const [f32; 4]) }
    }
}

impl AsMut<[f32; 4]> for Vec4 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 4] {
        unsafe { &mut *(self as *mut Self as *mut [f32; 4]) }
    }
}

impl fmt::Debug for Vec4 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let a = self.as_ref();
        fmt.debug_tuple("Vec4")
            .field(&a[0])
            .field(&a[1])
            .field(&a[2])
            .field(&a[3])
            .finish()
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let a = self.as_ref();
        write!(fmt, "[{}, {}, {}, {}]", a[0], a[1], a[2], a[3])
    }
}

impl Div<Vec4> for Vec4 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        Self(self.0.div(other.0))
    }
}

impl DivAssign<Vec4> for Vec4 {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        self.0 = self.0.div(other.0);
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;
    #[inline]
    fn div(self, other: f32) -> Self {
        // TODO: add div by scalar to inner?
        Self(self.0.div(Inner::splat(other)))
    }
}

impl DivAssign<f32> for Vec4 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        self.0 = self.0.div(Inner::splat(other));
    }
}

impl Div<Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn div(self, other: Vec4) -> Vec4 {
        Vec4(Inner::splat(self).div(other.0))
    }
}

impl Mul<Vec4> for Vec4 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        Self(self.0.mul(other.0))
    }
}

impl MulAssign<Vec4> for Vec4 {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        self.0 = self.0.mul(other.0);
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;
    #[inline]
    fn mul(self, other: f32) -> Self {
        Self(self.0.mul(Inner::splat(other)))
    }
}

impl MulAssign<f32> for Vec4 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        self.0 = self.0.mul(Inner::splat(other));
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn mul(self, other: Vec4) -> Vec4 {
        Vec4(Inner::splat(self).mul(other.0))
    }
}

impl Add for Vec4 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self(self.0.add(other.0))
    }
}

impl AddAssign for Vec4 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0.add(other.0);
    }
}

impl Sub for Vec4 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self(self.0.sub(other.0))
    }
}

impl SubAssign for Vec4 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.0 = self.0.sub(other.0);
    }
}

impl Neg for Vec4 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(self.0.neg())
    }
}

impl Index<usize> for Vec4 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for Vec4 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl From<(f32, f32, f32, f32)> for Vec4 {
    #[inline]
    fn from(t: (f32, f32, f32, f32)) -> Self {
        Self(Vector4::from_tuple(t))
    }
}

impl From<Vec4> for (f32, f32, f32, f32) {
    #[inline]
    fn from(v: Vec4) -> Self {
        Vector4::into_tuple(v.0)
    }
}

impl From<[f32; 4]> for Vec4 {
    #[inline]
    fn from(a: [f32; 4]) -> Self {
        Self(Vector4::from_array(a))
    }
}

impl From<Vec4> for [f32; 4] {
    #[inline]
    fn from(v: Vec4) -> Self {
        v.into_array()
    }
}

impl From<Vec4> for Vec3A {
    /// Creates a `Vec3` from the `x`, `y` and `z` elements of the `Vec4`, discarding `z`.
    ///
    /// On architectures where SIMD is supported such as SSE2 on x86_64 this conversion is a noop.
    #[inline]
    fn from(v: Vec4) -> Self {
        Self(v.0.into())
    }
}

impl From<Vec4> for Vec3 {
    /// Creates a `Vec3` from the `x`, `y` and `z` elements of the `Vec4`, discarding `z`.
    #[inline]
    fn from(v: Vec4) -> Self {
        // TODO: Vec3(Inner)
        #[cfg(vec4_sse2)]
        {
            let mut out: MaybeUninit<Align16<Vec3>> = MaybeUninit::uninit();
            unsafe {
                _mm_store_ps(out.as_mut_ptr() as *mut f32, v.0);
                out.assume_init().0
            }
        }

        #[cfg(vec4_f32)]
        {
            Vec3 {
                x: v.x,
                y: v.y,
                z: v.z,
            }
        }
    }
}

impl From<Vec4> for Vec2 {
    /// Creates a `Vec2` from the `x` and `y` elements of the `Vec4`, discarding `z`.
    #[inline]
    fn from(v: Vec4) -> Self {
        // TODO: Vec3(Inner)
        #[cfg(vec4_sse2)]
        {
            let mut out: MaybeUninit<Align16<Vec2>> = MaybeUninit::uninit();
            unsafe {
                _mm_store_ps(out.as_mut_ptr() as *mut f32, v.0);
                out.assume_init().0
            }
        }

        #[cfg(vec4_f32)]
        {
            Vec2 { x: v.x, y: v.y }
        }
    }
}

impl Deref for Vec4 {
    type Target = crate::XYZW<f32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        Vector4::deref(&self.0)
    }
}

impl DerefMut for Vec4 {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        Vector4::deref_mut(&mut self.0)
    }
}

#[cfg(feature = "std")]
impl<'a> Sum<&'a Self> for Vec4 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::zero(), |a, &b| Self::add(a, b))
    }
}

#[cfg(feature = "std")]
impl<'a> Product<&'a Self> for Vec4 {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::one(), |a, &b| Self::mul(a, b))
    }
}

#[test]
fn test_vec4_private() {
    assert_eq!(
        vec4(1.0, 1.0, 1.0, 1.0).mul_add(vec4(0.5, 2.0, -4.0, 0.0), vec4(-1.0, -1.0, -1.0, -1.0)),
        vec4(-0.5, 1.0, -5.0, -1.0)
    );
}
