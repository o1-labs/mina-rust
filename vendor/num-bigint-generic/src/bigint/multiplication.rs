use super::{
    BigInt,
    CheckedUnsignedAbs::{Negative, Positive},
    Sign::{self, Minus, NoSign, Plus},
    UnsignedAbs,
};

use crate::{IsizePromotion, UsizePromotion};

use core::{
    iter::Product,
    ops::{Mul, MulAssign},
};
use num_traits::{CheckedMul, One, Zero};

impl Mul<Sign> for Sign {
    type Output = Sign;

    #[inline]
    fn mul(self, other: Sign) -> Sign {
        match (self, other) {
            (NoSign, _) | (_, NoSign) => NoSign,
            (Plus, Plus) | (Minus, Minus) => Plus,
            (Plus, Minus) | (Minus, Plus) => Minus,
        }
    }
}

impl<const N: usize> Mul<BigInt<N>> for BigInt<N> {
    type Output = BigInt<N>;
    #[inline]
    fn mul(self, other: BigInt<N>) -> BigInt<N> {
        let BigInt { data: x, .. } = self;
        let BigInt { data: y, .. } = other;
        BigInt::from_biguint(self.sign * other.sign, x * y)
    }
}
impl<const N: usize> Mul<BigInt<N>> for &BigInt<N> {
    type Output = BigInt<N>;
    #[inline]
    fn mul(self, other: BigInt<N>) -> BigInt<N> {
        let BigInt { data: x, .. } = self;
        let BigInt { data: y, .. } = other;
        BigInt::from_biguint(self.sign * other.sign, x * y)
    }
}
impl<const N: usize> Mul<&BigInt<N>> for BigInt<N> {
    type Output = BigInt<N>;
    #[inline]
    fn mul(self, other: &BigInt<N>) -> BigInt<N> {
        let BigInt { data: x, .. } = self;
        let BigInt { data: y, .. } = other;
        BigInt::from_biguint(self.sign * other.sign, x * y)
    }
}
impl<const N: usize> Mul<&BigInt<N>> for &BigInt<N> {
    type Output = BigInt<N>;
    #[inline]
    fn mul(self, other: &BigInt<N>) -> BigInt<N> {
        let BigInt { data: x, .. } = self;
        let BigInt { data: y, .. } = other;
        BigInt::from_biguint(self.sign * other.sign, x * y)
    }
}

impl<const N: usize> MulAssign<BigInt<N>> for BigInt<N> {
    #[inline]
    fn mul_assign(&mut self, other: BigInt<N>) {
        let BigInt { data: y, .. } = other;
        self.data *= y;
        if self.data.is_zero() {
            self.sign = NoSign;
        } else {
            self.sign = self.sign * other.sign;
        }
    }
}
impl<const N: usize> MulAssign<&BigInt<N>> for BigInt<N> {
    #[inline]
    fn mul_assign(&mut self, other: &BigInt<N>) {
        let BigInt { data: y, .. } = other;
        self.data *= y;
        if self.data.is_zero() {
            self.sign = NoSign;
        } else {
            self.sign = self.sign * other.sign;
        }
    }
}

promote_all_scalars!(impl Mul for BigInt<N>, mul);
promote_all_scalars_assign!(impl MulAssign for BigInt<N>, mul_assign);
forward_all_scalar_binop_to_val_val_commutative!(impl Mul<u32> for BigInt<N>, mul);
forward_all_scalar_binop_to_val_val_commutative!(impl Mul<u64> for BigInt<N>, mul);
forward_all_scalar_binop_to_val_val_commutative!(impl Mul<u128> for BigInt<N>, mul);

impl<const N: usize> Mul<u32> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn mul(self, other: u32) -> BigInt<N> {
        BigInt::from_biguint(self.sign, self.data * other)
    }
}

impl<const N: usize> MulAssign<u32> for BigInt<N> {
    #[inline]
    fn mul_assign(&mut self, other: u32) {
        self.data *= other;
        if self.data.is_zero() {
            self.sign = NoSign;
        }
    }
}

impl<const N: usize> Mul<u64> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn mul(self, other: u64) -> BigInt<N> {
        BigInt::from_biguint(self.sign, self.data * other)
    }
}

impl<const N: usize> MulAssign<u64> for BigInt<N> {
    #[inline]
    fn mul_assign(&mut self, other: u64) {
        self.data *= other;
        if self.data.is_zero() {
            self.sign = NoSign;
        }
    }
}

impl<const N: usize> Mul<u128> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn mul(self, other: u128) -> BigInt<N> {
        BigInt::from_biguint(self.sign, self.data * other)
    }
}

impl<const N: usize> MulAssign<u128> for BigInt<N> {
    #[inline]
    fn mul_assign(&mut self, other: u128) {
        self.data *= other;
        if self.data.is_zero() {
            self.sign = NoSign;
        }
    }
}

forward_all_scalar_binop_to_val_val_commutative!(impl Mul<i32> for BigInt<N>, mul);
forward_all_scalar_binop_to_val_val_commutative!(impl Mul<i64> for BigInt<N>, mul);
forward_all_scalar_binop_to_val_val_commutative!(impl Mul<i128> for BigInt<N>, mul);

impl<const N: usize> Mul<i32> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn mul(self, other: i32) -> BigInt<N> {
        match other.checked_uabs() {
            Positive(u) => self * u,
            Negative(u) => -self * u,
        }
    }
}

impl<const N: usize> MulAssign<i32> for BigInt<N> {
    #[inline]
    fn mul_assign(&mut self, other: i32) {
        match other.checked_uabs() {
            Positive(u) => *self *= u,
            Negative(u) => {
                self.sign = -self.sign;
                self.data *= u;
            }
        }
    }
}

impl<const N: usize> Mul<i64> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn mul(self, other: i64) -> BigInt<N> {
        match other.checked_uabs() {
            Positive(u) => self * u,
            Negative(u) => -self * u,
        }
    }
}

impl<const N: usize> MulAssign<i64> for BigInt<N> {
    #[inline]
    fn mul_assign(&mut self, other: i64) {
        match other.checked_uabs() {
            Positive(u) => *self *= u,
            Negative(u) => {
                self.sign = -self.sign;
                self.data *= u;
            }
        }
    }
}

impl<const N: usize> Mul<i128> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn mul(self, other: i128) -> BigInt<N> {
        match other.checked_uabs() {
            Positive(u) => self * u,
            Negative(u) => -self * u,
        }
    }
}

impl<const N: usize> MulAssign<i128> for BigInt<N> {
    #[inline]
    fn mul_assign(&mut self, other: i128) {
        match other.checked_uabs() {
            Positive(u) => *self *= u,
            Negative(u) => {
                self.sign = -self.sign;
                self.data *= u;
            }
        }
    }
}

impl<const N: usize> CheckedMul for BigInt<N> {
    #[inline]
    fn checked_mul(&self, v: &BigInt<N>) -> Option<BigInt<N>> {
        Some(self.mul(v))
    }
}

impl_product_iter_type!(BigInt);
