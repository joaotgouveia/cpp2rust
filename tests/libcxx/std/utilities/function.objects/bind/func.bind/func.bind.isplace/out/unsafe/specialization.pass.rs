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
pub struct My2 {}
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
        let mut f: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut x: i32| -> i32 {
            return (((10) * (x)) + (9));
        });
        let mut place: My2 = <My2>::default();
        let mut bound: () = unimplemented!();
        assert!(
            ((unsafe {
                let mut ___args_0: i32 = 7;
                let mut ___args_1: i32 = 8;
                bound(&mut ___args_0, &mut ___args_1)
            }) == (89))
        );
    }
    {
        let mut f: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut x: i32| -> i32 {
            return (((10) * (x)) + (9));
        });
        let place: My2 = <My2>::default();
        let mut bound: () = unimplemented!();
        assert!(
            ((unsafe {
                let mut ___args_0: i32 = 7;
                let mut ___args_1: i32 = 8;
                bound(&mut ___args_0, &mut ___args_1)
            }) == (89))
        );
    }
    {
        let mut f: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut x: i32| -> i32 {
            return (((10) * (x)) + (9));
        });
        let mut place: My2 = <My2>::default();
        let mut bound: () = unimplemented!();
        assert!(
            ((unsafe {
                let mut ___args_0: i32 = 7;
                let mut ___args_1: i32 = 8;
                bound(&mut ___args_0, &mut ___args_1)
            }) == (89))
        );
    }
    {
        let mut f: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut x: i32| -> i32 {
            return (((10) * (x)) + (9));
        });
        let place: My2 = <My2>::default();
        let mut bound: () = unimplemented!();
        assert!(
            ((unsafe {
                let mut ___args_0: i32 = 7;
                let mut ___args_1: i32 = 8;
                bound(&mut ___args_0, &mut ___args_1)
            }) == (89))
        );
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
