//! A (de)serializer for anything that has implemented `FromStr` / `Display` (as `ToString`) but does not have `Serialize`/`Deserialize`, and is wrapped in an `Option` type, and may be represented as an empty string.
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
//! 	#[serde(with = "serde_str::emp")]
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
//! let with_ip: WithIp = from_str(r#"{"ip": ""}"#)?;
//! assert_eq!(with_ip, WithIp { ip: None });
//! assert_eq!(to_string(&with_ip)?, r#"{"ip":""}"#);
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
//! 	#[serde(with = "serde_str::emp", default)]
//! 	ip: Option<IpAddr>,
//! }
//!
//! # fn main() -> serde_json::Result<()> {
//! let with_ip: WithIp = from_str("{}")?;
//! assert_eq!(with_ip, WithIp { ip: None });
//! assert_eq!(to_string(&with_ip)?, r#"{"ip":""}"#);
//!
//! let with_ip_some: WithIp = from_str(r#"{"ip": "127.0.0.1"}"#)?;
//! assert_eq!(with_ip_some, WithIp { ip: Some([127, 0, 0, 1].into()) });
//! assert_eq!(to_string(&with_ip_some)?, r#"{"ip":"127.0.0.1"}"#);
//! # Ok(())
//! # }
//! ```
//!
//! Excess output can be avoided with `skip_serializing_if`
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
//! 	#[serde(with = "serde_str::emp", skip_serializing_if = "Option::is_none")]
//! 	ip: Option<IpAddr>,
//! }
//!
//! # fn main() -> serde_json::Result<()> {
//! let with_ip_empty: WithIp = from_str(r#"{"ip": ""}"#)?;
//! assert_eq!(with_ip_empty, WithIp { ip: None });
//! assert_eq!(to_string(&with_ip_empty)?, "{}");
//! # Ok(())
//! # }
//! ```
//!
//! Consistently inconsistent input can be normalized by combining `skip_serializing_if` and
//! `default`.
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
//! 	#[serde(default, with = "serde_str::emp", skip_serializing_if = "Option::is_none")]
//! 	ip: Option<IpAddr>,
//! }
//!
//! # fn main() -> serde_json::Result<()> {
//! let with_ip_empty: WithIp = from_str(r#"{"ip": ""}"#)?;
//! assert_eq!(with_ip_empty, WithIp { ip: None });
//! assert_eq!(to_string(&with_ip_empty)?, "{}");
//!
//! let with_ip_empty: WithIp = from_str(r#"{}"#)?;
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
/// Deserialize function, see [mod docs examples](https://docs.rs/serde_str/*/serde_str/emp/index.html) to see how to use it
pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
	T: FromStr,
	T::Err: Display,
	D: Deserializer<'de>,
{
	if let Some(s) = Option::deserialize(deserializer)? {
		if str::is_empty(s) {
			Ok(None)
		} else {
			T::from_str(s).map_err(de::Error::custom).map(|s| Some(s))
		}
	} else {
		Ok(None)
	}
}

/// Serialize function, see [mod docs examples](https://docs.rs/serde_str/*/serde_str/emp/index.html) to see how to use it
pub fn serialize<T, S>(
	value: &Option<T>,
	serializer: S,
) -> Result<S::Ok, S::Error>
where
	T: ToString,
	S: Serializer,
{
	let value = value.as_ref().map(|ty| ty.to_string()).unwrap_or_default();
	String::serialize(&value, serializer)
}
