extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn strlen_0(mut s: *const (), mut n: ()) -> () {
    return if ((*s) != 0) {
        (unsafe { strlen_0(s.offset((1_()) as isize), ((n) + (1_()))) })
    } else {
        n
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let s: [(); 4] = [('s' as ()), ('t' as ()), ('r' as ()), ('\0' as ())];
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
