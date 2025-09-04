use super::CheckedUnsignedAbs::{Negative, Positive};
use super::Sign::NoSign;
use super::{BigInt, UnsignedAbs};

use crate::{IsizePromotion, UsizePromotion};

use core::ops::{Div, DivAssign, Rem, RemAssign};
use num_integer::Integer;
use num_traits::{CheckedDiv, CheckedEuclid, Euclid, Signed, ToPrimitive, Zero};

forward_all_binop_to_ref_ref!(impl Div for BigInt<N>, div);

impl<const N: usize> Div<&BigInt<N>> for &BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn div(self, other: &BigInt<N>) -> BigInt<N> {
        let (q, _) = self.div_rem(other);
        q
    }
}

impl<const N: usize> DivAssign<&BigInt<N>> for BigInt<N> {
    #[inline]
    fn div_assign(&mut self, other: &BigInt<N>) {
        *self = &*self / other;
    }
}

forward_val_assign!(impl DivAssign for BigInt<N>, div_assign);

promote_all_scalars!(impl Div for BigInt<N>, div);
promote_all_scalars_assign!(impl DivAssign for BigInt<N>, div_assign);
forward_all_scalar_binop_to_val_val!(impl Div<u32> for BigInt<N>, div);
forward_all_scalar_binop_to_val_val!(impl Div<u64> for BigInt<N>, div);
forward_all_scalar_binop_to_val_val!(impl Div<u128> for BigInt<N>, div);

impl<const N: usize> Div<u32> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn div(self, other: u32) -> BigInt<N> {
        BigInt::from_biguint(self.sign, self.data / other)
    }
}

impl<const N: usize> DivAssign<u32> for BigInt<N> {
    #[inline]
    fn div_assign(&mut self, other: u32) {
        self.data /= other;
        if self.data.is_zero() {
            self.sign = NoSign;
        }
    }
}

impl<const N: usize> Div<BigInt<N>> for u32 {
    type Output = BigInt<N>;

    #[inline]
    fn div(self, other: BigInt<N>) -> BigInt<N> {
        BigInt::from_biguint(other.sign, self / other.data)
    }
}

impl<const N: usize> Div<u64> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn div(self, other: u64) -> BigInt<N> {
        BigInt::from_biguint(self.sign, self.data / other)
    }
}

impl<const N: usize> DivAssign<u64> for BigInt<N> {
    #[inline]
    fn div_assign(&mut self, other: u64) {
        self.data /= other;
        if self.data.is_zero() {
            self.sign = NoSign;
        }
    }
}

impl<const N: usize> Div<BigInt<N>> for u64 {
    type Output = BigInt<N>;

    #[inline]
    fn div(self, other: BigInt<N>) -> BigInt<N> {
        BigInt::from_biguint(other.sign, self / other.data)
    }
}

impl<const N: usize> Div<u128> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn div(self, other: u128) -> BigInt<N> {
        BigInt::from_biguint(self.sign, self.data / other)
    }
}

impl<const N: usize> DivAssign<u128> for BigInt<N> {
    #[inline]
    fn div_assign(&mut self, other: u128) {
        self.data /= other;
        if self.data.is_zero() {
            self.sign = NoSign;
        }
    }
}

impl<const N: usize> Div<BigInt<N>> for u128 {
    type Output = BigInt<N>;

    #[inline]
    fn div(self, other: BigInt<N>) -> BigInt<N> {
        BigInt::from_biguint(other.sign, self / other.data)
    }
}

forward_all_scalar_binop_to_val_val!(impl Div<i32> for BigInt<N>, div);
forward_all_scalar_binop_to_val_val!(impl Div<i64> for BigInt<N>, div);
forward_all_scalar_binop_to_val_val!(impl Div<i128> for BigInt<N>, div);

impl<const N: usize> Div<i32> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn div(self, other: i32) -> BigInt<N> {
        match other.checked_uabs() {
            Positive(u) => self / u,
            Negative(u) => -self / u,
        }
    }
}

impl<const N: usize> DivAssign<i32> for BigInt<N> {
    #[inline]
    fn div_assign(&mut self, other: i32) {
        match other.checked_uabs() {
            Positive(u) => *self /= u,
            Negative(u) => {
                self.sign = -self.sign;
                *self /= u;
            }
        }
    }
}

impl<const N: usize> Div<BigInt<N>> for i32 {
    type Output = BigInt<N>;

    #[inline]
    fn div(self, other: BigInt<N>) -> BigInt<N> {
        match self.checked_uabs() {
            Positive(u) => u / other,
            Negative(u) => u / -other,
        }
    }
}

