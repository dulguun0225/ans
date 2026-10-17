//! Step 2: write out low bytes, with counts.
//!
//! Goal: encode texts of any length by keeping x at most `LIMIT`.
//!
//! Before encoding each letter, while `x > LIMIT`, push the low byte of x
//! (`x as u8`) to `bytes` and shift it away (`x >>= 8`). Record in `counts` how
//! many bytes were pushed before that letter. Then `x = x * 26 + s`.
//! The start state is step 1's `START`.
//!
//! The decoder undoes the steps in reverse order. For each letter, last first:
//! `s = x % 26; x /= 26;` then read back that letter's bytes, last pushed
//! first: `x = (x << 8) | byte`. Writing out a byte and reading it back are
//! exact inverses, so x returns to the same value it had in the encoder.
//!
//! The problem left: the decoder needs `counts`, one extra byte per letter.
//! Step 3 removes it.
//!
//! Implement `encode` and `decode`. Run `cargo test streaming::step2`.
//!
//! Hints:
//! - The number of letters is `counts.len()`.
//! - `bytes.pop()` gives the last pushed byte. `decode` takes `&Encoded`, so
//!   clone `bytes` or keep a position that moves down.
//! - Decoder, per letter, from the last: take `s` from x, then read
//!   `counts[i]` bytes.

/// x <= LIMIT before each letter, so x * 26 + 25 <= u64::MAX. No larger value has this property.
pub const LIMIT: u64 = (u64::MAX - super::K + 1)/super::K;

#[derive(Clone)]
pub struct Encoded {
    pub state: u64,
    pub bytes: Vec<u8>,
    pub counts: Vec<u8>,
}

pub fn encode(text: &str) -> Encoded {
    let mut encoded: Encoded = Encoded {
        state: 1,
        bytes: vec![],
        counts: vec![],
    };

    for c in text.chars() {
        let mut pushed = 0;
        while LIMIT < encoded.state {
            encoded.bytes.push(encoded.state as u8);
            encoded.state >>= 8;
            pushed += 1;
        }

        encoded.counts.push(pushed);

        encoded.state = encoded.state * super::K + super::index(c);
    }

    encoded
}

pub fn decode(encoded: &Encoded) -> String {
    let mut encoded = encoded.clone();
    let mut chars: Vec<char> = vec![];

    while 1 < encoded.state {

        chars.push(super::letter(encoded.state % 26));
        encoded.state /= 26;

        if let Some(count) = encoded.counts.pop() {

            for _ in 0..count {
                encoded.state <<= 8;
                encoded.state += encoded.bytes.pop().unwrap() as u64;
            }
        }
    }

    chars.reverse();
    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::streaming::K;
    use crate::streaming::step1_start_state::START;
    use crate::streaming::test_util::{Rng, random_text};

    #[test]
    fn empty() {
        let e = encode("");
        assert_eq!(e.state, START);
        assert!(e.bytes.is_empty());
        assert!(e.counts.is_empty());
        assert_eq!(decode(&e), "");
    }

    #[test]
    fn round_trip_random() {
        let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
        for _ in 0..300 {
            let len = rng.below(2001);
            let s = random_text(&mut rng, len);
            let e = encode(&s);
            assert_eq!(e.counts.len(), len);
            // x <= LIMIT before the last letter, so the state is at most LIMIT * 26 + 25.
            assert!(e.state <= LIMIT * K + (K - 1), "state {} not bounded", e.state);
            assert_eq!(decode(&e), s);
        }
    }
}
