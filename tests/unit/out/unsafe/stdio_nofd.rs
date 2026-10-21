extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_fputc_fputs_0() {
    let mut path: *const () = (c"cpp2rust_stdio_nofd_puts.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    fp = unimplemented!();
    unimplemented!();
    let mut buf: [(); 16] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_puts_1() {
    unimplemented!();
}
pub unsafe fn test_fgets_getc_2() {
    let mut path: *const () = (c"cpp2rust_stdio_nofd_gets.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    fp = unimplemented!();
    unimplemented!();
    let mut buf: [(); 8] = [unimplemented!(); 8];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_freopen_3() {
    let mut path: *const () = (c"cpp2rust_stdio_nofd_reopen.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut fp2: () = unimplemented!();
    unimplemented!();
    let mut buf: [(); 8] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_fseeko_4() {
    let mut path: *const () = (c"cpp2rust_stdio_nofd_seek.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    fp = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut buf: [(); 8] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_rename_5() {
    let mut from: *const () = (c"cpp2rust_stdio_nofd_from.tmp".as_ptr().cast_mut()).cast_const();
    let mut to: *const () = (c"cpp2rust_stdio_nofd_to.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    fp = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_setvbuf_6() {
    let mut path: *const () = (c"cpp2rust_stdio_nofd_vbuf.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: () = unimplemented!();
    unimplemented!();
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
    (unsafe { test_fputc_fputs_0() });
    (unsafe { test_puts_1() });
    (unsafe { test_fgets_getc_2() });
    (unsafe { test_freopen_3() });
    (unsafe { test_fseeko_4() });
    (unsafe { test_rename_5() });
    (unsafe { test_setvbuf_6() });
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
