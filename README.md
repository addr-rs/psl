# A native Rust library for Mozilla's Public Suffix List

[![Build Status](https://travis-ci.org/addr-rs/psl.svg?branch=master)](https://travis-ci.org/addr-rs/psl) [![Latest Version](https://img.shields.io/crates/v/psl.svg)](https://crates.io/crates/psl) [![Docs](https://docs.rs/psl/badge.svg)](https://docs.rs/psl)

This library uses Mozilla's [Public Suffix List](https://publicsuffix.org) to reliably determine the suffix of a domain name. It compiles the list down to native Rust code for ultimate speed and correctness. This list compilation is done as a separate step by the maintainer so compile times for the crate itself are still very fast. This crate has no dependencies, not even on the `std` library, so it can even be used in embedded systems.

## Setting Up

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
psl = "1"
```

## Examples

```rust
let suffix = psl::suffix(b"www.example.com")?;
assert_eq!(suffix.as_bytes(), b"com");
assert_eq!(suffix.typ(), Some(Type::Icann));

let domain = psl::domain(b"www.example.com")?;
assert_eq!(domain.as_bytes(), b"example.com");
assert_eq!(domain.suffix().as_bytes(), b"com");

let domain = psl::domain("www.食狮.中国".as_bytes())?;
assert_eq!(domain.as_bytes(), "食狮.中国".as_bytes());
assert_eq!(domain.suffix().as_bytes(), "中国".as_bytes());

let domain = psl::domain(b"www.xn--85x722f.xn--55qx5d.cn")?;
assert_eq!(domain.as_bytes(), b"xn--85x722f.xn--55qx5d.cn");
assert_eq!(domain.suffix().as_bytes(), b"xn--55qx5d.cn");

let domain = psl::domain(b"a.b.example.uk.com")?;
assert_eq!(domain.as_bytes(), b"example.uk.com");
assert_eq!(domain.suffix().as_bytes(), b"uk.com");

let domain = psl::domain(b"_tcp.example.com.")?;
assert_eq!(domain.as_bytes(), b"example.com.");
assert_eq!(domain.suffix().as_bytes(), b"com.");
```
