extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn __asan_default_options_0() -> *const libc::c_char {
    return c"check_initialization_order=true:strict_init_order=true".as_ptr();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Global {}
impl Global {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        {
            let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
            let __s: *mut ::std::fs::File = __r;
            let _ = (*__s).write_all(::std::ffi::CStr::from_ptr(c"Hello!".as_ptr()).to_bytes());
            __s
        };
        this
    }
}
impl Default for Global {
    fn default() -> Self {
        unsafe { Global::new() }
    }
}
pub static mut global_1: std::cell::LazyCell<Global> =
    std::cell::LazyCell::new(|| unsafe { Global::new() });

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
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const global_1);
}
