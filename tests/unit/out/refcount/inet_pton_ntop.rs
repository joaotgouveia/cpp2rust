extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_s6_addr_access_0() {
    let loopback: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
    ])));
    let addr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut ap: Ptr<()> = (addr.as_pointer());
    unimplemented!();
    unimplemented!();
    elem!(
        (anon_1::__u6_addr8(unimplemented!().as_pointer()).reinterpret_cast::<()>() as Ptr::<()>),
        0_()
    )
    .write(32_());
    elem!(
        (anon_1::__u6_addr8(unimplemented!().as_pointer()).reinterpret_cast::<()>() as Ptr::<()>),
        1_()
    )
    .write(1_());
    unimplemented!();
    {
        elem!(
            (anon_1::__u6_addr8(unimplemented!().as_pointer()).reinterpret_cast::<()>()
                as Ptr::<()>),
            15_()
        )
        .with_mut(|__v| *__v = *__v + 4_())
    };
    elem!(
        (anon_1::__u6_addr8(unimplemented!().as_pointer()).reinterpret_cast::<()>() as Ptr::<()>),
        15_()
    )
    .with_mut(|__v| __v.postfix_inc());
    unimplemented!();
    let mut bytes: Ptr<()> =
        (anon_1::__u6_addr8(unimplemented!().as_pointer()).reinterpret_cast::<()>() as Ptr<()>);
    elem!(bytes, 2_()).write(7_());
    unimplemented!();
    let mut fourth: Ptr<()> = ((anon_1::__u6_addr8(unimplemented!().as_pointer())
        .reinterpret_cast::<()>() as Ptr<()>)
        .offset((3_()) as isize));
    fourth.write(8_());
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test_s6_addr_access_0() });
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..16).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let text: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..64).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    let four: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([10_(), 0_(), 0_(), 1_()])));
    unimplemented!();
    let sixteen: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    (*sixteen.borrow_mut())[(15_()) as usize] = 1_();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
