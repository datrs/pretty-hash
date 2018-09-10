#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]
#![feature(external_doc)]
#![doc(include = "../README.md")]

extern crate failure;

use failure::Error;
use std::fmt::Write;

/// Prettify a byte slice.
pub fn fmt(input: &[u8]) -> Result<String, Error> {
  let mut string = String::new();

  for &byte in input {
    write!(&mut string, "{:x}", byte)?;
  }

  if string.len() > 8 {
    let (head, tail) = string.split_at(6);
    let cut_off = tail.len() - 2;
    let (_, tail) = tail.split_at(cut_off);
    Ok(format!("{}..{}", head, tail))
  } else {
    Ok(string)
  }
}
