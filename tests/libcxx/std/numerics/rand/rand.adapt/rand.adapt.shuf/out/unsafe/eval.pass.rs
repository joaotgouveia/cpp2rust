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
    x_: u64,
}
impl rand1_unsigned_long_long__0__18446744073709551615_ {
    pub unsafe fn min() -> u64 {
        return 0_u64;
    }
    pub unsafe fn max() -> u64 {
        return 18446744073709551615_u64;
    }
    pub unsafe fn new(mut sd: Option<u64>) -> Self {
        let mut sd: u64 = sd.unwrap_or_else(|| unsafe { 0_u64 });
        let mut this = Self { x_: sd };
        if ((this.x_) > (18446744073709551615_u64)) {
            this.x_ = 18446744073709551615_u64;
        }
        this
    }
    pub unsafe fn operator_call(&mut self) -> u64 {
        let mut r: u64 = self.x_;
        if ((self.x_) < (18446744073709551615_u64)) {
            self.x_.prefix_inc();
        } else {
            self.x_ = 0_u64;
        }
        return r;
    }
}
impl Callable0<u64> for rand1_unsigned_long_long__0__18446744073709551615_ {
    fn call(&self) -> u64 {
        unsafe {
            rand1_unsigned_long_long__0__18446744073709551615_::operator_call(
                &mut *(&raw const *self).cast_mut(),
            )
        }
    }
}
impl Default for rand1_unsigned_long_long__0__18446744073709551615_ {
    fn default() -> Self {
        unsafe { rand1_unsigned_long_long__0__18446744073709551615_::new(None) }
    }
}
pub unsafe fn test1_0() {
    let mut e: () = unimplemented!();
    assert!(((unimplemented!()) == (152607844_())));
}
pub unsafe fn test2_1() {
    let mut e: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!()) == (501_())));
}
pub unsafe fn test3_2() {
    let mut e: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!()) == (500_())));
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
    (unsafe { test1_0() });
    (unsafe { test2_1() });
    (unsafe { test3_2() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
