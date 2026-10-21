extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut a_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut next: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
        unsafe {
            return (*std::cell::LazyCell::force_mut(&mut *&raw mut n_1)).prefix_inc();
        }
    });
    unimplemented!();
    unimplemented!();
    let mut copy: FnPtr<fn() -> ()> = next.copy_from();
    unimplemented!();
    unimplemented!();
    let mut fresh: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
        unsafe {
            return (*std::cell::LazyCell::force_mut(&mut *&raw mut n_1)).prefix_inc();
        }
    });
    unimplemented!();
    unimplemented!();
    let mut per_type: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut x: ()| -> () {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_2)).postfix_inc();
            return (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_2));
        },
        |mut x: ()| -> () {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_3)).postfix_inc();
            return (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_3));
        }
    );
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut read: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut _a0: ()| -> () {
        unsafe {
            return (*std::cell::LazyCell::force_mut(&mut *&raw mut a_0));
        }
    });
    unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut a_0)) = 1_();
    unimplemented!();
    let mut local: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
        unsafe {
            let mut p: P_4 = P_4 { x: 2_(), y: 3_() };
            return ((p.x) * (p.y));
        }
    });
    unimplemented!();
    return unimplemented!();
}
static mut n_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut calls_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut calls_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct P_4 {
    pub x: (),
    pub y: (),
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const a_0);
}
