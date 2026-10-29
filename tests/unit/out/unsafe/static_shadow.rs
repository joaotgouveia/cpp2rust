extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut value_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 5_() });
pub unsafe fn param_shadow_1(mut value: ()) -> () {
    return ((value) + (1_()));
}
pub unsafe fn local_shadow_2() -> () {
    let mut value: () = 99_();
    return value;
}
pub unsafe fn read_global_3() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut value_0));
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
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const value_0);
}
