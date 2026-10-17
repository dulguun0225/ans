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

// The three conditions:
// - [L, 2^32 * L) fits in a u64: 2^32 * L <= 2^64, so L <= 2^32.
// - L is a multiple of 26: L = 26 * 2^k.
// - x * 26 + 25 does not overflow: encode multiplies only x < (L / 26) * 2^32, so
//   x * 26 + 25 < L * 2^32 <= 2^64 by the first condition.
// 26 * 2^27 < 2^32 < 26 * 2^28, so k = 27.
pub const L: u64 = super::K << 27;
pub const H: u64 = L << 32;

pub fn encode(text: &str) -> Vec<u32> {
    let mut state = L;
    let mut words = vec![];

    // Encode the last letter first, so decode yields the text in order (step 5).
    for c in text.chars().rev() {
        // Step 4's threshold with 2^32 in place of 256. state < H = 26 * 2^59, so
        // one shift brings it below L, which is below the threshold 2^59: one word
        // per letter at most.
        if (L / super::K) << 32 <= state {
            words.push(state as u32);
            state >>= 32;
        }

        state = state * super::K + super::index(c);
    }

    // The final state first, as 2 words, low word first; then the words last
    // written first.
    let mut out = vec![state as u32, (state >> 32) as u32];
    out.extend(words.iter().rev());
    out
}

pub fn decode(encoded: &[u32]) -> String {
    let mut state = encoded[0] as u64 | (encoded[1] as u64) << 32;
    let mut i = 2;
    let mut text = String::new();

    // Step 5's stop rule: no words remain and state == L.
    while i < encoded.len() || state != L {
        text.push(super::letter(state % super::K));
        state /= super::K;

        // state >= L / 26 = 2^27 here, so one word makes it at least 2^59 >= L.
        if state < L {
            state = (state << 32) | encoded[i] as u64;
            i += 1;
        }
    }

    text
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
