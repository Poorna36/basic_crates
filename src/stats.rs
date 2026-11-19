use crate::error::{NumStatsError, Result};
use crate::gcd::gcd;
use crate::lcm::lcm;
use crate::prime::is_prime;

/// Borrows a list of numbers and calculates things about them.
///
/// use basic_math_number_theory::stats::NumStats;
///
/// let v = vec![12, 18, 7, 11];
/// let stats = NumStats::new(&v);
/// assert_eq!(stats.gcd(), Ok(1));
/// assert_eq!(stats.prime_count(), 2);
/// assert_eq!(stats.primes(), vec![7, 11]);
///

pub struct NumStats<'a> {
    data: &'a [u64],
}

impl<'a> NumStats<'a> {
    /// Creates a NumStats from a reference to a vector or slice.
    pub fn new(data: &'a [u64]) -> Self {
        NumStats { data }
    }

    /// GCD of all the numbers.
    ///
    /// Returns `EmptyInput` if there are no numbers.

    pub fn gcd(&self) -> Result<u64> {
        if self.data.is_empty() {
            return Err(NumStatsError::EmptyInput);
        }

        let mut result = self.data[0];
        for &n in &self.data[1..] {
            result = gcd(result, n);
        }
        Ok(result)
    }

    /// LCM of all the numbers.
    ///
    /// Returns "EmptyInput` if there are no numbers,
    /// "ZeroInLcm" if any number is 0
    /// "Overflow" if the answer is too big for a u64.

    pub fn lcm(&self) -> Result<u64> {
        if self.data.is_empty() {
            return Err(NumStatsError::EmptyInput);
        }
        if self.data[0] == 0 {
            return Err(NumStatsError::ZeroInLcm);
        }

        let mut result = self.data[0];
        for &n in &self.data[1..] {
            result = lcm(result, n)?;
        }
        Ok(result)
    }

    /// All the prime numbers in the list with the the same order (with duplicattes)

    pub fn primes(&self) -> Vec<u64> {
        let mut found = Vec::new();
        for &n in self.data {
            if is_prime(n) {
                found.push(n);
            }
        }
        found
    }

    /// count of prime numbers

    pub fn prime_count(&self) -> usize {
        self.primes().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_of_many() {
        let z = vec![12, 18, 24];
        assert_eq!(NumStats::new(&z).gcd(), Ok(6));
    }

    #[test]
    fn lcm_of_many() {
        let z = vec![4, 6, 10];
        assert_eq!(NumStats::new(&z).lcm(), Ok(60));
    }

    #[test]
    fn single_number() {
        let z = vec![7];
        assert_eq!(NumStats::new(&z).gcd(), Ok(7));
        assert_eq!(NumStats::new(&z).lcm(), Ok(7));
    }

    #[test]
    fn empty_vector_is_an_error() {
        let z: Vec<u64> = vec![];
        assert_eq!(NumStats::new(&z).gcd(), Err(NumStatsError::EmptyInput));
        assert_eq!(NumStats::new(&z).lcm(), Err(NumStatsError::EmptyInput));
    }

    #[test]
    fn zero_in_lcm_is_an_error() {
        let first = vec![0, 5];
        let last = vec![5, 0];
        assert_eq!(NumStats::new(&first).lcm(), Err(NumStatsError::ZeroInLcm));
        assert_eq!(NumStats::new(&last).lcm(), Err(NumStatsError::ZeroInLcm));
    }

    #[test]
    fn lcm_overflow_is_an_error() {
        let z = vec![u64::MAX, u64::MAX - 1];
        assert_eq!(NumStats::new(&z).lcm(), Err(NumStatsError::Overflow));
    }

    #[test]
    fn primes_and_count() {
        let z = vec![1, 2, 3, 4, 5, 5, 9];
        let stats = NumStats::new(&z);
        assert_eq!(stats.primes(), vec![2, 3, 5, 5]);
        assert_eq!(stats.prime_count(), 4);
    }

    #[test]
    fn no_primes() {
        let z = vec![0, 1, 4, 6];
        let stats = NumStats::new(&z);
        assert_eq!(stats.primes(), vec![]);
        assert_eq!(stats.prime_count(), 0);
    }
}