impl<const N: usize> Div<i64> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn div(self, other: i64) -> BigInt<N> {
        match other.checked_uabs() {
            Positive(u) => self / u,
            Negative(u) => -self / u,
        }
    }
}

impl<const N: usize> DivAssign<i64> for BigInt<N> {
    #[inline]
    fn div_assign(&mut self, other: i64) {
        match other.checked_uabs() {
            Positive(u) => *self /= u,
            Negative(u) => {
                self.sign = -self.sign;
                *self /= u;
            }
        }
    }
}

impl<const N: usize> Div<BigInt<N>> for i64 {
    type Output = BigInt<N>;

    #[inline]
    fn div(self, other: BigInt<N>) -> BigInt<N> {
        match self.checked_uabs() {
            Positive(u) => u / other,
            Negative(u) => u / -other,
        }
    }
}

impl<const N: usize> Div<i128> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn div(self, other: i128) -> BigInt<N> {
        match other.checked_uabs() {
            Positive(u) => self / u,
            Negative(u) => -self / u,
        }
    }
}

impl<const N: usize> DivAssign<i128> for BigInt<N> {
    #[inline]
    fn div_assign(&mut self, other: i128) {
        match other.checked_uabs() {
            Positive(u) => *self /= u,
            Negative(u) => {
                self.sign = -self.sign;
                *self /= u;
            }
        }
    }
}

impl<const N: usize> Div<BigInt<N>> for i128 {
    type Output = BigInt<N>;

    #[inline]
    fn div(self, other: BigInt<N>) -> BigInt<N> {
        match self.checked_uabs() {
            Positive(u) => u / other,
            Negative(u) => u / -other,
        }
    }
}

forward_all_binop_to_ref_ref!(impl Rem for BigInt<N>, rem);

impl<const N: usize> Rem<&BigInt<N>> for &BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn rem(self, other: &BigInt<N>) -> BigInt<N> {
        if let Some(other) = other.to_u32() {
            self % other
        } else if let Some(other) = other.to_i32() {
            self % other
        } else {
            let (_, r) = self.div_rem(other);
            r
        }
    }
}

impl<const N: usize> RemAssign<&BigInt<N>> for BigInt<N> {
    #[inline]
    fn rem_assign(&mut self, other: &BigInt<N>) {
        *self = &*self % other;
    }
}
impl<const N: usize> RemAssign<BigInt<N>> for BigInt<N> {
    #[inline]
    fn rem_assign(&mut self, other: BigInt<N>) {
        self.rem_assign(&other);
    }
}

promote_all_scalars!(impl Rem for BigInt<N>, rem);
promote_all_scalars_assign!(impl RemAssign for BigInt<N>, rem_assign);
forward_all_scalar_binop_to_val_val!(impl Rem<u32> for BigInt<N>, rem);
forward_all_scalar_binop_to_val_val!(impl Rem<u64> for BigInt<N>, rem);
forward_all_scalar_binop_to_val_val!(impl Rem<u128> for BigInt<N>, rem);

impl<const N: usize> Rem<u32> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn rem(self, other: u32) -> BigInt<N> {
        BigInt::from_biguint(self.sign, self.data % other)
    }
}

impl<const N: usize> RemAssign<u32> for BigInt<N> {
    #[inline]
    fn rem_assign(&mut self, other: u32) {
        self.data %= other;
        if self.data.is_zero() {
            self.sign = NoSign;
        }
    }
}

impl<const N: usize> Rem<BigInt<N>> for u32 {
    type Output = BigInt<N>;

    #[inline]
    fn rem(self, other: BigInt<N>) -> BigInt<N> {
        BigInt::from(self % other.data)
    }
}

impl<const N: usize> Rem<u64> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn rem(self, other: u64) -> BigInt<N> {
        BigInt::from_biguint(self.sign, self.data % other)
    }
}

impl<const N: usize> RemAssign<u64> for BigInt<N> {
    #[inline]
    fn rem_assign(&mut self, other: u64) {
        self.data %= other;
        if self.data.is_zero() {
            self.sign = NoSign;
        }
    }
}

impl<const N: usize> Rem<BigInt<N>> for u64 {
    type Output = BigInt<N>;

    #[inline]
    fn rem(self, other: BigInt<N>) -> BigInt<N> {
        BigInt::from(self % other.data)
    }
}

impl<const N: usize> Rem<u128> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn rem(self, other: u128) -> BigInt<N> {
        BigInt::from_biguint(self.sign, self.data % other)
    }
}

