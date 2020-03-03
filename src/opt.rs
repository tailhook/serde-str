//! A (de)serializer for anything that has implemented `FromStr` / `Display` (as `ToString`) but does not have `Serialize`/`Deserialize`, and is wrapped in an `Option` type.
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate serde_derive;
//! use std::net::IpAddr;
//!
//! /// A structure with an optional IP address.
//! #[derive(Serialize, Deserialize)]
//! # #[derive(PartialEq, Debug)]
//! struct WithIp {
//! 	#[serde(with = "serde_str::opt")]
//! 	ip: Option<IpAddr>,
//! }
//!
//! use serde_json::{
//! 	from_str,
//! 	to_string,
//! };
//! # fn main() -> serde_json::Result<()> {
//! let with_ip: WithIp = from_str(r#"{"ip": "127.0.0.1"}"#)?;
//! assert_eq!(with_ip, WithIp { ip: Some([127, 0, 0, 1].into()) });
//! assert_eq!(to_string(&with_ip)?, r#"{"ip":"127.0.0.1"}"#);
//! let with_ip: WithIp = from_str(r#"{"ip": null}"#)?;
//! assert_eq!(with_ip, WithIp { ip: None });
//! assert_eq!(to_string(&with_ip)?, r#"{"ip":null}"#);
//! # Ok(())
//! # }
//! ```
//!
//! Combined with `#[serde(default)]`, it allows fields to be omitted from input entirely.
//!
//! ```rust
//! # #[macro_use] extern crate serde_derive;
//! # use std::net::IpAddr;
//! # use serde_json::{from_str, to_string};
//! /// A structure with an optional IP address that might not exist in the input.
//! #[derive(Serialize, Deserialize)]
//! # #[derive(PartialEq, Debug)]
//! struct WithIp {
//! 	#[serde(with = "serde_str::opt", default)]
//! 	ip: Option<IpAddr>,
//! }
//!
//! # fn main() -> serde_json::Result<()> {
//! let with_ip: WithIp = from_str("{}")?;
//! assert_eq!(with_ip, WithIp { ip: None });
//! assert_eq!(to_string(&with_ip)?, r#"{"ip":null}"#);
//!
//! let with_ip_some: WithIp = from_str(r#"{"ip": "127.0.0.1"}"#)?;
//! assert_eq!(with_ip_some, WithIp { ip: Some([127, 0, 0, 1].into()) });
//! assert_eq!(to_string(&with_ip_some)?, r#"{"ip":"127.0.0.1"}"#);
//! # Ok(())
//! # }
//! ```
//!
//! Skipping is also easy:
//!
//! ```rust
//! # #[macro_use] extern crate serde_derive;
//! # use serde::{Serialize, Deserialize};
//! # use std::net::IpAddr;
//! # use serde_json::{from_str, to_string};
//! /// A structure with an optional IP address that might not exist in the input, and won't exist
//! /// in the output if it's empty.
//! #[derive(Serialize, Deserialize)]
//! # #[derive(PartialEq, Debug)]
//! struct WithIp {
//! 	#[serde(with = "serde_str::opt", default, skip_serializing_if = "Option::is_none")]
//! 	ip: Option<IpAddr>,
//! }
//!
//! # fn main() -> serde_json::Result<()> {
//! let with_ip_empty: WithIp = from_str("{}")?;
//! assert_eq!(with_ip_empty, WithIp { ip: None });
//! assert_eq!(to_string(&with_ip_empty)?, "{}");
//! # Ok(())
//! # }
//! ```
use serde::{
	de,
	Deserialize,
	Deserializer,
	Serialize,
	Serializer,
};
use std::{
	fmt::Display,
	str::FromStr,
};
/// Deserialize function, see [mod docs examples](https://docs.rs/serde_str/*/serde_str/opt/index.html) to see how to use it
pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
	T: FromStr,
	T::Err: Display,
	D: Deserializer<'de>,
{
	if let Some(s) = Option::deserialize(deserializer)? {
		T::from_str(s).map_err(de::Error::custom).map(|s| Some(s))
	} else {
		Ok(None)
	}
}

/// Serialize function, see [mod docs examples](https://docs.rs/serde_str/*/serde_str/opt/index.html) to see how to use it
pub fn serialize<T, S>(
	value: &Option<T>,
	serializer: S,
) -> Result<S::Ok, S::Error>
where
	T: ToString,
	S: Serializer,
{
	Option::serialize(&value.as_ref().map(|ty| ty.to_string()), serializer)
}
