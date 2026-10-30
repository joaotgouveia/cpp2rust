extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn strlen_0(mut s: *const ()) -> () {
    let mut count: () = 0_();
    'loop_: while ((*s.postfix_inc()) != 0) {
        count.prefix_inc();
    }
    return count;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let s: [(); 3] = [('s' as ()), ('t' as ()), ('r' as ())];
    return (unsafe { strlen_0(s.as_ptr()) });
}
pub unsafe fn __cpp2rust_init_globals() {}
