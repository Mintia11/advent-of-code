use std::ops::Sub;

pub trait Num
where
    Self: Sized,
    Self: Sub<Self, Output = Self>,
{
    type Unsigned;
    type Signed;

    fn as_signed(self) -> Self::Signed;
    fn abs_sub(&self, other: &Self) -> Self::Unsigned;
}

macro_rules! impl_traits {
    ($u:ty, $s:ty) => {
        impl Num for $s {
            type Unsigned = $u;
            type Signed = Self;

            #[inline]
            fn as_signed(self) -> Self::Signed {
                self
            }

            fn abs_sub(&self, other: &Self) -> $u {
                (self - other).unsigned_abs()
            }
        }

        impl Num for $u {
            type Unsigned = Self;
            type Signed = $s;

            #[inline]
            fn as_signed(self) -> Self::Signed {
                self as _
            }

            fn abs_sub(&self, other: &Self) -> $u {
                (self.as_signed() - other.as_signed()).unsigned_abs()
            }
        }
    };
}

impl_traits!(usize, isize);
