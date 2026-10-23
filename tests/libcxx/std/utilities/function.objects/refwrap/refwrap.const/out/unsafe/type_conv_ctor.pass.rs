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
pub struct test_macros_detail_is_same_int__int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct convertible_to_int_ref {
    pub val: (),
}
impl convertible_to_int_ref {
    pub unsafe fn to_pmut(&mut self) -> *mut () {
        return &mut self.val;
    }
    pub unsafe fn to_pconst(&self) -> *const () {
        return &self.val;
    }
}
impl Default for convertible_to_int_ref {
    fn default() -> Self {
        unsafe { convertible_to_int_ref { val: 0_() } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct nothrow_convertible_true_ {
    pub val: (),
}
impl Default for nothrow_convertible_true_ {
    fn default() -> Self {
        unsafe { nothrow_convertible_true_ { val: 0_() } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct nothrow_convertible_false_ {
    pub val: (),
}
impl Default for nothrow_convertible_false_ {
    fn default() -> Self {
        unsafe { nothrow_convertible_false_ { val: 0_() } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct convertible_from_int {}
impl convertible_from_int {
    pub unsafe fn new(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn meow_1(mut _a0: ()) {}
pub unsafe fn meow_2(mut _a0: convertible_from_int) {}
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
    {
        let mut t: convertible_to_int_ref = <convertible_to_int_ref>::default();
        let mut r: () = unimplemented!();
        unimplemented!();
    }
    {
        let t: convertible_to_int_ref = <convertible_to_int_ref>::default();
        let mut r: () = unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    (unsafe { meow_2(convertible_from_int::new({ 0_() })) });
    const _: () = assert!(true, "");;
    {
        let mut i: () = 0_();
        let mut ri: () = unimplemented!();
        const _: () = assert!(
            true,
            "(std::is_same<decltype(ri), std::reference_wrapper<int>>::value)"
        );;
    }
    {
        let j: () = 0_();
        let mut rj: () = unimplemented!();
        const _: () = assert!(
            true,
            "(std::is_same<decltype(rj), std::reference_wrapper<const int>>::value)"
        );;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
