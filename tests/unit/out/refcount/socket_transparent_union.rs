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
    let ssloc: Value<sockaddr_storage> = <Value<sockaddr_storage>>::default();
    let slen: Value<libc::socklen_t> = Rc::new(RefCell::new((128usize as libc::socklen_t)));
    unimplemented!();
    let sin: Value<sockaddr_in> = <Value<sockaddr_in>>::default();
    let inlen: Value<libc::socklen_t> = Rc::new(RefCell::new((16usize as libc::socklen_t)));
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
