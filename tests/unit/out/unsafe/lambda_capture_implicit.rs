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
    let mut a: () = 1_();
    let mut b: () = 2_();
    let mut c: () = 3_();
    let mut by_value: FnPtr<fn(()) -> ()> = lambda_unsafe!(
        {
            let a: () = a;
            let b: () = b;
            let c: () = c;
        },
        |mut x: ()| -> () {
            return ((((a) + (b)) + (c)) + (x));
        }
    );
    unimplemented!();
    a = 100_();
    unimplemented!();
    let mut by_ref: FnPtr<fn(()) -> ()> = lambda_unsafe!(
        {
            let a: *mut () = &mut a;
            let b: *mut () = &mut b;
            let c: *mut () = &mut c;
        },
        |mut x: ()| -> () {
            return ((((*a) + (*b)) + (*c)) + (x));
        }
    );
    unimplemented!();
    b = 200_();
    unimplemented!();
    let mut mixed: FnPtr<fn(()) -> ()> = lambda_unsafe!(
        {
            let c: *mut () = &mut c;
            let a: () = a;
            let b: () = b;
        },
        |mut x: ()| -> () {
            (*c) += x;
            return (((a) + (b)) + (*c));
        }
    );
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
