extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut count_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn f_int_0_1() -> i32 {
    return 3;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A_int_0 {}
impl A_int_0 {
    pub unsafe fn operator_call(&mut self) -> i32 {
        return 4;
    }
}
impl Callable0<i32> for A_int_0 {
    fn call(&self) -> i32 {
        unsafe { A_int_0::operator_call(&mut *(&raw const *self).cast_mut()) }
    }
}
pub unsafe fn test_int_0_2() {
    {
        let mut r1: Option<unsafe fn() -> i32> = Some(f_int_0_1);
        assert!(((unimplemented!()) == (3)));
    }
    {
        let mut fp: Option<unsafe fn() -> i32> = Some(f_int_0_1);
        let mut r1: *mut Option<unsafe fn() -> i32> = &mut fp;
        assert!(((unimplemented!()) == (3)));
    }
    {
        let mut a0: A_int_0 = <A_int_0>::default();
        let mut r1: *mut A_int_0 = &mut a0;
        assert!(((unimplemented!()) == (4)));
    }
}
pub unsafe fn f_void_1_3(mut i: i32) {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)) += i;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A_void_1 {}
impl A_void_1 {
    pub unsafe fn operator_call(&mut self, mut i: i32) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)) += i;
    }
}
impl Callable1<i32, ()> for A_void_1 {
    fn call(&self, a1: i32) -> () {
        unsafe { A_void_1::operator_call(&mut *(&raw const *self).cast_mut(), a1) }
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_int_0_2() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_0);
}
