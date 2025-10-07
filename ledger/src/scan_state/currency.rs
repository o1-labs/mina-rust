use rand::Rng;

use crate::proofs::{
    field::FieldWitness, to_field_elements::ToFieldElements, transaction::Check, witness::Witness,
};

// Re-export all number types from core
pub use mina_core::number::{
    Amount, Balance, BlockTime, BlockTimeSpan, Epoch, Fee, Index, Length, Magnitude, MinMax, Nonce,
    Sgn, Signed, Slot, SlotSpan, TxnVersion, N,
};

// Ledger-specific trait for converting to bits
pub trait ToBits {
    fn to_bits(&self) -> Vec<bool>;
}

// Ledger-specific trait implementations for number types
macro_rules! impl_ledger_traits {
    (32: { $($name32:ident,)* }, 64: { $($name64:ident,)* },) => {
        $(impl_ledger_traits!({$name32, u32, as_u32, append_u32},);)+
        $(impl_ledger_traits!({$name64, u64, as_u64, append_u64},);)+
    };
    ($({ $name:ident, $inner:ty, $as_name:ident, $append_name:ident },)*) => ($(
        impl ToBits for $name {
            fn to_bits(&self) -> Vec<bool> {
                use crate::proofs::transaction::legacy_input::bits_iter;

                let mut iter = bits_iter::<$inner, { <$inner>::BITS as usize }>(self.0);
                let mut result = Vec::with_capacity(<$inner>::BITS as usize);
                for _ in 0..<$inner>::BITS {
                    result.push(iter.next().unwrap());
                }
                result
            }
        }

        impl crate::ToInputs for $name {
            fn to_inputs(&self, inputs: &mut poseidon::hash::Inputs) {
                inputs.$append_name(self.0);
            }
        }

        impl<F: FieldWitness> ToFieldElements<F> for $name {
            fn to_field_elements(&self, fields: &mut Vec<F>) {
                fields.push(self.to_field());
            }
        }

        impl<F: FieldWitness> Check<F> for $name {
            fn check(&self, witnesses: &mut Witness<F>) {
                use crate::proofs::transaction::scalar_challenge::to_field_checked_prime;

                const NBITS: usize = <$inner>::BITS as usize;

                let number: $inner = self.$as_name();
                assert_eq!(NBITS, std::mem::size_of_val(&number) * 8);

                let number: F = number.into();
                to_field_checked_prime::<F, NBITS>(number, witnesses);
            }
        }
    )+)
}

impl_ledger_traits!(
    32: { Length, Slot, Nonce, Index, SlotSpan, TxnVersion, Epoch, },
    64: { Amount, Balance, Fee, BlockTime, BlockTimeSpan, N, },
);
