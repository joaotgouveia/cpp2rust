extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test01_0() {
    let mut c: libc::c_char = ('a' as libc::c_char);
    let cc: libc::c_char = ('b' as libc::c_char);
    let mut c1: *mut libc::c_char = (&mut c as *mut libc::c_char);
    let mut cc1: *const libc::c_char = (&cc as *const libc::c_char);
    let mut ccarray1: *const libc::c_char = c"san francisco roof garden inspectors".as_ptr();
    let mut ccarray2: *const libc::c_char = c"san francisco sunny-day park inspectors".as_ptr();
    let mut carray: [libc::c_char; 50] = [(0 as libc::c_char); 50];
    unimplemented!();
    let mut v: *mut ::libc::c_void = (carray.as_mut_ptr() as *mut ::libc::c_void);
    let mut cv: *const ::libc::c_void = (ccarray1 as *const ::libc::c_void);
    cc1 = (libc::strchr(ccarray1, (('c' as libc::c_char) as i32)) as *const libc::c_char);
    c1 = unimplemented!();
    cc1 = (libc::strpbrk(ccarray1, ccarray2) as *const libc::c_char);
    c1 = libc::strpbrk(carray.as_mut_ptr(), ccarray2);
    cc1 = (libc::strrchr(ccarray1, (('c' as libc::c_char) as i32)) as *const libc::c_char);
    c1 = libc::strrchr(carray.as_mut_ptr(), (('c' as libc::c_char) as i32));
    cc1 = (libc::strstr(ccarray1, ccarray2) as *const libc::c_char);
    c1 = libc::strstr(carray.as_mut_ptr(), (carray.as_mut_ptr()).cast_const());
    cv = libc::memchr(
        cv as *const ::libc::c_void,
        (('a' as libc::c_char) as i32),
        3_usize as usize,
    ) as *const ::libc::c_void;
    v = unimplemented!();
    cc1 = cc1;
    c1 = c1;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test01_0() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
