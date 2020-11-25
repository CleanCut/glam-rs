#[cfg(feature = "num-traits")]
use num_traits::Float;

use crate::vector_traits::*;
use crate::{Vec2Mask, Vec3, XY};
use core::{cmp::Ordering, f32, fmt, ops::*};

#[cfg(feature = "std")]
use std::iter::{Product, Sum};

type Inner = XY<f32>;

#[cfg(not(doc))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec2(pub(crate) Inner);

#[cfg(doc)]
#[derive(Clone, Copy)]
#[repr(C)]
/// A 2-dimensional vector.
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Default for Vec2 {
    #[inline]
    fn default() -> Self {
        Self(Inner::ZERO)
    }
}

impl PartialEq for Vec2 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.cmpeq(*other).all()
    }
}

impl PartialOrd for Vec2 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl From<Vec2> for Inner {
    #[inline]
    fn from(t: Vec2) -> Self {
        t.0
    }
}

impl From<Inner> for Vec2 {
    #[inline]
    fn from(t: Inner) -> Self {
        Self(t)
    }
}

/// Creates a `Vec2`.
#[inline]
pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}

impl Vec2 {
    /// Performs `is_nan` on each element of self, returning a `Vec2Mask` of the results.
    ///
    /// In other words, this computes `[x.is_nan(), y.is_nan()]`.
    #[inline]
    pub fn is_nan_mask(self) -> Vec2Mask {
        Vec2Mask(self.0.is_nan())
    }

    /// Returns a `Vec2` with elements representing the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    #[inline]
    pub fn signum(self) -> Self {
        Self(self.0.signum())
    }

    /// Returns a `Vec2` containing the reciprocal `1.0/n` of each element of `self`.
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

