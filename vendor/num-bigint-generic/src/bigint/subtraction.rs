use super::{
    BigInt,
    CheckedUnsignedAbs::{Negative, Positive},
    Sign::{Minus, NoSign, Plus},
    UnsignedAbs,
};

use crate::{IsizePromotion, UsizePromotion};

use core::{
    cmp::Ordering::{Equal, Greater, Less},
    mem,
    ops::{Sub, SubAssign},
};
use num_traits::CheckedSub;

// We want to forward to BigUint::sub, but it's not clear how that will go until
// we compare both sign and magnitude.  So we duplicate this body for every
// val/ref combination, deferring that decision to BigUint's own forwarding.
macro_rules! bigint_sub {
    ($a:expr, $a_owned:expr, $a_data:expr, $b:expr, $b_owned:expr, $b_data:expr) => {
        match ($a.sign, $b.sign) {
            (_, NoSign) => $a_owned,
            (NoSign, _) => -$b_owned,
            // opposite signs => keep the sign of the left with the sum of magnitudes
            (Plus, Minus) | (Minus, Plus) => BigInt::from_biguint($a.sign, $a_data + $b_data),
            // same sign => keep or toggle the sign of the left with the difference of magnitudes
            (Plus, Plus) | (Minus, Minus) => match $a.data.cmp(&$b.data) {
                Less => BigInt::from_biguint(-$a.sign, $b_data - $a_data),
                Greater => BigInt::from_biguint($a.sign, $a_data - $b_data),
                Equal => BigInt::zero(),
            },
        }
    };
}

impl<const N: usize> Sub<&BigInt<N>> for &BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn sub(self, other: &BigInt<N>) -> BigInt<N> {
        bigint_sub!(
            self,
            self.clone(),
            &self.data,
            other,
            other.clone(),
            &other.data
        )
    }
}

impl<const N: usize> Sub<BigInt<N>> for &BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn sub(self, other: BigInt<N>) -> BigInt<N> {
        bigint_sub!(self, self.clone(), &self.data, other, other, other.data)
    }
}

impl<const N: usize> Sub<&BigInt<N>> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn sub(self, other: &BigInt<N>) -> BigInt<N> {
        bigint_sub!(self, self, self.data, other, other.clone(), &other.data)
    }
}

impl<const N: usize> Sub<BigInt<N>> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn sub(self, other: BigInt<N>) -> BigInt<N> {
        bigint_sub!(self, self, self.data, other, other, other.data)
    }
}

impl<const N: usize> SubAssign<&BigInt<N>> for BigInt<N> {
    #[inline]
    fn sub_assign(&mut self, other: &BigInt<N>) {
        let n = mem::replace(self, Self::zero());
        *self = n - other;
    }
}
forward_val_assign!(impl SubAssign for BigInt<N>, sub_assign);

promote_all_scalars!(impl Sub for BigInt<N>, sub);
promote_all_scalars_assign!(impl SubAssign for BigInt<N>, sub_assign);
forward_all_scalar_binop_to_val_val!(impl Sub<u32> for BigInt<N>, sub);
forward_all_scalar_binop_to_val_val!(impl Sub<u64> for BigInt<N>, sub);
forward_all_scalar_binop_to_val_val!(impl Sub<u128> for BigInt<N>, sub);

impl<const N: usize> Sub<u32> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn sub(self, other: u32) -> BigInt<N> {
        match self.sign {
            NoSign => -BigInt::<N>::from(other),
            Minus => -BigInt::<N>::from(self.data + other),
            Plus => match self.data.cmp(&From::from(other)) {
                Equal => Self::zero(),
                Greater => BigInt::<N>::from(self.data - other),
                Less => -BigInt::<N>::from(other - self.data),
            },
        }
    }
}
impl<const N: usize> SubAssign<u32> for BigInt<N> {
    #[inline]
    fn sub_assign(&mut self, other: u32) {
        let n = mem::replace(self, Self::zero());
        *self = n - other;
    }
}

impl<const N: usize> Sub<BigInt<N>> for u32 {
    type Output = BigInt<N>;

    #[inline]
    fn sub(self, other: BigInt<N>) -> BigInt<N> {
        -(other - self)
    }
}

impl<const N: usize> Sub<BigInt<N>> for u64 {
    type Output = BigInt<N>;

    #[inline]
    fn sub(self, other: BigInt<N>) -> BigInt<N> {
        -(other - self)
    }
}

