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
    pub x: (),
    pub y: (),
}
pub unsafe fn read_0(v: *const ()) -> () {
    return (*v);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut factor: () = 3_();
    let mut scale: FnPtr<fn(()) -> ()> = lambda_unsafe!(
        {
            let factor: () = factor;
        },
        |mut x: ()| -> () {
            return ((x) * (factor));
        }
    );
    unimplemented!();
    factor = 100_();
    unimplemented!();
    let mut slot: () = 7_();
    let mut p: *mut () = (&mut slot as *mut ());
    let mut read_ptr: FnPtr<fn() -> ()> = lambda_unsafe!(
        {
            let p: *mut () = p;
        },
        || -> () {
            return (*p);
        }
    );
    slot = 8_();
    unimplemented!();
    let mut s: S = S { x: 1_(), y: 2_() };
    let mut sum: FnPtr<fn() -> ()> = lambda_unsafe!(
        {
            let s: S = s;
        },
        || -> () {
            return ((s.x) + (s.y));
        }
    );
    s.x = 50_();
    unimplemented!();
    let mut base: () = 10_();
    let mut shifted: FnPtr<fn(()) -> ()> = lambda_unsafe!(
        {
            let y: () = ((base) + (1_()));
        },
        |mut x: ()| -> () {
            return ((x) + (y));
        }
    );
    unimplemented!();
    let k: () = 3_();
    let mut by_copy: FnPtr<fn(()) -> ()> = lambda_unsafe!(
        {
            let k: () = k;
        },
        |mut x: ()| -> () {
            return ((x) + (3_()));
        }
    );
    unimplemented!();
    let mut by_copy_used: FnPtr<fn(()) -> ()> = lambda_unsafe!(
        {
            let k: () = k;
        },
        |mut x: ()| -> () {
            return ((x) + (unsafe { read_0(&k) }));
        }
    );
    unimplemented!();
    let mut implicit_used: FnPtr<fn(()) -> ()> = lambda_unsafe!(
        {
            let k: () = k;
        },
        |mut x: ()| -> () {
            return ((x) + (unsafe { read_0(&k) }));
        }
    );
    unimplemented!();
    let mut by_ref: FnPtr<fn(()) -> ()> = lambda_unsafe!(
        {
            let k: *const () = &k;
        },
        |mut x: ()| -> () {
            return ((x) + (3_()));
        }
    );
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