impl<const N: usize> RemAssign<u128> for BigInt<N> {
    #[inline]
    fn rem_assign(&mut self, other: u128) {
        self.data %= other;
        if self.data.is_zero() {
            self.sign = NoSign;
        }
    }
}

impl<const N: usize> Rem<BigInt<N>> for u128 {
    type Output = BigInt<N>;

    #[inline]
    fn rem(self, other: BigInt<N>) -> BigInt<N> {
        BigInt::from(self % other.data)
    }
}

forward_all_scalar_binop_to_val_val!(impl Rem<i32> for BigInt<N>, rem);
forward_all_scalar_binop_to_val_val!(impl Rem<i64> for BigInt<N>, rem);
forward_all_scalar_binop_to_val_val!(impl Rem<i128> for BigInt<N>, rem);

impl<const N: usize> Rem<i32> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn rem(self, other: i32) -> BigInt<N> {
        self % other.unsigned_abs()
    }
}

impl<const N: usize> RemAssign<i32> for BigInt<N> {
    #[inline]
    fn rem_assign(&mut self, other: i32) {
        *self %= other.unsigned_abs();
    }
}

impl<const N: usize> Rem<BigInt<N>> for i32 {
    type Output = BigInt<N>;

    #[inline]
    fn rem(self, other: BigInt<N>) -> BigInt<N> {
        match self.checked_uabs() {
            Positive(u) => u % other,
            Negative(u) => -(u % other),
        }
    }
}

impl<const N: usize> Rem<i64> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn rem(self, other: i64) -> BigInt<N> {
        self % other.unsigned_abs()
    }
}

impl<const N: usize> RemAssign<i64> for BigInt<N> {
    #[inline]
    fn rem_assign(&mut self, other: i64) {
        *self %= other.unsigned_abs();
    }
}

impl<const N: usize> Rem<BigInt<N>> for i64 {
    type Output = BigInt<N>;

    #[inline]
    fn rem(self, other: BigInt<N>) -> BigInt<N> {
        match self.checked_uabs() {
            Positive(u) => u % other,
            Negative(u) => -(u % other),
        }
    }
}

impl<const N: usize> Rem<i128> for BigInt<N> {
    type Output = BigInt<N>;

    #[inline]
    fn rem(self, other: i128) -> BigInt<N> {
        self % other.unsigned_abs()
    }
}

impl<const N: usize> RemAssign<i128> for BigInt<N> {
    #[inline]
    fn rem_assign(&mut self, other: i128) {
        *self %= other.unsigned_abs();
    }
}

impl<const N: usize> Rem<BigInt<N>> for i128 {
    type Output = BigInt<N>;

    #[inline]
    fn rem(self, other: BigInt<N>) -> BigInt<N> {
        match self.checked_uabs() {
            Positive(u) => u % other,
            Negative(u) => -(u % other),
        }
    }
}

impl<const N: usize> CheckedDiv for BigInt<N> {
    #[inline]
    fn checked_div(&self, v: &BigInt<N>) -> Option<BigInt<N>> {
        if v.is_zero() {
            return None;
        }
        Some(self.div(v))
    }
}

impl<const N: usize> CheckedEuclid for BigInt<N> {
    #[inline]
    fn checked_div_euclid(&self, v: &BigInt<N>) -> Option<BigInt<N>> {
        if v.is_zero() {
            return None;
        }
        Some(self.div_euclid(v))
    }

    #[inline]
    fn checked_rem_euclid(&self, v: &BigInt<N>) -> Option<BigInt<N>> {
        if v.is_zero() {
            return None;
        }
        Some(self.rem_euclid(v))
    }

    fn checked_div_rem_euclid(&self, v: &Self) -> Option<(Self, Self)> {
        Some(self.div_rem_euclid(v))
    }
}

impl<const N: usize> Euclid for BigInt<N> {
    #[inline]
    fn div_euclid(&self, v: &BigInt<N>) -> BigInt<N> {
        let (q, r) = self.div_rem(v);
        if r.is_negative() {
            if v.is_positive() {
                q - 1
            } else {
                q + 1
            }
        } else {
            q
        }
    }

    #[inline]
    fn rem_euclid(&self, v: &BigInt<N>) -> BigInt<N> {
        let r = self % v;
        if r.is_negative() {
            if v.is_positive() {
                r + v
            } else {
                r - v
            }
        } else {
            r
        }
    }

    fn div_rem_euclid(&self, v: &Self) -> (Self, Self) {
        let (q, r) = self.div_rem(v);
        if r.is_negative() {
            if v.is_positive() {
                (q - 1, r + v)
            } else {
                (q + 1, r - v)
            }
        } else {
            (q, r)
        }
    }
}
