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
pub struct POD {
    #[offset(0)]
    pub x1: (),
    #[offset(4)]
    pub x2: (),
    #[offset(8)]
    pub x3: (),
}
pub fn PODIncrement_0(pod: Ptr<POD>) {
    {
        field!(pod, x1).with_mut(|__v| *__v = *__v + 1_())
    };
    {
        field!(pod, x2).with_mut(|__v| *__v = *__v + 2_())
    };
    {
        field!(pod, x3).with_mut(|__v| *__v = *__v + 3_())
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut p1: POD = POD {
        x1: 10_(),
        x2: 11_(),
        x3: 12_(),
    };
    let p2: Value<POD> = Rc::new(RefCell::new(POD {
        x1: p1.x1,
        x2: p1.x2,
        x3: p1.x3,
    }));
    ({ PODIncrement_0(p2.as_pointer()) });
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
