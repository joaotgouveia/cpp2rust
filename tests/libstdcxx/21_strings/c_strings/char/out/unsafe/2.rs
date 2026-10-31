extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test02_0() {
    let mut ccarray1: *const libc::c_char = c"san francisco roof garden inspectors".as_ptr();
    let mut ccarray2: *const libc::c_char = c"san francisco sunny-day park inspectors".as_ptr();
    let mut carray: [libc::c_char; 50] = [(0 as libc::c_char); 50];
    unimplemented!();
    let mut cv: *const ::libc::c_void = (ccarray1 as *const ::libc::c_void);
    let mut cv1: *const ::libc::c_void = std::ptr::null();
    let mut cc: *const libc::c_char = std::ptr::null();
    let mut c: *mut libc::c_char = std::ptr::null_mut();
    cv1 = libc::memchr(
        cv as *const ::libc::c_void,
        (('/' as libc::c_char) as i32),
        3_usize as usize,
    ) as *const ::libc::c_void;
    cc = (libc::strchr(ccarray1, (('/' as libc::c_char) as i32)) as *const libc::c_char);
    cc = (libc::strrchr(ccarray1, (('c' as libc::c_char) as i32)) as *const libc::c_char);
    cc = (libc::strpbrk(ccarray1, ccarray2) as *const libc::c_char);
    c = libc::strstr(carray.as_mut_ptr(), (carray.as_mut_ptr()).cast_const());
    cv1 = cv1;
    cc = cc;
    c = c;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test02_0() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
