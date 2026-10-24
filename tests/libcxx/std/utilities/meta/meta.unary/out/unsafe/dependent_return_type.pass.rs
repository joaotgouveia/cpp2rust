extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn add_const_0() -> () {
    return unimplemented!();
}
pub unsafe fn add_cv_1() -> () {
    return unimplemented!();
}
pub unsafe fn add_lvalue_reference_2() -> *mut () {
    return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut i_3));
}
static mut i_3: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn add_pointer_4() -> *mut () {
    return std::ptr::null_mut();
}
pub unsafe fn add_rvalue_reference_5() -> *mut () {
    return &mut (*unimplemented!());
}
static mut i_6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn add_volatile_7() -> () {
    return (*unimplemented!());
}
static mut i_8: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn decay_9() -> () {
    return unimplemented!();
}
pub unsafe fn enable_if_10() -> () {
    return unimplemented!();
}
pub unsafe fn make_signed_11() -> () {
    return unimplemented!();
}
pub unsafe fn make_unsigned_12() -> () {
    return unimplemented!();
}
pub unsafe fn remove_all_extents_13() -> () {
    return unimplemented!();
}
pub unsafe fn remove_const_14() -> () {
    return unimplemented!();
}
pub unsafe fn remove_cv_15() -> () {
    return unimplemented!();
}
pub unsafe fn remove_cvref_16() -> () {
    return unimplemented!();
}
pub unsafe fn remove_extent_17() -> () {
    return unimplemented!();
}
pub unsafe fn remove_pointer_18() -> () {
    return unimplemented!();
}
pub unsafe fn remove_reference_19() -> () {
    return unimplemented!();
}
pub unsafe fn remove_volatile_20() -> () {
    return unimplemented!();
}
pub unsafe fn type_identity_21() -> () {
    return unimplemented!();
}
pub unsafe fn underlying_type_22() -> () {
    return unimplemented!();
}
pub type E = ();
pub unsafe fn void_t_23() {}
pub unsafe fn instantiate_24() {
    (unsafe { add_const_0() });
    (unsafe { add_cv_1() });
    (unsafe { add_lvalue_reference_2() });
    (unsafe { add_pointer_4() });
    (unsafe { add_rvalue_reference_5() });
    (unsafe { add_volatile_7() });
    (unsafe { decay_9() });
    (unsafe { enable_if_10() });
    (unsafe { make_signed_11() });
    (unsafe { make_unsigned_12() });
    (unsafe { remove_all_extents_13() });
    (unsafe { remove_const_14() });
    (unsafe { remove_cv_15() });
    (unsafe { remove_cvref_16() });
    (unsafe { remove_extent_17() });
    (unsafe { remove_pointer_18() });
    (unsafe { remove_reference_19() });
    (unsafe { remove_volatile_20() });
    (unsafe { type_identity_21() });
    (unsafe { underlying_type_22() });
    (unsafe { void_t_23() });
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
