// use crate::XYZ;
// use crate::XYZW;
use core::{
    marker::Sized,
    ops::{Add, Div, Mul, Neg, Sub},
};

pub trait NumConsts: Sized {
    const ZERO: Self;
    const ONE: Self;
}

pub trait FloatConsts: Sized {
    const NEG_ONE: Self;
    const TWO: Self;
    const HALF: Self;
}

pub trait Num:
    NumConsts
    + Copy
    + Clone
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Sub<Output = Self>
{
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
}

pub trait Float: Num + Neg<Output = Self> + FloatConsts {
    // TODO: Move to Signed
    fn abs(self) -> Self;
    fn ceil(self) -> Self;
    fn floor(self) -> Self;
    fn is_nan(self) -> bool;
    fn recip(self) -> Self;
    fn round(self) -> Self;
    fn signum(self) -> Self;
    fn sqrt(self) -> Self;
    fn acos_approx(self) -> Self;
    fn sin(self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn from_f32(f: f32) -> Self;
    fn from_f64(f: f64) -> Self;
}

impl NumConsts for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

impl FloatConsts for f32 {
    const NEG_ONE: Self = -1.0;
    const TWO: Self = 2.0;
    const HALF: Self = 0.5;
}

impl Num for f32 {
    #[inline(always)]
    fn min(self, other: Self) -> Self {
        f32::min(self, other)
    }
    #[inline(always)]
    fn max(self, other: Self) -> Self {
        f32::max(self, other)
    }
}

impl Float for f32 {
    #[inline(always)]
    fn from_f32(v: f32) -> Self {
        v
    }
    #[inline(always)]
    fn from_f64(v: f64) -> Self {
        v as Self
    }
    #[inline(always)]
    fn abs(self) -> Self {
        Self::abs(self)
    }
    #[inline(always)]
    fn ceil(self) -> Self {
        Self::ceil(self)
    }
    #[inline(always)]
    fn floor(self) -> Self {
        Self::floor(self)
    }
    #[inline(always)]
    fn is_nan(self) -> bool {
        Self::is_nan(self)
    }
    #[inline(always)]
    fn recip(self) -> Self {
        Self::recip(self)
    }
    #[inline(always)]
    fn round(self) -> Self {
        Self::round(self)
    }
    #[inline(always)]
    fn sin(self) -> Self {
        Self::sin(self)
    }
    #[inline(always)]
    fn sin_cos(self) -> (Self, Self) {
        Self::sin_cos(self)
    }
    #[inline(always)]
    fn signum(self) -> Self {
        Self::signum(self)
    }
    #[inline(always)]
    fn sqrt(self) -> Self {
        Self::sqrt(self)
    }
    #[inline(always)]
    fn acos_approx(self) -> Self {
        crate::f32::scalar_acos(self)
    }
}

impl NumConsts for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

impl FloatConsts for f64 {
    const NEG_ONE: Self = -1.0;
    const TWO: Self = 2.0;
    const HALF: Self = 0.5;
}

impl Num for f64 {
    #[inline(always)]
    fn min(self, other: Self) -> Self {
        f64::min(self, other)
    }
    #[inline(always)]
    fn max(self, other: Self) -> Self {
        f64::max(self, other)
    }
}

impl Float for f64 {
    #[inline(always)]
    fn from_f32(v: f32) -> Self {
        v as Self
    }
    #[inline(always)]
    fn from_f64(v: f64) -> Self {
        v
    }
    #[inline(always)]
    fn abs(self) -> Self {
        Self::abs(self)
    }
    #[inline(always)]
    fn ceil(self) -> Self {
        Self::ceil(self)
    }
    #[inline(always)]
    fn floor(self) -> Self {
        Self::floor(self)
    }
    #[inline(always)]
    fn is_nan(self) -> bool {
        Self::is_nan(self)
    }
    #[inline(always)]
    fn recip(self) -> Self {
        Self::recip(self)
    }
    #[inline(always)]
    fn round(self) -> Self {
        Self::round(self)
    }
    #[inline(always)]
    fn sin(self) -> Self {
        Self::sin(self)
    }
    #[inline(always)]
    fn sin_cos(self) -> (Self, Self) {
        Self::sin_cos(self)
    }
    #[inline(always)]
    fn signum(self) -> Self {
        Self::signum(self)
    }
    #[inline(always)]
    fn sqrt(self) -> Self {
        Self::sqrt(self)
    }
    #[inline(always)]
    fn acos_approx(self) -> Self {
        // TODO: clamp range
        Self::acos(self)
    }
}

/*
impl<T: Num> Add for XYZW<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        XYZW {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<T: Num> Div for XYZW<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        XYZW {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        }
    }
}

impl<T: Num> Div<T> for XYZW<T> {
    type Output = Self;
    fn div(self, other: T) -> Self::Output {
        XYZW {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl<T: Num> Mul for XYZW<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        XYZW {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl<T: Num> Mul<T> for XYZW<T> {
    type Output = Self;
    fn mul(self, other: T) -> Self::Output {
        XYZW {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl<T: Num> Sub for XYZW<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        XYZW {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}
*/
