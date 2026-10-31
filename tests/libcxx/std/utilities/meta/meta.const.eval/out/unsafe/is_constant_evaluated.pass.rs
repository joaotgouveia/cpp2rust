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
pub struct test_macros_detail_is_same_bool__bool_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_InTemplate_true___InTemplate_true__ {}
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
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let p: bool = unimplemented!();
        assert!(p);
    }
    {
        const _: () = assert!(true, "std::is_constant_evaluated()");;
        let mut p: bool = unimplemented!();
        assert!(!(p));
        const _: () = assert!(true, "");;
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut local_static_2)) == (42)));
    }
    return 0;
}
static mut local_static_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { if unimplemented!() { 42 } else { -1_i32 } });
pub unsafe fn __cpp2rust_init_globals() {}
