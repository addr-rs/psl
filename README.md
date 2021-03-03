# A native Rust library for Mozilla's Public Suffix List

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
