extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn get_greeting_0() -> *const () {
    return (c"hello".as_ptr().cast_mut()).cast_const();
}
pub unsafe fn get_empty_1() -> *const () {
    return (c"".as_ptr().cast_mut()).cast_const();
}
pub unsafe fn get_branch_2(mut x: ()) -> *const () {
    if ((((x) > (unimplemented!())) as ()) != 0) {
        return (c"positive".as_ptr().cast_mut()).cast_const();
    }
    return (c"non-positive".as_ptr().cast_mut()).cast_const();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: *const () = (unsafe { get_greeting_0() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut b: *const () = (unsafe { get_empty_1() });
    unimplemented!();
    let mut c: *const () = (unsafe { get_branch_2(1_()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut d: *const () = (unsafe { get_branch_2(-1_()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
