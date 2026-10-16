//! Exercises: streaming encoding (renormalization).
//!
//! `Coder::encode` stores the whole text in one number. That number grows by
//! about log2(26) bits per letter, so a u64 holds at most 13 letters. Streaming
//! keeps the state in a fixed range: whenever the state would grow too large,
//! the encoder writes out its low byte and shifts it away. The decoder reads
//! the bytes back in reverse order.
//!
//! How to work: each step file has a doc comment and functions whose bodies are
//! `todo!()`. Implement them in order and run that step's tests, for example
//! `cargo test streaming::step1`.
//!
//! - Step 1: start the state at 1 instead of 0, use digits 0..25.
//! - Step 2: write out low bytes; record how many bytes each letter wrote.
//! - Step 3: keep the state in an interval so the decoder needs no counts.
//! - Step 4: put the final state into the output; one byte vector, no length.
//! - Step 5: reverse the encoding order so the decoder yields letters in order.
//! - Step 6: write 32-bit words instead of bytes; choose the constants.
//!
//! All texts in these steps are lowercase a-z only.
#![allow(unused_variables, dead_code)]

pub mod step1_start_state;
pub mod step2_emit_with_counts;
pub mod step3_state_interval;
pub mod step4_self_contained;
pub mod step5_forward_decoding;
pub mod step6_word_size;

/// Alphabet size.
pub const K: u64 = 26;

/// 'a' -> 0, ..., 'z' -> 25.
pub fn index(c: char) -> u64 {
    assert!(c.is_ascii_lowercase(), "not a-z: {c:?}");
    (c as u8 - b'a') as u64
}

/// 0 -> 'a', ..., 25 -> 'z'.
pub fn letter(i: u64) -> char {
    assert!(i < K, "not a letter index: {i}");
    (b'a' + i as u8) as char
}

#[cfg(test)]
pub(crate) mod test_util {
    use super::{K, letter};

    // xorshift64, so the tests need no dependencies and are reproducible.
    pub struct Rng(pub u64);

    impl Rng {
        pub fn below(&mut self, n: usize) -> usize {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            (self.0 % n as u64) as usize
        }
    }

    pub fn random_text(rng: &mut Rng, len: usize) -> String {
        (0..len).map(|_| letter(rng.below(K as usize) as u64)).collect()
    }

    /// ceil(n * log2(26) / bits): the size, in units of `bits`, that n letters need.
    pub fn size_bound(n: usize, bits: f64) -> usize {
        ((n as f64) * (K as f64).log2() / bits).ceil() as usize
    }
}
