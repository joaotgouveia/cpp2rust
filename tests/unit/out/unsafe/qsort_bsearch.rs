extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn cmp_int_0(mut a: (), mut b: ()) -> () {
    let mut x: () = (*(a as *const ()));
    let mut y: () = (*(b as *const ()));
    return ((((x) > (y)) as ()) - (((x) < (y)) as ()));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut arr: [(); 8] = [5_(), 2_(), 9_(), 1_(), 7_(), 3_(), 8_(), 4_()];
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((((i) < (7_())) as ()) != 0) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut key: () = 7_();
    let mut hit: *mut () = (unimplemented!() as *mut ());
    unimplemented!();
    unimplemented!();
    let mut miss_key: () = 42_();
    let mut miss: *mut () = (unimplemented!() as *mut ());
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
