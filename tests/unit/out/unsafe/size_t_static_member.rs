extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut c_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 5_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {}
pub static mut table_size_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 256_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Table_char_ {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut pa: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut c_0)) as *const ());
    unimplemented!();
    let mut G: *const () =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut table_size_1)) as *const ());
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const c_0);
    std::cell::LazyCell::force(&*&raw const table_size_1);
}
