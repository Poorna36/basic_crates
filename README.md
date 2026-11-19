# basic_math-number_theory

[![Crates.io](https://crates.io/crates/basic_math-number_theory)
[![Docs.rs](https://docs.rs/basic_math-number_theory)

A small, dependency-free Rust crate for basic number theory on a list of numbers. Pass it a reference to a vector (or slice) and get the **GCD**, **LCM**, and the **prime numbers** it contains, with clear error handling and no panics on bad input.

## Features

- **GCD** of two numbers or a whole list (Euclid's algorithm)
- **LCM** of two numbers or a whole list, with overflow detection
- **Prime detection**: check one number, list the primes in a vector, or count them
- **Borrows your data**: your vector is not copied or moved
- **Typed errors**: `EmptyInput`, `ZeroInLcm` and `Overflow` instead of panics
- **Zero dependencies**

## Installation

Add  to  `Cargo.toml`:

```toml
[dependencies]
basic_math-number_theory = "0.1.0"
```

Or run:

```
cargo add basic_math-number_theory
```

> In Rust code the crate name is written with underscores: `basic_math_number_theory`.

## Quick start

```rust
use basic_math_number_theory::NumStats;

fn main() {
    let data = vec![12, 18, 24, 7, 11];
    let stats = NumStats::new(&data);

    println!("GCD: {:?}", stats.gcd());              // Ok(1)
    println!("LCM: {:?}", stats.lcm());              // Ok(5544)
    println!("Prime count: {}", stats.prime_count()); // 2
    println!("Primes: {:?}", stats.primes());        // [7, 11]
}
```

## Using the standalone functions

```rust
use basic_math_number_theory::{gcd, lcm, is_prime};

assert_eq!(gcd(48, 18), 6);
assert_eq!(lcm(4, 6), Ok(12));
assert!(is_prime(97));
```

## Error handling

Operations that can fail return `Result<T, NumStatsError>`.

| Error | When it happens |
|-------|-----------------|
| `EmptyInput` | `gcd()` or `lcm()` called on an empty vector |
| `ZeroInLcm` | The input contains `0`, for which LCM is undefined |
| `Overflow` | The LCM does not fit in a `u64` |

```rust
use basic_math_number_theory::{NumStats, NumStatsError};

let empty: Vec<u64> = vec![];
assert_eq!(NumStats::new(&empty).gcd(), Err(NumStatsError::EmptyInput));

let with_zero = vec![3, 0];
assert_eq!(NumStats::new(&with_zero).lcm(), Err(NumStatsError::ZeroInLcm));

let huge = vec![u64::MAX, u64::MAX - 1];
assert_eq!(NumStats::new(&huge).lcm(), Err(NumStatsError::Overflow));
```

## API overview

| Item | Description |
|------|-------------|
| `NumStats::new(&[u64])` | Creates a borrowed view over your numbers |
| `NumStats::gcd()` | GCD of all elements, `Result<u64>` |
| `NumStats::lcm()` | LCM of all elements, `Result<u64>` |
| `NumStats::primes()` | Prime elements in original order, `Vec<u64>` (duplicates kept) |
| `NumStats::prime_count()` | Number of prime elements, `usize` |
| `gcd(a, b)` | GCD of two numbers. `gcd(0, n) == n`, `gcd(0, 0) == 0` |
| `lcm(a, b)` | LCM of two numbers, `Result<u64>` |
| `is_prime(n)` | `true` if `n` is prime |

Full documentation is on [docs.rs](https://docs.rs/basic_math-number_theory).

## Project structure

```
src/
├── lib.rs     # crate docs and public re-exports
├── error.rs   # NumStatsError and Result alias
├── gcd.rs     # greatest common divisor
├── lcm.rs     # least common multiple
├── prime.rs   # primality check
└── stats.rs   # NumStats, which works on whole vectors
```


## Running the tests

```
cargo test
```


