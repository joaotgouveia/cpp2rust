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
unsafe fn main_0() -> () {
    let mut x: () = unimplemented!();
    let mut a: [(); 3] = [unimplemented!(), 1_(), 2_()];
    'loop_: while ((x) < (3_())) {
        a[(x.postfix_inc()) as usize].prefix_inc();
    }
    let mut out: () = unimplemented!();
    'loop_: while (x != 0) {
        out += a[(x.prefix_dec()) as usize];
    }
    out.postfix_inc();
    let mut x2: () = out.prefix_dec();
    out.prefix_inc();
    let mut x3: () = out.postfix_dec();
    unimplemented!();
    let mut n: () = x2;
    let mut d: () = 1.5_f64;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut wide: () = 5_();
    let mut w1: () = wide.postfix_inc();
    let mut w2: () = wide.prefix_inc();
    let mut w3: () = wide.postfix_dec();
    let mut w4: () = wide.prefix_dec();
    unimplemented!();
    let mut uwide: () = unimplemented!();
    uwide.prefix_dec();
    uwide.postfix_inc();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
