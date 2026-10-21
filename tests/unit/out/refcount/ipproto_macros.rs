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
    let mut tcp: () = (libc::IPPROTO_TCP as ());
    let mut udp: () = (libc::IPPROTO_UDP as ());
    let mut ip: () = (libc::IPPROTO_IP as ());
    let mut ip6: () = (libc::IPPROTO_IPV6 as ());
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
