// use crate::XYZ;
use crate::XYZW;
use core::{
    marker::Sized,
    ops::{Add, Div, Mul, Neg, Sub},
};

pub trait MaskConsts: Sized {
    const FALSE: Self;
}

pub trait Mask: MaskConsts + Copy + Clone {}

pub trait NumConsts: Sized {
    const ZERO: Self;
    const ONE: Self;
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

pub trait Float: Num + Neg<Output = Self> {
    // TODO: Move to Signed
    fn abs(self) -> Self;
    fn ceil(self) -> Self;
    fn floor(self) -> Self;
    fn is_nan(self) -> bool;
    fn recip(self) -> Self;
    fn round(self) -> Self;
    fn signum(self) -> Self;
    fn sqrt(self) -> Self;
}

impl NumConsts for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
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
    fn abs(self) -> Self {
        f32::abs(self)
    }
    #[inline(always)]
    fn ceil(self) -> Self {
        f32::ceil(self)
    }
    #[inline(always)]
    fn floor(self) -> Self {
        f32::floor(self)
    }
    #[inline(always)]
    fn is_nan(self) -> bool {
        f32::is_nan(self)
    }
    #[inline(always)]
    fn recip(self) -> Self {
        f32::recip(self)
    }
    #[inline(always)]
    fn round(self) -> Self {
        f32::round(self)
    }
    #[inline(always)]
    fn signum(self) -> Self {
        f32::signum(self)
    }
    #[inline(always)]
    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }
}

impl NumConsts for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
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
    fn abs(self) -> Self {
        f64::abs(self)
    }
    #[inline(always)]
    fn ceil(self) -> Self {
        f64::ceil(self)
    }
    #[inline(always)]
    fn floor(self) -> Self {
        f64::floor(self)
    }
    #[inline(always)]
    fn is_nan(self) -> bool {
        f64::is_nan(self)
    }
    #[inline(always)]
    fn recip(self) -> Self {
        f64::recip(self)
    }
    #[inline(always)]
    fn round(self) -> Self {
        f64::round(self)
    }
    #[inline(always)]
    fn signum(self) -> Self {
        f64::signum(self)
    }
    #[inline(always)]
    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }
}

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
