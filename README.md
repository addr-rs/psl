# PSL

A native Rust library for Mozilla's Public Suffix List

[![CI](https://github.com/addr-rs/psl/actions/workflows/ci.yml/badge.svg)](https://github.com/addr-rs/psl/actions/workflows/ci.yml)
[![Publish](https://github.com/addr-rs/psl/actions/workflows/update.yaml/badge.svg)](https://github.com/addr-rs/psl/actions/workflows/update.yaml)
[![Latest Version](https://img.shields.io/crates/v/psl.svg)](https://crates.io/crates/psl)
[![Crates.io downloads](https://img.shields.io/crates/d/psl)](https://crates.io/crates/psl)
[![Docs](https://docs.rs/psl/badge.svg)](https://docs.rs/psl)
[![Minimum supported Rust version](https://img.shields.io/badge/rustc-1.41+-yellow.svg)](https://www.rust-lang.org)
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

This library uses Mozilla's [Public Suffix List](https://publicsuffix.org) to reliably determine the suffix of a domain name.

It compiles the list down to native Rust code for ultimate speed. This list compilation is done as a separate step by the `Publish` GitHub Action so the crate still compiles very quickly. The `Publish` action automatically checks for updates everyday and pushes an updated crate to crates.io if there were any updates in the upstream domain suffixes. This keeps the crate automatically synchronised with the official list.

If you need a dynamic list that can be updated at runtime, though a bit slower, please use the [publicsuffix](https://crates.io/crates/publicsuffix) crate instead (which also has optional support for looking up domain names in any case).

## Setting Up

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
psl = "2"
```

## Examples

```rust
use psl::{Psl, List};

let suffix = List.suffix(b"www.example.com")?;
assert_eq!(suffix.as_bytes(), b"com");
assert_eq!(suffix.typ(), Some(Type::Icann));

let domain = List.domain(b"www.example.com")?;
assert_eq!(domain.as_bytes(), b"example.com");
assert_eq!(domain.suffix().as_bytes(), b"com");

let domain = List.domain("www.食狮.中国".as_bytes())?;
assert_eq!(domain.as_bytes(), "食狮.中国".as_bytes());
assert_eq!(domain.suffix().as_bytes(), "中国".as_bytes());

let domain = List.domain(b"www.xn--85x722f.xn--55qx5d.cn")?;
assert_eq!(domain.as_bytes(), b"xn--85x722f.xn--55qx5d.cn");
assert_eq!(domain.suffix().as_bytes(), b"xn--55qx5d.cn");

let domain = List.domain(b"a.b.example.uk.com")?;
assert_eq!(domain.as_bytes(), b"example.uk.com");
assert_eq!(domain.suffix().as_bytes(), b"uk.com");

let domain = List.domain(b"_tcp.example.com.")?;
assert_eq!(domain.as_bytes(), b"example.com.");
assert_eq!(domain.suffix().as_bytes(), b"com.");
```
