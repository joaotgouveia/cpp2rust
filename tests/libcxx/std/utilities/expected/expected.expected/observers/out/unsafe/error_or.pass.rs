extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct ConstructFromInt {
    pub value: i32,
}
impl ConstructFromInt {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        this
    }
}
pub unsafe fn test_default_template_arg_0() -> bool {
    {
        let e: () = unimplemented!();
        let mut x: ConstructFromInt = unimplemented!();
        assert!(((x.value) == (10)));
    }
    {
        let e: () = unimplemented!();
        let mut x: ConstructFromInt = unimplemented!();
        assert!(((x.value) == (5)));
    }
    {
        let e: () = unimplemented!();
        let mut x: ConstructFromInt = unimplemented!();
        assert!(((x.value) == (10)));
    }
    {
        let e: () = unimplemented!();
        let mut x: ConstructFromInt = unimplemented!();
        assert!(((x.value) == (5)));
    }
    return true;
}
pub unsafe fn test_1() -> bool {
    {
        let e: () = unimplemented!();
        let mut x: i32 = unimplemented!();
        assert!(((x) == (10)));
    }
    {
        let e: () = unimplemented!();
        let mut x: i32 = unimplemented!();
        assert!(((x) == (5)));
    }
    {
        let mut e: () = unimplemented!();
        let mut x: i32 = unimplemented!();
        assert!(((x) == (10)));
    }
    {
        let mut e: () = unimplemented!();
        let mut x: i32 = unimplemented!();
        assert!(((x) == (5)));
    }
    return true;
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
    (unsafe { test_1() });
    const _: () = assert!(true, "test()");;
    (unsafe { test_default_template_arg_0() });
    const _: () = assert!(true, "test_default_template_arg()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
