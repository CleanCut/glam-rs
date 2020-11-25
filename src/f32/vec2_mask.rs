use crate::vector_traits::{MaskVector, MaskVector2, MaskVectorConsts};
use crate::XY;
use core::{cmp::Ordering, hash};
use core::{fmt, ops::*};

type XYU32 = XY<u32>;

#[cfg(not(doc))]
#[derive(Clone, Copy)]
// #[derive(Clone, Copy, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Vec2Mask(pub(crate) XYU32);

/// A 2-dimensional vector mask.
///
/// This type is typically created by comparison methods on `Vec2`.
#[cfg(doc)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Vec2Mask(u32, u32);

impl Default for Vec2Mask {
    #[inline]
    fn default() -> Self {
        Self(XYU32::FALSE)
    }
}

impl PartialEq for Vec2Mask {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}

impl Eq for Vec2Mask {}

impl Ord for Vec2Mask {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl PartialOrd for Vec2Mask {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl hash::Hash for Vec2Mask {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}
impl Vec2Mask {
    /// Creates a new `Vec2Mask`.
    #[inline]
    pub fn new(x: bool, y: bool) -> Self {
        Self(MaskVector2::new(x, y))
    }

    /// Returns a bitmask with the lowest two bits set from the elements of `self`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    pub fn bitmask(self) -> u32 {
        self.0.bitmask()
    }

    /// Returns true if any of the elements are true, false otherwise.
    ///
    /// In other words: `x || y`.
    #[inline]
    pub fn any(self) -> bool {
        self.0.any()
    }

    /// Returns true if all the elements are true, false otherwise.
    ///
    /// In other words: `x && y`.
    #[inline]
    pub fn all(self) -> bool {
        self.0.all()
    }

    ///// Creates a `Vec2` from the elements in `if_true` and `if_false`, selecting which to use for
    ///// each element of `self`.
    /////
    ///// A true element in the mask uses the corresponding element from `if_true`, and false uses
    ///// the element from `if_false`.
    //#[inline]
    //pub fn select(self, if_true: Vec2, if_false: Vec2) -> Vec2 {
    //    Vec2 {
    //        x: if self.0 != 0 { if_true.x } else { if_false.x },
    //        y: if self.1 != 0 { if_true.y } else { if_false.y },
    //    }
    //}
}

impl BitAnd for Vec2Mask {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self(self.0.bitand(other.0))
    }
}

impl BitAndAssign for Vec2Mask {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        self.0 = self.0.bitand(other.0);
    }
}

impl BitOr for Vec2Mask {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self(self.0.bitor(other.0))
    }
}

impl BitOrAssign for Vec2Mask {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        self.0 = self.0.bitor(other.0);
    }
}

impl Not for Vec2Mask {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Self(self.0.not())
    }
}

impl fmt::Debug for Vec2Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.as_ref();
        write!(f, "Vec2Mask({:#x}, {:#x})", arr[0], arr[1])
    }
}

impl fmt::Display for Vec2Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.as_ref();
        write!(f, "[{}, {}]", arr[0] != 0, arr[1] != 0)
    }
}

impl From<Vec2Mask> for [u32; 2] {
    #[inline]
    fn from(mask: Vec2Mask) -> Self {
        *mask.as_ref()
    }
}

impl From<Vec2Mask> for XYU32 {
    #[inline]
    fn from(t: Vec2Mask) -> Self {
        t.0
    }
}

impl AsRef<[u32; 2]> for Vec2Mask {
    #[inline]
    fn as_ref(&self) -> &[u32; 2] {
        unsafe { &*(self as *const Self as *const [u32; 2]) }
    }
}
