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
    unimplemented!();
    let mut v: () = (carray.as_mut_ptr() as ());
    let mut cv: () = (ccarray1 as ());
    cc1 = unimplemented!();
    c1 = unimplemented!();
    cc1 = unimplemented!();
    c1 = unimplemented!();
    cc1 = unimplemented!();
    c1 = unimplemented!();
    cc1 = unimplemented!();
    c1 = unimplemented!();
    cv = unimplemented!();
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
unsafe fn main_0() -> () {
    (unsafe { test01_0() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
