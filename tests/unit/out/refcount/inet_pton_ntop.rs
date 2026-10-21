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
    let addr: Value<::libc::in6_addr> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
    let mut ap: Ptr<::libc::in6_addr> = (addr.as_pointer());
    ({
        memcpy_1(
            ((&raw mut (*addr.as_pointer()).s6_addr as Ptr<u8>) as Ptr<u8>).to_any(),
            ((loopback.as_pointer() as Ptr<()>) as Ptr<()>).to_any(),
            16_(),
        )
    });
    unimplemented!();
    elem!((&raw mut (*addr.as_pointer()).s6_addr as Ptr::<u8>), 0_()).write(32_u8);
    elem!((&raw mut (*ap).s6_addr as Ptr::<u8>), 1_()).write(1_u8);
    unimplemented!();
    elem!((&raw mut (*addr.as_pointer()).s6_addr as Ptr::<u8>), 15_()).write({
        (((elem!((&raw mut (*addr.as_pointer()).s6_addr as Ptr::<u8>), 15_()).read()) as ()) + 4_())
            as u8
    });
    elem!((&raw mut (*ap).s6_addr as Ptr::<u8>), 15_()).with_mut(|__v| __v.postfix_inc());
    unimplemented!();
    let mut bytes: Ptr<()> =
        (&raw mut (*addr.as_pointer()).s6_addr as Ptr<u8>).reinterpret_cast::<()>();
    elem!(bytes, 2_()).write(7_());
    unimplemented!();
    let mut fourth: Ptr<()> =
        ((&raw mut (*ap).s6_addr as Ptr<u8>).offset((3_()) as isize)).reinterpret_cast::<()>();
    fourth.write(8_());
    unimplemented!();
    ({
        memcpy_1(
            ((&raw mut (*ap).s6_addr as Ptr<u8>) as Ptr<u8>).to_any(),
            ((loopback.as_pointer() as Ptr<()>) as Ptr<()>).to_any(),
            16_(),
        )
    });
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
