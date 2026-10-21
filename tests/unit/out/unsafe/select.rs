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
    let mut fds: [(); 2] = [unimplemented!(); 2];
    unimplemented!();
    let mut rset: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut tv: ::libc::timeval = unsafe { std::mem::zeroed() };
    (unsafe {
        let ___s: () = ((&mut tv as *mut ::libc::timeval) as ());
        let ___n: () = ::std::mem::size_of::<::libc::timeval>();
        memset_0(___s, 0_(), ___n)
    });
    tv.tv_sec = 0_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    tv.tv_sec = 1_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
