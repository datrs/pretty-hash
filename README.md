# pretty-hash
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Output binary buffers as a nice, shortened hex string. Adapted from
[pfrazee/pretty-hash](https://github.com/pfrazee/pretty-hash).

- [Documentation][8]
- [Crates.io][2]

## Usage
```rust
extern crate pretty_hash;

let hash = pretty_hash::fmt(b"1234").unwrap();
assert_eq!(hash, "31323334");

let hash = pretty_hash::fmt(b"12345").unwrap();
assert_eq!(hash, "313233..35");
```

## Installation
```sh
$ cargo add pretty-hash
```

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/pretty-hash.svg?style=flat-square
[2]: https://crates.io/crates/pretty-hash
[3]: https://img.shields.io/travis/datrs/pretty-hash.svg?style=flat-square
[4]: https://travis-ci.org/datrs/pretty-hash
[5]: https://img.shields.io/crates/d/pretty-hash.svg?style=flat-square
[6]: https://crates.io/crates/pretty-hash
[7]: https://docs.rs/pretty-hash/badge.svg
[8]: https://docs.rs/pretty-hash
