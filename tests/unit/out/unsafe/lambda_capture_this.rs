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
    pub step: (),
}
impl S {
    pub unsafe fn add(&mut self, mut k: ()) {
        self.n += k;
    }
    pub unsafe fn scaled(&self) -> () {
        return ((self.n) * (self.step));
    }
    pub unsafe fn bump(&mut self, mut by: ()) {
        let mut inc: FnPtr<fn(())> = lambda_unsafe!(
            {
                let this_: *mut S = (self as *mut S);
            },
            |mut k: ()| {
                (*this_).n += k;
            }
        );
        (unsafe { inc.call(by) });
        (unsafe { inc.call(by) });
    }
    pub unsafe fn bump_via_method(&mut self, mut by: ()) {
        let mut inc: FnPtr<fn(())> = lambda_unsafe!(
            {
                let this_: *mut S = (self as *mut S);
            },
            |mut k: ()| {
                (unsafe { S::add(&mut (*this_), k) });
            }
        );
        (unsafe { inc.call(by) });
    }
    pub unsafe fn read_scaled(&self) -> () {
        let mut get: FnPtr<fn() -> ()> = lambda_unsafe!(
            {
                let this_: *const S = (self as *const S);
            },
            || -> () {
                return (unsafe { S::scaled(&(*this_)) });
            }
        );
        return (unsafe { get.call() });
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: S = S {
        n: unimplemented!(),
        step: 2_(),
    };
    (unsafe { S::bump(&mut s, 3_()) });
    unimplemented!();
    (unsafe { S::bump_via_method(&mut s, 4_()) });
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
