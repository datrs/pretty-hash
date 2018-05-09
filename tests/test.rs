extern crate pretty_hash;

#[test]
fn formats_stuff() {
  let hash = pretty_hash::fmt(b"1234").unwrap();
  assert_eq!(hash, "31323334");

  let hash = pretty_hash::fmt(b"12345").unwrap();
  assert_eq!(hash, "313233..35");
}
