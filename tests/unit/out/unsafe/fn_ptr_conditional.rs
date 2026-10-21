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
pub unsafe fn identity_2(mut x: ()) -> () {
    return x;
}
pub unsafe fn pick_3(mut mode: ()) -> Option<unsafe fn(()) -> ()> {
    return if ((mode) > (unimplemented!())) {
        Some(inc_0)
    } else {
        if ((mode) < (unimplemented!())) {
            Some(dec_1)
        } else {
            Some(identity_2)
        }
    };
}
pub unsafe fn apply_4(mut fn_: Option<unsafe fn(()) -> ()>, mut x: ()) -> () {
    let mut actual: Option<unsafe fn(()) -> ()> = if !(fn_).is_none() {
        fn_
    } else {
        Some(identity_2)
    };
    return (unsafe { (actual).unwrap()(x) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
