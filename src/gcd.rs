//! Greatest Common Divisior

/// GCD of two numbers using basic Euclid's Algorithm
///
///'gcd(0,n)==n' and 'gcd(0,0)==0'
///
/// use basic_math-number_theory::gcd
/// assert_eq!(gcd(54,12),6);
/// assert_eq!(gcd(0,5),5);
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 5), 1);
    }

    #[test]
    fn with_zero() {
        assert_eq!(gcd(0, 7), 7);
        assert_eq!(gcd(7, 0), 7);
        assert_eq!(gcd(0, 0), 0);
    }
}
