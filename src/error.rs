// All the ways our functions can go wrong live in this one enum.
// Each variant is a different kind of failure.

use std::fmt;

// Debug lets us print the error with {:?}.
// PartialEq lets us compare errors in tests with assert_eq!.
#[derive(Debug, PartialEq)]
pub enum NumStatsError {
    // The vector had nothing in it, so there is nothing to calculate.
    EmptyInput,
    // LCM doesn't make sense when one of the numbers is 0.
    ZeroInLcm,
    // The answer is too big to fit inside a u64.
    Overflow,
}

// Display decides what the user sees when we print the error with {}.
impl fmt::Display for NumStatsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            NumStatsError::EmptyInput => "input is empty",
            NumStatsError::ZeroInLcm => "LCM is undefined when input contains zero",
            NumStatsError::Overflow => "result is too big for u64",
        };
        write!(f, "{}", message)
    }
}

// This empty impl marks our type as a proper Rust error.
// It lets other code use it with `?` and Box<dyn Error>.
impl std::error::Error for NumStatsError {}

// Shortcut so we can write Result<u64> instead of
// Result<u64, NumStatsError> everywhere.
pub type Result<T> = std::result::Result<T, NumStatsError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_messages_are_readable() {
        assert_eq!(NumStatsError::EmptyInput.to_string(), "input is empty");
        assert_eq!(
            NumStatsError::Overflow.to_string(),
            "result is too big for u64"
        );
    }
}