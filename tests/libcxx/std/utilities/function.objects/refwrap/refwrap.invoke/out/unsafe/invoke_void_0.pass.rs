extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut count_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn f_void_0_1() {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A_void_0 {}
impl A_void_0 {
    pub unsafe fn operator_call(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
    }
}
pub unsafe fn test_void_0_2() {
    let mut save_count: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0));
    {
        let mut r1: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        save_count = (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0));
    }
    {
        let mut fp: Option<unsafe fn()> = Some(f_void_0_1);
        let mut r1: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        save_count = (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0));
    }
    {
        let mut a0: A_void_0 = <A_void_0>::default();
        let mut r1: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        save_count = (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0));
    }
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
    (unsafe { test_void_0_2() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_0);
}