    /// Returns whether `self` is length `1.0` or not.
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
        self.x.is_finite() && self.y.is_finite()
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline]
    pub fn is_nan(self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }

    /// Returns true if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two `Vec2`'s contain similar elements. It
    /// works best when comparing with a known value. The `max_abs_diff` that
    /// should be used used depends on the values being compared against.
    ///
    /// For more on floating point comparisons see
    /// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
    #[inline]
    pub fn abs_diff_eq(self, other: Self, max_abs_diff: f32) -> bool {
        abs_diff_eq!(self, other, max_abs_diff)
    }

    /// Creates a new `Vec2`.
    #[inline]
    pub fn new(x: f32, y: f32) -> Vec2 {
        Self(Vector2::new(x, y))
    }

    /// Creates a `Vec2` with all elements set to `0.0`.
    #[inline]
    pub const fn zero() -> Vec2 {
        Self(Inner::ZERO)
    }

    /// Creates a `Vec2` with all elements set to `1.0`.
    #[inline]
    pub const fn one() -> Vec2 {
        Self(Inner::ONE)
    }

    /// Creates a `Vec2` with values `[x: 1.0, y: 0.0]`.
    #[inline]
    pub const fn unit_x() -> Vec2 {
        Self(Inner::UNIT_X)
    }

    /// Creates a `Vec2` with values `[x: 0.0, y: 1.0]`.
    #[inline]
    pub const fn unit_y() -> Vec2 {
        Self(Inner::UNIT_Y)
    }

    /// Creates a `Vec2` with all elements set to `v`.
    #[inline]
    pub fn splat(v: f32) -> Vec2 {
        Self(Inner::splat(v))
    }

    /// Creates a `Vec2` from the elements in `if_true` and `if_false`, selecting which to use for
    /// each element of `self`.
    ///
    /// A true element in the mask uses the corresponding element from `if_true`, and false uses
    /// the element from `if_false`.
    #[inline]
    pub fn select(mask: Vec2Mask, if_true: Vec2, if_false: Vec2) -> Vec2 {
        Self(Inner::select(mask.0, if_true.0, if_false.0))
    }

    /// Creates a `Vec2` from `self` and the given `z` value.
    #[inline]
    pub fn extend(self, z: f32) -> Vec3 {
        Vec3::new(self.x, self.y, z)
    }

    /// Computes the dot product of `self` and `other`.
    #[inline]
    pub fn dot(self, other: Vec2) -> f32 {
        FloatVector2::dot(self.0, other.0)
    }

    /// Computes the length of `self`.
    #[inline]
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Computes the squared length of `self`.
    ///
    /// This is generally faster than `Vec2::length()` as it avoids a square
    /// root operation.
    #[inline]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    /// Computes `1.0 / Vec2::length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn length_recip(self) -> f32 {
        self.length().recip()
    }

    /// Computes the Euclidean distance between two points.
    #[inline]
    pub fn distance(self, other: Vec2) -> f32 {
        (self - other).length()
    }

    /// Compute the squared Euclidean distance between two points.
    #[inline]
    pub fn distance_squared(self, other: Vec2) -> f32 {
        (self - other).length_squared()
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn normalize(self) -> Vec2 {
        self * self.length_recip()
    }

    /// Returns the vertical minimum of `self` and `other`.
    ///
    /// In other words, this computes
    /// `[x: min(x1, x2), y: min(y1, y2)]`,
    /// taking the minimum of each element individually.
    #[inline]
    pub fn min(self, other: Vec2) -> Vec2 {
        Self(self.0.min(other.0))
    }

    /// Returns the vertical maximum of `self` and `other`.
    ///
    /// In other words, this computes
    /// `[x: max(x1, x2), y: max(y1, y2)]`,
    /// taking the maximum of each element individually.
    #[inline]
    pub fn max(self, other: Vec2) -> Vec2 {
        Self(self.0.max(other.0))
    }

    /// Returns the horizontal minimum of `self`'s elements.
    ///
    /// In other words, this computes `min(x, y)`.
    #[inline]
    pub fn min_element(self) -> f32 {
        self.0.min_element()
    }

    /// Returns the horizontal maximum of `self`'s elements.
    ///
    /// In other words, this computes `max(x, y)`.
    #[inline]
    pub fn max_element(self) -> f32 {
        self.0.max_element()
    }

    /// Performs a vertical `==` comparison between `self` and `other`,
    /// returning a `Vec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 == x2, y1 == y2]`.
    #[inline]
    pub fn cmpeq(self, other: Vec2) -> Vec2Mask {
        Vec2Mask(self.0.cmpeq(other.0))
    }

    /// Performs a vertical `!=` comparison between `self` and `other`,
    /// returning a `Vec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 != x2, y1 != y2]`.
    #[inline]
    pub fn cmpne(self, other: Vec2) -> Vec2Mask {
        Vec2Mask(self.0.cmpne(other.0))
    }

    /// Performs a vertical `>=` comparison between `self` and `other`,
    /// returning a `Vec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 >= x2, y1 >= y2]`.
    #[inline]
    pub fn cmpge(self, other: Vec2) -> Vec2Mask {
        Vec2Mask(self.0.cmpge(other.0))
    }

    /// Performs a vertical `>` comparison between `self` and `other`,
    /// returning a `Vec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 > x2, y1 > y2]`.
    #[inline]
    pub fn cmpgt(self, other: Vec2) -> Vec2Mask {
        Vec2Mask(self.0.cmpgt(other.0))
    }

    /// Performs a vertical `<=` comparison between `self` and `other`,
    /// returning a `Vec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 <= x2, y1 <= y2]`.
    #[inline]
    pub fn cmple(self, other: Vec2) -> Vec2Mask {
        Vec2Mask(self.0.cmple(other.0))
    }

    /// Performs a vertical `<` comparison between `self` and `other`,
    /// returning a `Vec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 < x2, y1 < y2]`.
    #[inline]
    pub fn cmplt(self, other: Vec2) -> Vec2Mask {
        Vec2Mask(self.0.cmplt(other.0))
    }

    /// Creates a `Vec2` from the first two values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than two elements long.
    #[inline]
    pub fn from_slice_unaligned(slice: &[f32]) -> Self {
        Self(Inner::from_slice_unaligned(slice))
    }

    /// Writes the elements of `self` to the first two elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than two elements long.
    #[inline]
    pub fn write_to_slice_unaligned(self, slice: &mut [f32]) {
        self.0.write_to_slice_unaligned(slice)
    }

    /// Returns a `Vec2` containing the absolute value of each element of `self`.
    #[inline]
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// Returns a `Vec2` containing the nearest integer to a number for each element of `self`.
    /// Round half-way cases away from 0.0.
    #[inline]
    pub fn round(self) -> Self {
        Self(self.0.round())
    }

    /// Returns a `Vec2` containing the largest integer less than or equal to a number for each
    /// element of `self`.
    #[inline]
    pub fn floor(self) -> Self {
        Self(self.0.floor())
    }

    /// Returns a `Vec2` containing the smallest integer greater than or equal to a number for each
    /// element of `self`.
    #[inline]
    pub fn ceil(self) -> Self {
        Self(self.0.ceil())
    }

    /// Returns a `Vec2` containing `e^self` (the exponential function) for each element of `self`.
    #[inline]
    pub fn exp(self) -> Self {
        Self::new(self.x.exp(), self.y.exp())
    }

    /// Returns a `Vec2` containing each element of `self` raised to the power of `n`.
    #[inline]
    pub fn powf(self, n: f32) -> Self {
        Self::new(self.x.powf(n), self.y.powf(n))
    }

    /// Returns a `Vec2` that is equal to `self` rotated by 90 degrees.
    #[inline]
    pub fn perp(self) -> Self {
        Self(self.0.perp())
    }

    /// The perpendicular dot product of the vector and `other`.
    #[inline]
    pub fn perp_dot(self, other: Vec2) -> f32 {
        self.0.perp_dot(other.0)
    }

    /// Returns the angle between two vectors, in radians.
    ///
    /// The vectors do not need to be unit length, but this function does
    /// perform a `sqrt`.
    #[inline]
    pub fn angle_between(self, other: Self) -> f32 {
        self.0.angle_between(other.0)
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl Div<Vec2> for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, other: Vec2) -> Self {
        Self(self.0.div(other.0))
    }
}

