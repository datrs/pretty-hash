#![forbid(unsafe_code, missing_debug_implementations, missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! ## Example
//! ```rust
//! extern crate pretty_hash;
//!
//! let hash = pretty_hash::fmt(b"1234").unwrap();
//! assert_eq!(hash, "31323334");
//!
//! let hash = pretty_hash::fmt(b"12345").unwrap();
//! assert_eq!(hash, "313233..35");
//! ```

extern crate failure;

use failure::Error;
// use std::fmt::Write;

/// Prettify a byte slice.
pub fn fmt(input: &[u8]) -> Result<String, Error> {
  let string = input
    .iter()
    .map(|byte| format!("{:02x}", byte))
    .collect::<String>();

  if string.len() > 8 {
    let (head, tail) = string.split_at(6);
    let cut_off = tail.len() - 2;
    let (_, tail) = tail.split_at(cut_off);
    Ok(format!("{}..{}", head, tail))
  } else {
    Ok(string)
  }
}
