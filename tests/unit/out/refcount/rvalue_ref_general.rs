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
    let i1: Value<()> = Rc::new(RefCell::new(3_()));
    let mut i2: () = (*i1.borrow());
    unimplemented!();
    unimplemented!();
    let __tmp_0: Value<()> = Rc::new(RefCell::new(40_()));
    let i3: Ptr<()> = __tmp_0.as_pointer();
    {
        i3.with_mut(|__v| *__v = *__v + 2_())
    };
    unimplemented!();
    let __tmp_1: Value<()> = Rc::new(RefCell::new((2_() + 3_())));
    let i4: Ptr<()> = __tmp_1.as_pointer();
    unimplemented!();
    let __tmp_2: Value<()> = Rc::new(RefCell::new(40_()));
    let i5: Ptr<()> = __tmp_2.as_pointer();
    let i6: Ptr<()> = (i3).clone();
    let i7: Ptr<()> = (i4).clone();
    unimplemented!();
    unimplemented!();
    let i8: Value<()> = Rc::new(RefCell::new(3_()));
    let i9: Ptr<()> = i8.as_pointer();
    unimplemented!();
    let mut p1: Ptr<()> = (i1.as_pointer());
    let mut p2: Ptr<()> = (i3).clone();
    let mut p3: Ptr<()> = (i6).clone();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
