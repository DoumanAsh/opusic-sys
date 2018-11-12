# opusic-sys

[![Crates.io](https://img.shields.io/crates/v/opusic-sys.svg)](https://crates.io/crates/opusic-sys)
[![Documentation](https://docs.rs/opusic-sys/badge.svg)](https://docs.rs/crate/opusic-sys/)

Bindings to [libopus](https://github.com/xiph/opus)

## Re-generate bindings

The feature `build-bindgen` is used to generate bindings.

To use it set env variable `LIBCLANG_PATH` to directory that contains clang binaries

## Requirements

### Unix and windows-gnu

Being able to build `libopus` which means you need make and gcc/clang

### MSVC toolchain

It uses statically pre-built binaries instead of building it on fly
As otherwise it is too bothersome.
