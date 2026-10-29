extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn apply_0(mut x: (), mut fn_: Option<unsafe fn(()) -> ()>) -> () {
    return (unsafe { (fn_).unwrap()(x) });
}
pub unsafe fn call_nttp_1(mut x: ()) -> () {
    return (unsafe {
        (Some(|mut x: ()| -> () {
            unsafe {
                return ((x) - (1_()));
            }
        }))
        .unwrap()(x)
    });
}
pub unsafe fn call_nttp_2(mut x: ()) -> () {
    return (unsafe {
        (Some(|mut x: ()| -> () {
            unsafe {
                return ((x) * (10_()));
            }
        }))
        .unwrap()(x)
    });
}
pub unsafe fn call_nttp_3(mut x: ()) -> () {
    return (unsafe {
        FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
            unsafe {
                return ((x) + (100_()));
            }
        })
        .call(x)
    });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut fresh: Option<unsafe fn(()) -> ()> = Some(|mut x: ()| -> () {
        unsafe {
            return -x;
        }
    });
    unimplemented!();
    let mut twice: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return ((x) * (2_()));
        }
    });
    let mut named: Option<unsafe fn(()) -> ()> = Some(|mut x: ()| -> () {
        unsafe {
            return ((x) * (2_()));
        }
    });
    unimplemented!();
    unimplemented!();
    named = fresh;
    unimplemented!();
    let mut p: Option<unsafe fn(()) -> ()> = Some(|mut x: ()| -> () {
        unsafe {
            return ((x) * (3_()));
        }
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut sum: FnPtr<fn((), Box<[VaArg]>) -> ()> =
        FnPtr::<fn((), Box<[VaArg]>) -> ()>::new(|mut n: (), __args: Box<[VaArg]>| -> () {
            unsafe {
                let mut ap: *const VaArg = ::std::ptr::null();
                ap = __args.as_ptr();
                let mut s: () = 0_();
                let mut i: () = 0_();
                'loop_: while ((i) < (n)) {
                    s += ap.arg::<()>();
                    i.postfix_inc();
                }
                return s;
            }
        });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
