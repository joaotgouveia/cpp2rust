extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_const_container_0(c: *const ()) {
    unimplemented!();
}
pub unsafe fn test_const_container_1(c: *const ()) {
    unimplemented!();
}
pub unsafe fn test_const_container_2(c: *const ()) {
    unimplemented!();
}
pub unsafe fn test_const_container_3(c: *const ()) {
    unimplemented!();
}
pub unsafe fn test_const_container_4(c: *const ()) {
    unimplemented!();
}
pub unsafe fn test_container_5(c: *mut ()) {
    unimplemented!();
}
pub unsafe fn test_container_6(c: *mut ()) {
    unimplemented!();
}
pub unsafe fn test_container_7(c: *mut ()) {
    unimplemented!();
}
pub unsafe fn test_container_8(c: *mut ()) {
    unimplemented!();
}
pub unsafe fn test_container_9(c: *mut ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn test_const_array_10(array: *const [(); 3]) {
    const _: () = assert!(true, "");;
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
    (unsafe { test_container_5(&mut v) });
    (unsafe { test_container_6(&mut l) });
    (unsafe { test_container_7(&mut a) });
    (unsafe { test_container_9(&mut il) });
    (unsafe { test_const_container_0(&v) });
    (unsafe { test_const_container_1(&l) });
    (unsafe { test_const_container_2(&a) });
    (unsafe { test_const_container_4(&il) });
    let mut sv: () = unimplemented!();
    (unsafe { test_container_8(&mut sv) });
    (unsafe { test_const_container_3(&sv) });
    (unsafe { test_const_array_10(&(*std::cell::LazyCell::force_mut(&mut *&raw mut arrA_11))) });
    return 0_();
}
static mut arrA_11: std::cell::LazyCell<[(); 3]> =
    std::cell::LazyCell::new(|| unsafe { [1_(), 2_(), 3_()] });
pub unsafe fn __cpp2rust_init_globals() {}
