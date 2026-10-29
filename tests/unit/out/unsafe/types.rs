extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
const _: () = assert!(
    ((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>())),
    "sizeof(std::size_t) == sizeof(void *)"
);
const _: () = assert!(
    ((::std::mem::size_of::<*mut ()>()) == (8_())),
    "sizeof(int *) == 8"
);
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let xu8: () = 8_();
    let xu16: () = 16_();
    let mut xu32: () = 32_();
    let mut xu64: () = 64_();
    let xsz1: () = 64_();
    let mut xsz2: () = 64_();
    let xi1: () = -8_();
    let xi2: () = 16_();
    let mut xi3: () = 32_();
    let mut xi4: () = 64_();
    let mut b: () = ((xu64) == (64_()));
    let mut xld: () = 1.5_f64;
    let mut xwc: () = 65_();
    let mut xc8: () = 66_();
    let mut xc16: () = 67_();
    let mut xc32: () = 68_();
    let mut xnp: () = Default::default();
    unimplemented!();
    unimplemented!();
    let mut pi: () = 3.14159265358979_f64;
    unimplemented!();
    let mut dmax: () = -1.7976931348623157e+308_f64;
    unimplemented!();
    let mut big: () = f64::INFINITY;
    unimplemented!();
    let mut fbig: () = f32::INFINITY;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut ip: *mut () = std::ptr::null_mut();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
