extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut p_mut_0: std::cell::LazyCell<*mut ()> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub static mut p_const_1: std::cell::LazyCell<*const ()> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null() });
pub static mut cp_2: std::cell::LazyCell<*const ()> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null() });
pub static mut arr_of_ptr_3: std::cell::LazyCell<[*mut (); 4]> =
    std::cell::LazyCell::new(|| unsafe { [std::ptr::null_mut(); 4] });
pub static mut pp_4: std::cell::LazyCell<*mut *mut ()> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub static mut const_arr_of_ptr_5: std::cell::LazyCell<[*const (); 3]> =
    std::cell::LazyCell::new(|| unsafe { [std::ptr::null(); 3] });
pub static mut cp_explicit_null_6: std::cell::LazyCell<*const ()> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null() });
pub static mut p_zero_7: std::cell::LazyCell<*mut ()> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while ((i) < (4_())) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while ((i) < (3_())) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const p_mut_0);
    std::cell::LazyCell::force(&*&raw const p_const_1);
    std::cell::LazyCell::force(&*&raw const cp_2);
    std::cell::LazyCell::force(&*&raw const arr_of_ptr_3);
    std::cell::LazyCell::force(&*&raw const pp_4);
    std::cell::LazyCell::force(&*&raw const const_arr_of_ptr_5);
    std::cell::LazyCell::force(&*&raw const cp_explicit_null_6);
    std::cell::LazyCell::force(&*&raw const p_zero_7);
}
