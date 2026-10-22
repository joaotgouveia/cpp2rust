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
    let mut path: *const () = (c"cpp2rust_fd_io_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fd: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    fd = unimplemented!();
    unimplemented!();
    let mut buf: [(); 16] = [unimplemented!(); 16];
    (unsafe {
        let ___s: () = (buf.as_mut_ptr() as ());
        let ___n: () = ::std::mem::size_of::<[(); 16]>();
        memset_0(___s, 0_(), ___n)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
