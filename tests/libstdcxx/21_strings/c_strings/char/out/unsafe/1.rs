extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test01_0() {
    let mut c: () = ('a' as ());
    let cc: () = ('b' as ());
    let mut c1: *mut () = (&mut c as *mut ());
    let mut cc1: *const () = (&cc as *const ());
    let mut ccarray1: *const () = c"san francisco roof garden inspectors".as_ptr();
    let mut ccarray2: *const () = c"san francisco sunny-day park inspectors".as_ptr();
    let mut carray: [(); 50] = [unimplemented!(); 50];
    (unsafe { strcpy_1(carray.as_mut_ptr(), ccarray1) });
    let mut v: () = (carray.as_mut_ptr() as ());
    let mut cv: () = (ccarray1 as ());
    cc1 = (unsafe { strchr_2(ccarray1, ('c' as ())) });
    c1 = (unsafe { strchr_3(carray.as_mut_ptr(), ('c' as ())) });
    cc1 = (unsafe { strpbrk_4(ccarray1, ccarray2) });
    c1 = (unsafe { strpbrk_5(carray.as_mut_ptr(), ccarray2) });
    cc1 = (unsafe { strrchr_6(ccarray1, ('c' as ())) });
    c1 = (unsafe { strrchr_7(carray.as_mut_ptr(), ('c' as ())) });
    cc1 = (unsafe { strstr_8(ccarray1, ccarray2) });
    c1 = (unsafe {
        let ___haystack: *mut () = carray.as_mut_ptr();
        let ___needle: *const () = (carray.as_mut_ptr()).cast_const();
        strstr_9(___haystack, ___needle)
    });
    cv = (unsafe { memchr_10(cv, ('a' as ()), 3_()) });
    v = (unsafe { memchr_11(v, ('a' as ()), 3_()) });
    cc1 = cc1;
    c1 = c1;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test01_0() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
