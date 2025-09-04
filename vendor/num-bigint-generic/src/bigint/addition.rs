use super::{
    BigInt,
    CheckedUnsignedAbs::{Negative, Positive},
    Sign::{Minus, NoSign, Plus},
    UnsignedAbs,
};

use crate::{IsizePromotion, UsizePromotion};

use core::{
    cmp::Ordering::{Equal, Greater, Less},
    iter::Sum,
    mem,
    ops::{Add, AddAssign},
};
use num_traits::CheckedAdd;

// We want to forward to BigUint::add, but it's not clear how that will go until
// we compare both sign and magnitude.  So we duplicate this body for every
// val/ref combination, deferring that decision to BigUint's own forwarding.
macro_rules! bigint_add {
    ($a:expr, $a_owned:expr, $a_data:expr, $b:expr, $b_owned:expr, $b_data:expr) => {
        match ($a.sign, $b.sign) {
            (_, NoSign) => $a_owned,
            (NoSign, _) => $b_owned,
            // same sign => keep the sign with the sum of magnitudes
            (Plus, Plus) | (Minus, Minus) => BigInt::from_biguint($a.sign, $a_data + $b_data),
            // opposite signs => keep the sign of the larger with the difference of magnitudes
            (Plus, Minus) | (Minus, Plus) => match $a.data.cmp(&$b.data) {
                Less => BigInt::from_biguint($b.sign, $b_data - $a_data),
                Greater => BigInt::from_biguint($a.sign, $a_data - $b_data),
                Equal => BigInt::zero(),
            },
        }
    };
}

impl<const N: usize> Add<&BigInt<N>> for &BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn add(self, other: &BigInt<N>) -> BigInt<N> {
        bigint_add!(
            self,
            self.clone(),
            &self.data,
            other,
            other.clone(),
            &other.data
        )
    }
}

impl<const N: usize> Add<BigInt<N>> for &BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn add(self, other: BigInt<N>) -> BigInt<N> {
        bigint_add!(self, self.clone(), &self.data, other, other, other.data)
    }
}

impl<const N: usize> Add<&BigInt<N>> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn add(self, other: &BigInt<N>) -> BigInt<N> {
        bigint_add!(self, self, self.data, other, other.clone(), &other.data)
    }
}

impl<const N: usize> Add<BigInt<N>> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn add(self, other: BigInt<N>) -> BigInt<N> {
        bigint_add!(self, self, self.data, other, other, other.data)
    }
}

impl<const N: usize> AddAssign<&BigInt<N>> for BigInt<N> {
    #[inline]
    fn add_assign(&mut self, other: &BigInt<N>) {
        let n = mem::replace(self, Self::zero());
        *self = n + other;
    }
}
impl<const N: usize> AddAssign<BigInt<N>> for BigInt<N> {
    #[inline]
    fn add_assign(&mut self, other: BigInt<N>) {
        self.add_assign(&other);
    }
}

promote_all_scalars!(impl Add for BigInt<N>, add);
promote_all_scalars_assign!(impl AddAssign for BigInt<N>, add_assign);
forward_all_scalar_binop_to_val_val_commutative!(impl Add<u32> for BigInt<N>, add);
forward_all_scalar_binop_to_val_val_commutative!(impl Add<u64> for BigInt<N>, add);
forward_all_scalar_binop_to_val_val_commutative!(impl Add<u128> for BigInt<N>, add);

impl<const N: usize> Add<u32> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn add(self, other: u32) -> BigInt<N> {
        match self.sign {
            NoSign => From::from(other),
            Plus => BigInt::<N>::from(self.data + other),
            Minus => match self.data.cmp(&From::from(other)) {
                Equal => Self::zero(),
                Less => BigInt::<N>::from(other - self.data),
                Greater => -BigInt::<N>::from(self.data - other),
            },
        }
    }
}

impl<const N: usize> AddAssign<u32> for BigInt<N> {
    #[inline]
    fn add_assign(&mut self, other: u32) {
        let n = mem::replace(self, Self::zero());
        *self = n + other;
    }
}

impl<const N: usize> Add<u64> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn add(self, other: u64) -> BigInt<N> {
        match self.sign {
            NoSign => From::from(other),
            Plus => BigInt::<N>::from(self.data + other),
            Minus => match self.data.cmp(&From::from(other)) {
                Equal => Self::zero(),
                Less => BigInt::<N>::from(other - self.data),
                Greater => -BigInt::<N>::from(self.data - other),
            },
        }
    }
}

impl<const N: usize> AddAssign<u64> for BigInt<N> {
    #[inline]
    fn add_assign(&mut self, other: u64) {
        let n = mem::replace(self, Self::zero());
        *self = n + other;
    }
}

impl<const N: usize> Add<u128> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn add(self, other: u128) -> BigInt<N> {
        match self.sign {
            NoSign => BigInt::<N>::from(other),
            Plus => BigInt::<N>::from(self.data + other),
            Minus => match self.data.cmp(&From::from(other)) {
                Equal => Self::zero(),
                Less => BigInt::<N>::from(other - self.data),
                Greater => -BigInt::<N>::from(self.data - other),
            },
        }
    }
}
impl<const N: usize> AddAssign<u128> for BigInt<N> {
    #[inline]
    fn add_assign(&mut self, other: u128) {
        let n = mem::replace(self, Self::zero());
        *self = n + other;
    }
}

forward_all_scalar_binop_to_val_val_commutative!(impl Add<i32> for BigInt<N>, add);
forward_all_scalar_binop_to_val_val_commutative!(impl Add<i64> for BigInt<N>, add);
forward_all_scalar_binop_to_val_val_commutative!(impl Add<i128> for BigInt<N>, add);

impl<const N: usize> Add<i32> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn add(self, other: i32) -> BigInt<N> {
        match other.checked_uabs() {
            Positive(u) => self + u,
            Negative(u) => self - u,
        }
    }
}
impl<const N: usize> AddAssign<i32> for BigInt<N> {
    #[inline]
    fn add_assign(&mut self, other: i32) {
        match other.checked_uabs() {
            Positive(u) => *self += u,
            Negative(u) => *self -= u,
        }
    }
}

impl<const N: usize> Add<i64> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn add(self, other: i64) -> BigInt<N> {
        match other.checked_uabs() {
            Positive(u) => self + u,
            Negative(u) => self - u,
        }
    }
}
impl<const N: usize> AddAssign<i64> for BigInt<N> {
    #[inline]
    fn add_assign(&mut self, other: i64) {
        match other.checked_uabs() {
            Positive(u) => *self += u,
            Negative(u) => *self -= u,
        }
    }
}

impl<const N: usize> Add<i128> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn add(self, other: i128) -> BigInt<N> {
        match other.checked_uabs() {
            Positive(u) => self + u,
            Negative(u) => self - u,
        }
    }
}
impl<const N: usize> AddAssign<i128> for BigInt<N> {
    #[inline]
    fn add_assign(&mut self, other: i128) {
        match other.checked_uabs() {
            Positive(u) => *self += u,
            Negative(u) => *self -= u,
        }
    }
}

impl<const N: usize> CheckedAdd for BigInt<N> {
    #[inline]
    fn checked_add(&self, v: &BigInt<N>) -> Option<BigInt<N>> {
        Some(self.add(v))
    }
}

impl_sum_iter_type!(BigInt);
