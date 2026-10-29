extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_long__long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct short_container {}
impl short_container {
    pub unsafe fn size(&self) -> () {
        return 60000_();
    }
}
pub unsafe fn test_container_1(c: *mut ()) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    unimplemented!();
}
pub unsafe fn test_container_2(c: *mut ()) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    unimplemented!();
}
pub unsafe fn test_container_3(c: *mut ()) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    unimplemented!();
}
pub unsafe fn test_container_4(c: *mut ()) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    unimplemented!();
}
pub unsafe fn test_const_container_5(c: *const ()) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    unimplemented!();
}
pub unsafe fn test_const_container_6(c: *const ()) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    unimplemented!();
}
pub unsafe fn test_const_container_7(c: *const ()) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    unimplemented!();
}
pub unsafe fn test_const_container_8(c: *const ()) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    unimplemented!();
}
pub unsafe fn test_const_container_9(c: *const ()) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    unimplemented!();
}
pub unsafe fn test_container_10(c: *mut ()) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    unimplemented!();
}
pub unsafe fn test_const_array_11(array: *const [(); 3]) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(array))>");;
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
    let mut v: () = unimplemented!();
    unimplemented!();
    let mut l: () = unimplemented!();
    unimplemented!();
    let mut a: () = unimplemented!();
    (*unimplemented!()) = 3_();
    let mut il: () = unimplemented!();
    (unsafe { test_container_1(&mut v) });
    const _: () = assert!(true, "");;
    (unsafe { test_container_2(&mut l) });
    const _: () = assert!(true, "");;
    (unsafe { test_container_3(&mut a) });
    const _: () = assert!(true, "");;
    (unsafe { test_container_10(&mut il) });
    const _: () = assert!(true, "");;
    (unsafe { test_const_container_5(&v) });
    (unsafe { test_const_container_6(&l) });
    (unsafe { test_const_container_7(&a) });
    (unsafe { test_const_container_9(&il) });
    let mut sv: () = unimplemented!();
    (unsafe { test_container_4(&mut sv) });
    const _: () = assert!(true, "");;
    (unsafe { test_const_container_8(&sv) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(arrA))>");;
    (unsafe { test_const_array_11(&(*std::cell::LazyCell::force_mut(&mut *&raw mut arrA_12))) });
    let mut sc: short_container = <short_container>::default();
    const _: () = assert!(
        true,
        "std::is_signed_v<                      decltype(std::ssize(sc))>"
    );;
    const _: () = assert!(
        true,
        "std::numeric_limits<                   decltype(std::ssize(sc))>::max()  > 60000"
    );;
    const _: () = assert!(
        true,
        "std::numeric_limits<std::make_signed_t<decltype(std:: size(sc))>>::max() < 60000"
    );;
    unimplemented!();
    const _: () = assert!(true, "");;
    return 0_();
}
static mut arrA_12: std::cell::LazyCell<[(); 3]> =
    std::cell::LazyCell::new(|| unsafe { [1_(), 2_(), 3_()] });
pub unsafe fn __cpp2rust_init_globals() {}
