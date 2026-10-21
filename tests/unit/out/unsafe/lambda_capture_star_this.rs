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
    pub n: (),
}
impl S {
    pub unsafe fn twice(&self) -> () {
        return ((self.n) * (2_()));
    }
    pub unsafe fn modify_copy(&mut self) -> () {
        let mut f: FnPtr<fn() -> ()> = lambda_unsafe!(
            {
                let this_: S = (*(self as *mut S));
            },
            || -> () {
                (*(&raw mut this_)).n += 10_();
                return (*(&raw mut this_)).n;
            }
        );
        let mut r: () = (unsafe { f.call() });
        return (((r) * (100_())) + (self.n));
    }
    pub unsafe fn snapshot(&mut self) -> () {
        let mut f: FnPtr<fn() -> ()> = lambda_unsafe!(
            {
                let this_: S = (*(self as *mut S));
            },
            || -> () {
                return (unsafe { S::twice(&(*(&raw mut this_))) });
            }
        );
        self.n = 99_();
        return (unsafe { f.call() });
    }
    pub unsafe fn mixed(&mut self, mut k: ()) -> () {
        let mut f: FnPtr<fn() -> ()> = lambda_unsafe!(
            {
                let this_: S = (*(self as *mut S));
                let k: () = k;
            },
            || -> () {
                return (((*(&raw mut this_)).n) + (k));
            }
        );
        self.n = unimplemented!();
        return (unsafe { f.call() });
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: S = S { n: 1_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
