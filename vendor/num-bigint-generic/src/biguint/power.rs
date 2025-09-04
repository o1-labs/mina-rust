use super::{monty::monty_modpow, BigUint};

use crate::big_digit::{self, BigDigit};

use num_integer::Integer;
use num_traits::{One, Pow, ToPrimitive, Zero};

impl<const N: usize> Pow<&BigUint<N>> for BigUint<N> {
    type Output = BigUint<N>;

    #[inline]
    fn pow(self, exp: &BigUint<N>) -> BigUint<N> {
        if self.is_one() || exp.is_zero() {
            BigUint::one()
        } else if self.is_zero() {
            Self::zero()
        } else if let Some(exp) = exp.to_u64() {
            self.pow(exp)
        } else if let Some(exp) = exp.to_u128() {
            self.pow(exp)
        } else {
            // At this point, `self >= 2` and `exp >= 2¹²⁸`. The smallest possible result given
            // `2.pow(2¹²⁸)` would require far more memory than 64-bit targets can address!
            panic!("memory overflow")
        }
    }
}

impl<const N: usize> Pow<BigUint<N>> for BigUint<N> {
    type Output = BigUint<N>;

    #[inline]
    fn pow(self, exp: BigUint<N>) -> BigUint<N> {
        Pow::pow(self, &exp)
    }
}

impl<const N: usize> Pow<&BigUint<N>> for &BigUint<N> {
    type Output = BigUint<N>;

    #[inline]
    fn pow(self, exp: &BigUint<N>) -> BigUint<N> {
        if self.is_one() || exp.is_zero() {
            BigUint::one()
        } else if self.is_zero() {
            BigUint::zero()
        } else {
            self.clone().pow(exp)
        }
    }
}

impl<const N: usize> Pow<BigUint<N>> for &BigUint<N> {
    type Output = BigUint<N>;

    #[inline]
    fn pow(self, exp: BigUint<N>) -> BigUint<N> {
        Pow::pow(self, &exp)
    }
}

macro_rules! pow_impl {
    ($T:ty) => {
        impl<const N: usize> Pow<$T> for BigUint<N> {
            type Output = BigUint<N>;

            fn pow(self, mut exp: $T) -> BigUint<N> {
                if exp == 0 {
                    return BigUint::one();
                }
                let mut base = self;

                while exp & 1 == 0 {
                    base = &base * &base;
                    exp >>= 1;
                }

                if exp == 1 {
                    return base;
                }

                let mut acc = base.clone();
                while exp > 1 {
                    exp >>= 1;
                    base = &base * &base;
                    if exp & 1 == 1 {
                        acc *= &base;
                    }
                }
                acc
            }
        }

        impl<const N: usize> Pow<&$T> for BigUint<N> {
            type Output = BigUint<N>;

            #[inline]
            fn pow(self, exp: &$T) -> BigUint<N> {
                Pow::pow(self, *exp)
            }
        }

        impl<const N: usize> Pow<$T> for &BigUint<N> {
            type Output = BigUint<N>;

            #[inline]
            fn pow(self, exp: $T) -> BigUint<N> {
                if exp == 0 {
                    return BigUint::one();
                }
                Pow::pow(self.clone(), exp)
            }
        }

        impl<const N: usize> Pow<&$T> for &BigUint<N> {
            type Output = BigUint<N>;

            #[inline]
            fn pow(self, exp: &$T) -> BigUint<N> {
                Pow::pow(self, *exp)
            }
        }
    };
}

pow_impl!(u8);
pow_impl!(u16);
pow_impl!(u32);
pow_impl!(u64);
pow_impl!(usize);
pow_impl!(u128);

pub(super) fn modpow<const N: usize>(
    x: &BigUint<N>,
    exponent: &BigUint<N>,
    modulus: &BigUint<N>,
) -> BigUint<N> {
    assert!(
        !modulus.is_zero(),
        "attempt to calculate with zero modulus!"
    );

    if modulus.is_odd() {
        // For an odd modulus, we can use Montgomery multiplication in base 2^32.
        monty_modpow(x, exponent, modulus)
    } else {
        // Otherwise do basically the same as `num::pow`, but with a modulus.
        plain_modpow(x, &exponent.data, modulus)
    }
}

fn plain_modpow<const N: usize>(
    base: &BigUint<N>,
    exp_data: &[BigDigit],
    modulus: &BigUint<N>,
) -> BigUint<N> {
    assert!(
        !modulus.is_zero(),
        "attempt to calculate with zero modulus!"
    );

    let i = match exp_data.iter().position(|&r| r != 0) {
        None => return BigUint::one(),
        Some(i) => i,
    };

    let mut base = base % modulus;
    for _ in 0..i {
        for _ in 0..big_digit::BITS {
            base = &base * &base % modulus;
        }
    }

    let mut r = exp_data[i];
    let mut b = 0u8;
    while r.is_even() {
        base = &base * &base % modulus;
        r >>= 1;
        b += 1;
    }

    let mut exp_iter = exp_data[i + 1..].iter();
    if exp_iter.len() == 0 && r.is_one() {
        return base;
    }

    let mut acc = base.clone();
    r >>= 1;
    b += 1;

    {
        let mut unit = |exp_is_odd| {
            base = &base * &base % modulus;
            if exp_is_odd {
                acc *= &base;
                acc %= modulus;
            }
        };

        if let Some(&last) = exp_iter.next_back() {
            // consume exp_data[i]
            for _ in b..big_digit::BITS {
                unit(r.is_odd());
                r >>= 1;
            }

            // consume all other digits before the last
            for &r in exp_iter {
                let mut r = r;
                for _ in 0..big_digit::BITS {
                    unit(r.is_odd());
                    r >>= 1;
                }
            }
            r = last;
        }

        debug_assert_ne!(r, 0);
        while !r.is_zero() {
            unit(r.is_odd());
            r >>= 1;
        }
    }
    acc
}

#[test]
fn test_plain_modpow() {
    let two = &BigUint::<32>::from(2u32);
    let modulus = BigUint::from(0x1100u32);

    let exp = vec![0, 0b1];
    assert_eq!(
        two.pow(0b1_00000000_u32) % &modulus,
        plain_modpow(two, &exp, &modulus)
    );
    let exp = vec![0, 0b10];
    assert_eq!(
        two.pow(0b10_00000000_u32) % &modulus,
        plain_modpow(two, &exp, &modulus)
    );
    let exp = vec![0, 0b110010];
    assert_eq!(
        two.pow(0b110010_00000000_u32) % &modulus,
        plain_modpow(two, &exp, &modulus)
    );
    let exp = vec![0b1, 0b1];
    assert_eq!(
        two.pow(0b1_00000001_u32) % &modulus,
        plain_modpow(two, &exp, &modulus)
    );
    let exp = vec![0b1100, 0, 0b1];
    assert_eq!(
        two.pow(0b1_00000000_00001100_u32) % &modulus,
        plain_modpow(two, &exp, &modulus)
    );
}

#[test]
fn test_pow_biguint() {
    let base = BigUint::<32>::from(5u8);
    let exponent = BigUint::from(3u8);

    assert_eq!(BigUint::from(125u8), base.pow(exponent));
}
