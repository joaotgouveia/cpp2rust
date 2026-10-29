extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct Triple {
    #[offset(0)]
    pub a: (),
    #[offset(4)]
    pub b: (),
    #[offset(8)]
    pub c: (),
}
pub fn sum_0(t: Ptr<Triple>) -> () {
    let __decomp_1: Ptr<Triple> = (t).clone();
    return ((__decomp_1.with(|__s| __s.a)
        + (if __decomp_1.with(|__s| __s.b) {
            1_()
        } else {
            0_()
        }))
        + __decomp_1.with(|__s| __s.c));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let t: Value<Triple> = Rc::new(RefCell::new(Triple {
        a: 10_(),
        b: false,
        c: 20_(),
    }));
    let __decomp_2: Ptr<Triple> = t.as_pointer();
    field!(__decomp_2, a).write(11_());
    field!(__decomp_2, b).write(true);
    {
        field!(__decomp_2, c).with_mut(|__v| *__v = *__v + 1_())
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*t.borrow_mut()).a = 12_();
    unimplemented!();
    let __decomp_3: Ptr<Triple> = t.as_pointer();
    (*t.borrow_mut()).c = 30_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
