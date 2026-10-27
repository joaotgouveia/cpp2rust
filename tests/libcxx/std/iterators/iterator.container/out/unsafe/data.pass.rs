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
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn test_container_4(c: *mut ()) {
    unimplemented!();
}
pub unsafe fn test_container_5(c: *mut ()) {
    unimplemented!();
}
pub unsafe fn test_container_6(c: *mut ()) {
    unimplemented!();
}
pub unsafe fn test_container_7(c: *mut ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn test_const_array_8(array: *const [(); 3]) {
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
    let mut a: () = unimplemented!();
    (*unimplemented!()) = 3_();
    let mut il: () = unimplemented!();
    (unsafe { test_container_4(&mut v) });
    (unsafe { test_container_5(&mut a) });
    (unsafe { test_container_7(&mut il) });
    (unsafe { test_const_container_0(&v) });
    (unsafe { test_const_container_1(&a) });
    (unsafe { test_const_container_3(&il) });
    let mut sv: () = unimplemented!();
    (unsafe { test_container_6(&mut sv) });
    (unsafe { test_const_container_2(&sv) });
    (unsafe { test_const_array_8(&(*std::cell::LazyCell::force_mut(&mut *&raw mut arrA_9))) });
    return 0_();
}
static mut arrA_9: std::cell::LazyCell<[(); 3]> =
    std::cell::LazyCell::new(|| unsafe { [1_(), 2_(), 3_()] });
pub unsafe fn __cpp2rust_init_globals() {}
