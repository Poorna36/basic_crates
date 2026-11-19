//! # basic_math_number_theory
//!
//! Small tools for working with a list of numbers: GCD, LCM,
//! and finding prime numbers.
//!
//!
//! use basic_math_number_theory::NumStats;
//!
//! let data = vec![12, 18, 24, 7, 11];
//! let stats = NumStats::new(&data);
//!
//! assert_eq!(stats.gcd(), Ok(1));
//!
//! assert_eq!(stats.lcm(), Ok(5544));
//!
//! assert_eq!(stats.prime_count(), 2);
//!
//! assert_eq!(stats.primes(), vec![7, 11]);
//!
pub mod error;
pub mod gcd;
pub mod lcm;
pub mod prime;
pub mod stats;

pub use error::{NumStatsError, Result};
pub use gcd::gcd;
pub use lcm::lcm;
pub use prime::is_prime;
pub use stats::NumStats;
