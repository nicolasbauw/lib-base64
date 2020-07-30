# lib-base64

[![Current Crates.io Version](https://img.shields.io/crates/v/lib-base64.svg)](https://crates.io/crates/lib-base64)
[![Downloads badge](https://img.shields.io/crates/d/lib-base64.svg)](https://crates.io/crates/lib-base64)

A base64 encoding and decoding library, which implements the encode() and decode() methods for the String type.

Example:
```
use lib_base64::Base64;
let s = String::from("Test");
assert_eq!("VGVzdA==", s.encode())
```

License: GPL-3.0
