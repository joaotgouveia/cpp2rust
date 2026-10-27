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
pub struct test_macros_detail_is_same_std_valarray_int___std_valarray_int__ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_valarray_long___std_valarray_long__ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_valarray_long_ptr___std_valarray_long_ptr__ {}
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
        let mut v: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
        let mut v: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
        let mut v: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        const _: () = assert!(true, "std::is_same_v<decltype(v2), std::valarray<long>>");;
    }
    {
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        const _: () = assert!(true, "std::is_same_v<decltype(v2), std::valarray<long>>");;
    }
    {
        let mut v: () = unimplemented!();
        let mut m: () = unimplemented!();
        let mut v2: () = unimplemented!();
        const _: () = assert!(true, "std::is_same_v<decltype(v2), std::valarray<long>>");;
    }
    {
        let mut v: () = unimplemented!();
        let mut i: () = unimplemented!();
        let mut v2: () = unimplemented!();
        const _: () = assert!(true, "std::is_same_v<decltype(v2), std::valarray<long>>");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
