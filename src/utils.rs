//! Utility functions and various helpers.

use rand::rngs::OsRng;

/// Creates Random Number Generator (RNG).
pub fn rng() -> OsRng {
    Default::default()
}
