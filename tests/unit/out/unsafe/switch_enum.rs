extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Color = ();
pub const Color_kRed: Color = 0;
pub const Color_kGreen: Color = 1;
pub const Color_kBlue: Color = 2;
pub unsafe fn switch_enum_0(mut c: Color) -> () {
    'switch: {
        match { (c as ()) } {
            __v if __v == (Color_kRed as ()) => {
                return 10_();
            }
            __v if __v == (Color_kGreen as ()) => {
                return 20_();
            }
            __v if __v == (Color_kBlue as ()) => {
                return 30_();
            }
            _ => {}
        }
    };
    return -1_();
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
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
