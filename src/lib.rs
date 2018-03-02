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
//! #[macro_use]
//! extern crate serde_derive;
//!
//! extern crate serde;
//! extern crate serde_str;
//!
//! use std::net::IpAddr;
//!
//!
//! #[derive(Serialize, Deserialize)]
//! struct Timestamps {
//!     #[serde(with = "serde_str")]
//!     pattern: IpAddr,
//! }
//!
//! #
//! # fn main() {}
//! ```
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
extern crate serde;

#[cfg(test)] extern crate serde_json;
#[cfg(test)] #[macro_use] extern crate serde_derive;

use std::fmt;
use std::str::FromStr;
use std::marker::PhantomData;

use serde::de::{Visitor, Error};
use serde::{Deserializer, Serializer};


struct StrVisitor<T>(PhantomData<*const T>);


impl<'a, T: FromStr> Visitor<'a> for StrVisitor<T>
    where <T as FromStr>::Err: fmt::Display
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("valid regular expression")
    }
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where E: Error
    {
        value.parse().map_err(E::custom)
    }
}

/// Deserialize function, see crate docs to see how to use it
pub fn deserialize<'de, D, T: FromStr>(deserializer: D) -> Result<T, D::Error>
    where D: Deserializer<'de>,
          <T as FromStr>::Err: fmt::Display
{
    deserializer.deserialize_str(StrVisitor(PhantomData))
}

/// Serialize function, see crate docs to see how to use it
pub fn serialize<S, T: ToString>(value: &T, serializer: S)
    -> Result<S::Ok, S::Error>
    where S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}


#[cfg(test)]
mod test {
    use serde_json::{from_str, to_string};
    use std::net::IpAddr;

    #[derive(Deserialize, Serialize)]
    struct WithIp {
        ip: IpAddr,
    }

    #[test]
    fn roundtrip() {
        let with_ip: WithIp = from_str(r#"{"ip": "127.0.0.1"}"#).unwrap();
        assert_eq!(to_string(&with_ip).unwrap(), r#"{"ip":"127.0.0.1"}"#);
    }

}
