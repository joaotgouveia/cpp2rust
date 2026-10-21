extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub first: (),
    #[offset(4)]
    pub second: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let arr: Value<Box<[Pair]>> = Rc::new(RefCell::new(Box::new([
        Pair {
            first: 1_(),
            second: 2_(),
        },
        Pair {
            first: 3_(),
            second: 4_(),
        },
        Pair {
            first: 5_(),
            second: 6_(),
        },
    ])));
    'loop_: for __decomp_0 in IterRange::new(
        (arr.as_pointer() as Ptr<Pair>),
        (arr.as_pointer() as Ptr<Pair>).offset((3_()) as isize),
    ) {
        let __decomp_0: Ptr<Pair> = (__decomp_0).clone();
        {
            let __rhs = __decomp_0.with(|__s| __s.second);
            field!(__decomp_0, first).with_mut(|__v| *__v = *__v + __rhs)
        };
    }
    unimplemented!();
    unimplemented!();
    'loop_: for __decomp_1 in IterRange::new(
        (arr.as_pointer() as Ptr<Pair>),
        (arr.as_pointer() as Ptr<Pair>).offset((3_()) as isize),
    ) {
        let __decomp_1: Value<Pair> =
            Rc::new(RefCell::new((*__decomp_1.upgrade().deref()).clone()));
        (*__decomp_1.borrow_mut()).first = 0_();
        (*__decomp_1.borrow_mut()).second = 0_();
    }
    unimplemented!();
    unimplemented!();
    let mut sum: () = 0_();
    'loop_: for __decomp_2 in IterRange::new(
        (arr.as_pointer() as Ptr<Pair>),
        (arr.as_pointer() as Ptr<Pair>).offset((3_()) as isize),
    ) {
        let __decomp_2: Ptr<Pair> = (__decomp_2).clone();
        sum += (__decomp_2.with(|__s| __s.first) * __decomp_2.with(|__s| __s.second));
    }
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
