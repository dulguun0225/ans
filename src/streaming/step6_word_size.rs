//! Step 6: 32-bit words.
//!
//! Goal: write 32-bit words instead of bytes, with a u64 state.
//!
//! The method is step 4's (or step 5's) with `u32` in place of `u8` and a
//! shift of 32 in place of 8. You choose the constants. They must satisfy:
//! - the interval `[L, 2^32 * L)` fits in a u64;
//! - L is a multiple of 26, so the encoder threshold is exact;
//! - `x * 26 + 25` does not overflow for any x the encoder multiplies.
//!
//! The output format is your choice (final state at the end and decode from
//! the end, as in step 4, or at the start and decode forward, as in step 5),
//! except that the final state takes exactly 2 words.
//!
//! Implement `encode` and `decode`. Run `cargo test streaming::step6`.
//!
//! Hints:
//! - Write the three conditions as inequalities on L and find the largest
//!   power of two times 26 that satisfies them.
//! - The first condition gives `L <= 2^32`.
//! - With 32-bit words, at most one word is written or read per letter, so
//!   `while` can become `if`. Check this with your L: after `x /= 26`, is one
//!   word always enough to bring x back to at least L?

pub fn encode(text: &str) -> Vec<u32> {
    todo!()
}

pub fn decode(encoded: &[u32]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::streaming::test_util::{Rng, random_text, size_bound};

    #[test]
    fn empty() {
        let e = encode("");
        assert_eq!(e.len(), 2);
        assert_eq!(decode(&e), "");
    }

    #[test]
    fn round_trip_random() {
        let mut rng = Rng(0x6a09_e667_f3bc_c908);
        for _ in 0..300 {
            let n = rng.below(2001);
            let s = random_text(&mut rng, n);
            let e = encode(&s);
            let bound = size_bound(n, 32.0) + 2;
            assert!(e.len() <= bound, "{} words, bound {bound}", e.len());
            assert_eq!(decode(&e), s);
        }
    }
}
