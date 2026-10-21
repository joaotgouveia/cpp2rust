extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut counter_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn inc_1() {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_0)) += 1_();
}
pub unsafe fn dec_2() {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_0)) -= 1_();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut h: () = 15_();
    let h_ref1: *mut () = &mut h;
    (*h_ref1) = 16_();
    let mut h_ptr: *mut () = (h_ref1);
    let h_ref2: *mut () = &mut (*h_ptr);
    (*h_ref2) = 17_();
    unimplemented!();
    let mut a: () = 1_();
    let mut b: () = 2_();
    let r: *mut () = if ((a) < (b)) {
        (&mut a as *mut ())
    } else {
        (&mut b as *mut ())
    };
    (*r) = 10_();
    unimplemented!();
    let cr: *const () = if ((a) > (b)) {
        (&mut a as *mut ())
    } else {
        (&mut b as *mut ())
    };
    unimplemented!();
    let x: () = 1_();
    let y: () = 2_();
    let cx: *const () = if ((x) < (y)) {
        (&x as *const ())
    } else {
        (&y as *const ())
    };
    unimplemented!();
    let mut cp: *const () = (if ((a) > (b)) {
        (&mut a as *mut ())
    } else {
        (&mut b as *mut ())
    })
    .cast_const();
    unimplemented!();
    let mut mp: *mut () = if ((a) < (b)) {
        (&mut a as *mut ())
    } else {
        (&mut b as *mut ())
    };
    (*mp) = 20_();
    unimplemented!();
    (*if ((a) < (b)) {
        (&mut a as *mut ())
    } else {
        (&mut b as *mut ())
    }) = 30_();
    unimplemented!();
    unimplemented!();
    (*if ((a) < (b)) {
        (&mut a as *mut ())
    } else {
        (&mut b as *mut ())
    }) += 5_();
    unimplemented!();
    let mut ap: *mut () = (if ((a) > (b)) {
        (&mut a as *mut ())
    } else {
        (&mut b as *mut ())
    });
    (*ap) = 40_();
    unimplemented!();
    if ((a) < (b)) {
        (unsafe { inc_1() });
    } else {
        (unsafe { dec_2() });
    };
    unimplemented!();
    if ((a) > (b)) {
        (unsafe { inc_1() });
    } else {
        (unsafe { dec_2() });
    };
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const counter_0);
}
