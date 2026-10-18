# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Purpose

A personal project for learning, fun and pleasure: working through streaming entropy coding (the renormalization used in asymmetric numeral systems, ANS) in Rust, one small exercise at a time. Understanding the method matters more than finishing it, so explain the reasoning behind changes, and do not fill in an exercise's `todo!()` unless asked.

## Commands

Toolchain comes from `mise.toml` (latest stable Rust, edition 2024). No dependencies.

```bash
cargo test                                          # all tests
cargo test streaming::step3                         # one step's tests (substring filter)
cargo test step3_state_interval::tests::empty       # one test
cargo test encode_head -- --nocapture               # tests that print (encode_head prints a table)
cargo clippy --all-targets
```

Several files (`lib.rs`, `streaming/mod.rs`, steps 2–4) are not rustfmt-clean, and `lib.rs` has clippy warnings. `cargo fmt` reformats the whole crate; to keep a diff to the files you changed, run `rustfmt src/streaming/<file>.rs` instead.

## Structure

- `src/lib.rs` — the original coder, outside the exercise sequence. `Coder` takes any alphabet (`Vec<char>`, mapped through a `HashMap`) and encodes a whole text into one `usize` (`encode`, `encode_w_formula`, `decode`). `encode_stream`/`decode_stream` are a byte-renormalizing version for any alphabet size, with `L = k << 32` and the final state appended as 8 little-endian bytes (the same format as step 4).
- `src/streaming/` — the exercise sequence. `mod.rs` holds the overview of all steps, the shared alphabet (`K = 26`, `index`, `letter`; texts are lowercase a–z only, and both functions assert this), and `test_util` (`Rng`, `random_text`, `size_bound`). `#![allow(unused_variables, dead_code)]` there lets unimplemented `todo!()` stubs compile.

Each step removes one limitation of the previous one:

| step | change | output |
|------|--------|--------|
| 1 | start state 1, digits 0..25 | one `u64` (≤ 13 letters) |
| 2 | write out low bytes while `x >= LIMIT`; record per-letter byte counts | state + bytes + counts |
| 3 | keep state in `[L, 256·L)` so the decoder needs no counts | state + bytes |
| 4 | append final state (8 LE bytes); stop when no bytes remain and `x == L` | `Vec<u8>` |
| 5 | encode in reverse, state first, bytes reversed; `Decoder` yields letters in text order | `Vec<u8>` |
| 6 | 32-bit words, `L = 26 << 27`; at most one word per letter, so `if` replaces `while` | `Vec<u32>` |

Later steps import earlier ones' constants and functions (step 5 uses `step3_state_interval::L` and step 3's encoder).

## Exercise file format

A new step follows the existing files:

- Module doc comment: `Goal:`, an explanation of the problem it solves, pseudocode in a ```` ```text ```` block, what to implement and the `cargo test streaming::stepN` command, then `Hints:` (the last hint may give the answer).
- Public functions with `todo!()` bodies; tests are written in full up front.
- Tests use `test_util::Rng` with a fixed seed (no `rand` dependency, reproducible), check the round trip, and check the output size against `size_bound(n, bits)` = ⌈n·log2(26)/bits⌉ plus the fixed header (8 bytes or 2 words).

## Conventions

- Comments justify bounds as a chain of inequalities, one step per line (see the threshold derivation in `step3_state_interval.rs` and the choice of `L` in `step6_word_size.rs`).
- One commit per step, pushed directly to `main`: subject `Implement streaming step N: <step title>`, body stating in plain terms what encode and decode now do.
