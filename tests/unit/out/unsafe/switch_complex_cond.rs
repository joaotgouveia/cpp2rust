extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn switch_complex_cond_0(mut p: *mut (), mut bias: ()) -> () {
    'switch: {
        match { ((*p) + (bias)) } {
            __v if __v == unimplemented!() => {
                return 1_();
            }
            __v if __v == 5_() => {
                return 2_();
            }
            __v if __v == 10_() => {
                return 3_();
            }
            _ => {
                return unimplemented!();
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut p_val: () = 5_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
