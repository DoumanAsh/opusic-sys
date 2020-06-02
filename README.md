# opusic-sys

![Rust](https://github.com/DoumanAsh/opusic-sys/workflows/Rust/badge.svg?branch=master)
[![Crates.io](https://img.shields.io/crates/v/opusic-sys.svg)](https://crates.io/crates/opusic-sys)
[![Documentation](https://docs.rs/opusic-sys/badge.svg)](https://docs.rs/crate/opusic-sys/)

Bindings to [libopus](https://github.com/xiph/opus)

Target version [1.3.1](https://github.com/xiph/opus/releases/tag/v1.3.1)

## Re-generate bindings

The feature `build-bindgen` is used to generate bindings.

To use it set env variable `LIBCLANG_PATH` to directory that contains clang binaries

## Requirements

- `cmake`
