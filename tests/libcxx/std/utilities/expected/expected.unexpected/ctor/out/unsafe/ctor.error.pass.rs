extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
const _: () = assert!(true, "std::constructible_from<std::unexpected<int>, int>");
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct CstrFromUnexpected {}
impl CstrFromUnexpected {}
const _: () = assert!(
    !(false),
    "!std::constructible_from<std::unexpected<CstrFromUnexpected>, std::unexpected<CstrFromUnexpected>>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CstrFromInplace {}
impl CstrFromInplace {}
const _: () = assert!(
    !(false),
    "!std::constructible_from<std::unexpected<CstrFromInplace>, std::in_place_t>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Foo {}
const _: () = assert!(
    !(false),
    "!std::constructible_from<std::unexpected<Foo>, int>"
);
const _: () = assert!(true, "std::convertible_to<int, int>");
const _: () = assert!(!(false), "!std::convertible_to<int, std::unexpected<int>>");
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[cpp_from(i32, new_1, value)]
#[cpp_from(Vec<Error > , new_2, value)]
pub struct Error {
    pub i: i32,
}
impl Error {
    pub unsafe fn new_1(mut ii: i32) -> Self {
        let mut this = Self { i: ii };
        this
    }
    pub unsafe fn copy_from(other: *const Error) -> Self {
        let mut this = Self { i: (*other).i };
        this
    }
    pub unsafe fn move_from(other: *mut Error) -> Self {
        let mut this = Self { i: (*other).i };
        (*other).i = 0;
        this
    }
    pub unsafe fn new_2(mut _a0: Vec<Error>) -> Self {
        let mut this = Self { i: 0_i32 };
        assert!(false);
        this
    }
}
impl Clone for Error {
    fn clone(&self) -> Self {
        unsafe { Error::copy_from(self as *const Error) }
    }
}
pub unsafe fn test_0() -> bool {
    {
        let mut e: Error = Error::new_1({ 5 });
        let mut unex: () = unimplemented!();
        assert!((((*unimplemented!()).i) == (5)));
        assert!(((e.i) == (5)));
    }
    {
        let mut e: Error = Error::new_1({ 5 });
        let mut unex: () = unimplemented!();
        assert!((((*unimplemented!()).i) == (5)));
        assert!(((e.i) == (0)));
    }
    {
        let mut e: Error = Error::new_1({ 5 });
        let mut unex: () = unimplemented!();
    }
    {
        let mut ue: () = unimplemented!();
        assert!((((*unimplemented!()).i) == (5)));
        assert!((((*unimplemented!()).j) == (6)));
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Bar_1 {
    pub i: i32,
    pub j: i32,
}
impl Bar_1 {
    pub unsafe fn new(mut ii: i32, mut jj: i32) -> Self {
        let mut this = Self { i: ii, j: jj };
        this
    }
}
pub unsafe fn testException_2() {}
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
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    (unsafe { testException_2() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
