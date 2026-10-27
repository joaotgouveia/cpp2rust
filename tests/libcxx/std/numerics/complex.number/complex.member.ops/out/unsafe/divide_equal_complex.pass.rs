extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn is_close_0(mut v: (), mut comp: ()) -> () {
    return ((v) <= ((comp) + (unimplemented!()))) && ((v) >= ((comp) - (unimplemented!())));
}
pub unsafe fn is_close_1(mut v: (), mut comp: ()) -> () {
    return ((v) <= ((comp) + (unimplemented!()))) && ((v) >= ((comp) - (unimplemented!())));
}
pub unsafe fn is_close_2(mut v: (), mut comp: ()) -> () {
    return ((v) <= ((comp) + (unimplemented!()))) && ((v) >= ((comp) - (unimplemented!())));
}
pub unsafe fn test_3() -> () {
    let mut c: () = unimplemented!();
    let c2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut c3: () = unimplemented!();
    unimplemented!();
    let mut ic: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut fc: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_4() -> () {
    let mut c: () = unimplemented!();
    let c2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut c3: () = unimplemented!();
    unimplemented!();
    let mut ic: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut fc: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_5() -> () {
    let mut c: () = unimplemented!();
    let c2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut c3: () = unimplemented!();
    unimplemented!();
    let mut ic: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut fc: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
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
    (unsafe { test_3() });
    (unsafe { test_4() });
    (unsafe { test_5() });
    const _: () = assert!(true, "test<float>()");;
    const _: () = assert!(true, "test<double>()");;
    const _: () = assert!(true, "test<long double>()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
