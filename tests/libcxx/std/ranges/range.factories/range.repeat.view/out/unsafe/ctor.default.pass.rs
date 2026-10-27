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
pub struct DefaultInt42 {
    pub value: (),
}
impl Default for DefaultInt42 {
    fn default() -> Self {
        unsafe { DefaultInt42 { value: 42_() } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Int {}
impl Int {
    pub unsafe fn new(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
}
const _: () = assert!(
    true,
    "std::default_initializable<std::ranges::repeat_view<DefaultInt42>>"
);
const _: () = assert!(
    !(false),
    "!std::default_initializable<std::ranges::repeat_view<Int>>"
);
pub unsafe fn test_0() -> () {
    let mut rv: () = unimplemented!();
    unimplemented!();
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
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
