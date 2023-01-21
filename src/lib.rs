//! # lexicoid
//!
//! Short & stable IDs based on timestamps.
//!
//! Heavily inspired by [Short, friendly base32 slugs from timestamps](https://brandur.org/fragments/base32-slugs) by [@brandur](https://github.com/brandur).
//!
//!
//! ## Quickstart
//!
//! ```rust
//! # use lexicoid::*;
//! #
//! // generates a lexicoid for the current timestamp
//! println!("{}", lexicoid_now()); // gj7x3vc
//!
//! // generates a lexicoid for a given unix timestamp (as u64)
//! println!("{}", lexicoid(1654401676)); // gei4p52
//! ```

#[doc = include_str!("../README.md")]
#[macro_use]
extern crate lazy_static;

use data_encoding::Encoding;
use num::bigint::BigInt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::time::{SystemTime, UNIX_EPOCH};

lazy_static! {
    static ref B32: Encoding = {
        let mut spec = data_encoding::Specification::new();
        spec.symbols.push_str("234567abcdefghijklmnopqrstuvwxyz");
        spec.encoding().unwrap()
    };
}

/// A lexicographically sortable identifier generated from a unix timestamp.
///
/// It's a wrapper around a `String` that implements `Ord`, `PartialOrd`, and `Display`.
///
/// The sorting is based on the following rules:
/// - The shorter the ID, the earlier it is in the sort order.
/// - If the IDs have the same length, the lexicographical order is used.
///
/// It's also `Deref` to `String` so you can use it as a `String` if you need to.
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Id(String);

impl Deref for Id {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Ord for Id {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.len() == other.len() {
            return self.0.cmp(&(other.0));
        }
        self.len().cmp(&(other.len()))
    }
}

impl PartialOrd for Id {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Generates a lexicoid for a given unix timestamp (as u64).
pub fn lexicoid(timestamp: u64) -> Id {
    let ts_bi = BigInt::from(timestamp);
    Id(B32.encode(ts_bi.to_bytes_be().1.as_slice()))
}

/// Generates a lexicoid for the current timestamp (using the system time).
pub fn lexicoid_now() -> Id {
    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
    lexicoid(timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cases = [
            (0, "22"),               // Thu Jan 01 1970 00:00:00 GMT+0000
            (100, "gk"),             // Thu Jan 01 1970 00:01:40 GMT+0000
            (10000, "6wc2"),         // Thu Jan 01 1970 02:46:40 GMT+0000
            (500000, "2ykm2"),       // Tue Jan 06 1970 18:53:20 GMT+0000
            (1700000, "5bse2"),      // Tue Jan 20 1970 16:13:20 GMT+0000
            (28000000, "2apny22"),   // Sat Nov 21 1970 01:46:40 GMT+0000
            (550000000, "6567f22"),  // Sat Jun 06 1987 17:46:40 GMT+0000
            (1550000000, "flllz22"), // Tue Feb 12 2019 19:33:20 GMT+0000
            (1654301676, "gehebv2"), // Sat Jun 04 2022 00:14:36 GMT+0000
            (1654401676, "gei4p52"), // Sun Jun 05 2022 04:01:16 GMT+0000
            (1674301676, "gj7x3v2"), // Sat Jan 21 2023 11:47:56 GMT+0000
            (1674301677, "gj7x3vc"), // Sat Jan 21 2023 11:47:57 GMT+0000
        ];

        let mut ids: Vec<Id> = cases.iter().map(|(ts, _)| lexicoid(*ts)).collect();
        ids.sort(); // sorts the results to prove that the lexico order is correct
        for (i, (ts, expected)) in cases.iter().enumerate() {
            assert_eq!(ids[i].as_str(), *expected);
            assert_eq!(ids[i].as_str(), lexicoid(*ts).as_str());
        }

        // a timestamp generated now must be greater than the latest timestamp in the test cases
        let id_now = lexicoid_now();
        assert!(matches!(
            id_now.cmp(&(ids[ids.len() - 1])),
            std::cmp::Ordering::Greater
        ));
    }
}
