#![feature(proc_macro)]

extern crate byteorder;
extern crate strukt;

use strukt::pack;

#[test]
fn test_pack() {
    assert_eq!(pack!("<bI")(b' ', 0x41424344), b" DCBA");
    assert_eq!(pack!(">bI")(b' ', 0x41424344), b" ABCD");
    assert_eq!(pack!("Q")(0x41424344), b"DCBA\0\0\0\0");
}
