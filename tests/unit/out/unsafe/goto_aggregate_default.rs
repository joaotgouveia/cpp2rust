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
pub struct Point {
    pub x: (),
    pub y: (),
}
pub unsafe fn agg_0(mut n: ()) -> () {
    let mut buf40: [(); 40] = [unimplemented!(); 40];
    let mut buf256: [(); 256] = [unimplemented!(); 256];
    let mut arr64: [(); 64] = [unimplemented!(); 64];
    let mut longs: [(); 33] = [unimplemented!(); 33];
    let mut p: Point = <Point>::default();
    let mut ptr: *mut () = std::ptr::null_mut();
    let mut fp: Option<unsafe fn(()) -> ()> = None;
    let mut file: *mut ::libc::FILE = std::ptr::null_mut();
    let mut total: () = unimplemented!();
    goto_block!({
        '__entry: {
            total = 0_();
            if ((((n) < (0_())) as ()) != 0) {
                goto!('out);
            }
            total = 1_();
        }
        'out: {
            return total;
        }
    });
    panic!("ub: non-void function does not return a value")
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
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
