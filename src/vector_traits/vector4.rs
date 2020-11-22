use crate::{XY, XYZ, XYZW};

const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];

pub trait MaskVectorConsts: Sized + Copy + Clone {
    const FALSE: Self;
}

pub trait MaskVector: MaskVectorConsts {
    fn and(self, other: Self) -> Self;
    fn or(self, other: Self) -> Self;
    fn not(self) -> Self;
}

pub trait MaskVector2: MaskVector {
    fn new(x: bool, y: bool) -> Self;
    fn bitmask(self) -> u32;
    fn any(self) -> bool;
    fn all(self) -> bool;
}

pub trait MaskVector3: MaskVector {
    fn new(x: bool, y: bool, z: bool) -> Self;
    fn bitmask(self) -> u32;
    fn any(self) -> bool;
    fn all(self) -> bool;
}

pub trait MaskVector4: MaskVector {
    fn new(x: bool, y: bool, z: bool, w: bool) -> Self;
    fn bitmask(self) -> u32;
    fn any(self) -> bool;
    fn all(self) -> bool;
}

pub trait VectorConsts: Sized + Copy + Clone {
    const ZERO: Self;
    const ONE: Self;
}

pub trait Vector2Consts: VectorConsts {
    const UNIT_X: Self;
    const UNIT_Y: Self;
}

pub trait Vector3Consts: VectorConsts {
    const UNIT_X: Self;
    const UNIT_Y: Self;
    const UNIT_Z: Self;
}

pub trait Vector4Consts: VectorConsts {
    const UNIT_X: Self;
    const UNIT_Y: Self;
    const UNIT_Z: Self;
    const UNIT_W: Self;
}

pub trait Vector: Sized + Copy + Clone {
    type S: Sized;
    type Mask: Sized;

    fn splat(s: Self::S) -> Self;

    fn select(mask: Self::Mask, a: Self, b: Self) -> Self;

    fn cmpeq(self, other: Self) -> Self::Mask;
    fn cmpne(self, other: Self) -> Self::Mask;
    fn cmpge(self, other: Self) -> Self::Mask;
    fn cmpgt(self, other: Self) -> Self::Mask;
    fn cmple(self, other: Self) -> Self::Mask;
    fn cmplt(self, other: Self) -> Self::Mask;

    fn add(self, other: Self) -> Self;
    fn div(self, other: Self) -> Self;
    fn mul(self, other: Self) -> Self;
    fn mul_add(self, a: Self, b: Self) -> Self;
    fn sub(self, other: Self) -> Self;

    fn scale(self, other: Self::S) -> Self;

    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;

    fn min_element(self) -> Self::S;
    fn max_element(self) -> Self::S;
}

pub trait Vector2: Vector {
    fn new(x: Self::S, y: Self::S) -> Self;
    fn from_slice_unaligned(slice: &[Self::S]) -> Self;
    fn write_to_slice_unaligned(self, slice: &mut [Self::S]);
    fn deref(&self) -> &XY<Self::S>;
    fn deref_mut(&mut self) -> &mut XY<Self::S>;
    fn into_xyz(self, z: Self::S) -> XYZ<Self::S>;
    fn into_xyzw(self, z: Self::S, w: Self::S) -> XYZW<Self::S>;
    fn from_array(a: [Self::S; 2]) -> Self;
    fn into_array(self) -> [Self::S; 2];
    fn from_tuple(t: (Self::S, Self::S)) -> Self;
    fn into_tuple(self) -> (Self::S, Self::S);
}

pub trait Vector3: Vector {
    fn new(x: Self::S, y: Self::S, z: Self::S) -> Self;
    fn from_slice_unaligned(slice: &[Self::S]) -> Self;
    fn write_to_slice_unaligned(self, slice: &mut [Self::S]);
    fn deref(&self) -> &XYZ<Self::S>;
    fn deref_mut(&mut self) -> &mut XYZ<Self::S>;
    fn into_xy(self) -> XY<Self::S>;
    fn into_xyzw(self, w: Self::S) -> XYZW<Self::S>;
    fn from_array(a: [Self::S; 3]) -> Self;
    fn into_array(self) -> [Self::S; 3];
    fn from_tuple(t: (Self::S, Self::S, Self::S)) -> Self;
    fn into_tuple(self) -> (Self::S, Self::S, Self::S);
}

pub trait Vector4: Vector {
    fn new(x: Self::S, y: Self::S, z: Self::S, w: Self::S) -> Self;
    fn from_slice_unaligned(slice: &[Self::S]) -> Self;
    fn write_to_slice_unaligned(self, slice: &mut [Self::S]);
    fn deref(&self) -> &XYZW<Self::S>;
    fn deref_mut(&mut self) -> &mut XYZW<Self::S>;
    fn into_xy(self) -> XY<Self::S>;
    fn into_xyz(self) -> XYZ<Self::S>;
    fn from_array(a: [Self::S; 4]) -> Self;
    fn into_array(self) -> [Self::S; 4];
    fn from_tuple(t: (Self::S, Self::S, Self::S, Self::S)) -> Self;
    fn into_tuple(self) -> (Self::S, Self::S, Self::S, Self::S);
}

pub trait FloatVector: Vector {
    fn abs(self) -> Self;
    fn ceil(self) -> Self;
    fn floor(self) -> Self;
    fn is_nan(self) -> Self::Mask;
    fn neg(self) -> Self;
    fn recip(self) -> Self;
    fn round(self) -> Self;
    fn signum(self) -> Self;
}

pub trait FloatVector3: FloatVector + Vector3 {
    fn dot(self, other: Self) -> Self::S;
    fn dot_into_vec(self, other: Self) -> Self;
    fn cross(self, other: Self) -> Self;
    fn length(self) -> Self::S;
    fn length_recip(self) -> Self::S;
    fn normalize(self) -> Self;
}

pub trait FloatVector4: Sized + FloatVector + Vector4 {
    fn dot(self, other: Self) -> Self::S;
    fn dot_into_vec(self, other: Self) -> Self;
    fn length(self) -> Self::S;
    fn length_recip(self) -> Self::S;
    fn normalize(self) -> Self;
}

pub trait Quaternion: FloatVector4 {
    // fn from_axis_angle(axis: XYZ<Self::S>, angle: Self::S) -> Self;
    // fn to_axis_angle(self) -> (XYZ<Self::S>, Self::S);
    // fn is_near_identity(self) -> bool;
    fn conjugate(self) -> Self;
    fn lerp(self, end: Self, s: Self::S) -> Self;
    fn slerp(self, end: Self, s: Self::S) -> Self;
    fn mul_quaternion(self, other: Self) -> Self;
    // fn rotate_vector<T: FloatVector3>(self, other: T) -> T;
}

mod scalar {
    use super::MASK;
    use crate::scalar_traits::{Float, Num, NumConsts};
    use crate::vector_traits::*;
    use crate::{XY, XYZ, XYZW};

    impl<T> XY<T> {
        #[inline(always)]
        fn map<D, F>(self, f: F) -> XY<D>
        where
            F: Fn(T) -> D,
        {
            XY {
                x: f(self.x),
                y: f(self.y),
            }
        }

        #[inline(always)]
        fn map2<D, F>(self, other: Self, f: F) -> XY<D>
        where
            F: Fn(T, T) -> D,
        {
            XY {
                x: f(self.x, other.x),
                y: f(self.y, other.y),
            }
        }

        #[inline(always)]
        fn map3<D, F>(self, a: Self, b: Self, f: F) -> XY<D>
        where
            F: Fn(T, T, T) -> D,
        {
            XY {
                x: f(self.x, a.x, b.x),
                y: f(self.y, a.y, b.y),
            }
        }
    }

    impl<T> XYZ<T> {
        #[inline(always)]
        fn map<D, F>(self, f: F) -> XYZ<D>
        where
            F: Fn(T) -> D,
        {
            XYZ {
                x: f(self.x),
                y: f(self.y),
                z: f(self.z),
            }
        }

