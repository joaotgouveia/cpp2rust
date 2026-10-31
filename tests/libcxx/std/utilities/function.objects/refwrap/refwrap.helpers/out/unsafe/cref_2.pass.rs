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
pub struct adl_A {}
pub unsafe fn cref_0(mut _a0: adl_A) {}
pub unsafe fn test_1() -> bool {
    {
        let i: i32 = 0;
        let mut r1: *const i32 = unimplemented!();
        let mut r2: *const i32 = unimplemented!();
        assert!(((r2) == (&i as *const i32)));
    }
    {
        let mut a: adl_A = <adl_A>::default();
        let mut a1: *const adl_A = unimplemented!();
        let mut a2: *const adl_A = unimplemented!();
        assert!(((a2) == ((&mut a as *mut adl_A).cast_const())));
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
    (unsafe { test_1() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
