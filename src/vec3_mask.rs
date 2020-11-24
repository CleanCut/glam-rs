use crate::vector_traits::{MaskVector, MaskVector3, MaskVectorConsts};
use crate::XYZ;
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

use core::{cmp::Ordering, hash};

macro_rules! impl_vec3mask {
    ($vec3mask:ident, $inner:ident) => {
        impl Default for $vec3mask {
            #[inline]
            fn default() -> Self {
                Self($inner::FALSE)
            }
        }

        impl PartialEq for $vec3mask {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.as_ref().eq(other.as_ref())
            }
        }

        impl Eq for $vec3mask {}

        impl Ord for $vec3mask {
            #[inline]
            fn cmp(&self, other: &Self) -> Ordering {
                self.as_ref().cmp(other.as_ref())
            }
        }

        impl PartialOrd for $vec3mask {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl hash::Hash for $vec3mask {
            #[inline]
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                self.as_ref().hash(state);
            }
        }

        impl $vec3mask {
            /// Creates a new `$vec3mask`.
            #[inline]
            pub fn new(x: bool, y: bool, z: bool) -> Self {
                Self(MaskVector3::new(x, y, z))
            }

            /// Returns a bitmask with the lowest three bits set from the elements of `self`.
            ///
            /// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
            /// into the first lowest bit, element `y` into the second, etc.
            #[inline]
            pub fn bitmask(self) -> u32 {
                self.0.bitmask()
            }

            /// Returns true if any of the elements are true, false otherwise.
            ///
            /// In other words: `x || y || z`.
            #[inline]
            pub fn any(self) -> bool {
                self.0.any()
            }

            /// Returns true if all the elements are true, false otherwise.
            ///
            /// In other words: `x && y && z`.
            #[inline]
            pub fn all(self) -> bool {
                self.0.all()
            }

            ///// Creates a new `Vec3A` from the elements in `if_true` and `if_false`, selecting which to use
            ///// for each element of `self`.
            /////
            ///// A true element in the mask uses the corresponding element from `if_true`, and false uses
            ///// the element from `if_false`.
            // #[inline]
            // pub fn select(self, if_true: Vec3A, if_false: Vec3A) -> Vec3A {
            //     Vec3A::select(self, if_true, if_false)
            // }
        }

        impl BitAnd for $vec3mask {
            type Output = Self;
            #[inline]
            fn bitand(self, other: Self) -> Self {
                Self(self.0.bitand(other.0))
            }
        }

        impl BitAndAssign for $vec3mask {
            #[inline]
            fn bitand_assign(&mut self, other: Self) {
                self.0 = self.0.bitand(other.0);
            }
        }

        impl BitOr for $vec3mask {
            type Output = Self;
            #[inline]
            fn bitor(self, other: Self) -> Self {
                Self(self.0.bitor(other.0))
            }
        }

        impl BitOrAssign for $vec3mask {
            #[inline]
            fn bitor_assign(&mut self, other: Self) {
                self.0 = self.0.bitor(other.0);
            }
        }

        impl Not for $vec3mask {
            type Output = Self;
            #[inline]
            fn not(self) -> Self {
                Self(self.0.not())
            }
        }

        impl fmt::Debug for $vec3mask {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let arr = self.as_ref();
                write!(
                    f,
                    "{}({:#x}, {:#x}, {:#x})",
                    stringify!($vec3mask),
                    arr[0],
                    arr[1],
                    arr[2]
                )
            }
        }

        impl fmt::Display for $vec3mask {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let arr = self.as_ref();
                write!(f, "[{}, {}, {}]", arr[0] != 0, arr[1] != 0, arr[2] != 0,)
            }
        }

        impl From<$vec3mask> for [u32; 3] {
            #[inline]
            fn from(mask: $vec3mask) -> Self {
                *mask.as_ref()
            }
        }

        impl From<$vec3mask> for $inner {
            #[inline]
            fn from(t: $vec3mask) -> Self {
                t.0
            }
        }

        impl AsRef<[u32; 3]> for $vec3mask {
            #[inline]
            fn as_ref(&self) -> &[u32; 3] {
                unsafe { &*(self as *const Self as *const [u32; 3]) }
            }
        }
    };
}

type XYZU32 = XYZ<u32>;

/// A 3-dimensional vector mask.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec3Mask(pub(crate) XYZU32);

#[cfg(not(doc))]
#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
#[derive(Clone, Copy)]
#[repr(align(16), C)]
pub struct Vec3AMask(pub(crate) __m128);

#[cfg(not(doc))]
#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
#[derive(Clone, Copy)]
#[repr(align(16), C)]
pub struct Vec3AMask(pub(crate) XYZU32);

/// A 3-dimensional vector mask.
///
/// This type is typically created by comparison methods on `Vec3A`.  It is essentially a vector of
/// three boolean values.
#[cfg(doc)]
#[repr(align(16), C)]
pub struct Vec3AMask(u32, u32, u32);

impl_vec3mask!(Vec3Mask, XYZU32);

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
impl_vec3mask!(Vec3AMask, __m128);

#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
impl_vec3mask!(Vec3AMask, XYZU32);
