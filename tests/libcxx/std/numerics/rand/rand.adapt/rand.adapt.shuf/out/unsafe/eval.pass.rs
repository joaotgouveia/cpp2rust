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
pub struct rand1_unsigned_long_long__0__18446744073709551615_ {
    x_: (),
}
impl rand1_unsigned_long_long__0__18446744073709551615_ {
    pub unsafe fn min() -> () {
        return 0_();
    }
    pub unsafe fn max() -> () {
        return 18446744073709551615_();
    }
    pub unsafe fn new(mut sd: Option<()>) -> Self {
        let mut sd: () = sd.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { x_: sd };
        if ((this.x_) > (18446744073709551615_())) {
            this.x_ = 18446744073709551615_();
        }
        this
    }
    pub unsafe fn operator_call(&mut self) -> () {
        let mut r: () = self.x_;
        if ((self.x_) < (18446744073709551615_())) {
            self.x_.prefix_inc();
        } else {
            self.x_ = 0_();
        }
        return r;
    }
}
impl Default for rand1_unsigned_long_long__0__18446744073709551615_ {
    fn default() -> Self {
        unsafe { rand1_unsigned_long_long__0__18446744073709551615_::new(None) }
    }
}
pub unsafe fn test1_0() {
    let mut e: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test2_1() {
    let mut e: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test3_2() {
    let mut e: () = unimplemented!();
    unimplemented!();
    unimplemented!();
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
    (unsafe { test1_0() });
    (unsafe { test2_1() });
    (unsafe { test3_2() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
