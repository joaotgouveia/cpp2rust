extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn nested_0(mut a: (), mut b: ()) -> () {
    let mut r: () = unimplemented!();
    'switch: {
        match { a } {
            __v if __v == 1_() => {
                'switch: {
                    match { b } {
                        __v if __v == 10_() => {
                            r = 11_();
                            break 'switch;
                        }
                        __v if __v == 20_() => {
                            r = 12_();
                            break 'switch;
                        }
                        _ => {
                            r = 13_();
                            break 'switch;
                        }
                    }
                };
                r += 1_();
                break 'switch;
            }
            __v if __v == 2_() => {
                r = 2_();
                break 'switch;
            }
            _ => {
                r = -1_();
                break 'switch;
            }
        }
    };
    return r;
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
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
