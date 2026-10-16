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

pub fn encode(text: &str) -> Vec<u8> {
    todo!()
}

pub fn decode(encoded: &[u8]) -> String {
    todo!()
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
