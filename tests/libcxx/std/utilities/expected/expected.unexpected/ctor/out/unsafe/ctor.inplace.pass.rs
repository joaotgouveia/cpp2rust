extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
const _: () = assert!(
    true,
    "std::constructible_from<std::unexpected<int>, std::in_place_t, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Foo {}
const _: () = assert!(
    !(false),
    "!std::constructible_from<std::unexpected<Foo>, std::in_place_t, int>"
);
const _: () = assert!(true, "ImplicitlyConstructible<int, int>");
const _: () = assert!(
    !(false),
    "!ImplicitlyConstructible<std::unexpected<int>, std::in_place_t, int>"
);
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct Arg {
    pub i: i32,
}
impl Arg {
    pub unsafe fn new(mut ii: i32) -> Self {
        let mut this = Self { i: ii };
        this
    }
    pub unsafe fn copy_from(other: *const Arg) -> Self {
        let mut this = Self { i: (*other).i };
        this
    }
    pub unsafe fn move_from(other: *mut Arg) -> Self {
        let mut this = Self { i: (*other).i };
        (*other).i = 0;
        this
    }
}
impl Clone for Arg {
    fn clone(&self) -> Self {
        unsafe { Arg::copy_from(self as *const Arg) }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[cpp_from(Vec<Error > , new_1, value)]
pub struct Error {
    pub arg: Arg,
}
impl Error {
    pub unsafe fn new_2(a: *const Arg) -> Self {
        let mut this = Self {
            arg: Arg::copy_from({ a }),
        };
        this
    }
    pub unsafe fn new_3(a: *mut Arg) -> Self {
        let mut this = Self {
            arg: Arg::move_from({ a }),
        };
        this
    }
    pub unsafe fn new_1(mut _a0: Vec<Error>) -> Self {
        let mut this = Self {
            arg: Arg::new({ 0 }),
        };
        assert!(false);
        this
    }
    pub unsafe fn move_from(_a0: *mut Error) -> Self {
        let mut this = Self {
            arg: Arg::move_from({ &mut (*_a0).arg }),
        };
        this
    }
}
pub unsafe fn test_0() -> bool {
    {
        let mut a: Arg = Arg::new({ 5 });
        let mut unex: () = unimplemented!();
        assert!((((*unimplemented!()).arg.i) == (5)));
        assert!(((a.i) == (5)));
    }
    {
        let mut a: Arg = Arg::new({ 5 });
        let mut unex: () = unimplemented!();
        assert!((((*unimplemented!()).arg.i) == (5)));
        assert!(((a.i) == (0)));
    }
    {
        let mut e: Error = {
            let mut __tmp_0: Arg = Arg::new({ 5 });
            Error::new_3({ &mut __tmp_0 })
        };
        let mut unex: () = unimplemented!();
    }
    return true;
}
pub unsafe fn testException_1() {}
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
    (unsafe { testException_1() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
