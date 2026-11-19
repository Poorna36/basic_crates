// Checks whether a number is prime.
// lOgic: A prime is a number greater than 1 that only divides by 1 and itself.

/// Returns true if `n` is a prime number.
///
/// 0 and 1 are not prime. 2 is the only even prime.
///
/// use basic_math_number_theory::prime::is_prime;
/// assert!(is_prime(29));
/// assert!(!is_prime(1));

pub fn is_prime(n: u64) ->bool
{
    if n<2
    {
        return false;
    }
    if n==2
    {
        return true;
    }

    if n%2==0
    {
        return false;
    }

    let mut i=3;

    while i<=(n/i)
    {
        if n%i==0
        {
            return false;
        }
        i=i+2;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_numbers() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
    }

    #[test]
    fn primes_and_non_primes() {
        assert!(is_prime(29));
        assert!(is_prime(97));
        assert!(!is_prime(25)); // 5 * 5
        assert!(!is_prime(91)); // 7 * 13, looks prime but isn't
    }

    #[test]
    fn big_prime() {
        assert!(is_prime(1_000_000_007));
    }
}