        #[inline(always)]
        fn map2<D, F>(self, other: Self, f: F) -> XYZ<D>
        where
            F: Fn(T, T) -> D,
        {
            XYZ {
                x: f(self.x, other.x),
                y: f(self.y, other.y),
                z: f(self.z, other.z),
            }
        }

        #[inline(always)]
        fn map3<D, F>(self, a: Self, b: Self, f: F) -> XYZ<D>
        where
            F: Fn(T, T, T) -> D,
        {
            XYZ {
                x: f(self.x, a.x, b.x),
                y: f(self.y, a.y, b.y),
                z: f(self.z, a.z, b.z),
            }
        }
    }

    impl<T> XYZW<T> {
        #[inline(always)]
        fn map<D, F>(self, f: F) -> XYZW<D>
        where
            F: Fn(T) -> D,
        {
            XYZW {
                x: f(self.x),
                y: f(self.y),
                z: f(self.z),
                w: f(self.w),
            }
        }

        #[inline(always)]
        fn map2<D, F>(self, other: Self, f: F) -> XYZW<D>
        where
            F: Fn(T, T) -> D,
        {
            XYZW {
                x: f(self.x, other.x),
                y: f(self.y, other.y),
                z: f(self.z, other.z),
                w: f(self.w, other.w),
            }
        }

        #[inline(always)]
        fn map3<D, F>(self, a: Self, b: Self, f: F) -> XYZW<D>
        where
            F: Fn(T, T, T) -> D,
        {
            XYZW {
                x: f(self.x, a.x, b.x),
                y: f(self.y, a.y, b.y),
                z: f(self.z, a.z, b.z),
                w: f(self.w, a.w, b.w),
            }
        }
    }

    impl MaskVectorConsts for XY<u32> {
        const FALSE: Self = Self { x: 0, y: 0 };
    }

    impl MaskVectorConsts for XYZ<u32> {
        const FALSE: Self = Self { x: 0, y: 0, z: 0 };
    }

    impl MaskVectorConsts for XYZW<u32> {
        const FALSE: Self = Self {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
        };
    }

    impl MaskVector for XY<u32> {
        #[inline]
        fn and(self, other: Self) -> Self {
            self.map2(other, |a, b| a & b)
        }

        #[inline]
        fn or(self, other: Self) -> Self {
            self.map2(other, |a, b| a | b)
        }

        #[inline]
        fn not(self) -> Self {
            self.map(|a| !a)
        }
    }

    impl MaskVector for XYZ<u32> {
        #[inline]
        fn and(self, other: Self) -> Self {
            self.map2(other, |a, b| a & b)
        }

        #[inline]
        fn or(self, other: Self) -> Self {
            self.map2(other, |a, b| a | b)
        }

        #[inline]
        fn not(self) -> Self {
            self.map(|a| !a)
        }
    }

    impl MaskVector for XYZW<u32> {
        #[inline]
        fn and(self, other: Self) -> Self {
            self.map2(other, |a, b| a & b)
        }

        #[inline]
        fn or(self, other: Self) -> Self {
            self.map2(other, |a, b| a | b)
        }

        #[inline]
        fn not(self) -> Self {
            self.map(|a| !a)
        }
    }

    impl MaskVector2 for XY<u32> {
        #[inline]
        fn new(x: bool, y: bool) -> Self {
            Self {
                x: MASK[x as usize],
                y: MASK[y as usize],
            }
        }

        #[inline]
        fn bitmask(self) -> u32 {
            (self.x & 0x1) | (self.y & 0x1) << 1
        }

        #[inline]
        fn any(self) -> bool {
            ((self.x | self.y) & 0x1) != 0
        }

        #[inline]
        fn all(self) -> bool {
            ((self.x & self.y) & 0x1) != 0
        }
    }

    impl MaskVector3 for XYZ<u32> {
        #[inline]
        fn new(x: bool, y: bool, z: bool) -> Self {
            // A SSE2 mask can be any bit pattern but for the `Vec3Mask` implementation of select
            // we expect either 0 or 0xff_ff_ff_ff. This should be a safe assumption as this type
            // can only be created via this function or by `Vec3` methods.
            Self {
                x: MASK[x as usize],
                y: MASK[y as usize],
                z: MASK[z as usize],
            }
        }
        #[inline]
        fn bitmask(self) -> u32 {
            (self.x & 0x1) | (self.y & 0x1) << 1 | (self.z & 0x1) << 2
        }

        #[inline]
        fn any(self) -> bool {
            ((self.x | self.y | self.z) & 0x1) != 0
        }

        #[inline]
        fn all(self) -> bool {
            ((self.x & self.y & self.z) & 0x1) != 0
        }
    }

    impl MaskVector4 for XYZW<u32> {
        #[inline]
        fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
            // A SSE2 mask can be any bit pattern but for the `Vec4Mask` implementation of select
            // we expect either 0 or 0xff_ff_ff_ff. This should be a safe assumption as this type
            // can only be created via this function or by `Vec4` methods.
            Self {
                x: MASK[x as usize],
                y: MASK[y as usize],
                z: MASK[z as usize],
                w: MASK[w as usize],
            }
        }

        #[inline]
        fn bitmask(self) -> u32 {
            (self.x & 0x1) | (self.y & 0x1) << 1 | (self.z & 0x1) << 2 | (self.w & 0x1) << 3
        }

        #[inline]
        fn any(self) -> bool {
            ((self.x | self.y | self.z | self.w) & 0x1) != 0
        }

