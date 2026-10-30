extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {}
impl S {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_trivial_int__true_ {}
impl test_trivial_int__true_ {
    pub unsafe fn operator_call(&self) {
        let it: () = unimplemented!();
        &(it);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_trivial_char__true_ {}
impl test_trivial_char__true_ {
    pub unsafe fn operator_call(&self) {
        let it: () = unimplemented!();
        &(it);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_trivial_double__true_ {}
impl test_trivial_double__true_ {
    pub unsafe fn operator_call(&self) {
        let it: () = unimplemented!();
        &(it);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_trivial_S__false_ {}
impl test_trivial_S__false_ {
    pub unsafe fn operator_call(&self) {}
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_trivial_std_basic_string_char__std_char_traits_char___std_allocator_char____false_
{}
impl test_trivial_std_basic_string_char__std_char_traits_char___std_allocator_char____false_ {
    pub unsafe fn operator_call(&self) {}
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
    {
        let mut it: () = unimplemented!();
        unimplemented!();
        let it2: () = unimplemented!();
        &(it2);
    }
    (unsafe { test_trivial_int__true_::operator_call(&<test_trivial_int__true_>::default()) });
    (unsafe { test_trivial_char__true_::operator_call(&<test_trivial_char__true_>::default()) });
    (unsafe {
        test_trivial_double__true_::operator_call(&<test_trivial_double__true_>::default())
    });
    (unsafe { test_trivial_S__false_::operator_call(&<test_trivial_S__false_>::default()) });
    (unsafe {
        test_trivial_std_basic_string_char__std_char_traits_char___std_allocator_char____false_ :: operator_call ( & <test_trivial_std_basic_string_char__std_char_traits_char___std_allocator_char____false_ >::default()  , )
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
