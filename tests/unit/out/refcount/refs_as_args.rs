extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn more_refs_0(x1: (), x2: (), r1: Ptr<()>, r2: Ptr<()>) {
    let x1: Value<()> = Rc::new(RefCell::new(x1));
    let x2: Value<()> = Rc::new(RefCell::new(x2));
    let rx1: Ptr<()> = x1.as_pointer();
    let rx2: Ptr<()> = x2.as_pointer();
    let mut pr1: Ptr<()> = (r1).clone();
    let mut pr2: Ptr<()> = (r2).clone();
    let rpr1: Ptr<()> = (pr1).clone();
    let rpr2: Ptr<()> = (pr2).clone();
    let r: Ptr<()> = (r1).clone();
    {
        let __rhs = {
            ({
                ({
                    ({
                        ({
                            ({ ({ (1_() + (rx1.read())) } + { (rx2.read()) }) } + { (pr1.read()) })
                        } + { (pr2.read()) })
                    } + { (rpr1.read()) })
                } + { (rpr2.read()) })
            } + { (r.read()) })
        };
        rx2.with_mut(|__v| *__v = *__v + __rhs)
    };
    r1.write({ (rx2.read()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Val {
    #[offset(0)]
    pub x: (),
}
pub fn sum_1(mut a: Val, mut b: Val) -> () {
    return (a.x + b.x);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let x1: Value<()> = Rc::new(RefCell::new(1_()));
    let x2: Value<()> = Rc::new(RefCell::new(2_()));
    ({ more_refs_0(3_(), 4_(), x1.as_pointer(), x2.as_pointer()) });
    unimplemented!();
    let v: Value<Val> = Rc::new(RefCell::new(Val { x: 5_() }));
    let mut acc: () = ({
        let _a: Val = (*v.borrow()).clone();
        let _b: Val = (*v.borrow()).clone();
        sum_1(_a, _b)
    });
    acc += ({
        let _a: Val = (*v.borrow()).clone();
        let _b: Val = (*v.borrow()).clone();
        sum_1(_a, _b)
    });
    acc += ({
        let _a: Val = (*v.borrow()).clone();
        let _b: Val = (*v.borrow()).clone();
        sum_1(_a, _b)
    });
    acc += ({
        let _a: Val = (*v.borrow()).clone();
        let _b: Val = (*v.borrow()).clone();
        sum_1(_a, _b)
    });
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
