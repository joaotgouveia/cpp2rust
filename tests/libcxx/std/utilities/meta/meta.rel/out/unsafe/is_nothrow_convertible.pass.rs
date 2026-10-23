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
pub struct A {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct B {
    pub a: A,
}
impl B {
    pub unsafe fn to_A(&mut self) -> A {
        return self.a;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct C {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct D {
    pub c: C,
}
impl D {
    pub unsafe fn to_C(&mut self) -> C {
        return self.c;
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
    const _: () = assert!(true, "(std::is_nothrow_convertible<int, double>::value)");;
    const _: () = assert!(true, "!(std::is_nothrow_convertible<int, char*>::value)");;
    const _: () = assert!(true, "!(std::is_nothrow_convertible<A, B>::value)");;
    const _: () = assert!(true, "(std::is_nothrow_convertible<D, C>::value)");;
    const _: () = assert!(true, "(std::is_nothrow_convertible_v<int, double>)");;
    const _: () = assert!(true, "!(std::is_nothrow_convertible_v<int, char*>)");;
    const _: () = assert!(true, "!(std::is_nothrow_convertible_v<A, B>)");;
    const _: () = assert!(true, "(std::is_nothrow_convertible_v<D, C>)");;
    const _: () = assert!(true, "(std::is_nothrow_convertible_v<const void, void>)");;
    const _: () = assert!(true, "(std::is_nothrow_convertible_v<volatile void, void>)");;
    const _: () = assert!(true, "(std::is_nothrow_convertible_v<void, const void>)");;
    const _: () = assert!(true, "(std::is_nothrow_convertible_v<void, volatile void>)");;
    const _: () = assert!(true, "!(std::is_nothrow_convertible_v<int[], double[]>)");;
    const _: () = assert!(true, "!(std::is_nothrow_convertible_v<int[], int[]>)");;
    const _: () = assert!(true, "!(std::is_nothrow_convertible_v<int[10], int[10]>)");;
    const _: () = assert!(
        true,
        "!(std::is_nothrow_convertible_v<int[10], double[10]>)"
    );;
    const _: () = assert!(true, "!(std::is_nothrow_convertible_v<int[5], double[10]>)");;
    const _: () = assert!(true, "!(std::is_nothrow_convertible_v<int[10], A[10]>)");;
    const _: () = assert!(true, "!(std::is_nothrow_convertible_v<V, V>)");;
    const _: () = assert!(true, "!(std::is_nothrow_convertible_v<V, I>)");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
