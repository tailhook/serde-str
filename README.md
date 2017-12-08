Serde Str
=========

[Documentation](https://docs.rs/serde_str) |
[Github](https://github.com/tailhook/serde-str) |
[Crate](https://crates.io/crates/serde_str)

A serde wrapper, that can be used to serialize data types using Display
(or `.to_string()`) and FromStr.

Example
-------

```rust
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_str;

use std::net::IpAddr;

#[derive(Serialize, Deserialize)]
struct Struct {
    /// By default IpAddr is serialized the same in human-readable formats
    /// like json. This forces the impl even for binary formats.
    ///
    /// More inporantly this is useful for types which don't have serde impl.
    #[serde(with = "serde_str")]
    ip: IpAddr,
}
```


License
=======

Licensed under either of

* Apache License, Version 2.0,
  (./LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (./LICENSE-MIT or http://opensource.org/licenses/MIT)
  at your option.

Contribution
------------

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

