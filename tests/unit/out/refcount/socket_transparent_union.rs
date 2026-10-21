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
    let mut fd: () = 0_();
    let ssloc: Value<libc::sockaddr_storage> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
    let slen: Value<libc::socklen_t> = Rc::new(RefCell::new((128usize as libc::socklen_t)));
    unimplemented!();
    let sin: Value<::libc::sockaddr_in> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
    let inlen: Value<libc::socklen_t> = Rc::new(RefCell::new((16usize as libc::socklen_t)));
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
