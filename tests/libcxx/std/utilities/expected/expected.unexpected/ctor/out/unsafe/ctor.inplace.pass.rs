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
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct Arg {
    pub i: (),
}
impl Arg {
    pub unsafe fn new(mut ii: ()) -> Self {
        let mut this = Self { i: ii };
        this
    }
    pub unsafe fn copy_from(other: *const Arg) -> Self {
        let mut this = Self { i: (*other).i };
        this
    }
    pub unsafe fn move_from(other: *mut Arg) -> Self {
        let mut this = Self { i: (*other).i };
        (*other).i = 0_();
        this
    }
}
impl Clone for Arg {
    fn clone(&self) -> Self {
        unsafe { Arg::copy_from(self as *const Arg) }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct Error {
    pub arg: Arg,
}
impl Error {
    pub unsafe fn new_1(a: *const Arg) -> Self {
        let mut this = Self {
            arg: Arg::copy_from({ a }),
        };
        this
    }
    pub unsafe fn new_2(a: *mut Arg) -> Self {
        let mut this = Self {
            arg: Arg::move_from({ unimplemented!() }),
        };
        this
    }
    pub unsafe fn new_3(mut _a0: ()) -> Self {
        let mut this = Self {
            arg: Arg::new({ 0_() }),
        };
        unimplemented!();
        this
    }
}
pub unsafe fn test_0() -> () {
    {
        let mut a: Arg = Arg::new({ 5_() });
        let mut unex: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a: Arg = Arg::new({ 5_() });
        let mut unex: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut e: Error = {
            let mut __tmp_0: Arg = Arg::new({ 5_() });
            Error::new_2({ &mut __tmp_0 })
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    (unsafe { testException_1() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
