extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn tests_0() -> bool {
    let mut dd0: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut dd1: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut dd2: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut dd3: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut dd4: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut dd5: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut dd6: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut dd7: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut dd8: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut dd9: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut dd10: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut dd11: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut dd12: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut f0: *const f32 = (&raw const unimplemented!() as *const f32);
    let mut f1: *const f32 = (&raw const unimplemented!() as *const f32);
    let mut f2: *const f32 = (&raw const unimplemented!() as *const f32);
    let mut f3: *const f32 = (&raw const unimplemented!() as *const f32);
    let mut f4: *const f32 = (&raw const unimplemented!() as *const f32);
    let mut f5: *const f32 = (&raw const unimplemented!() as *const f32);
    let mut f6: *const f32 = (&raw const unimplemented!() as *const f32);
    let mut f7: *const f32 = (&raw const unimplemented!() as *const f32);
    let mut f8: *const f32 = (&raw const unimplemented!() as *const f32);
    let mut f9: *const f32 = (&raw const unimplemented!() as *const f32);
    let mut f10: *const f32 = (&raw const unimplemented!() as *const f32);
    let mut f11: *const f32 = (&raw const unimplemented!() as *const f32);
    let mut f12: *const f32 = (&raw const unimplemented!() as *const f32);
    let mut d0: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut d1: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut d2: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut d3: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut d4: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut d5: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut d6: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut d7: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut d8: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut d9: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut d10: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut d11: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut d12: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut ld0: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut ld1: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut ld2: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut ld3: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut ld4: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut ld5: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut ld6: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut ld7: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut ld8: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut ld9: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut ld10: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut ld11: *const f64 = (&raw const unimplemented!() as *const f64);
    let mut ld12: *const f64 = (&raw const unimplemented!() as *const f64);
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { tests_0() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
