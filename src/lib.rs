use std::collections::HashMap;

pub mod streaming;

pub struct Coder {
    i2a: Vec<char>,
    a2i: HashMap<char, usize>,
}

impl Coder {
    pub fn new(alphabets: Vec<char>) -> Self {
        let mut a2i: HashMap<char, usize> = HashMap::new();
        for (i, c) in alphabets.iter().enumerate() {
            a2i.insert(*c, i);
        }

        Self {
            i2a: alphabets,
            a2i,
        }
    }

    pub fn encode(&self, original: &str) -> usize {
        let k = self.i2a.len();

        let mut encoded = 1;

        for c in original.chars() {
            let d: usize = *self.a2i.get(&c).unwrap();
            encoded *= k; 
            encoded += d;
        }

        encoded
    }

    pub fn encode_w_formula(&self, original: &str) -> usize {
        let k = self.i2a.len();
        let n = original.chars().count();
        let mut value_in_base_k = 0;

        for c in original.chars() {
            let i = self.a2i.get(&c).unwrap();
            value_in_base_k = value_in_base_k * k + i;
        }

        let encoded = k.pow(n as u32) + value_in_base_k;

        encoded
    }

    pub fn decode(&self, encoded: usize) -> String {
        let mut result = vec![];
        let mut current = encoded;

        while current > 1 {
            let d = current % self.i2a.len();
            let c = self.i2a.get(d).unwrap();
            current = current / self.i2a.len();
            result.push(*c);
        }

        result.reverse();
        result.into_iter().collect()
    }
    // The state x stays in [lower, lower * 256); bytes below that range are emitted.
    pub fn encode_stream(&self, original: &str) -> Vec<u8> {
        let k = self.i2a.len() as u64;
        let lower = k << 32;
        let mut x = lower;
        let mut out = vec![];

        for c in original.chars() {
            let s = self.a2i[&c] as u64;
            while x >= (lower / k) << 8 {
                out.push(x as u8);
                x >>= 8;
            }
            x = x * k + s;
        }

        out.extend(x.to_le_bytes());
        out
    }

    // Decoding runs in reverse order: the last symbol and the last emitted byte come out first.
    pub fn decode_stream(&self, encoded: &[u8]) -> String {
        let k = self.i2a.len() as u64;
        let lower = k << 32;
        let mut pos = encoded.len() - 8;
        let mut x = u64::from_le_bytes(encoded[pos..].try_into().unwrap());
        let mut result = vec![];

        while !(x == lower && pos == 0) {
            let s = x % k;
            x /= k;
            result.push(self.i2a[s as usize]);
            while x < lower {
                pos -= 1;
                x = (x << 8) | encoded[pos] as u64;
            }
        }

        result.reverse();
        result.into_iter().collect()
    }
}

pub fn max_encoded(alphabet_len: usize, text_len: u32) -> usize {
    // k^n for the start digit plus the largest value of n digits, k^n - 1.
    // Written this way so that 2 * k^n - 1 = usize::MAX does not overflow.
    let power = alphabet_len.pow(text_len);
    power + (power - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    // xorshift64, so the test needs no dependencies and is reproducible.
    struct Rng(u64);

    impl Rng {
        fn below(&mut self, n: usize) -> usize {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            (self.0 % n as u64) as usize
        }
    }

    // Longest text whose largest encoding (every letter the last one) fits in usize.
    // That encoding is 2 * alphabet_len^len - 1.
    fn max_text_len(alphabet_len: usize) -> usize {
        let mut len = 0;
        let mut power: usize = 1;
        while let Some(next_power) = power.checked_mul(alphabet_len) {
            // 2 * next_power - 1, computed as 2 * (next_power - 1) + 1 so it can equal usize::MAX.
            let next_max = (next_power - 1).checked_mul(2).and_then(|m| m.checked_add(1));
            if next_max.is_none() {
                break;
            }
            power = next_power;
            len += 1;
        }
        len
    }

    #[test]
    fn encode_list() {
        let alphabets: Vec<char> = ('a'..='z').collect();
        let coder = Coder::new(alphabets.clone());
        let max_text_len = max_text_len(alphabets.len());

        let mut rng = Rng(0x2545_f491_4f6c_dd1d);
        let mut test_strs: Vec<String> = vec![
            "".to_string(),
            alphabets[alphabets.len() - 1]
                .to_string()
                .repeat(max_text_len),
        ];

        for _ in 0..10_000 {
            let len = 1 + rng.below(max_text_len);
            let s = (0..len)
                .map(|_| alphabets[rng.below(alphabets.len())])
                .collect();
            test_strs.push(s);
        }

        for s in test_strs {
            let encoded = coder.encode(&s);
            assert_eq!(coder.decode(encoded), s, "encoded as {encoded}");
        }
    }

    #[test]
    fn encode_head() {
        let alphabets: Vec<char> = ('a'..='b').collect();
        let coder = Coder::new(alphabets.clone());

        let mut head = vec!["".to_string()];

        {
            let mut prev = head.clone();
            for _ in 0..3 {
                let mut current = vec![];

                for s in prev.iter() {
                    for c in alphabets.iter() {
                        let mut next_s = s.clone();
                        next_s.push(*c);
                        current.push(next_s);
                    }
                }

                head.extend_from_slice(&current);

                prev = current;
            }
        }

        for s in head.iter() {
            let encoded = coder.encode(s);
            println!("{encoded}\t\t{s:?}");
        }
    }

    #[test]
    fn decode() {
        let alphabets = vec!['a', 'b', 'c'];
        let coder = Coder::new(alphabets.clone());
        let encoded = coder.encode("cab");

        let _ = coder.decode(encoded);
        // dbg!(decoded);
    }

    #[test]
    fn encode_stream_random() {
        let alphabets: Vec<char> = ('a'..='z').collect();
        let coder = Coder::new(alphabets.clone());
        let k = alphabets.len();

        let mut rng = Rng(0x2545_f491_4f6c_dd1d);
        let mut test_strs: Vec<String> = vec!["".to_string()];
        for _ in 0..1000 {
            let len = 1 + rng.below(2000);
            let s = (0..len).map(|_| alphabets[rng.below(k)]).collect();
            test_strs.push(s);
        }

        for s in test_strs {
            let encoded = coder.encode_stream(&s);
            assert_eq!(coder.decode_stream(&encoded), s);

            let n = s.chars().count();
            let bound = ((n as f64) * (k as f64).log2() / 8.0).ceil() as usize + 8;
            assert!(
                encoded.len() <= bound,
                "{} bytes for {n} chars, bound {bound}",
                encoded.len()
            );
        }
    }

    // #[test]
    // fn test_max_encoded() {
    //     println!("{} {}", max_encoded(128, 3), u32::MAX);
    // }
}
