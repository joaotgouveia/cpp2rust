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
pub struct two {
    pub __: [(); 2],
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
    const _: () = assert!(true, "std::round_indeterminate == -1");;
    const _: () = assert!(true, "std::round_toward_zero == 0");;
    const _: () = assert!(true, "std::round_to_nearest == 1");;
    const _: () = assert!(true, "std::round_toward_infinity == 2");;
    const _: () = assert!(true, "std::round_toward_neg_infinity == 3");;
    const _: () = assert!(
        ((::std::mem::size_of::<()>()) == (1_())),
        "sizeof(test(std::round_to_nearest)) == 1"
    );;
    const _: () = assert!(
        ((::std::mem::size_of::<two>()) == (2_())),
        "sizeof(test(1)) == 2"
    );;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
