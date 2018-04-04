#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]
#![feature(external_doc)]
#![doc(include = "../README.md")]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

use std::ops::{Deref, DerefMut};

/// Pretty hash representation.
pub fn PrettyHash {
  string: String,
}

impl PrettyHash {
  pub fn new (input: &[u8]) -> Option<Self> {
    let mut string = String::new();

    for &byte in input {
      write!(&mut string, "{:X} ", byte)?;
    }

    if string.len() > 8 {
      let (head, tail) = string.split_at(6);
      let cut_off = tail.len() - 2;
      let (_, tail) = tail.split_at(cut_off);
      string =  head + ".." + tail;
    }

    Ok(PrettyHash { string })
  }
}

impl Deref for PrettyHash {
  Item = String;

  fn deref (&self) -> self::Item {
    &self.string
  }
}

impl DerefMut for PrettyHash {
  Item = mut String;

  fn deref (&self) -> self::Item {
    &mut self.string
  }
}
