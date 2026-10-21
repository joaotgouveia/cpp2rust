extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static counter_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
pub fn inc_1() {
    counter_0.with(|rc| *rc.borrow_mut() += 1_());
}
pub fn dec_2() {
    counter_0.with(|rc| *rc.borrow_mut() -= 1_());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let h: Value<()> = Rc::new(RefCell::new(15_()));
    let h_ref1: Ptr<()> = h.as_pointer();
    h_ref1.write(16_());
    let mut h_ptr: Ptr<()> = (h_ref1).clone();
    let h_ref2: Ptr<()> = (h_ptr).clone();
    h_ref2.write(17_());
    unimplemented!();
    let a: Value<()> = Rc::new(RefCell::new(1_()));
    let b: Value<()> = Rc::new(RefCell::new(2_()));
    let r: Ptr<()> = if ((*a.borrow()) < (*b.borrow())) {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    };
    r.write(10_());
    unimplemented!();
    let cr: Ptr<()> = if ((*a.borrow()) > (*b.borrow())) {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    };
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(1_()));
    let y: Value<()> = Rc::new(RefCell::new(2_()));
    let cx: Ptr<()> = if ((*x.borrow()) < (*y.borrow())) {
        (x.as_pointer())
    } else {
        (y.as_pointer())
    };
    unimplemented!();
    let mut cp: Ptr<()> = if ((*a.borrow()) > (*b.borrow())) {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    };
    unimplemented!();
    let mut mp: Ptr<()> = if ((*a.borrow()) < (*b.borrow())) {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    };
    mp.write(20_());
    unimplemented!();
    if ((*a.borrow()) < (*b.borrow())) {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    }
    .write(30_());
    unimplemented!();
    unimplemented!();
    {
        if ((*a.borrow()) < (*b.borrow())) {
            (a.as_pointer())
        } else {
            (b.as_pointer())
        }
        .with_mut(|__v| *__v = *__v + 5_())
    };
    unimplemented!();
    let mut ap: Ptr<()> = (if ((*a.borrow()) > (*b.borrow())) {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    });
    ap.write(40_());
    unimplemented!();
    if ((*a.borrow()) < (*b.borrow())) {
        ({ inc_1() });
    } else {
        ({ dec_2() });
    };
    unimplemented!();
    if ((*a.borrow()) > (*b.borrow())) {
        ({ inc_1() });
    } else {
        ({ dec_2() });
    };
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {
    let _ = counter_0.with(|_| ());
}
