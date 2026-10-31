extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
impl Key {
    pub unsafe fn new(_a0: *const i32) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, ptr)]
pub struct Key {}
impl Key {
    pub unsafe fn operator_eq(&self, _a0: *const Key) -> bool {
        return true;
    }
}
impl std::cmp::PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Key::operator_eq(self, other as *const Key) }
    }
}
impl std::cmp::Eq for Key {}
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
    let mut map: () = unimplemented!();
    let mut it: () = unimplemented!();
    assert!(it == unimplemented!());
    let mut result: ((), bool) = unimplemented!();
    assert!((result.1 as bool));
    assert!((((*result.0.iter_deref()).1) == (42)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
