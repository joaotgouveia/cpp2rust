extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn dowhile_0(mut x: ()) -> () {
    let mut __do_while = true;
    'loop_: while __do_while || ((x) <= (200_())) {
        __do_while = false;
        x += 1_();
        let mut __do_while = true;
        'loop_: while __do_while || ((x) <= (100_())) {
            __do_while = false;
            x += 1_();
            x += 1_();
        }
        x += 1_();
    }
    return x;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
