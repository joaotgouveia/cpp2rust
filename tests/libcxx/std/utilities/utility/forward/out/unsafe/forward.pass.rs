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
pub struct A {}
pub unsafe fn source_0() -> A {
    return <A>::default();
}
pub unsafe fn csource_1() -> A {
    return <A>::default();
}
pub unsafe fn test_constexpr_forward_2() -> () {
    let mut x: () = 42_();
    let cx: () = 101_();
    return ((((((((x) == (42_())) && ((x) == (42_()))) && ((x) == (42_())))
        && ((x) == (42_())))
        && ((x) == (42_())))
        && ((x) == (42_())))
        && ((cx) == (101_())))
        && ((cx) == (101_()));
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
    let mut a: A = <A>::default();
    let ca: A = <A>::default();
    (&(a));
    (&(ca));
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::forward<A&>(a)), A&>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::forward<A>(a)), A&&>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::forward<A>(source())), A&&>::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::forward<const A&>(a)), const A&>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::forward<const A>(a)), const A&&>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::forward<const A>(source())), const A&&>::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::forward<const A&>(ca)), const A&>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::forward<const A>(ca)), const A&&>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::forward<const A>(csource())), const A&&>::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let i2: () = 42_();
        const _: () = assert!(true, "std::forward<int>(42) == 42");;
        const _: () = assert!(true, "std::forward<const int&>(i2) == 42");;
        const _: () = assert!(true, "test_constexpr_forward()");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
