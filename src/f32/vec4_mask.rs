use crate::vector_traits::{MaskVector, MaskVector4, MaskVectorConsts};
use crate::Vec4;
use core::{fmt, ops::*};

#[cfg(all(vec4_sse2, target_arch = "x86"))]
use core::arch::x86::*;
#[cfg(all(vec4_sse2, target_arch = "x86_64"))]
use core::arch::x86_64::*;
use core::{cmp::Ordering, hash};

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
type Inner = __m128;

#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
type Inner = crate::XYZW<bool>;

#[cfg(not(doc))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec4Mask(pub(crate) Inner);

/// A 4-dimensional vector mask.
///
/// This type is typically created by comparison methods on `Vec4`.  It is
/// essentially a vector of four boolean values.
#[cfg(doc)]
#[repr(C)]
pub struct Vec4Mask(bool, bool, bool, bool);

impl Default for Vec4Mask {
    #[inline]
    fn default() -> Self {
        Self(Inner::FALSE)
    }
}

impl PartialEq for Vec4Mask {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}

impl Eq for Vec4Mask {}

impl Ord for Vec4Mask {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl PartialOrd for Vec4Mask {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl hash::Hash for Vec4Mask {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl Vec4Mask {
    /// Creates a new `Vec4Mask`.
    #[inline]
    pub fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        Self(MaskVector4::new(x, y, z, w))
    }

    /// Returns a bitmask with the lowest four bits set from the elements of `self`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    pub fn bitmask(self) -> u32 {
        self.0.bitmask()
    }

    /// Returns true if any of the elements are true, false otherwise.
    ///
    /// In other words: `x || y || z || w`.
    #[inline]
    pub fn any(self) -> bool {
        self.0.any()
    }

    /// Returns true if all the elements are true, false otherwise.
    ///
    /// In other words: `x && y && z && w`.
    #[inline]
    pub fn all(self) -> bool {
        self.0.all()
    }

    /// Creates a `Vec4` from the elements in `if_true` and `if_false`, selecting which to use for
    /// each element of `self`.
    ///
    /// A true element in the mask uses the corresponding element from `if_true`, and false uses
    /// the element from `if_false`.
    #[inline]
    pub fn select(self, if_true: Vec4, if_false: Vec4) -> Vec4 {
        Vec4::select(self, if_true, if_false)
    }
}

impl BitAnd for Vec4Mask {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self(self.0.and(other.0))
    }
}

impl BitAndAssign for Vec4Mask {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        self.0 = self.0.and(other.0);
    }
}

impl BitOr for Vec4Mask {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self(self.0.or(other.0))
    }
}

impl BitOrAssign for Vec4Mask {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        self.0 = self.0.or(other.0);
    }
}

impl Not for Vec4Mask {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Self(self.0.not())
    }
}

impl fmt::Debug for Vec4Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.as_ref();
        write!(
            f,
            "Vec4Mask({:#x}, {:#x}, {:#x}, {:#x})",
            arr[0], arr[1], arr[2], arr[3]
        )
    }
}

impl fmt::Display for Vec4Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.as_ref();
        write!(
            f,
            "[{}, {}, {}, {}]",
            arr[0] != 0,
            arr[1] != 0,
            arr[2] != 0,
            arr[3] != 0
        )
    }
}

impl From<Vec4Mask> for [u32; 4] {
    #[inline]
    fn from(mask: Vec4Mask) -> Self {
        *mask.as_ref()
    }
}

impl From<Vec4Mask> for Inner {
    #[inline]
    fn from(t: Vec4Mask) -> Self {
        t.0
    }
}

impl AsRef<[u32; 4]> for Vec4Mask {
    #[inline]
    fn as_ref(&self) -> &[u32; 4] {
        unsafe { &*(self as *const Self as *const [u32; 4]) }
    }
}
