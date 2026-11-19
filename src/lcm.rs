//! Least Common Multiples

/// we can use the gcd to find lcm
use crate::error::{NumStatsError, Result};
use crate::gcd::gcd;

///LCM of two numbers
///
/// Sfely handles errors if number is 0 or if the result is to be for 64 bytes.
///
///use basic_math-number_theory::lcm::lcm;
///
/// assert_eq!(lcm(4,6),OK(12));
///
pub fn lcm(a: u64, b: u64) -> Result<u64> {
    if a == 0 || b == 0 {
        return Err(NumStatsError::ZeroInLcm);
    }

    let divide = a / gcd(a, b); // small otpmisation to keep the number small by dividing first

    match divide.checked_mul(b) {
        Some(ans) => Ok(ans),
        None => Err(NumStatsError::Overflow),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(lcm(4, 6), Ok(12));
        assert_eq!(lcm(7, 5), Ok(35));
    }

    #[test]
    fn same_numbers() {
        assert_eq!(lcm(9, 9), Ok(9));
    }

    #[test]
    fn zero_is_an_error() {
        assert_eq!(lcm(0, 5), Err(NumStatsError::ZeroInLcm));
        assert_eq!(lcm(5, 0), Err(NumStatsError::ZeroInLcm));
    }

    #[test]
    fn too_big_is_an_error() {
        // These two are next to each other, so their gcd is 1
        // and the lcm would be about u64::MAX squared.
        assert_eq!(lcm(u64::MAX, u64::MAX - 1), Err(NumStatsError::Overflow));
    }
}
