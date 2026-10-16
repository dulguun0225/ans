//! Step 5: decode from the start.
//!
//! Goal: a decoder that yields letters in text order, one at a time.
//!
//! So far the decoder reads from the end and yields the last letter first, so
//! it needs the whole input before it produces anything. Reverse everything
//! in the encoder instead:
//! - encode the letters in reverse order (`text.chars().rev()`);
//! - output the final state first (8 bytes, `to_le_bytes`), followed by the
//!   written bytes in reverse order of writing.
//!
//! Then the decoder reads from the start: the state from the first 8 bytes,
//! then one byte at a time forward. The first letter it decodes is the last
//! one encoded, which is the first letter of the text.
//!
//! Implement `encode`, `Decoder::new` and `Decoder::next`. The fields of
//! `Decoder` are a suggestion; change them if you like. Stop rule:
//! `x == L` and `pos == bytes.len()`. Run `cargo test streaming::step5`.
//!
//! Hints:
//! - Step 3's `encode` gives the state and bytes; only the order changes.
//! - `next` is one iteration of step 4's decode loop, returning one letter.
//! - The stop rule is step 4's rule with the byte order reversed: "no bytes
//!   remain" is now `pos == bytes.len()`.

pub fn encode(text: &str) -> Vec<u8> {
    todo!()
}

pub struct Decoder<'a> {
    bytes: &'a [u8],
    pos: usize,
    x: u64,
}

impl<'a> Decoder<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        todo!()
    }
}

impl Iterator for Decoder<'_> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::streaming::test_util::{Rng, random_text};

    #[test]
    fn empty() {
        let e = encode("");
        assert_eq!(e.len(), 8);
        assert_eq!(Decoder::new(&e).collect::<String>(), "");
    }

    #[test]
    fn round_trip_random() {
        let mut rng = Rng(0xbf58_476d_1ce4_e5b9);
        for _ in 0..300 {
            let n = rng.below(2001);
            let s = random_text(&mut rng, n);
            let e = encode(&s);
            assert_eq!(Decoder::new(&e).collect::<String>(), s);
        }
    }

    #[test]
    fn take_first_letters() {
        let mut rng = Rng(0x2545_f491_4f6c_dd1d);
        let s = random_text(&mut rng, 1000);
        let e = encode(&s);
        assert_eq!(Decoder::new(&e).take(10).collect::<String>(), s[..10]);
    }

    // With L = 26 << 32 and bytes, each letter reads at most one byte: after
    // x /= 26 the state is at least L / 26 = 2^32, and one byte makes it at
    // least 2^40 > L. So m letters need at most the first 8 + m bytes.
    #[test]
    fn decode_from_prefix() {
        let mut rng = Rng(0x0123_4567_89ab_cdef);
        let s = random_text(&mut rng, 1000);
        let e = encode(&s);
        for m in [0, 1, 2, 10, 100, 500, 1000] {
            let prefix = &e[..(8 + m).min(e.len())];
            let decoded: String = Decoder::new(prefix).take(m).collect();
            assert_eq!(decoded, s[..m], "m = {m}");
        }
    }
}
