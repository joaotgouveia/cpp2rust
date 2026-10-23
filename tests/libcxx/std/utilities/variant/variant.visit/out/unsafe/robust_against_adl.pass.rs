extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(mut do_it: ()) -> () {
    if do_it {
        let mut v: () = Default::default();
        (unsafe {
            let mut ___visitor: FnPtr<Generic> = lambda_unsafe!(
                Generic,
                {},
                |mut _a0: *mut Holder_Incomplete_| {},
                |mut _a0: ()| {}
            );
            visit_1(&mut ___visitor, &mut v)
        });
        (unsafe {
            let mut ___visitor: FnPtr<Generic> = lambda_unsafe!(
                Generic,
                {},
                |mut _a0: *mut Holder_Incomplete_| -> *mut Holder_Incomplete_ {
                    return std::ptr::null_mut();
                },
                |mut _a0: ()| -> *mut Holder_Incomplete_ {
                    return std::ptr::null_mut();
                }
            );
            visit_2(&mut ___visitor, &mut v)
        });
        (unsafe {
            let mut ___visitor: FnPtr<Generic> = lambda_unsafe!(
                Generic,
                {},
                |mut _a0: *mut Holder_Incomplete_| {},
                |mut _a0: ()| {}
            );
            visit_3(&mut ___visitor, &mut v)
        });
        (unsafe {
            let mut ___visitor: FnPtr<Generic> = lambda_unsafe!(
                Generic,
                {},
                |mut _a0: *mut Holder_Incomplete_| -> *mut Holder_Incomplete_ {
                    return std::ptr::null_mut();
                },
                |mut _a0: ()| -> *mut Holder_Incomplete_ {
                    return std::ptr::null_mut();
                }
            );
            visit_4(&mut ___visitor, &mut v)
        });
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
    (unsafe { test_0(true) });
    const _: () = assert!(true, "test(true)");;
    return 0_();
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct Holder_Incomplete_;
pub unsafe fn __cpp2rust_init_globals() {}
