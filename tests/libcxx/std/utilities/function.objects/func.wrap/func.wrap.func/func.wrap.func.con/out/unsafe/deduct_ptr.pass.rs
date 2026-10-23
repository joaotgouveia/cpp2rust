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
pub struct test_macros_detail_is_same_std_function_R______std_function_R_____ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_function_R__A1____std_function_R__A1___ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_function_R__A1__A2____std_function_R__A1__A2___ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_function_R__A1__A2__A3____std_function_R__A1__A2__A3___ {
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct R {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A1 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A2 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A3 {}
pub unsafe fn f0_4() -> R {
    return <R>::default();
}
pub unsafe fn f1_5(mut _a0: A1) -> R {
    return <R>::default();
}
pub unsafe fn f2_6(mut _a0: A1, mut _a1: A2) -> R {
    return <R>::default();
}
pub unsafe fn f3_7(mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
    return <R>::default();
}
pub unsafe fn f4_8(mut _a0: Option<A1>) -> R {
    let mut _a0: A1 = _a0.unwrap_or_else(|| unsafe { <A1>::default() });
    return <R>::default();
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
        let mut a: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut b: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut c: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut d: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut a: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut b: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut c: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut d: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut a: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut b: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut c: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut d: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut a: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut b: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut c: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut d: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut a: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut b: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut c: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut d: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
