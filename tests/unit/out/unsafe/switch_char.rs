extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn switch_char_0(mut c: ()) -> () {
    'switch: {
        match { c } {
            __v if __v == ('a' as ()) => {
                return 1_();
            }
            __v if __v == ('b' as ()) => {
                return 2_();
            }
            __v if __v == ('\n' as ()) => {
                return 3_();
            }
            __v if __v == ('\0' as ()) => {
                return 4_();
            }
            _ => {
                return unimplemented!();
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub type Color = ();
pub const Color_kRed: Color = 0;
pub const Color_kGreen: Color = 1;
pub const Color_kBlue: Color = 2;
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
