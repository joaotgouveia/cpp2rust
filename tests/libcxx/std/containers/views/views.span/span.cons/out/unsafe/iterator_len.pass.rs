extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_constructibility_0() {
    const _: () = assert!(
        true,
        "std::is_constructible_v<std::span<int, Extent>, int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, const int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<std::span<const int, Extent>, int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<std::span<const int, Extent>, const int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, volatile int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, const volatile int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<const int, Extent>, volatile int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<const int, Extent>, const volatile int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<volatile int, Extent>, const int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<volatile int, Extent>, const volatile int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, double*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, std::size_t, size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, Other*, std::size_t>"
    );;
}
pub unsafe fn test_constructibility_2() {
    const _: () = assert!(
        true,
        "std::is_constructible_v<std::span<int, Extent>, int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, const int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<std::span<const int, Extent>, int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<std::span<const int, Extent>, const int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, volatile int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, const volatile int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<const int, Extent>, volatile int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<const int, Extent>, const volatile int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<volatile int, Extent>, const int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<volatile int, Extent>, const volatile int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, double*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, std::size_t, size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, Other*, std::size_t>"
    );;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Other_1 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Other_3 {}
pub unsafe fn test_ctor_4() -> () {
    let mut val: [(); 2] = [unimplemented!(), unimplemented!()];
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_ctor_5() -> () {
    let mut val: [A_6; 2] = [<A_6>::default(), <A_6>::default()];
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_7() -> () {
    (unsafe { test_constructibility_0() });
    (unsafe { test_constructibility_2() });
    (unsafe { test_ctor_4() });
    (unsafe { test_ctor_5() });
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A_6 {}
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
    (unsafe { test_7() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
