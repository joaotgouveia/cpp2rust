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
pub struct NoDefault {}
impl NoDefault {
    pub unsafe fn new(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe)]
pub struct NonTrivialCopy {}
impl NonTrivialCopy {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_from(_a0: *const NonTrivialCopy) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const NonTrivialCopy) -> *mut NonTrivialCopy {
        return &mut (*(self as *mut NonTrivialCopy));
    }
}
impl Clone for NonTrivialCopy {
    fn clone(&self) -> Self {
        unsafe { NonTrivialCopy::copy_from(self as *const NonTrivialCopy) }
    }
}
impl Default for NonTrivialCopy {
    fn default() -> Self {
        unsafe { NonTrivialCopy::new() }
    }
}
pub unsafe fn tests_0() -> () {
    {
        let mut array: () = unimplemented!();
        let mut copy: () = array.clone();
        copy = (array).clone();
        const _: () = assert!(true, "std::is_copy_constructible<Array>::value");;
        const _: () = assert!(true, "std::is_copy_assignable<Array>::value");;
    }
    {
        let mut array: () = unimplemented!();
        let mut copy: () = array.clone();
        &(copy);
        const _: () = assert!(true, "std::is_copy_constructible<Array>::value");;
        const _: () = assert!(true, "");;
    }
    {
        let mut array: () = unimplemented!();
        let mut copy: () = array.clone();
        copy = (array).clone();
        const _: () = assert!(true, "std::is_copy_constructible<Array>::value");;
        const _: () = assert!(true, "std::is_copy_assignable<Array>::value");;
    }
    {
        let mut array: () = unimplemented!();
        let mut copy: () = array.clone();
        &(copy);
        const _: () = assert!(true, "std::is_copy_constructible<Array>::value");;
        const _: () = assert!(true, "");;
    }
    {
        let mut array: () = unimplemented!();
        let mut copy: () = array.clone();
        copy = (array).clone();
        const _: () = assert!(true, "std::is_copy_constructible<Array>::value");;
        const _: () = assert!(true, "std::is_copy_assignable<Array>::value");;
    }
    {
        let mut array: () = unimplemented!();
        let mut copy: () = array.clone();
        &(copy);
        const _: () = assert!(true, "std::is_copy_constructible<Array>::value");;
        const _: () = assert!(true, "");;
    }
    {
        let mut array: () = unimplemented!();
        let mut copy: () = array.clone();
        copy = (array).clone();
        const _: () = assert!(true, "std::is_copy_constructible<Array>::value");;
    }
    {
        let mut array: () = unimplemented!();
        let mut copy: () = array.clone();
        copy = (array).clone();
        const _: () = assert!(true, "std::is_copy_constructible<Array>::value");;
    }
    {
        let mut array: () = unimplemented!();
        let mut copy: () = array.clone();
        copy = (array).clone();
        const _: () = assert!(true, "std::is_copy_constructible<Array>::value");;
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { tests_0() });
    const _: () = assert!(true, "tests()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
