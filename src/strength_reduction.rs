
use std::ops::{Div, Rem};

// We have to do a lot of division and mod operations when transposing in-place.
// We can make them faster by using "strength-reduction", where we replace division by multiplication and shifting.
#[derive(Clone, Copy, Debug)]
pub(crate) struct StrengthReducedUsize {
	multiplier: u128,
	divisor: usize,
	shift_value: u32,
}

impl StrengthReducedUsize {
	pub(crate) fn new(divisor: usize) -> Self {
		assert!(divisor > 0);
		if divisor == 1 { 
			Self{ multiplier: 1u128 << 64, divisor, shift_value: 0}
		} else {
			let big_divisor = divisor as u128;
			let trailing_zeros = big_divisor.next_power_of_two().trailing_zeros();

			Self {
				multiplier: ((1u128 << trailing_zeros + 63) + big_divisor - 1) / big_divisor,
				divisor,
				shift_value: trailing_zeros - 1
			}
		}
	}

	pub(crate) fn div_rem(numerator: usize, denom: StrengthReducedUsize) -> (usize, usize) {
		let quotient = numerator / denom;
		let remainder = numerator - quotient * denom.divisor;
		(quotient, remainder)
	}

	pub(crate) fn get(&self) -> usize {
		self.divisor
	}
}

impl Div<StrengthReducedUsize> for usize {
	type Output = usize;

	fn div(self, rhs: StrengthReducedUsize) -> Self::Output {
		let multiplied = ((self as u128) * rhs.multiplier) >> 64;
		(multiplied >> rhs.shift_value) as usize
	}
}

impl Rem<StrengthReducedUsize> for usize {
	type Output = usize;

	fn rem(self, rhs: StrengthReducedUsize) -> Self::Output {
		let quotient = self / rhs;
		self - quotient * rhs.divisor
	}
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use num_integer;

    #[test]
    fn test_strength_reduced_div() {
    	let divisors: Vec<usize> =   (1..20).chain([std::usize::MAX - 1, std::usize::MAX].iter().map(|item| *item)).collect();
    	let numerators: Vec<usize> = (0..20).chain([std::usize::MAX - 1, std::usize::MAX].iter().map(|item| *item)).collect();

    	for &divisor in &divisors {
    		let reduced_divisor = StrengthReducedUsize::new(divisor);
    		for &numerator in &numerators {
    			let expected_div = numerator / divisor;
    			let reduced_div = numerator / reduced_divisor;

    			assert_eq!(expected_div, reduced_div, "Failed with numerator: {}, divisor: {}", numerator, divisor);
    		}
    	}
    }

    #[test]
    fn test_strength_reduced_rem() {
    	let divisors: Vec<usize> =   (1..20).chain([std::usize::MAX - 1, std::usize::MAX].iter().map(|item| *item)).collect();
    	let numerators: Vec<usize> = (0..20).chain([std::usize::MAX - 1, std::usize::MAX].iter().map(|item| *item)).collect();

    	for &divisor in &divisors {
    		let reduced_divisor = StrengthReducedUsize::new(divisor);
    		for &numerator in &numerators {
    			let expected_rem = numerator % divisor;
    			let reduced_rem = numerator % reduced_divisor;

    			assert_eq!(expected_rem, reduced_rem, "Failed with numerator: {}, divisor: {}", numerator, divisor);
    		}
    	}
    }

    #[test]
    fn test_strength_reduced_div_rem() {
    	let divisors: Vec<usize> =   (1..20).chain([std::usize::MAX - 1, std::usize::MAX].iter().map(|item| *item)).collect();
    	let numerators: Vec<usize> = (0..20).chain([std::usize::MAX - 1, std::usize::MAX].iter().map(|item| *item)).collect();

    	for &divisor in &divisors {
    		let reduced_divisor = StrengthReducedUsize::new(divisor);
    		for &numerator in &numerators {
    			let (reduced_div, reduced_rem) = StrengthReducedUsize::div_rem(numerator, reduced_divisor);
    			let (expected_div, expected_rem) = num_integer::div_rem(numerator, divisor);

    			assert_eq!(expected_div, reduced_div, "Failed with numerator: {}, divisor: {}", numerator, divisor);
    			assert_eq!(expected_rem, reduced_rem, "Failed with numerator: {}, divisor: {}", numerator, divisor);
    		}
    	}
    }
}
