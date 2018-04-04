extern crate pretty_hash;

use pretty_hash::PrettyHash;

#[test]
fn formats_stuff() {
  let hash = PrettyHash::fmt(b"1234").unwrap();
  assert_eq!(hash, "31323334");

  let hash = PrettyHash::fmt(b"12345").unwrap();
  assert_eq!(hash, "313233..35");
}
