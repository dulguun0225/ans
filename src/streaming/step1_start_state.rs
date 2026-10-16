//! Step 1: a nonzero start state.
//!
//! Goal: encode a text into one u64 with digits 0..25, starting from `START = 1`.
//!
//! `Coder::encode` uses digits 1..26 (`index + 1`) and starts at 0. The
//! streaming steps need digits 0..25, so that `x % 26` is the letter index
//! directly. With digits 0..25 and a start of 0, 'a' (digit 0) leaves x at 0,
//! so "a", "aa" and "" all encode to 0. Starting at 1 fixes this: every letter
//! multiplies by 26, so "a" gives 26, "aa" gives 676, and "" gives 1.
//!
//! ```text
//! encode: x = START; for each letter s: x = x * 26 + s
//! decode: while x != START: s = x % 26; x = x / 26
//! ```
//!
//! The decoder gets the last letter first, so reverse the result at the end.
//!
//! Implement `encode` and `decode`. Run `cargo test streaming::step1`.
//!
//! Hints:
//! - Use `super::index` and `super::letter` to convert between char and digit.
//! - The decoder knows it is done when x is back at the value the encoder
//!   started from.
//! - 13 letters fit in a u64; 14 'z's would overflow (26^14 > 2^64).

pub const START: u64 = 1;

pub fn encode(text: &str) -> u64 {
    text.chars().fold(1, |a, b| a * 26 + super::index(b))
}

pub fn decode(mut x: u64) -> String {
    let mut chars = vec![];

    while x > 1 {
        chars.push(super::letter(x % 26));
        x = x / 26;
    }

    chars.reverse();
    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::streaming::K;
    use crate::streaming::test_util::{Rng, random_text};

    #[test]
    fn empty_is_start() {
        assert_eq!(encode(""), START);
    }

    #[test]
    fn leading_a_is_kept() {
        assert_ne!(encode("a"), encode("aa"));
        assert_ne!(encode(""), encode("a"));
    }

    #[test]
    fn round_trip_fixed() {
        for s in ["", "a", "aaa", "abc", &"z".repeat(13)] {
            assert_eq!(decode(encode(s)), s);
        }
    }

    // 13 'z's encode to 2 * 26^13 - 1, which fits in u64; 14 'z's do not fit.
    #[test]
    fn thirteen_letters_is_the_limit() {
        let fits =
            |len: usize| (0..len).try_fold(START, |x, _| x.checked_mul(K)?.checked_add(K - 1));
        assert!(fits(13).is_some());
        assert!(fits(14).is_none());
    }

    #[test]
    fn round_trip_random() {
        let mut rng = Rng(0x2545_f491_4f6c_dd1d);
        for _ in 0..1000 {
            let len = rng.below(14);
            let s = random_text(&mut rng, len);
            assert_eq!(decode(encode(&s)), s);
        }
    }
}
