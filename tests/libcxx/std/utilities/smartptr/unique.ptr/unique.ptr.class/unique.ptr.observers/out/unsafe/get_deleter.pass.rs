extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_Deleter_ref__const_Deleter_ref_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_Deleter_ref__Deleter_ref_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Deleter {}
impl Deleter {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn operator_call(&self, mut _a0: ()) {}
    pub unsafe fn test_3(&mut self) -> () {
        return 5_();
    }
    pub unsafe fn test_4(&self) -> () {
        return 6_();
    }
}
impl Default for Deleter {
    fn default() -> Self {
        unsafe { Deleter::new() }
    }
}
pub unsafe fn test_basic_2() {
    {
        let mut p: () = unimplemented!();
        unimplemented!();
    }
    {
        let p: () = unimplemented!();
        unimplemented!();
    }
    {
        let d: Deleter = Deleter::new();
        let mut p: () = unimplemented!();
        let cp: *const () = &p;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut d: Deleter = Deleter::new();
        let mut p: () = unimplemented!();
        let cp: *const () = &p;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_basic_3() {
    {
        let mut p: () = unimplemented!();
        unimplemented!();
    }
    {
        let p: () = unimplemented!();
        unimplemented!();
    }
    {
        let d: Deleter = Deleter::new();
        let mut p: () = unimplemented!();
        let cp: *const () = &p;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut d: Deleter = Deleter::new();
        let mut p: () = unimplemented!();
        let cp: *const () = &p;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_4() -> () {
    (unsafe { test_basic_2() });
    (unsafe { test_basic_3() });
    return true;
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
    (unsafe { test_4() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
