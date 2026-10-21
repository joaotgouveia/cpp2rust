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
    let mut n: () = 3_();
    let mut zero: () = unimplemented!();
    let mut u: () = 4_();
    let mut ul: () = 5_();
    let mut ll: () = 6_();
    let mut ch: () = ('a' as ());
    if (n != 0) {
        unimplemented!();
    }
    if !(n != 0) {
        unimplemented!();
    }
    if (zero != 0) {
        unimplemented!();
    }
    if !(zero != 0) {
        unimplemented!();
    }
    if (u != 0) {
        unimplemented!();
    }
    if (ul != 0) {
        unimplemented!();
    }
    if (ll != 0) {
        unimplemented!();
    }
    if (ch != 0) {
        unimplemented!();
    }
    let mut loop_count: () = unimplemented!();
    let mut counter: () = 3_();
    'loop_: while (counter != 0) {
        counter.prefix_dec();
        loop_count.prefix_inc();
    }
    unimplemented!();
    let mut i: () = 5_();
    'loop_: while (i != 0) {
        loop_count.prefix_inc();
        i.prefix_dec();
    }
    unimplemented!();
    let mut t: () = if (n != 0) { 100_() } else { 200_() };
    unimplemented!();
    let mut t2: () = if (zero != 0) { 100_() } else { 200_() };
    unimplemented!();
    let mut t7: () = (!(n != 0) as ());
    unimplemented!();
    let mut t8: () = (!(zero != 0) as ());
    unimplemented!();
    let mut b1: () = (n != 0);
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
