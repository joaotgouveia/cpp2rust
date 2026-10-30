extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn tests_0() -> () {
    let mut dd0: *const () = (&raw const unimplemented!() as *const ());
    let mut dd1: *const () = (&raw const unimplemented!() as *const ());
    let mut dd2: *const () = (&raw const unimplemented!() as *const ());
    let mut dd3: *const () = (&raw const unimplemented!() as *const ());
    let mut dd4: *const () = (&raw const unimplemented!() as *const ());
    let mut dd5: *const () = (&raw const unimplemented!() as *const ());
    let mut dd6: *const () = (&raw const unimplemented!() as *const ());
    let mut dd7: *const () = (&raw const unimplemented!() as *const ());
    let mut dd8: *const () = (&raw const unimplemented!() as *const ());
    let mut dd9: *const () = (&raw const unimplemented!() as *const ());
    let mut dd10: *const () = (&raw const unimplemented!() as *const ());
    let mut dd11: *const () = (&raw const unimplemented!() as *const ());
    let mut dd12: *const () = (&raw const unimplemented!() as *const ());
    let mut f0: *const () = (&raw const unimplemented!() as *const ());
    let mut f1: *const () = (&raw const unimplemented!() as *const ());
    let mut f2: *const () = (&raw const unimplemented!() as *const ());
    let mut f3: *const () = (&raw const unimplemented!() as *const ());
    let mut f4: *const () = (&raw const unimplemented!() as *const ());
    let mut f5: *const () = (&raw const unimplemented!() as *const ());
    let mut f6: *const () = (&raw const unimplemented!() as *const ());
    let mut f7: *const () = (&raw const unimplemented!() as *const ());
    let mut f8: *const () = (&raw const unimplemented!() as *const ());
    let mut f9: *const () = (&raw const unimplemented!() as *const ());
    let mut f10: *const () = (&raw const unimplemented!() as *const ());
    let mut f11: *const () = (&raw const unimplemented!() as *const ());
    let mut f12: *const () = (&raw const unimplemented!() as *const ());
    let mut d0: *const () = (&raw const unimplemented!() as *const ());
    let mut d1: *const () = (&raw const unimplemented!() as *const ());
    let mut d2: *const () = (&raw const unimplemented!() as *const ());
    let mut d3: *const () = (&raw const unimplemented!() as *const ());
    let mut d4: *const () = (&raw const unimplemented!() as *const ());
    let mut d5: *const () = (&raw const unimplemented!() as *const ());
    let mut d6: *const () = (&raw const unimplemented!() as *const ());
    let mut d7: *const () = (&raw const unimplemented!() as *const ());
    let mut d8: *const () = (&raw const unimplemented!() as *const ());
    let mut d9: *const () = (&raw const unimplemented!() as *const ());
    let mut d10: *const () = (&raw const unimplemented!() as *const ());
    let mut d11: *const () = (&raw const unimplemented!() as *const ());
    let mut d12: *const () = (&raw const unimplemented!() as *const ());
    let mut ld0: *const () = (&raw const unimplemented!() as *const ());
    let mut ld1: *const () = (&raw const unimplemented!() as *const ());
    let mut ld2: *const () = (&raw const unimplemented!() as *const ());
    let mut ld3: *const () = (&raw const unimplemented!() as *const ());
    let mut ld4: *const () = (&raw const unimplemented!() as *const ());
    let mut ld5: *const () = (&raw const unimplemented!() as *const ());
    let mut ld6: *const () = (&raw const unimplemented!() as *const ());
    let mut ld7: *const () = (&raw const unimplemented!() as *const ());
    let mut ld8: *const () = (&raw const unimplemented!() as *const ());
    let mut ld9: *const () = (&raw const unimplemented!() as *const ());
    let mut ld10: *const () = (&raw const unimplemented!() as *const ());
    let mut ld11: *const () = (&raw const unimplemented!() as *const ());
    let mut ld12: *const () = (&raw const unimplemented!() as *const ());
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
