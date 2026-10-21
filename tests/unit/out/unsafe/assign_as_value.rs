extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut buf: [(); 2] = [unimplemented!(); 2];
    let mut p: *mut () = buf.as_mut_ptr();
    let mut q: *mut () = std::ptr::null_mut();
    q = {
        p = (p).wrapping_add((1_() as ()) as usize);
        p
    };
    unimplemented!();
    let mut out: () = unimplemented!();
    'switch: {
        match {
            {
                out = ('x' as ());
                out
            }
        } {
            __v if __v == ('x' as ()) => {
                unimplemented!();
                break 'switch;
            }
            _ => {
                unimplemented!();
                break 'switch;
            }
        }
    };
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
