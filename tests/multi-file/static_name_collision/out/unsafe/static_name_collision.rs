extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut same_name_different_type_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1_() });
pub static mut same_name_same_type_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5_() });
pub unsafe fn a_foo_2() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut same_name_different_type_0));
}
pub unsafe fn a_bar_3() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut same_name_same_type_1));
}
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
    unimplemented!();
    return unimplemented!();
}
pub static mut same_name_different_type_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1_f32 });
pub static mut same_name_same_type_5: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 6_() });
pub unsafe fn b_foo_6() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut same_name_different_type_4));
}
pub unsafe fn b_bar_7() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut same_name_same_type_5));
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const same_name_different_type_0);
    std::cell::LazyCell::force(&*&raw const same_name_same_type_1);
    std::cell::LazyCell::force(&*&raw const same_name_different_type_4);
    std::cell::LazyCell::force(&*&raw const same_name_same_type_5);
}
