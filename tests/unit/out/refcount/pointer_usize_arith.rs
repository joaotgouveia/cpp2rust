extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        10_(),
        11_(),
        12_(),
        13_(),
        14_(),
        15_(),
        16_(),
        17_(),
    ])));
    let mut p: Ptr<()> = (arr.as_pointer() as Ptr<()>);
    let mut q: Ptr<()> = p.offset((1_()) as isize);
    unimplemented!();
    let mut r: Ptr<()> = p.offset((3_()) as isize);
    unimplemented!();
    let mut s: Ptr<()> = r.offset(-((2_()) as isize));
    unimplemented!();
    let mut diff: () = (((r).clone() - (p).clone()) as () as ());
    unimplemented!();
    let mut idx: () = (((r).clone() - (p).clone()) as ());
    unimplemented!();
    let mut q2: Ptr<()> = (p).clone();
    q2.prefix_inc();
    unimplemented!();
    q2.postfix_inc();
    unimplemented!();
    q2.prefix_dec();
    unimplemented!();
    q2.postfix_dec();
    unimplemented!();
    unimplemented!();
    let mut q3: Ptr<()> = (p).clone();
    q3 += 4_();
    unimplemented!();
    q3 -= 2_();
    unimplemented!();
    let mut step: () = 2_();
    let mut q4: Ptr<()> = p.offset((step) as isize);
    unimplemented!();
    let mut v: () = (elem!(p, 3_()).read());
    unimplemented!();
    let mut v2: () = ((p.offset((4_()) as isize)).read());
    unimplemented!();
    (p.offset((5_()) as isize)).write(99_());
    unimplemented!();
    unimplemented!();
    let mut end: Ptr<()> = p.offset((8_()) as isize);
    let mut sum: () = 0_();
    let mut it: Ptr<()> = (p).clone();
    'loop_: while ({ (it).clone() } != { (end).clone() }) {
        sum += { (it.read()) };
        it.prefix_inc();
    }
    unimplemented!();
    let bytes: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
        5_(),
        6_(),
        7_(),
    ])));
    let mut bp: Ptr<()> = (bytes.as_pointer() as Ptr<()>);
    let mut bq: Ptr<()> = bp.offset((4_()) as isize);
    unimplemented!();
    let mut bdiff: () = (((bq).clone() - (bp).clone()) as () as ());
    unimplemented!();
    let mut cp: Ptr<()> = (arr.as_pointer() as Ptr<()>);
    let mut cq: Ptr<()> = cp.offset((2_()) as isize);
    unimplemented!();
    let mut cdiff: () = (((cq).clone() - (cp).clone()) as () as ());
    unimplemented!();
    let mut n: () = 3_();
    let mut q5: Ptr<()> = (arr.as_pointer() as Ptr<()>).offset((n) as isize);
    unimplemented!();
    let mut q6: Ptr<()> = ((arr.as_pointer() as Ptr<()>).offset(n));
    unimplemented!();
    let matrix: Value<Box<[Box<[()]>]>> = Rc::new(RefCell::new(Box::new([
        Box::new([0_(), 1_(), 2_(), 3_()]),
        Box::new([4_(), 5_(), 6_(), 7_()]),
        Box::new([8_(), 9_(), 10_(), 11_()]),
    ])));
    let mut row1: Ptr<()> = ((((matrix.as_pointer() as Ptr<Box<[()]>>)
        .offset(1_())
        .as_pointer()) as Ptr<()>)
        .offset(0_()));
    unimplemented!();
    let mut back: Ptr<()> = end.offset(-((1_()) as isize));
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
