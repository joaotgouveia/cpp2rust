extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn deref_0(mut p: *mut ()) -> () {
    return (*p);
}
pub unsafe fn strlen_1(mut s: *mut ()) -> () {
    let mut c: () = 0_();
    'loop_: while ((*s.postfix_inc()) != 0) {
        c.prefix_inc();
    }
    return c;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: [(); 2] = [1_(), 2_()];
    let mut s: [(); 4] = [('a' as ()), ('b' as ()), ('c' as ()), ('\0' as ())];
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
