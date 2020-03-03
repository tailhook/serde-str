//! # Serde Str
//!
//! [Documentation](https://docs.rs/serde_str) |
//! [Github](https://github.com/tailhook/serde-str) |
//! [Crate](https://crates.io/crates/serde_str)
//!
//! A (de)serializer for anything that has implemented `FromStr` / `Display`
//! but does not have `Serialize`/`Deserialize`.
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate serde_derive;
//! use std::net::IpAddr;
//!
//! /// A demonstration structure that holds a lonesome IP address.
//! #[derive(Serialize, Deserialize)]
//! # #[derive(PartialEq, Debug)]
//! struct WithIp {
//! 	#[serde(with = "serde_str")]
//! 	ip: IpAddr,
//! }
//!
//! use serde_json::{
//! 	from_str,
//! 	to_string,
//! };
//! # fn main() -> serde_json::Result<()> {
//! let with_ip: WithIp = from_str(r#"{"ip": "127.0.0.1"}"#)?;
//! assert_eq!(WithIp { ip: [127, 0, 0, 1].into() }, with_ip);
//! assert_eq!(to_string(&with_ip)?, r#"{"ip":"127.0.0.1"}"#);
//!
//! let with_ip: WithIp = from_str(r#"{"ip": "::"}"#)?;
//! assert_eq!(WithIp { ip: [0u16; 8].into() }, with_ip);
//! assert_eq!(to_string(&with_ip)?, r#"{"ip":"::"}"#);
//! # Ok(())
//! # }
//! ```
#![forbid(missing_docs, missing_debug_implementations, unsafe_code, dead_code)]
#![deny(unused)]
use serde::{
	de::{
		Deserialize,
		Error as DeserializeError,
	},
	ser::Serialize,
	Deserializer,
	Serializer,
};
use std::{
	fmt,
	str::FromStr,
};

/// Deserialize function, see crate docs to see how to use it
pub fn deserialize<'de, D, T: FromStr>(deserializer: D) -> Result<T, D::Error>
where
	D: Deserializer<'de>,
	<T as FromStr>::Err: fmt::Display,
{
	let s = String::deserialize(deserializer)?;
	T::from_str(&s).map_err(DeserializeError::custom)
}

/// Serialize function, see crate docs to see how to use it
pub fn serialize<S, T: ToString>(
	value: &T,
	serializer: S,
) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let s = value.to_string();
	String::serialize(&s, serializer)
}

pub mod emp;
pub mod opt;
