extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_constexpr_0() -> () {
    let mut io: () = unimplemented!();
    let mut i: () = io.clone();
    return true;
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe)]
pub struct throwing_copy_constructor {}
impl throwing_copy_constructor {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_from(_a0: *const throwing_copy_constructor) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const throwing_copy_constructor,
    ) -> *mut throwing_copy_constructor {
        return &mut (*(self as *mut throwing_copy_constructor));
    }
}
impl Clone for throwing_copy_constructor {
    fn clone(&self) -> Self {
        unsafe { throwing_copy_constructor::copy_from(self as *const throwing_copy_constructor) }
    }
}
impl Default for throwing_copy_constructor {
    fn default() -> Self {
        unsafe { throwing_copy_constructor::new() }
    }
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
    {
        let mut io: () = unimplemented!();
        let mut i: () = io.clone();
        unimplemented!();
        const _: () = assert!(
            true,
            "std::is_nothrow_copy_constructible<std::istream_iterator<int>>::value"
        );;
    }
    {
        let mut io: () = unimplemented!();
        let mut i: () = io.clone();
        unimplemented!();
        const _: () = assert!(
            true,
            "!std::is_nothrow_copy_constructible<std::istream_iterator<throwing_copy_constructor>>::value"
        );;
    }
    {
        let mut inf: () = unimplemented!();
        let mut io: () = unimplemented!();
        let mut i: () = io.clone();
        unimplemented!();
        let mut j: () = 0_();
        j = (*unimplemented!());
        unimplemented!();
    }
    const _: () = assert!(true, "test_constexpr()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
