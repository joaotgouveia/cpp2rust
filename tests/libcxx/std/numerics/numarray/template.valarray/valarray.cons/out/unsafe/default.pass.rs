extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct S {}
impl S {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut ctor_called_0)) = true;
        this
    }
}
impl Default for S {
    fn default() -> Self {
        unsafe { S::new() }
    }
}
pub static mut ctor_called_0: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });

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
    {
        let mut v: () = unimplemented!();
        assert!(((unimplemented!()) == (0_usize)));
    }
    {
        let mut v: () = unimplemented!();
        assert!(((unimplemented!()) == (0_usize)));
    }
    {
        let mut v: () = unimplemented!();
        assert!(((unimplemented!()) == (0_usize)));
    }
    {
        let mut v: () = unimplemented!();
        assert!(((unimplemented!()) == (0_usize)));
    }
    {
        let mut v: () = unimplemented!();
        assert!(((unimplemented!()) == (0_usize)));
        assert!(!(*std::cell::LazyCell::force_mut(&mut *&raw mut ctor_called_0)));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const ctor_called_0);
}
