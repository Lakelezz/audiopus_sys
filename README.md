[![ci-badge][]][ci] [![docs-badge][]][docs] [![rust 1.33+ badge]][rust 1.48+ link] [![crates.io version]][crates.io link]

# About

`audiopus_sys` is an FFI-Rust-binding to [`Opus`] version 1.3.

Orginally, this sys-crate was made to empower the [`serenity`]-crate to build audio features on Windows, Linux, and Mac. However, it's not limited to that.

Everyone is welcome to contribute,
check out the [`CONTRIBUTING.md`](CONTRIBUTING.md) for further guidance.

# Building

## Requirements
If you want to build Opus, you will need `cmake`.

If you have `pkg-config`, it will attempt to use that before building.

You can also link a pre-installed Opus, see [**Pre-installed Opus**](#Pre-installed-Opus)
below.

This crate provides a pre-built binding. In case you want to generate the
binding yourself, you will need [`Clang`](https://rust-lang.github.io/rust-bindgen/requirements.html#clang),
see [**Pre-installed Opus**](#Generating-The-Binding) below for further
instructions.

## Linking
`audiopus_sys` links to Opus 1.3 and supports Windows, Linux, and MacOS
By default, we statically link to Windows, MacOS, and if you use the
`musl`-environment. We will link dynamically for Linux except when using
mentioned `musl`.

This can be altered by compiling with the `static` or `dynamic` feature having
effects respective to their names. If both features are enabled,
we will pick your system's default.

Environment variables named `LIBOPUS_STATIC` or `OPUS_STATIC` will take
precedence over features thus overriding the behaviour. The value of these
environment variables have no influence of the result: If one of them is set,
statically linking will be picked.

## Pkg-Config
By default, `audiopus_sys` will use `pkg-config` on Unix or GNU.
Setting the environment variable `LIBOPUS_NO_PKG` or `OPUS_NO_PKG` will bypass
probing for Opus via `pkg-config`.

## Pre-installed Opus
If you have Opus pre-installed, you can set `LIBOPUS_LIB_DIR` or
`OPUS_LIB_DIR` to the directory containing Opus.

Be aware that using an Opus other than version 1.3 may not work.

# Generating The Binding
If you want to generate the binding yourself, you can use the
`generate_binding`-feature.

Be aware, `bindgen` requires Clang and its `LIBCLANG_PATH`
environment variable to be specified.

# Installation
Add this to your `Cargo.toml`:

```toml
[dependencies]
audiopus_sys = "0.2"
```
[`serenity`]: https://crates.io/crates/serenity

[`Opus`]: https://www.opus-codec.org/

[ci]: https://dev.azure.com/lakeware/audiopus_sys/_build?definitionId=10
[ci-badge]: https://img.shields.io/azure-devops/build/lakeware/cefad0bd-3570-41d2-b886-f452aedd028c/10/master.svg?style=flat-square

[docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square&colorB=32b6b7
[docs]: https://docs.rs/audiopus_sys

[rust 1.48+ badge]: https://img.shields.io/badge/rust-1.48+-93450a.svg?style=flat-square&colorB=ff9a0d
[rust 1.48+ link]: https://blog.rust-lang.org/2020/11/19/Rust-1.48.html

[crates.io link]: https://crates.io/crates/audiopus_sys
[crates.io version]: https://img.shields.io/crates/v/audiopus_sys.svg?style=flat-square&colorB=b73732
