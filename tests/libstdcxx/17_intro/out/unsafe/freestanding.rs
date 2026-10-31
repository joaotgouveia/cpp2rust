extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut e: () = unimplemented!();
    let mut str: *const libc::c_char = unimplemented!();
    let mut limit_l: () = unimplemented!();
    let mut r: i32 = unimplemented!();
    let mut cp: *const libc::c_char =
        (Box::leak(Box::new((0 as libc::c_char))) as *mut libc::c_char).cast_const();
    {
        let __p = cp;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p.cast_mut()))
        }
    };
    let mut b: bool = unimplemented!();
    let mut ilisti: Vec<i32> = unimplemented!();
    const _: () = assert!(true, "std::has_single_bit(256u)");;
    const _: () = assert!(
        ((201411_i64) >= (201411_i64)),
        "__cpp_lib_void_t >= 201411L"
    );;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
