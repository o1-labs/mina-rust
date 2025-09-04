use super::{biguint_from_tinyvec, BigUint};

use crate::big_digit;

use alloc::borrow::Cow;
use core::mem;
use core::ops::{Shl, ShlAssign, Shr, ShrAssign};
use num_traits::{PrimInt, Zero};
use tinyvec::TinyVec;

#[inline]
fn biguint_shl<T: PrimInt, const N: usize>(n: Cow<'_, BigUint<N>>, shift: T) -> BigUint<N> {
    if shift < T::zero() {
        panic!("attempt to shift left with negative");
    }
    if n.is_zero() {
        return n.into_owned();
    }
    let bits = T::from(big_digit::BITS).unwrap();
    let digits = (shift / bits).to_usize().expect("capacity overflow");
    let shift = (shift % bits).to_u8().unwrap();
    biguint_shl2(n, digits, shift)
}

fn biguint_shl2<const N: usize>(n: Cow<'_, BigUint<N>>, digits: usize, shift: u8) -> BigUint<N> {
    let mut data = match digits {
        0 => n.into_owned().data,
        _ => {
            let len = digits.saturating_add(n.data.len() + 1);
            let mut data = TinyVec::with_capacity(len);
            data.resize(digits, 0);
            data.extend(n.data.iter().copied());
            data
        }
    };

    if shift > 0 {
        let mut carry = 0;
        let carry_shift = big_digit::BITS - shift;
        for elem in data[digits..].iter_mut() {
            let new_carry = *elem >> carry_shift;
            *elem = (*elem << shift) | carry;
            carry = new_carry;
        }
        if carry != 0 {
            data.push(carry);
        }
    }

    biguint_from_tinyvec(data)
}

#[inline]
fn biguint_shr<T: PrimInt, const N: usize>(n: &mut BigUint<N>, shift: T) {
    if shift < T::zero() {
        panic!("attempt to shift right with negative");
    }
    if n.is_zero() {
        return;
    }
    let bits = T::from(big_digit::BITS).unwrap();
    let digits = (shift / bits).to_usize().unwrap_or(usize::MAX);
    let shift = (shift % bits).to_u8().unwrap();
    biguint_shr2(n, digits, shift)
}

fn biguint_shr2<const N: usize>(n: &mut BigUint<N>, digits: usize, shift: u8) {
    if digits >= n.data.len() {
        n.set_zero();
        return;
    }

    let data = &mut n.data[digits..];

    if shift > 0 {
        let mut borrow = 0;
        let borrow_shift = big_digit::BITS - shift;
        for elem in data.iter_mut().rev() {
            let new_borrow = *elem << borrow_shift;
            *elem = (*elem >> shift) | borrow;
            borrow = new_borrow;
        }
    }

    let len = data.len();
    n.data.copy_within(digits.., 0);
    n.data.truncate(len);
    n.normalize();
}

macro_rules! impl_shift {
    (@ref $Shx:ident :: $shx:ident, $ShxAssign:ident :: $shx_assign:ident, $rhs:ty) => {
        impl<const N: usize> $Shx<&$rhs> for BigUint<N> {
            type Output = BigUint<N>;

            #[inline]
            fn $shx(self, rhs: &$rhs) -> BigUint<N> {
                $Shx::$shx(self, *rhs)
            }
        }
        impl<const N: usize> $Shx<&$rhs> for &BigUint<N> {
            type Output = BigUint<N>;

            #[inline]
            fn $shx(self, rhs: &$rhs) -> BigUint<N> {
                $Shx::$shx(self, *rhs)
            }
        }
        impl<const N: usize> $ShxAssign<&$rhs> for BigUint<N> {
            #[inline]
            fn $shx_assign(&mut self, rhs: &$rhs) {
                $ShxAssign::$shx_assign(self, *rhs);
            }
        }
    };
    ($($rhs:ty),+) => {$(
        impl<const N: usize> Shl<$rhs> for BigUint<N> {
            type Output = BigUint<N>;

            #[inline]
            fn shl(self, rhs: $rhs) -> BigUint<N> {
                biguint_shl(Cow::Owned(self), rhs)
            }
        }
        impl<const N: usize> Shl<$rhs> for &BigUint<N> {
            type Output = BigUint<N>;

            #[inline]
            fn shl(self, rhs: $rhs) -> BigUint<N> {
                biguint_shl(Cow::Borrowed(self), rhs)
            }
        }
        impl<const N: usize> ShlAssign<$rhs> for BigUint<N> {
            #[inline]
            fn shl_assign(&mut self, rhs: $rhs) {
                let n = mem::replace(self, Self::zero());
                *self = n << rhs;
            }
        }
        impl_shift! { @ref Shl::shl, ShlAssign::shl_assign, $rhs }

        impl<const N: usize> Shr<$rhs> for BigUint<N> {
            type Output = BigUint<N>;

            #[inline]
            fn shr(mut self, rhs: $rhs) -> BigUint<N> {
                biguint_shr(&mut self, rhs);
                self
            }
        }
        impl<const N: usize> Shr<$rhs> for &BigUint<N> {
            type Output = BigUint<N>;

            #[inline]
            fn shr(self, rhs: $rhs) -> BigUint<N> {
                let mut this  = self.clone();
                biguint_shr(&mut this, rhs);
                this
            }
        }
        impl<const N: usize> ShrAssign<$rhs> for BigUint<N> {
            #[inline]
            fn shr_assign(&mut self, rhs: $rhs) {
                biguint_shr(self, rhs);
            }
        }
        impl_shift! { @ref Shr::shr, ShrAssign::shr_assign, $rhs }
    )*};
}

impl_shift! { u8, u16, u32, u64, u128, usize }
impl_shift! { i8, i16, i32, i64, i128, isize }
