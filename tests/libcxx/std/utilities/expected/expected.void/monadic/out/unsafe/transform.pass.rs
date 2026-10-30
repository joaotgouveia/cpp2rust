extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
const _: () = assert!(
    !(false),
    "!has_transform<const std::expected<int, std::unique_ptr<int>>&, int()>"
);
const _: () = assert!(
    !(false),
    "!has_transform<const std::expected<int, std::unique_ptr<int>>&&, int()>"
);
pub unsafe fn test_val_types_0() {
    {
        let mut l: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
            unsafe {
                return 1_();
            }
        });
        let mut v: () = unimplemented!();
        let mut val: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
            unsafe {
                return 1_();
            }
        });
        let v: () = unimplemented!();
        let mut val: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
            unsafe {
                return 1_();
            }
        });
        let mut v: () = unimplemented!();
        let mut val: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
            unsafe {
                return 1_();
            }
        });
        let v: () = unimplemented!();
        let mut val: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_fail_1() {
    {
        let mut l: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
            unsafe {
                unimplemented!();
                return 0_();
            }
        });
        let mut v: () = unimplemented!();
        let mut val: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
            unsafe {
                unimplemented!();
                return 0_();
            }
        });
        let v: () = unimplemented!();
        let mut val: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
            unsafe {
                unimplemented!();
                return 0_();
            }
        });
        let mut v: () = unimplemented!();
        let mut val: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
            unsafe {
                unimplemented!();
                return 0_();
            }
        });
        let v: () = unimplemented!();
        let mut val: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_2() -> () {
    (unsafe { test_fail_1() });
    (unsafe { test_val_types_0() });
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
    (unsafe { test_2() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
