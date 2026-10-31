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
pub struct multiply {}
impl multiply {
    pub unsafe fn operator_call_1_int(&mut self, mut a: i32, mut b: i32) -> i32 {
        return ((a) * (b));
    }
}
impl Callable2<i32, i32, i32> for multiply {
    fn call(&self, a1: i32, a2: i32) -> i32 {
        unsafe { multiply::operator_call_1_int(&mut *(&raw const *self).cast_mut(), a1, a2) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct plus_one {}
impl plus_one {
    pub unsafe fn operator_call_1_int(&mut self, mut a: i32) -> i32 {
        return ((a) + (1));
    }
}
impl Callable1<i32, i32> for plus_one {
    fn call(&self, a1: i32) -> i32 {
        unsafe { plus_one::operator_call_1_int(&mut *(&raw const *self).cast_mut(), a1) }
    }
}
pub unsafe fn test_0() -> bool {
    let mut g: () = unimplemented!();
    assert!(
        ((unsafe {
            let mut ___args: i32 = 5;
            g(&mut ___args)
        }) == (10))
    );
    assert!(
        ((unsafe {
            let mut ___args: i32 = 5;
            unimplemented!()(&mut ___args)
        }) == (11))
    );
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
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
