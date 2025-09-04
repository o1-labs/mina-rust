use super::BigUint;

use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

forward_val_val_binop!(impl BitAnd for BigUint<N>, bitand);
forward_ref_val_binop!(impl BitAnd for BigUint<N>, bitand);

// do not use forward_ref_ref_binop_commutative! for bitand so that we can
// clone the smaller value rather than the larger, avoiding over-allocation
impl<const N: usize> BitAnd<&BigUint<N>> for &BigUint<N> {
    type Output = BigUint<N>;

    #[inline]
    fn bitand(self, other: &BigUint<N>) -> BigUint<N> {
        // forward to val-ref, choosing the smaller to clone
        if self.data.len() <= other.data.len() {
            self.clone() & other
        } else {
            other.clone() & self
        }
    }
}

forward_val_assign!(impl BitAndAssign for BigUint<N>, bitand_assign);

impl<const N: usize> BitAnd<&BigUint<N>> for BigUint<N> {
    type Output = BigUint<N>;

    #[inline]
    fn bitand(mut self, other: &BigUint<N>) -> BigUint<N> {
        self &= other;
        self
    }
}
impl<const N: usize> BitAndAssign<&BigUint<N>> for BigUint<N> {
    #[inline]
    fn bitand_assign(&mut self, other: &BigUint<N>) {
        for (ai, &bi) in self.data.iter_mut().zip(other.data.iter()) {
            *ai &= bi;
        }
        self.data.truncate(other.data.len());
        self.normalize();
    }
}

forward_all_binop_to_val_ref_commutative!(impl BitOr for BigUint<N>, bitor);
forward_val_assign!(impl BitOrAssign for BigUint<N>, bitor_assign);

impl<const N: usize> BitOr<&BigUint<N>> for BigUint<N> {
    type Output = BigUint<N>;

    fn bitor(mut self, other: &BigUint<N>) -> BigUint<N> {
        self |= other;
        self
    }
}
impl<const N: usize> BitOrAssign<&BigUint<N>> for BigUint<N> {
    #[inline]
    fn bitor_assign(&mut self, other: &BigUint<N>) {
        for (ai, &bi) in self.data.iter_mut().zip(other.data.iter()) {
            *ai |= bi;
        }
        if other.data.len() > self.data.len() {
            let extra = &other.data[self.data.len()..];
            self.data.extend(extra.iter().cloned());
        }
    }
}

forward_all_binop_to_val_ref_commutative!(impl BitXor for BigUint<N>, bitxor);
forward_val_assign!(impl BitXorAssign for BigUint<N>, bitxor_assign);

impl<const N: usize> BitXor<&BigUint<N>> for BigUint<N> {
    type Output = BigUint<N>;

    fn bitxor(mut self, other: &BigUint<N>) -> BigUint<N> {
        self ^= other;
        self
    }
}
impl<const N: usize> BitXorAssign<&BigUint<N>> for BigUint<N> {
    #[inline]
    fn bitxor_assign(&mut self, other: &BigUint<N>) {
        for (ai, &bi) in self.data.iter_mut().zip(other.data.iter()) {
            *ai ^= bi;
        }
        if other.data.len() > self.data.len() {
            let extra = &other.data[self.data.len()..];
            self.data.extend(extra.iter().cloned());
        }
        self.normalize();
    }
}