impl<const N: usize> Sub<BigInt<N>> for u128 {
    type Output = BigInt<N>;

    #[inline]
    fn sub(self, other: BigInt<N>) -> BigInt<N> {
        -(other - self)
    }
}

impl<const N: usize> Sub<u64> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn sub(self, other: u64) -> BigInt<N> {
        match self.sign {
            NoSign => -BigInt::<N>::from(other),
            Minus => -BigInt::<N>::from(self.data + other),
            Plus => match self.data.cmp(&From::from(other)) {
                Equal => Self::zero(),
                Greater => BigInt::<N>::from(self.data - other),
                Less => -BigInt::<N>::from(other - self.data),
            },
        }
    }
}

impl<const N: usize> SubAssign<u64> for BigInt<N> {
    #[inline]
    fn sub_assign(&mut self, other: u64) {
        let n = mem::replace(self, Self::zero());
        *self = n - other;
    }
}

impl<const N: usize> Sub<u128> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn sub(self, other: u128) -> BigInt<N> {
        match self.sign {
            NoSign => -BigInt::from(other),
            Minus => -BigInt::from(self.data + other),
            Plus => match self.data.cmp(&From::from(other)) {
                Equal => Self::zero(),
                Greater => BigInt::from(self.data - other),
                Less => -BigInt::from(other - self.data),
            },
        }
    }
}

impl<const N: usize> SubAssign<u128> for BigInt<N> {
    #[inline]
    fn sub_assign(&mut self, other: u128) {
        let n = mem::replace(self, Self::zero());
        *self = n - other;
    }
}

forward_all_scalar_binop_to_val_val!(impl Sub<i32> for BigInt<N>, sub);
forward_all_scalar_binop_to_val_val!(impl Sub<i64> for BigInt<N>, sub);
forward_all_scalar_binop_to_val_val!(impl Sub<i128> for BigInt<N>, sub);

impl<const N: usize> Sub<i32> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn sub(self, other: i32) -> BigInt<N> {
        match other.checked_uabs() {
            Positive(u) => self - u,
            Negative(u) => self + u,
        }
    }
}
impl<const N: usize> SubAssign<i32> for BigInt<N> {
    #[inline]
    fn sub_assign(&mut self, other: i32) {
        match other.checked_uabs() {
            Positive(u) => *self -= u,
            Negative(u) => *self += u,
        }
    }
}

impl<const N: usize> Sub<BigInt<N>> for i32 {
    type Output = BigInt<N>;

    #[inline]
    fn sub(self, other: BigInt<N>) -> BigInt<N> {
        match self.checked_uabs() {
            Positive(u) => u - other,
            Negative(u) => -other - u,
        }
    }
}

impl<const N: usize> Sub<i64> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn sub(self, other: i64) -> BigInt<N> {
        match other.checked_uabs() {
            Positive(u) => self - u,
            Negative(u) => self + u,
        }
    }
}
impl<const N: usize> SubAssign<i64> for BigInt<N> {
    #[inline]
    fn sub_assign(&mut self, other: i64) {
        match other.checked_uabs() {
            Positive(u) => *self -= u,
            Negative(u) => *self += u,
        }
    }
}

impl<const N: usize> Sub<BigInt<N>> for i64 {
    type Output = BigInt<N>;

    #[inline]
    fn sub(self, other: BigInt<N>) -> BigInt<N> {
        match self.checked_uabs() {
            Positive(u) => u - other,
            Negative(u) => -other - u,
        }
    }
}

impl<const N: usize> Sub<i128> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn sub(self, other: i128) -> BigInt<N> {
        match other.checked_uabs() {
            Positive(u) => self - u,
            Negative(u) => self + u,
        }
    }
}

impl<const N: usize> SubAssign<i128> for BigInt<N> {
    #[inline]
    fn sub_assign(&mut self, other: i128) {
        match other.checked_uabs() {
            Positive(u) => *self -= u,
            Negative(u) => *self += u,
        }
    }
}

impl<const N: usize> Sub<BigInt<N>> for i128 {
    type Output = BigInt<N>;

    #[inline]
    fn sub(self, other: BigInt<N>) -> BigInt<N> {
        match self.checked_uabs() {
            Positive(u) => u - other,
            Negative(u) => -other - u,
        }
    }
}

impl<const N: usize> CheckedSub for BigInt<N> {
    #[inline]
    fn checked_sub(&self, v: &BigInt<N>) -> Option<BigInt<N>> {
        Some(self.sub(v))
    }
}
