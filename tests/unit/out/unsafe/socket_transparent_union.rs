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
unsafe fn main_0() -> i32 {
    let mut fd: i32 = 0;
    let mut ssloc: sockaddr_storage = unsafe { std::mem::zeroed::<sockaddr_storage>() };
    let mut slen: libc::socklen_t = (::std::mem::size_of::<sockaddr_storage>() as libc::socklen_t);
    assert!(
        ((((libc::getsockname(
            fd,
            ((&mut ssloc as *mut sockaddr_storage) as *mut sockaddr),
            (&mut slen as *mut libc::socklen_t)
        )) == (-1_i32)) as i32)
            != 0)
    );
    let mut sin: sockaddr_in = unsafe { std::mem::zeroed::<sockaddr_in>() };
    let mut inlen: libc::socklen_t = (::std::mem::size_of::<sockaddr_in>() as libc::socklen_t);
    assert!(
        ((((libc::getsockname(
            fd,
            ((&mut sin as *mut sockaddr_in) as *mut sockaddr),
            (&mut inlen as *mut libc::socklen_t)
        )) == (-1_i32)) as i32)
            != 0)
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
