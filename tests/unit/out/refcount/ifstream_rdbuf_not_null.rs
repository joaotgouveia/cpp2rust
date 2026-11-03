extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let ofs: Value<::std::fs::File> = Rc::new(RefCell::new(
        ::std::fs::File::create(
            Ptr::<i8>::from_string_literal(b"./ifstream_rdbuf_not_null.txt").to_string(),
        )
        .expect("Failed to open file"),
    ));
    let mut ob: Ptr<::std::fs::File> = ofs.as_pointer();
    assert!(!((ob).is_null()));
    let ifs: Value<::std::fs::File> = Rc::new(RefCell::new(
        ::std::fs::File::open(
            Ptr::<i8>::from_string_literal(b"./ifstream_rdbuf_not_null.txt").to_string(),
        )
        .expect("Failed to open file"),
    ));
    let mut buf: Ptr<::std::fs::File> = ifs.as_pointer();
    assert!(!((buf).is_null()));
    let mut fb: Ptr<::std::fs::File> = ifs.as_pointer();
    assert!(!((fb).is_null()));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
