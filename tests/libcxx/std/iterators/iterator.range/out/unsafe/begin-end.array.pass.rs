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
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_reverse_iterator_int_ptr___std_reverse_iterator_int_ptr__
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_reverse_iterator_const_int_ptr___std_reverse_iterator_const_int_ptr__
{}
pub unsafe fn test_4() -> () {
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    let ca: *const [(); 3] = &a;
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    return true;
}
pub unsafe fn test_r_5() -> () {
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    let ca: *const [(); 3] = &a;
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_4() });
    (unsafe { test_r_5() });
    const _: () = assert!(true, "test()");;
    const _: () = assert!(true, "test_r()");;
    {
        let b: *const () = unimplemented!();
        unimplemented!();
        let e: *const () = unimplemented!();
        unimplemented!();
    }
    return 0_();
}
static mut a_6: std::cell::LazyCell<[(); 3]> =
    std::cell::LazyCell::new(|| unsafe { [1_(), 2_(), 3_()] });
pub unsafe fn __cpp2rust_init_globals() {}
