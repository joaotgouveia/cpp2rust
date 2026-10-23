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
pub struct test_macros_detail_is_same_const_std_filesystem_path_ref__const_std_filesystem_path_ref_
{}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_char_ptr__const_char_ptr_ {}
pub unsafe fn test_constructors_2() {
    {
        let what_arg: () = unimplemented!();
        let what_contains: () = unimplemented!();
        unimplemented!();
        let mut CheckWhat: FnPtr<fn(*const ())> = lambda_unsafe!(
            {
                let what_contains: () = unimplemented!();
            },
            |e: *const ()| {
                let mut s: () = unimplemented!();
                unimplemented!();
            }
        );
        let mut ec: () = unimplemented!();
        let p1: () = unimplemented!();
        let p2: () = unimplemented!();
        {
            const _: () = assert!(true, "");;
            let mut e: () = unimplemented!();
            (unsafe { CheckWhat.call(&e) });
            unimplemented!();
            unimplemented!();
        }
        {
            const _: () = assert!(true, "");;
            let mut e: () = unimplemented!();
            (unsafe { CheckWhat.call(&e) });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            const _: () = assert!(true, "");;
            let mut e: () = unimplemented!();
            (unsafe { CheckWhat.call(&e) });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let what_arg: () = unimplemented!();
        let what_contains: () = unimplemented!();
        unimplemented!();
        let mut CheckWhat: FnPtr<fn(*const ())> = lambda_unsafe!(
            {
                let what_contains: () = unimplemented!();
            },
            |e: *const ()| {
                let mut s: () = unimplemented!();
                unimplemented!();
            }
        );
        let mut ec: () = unimplemented!();
        let p1: () = unimplemented!();
        let p2: () = unimplemented!();
        {
            const _: () = assert!(true, "");;
            let mut e: () = unimplemented!();
            (unsafe { CheckWhat.call(&e) });
            unimplemented!();
            unimplemented!();
        }
        {
            const _: () = assert!(true, "");;
            let mut e: () = unimplemented!();
            (unsafe { CheckWhat.call(&e) });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            const _: () = assert!(true, "");;
            let mut e: () = unimplemented!();
            (unsafe { CheckWhat.call(&e) });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_signatures_3() {
    let p: () = unimplemented!();
    let mut ec: () = unimplemented!();
    let e: () = unimplemented!();
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
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
    const _: () = assert!(
        true,
        "std::is_base_of<std::system_error, fs::filesystem_error>::value"
    );;
    (unsafe { test_constructors_2() });
    (unsafe { test_signatures_3() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
