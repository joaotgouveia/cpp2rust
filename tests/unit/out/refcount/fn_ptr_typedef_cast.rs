extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn call_with_ulong_0(mut g: FnPtr<fn(()) -> ()>) -> () {
    return ({ g.call(3_()) }).wrapping_add(1_());
}
pub fn same_type_1(mut a: ()) -> () {
    return a;
}
pub fn via_size_t_param_2(mut b: ()) -> () {
    return b;
}
pub fn via_size_t_return_3(mut b: ()) -> () {
    return b;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct pair {
    #[offset(0)]
    pub a: (),
    #[offset(4)]
    pub b: (),
}
pub fn pair_scaled_4(mut p: pair, mut n: ()) -> () {
    return ((p.a).wrapping_mul(n)).wrapping_add(p.b);
}
pub fn make_pair_5(mut n: ()) -> pair {
    let mut p: pair = pair {
        a: (n as ()),
        b: ((n as ()) * 2_()),
    };
    return p;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut original: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(via_size_t_param_2);
    let mut adapted: FnPtr<fn(()) -> ()> = original.cast::<fn(()) -> ()>();
    let mut back: FnPtr<fn(()) -> ()> = adapted.cast::<fn(()) -> ()>();
    unimplemented!();
    unimplemented!();
    let mut scaled: FnPtr<fn(pair, ()) -> ()> =
        FnPtr::<fn(pair, ()) -> ()>::new(pair_scaled_4).cast::<fn(pair, ()) -> ()>();
    let mut p: pair = pair { a: 3_(), b: 4_() };
    unimplemented!();
    let mut make: FnPtr<fn(()) -> pair> =
        FnPtr::<fn(()) -> pair>::new(make_pair_5).cast::<fn(()) -> pair>();
    let mut q: pair = ({ make.call(5_()) });
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
