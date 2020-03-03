Serde Str
=========

[Documentation][docs.rs] |
[Github][git] |
[Crate][crates.io] |
[Libs][lib.rs]

[docs.rs]: https://docs.rs/serde_str
[git]: https://github.com/tailhook/serde-str
[crates.io]: https://crates.io/crates/serde_str
[lib.rs]: https://lib.rs/serde_str
[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[`ToString`]: https://doc.rust-lang.org/std/string/trait.ToString.html
[`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
[serde]: https://serde.rs/

A [serde][] wrapper that simplifies (de)serializaton of data types using [`Display`][]
(as [`ToString::to_string(&self)`][`ToString`]) and [`FromStr`][] as intermediataries.

Examples
-------

```rust
use serde::{Serialize, Deserialize};
use std::net::IpAddr;

#[derive(Serialize, Deserialize)]
struct Struct {
	/// By default IpAddr serializes the same in human-readable formats
	/// like json. This forces the impl even for binary formats.
	///
	/// More imporantly this is useful for types which don't have serde impl.
	#[serde(with = "serde_str")]
	ip: IpAddr,
}
#[derive(Serialize, Deserialize)]
struct Optional {
	/// The above but handling null types
	#[serde(with = "serde_str::opt")]
	ip: Option<IpAddr>,
}
#[derive(Serialize, Deserialize)]
struct Empty {
	/// The above but an empty string is a none-value
	#[serde(with = "serde_str::emp")]
	ip: Option<IpAddr>,
}
#[derive(Serialize, Deserialize)]
struct EmptyOptional {
	/// The above but an empty string, null, or unspecified is a none-value.
	#[serde(with = "serde_str::emp", default)]
	ip: Option<IpAddr>,
}
```

See [docs.rs][] for more examples and usage.

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