        #[inline]
        fn all(self) -> bool {
            ((self.x & self.y & self.z & self.w) & 0x1) != 0
        }
    }

    impl<T: Float> VectorConsts for XY<T> {
        const ZERO: Self = Self {
            x: <T as NumConsts>::ZERO,
            y: <T as NumConsts>::ZERO,
        };
        const ONE: Self = Self {
            x: <T as NumConsts>::ONE,
            y: <T as NumConsts>::ONE,
        };
    }

    impl<T: Float> Vector2Consts for XY<T> {
        const UNIT_X: Self = Self {
            x: <T as NumConsts>::ONE,
            y: <T as NumConsts>::ZERO,
        };
        const UNIT_Y: Self = Self {
            x: <T as NumConsts>::ZERO,
            y: <T as NumConsts>::ONE,
        };
    }

    impl<T: Float> VectorConsts for XYZ<T> {
        const ZERO: Self = Self {
            x: <T as NumConsts>::ZERO,
            y: <T as NumConsts>::ZERO,
            z: <T as NumConsts>::ZERO,
        };
        const ONE: Self = Self {
            x: <T as NumConsts>::ONE,
            y: <T as NumConsts>::ONE,
            z: <T as NumConsts>::ONE,
        };
    }
    impl<T: Float> Vector3Consts for XYZ<T> {
        const UNIT_X: Self = Self {
            x: <T as NumConsts>::ONE,
            y: <T as NumConsts>::ZERO,
            z: <T as NumConsts>::ZERO,
        };
        const UNIT_Y: Self = Self {
            x: <T as NumConsts>::ZERO,
            y: <T as NumConsts>::ONE,
            z: <T as NumConsts>::ZERO,
        };
        const UNIT_Z: Self = Self {
            x: <T as NumConsts>::ZERO,
            y: <T as NumConsts>::ZERO,
            z: <T as NumConsts>::ONE,
        };
    }

    impl<T: Float> VectorConsts for XYZW<T> {
        const ZERO: Self = Self {
            x: <T as NumConsts>::ZERO,
            y: <T as NumConsts>::ZERO,
            z: <T as NumConsts>::ZERO,
            w: <T as NumConsts>::ZERO,
        };
        const ONE: Self = Self {
            x: <T as NumConsts>::ONE,
            y: <T as NumConsts>::ONE,
            z: <T as NumConsts>::ONE,
            w: <T as NumConsts>::ONE,
        };
    }
    impl<T: Float> Vector4Consts for XYZW<T> {
        const UNIT_X: Self = Self {
            x: <T as NumConsts>::ONE,
            y: <T as NumConsts>::ZERO,
            z: <T as NumConsts>::ZERO,
            w: <T as NumConsts>::ZERO,
        };
        const UNIT_Y: Self = Self {
            x: <T as NumConsts>::ZERO,
            y: <T as NumConsts>::ONE,
            z: <T as NumConsts>::ZERO,
            w: <T as NumConsts>::ZERO,
        };
        const UNIT_Z: Self = Self {
            x: <T as NumConsts>::ZERO,
            y: <T as NumConsts>::ZERO,
            z: <T as NumConsts>::ONE,
            w: <T as NumConsts>::ZERO,
        };
        const UNIT_W: Self = Self {
            x: <T as NumConsts>::ZERO,
            y: <T as NumConsts>::ZERO,
            z: <T as NumConsts>::ZERO,
            w: <T as NumConsts>::ONE,
        };
    }

    impl<T: Num> Vector for XY<T> {
        type S = T;
        type Mask = XY<u32>;

        #[inline]
        fn splat(s: T) -> Self {
            Self { x: s, y: s }
        }

        #[inline]
        fn select(mask: Self::Mask, if_true: Self, if_false: Self) -> Self {
            Self {
                x: if mask.x != 0 { if_true.x } else { if_false.x },
                y: if mask.y != 0 { if_true.y } else { if_false.y },
            }
        }

        #[inline]
        fn cmpeq(self, other: Self) -> Self::Mask {
            self.map2(other, |a, b| MASK[a.eq(&b) as usize])
        }

        #[inline]
        fn cmpne(self, other: Self) -> Self::Mask {
            self.map2(other, |a, b| MASK[a.ne(&b) as usize])
        }

        #[inline]
        fn cmpge(self, other: Self) -> Self::Mask {
            self.map2(other, |a, b| MASK[a.ge(&b) as usize])
        }

        #[inline]
        fn cmpgt(self, other: Self) -> Self::Mask {
            self.map2(other, |a, b| MASK[a.gt(&b) as usize])
        }

        #[inline]
        fn cmple(self, other: Self) -> Self::Mask {
            self.map2(other, |a, b| MASK[a.le(&b) as usize])
        }

        #[inline]
        fn cmplt(self, other: Self) -> Self::Mask {
            self.map2(other, |a, b| MASK[a.lt(&b) as usize])
        }

        #[inline]
        fn add(self, other: Self) -> Self {
            self.map2(other, |a, b| a + b)
        }

        #[inline]
        fn div(self, other: Self) -> Self {
            self.map2(other, |a, b| a / b)
        }

        #[inline]
        fn mul(self, other: Self) -> Self {
            self.map2(other, |a, b| a * b)
        }

        #[inline]
        fn mul_add(self, b: Self, c: Self) -> Self {
            self.map3(b, c, |a, b, c| a * b + c)
        }

        #[inline]
        fn sub(self, other: Self) -> Self {
            self.map2(other, |a, b| a - b)
        }

        fn scale(self, other: Self::S) -> Self {
            self.map(|a| a * other)
        }

        #[inline]
        fn min(self, other: Self) -> Self {
            self.map2(other, |a, b| a.min(b))
        }

        #[inline]
        fn max(self, other: Self) -> Self {
            self.map2(other, |a, b| a.max(b))
        }

        #[inline]
        fn min_element(self) -> Self::S {
            self.x.min(self.y)
        }

        #[inline]
        fn max_element(self) -> Self::S {
            self.x.max(self.y)
        }
    }

    impl<T: Num> Vector for XYZ<T> {
        type S = T;
        type Mask = XYZ<u32>;

        #[inline]
        fn splat(s: T) -> Self {
            Self { x: s, y: s, z: s }
        }

        #[inline]
        fn select(mask: Self::Mask, if_true: Self, if_false: Self) -> Self {
            Self {
                x: if mask.x != 0 { if_true.x } else { if_false.x },
                y: if mask.y != 0 { if_true.y } else { if_false.y },
                z: if mask.z != 0 { if_true.z } else { if_false.z },
            }
        }

        #[inline]
        fn cmpeq(self, other: Self) -> Self::Mask {
            self.map2(other, |a, b| MASK[a.eq(&b) as usize])
        }

        #[inline]
        fn cmpne(self, other: Self) -> Self::Mask {
            self.map2(other, |a, b| MASK[a.ne(&b) as usize])
        }

        #[inline]
        fn cmpge(self, other: Self) -> Self::Mask {
            self.map2(other, |a, b| MASK[a.ge(&b) as usize])
        }

        #[inline]
        fn cmpgt(self, other: Self) -> Self::Mask {
            self.map2(other, |a, b| MASK[a.gt(&b) as usize])
        }

        #[inline]
        fn cmple(self, other: Self) -> Self::Mask {
            self.map2(other, |a, b| MASK[a.le(&b) as usize])
        }

        #[inline]
        fn cmplt(self, other: Self) -> Self::Mask {
            self.map2(other, |a, b| MASK[a.lt(&b) as usize])
        }

        #[inline]
        fn add(self, other: Self) -> Self {
            self.map2(other, |a, b| a + b)
        }

        #[inline]
        fn div(self, other: Self) -> Self {
            self.map2(other, |a, b| a / b)
        }

        #[inline]
        fn mul(self, other: Self) -> Self {
            self.map2(other, |a, b| a * b)
        }

        #[inline]
        fn mul_add(self, b: Self, c: Self) -> Self {
            self.map3(b, c, |a, b, c| a * b + c)
        }

        #[inline]
        fn sub(self, other: Self) -> Self {
            self.map2(other, |a, b| a - b)
        }

        fn scale(self, other: Self::S) -> Self {
            self.map(|a| a * other)
        }

        #[inline]
        fn min(self, other: Self) -> Self {
            self.map2(other, |a, b| a.min(b))
        }

        #[inline]
        fn max(self, other: Self) -> Self {
            self.map2(other, |a, b| a.max(b))
        }

        #[inline]
        fn min_element(self) -> Self::S {
            self.x.min(self.y.min(self.z))
        }

        #[inline]
        fn max_element(self) -> Self::S {
            self.x.max(self.y.max(self.z))
        }
    }

    impl<T: Num> Vector for XYZW<T> {
        type S = T;
        type Mask = XYZW<u32>;

        #[inline]
        fn splat(s: T) -> Self {
            Self {
                x: s,
                y: s,
                z: s,
                w: s,
            }
        }

        #[inline]
        fn select(mask: Self::Mask, if_true: Self, if_false: Self) -> Self {
            Self {
                x: if mask.x != 0 { if_true.x } else { if_false.x },
                y: if mask.y != 0 { if_true.y } else { if_false.y },
                z: if mask.z != 0 { if_true.z } else { if_false.z },
                w: if mask.w != 0 { if_true.w } else { if_false.w },
            }
        }

        #[inline]
        fn cmpeq(self, other: Self) -> Self::Mask {
            self.map2(other, |a, b| MASK[a.eq(&b) as usize])
        }

        #[inline]
        fn cmpne(self, other: Self) -> Self::Mask {
            self.map2(other, |a, b| MASK[a.ne(&b) as usize])
        }

        #[inline]
        fn cmpge(self, other: Self) -> Self::Mask {
            self.map2(other, |a, b| MASK[a.ge(&b) as usize])
        }

        #[inline]
        fn cmpgt(self, other: Self) -> Self::Mask {
            self.map2(other, |a, b| MASK[a.gt(&b) as usize])
        }

        #[inline]
        fn cmple(self, other: Self) -> Self::Mask {
            self.map2(other, |a, b| MASK[a.le(&b) as usize])
        }

        #[inline]
        fn cmplt(self, other: Self) -> Self::Mask {
            self.map2(other, |a, b| MASK[a.lt(&b) as usize])
        }

        #[inline]
        fn add(self, other: Self) -> Self {
            self.map2(other, |a, b| a + b)
        }

        #[inline]
        fn div(self, other: Self) -> Self {
            self.map2(other, |a, b| a / b)
        }

        #[inline]
        fn mul(self, other: Self) -> Self {
            self.map2(other, |a, b| a * b)
        }

        #[inline]
        fn mul_add(self, b: Self, c: Self) -> Self {
            self.map3(b, c, |a, b, c| a * b + c)
        }

        #[inline]
        fn sub(self, other: Self) -> Self {
            self.map2(other, |a, b| a - b)
        }

        fn scale(self, other: Self::S) -> Self {
            self.map(|a| a * other)
        }

        #[inline]
        fn min(self, other: Self) -> Self {
            self.map2(other, |a, b| a.min(b))
        }

        #[inline]
        fn max(self, other: Self) -> Self {
            self.map2(other, |a, b| a.max(b))
        }

        #[inline]
        fn min_element(self) -> Self::S {
            self.x.min(self.y.min(self.z.min(self.w)))
        }

        #[inline]
        fn max_element(self) -> Self::S {
            self.x.max(self.y.max(self.z.min(self.w)))
        }
    }

    impl<T: Num> Vector2 for XY<T> {
        #[inline]
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }

        #[inline]
        fn from_slice_unaligned(slice: &[Self::S]) -> Self {
            Self {
                x: slice[0],
                y: slice[1],
            }
        }

        #[inline]
        fn write_to_slice_unaligned(self, slice: &mut [Self::S]) {
            slice[0] = self.x;
            slice[1] = self.y;
        }

        #[inline]
        fn deref(&self) -> &XY<Self::S> {
            self
        }

        #[inline]
        fn deref_mut(&mut self) -> &mut XY<Self::S> {
            self
        }

        #[inline]
        fn into_xyz(self, z: Self::S) -> XYZ<Self::S> {
            XYZ {
                x: self.x,
                y: self.y,
                z,
            }
        }

        #[inline]
        fn into_xyzw(self, z: Self::S, w: Self::S) -> XYZW<Self::S> {
            XYZW {
                x: self.x,
                y: self.y,
                z,
                w,
            }
        }

        #[inline]
        fn from_array(a: [Self::S; 2]) -> Self {
            Self { x: a[0], y: a[1] }
        }

        #[inline]
        fn into_array(self) -> [Self::S; 2] {
            [self.x, self.y]
        }

        #[inline]
        fn from_tuple(t: (Self::S, Self::S)) -> Self {
            Self::new(t.0, t.1)
        }

        #[inline]
        fn into_tuple(self) -> (Self::S, Self::S) {
            (self.x, self.y)
        }
    }

    impl<T: Num> Vector3 for XYZ<T> {
        #[inline]
        fn new(x: T, y: T, z: T) -> Self {
            Self { x, y, z }
        }

        #[inline]
        fn from_slice_unaligned(slice: &[Self::S]) -> Self {
            Self {
                x: slice[0],
                y: slice[1],
                z: slice[2],
            }
        }

        #[inline]
        fn write_to_slice_unaligned(self, slice: &mut [Self::S]) {
            slice[0] = self.x;
            slice[1] = self.y;
            slice[2] = self.z;
        }

        #[inline]
        fn deref(&self) -> &XYZ<Self::S> {
            self
        }

        #[inline]
        fn deref_mut(&mut self) -> &mut XYZ<Self::S> {
            self
        }

        #[inline]
        fn into_xy(self) -> XY<Self::S> {
            XY {
                x: self.x,
                y: self.y,
            }
        }

        #[inline]
        fn into_xyzw(self, w: Self::S) -> XYZW<Self::S> {
            XYZW {
                x: self.x,
                y: self.y,
                z: self.z,
                w,
            }
        }

        #[inline]
        fn from_array(a: [Self::S; 3]) -> Self {
            Self {
                x: a[0],
                y: a[1],
                z: a[2],
            }
        }

        #[inline]
        fn into_array(self) -> [Self::S; 3] {
            [self.x, self.y, self.z]
        }

        #[inline]
        fn from_tuple(t: (Self::S, Self::S, Self::S)) -> Self {
            Self::new(t.0, t.1, t.2)
        }

        #[inline]
        fn into_tuple(self) -> (Self::S, Self::S, Self::S) {
            (self.x, self.y, self.z)
        }
    }

    impl<T: Num> Vector4 for XYZW<T> {
        #[inline]
        fn new(x: T, y: T, z: T, w: T) -> Self {
            Self { x, y, z, w }
        }

        #[inline]
        fn from_slice_unaligned(slice: &[Self::S]) -> Self {
            Self {
                x: slice[0],
                y: slice[1],
                z: slice[2],
                w: slice[3],
            }
        }

        #[inline]
        fn write_to_slice_unaligned(self, slice: &mut [Self::S]) {
            slice[0] = self.x;
            slice[1] = self.y;
            slice[2] = self.z;
            slice[3] = self.w;
        }

        #[inline]
        fn deref(&self) -> &XYZW<Self::S> {
            self
        }

        #[inline]
        fn deref_mut(&mut self) -> &mut XYZW<Self::S> {
            self
        }

        #[inline]
        fn into_xy(self) -> XY<Self::S> {
            XY {
                x: self.x,
                y: self.y,
            }
        }

        #[inline]
        fn into_xyz(self) -> XYZ<Self::S> {
            XYZ {
                x: self.x,
                y: self.y,
                z: self.z,
            }
        }

        #[inline]
        fn from_array(a: [Self::S; 4]) -> Self {
            Self {
                x: a[0],
                y: a[1],
                z: a[2],
                w: a[3],
            }
        }

        #[inline]
        fn into_array(self) -> [Self::S; 4] {
            [self.x, self.y, self.z, self.w]
        }

        #[inline]
        fn from_tuple(t: (Self::S, Self::S, Self::S, Self::S)) -> Self {
            Self::new(t.0, t.1, t.2, t.3)
        }

        #[inline]
        fn into_tuple(self) -> (Self::S, Self::S, Self::S, Self::S) {
            (self.x, self.y, self.z, self.w)
        }
    }

    impl<T: Float> FloatVector for XY<T> {
        #[inline]
        fn is_nan(self) -> Self::Mask {
            self.map(|a| MASK[a.is_nan() as usize])
        }

        #[inline]
        fn abs(self) -> Self {
            self.map(Float::abs)
        }

        #[inline]
        fn floor(self) -> Self {
            self.map(Float::floor)
        }

        #[inline]
        fn ceil(self) -> Self {
            self.map(Float::ceil)
        }

        #[inline]
        fn round(self) -> Self {
            self.map(Float::round)
        }

        #[inline]
        fn neg(self) -> Self {
            self.map(|a| a.neg())
        }

        #[inline]
        fn recip(self) -> Self {
            self.map(Float::recip)
        }

        #[inline]
        fn signum(self) -> Self {
            self.map(Float::signum)
        }
    }

    impl<T: Float> FloatVector for XYZ<T> {
        #[inline]
        fn is_nan(self) -> Self::Mask {
            self.map(|a| MASK[a.is_nan() as usize])
        }

        #[inline]
        fn abs(self) -> Self {
            self.map(Float::abs)
        }

        #[inline]
        fn floor(self) -> Self {
            self.map(Float::floor)
        }

        #[inline]
        fn ceil(self) -> Self {
            self.map(Float::ceil)
        }

        #[inline]
        fn round(self) -> Self {
            self.map(Float::round)
        }

        #[inline]
        fn neg(self) -> Self {
            self.map(|a| a.neg())
        }

        #[inline]
        fn recip(self) -> Self {
            self.map(Float::recip)
        }

        #[inline]
        fn signum(self) -> Self {
            self.map(Float::signum)
        }
    }

    impl<T: Float> FloatVector for XYZW<T> {
        #[inline]
        fn is_nan(self) -> Self::Mask {
            self.map(|a| MASK[a.is_nan() as usize])
        }

        #[inline]
        fn abs(self) -> Self {
            self.map(Float::abs)
        }

        #[inline]
        fn floor(self) -> Self {
            self.map(Float::floor)
        }

        #[inline]
        fn ceil(self) -> Self {
            self.map(Float::ceil)
        }

        #[inline]
        fn round(self) -> Self {
            self.map(Float::round)
        }

        #[inline]
        fn neg(self) -> Self {
            self.map(|a| a.neg())
        }

        #[inline]
        fn recip(self) -> Self {
            self.map(Float::recip)
        }

        #[inline]
        fn signum(self) -> Self {
            self.map(Float::signum)
        }
    }

    impl<T: Float> FloatVector3 for XYZ<T> {
        #[inline]
        fn dot(self, other: Self) -> Self::S {
            (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
        }

        #[inline]
        fn dot_into_vec(self, other: Self) -> Self {
            Self::splat(self.dot(other))
        }

        #[inline]
        fn cross(self, other: Self) -> Self {
            Self {
                x: self.y * other.z - other.y * self.z,
                y: self.z * other.x - other.z * self.x,
                z: self.x * other.y - other.x * self.y,
            }
        }

        #[inline]
        fn length(self) -> Self::S {
            self.dot(self).sqrt()
        }

        #[inline]
        fn length_recip(self) -> Self::S {
            self.length().recip()
        }

        #[inline]
        fn normalize(self) -> Self {
            self.scale(self.length_recip())
        }
    }

    impl<T: Float> FloatVector4 for XYZW<T> {
        #[inline]
        fn dot(self, other: Self) -> Self::S {
            (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
        }

        #[inline]
        fn dot_into_vec(self, other: Self) -> Self {
            Self::splat(self.dot(other))
        }

        #[inline]
        fn length(self) -> Self::S {
            self.dot(self).sqrt()
        }

        #[inline]
        fn length_recip(self) -> Self::S {
            self.length().recip()
        }

        #[inline]
        fn normalize(self) -> Self {
            self.scale(self.length_recip())
        }
    }

    impl Quaternion for XYZW<f32> {
        // fn from_axis_angle(axis: XYZ<Self::S>, angle: Self::S) -> Self {
        // }

        // fn to_axis_angle(self) -> (XYZ<Self::S>, Self::S) {
        // }

        //fn is_near_identity(self) -> bool {
        //    // Based on https://github.com/nfrechette/rtm `rtm::quat_near_identity`
        //    const THRESHOLD_ANGLE: f32 = 0.002_847_144_6;
        //    // Because of floating point precision, we cannot represent very small rotations.
        //    // The closest f32 to 1.0 that is not 1.0 itself yields:
        //    // 0.99999994.acos() * 2.0  = 0.000690533954 rad
        //    //
        //    // An error threshold of 1.e-6 is used by default.
        //    // (1.0 - 1.e-6).acos() * 2.0 = 0.00284714461 rad
        //    // (1.0 - 1.e-7).acos() * 2.0 = 0.00097656250 rad
        //    //
        //    // We don't really care about the angle value itself, only if it's close to 0.
        //    // This will happen whenever quat.w is close to 1.0.
        //    // If the quat.w is close to -1.0, the angle will be near 2*PI which is close to
        //    // a negative 0 rotation. By forcing quat.w to be positive, we'll end up with
        //    // the shortest path.
        //    let positive_w_angle = self.w.abs().acos_approx() * 2.0;
        //    positive_w_angle < THRESHOLD_ANGLE
        //}

        fn conjugate(self) -> Self {
            Self::new(-self.x, -self.y, -self.z, self.w)
        }

        fn lerp(self, end: Self, s: Self::S) -> Self {
            glam_assert!(self.is_normalized());
            glam_assert!(end.is_normalized());

            let start = self;
            let end = end;
            let dot = start.dot(end);
            let bias = if dot >= 0.0 { 1.0 } else { -1.0 };
            let interpolated = start.add(end.scale(bias).sub(start).scale(s));
            interpolated.normalize()
        }

        fn slerp(self, end: Self, s: Self::S) -> Self {
            // http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/

            glam_assert!(self.is_normalized());
            glam_assert!(end.is_normalized());

            const DOT_THRESHOLD: f32 = 0.9995;

            let dot = self.dot(end);

            if dot > DOT_THRESHOLD {
                // assumes lerp returns a normalized quaternion
                self.lerp(end, s)
            } else {
                // assumes scalar_acos clamps the input to [-1.0, 1.0]
                let theta = dot.acos_approx();
                let scale1 = f32::sin(theta * (1.0 - s));
                let scale2 = f32::sin(theta * s);
                let theta_sin = f32::sin(theta);

                self.scale(scale1)
                    .add(end.scale(scale2))
                    .scale(theta_sin.recip())
            }
        }

        // fn rotate_vector<T: FloatVector3>(self, other: T) -> T {
        //     let w = self.w;
        //     let b = XYZ { x: self.x, y: self.y, z: self.z };
        //     let b2 = b.dot(b);
        //     other * (w * w - b2) + b * (other.dot(b) * 2.0) + b.cross(other) * (w * 2.0)
        // }

        fn mul_quaternion(self, other: Self) -> Self {
            glam_assert!(self.is_normalized());
            glam_assert!(other.is_normalized());
            let (x0, y0, z0, w0) = self.into_tuple();
            let (x1, y1, z1, w1) = other.into_tuple();
            Self::new(
                w0 * x1 + x0 * w1 + y0 * z1 - z0 * y1,
                w0 * y1 - x0 * z1 + y0 * w1 + z0 * x1,
                w0 * z1 + x0 * y1 - y0 * x1 + z0 * w1,
                w0 * w1 - x0 * x1 - y0 * y1 - z0 * z1,
            )
        }
    }

    impl<T> From<XYZW<T>> for XYZ<T> {
        #[inline]
        fn from(v: XYZW<T>) -> Self {
            Self {
                x: v.x,
                y: v.y,
                z: v.z,
            }
        }
    }

    impl<T> From<XYZW<T>> for XY<T> {
        #[inline]
        fn from(v: XYZW<T>) -> Self {
            Self { x: v.x, y: v.y }
        }
    }

    impl<T> From<XYZ<T>> for XY<T> {
        #[inline]
        fn from(v: XYZ<T>) -> Self {
            Self { x: v.x, y: v.y }
        }
    }
}

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
mod sse2 {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;

    use super::MASK;
    use crate::vector_traits::*;
    use crate::Align16;
    use crate::{const_m128, XY, XYZ, XYZW};
    use core::mem::MaybeUninit;

    impl MaskVectorConsts for __m128 {
        const FALSE: __m128 = const_m128!([0.0; 4]);
    }

    impl MaskVector for __m128 {
        #[inline]
        fn and(self, other: Self) -> Self {
            unsafe { _mm_and_ps(self, other) }
        }

        #[inline]
        fn or(self, other: Self) -> Self {
            unsafe { _mm_or_ps(self, other) }
        }

        #[inline]
        fn not(self) -> Self {
            unsafe { _mm_andnot_ps(self, _mm_set_ps1(f32::from_bits(0xff_ff_ff_ff))) }
        }
    }

    impl MaskVector3 for __m128 {
        #[inline]
        fn new(x: bool, y: bool, z: bool) -> Self {
            // A SSE2 mask can be any bit pattern but for the `MaskVector3` implementation of select we
            // expect either 0 or 0xff_ff_ff_ff. This should be a safe assumption as this type can only
            // be created via this function or by `Vector3` methods.

            unsafe {
                _mm_set_ps(
                    0.0,
                    f32::from_bits(MASK[z as usize]),
                    f32::from_bits(MASK[y as usize]),
                    f32::from_bits(MASK[x as usize]),
                )
            }
        }

        #[inline]
        fn bitmask(self) -> u32 {
            unsafe { (_mm_movemask_ps(self) as u32) & 0x7 }
        }

        #[inline]
        fn any(self) -> bool {
            unsafe { (_mm_movemask_ps(self) & 0x7) != 0 }
        }

        #[inline]
        fn all(self) -> bool {
            unsafe { (_mm_movemask_ps(self) & 0x7) == 0x7 }
        }
    }

    impl MaskVector4 for __m128 {
        #[inline]
        fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
            // A SSE2 mask can be any bit pattern but for the `Vec4Mask` implementation of select we
            // expect either 0 or 0xff_ff_ff_ff. This should be a safe assumption as this type can only
            // be created via this function or by `Vec4` methods.

            const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];
            unsafe {
                _mm_set_ps(
                    f32::from_bits(MASK[w as usize]),
                    f32::from_bits(MASK[z as usize]),
                    f32::from_bits(MASK[y as usize]),
                    f32::from_bits(MASK[x as usize]),
                )
            }
        }
        #[inline]
        fn bitmask(self) -> u32 {
            unsafe { _mm_movemask_ps(self) as u32 }
        }

        #[inline]
        fn any(self) -> bool {
            unsafe { _mm_movemask_ps(self) != 0 }
        }

        #[inline]
        fn all(self) -> bool {
            unsafe { _mm_movemask_ps(self) == 0xf }
        }
    }

    /// Calculates the vector 3 dot product and returns answer in x lane of __m128.
    #[inline]
    unsafe fn dot3_in_x(lhs: __m128, rhs: __m128) -> __m128 {
        let x2_y2_z2_w2 = _mm_mul_ps(lhs, rhs);
        let y2_0_0_0 = _mm_shuffle_ps(x2_y2_z2_w2, x2_y2_z2_w2, 0b00_00_00_01);
        let z2_0_0_0 = _mm_shuffle_ps(x2_y2_z2_w2, x2_y2_z2_w2, 0b00_00_00_10);
        let x2y2_0_0_0 = _mm_add_ss(x2_y2_z2_w2, y2_0_0_0);
        _mm_add_ss(x2y2_0_0_0, z2_0_0_0)
    }

    /// Calculates the vector 4 dot product and returns answer in x lane of __m128.
    #[inline]
    unsafe fn dot4_in_x(lhs: __m128, rhs: __m128) -> __m128 {
        let x2_y2_z2_w2 = _mm_mul_ps(lhs, rhs);
        let z2_w2_0_0 = _mm_shuffle_ps(x2_y2_z2_w2, x2_y2_z2_w2, 0b00_00_11_10);
        let x2z2_y2w2_0_0 = _mm_add_ps(x2_y2_z2_w2, z2_w2_0_0);
        let y2w2_0_0_0 = _mm_shuffle_ps(x2z2_y2w2_0_0, x2z2_y2w2_0_0, 0b00_00_00_01);
        _mm_add_ps(x2z2_y2w2_0_0, y2w2_0_0_0)
    }

    impl VectorConsts for __m128 {
        const ZERO: __m128 = const_m128!([0.0; 4]);
        const ONE: __m128 = const_m128!([1.0; 4]);
    }

    impl Vector3Consts for __m128 {
        const UNIT_X: __m128 = const_m128!([1.0, 0.0, 0.0, 0.0]);
        const UNIT_Y: __m128 = const_m128!([0.0, 1.0, 0.0, 0.0]);
        const UNIT_Z: __m128 = const_m128!([0.0, 0.0, 1.0, 0.0]);
    }

    impl Vector4Consts for __m128 {
        const UNIT_X: __m128 = const_m128!([1.0, 0.0, 0.0, 0.0]);
        const UNIT_Y: __m128 = const_m128!([0.0, 1.0, 0.0, 0.0]);
        const UNIT_Z: __m128 = const_m128!([0.0, 0.0, 1.0, 0.0]);
        const UNIT_W: __m128 = const_m128!([0.0, 0.0, 0.0, 1.0]);
    }

    impl Vector for __m128 {
        type S = f32;
        type Mask = __m128;

        #[inline]
        fn splat(s: f32) -> Self {
            unsafe { _mm_set_ps1(s) }
        }

        #[inline]
        fn select(mask: Self::Mask, if_true: Self, if_false: Self) -> Self {
            unsafe { _mm_or_ps(_mm_andnot_ps(mask, if_false), _mm_and_ps(if_true, mask)) }
        }

        #[inline]
        fn cmpeq(self, other: Self) -> Self::Mask {
            unsafe { _mm_cmpeq_ps(self, other) }
        }

        #[inline]
        fn cmpne(self, other: Self) -> Self::Mask {
            unsafe { _mm_cmpneq_ps(self, other) }
        }

        #[inline]
        fn cmpge(self, other: Self) -> Self::Mask {
            unsafe { _mm_cmpge_ps(self, other) }
        }

        #[inline]
        fn cmpgt(self, other: Self) -> Self::Mask {
            unsafe { _mm_cmpgt_ps(self, other) }
        }

        #[inline]
        fn cmple(self, other: Self) -> Self::Mask {
            unsafe { _mm_cmple_ps(self, other) }
        }

        #[inline]
        fn cmplt(self, other: Self) -> Self::Mask {
            unsafe { _mm_cmplt_ps(self, other) }
        }

        #[inline]
        fn add(self, other: Self) -> Self {
            unsafe { _mm_add_ps(self, other) }
        }

        #[inline]
        fn div(self, other: Self) -> Self {
            unsafe { _mm_div_ps(self, other) }
        }

        #[inline]
        fn mul(self, other: Self) -> Self {
            unsafe { _mm_mul_ps(self, other) }
        }

        #[inline]
        fn mul_add(self, a: Self, b: Self) -> Self {
            unsafe { _mm_add_ps(_mm_mul_ps(self, a), b) }
        }

        #[inline]
        fn sub(self, other: Self) -> Self {
            unsafe { _mm_sub_ps(self, other) }
        }

        #[inline]
        fn scale(self, other: Self::S) -> Self {
            unsafe { _mm_mul_ps(self, _mm_set_ps1(other)) }
        }

        #[inline]
        fn min(self, other: Self) -> Self {
            unsafe { _mm_min_ps(self, other) }
        }

        #[inline]
        fn max(self, other: Self) -> Self {
            unsafe { _mm_max_ps(self, other) }
        }

        #[inline]
        fn min_element(self) -> Self::S {
            unsafe {
                let v = self;
                let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b00_00_11_10));
                let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
                _mm_cvtss_f32(v)
            }
        }

        #[inline]
        fn max_element(self) -> Self::S {
            unsafe {
                let v = self;
                let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_11_10));
                let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
                _mm_cvtss_f32(v)
            }
        }
    }

    impl Vector3 for __m128 {
        #[inline]
        fn new(x: f32, y: f32, z: f32) -> Self {
            unsafe { _mm_set_ps(0.0, z, y, x) }
        }

        #[inline]
        fn from_slice_unaligned(slice: &[Self::S]) -> Self {
            Vector3::new(slice[0], slice[1], slice[2])
        }

        #[inline]
        fn write_to_slice_unaligned(self, slice: &mut [Self::S]) {
            let xyz = Vector3::deref(&self);
            slice[0] = xyz.x;
            slice[1] = xyz.y;
            slice[2] = xyz.z;
        }

        #[inline]
        fn deref(&self) -> &XYZ<Self::S> {
            unsafe { &*(self as *const Self as *const XYZ<Self::S>) }
        }

        #[inline]
        fn deref_mut(&mut self) -> &mut XYZ<Self::S> {
            unsafe { &mut *(self as *mut Self as *mut XYZ<Self::S>) }
        }

        #[inline]
        fn into_xy(self) -> XY<f32> {
            let mut out: MaybeUninit<Align16<XY<f32>>> = MaybeUninit::uninit();
            unsafe {
                _mm_store_ps(out.as_mut_ptr() as *mut f32, self);
                out.assume_init().0
            }
        }

        #[inline]
        fn into_xyzw(self, w: Self::S) -> XYZW<f32> {
            unsafe {
                let mut t = _mm_move_ss(self, _mm_set_ss(w));
                t = _mm_shuffle_ps(t, t, 0b00_10_01_00);
                // TODO: need a SIMD path
                *Vector4::deref(&_mm_move_ss(t, self))
            }
        }

        #[inline]
        fn from_array(a: [Self::S; 3]) -> Self {
            unsafe { _mm_loadu_ps(a.as_ptr()) }
        }

        #[inline]
        fn into_array(self) -> [Self::S; 3] {
            let mut out: MaybeUninit<Align16<[f32; 3]>> = MaybeUninit::uninit();
            unsafe {
                _mm_store_ps(out.as_mut_ptr() as *mut f32, self);
                out.assume_init().0
            }
        }

        #[inline]
        fn from_tuple(t: (Self::S, Self::S, Self::S)) -> Self {
            unsafe { _mm_set_ps(0.0, t.2, t.1, t.0) }
        }

        #[inline]
        fn into_tuple(self) -> (Self::S, Self::S, Self::S) {
            let mut out: MaybeUninit<Align16<(f32, f32, f32)>> = MaybeUninit::uninit();
            unsafe {
                _mm_store_ps(out.as_mut_ptr() as *mut f32, self);
                out.assume_init().0
            }
        }
    }

    impl Vector4 for __m128 {
        #[inline]
        fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
            unsafe { _mm_set_ps(w, z, y, x) }
        }

        #[inline]
        fn from_slice_unaligned(slice: &[Self::S]) -> Self {
            assert!(slice.len() >= 4);
            unsafe { _mm_loadu_ps(slice.as_ptr()) }
        }

        #[inline]
        fn write_to_slice_unaligned(self, slice: &mut [Self::S]) {
            unsafe {
                assert!(slice.len() >= 4);
                _mm_storeu_ps(slice.as_mut_ptr(), self);
            }
        }

        #[inline]
        fn deref(&self) -> &XYZW<Self::S> {
            unsafe { &*(self as *const Self as *const XYZW<Self::S>) }
        }

        #[inline]
        fn deref_mut(&mut self) -> &mut XYZW<Self::S> {
            unsafe { &mut *(self as *mut Self as *mut XYZW<Self::S>) }
        }

        #[inline]
        fn into_xy(self) -> XY<f32> {
            let mut out: MaybeUninit<Align16<XY<f32>>> = MaybeUninit::uninit();
            unsafe {
                _mm_store_ps(out.as_mut_ptr() as *mut f32, self);
                out.assume_init().0
            }
        }

        #[inline]
        fn into_xyz(self) -> XYZ<f32> {
            let mut out: MaybeUninit<Align16<XYZ<f32>>> = MaybeUninit::uninit();
            unsafe {
                _mm_store_ps(out.as_mut_ptr() as *mut f32, self);
                out.assume_init().0
            }
        }

        #[inline]
        fn from_array(a: [Self::S; 4]) -> Self {
            unsafe { _mm_loadu_ps(a.as_ptr()) }
        }

        #[inline]
        fn into_array(self) -> [Self::S; 4] {
            let mut out: MaybeUninit<Align16<[f32; 4]>> = MaybeUninit::uninit();
            unsafe {
                _mm_store_ps(out.as_mut_ptr() as *mut f32, self);
                out.assume_init().0
            }
        }

        #[inline]
        fn from_tuple(t: (Self::S, Self::S, Self::S, Self::S)) -> Self {
            unsafe { _mm_set_ps(t.3, t.2, t.1, t.0) }
        }

        #[inline]
        fn into_tuple(self) -> (Self::S, Self::S, Self::S, Self::S) {
            let mut out: MaybeUninit<Align16<(f32, f32, f32, f32)>> = MaybeUninit::uninit();
            unsafe {
                _mm_store_ps(out.as_mut_ptr() as *mut f32, self);
                out.assume_init().0
            }
        }
    }

    impl FloatVector for __m128 {
        #[inline]
        fn is_nan(self) -> Self::Mask {
            unsafe { _mm_cmpunord_ps(self, self) }
        }

        #[inline]
        fn abs(self) -> Self {
            unsafe { _mm_and_ps(self, _mm_castsi128_ps(_mm_set1_epi32(0x7f_ff_ff_ff))) }
        }

        #[inline]
        fn round(self) -> Self {
            unsafe {
                use crate::f32::funcs::sse2::m128_round;
                m128_round(self)
            }
        }

        #[inline]
        fn floor(self) -> Self {
            unsafe {
                use crate::f32::funcs::sse2::m128_floor;
                m128_floor(self)
            }
        }

        #[inline]
        fn ceil(self) -> Self {
            unsafe {
                use crate::f32::funcs::sse2::m128_ceil;
                m128_ceil(self)
            }
        }

        #[inline]
        fn recip(self) -> Self {
            unsafe { _mm_div_ps(Self::ONE, self) }
        }

        #[inline]
        fn signum(self) -> Self {
            const NEG_ONE: __m128 = const_m128!([-1.0; 4]);
            let mask = self.cmpge(Self::ZERO);
            let result = Self::select(mask, Self::ONE, NEG_ONE);
            Self::select(self.is_nan(), self, result)
        }

        #[inline]
        fn neg(self) -> Self {
            unsafe { _mm_sub_ps(Self::ZERO, self) }
        }
    }

    impl FloatVector3 for __m128 {
        #[inline]
        fn dot(self, other: Self) -> f32 {
            unsafe { _mm_cvtss_f32(dot3_in_x(self, other)) }
        }

        #[inline]
        fn dot_into_vec(self, other: Self) -> Self {
            unsafe {
                let dot_in_x = dot3_in_x(self, other);
                _mm_shuffle_ps(dot_in_x, dot_in_x, 0b00_00_00_00)
            }
        }

        #[inline]
        fn cross(self, other: Self) -> Self {
            unsafe {
                // x  <-  a.y*b.z - a.z*b.y
                // y  <-  a.z*b.x - a.x*b.z
                // z  <-  a.x*b.y - a.y*b.x
                // We can save a shuffle by grouping it in this wacky order:
                // (self.zxy() * other - self * other.zxy()).zxy()
                let lhszxy = _mm_shuffle_ps(self, self, 0b01_01_00_10);
                let rhszxy = _mm_shuffle_ps(other, other, 0b01_01_00_10);
                let lhszxy_rhs = _mm_mul_ps(lhszxy, other);
                let rhszxy_lhs = _mm_mul_ps(rhszxy, self);
                let sub = _mm_sub_ps(lhszxy_rhs, rhszxy_lhs);
                _mm_shuffle_ps(sub, sub, 0b01_01_00_10)
            }
        }

        #[inline]
        fn length(self) -> f32 {
            unsafe {
                let dot = dot3_in_x(self, self);
                _mm_cvtss_f32(_mm_sqrt_ps(dot))
            }
        }

        #[inline]
        fn length_recip(self) -> f32 {
            unsafe {
                let dot = dot3_in_x(self, self);
                // _mm_rsqrt_ps is lower precision
                _mm_cvtss_f32(_mm_div_ps(Self::ONE, _mm_sqrt_ps(dot)))
            }
        }

        #[inline]
        fn normalize(self) -> Self {
            unsafe {
                let dot = FloatVector3::dot_into_vec(self, self);
                _mm_div_ps(self, _mm_sqrt_ps(dot))
            }
        }
    }

    impl FloatVector4 for __m128 {
        #[inline]
        fn dot(self, other: Self) -> f32 {
            unsafe { _mm_cvtss_f32(dot4_in_x(self, other)) }
        }

        #[inline]
        fn dot_into_vec(self, other: Self) -> Self {
            unsafe {
                let dot_in_x = dot4_in_x(self, other);
                _mm_shuffle_ps(dot_in_x, dot_in_x, 0b00_00_00_00)
            }
        }

        #[inline]
        fn length(self) -> f32 {
            unsafe {
                let dot = dot4_in_x(self, self);
                _mm_cvtss_f32(_mm_sqrt_ps(dot))
            }
        }

        #[inline]
        fn length_recip(self) -> f32 {
            unsafe {
                let dot = dot4_in_x(self, self);
                // _mm_rsqrt_ps is lower precision
                _mm_cvtss_f32(_mm_div_ps(Self::ONE, _mm_sqrt_ps(dot)))
            }
        }

        #[inline]
        fn normalize(self) -> Self {
            unsafe {
                let dot = FloatVector4::dot_into_vec(self, self);
                _mm_div_ps(self, _mm_sqrt_ps(dot))
            }
        }
    }

    impl Quaternion for __m128 {
        // fn from_axis_angle(axis: XYZ<Self::S>, angle: Self::S) -> Self {
        // }

        // fn to_axis_angle(self) -> (XYZ<Self::S>, Self::S) {
        // }

        //fn is_near_identity(self) -> bool {
        //    // Based on https://github.com/nfrechette/rtm `rtm::quat_near_identity`
        //    const THRESHOLD_ANGLE: f32 = 0.002_847_144_6;
        //    // Because of floating point precision, we cannot represent very small rotations.
        //    // The closest f32 to 1.0 that is not 1.0 itself yields:
        //    // 0.99999994.acos() * 2.0  = 0.000690533954 rad
        //    //
        //    // An error threshold of 1.e-6 is used by default.
        //    // (1.0 - 1.e-6).acos() * 2.0 = 0.00284714461 rad
        //    // (1.0 - 1.e-7).acos() * 2.0 = 0.00097656250 rad
        //    //
        //    // We don't really care about the angle value itself, only if it's close to 0.
        //    // This will happen whenever quat.w is close to 1.0.
        //    // If the quat.w is close to -1.0, the angle will be near 2*PI which is close to
        //    // a negative 0 rotation. By forcing quat.w to be positive, we'll end up with
        //    // the shortest path.
        //    let positive_w_angle = self.w.abs().acos_approx() * 2.0;
        //    positive_w_angle < THRESHOLD_ANGLE
        //}

        fn conjugate(self) -> Self {
            const SIGN: __m128 = const_m128!([-0.0, -0.0, -0.0, 0.0]);
            unsafe { _mm_xor_ps(self, SIGN) }
        }

        fn lerp(self, end: Self, s: Self::S) -> Self {
            glam_assert!(self.is_normalized());
            glam_assert!(end.is_normalized());

            unsafe {
                const NEG_ZERO: __m128 = const_m128!([-0.0; 4]);
                let start = self;
                let end = end;
                let dot = FloatVector4::dot_into_vec(start, end);
                // Calculate the bias, if the dot product is positive or zero, there is no bias
                // but if it is negative, we want to flip the 'end' rotation XYZW components
                let bias = _mm_and_ps(dot, NEG_ZERO);
                let interpolated = _mm_add_ps(
                    _mm_mul_ps(_mm_sub_ps(_mm_xor_ps(end, bias), start), _mm_set_ps1(s)),
                    start,
                );
                FloatVector4::normalize(interpolated)
            }
        }

        fn slerp(self, end: Self, s: Self::S) -> Self {
            // http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/
            use crate::scalar_traits::Float;

            glam_assert!(self.is_normalized());
            glam_assert!(end.is_normalized());

            const DOT_THRESHOLD: f32 = 0.9995;

            let dot = FloatVector4::dot(self, end);

            if dot > DOT_THRESHOLD {
                // assumes lerp returns a normalized quaternion
                self.lerp(end, s)
            } else {
                // assumes scalar_acos clamps the input to [-1.0, 1.0]
                let theta = dot.acos_approx();

                let x = 1.0 - s;
                let y = s;
                let z = 1.0;

                unsafe {
                    let tmp = _mm_mul_ps(_mm_set_ps1(theta), _mm_set_ps(0.0, z, y, x));
                    let tmp = crate::f32::funcs::sse2::m128_sin(tmp);

                    let scale1 = _mm_shuffle_ps(tmp, tmp, 0b00_00_00_00);
                    let scale2 = _mm_shuffle_ps(tmp, tmp, 0b01_01_01_01);
                    let theta_sin = _mm_shuffle_ps(tmp, tmp, 0b10_10_10_10);

                    let theta_sin_recip = _mm_rcp_ps(theta_sin);

                    self.mul(scale1).add(end.mul(scale2)).mul(theta_sin_recip)
                }
            }
        }

        fn mul_quaternion(self, other: Self) -> Self {
            unsafe {
                // Based on https://github.com/nfrechette/rtm `rtm::quat_mul`
                let lhs = self;
                let rhs = other;

                const CONTROL_WZYX: __m128 = const_m128!([1.0, -1.0, 1.0, -1.0]);
                const CONTROL_ZWXY: __m128 = const_m128!([1.0, 1.0, -1.0, -1.0]);
                const CONTROL_YXWZ: __m128 = const_m128!([-1.0, 1.0, 1.0, -1.0]);

                let r_xxxx = _mm_shuffle_ps(lhs, lhs, 0b00_00_00_00);
                let r_yyyy = _mm_shuffle_ps(lhs, lhs, 0b01_01_01_01);
                let r_zzzz = _mm_shuffle_ps(lhs, lhs, 0b10_10_10_10);
                let r_wwww = _mm_shuffle_ps(lhs, lhs, 0b11_11_11_11);

                let lxrw_lyrw_lzrw_lwrw = _mm_mul_ps(r_wwww, rhs);
                let l_wzyx = _mm_shuffle_ps(rhs, rhs, 0b00_01_10_11);

                let lwrx_lzrx_lyrx_lxrx = _mm_mul_ps(r_xxxx, l_wzyx);
                let l_zwxy = _mm_shuffle_ps(l_wzyx, l_wzyx, 0b10_11_00_01);

                let lwrx_nlzrx_lyrx_nlxrx = _mm_mul_ps(lwrx_lzrx_lyrx_lxrx, CONTROL_WZYX);

                let lzry_lwry_lxry_lyry = _mm_mul_ps(r_yyyy, l_zwxy);
                let l_yxwz = _mm_shuffle_ps(l_zwxy, l_zwxy, 0b00_01_10_11);

                let lzry_lwry_nlxry_nlyry = _mm_mul_ps(lzry_lwry_lxry_lyry, CONTROL_ZWXY);

                let lyrz_lxrz_lwrz_lzrz = _mm_mul_ps(r_zzzz, l_yxwz);
                let result0 = _mm_add_ps(lxrw_lyrw_lzrw_lwrw, lwrx_nlzrx_lyrx_nlxrx);

                let nlyrz_lxrz_lwrz_wlzrz = _mm_mul_ps(lyrz_lxrz_lwrz_lzrz, CONTROL_YXWZ);
                let result1 = _mm_add_ps(lzry_lwry_nlxry_nlyry, nlyrz_lxrz_lwrz_wlzrz);
                _mm_add_ps(result0, result1)
            }
        }
    }

    impl From<__m128> for XYZW<f32> {
        #[inline]
        fn from(v: __m128) -> XYZW<f32> {
            let mut out: MaybeUninit<Align16<XYZW<f32>>> = MaybeUninit::uninit();
            unsafe {
                _mm_store_ps(out.as_mut_ptr() as *mut f32, v);
                out.assume_init().0
            }
        }
    }

    impl From<__m128> for XYZ<f32> {
        #[inline]
        fn from(v: __m128) -> XYZ<f32> {
            let mut out: MaybeUninit<Align16<XYZ<f32>>> = MaybeUninit::uninit();
            unsafe {
                _mm_store_ps(out.as_mut_ptr() as *mut f32, v);
                out.assume_init().0
            }
        }
    }

    impl From<__m128> for XY<f32> {
        #[inline]
        fn from(v: __m128) -> XY<f32> {
            let mut out: MaybeUninit<Align16<XY<f32>>> = MaybeUninit::uninit();
            unsafe {
                _mm_store_ps(out.as_mut_ptr() as *mut f32, v);
                out.assume_init().0
            }
        }
    }
}
