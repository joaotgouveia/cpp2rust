extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut fd: () = 0_();
    let mut ssloc: sockaddr_storage = unsafe { std::mem::zeroed::<sockaddr_storage>() };
    let mut slen: libc::socklen_t = (::std::mem::size_of::<sockaddr_storage>() as libc::socklen_t);
    unimplemented!();
    let mut sin: sockaddr_in = unsafe { std::mem::zeroed::<sockaddr_in>() };
    let mut inlen: libc::socklen_t = (::std::mem::size_of::<sockaddr_in>() as libc::socklen_t);
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
