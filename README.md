# lib-base64

Yet another base64 encoding library, which implements the encode() and decode() methods for the String type.
My goals were:
- to write it myself
- to use it in an interactive command line string encoding utility that fits my needs.

Example:
```
use lib_base64::Base64;
let s = String::from("Test");
assert_eq!("VGVzdA==", s.encode())
```

License: GPL-3.0
