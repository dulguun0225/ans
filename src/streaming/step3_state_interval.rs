//! Step 3: a state interval instead of counts.
//!
//! Goal: decode without `counts`, so the output has no extra bytes.
//!
//! Keep the state in `[L, 256 * L)` after every letter, and start at `x = L`.
//! The encoder writes out low bytes until `x * 26 + s` lands in that interval
//! again. The decoder then knows when to read a byte back: after
//! `s = x % 26; x /= 26;`, x is below L exactly when the encoder wrote bytes
//! before this letter. So: `while x < L`, read a byte back.
//!
//! ```text
//! encode, per letter:  while x >= THRESHOLD { write low byte; x >>= 8 }
//!                      x = x * 26 + s
//! decode, per letter:  s = x % 26; x /= 26
//!                      while x < L { x = (x << 8) | last written byte }
//! ```
//!
//! Find `THRESHOLD`. `decode` gets the number of letters `n`.
//!
//! Implement `encode` and `decode`. Run `cargo test streaming::step3`.
//!
//! Hints:
//! - After `x >>= 8` the interval is still reachable: each shift divides x by
//!   256, and the interval spans a factor of 256.
//! - Which values of x make `x * 26 + s` land in `[L, 256 * L)` for every s?
//! - Answer: `[L / 26, 256 * L / 26)`. L is a multiple of 26, so this is exact:
//!   the threshold is `(L / 26) * 256`.

pub const L: u64 = super::K << 32;
pub const H: u64 = L * 256;

/// Returns the final state and the written bytes.
pub fn encode(text: &str) -> (u64, Vec<u8>) {
    let mut state = L;
    let mut bytes = vec![];

    for c in text.chars() {

        // `state = state * K + index(c)` must keep state below H for every letter,
        // so for the largest index, K - 1:
        //   state * K + (K - 1) < H
        //   state * K + K <= H          (whole numbers: a < b is the same as a + 1 <= b)
        //   (state + 1) * K <= H
        //   state + 1 <= (L / K) * 256  (H / K, exact because L is a multiple of K)
        //   state < (L / K) * 256
        // The loop runs while this is false. Each shift divides state by 256, so the
        // loop stops at or above L / K, and state * K stays at least L.
        while (L / super::K) * 256 <= state {
            bytes.push(state as u8);
            state >>= 8;
        }

        // state was in [L, H) before the loop. If the loop wrote a byte, state is now
        // below L: state < H, so state >> 8 < H / 256 = L. If it wrote nothing, state
        // is unchanged and still at least L. decode reads this difference.

        // Returns state to [L, H).
        state = state * super::K + super::index(c);
    }

    (state, bytes)
}

pub fn decode(mut state: u64, bytes: &[u8], n: usize) -> String {
    let mut chars: Vec<char> = vec![];
    // Position just past the next byte to read; bytes are read back last written first.
    let mut i = bytes.len();

    // Loop n times: state alone cannot mark the end. It returns to L after the last
    // letter, but it can also equal L partway through.
    for _ in 0..n {
        // Undoes `state = state * K + index(c)`. state is now what encode had after
        // its shift loop for this letter: below L if that loop wrote bytes, at least
        // L if it wrote nothing.
        chars.push(super::letter(state % super::K));
        state /= super::K;

        // Undoes the shifts, last written byte first. Each read multiplies state by
        // 256, so it reaches [L, H) after exactly as many bytes as encode wrote.
        while state < L {
            i -= 1;
            state = (state << 8) | bytes[i] as u64;
        }
    }

    chars.reverse();
    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::streaming::test_util::{Rng, random_text, size_bound};

    #[test]
    fn empty() {
        let (state, bytes) = encode("");
        assert_eq!(state, L);
        assert!(bytes.is_empty());
        assert_eq!(decode(state, &bytes, 0), "");
    }

    #[test]
    fn round_trip_random() {
        let mut rng = Rng(0xd1b5_4a32_d192_ed03);
        for _ in 0..300 {
            let n = rng.below(2001);
            let s = random_text(&mut rng, n);
            let (state, bytes) = encode(&s);
            assert!((L..256 * L).contains(&state), "state {state} out of range");
            // No extra space: the bytes are at most what n letters need.
            let bound = size_bound(n, 8.0);
            assert!(bytes.len() <= bound, "{} bytes, bound {bound}", bytes.len());
            assert_eq!(decode(state, &bytes, n), s);
        }
    }
}
