extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn inc_0(mut x: ()) -> () {
    return ((x) + (1_()));
}
pub unsafe fn dec_1(mut x: ()) -> () {
    return ((x) - (1_()));
}
pub unsafe fn pick_2(mut choose_inc: ()) -> Option<unsafe fn(()) -> ()> {
    if (choose_inc != 0) {
        return Some(inc_0);
    }
    return Some(dec_1);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut f: Option<unsafe fn(()) -> ()> = (unsafe { pick_2(1_()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut g: Option<unsafe fn(()) -> ()> = (unsafe { pick_2(0_()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
