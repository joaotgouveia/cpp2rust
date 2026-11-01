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
            Ptr::<i8>::from_string_literal(b"./ofstream_default_openmode.txt").to_string(),
        )
        .expect("Failed to open file"),
    ));
    let app: Value<::std::fs::File> = Rc::new(RefCell::new(
        ::std::fs::File::create(
            Ptr::<i8>::from_string_literal(b"./ofstream_default_openmode.txt").to_string(),
        )
        .expect("Failed to open file"),
    ));
    assert!((16_i32 != 1_i32));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
