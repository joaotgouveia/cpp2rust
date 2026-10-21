extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn cmp_eq_0(mut rc: ()) -> () {
    return (rc == -1_());
}
pub fn cmp_or_ptr_1(mut p: Ptr<()>, mut q: Ptr<()>) -> () {
    return (!(p).is_null()) || (!(q).is_null());
}
pub fn both_null_2(mut s1: Ptr<()>, mut s2: Ptr<()>) -> () {
    return ((s1).is_null()) && ((s2).is_null());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut a: () = 0_();
    let mut b: () = unimplemented!();
    if ({
        b = a;
        b
    } != 0)
    {}
    'loop_: while (({
        b = a;
        b
    }) != 0_())
    {}
    if (a != 0) {}
    if (a == b) {}
    if (a < b) {}
    unimplemented!();
    unimplemented!();
    let mut c: () = unimplemented!();
    c = ({
        a = b;
        a
    } != 0);
    c = (({
        b = a;
        b
    }) != 0_());
    c = (a != 0);
    c = (a == b);
    c = (a < b);
    let mut x: () = 5_();
    let mut y: () = 5_();
    let mut eq: () = (x == y);
    let mut lt: () = (x < y);
    let mut neq: () = (x != y);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p1: Ptr<()> = Ptr::<()>::from_string_literal(b"hi");
    let mut p2: Ptr<()> = Ptr::<()>::null();
    let mut either: () = (!(p1).is_null()) || (!(p2).is_null());
    let mut both: () = (!(p1).is_null()) && (!(p2).is_null());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut d: () = 0.5_f64;
    let mut taken: () = 0_();
    if (d != 0.0) {
        taken = 1_();
    }
    unimplemented!();
    'loop_: while (d != 0.0) {
        d = 0_f64;
    }
    unimplemented!();
    let mut f: () = 2_f32;
    let mut fb: () = (f != 0.0);
    let mut nf: () = !(f != 0.0);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut fi: () = (f != 0.0);
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
