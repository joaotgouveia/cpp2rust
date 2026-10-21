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
pub struct S {
    pub v: (),
}
impl S {
    pub unsafe fn nested_this(&mut self) -> () {
        let mut outer: FnPtr<fn(()) -> ()> = lambda_unsafe!(
            {
                let this_: *mut S = (self as *mut S);
            },
            |mut y: ()| -> () {
                let mut inner: FnPtr<fn(()) -> ()> = lambda_unsafe!(
                    {
                        let this_: *mut S = this_;
                        let y: () = y;
                    },
                    |mut z: ()| -> () {
                        return ((((*this_).v) + (y)) + (z));
                    }
                );
                return (unsafe { inner.call(1_()) });
            }
        );
        return (unsafe { outer.call(20_()) });
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x: () = 10_();
    let mut outer: FnPtr<fn(()) -> ()> = lambda_unsafe!(
        {
            let x: *mut () = &mut x;
        },
        |mut y: ()| -> () {
            let mut inner: FnPtr<fn(()) -> ()> = lambda_unsafe!(
                {
                    let x: *mut () = x;
                    let y: () = y;
                },
                |mut z: ()| -> () {
                    return (((*x) + (y)) + (z));
                }
            );
            return (unsafe { inner.call(1_()) });
        }
    );
    unimplemented!();
    x = 100_();
    unimplemented!();
    let mut s: S = S { v: 5_() };
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
