extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn second_of_0(p: *const std_pair_double__int_) -> () {
    return (*(unsafe { get_1(p) }));
}
pub unsafe fn first_of_2(p: *const std_pair_double__int_) -> () {
    return (*(unsafe { get_3(p) }));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut p: std_pair_double__int_ = {
        let mut __tmp_0: () = 1.5_f64;
        let mut __tmp_1: () = 2_();
        std_pair_double__int_::new({ &mut __tmp_0 }, { &mut __tmp_1 })
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
