extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_Deleter_ref__const_Deleter_ref_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_Deleter_ref__Deleter_ref_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Deleter {}
impl Deleter {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn operator_call(&self, mut _a0: *mut ::libc::c_void) {}
    pub unsafe fn test_3(&mut self) -> i32 {
        return 5;
    }
    pub unsafe fn test_4(&self) -> i32 {
        return 6;
    }
}
impl Callable1<*mut ::libc::c_void, ()> for Deleter {
    fn call(&self, a1: *mut ::libc::c_void) -> () {
        unsafe { Deleter::operator_call(self, a1) }
    }
}
impl Default for Deleter {
    fn default() -> Self {
        unsafe { Deleter::new() }
    }
}
pub unsafe fn test_basic_2() {
    {
        let mut p: Option<Box<i32>> = None;
        assert!(((unsafe { Deleter::test_3(&mut (*unimplemented!()),) }) == (5)));
    }
    {
        let p: Option<Box<i32>> = None;
        assert!(((unsafe { Deleter::test_4(&(*unimplemented!()),) }) == (6)));
    }
    {
        let d: Deleter = Deleter::new();
        let mut p: Option<Box<i32>> = unimplemented!();
        let cp: *const Option<Box<i32>> = &p;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(((unsafe { Deleter::test_4(&(*unimplemented!()),) }) == (6)));
        assert!(((unsafe { Deleter::test_4(&(*unimplemented!()),) }) == (6)));
    }
    {
        let mut d: Deleter = Deleter::new();
        let mut p: Option<Box<i32>> = unimplemented!();
        let cp: *const Option<Box<i32>> = &p;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(((unsafe { Deleter::test_3(&mut (*unimplemented!()),) }) == (5)));
        assert!(((unsafe { Deleter::test_3(&mut (*unimplemented!()),) }) == (5)));
    }
}
pub unsafe fn test_basic_3() {
    {
        let mut p: Option<Box<i32>> = None;
        assert!(((unsafe { Deleter::test_3(&mut (*unimplemented!()),) }) == (5)));
    }
    {
        let p: Option<Box<[i32]>> = None;
        assert!(((unsafe { Deleter::test_4(&(*unimplemented!()),) }) == (6)));
    }
    {
        let d: Deleter = Deleter::new();
        let mut p: Option<Box<[i32]>> = unimplemented!();
        let cp: *const Option<Box<[i32]>> = &p;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(((unsafe { Deleter::test_4(&(*unimplemented!()),) }) == (6)));
        assert!(((unsafe { Deleter::test_4(&(*unimplemented!()),) }) == (6)));
    }
    {
        let mut d: Deleter = Deleter::new();
        let mut p: Option<Box<[i32]>> = unimplemented!();
        let cp: *const Option<Box<[i32]>> = &p;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(((unsafe { Deleter::test_3(&mut (*unimplemented!()),) }) == (5)));
        assert!(((unsafe { Deleter::test_3(&mut (*unimplemented!()),) }) == (5)));
    }
}
pub unsafe fn test_4() -> bool {
    (unsafe { test_basic_2() });
    (unsafe { test_basic_3() });
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
    (unsafe { test_4() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
