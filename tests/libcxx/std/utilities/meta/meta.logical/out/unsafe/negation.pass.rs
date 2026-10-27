extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut value_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { true });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct True {}
pub static mut value_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct False {}
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
    const _: () = assert!(true, "!std::negation<std::true_type >::value");;
    const _: () = assert!(true, "std::negation<std::false_type>::value");;
    const _: () = assert!(true, "!std::negation_v<std::true_type >");;
    const _: () = assert!(true, "std::negation_v<std::false_type>");;
    const _: () = assert!(true, "!std::negation<True >::value");;
    const _: () = assert!(true, "std::negation<False>::value");;
    const _: () = assert!(true, "!std::negation_v<True >");;
    const _: () = assert!(true, "std::negation_v<False>");;
    const _: () = assert!(true, "std::negation<std::negation<std::true_type >>::value");;
    const _: () = assert!(
        true,
        "!std::negation<std::negation<std::false_type>>::value"
    );;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const value_0);
    std::cell::LazyCell::force(&*&raw const value_1);
}
