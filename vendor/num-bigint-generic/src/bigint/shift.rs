use super::{BigInt, Sign::NoSign};

use core::ops::{Shl, ShlAssign, Shr, ShrAssign};
use num_traits::{PrimInt, Signed, Zero};

macro_rules! impl_shift {
    (@ref $Shx:ident :: $shx:ident, $ShxAssign:ident :: $shx_assign:ident, $rhs:ty) => {
        impl<const N: usize> $Shx<&$rhs> for BigInt<N> {
            type Output = BigInt<N>;

            #[inline]
            fn $shx(self, rhs: &$rhs) -> BigInt<N> {
                $Shx::$shx(self, *rhs)
            }
        }
        impl<const N: usize> $Shx<&$rhs> for &BigInt<N> {
            type Output = BigInt<N>;

            #[inline]
            fn $shx(self, rhs: &$rhs) -> BigInt<N> {
                $Shx::$shx(self, *rhs)
            }
        }
        impl<const N: usize> $ShxAssign<&$rhs> for BigInt<N> {
            #[inline]
            fn $shx_assign(&mut self, rhs: &$rhs) {
                $ShxAssign::$shx_assign(self, *rhs);
            }
        }
    };
    ($($rhs:ty),+) => {$(
        impl<const N: usize> Shl<$rhs> for BigInt<N> {
            type Output = BigInt<N>;

            #[inline]
            fn shl(self, rhs: $rhs) -> BigInt<N> {
                BigInt::<N>::from_biguint(self.sign, self.data << rhs)
            }
        }
        impl<const N: usize> Shl<$rhs> for &BigInt<N> {
            type Output = BigInt<N>;

            #[inline]
            fn shl(self, rhs: $rhs) -> BigInt<N> {
                BigInt::<N>::from_biguint(self.sign, &self.data << rhs)
            }
        }
        impl<const N: usize> ShlAssign<$rhs> for BigInt<N> {
            #[inline]
            fn shl_assign(&mut self, rhs: $rhs) {
                self.data <<= rhs
            }
        }
        impl_shift! { @ref Shl::shl, ShlAssign::shl_assign, $rhs }

        impl<const N: usize> Shr<$rhs> for BigInt<N> {
            type Output = BigInt<N>;

            #[inline]
            fn shr(self, rhs: $rhs) -> BigInt<N> {
                let round_down = shr_round_down(&self, rhs);
                let data = self.data >> rhs;
                let data = if round_down { data + 1u8 } else { data };
                BigInt::from_biguint(self.sign, data)
            }
        }
        impl<const N: usize> Shr<$rhs> for &BigInt<N> {
            type Output = BigInt<N>;

            #[inline]
            fn shr(self, rhs: $rhs) -> BigInt<N> {
                let round_down = shr_round_down(self, rhs);
                let data = &self.data >> rhs;
                let data = if round_down { data + 1u8 } else { data };
                BigInt::from_biguint(self.sign, data)
            }
        }
        impl<const N: usize> ShrAssign<$rhs> for BigInt<N> {
            #[inline]
            fn shr_assign(&mut self, rhs: $rhs) {
                let round_down = shr_round_down(self, rhs);
                self.data >>= rhs;
                if round_down {
                    self.data += 1u8;
                } else if self.data.is_zero() {
                    self.sign = NoSign;
                }
            }
        }
        impl_shift! { @ref Shr::shr, ShrAssign::shr_assign, $rhs }
    )*};
}

impl_shift! { u8, u16, u32, u64, u128, usize }
impl_shift! { i8, i16, i32, i64, i128, isize }

// Negative values need a rounding adjustment if there are any ones in the
// bits that are getting shifted out.
fn shr_round_down<T: PrimInt, const N: usize>(i: &BigInt<N>, shift: T) -> bool {
    if i.is_negative() {
        let zeros = i.trailing_zeros().expect("negative values are non-zero");
        shift > T::zero() && shift.to_u64().map(|shift| zeros < shift).unwrap_or(true)
    } else {
        false
    }
}
