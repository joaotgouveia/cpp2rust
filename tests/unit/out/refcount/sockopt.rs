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
    let mut s: () = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0_());
    unimplemented!();
    let on: Value<()> = Rc::new(RefCell::new(1_()));
    unimplemented!();
    unimplemented!();
    let err: Value<()> = Rc::new(RefCell::new(-1_()));
    let len: Value<libc::socklen_t> = Rc::new(RefCell::new(
        (::std::mem::size_of::<()>() as libc::socklen_t),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
