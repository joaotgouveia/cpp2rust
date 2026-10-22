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
    let mut buf: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
    (unsafe {
        let ___dest: () = (buf.as_mut_ptr().offset((2_()) as isize) as ());
        let ___src: () = (buf.as_mut_ptr() as ());
        memmove_0(___dest, ___src, 4_())
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
