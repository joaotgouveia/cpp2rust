extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn tests_0() -> () {
    let mut f0: () = unimplemented!();
    let mut f1: () = unimplemented!();
    let mut f2: () = unimplemented!();
    let mut f3: () = unimplemented!();
    let mut f4: () = unimplemented!();
    let mut f5: () = unimplemented!();
    let mut f6: () = unimplemented!();
    let mut f7: () = unimplemented!();
    let mut f8: () = unimplemented!();
    let mut f9: () = unimplemented!();
    let mut f10: () = unimplemented!();
    let mut f11: () = unimplemented!();
    let mut f12: () = unimplemented!();
    let mut d0: () = unimplemented!();
    let mut d1: () = unimplemented!();
    let mut d2: () = unimplemented!();
    let mut d3: () = unimplemented!();
    let mut d4: () = unimplemented!();
    let mut d5: () = unimplemented!();
    let mut d6: () = unimplemented!();
    let mut d7: () = unimplemented!();
    let mut d8: () = unimplemented!();
    let mut d9: () = unimplemented!();
    let mut d10: () = unimplemented!();
    let mut d11: () = unimplemented!();
    let mut d12: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut ld0: () = unimplemented!();
    let mut ld1: () = unimplemented!();
    let mut ld2: () = unimplemented!();
    let mut ld3: () = unimplemented!();
    let mut ld4: () = unimplemented!();
    let mut ld5: () = unimplemented!();
    let mut ld6: () = unimplemented!();
    let mut ld7: () = unimplemented!();
    let mut ld8: () = unimplemented!();
    let mut ld9: () = unimplemented!();
    let mut ld10: () = unimplemented!();
    let mut ld11: () = unimplemented!();
    let mut ld12: () = unimplemented!();
    return true;
}
const _: () = assert!(true, "tests()");
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
    (unsafe { tests_0() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
