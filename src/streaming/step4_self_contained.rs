//! Step 4: a self-contained output.
//!
//! Goal: one byte vector holds everything; no separate state, no length.
//!
//! Append the final state to the written bytes as 8 bytes (`to_le_bytes`).
//! The decoder reads the state from the last 8 bytes, then decodes as in
//! step 3, reading bytes from the end towards the start. It stops when
//! `x == L` and no bytes remain.
//!
//! Implement `encode` and `decode`. Run `cargo test streaming::step4`.
//! You may use step 3's `L` and `encode`
//! (`super::step3_state_interval::{L, encode}`).
//!
//! Hints:
//! - `u64::from_le_bytes(slice.try_into().unwrap())` reads 8 bytes back.
//! - Why the stop rule works: until the encoder writes its first byte, every
//!   letter makes x larger, so x is L with an empty output only at the start.
//!   After that the output is never empty again. The decoder goes through the
//!   same states in reverse, so both conditions hold together only at the end.
//! - Checking `x == L` alone is not enough: the state can return to L after
//!   reading a byte back, while bytes remain.


pub const L: u64 = super::K << 32;
pub const H: u64 = L * 256;


pub fn encode(text: &str) -> Vec<u8> {
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

    bytes.extend(state.to_le_bytes());
    bytes
}

pub fn decode(encoded: &[u8]) -> String {
    let mut chars: Vec<char> = vec![];
    // The last 8 bytes hold the final state. `from_le_bytes` takes a `[u8; 8]`, and
    // `try_into` converts the 8-byte slice into one.
    let mut state = u64::from_le_bytes(encoded[encoded.len() - 8..].try_into().unwrap());
    let bytes = &encoded[0..encoded.len() - 8];
    // Position just past the next byte to read; bytes are read back last written first.
    let mut i = bytes.len();


    // Stop when no bytes remain and state == L. Until encode writes its first byte,
    // every letter makes state larger, so this pair occurs only at the start of
    // encode. state == L alone is not enough: it can equal L partway through.
    while i > 0 || state != L {
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
        let e = encode("");
        assert_eq!(e.len(), 8);
        assert_eq!(decode(&e), "");
    }

    #[test]
    fn round_trip_random() {
        let mut rng = Rng(0x94d0_49bb_1331_11eb);
        for _ in 0..300 {
            let n = rng.below(2001);
            let s = random_text(&mut rng, n);
            let e = encode(&s);
            let bound = size_bound(n, 8.0) + 8;
            assert!(e.len() <= bound, "{} bytes, bound {bound}", e.len());
            assert_eq!(decode(&e), s);
        }
    }
}
