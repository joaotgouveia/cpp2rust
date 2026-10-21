extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct node {
    pub data: (),
    pub next: *mut node,
}
pub type opt = ();
pub const opt_OPT_STRING_OUT: opt = 0;
pub const opt_OPT_FILE: opt = 1;
pub const opt_OPT_NODE: opt = 2;
pub const opt_OPT_NODE_OUT: opt = 3;
pub unsafe fn dispatch_0(mut option: (), __args: Box<[VaArg]>) -> () {
    let mut ap: *const VaArg = ::std::ptr::null();
    ap = __args.as_ptr();
    let mut result: () = unimplemented!();
    'switch: {
        match { option } {
            __v if __v == (opt_OPT_STRING_OUT as i32) => {
                let mut out: *mut *const () = ap.arg::<*mut *const ()>();
                (*out) = (c"hello".as_ptr().cast_mut()).cast_const();
                result = 1_();
                break 'switch;
            }
            __v if __v == (opt_OPT_FILE as i32) => {
                let mut f: () = ap.arg::<()>();
                result = ((!((f).is_null())) as ());
                break 'switch;
            }
            __v if __v == (opt_OPT_NODE as i32) => {
                let mut n: *mut node = ap.arg::<*mut node>();
                result = (*n).data;
                break 'switch;
            }
            __v if __v == (opt_OPT_NODE_OUT as i32) => {
                let mut out: *mut *mut node = ap.arg::<*mut *mut node>();
                (*out) = std::ptr::null_mut();
                result = 2_();
                break 'switch;
            }
            _ => {}
        }
    };
    return result;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: *const () = std::ptr::null();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut head: node = node {
        data: 42_(),
        next: std::ptr::null_mut(),
    };
    unimplemented!();
    let mut outp: *mut node = (&mut head as *mut node);
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
