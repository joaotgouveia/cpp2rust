extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_fputc_0() {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_fputs_1() {
    unimplemented!();
    unimplemented!();
    let mut s: *const () = (c"from variable".as_ptr().cast_mut()).cast_const();
    unimplemented!();
    unimplemented!();
    let mut buf: [(); 4] = [('b' as ()), ('u' as ()), ('f' as ()), ('\0' as ())];
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_puts_2() {
    unimplemented!();
    let mut s: *const () = (c"puts variable".as_ptr().cast_mut()).cast_const();
    unimplemented!();
}
pub unsafe fn test_fileno_3() {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut file: *const () = (c"cpp2rust_fileno_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test_fputc_0() });
    (unsafe { test_fputs_1() });
    (unsafe { test_puts_2() });
    (unsafe { test_fileno_3() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
