extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn cmp_eq_0(mut rc: ()) -> () {
    return ((rc == -1_()) as ());
}
pub fn cmp_or_ptr_1(mut p: Ptr<()>, mut q: Ptr<()>) -> () {
    return (((!(p).is_null()) || (!(q).is_null())) as ());
}
pub fn both_null_2(mut s1: Ptr<()>, mut s2: Ptr<()>) -> () {
    return ((((((s1).is_null()) as ()) != 0) && ((((s2).is_null()) as ()) != 0)) as ());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut a: () = unimplemented!();
    let mut b: () = unimplemented!();
    if ({
        b = a;
        b
    } != 0)
    {}
    'loop_: while (((({
        b = a;
        b
    }) != unimplemented!()) as ())
        != 0)
    {}
    if (a != 0) {}
    if (((a == b) as ()) != 0) {}
    if (((a < b) as ()) != 0) {}
    unimplemented!();
    unimplemented!();
    let mut c: () = unimplemented!();
    c = ({
        a = b;
        a
    } != 0);
    c = (((({
        b = a;
        b
    }) != unimplemented!()) as ())
        != 0);
    c = (a != 0);
    c = (((a == b) as ()) != 0);
    c = (((a < b) as ()) != 0);
    let mut x: () = 5_();
    let mut y: () = 5_();
    let mut eq: () = ((x == y) as ());
    let mut lt: () = ((x < y) as ());
    let mut neq: () = ((x != y) as ());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p1: Ptr<()> = Ptr::<()>::from_string_literal(b"hi");
    let mut p2: Ptr<()> = Ptr::<()>::null();
    let mut either: () = (((!(p1).is_null()) || (!(p2).is_null())) as ());
    let mut both: () = (((!(p1).is_null()) && (!(p2).is_null())) as ());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
