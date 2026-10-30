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
pub struct test_macros_detail_is_same_const_bool__const_bool_ {}
pub unsafe fn test_1(_a0: *const ()) {
    const _: () = assert!(true, "std::is_bind_expression<T>::value == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<bool, Expected>, std::is_bind_expression<T> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_bind_expression_v<T> == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_2(_a0: *const ()) {
    const _: () = assert!(true, "std::is_bind_expression<T>::value == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<bool, Expected>, std::is_bind_expression<T> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_bind_expression_v<T> == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_3(_a0: *const ()) {
    const _: () = assert!(true, "std::is_bind_expression<T>::value == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<bool, Expected>, std::is_bind_expression<T> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_bind_expression_v<T> == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_4(_a0: *const ()) {
    const _: () = assert!(true, "std::is_bind_expression<T>::value == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<bool, Expected>, std::is_bind_expression<T> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_bind_expression_v<T> == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_5(_a0: *const ()) {
    const _: () = assert!(true, "std::is_bind_expression<T>::value == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<bool, Expected>, std::is_bind_expression<T> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_bind_expression_v<T> == Expected");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct C {}
impl C {
    pub unsafe fn operator_call(&self, __args: Box<[VaArg]>) -> () {
        return 0_();
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
    (unsafe {
        let mut _arg0: () = unimplemented!();
        test_1(&mut _arg0)
    });
    (unsafe {
        let mut _arg0: () = unimplemented!();
        test_2(&mut _arg0)
    });
    (unsafe {
        let mut _arg0: () = unimplemented!();
        test_3(&mut _arg0)
    });
    (unsafe {
        let mut _arg0: () = 1_();
        test_4(&mut _arg0)
    });
    (unsafe { test_5(&(*unimplemented!())) });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
