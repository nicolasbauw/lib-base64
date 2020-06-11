# lib-base64

[![Current Crates.io Version](https://img.shields.io/crates/v/lib-base64.svg)](https://crates.io/crates/lib-base64)
[![Downloads badge](https://img.shields.io/crates/d/lib-base64.svg)](https://crates.io/crates/lib-base64)

Yet another base64 encoding library, which implements the encode() and decode() methods for the String type.
My goals were:
- to write it myself
- to encode / decode with padding
- to use it in an interactive command line string encoding utility that fits my needs.

Example:
```
use lib_base64::Base64;
let s = String::from("Test");
assert_eq!("VGVzdA==", s.encode())
```

License: GPL-3.0