impl DivAssign<Vec2> for Vec2 {
    #[inline]
    fn div_assign(&mut self, other: Vec2) {
        self.0 = self.0.div(other.0)
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, other: f32) -> Self {
        Self(self.0.div_scalar(other))
    }
}

impl DivAssign<f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        self.0 = self.div_scalar(other)
    }
}

impl Div<Vec2> for f32 {
    type Output = Vec2;
    #[inline]
    fn div(self, other: Vec2) -> Vec2 {
        Vec2(Inner::splat(self).div(other.0))
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Vec2) -> Self {
        Self(self.0.mul(other.0))
    }
}

impl MulAssign<Vec2> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, other: Vec2) {
        self.0 = self.0.mul(other.0)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, other: f32) -> Self {
        Self(self.0.mul_scalar(other))
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        self.0 = self.0.mul_scalar(other)
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;
    #[inline]
    fn mul(self, other: Vec2) -> Vec2 {
        Vec2(Inner::splat(self).mul(other.0))
    }
}

impl Add for Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self(self.0.add(other.0))
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0.add(other.0)
    }
}

impl Sub for Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Vec2) -> Self {
        Self(self.0.sub(other.0))
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, other: Vec2) {
        self.0 = self.0.sub(other.0)
    }
}

impl Neg for Vec2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(self.0.neg())
    }
}

impl AsRef<[f32; 2]> for Vec2 {
    #[inline]
    fn as_ref(&self) -> &[f32; 2] {
        unsafe { &*(self as *const Vec2 as *const [f32; 2]) }
    }
}

impl AsMut<[f32; 2]> for Vec2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 2] {
        unsafe { &mut *(self as *mut Vec2 as *mut [f32; 2]) }
    }
}

impl fmt::Debug for Vec2 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_tuple("Vec2")
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl Index<usize> for Vec2 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for Vec2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl From<(f32, f32)> for Vec2 {
    #[inline]
    fn from(t: (f32, f32)) -> Self {
        Self(Inner::from_tuple(t))
    }
}

impl From<Vec2> for (f32, f32) {
    #[inline]
    fn from(v: Vec2) -> Self {
        v.0.into_tuple()
    }
}

impl From<[f32; 2]> for Vec2 {
    #[inline]
    fn from(a: [f32; 2]) -> Self {
        Self(Inner::from_array(a))
    }
}

impl From<Vec2> for [f32; 2] {
    #[inline]
    fn from(v: Vec2) -> Self {
        v.0.into_array()
    }
}

impl Deref for Vec2 {
    type Target = Inner;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}

impl DerefMut for Vec2 {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self as *mut Self::Target) }
    }
}

#[cfg(feature = "std")]
impl<'a> Sum<&'a Self> for Vec2 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::zero(), |a, &b| Self::add(a, b))
    }
}

#[cfg(feature = "std")]
impl<'a> Product<&'a Self> for Vec2 {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::one(), |a, &b| Self::mul(a, b))
    }
}
