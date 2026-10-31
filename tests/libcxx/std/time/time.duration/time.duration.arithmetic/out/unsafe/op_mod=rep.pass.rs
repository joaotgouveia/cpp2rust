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
pub struct NotARep {}
pub unsafe fn operator_rem_assign_0(mut d: (), mut _a1: NotARep) -> () {
    return unimplemented!();
}
pub unsafe fn test_constexpr_1() -> bool {
    let mut s: () = unimplemented!();
    unimplemented!();
    return ((unimplemented!()) == (2_()));
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
    {
        let mut us: () = unimplemented!();
        unimplemented!();
        assert!(((unimplemented!()) == (2_())));
    }
    const _: () = assert!(true, "test_constexpr()");;
    {
        let mut d: () = unimplemented!();
        let mut n: NotARep = <NotARep>::default();
        (unsafe {
            let _d: () = unimplemented!();
            operator_rem_assign_0(_d, n)
        });
        assert!(((unimplemented!()) == (5_())));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
