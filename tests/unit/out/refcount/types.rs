extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
const _: () = assert!(
    (::std::mem::size_of::<()>() == 8usize),
    "sizeof(std::size_t) == sizeof(void *)"
);
const _: () = assert!((8usize == 8_()), "sizeof(int *) == 8");
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut xu8: () = 8_();
    let mut xu16: () = 16_();
    let mut xu32: () = 32_();
    let mut xu64: () = 64_();
    let mut xsz1: () = 64_();
    let mut xsz2: () = 64_();
    let mut xi1: () = -8_();
    let mut xi2: () = 16_();
    let mut xi3: () = 32_();
    let mut xi4: () = 64_();
    let mut b: () = (xu64 == 64_());
    let mut xld: () = 1.5_f64;
    let mut xwc: () = 65_();
    let mut xc8: () = 66_();
    let mut xc16: () = 67_();
    let mut xc32: () = 68_();
    let xnp: Value<()> = Rc::new(RefCell::new(Default::default()));
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
    let mut ip: Ptr<()> = Ptr::<()>::null();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
