extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_reference_wrapper_std_reference_wrapper_int____std_reference_wrapper_std_reference_wrapper_int___
{}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_reference_wrapper_const_std_reference_wrapper_int____std_reference_wrapper_const_std_reference_wrapper_int___
{}
pub unsafe fn test_2() -> bool {
    {
        let mut i: i32 = 0;
        let mut ri: *mut i32 = &mut i;
        let mut rri: *mut *mut i32 = &mut ri;
        let mut rrj: *mut *mut i32 = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(((rrj) == (&mut ri as *mut *mut i32)));
    }
    {
        let mut i: i32 = 0;
        let mut ri: *mut i32 = &mut i;
        let mut rri: *const *mut i32 = &mut ri;
        let mut rrj: *const *mut i32 = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(((rrj) == ((&mut ri as *mut *mut i32).cast_const())));
    }
    {
        let mut i: i32 = 0;
        let mut ri: *mut i32 = &mut i;
        let mut rri: *mut *mut i32 = &mut ri;
        let mut rrj: *const *mut i32 = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(((rrj) == ((&mut ri as *mut *mut i32).cast_const())));
    }
    {
        let mut i: i32 = 0;
        let mut ri: *mut i32 = &mut i;
        let mut rri: *const *mut i32 = &mut ri;
        let mut rrj: *const *mut i32 = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(((rrj) == ((&mut ri as *mut *mut i32).cast_const())));
    }
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_2() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
