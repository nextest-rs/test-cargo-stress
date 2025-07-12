# test-cargo-stress

This is a reproduction for [rust-lang/cargo#15744](https://github.com/rust-lang/cargo/issues/15744) (originally [nextest-rs/nextest#2463](https://github.com/nextest-rs/nextest/issues/2463)). To execute, run `cargo run`.

A bisect shows this is okay on nightly-2025-04-28, and repros on nightly-2025-04-29.

```
% cargo +nightly-2025-04-28 -V
cargo 1.88.0-nightly (d811228b1 2025-04-15)

% cargo +nightly-2025-04-29 -V
cargo 1.88.0-nightly (7918c7eb5 2025-04-27)
```

The range of Cargo commits is [here](https://github.com/rust-lang/cargo/compare/d811228b1..7918c7eb5).
