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
fn main_0() -> () {
    let mut n: () = 17_();
    let d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let name: Value<Ptr<()>> = Rc::new(RefCell::new(Ptr::<()>::from_string_literal(
        b"dir/file.txt",
    )));
    let p: Value<()> = Rc::new(RefCell::new(std_filesystem_path::new_1(
        { name.as_pointer() },
        None,
    )));
    let mut s: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
