extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct MyNS_A {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct MyNS_B {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MyNS_C {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MyNS_D {}
pub unsafe fn swap_0(_a0: *mut MyNS_A, _a1: *mut MyNS_A) {}
pub unsafe fn swap_1(_a0: *mut MyNS_A, _a1: *mut MyNS_B) {}
pub unsafe fn swap_2(_a0: *mut MyNS_B, _a1: *mut MyNS_A) {}
pub unsafe fn swap_3(_a0: *mut MyNS_A, _a1: *mut MyNS_C) {}
pub unsafe fn swap_4(_a0: *mut MyNS_D, _a1: *mut MyNS_C) {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct MyNS_M {}
pub unsafe fn swap_5(_a0: *mut MyNS_M, _a1: *mut MyNS_M) {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MyNS_DeletedSwap {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MyNS2_AmbiguousSwap {}
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
    {
        const _: () = assert!(true, "std::is_swappable<A>::value");;
        const _: () = assert!(true, "std::is_swappable<A&>::value");;
        const _: () = assert!(true, "!std::is_swappable<M>::value");;
        const _: () = assert!(true, "!std::is_swappable<M&&>::value");;
    }
    const _: () = assert!(true, "!std::is_swappable<B>::value");;
    const _: () = assert!(true, "std::is_swappable<C>::value");;
    {
        const _: () = assert!(true, "!std::is_swappable<void>::value");;
        const _: () = assert!(true, "!std::is_swappable<int() const>::value");;
        const _: () = assert!(true, "!std::is_swappable<int() &>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_swappable<DeletedSwap>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_swappable<MyNS2::AmbiguousSwap>::value");;
    }
    {
        const _: () = assert!(true, "std::is_swappable_v<int>");;
        const _: () = assert!(true, "!std::is_swappable_v<M>");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
