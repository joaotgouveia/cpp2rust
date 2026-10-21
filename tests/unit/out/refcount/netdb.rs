extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_ipv4_literal_0() {
    let hints: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*hints.borrow_mut()).ai_flags = unimplemented!();
    (*hints.borrow_mut()).ai_protocol = unimplemented!();
    (*hints.borrow_mut()).ai_addrlen = unimplemented!();
    (*hints.borrow_mut()).ai_addr = Ptr::<()>::null();
    (*hints.borrow_mut()).ai_canonname = Ptr::<()>::null();
    (*hints.borrow_mut()).ai_next = Ptr::<()>::null();
    (*hints.borrow_mut()).ai_family = unimplemented!();
    (*hints.borrow_mut()).ai_socktype = unimplemented!();
    let res: Value<Ptr<()>> = Rc::new(RefCell::new(Ptr::<()>::null()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut sin: Ptr<()> = (*res.borrow())
        .with(|__s| __s.ai_addr.clone())
        .reinterpret_cast::<()>();
    unimplemented!();
    let port_be: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        (8080_() / 256_()),
        (8080_() % 256_()),
    ])));
    unimplemented!();
    let addr_be: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        127_(),
        unimplemented!(),
        unimplemented!(),
        1_(),
    ])));
    unimplemented!();
    unimplemented!();
}
pub fn test_ipv6_literal_1() {
    let hints: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*hints.borrow_mut()).ai_flags = unimplemented!();
    (*hints.borrow_mut()).ai_protocol = unimplemented!();
    (*hints.borrow_mut()).ai_addrlen = unimplemented!();
    (*hints.borrow_mut()).ai_addr = Ptr::<()>::null();
    (*hints.borrow_mut()).ai_canonname = Ptr::<()>::null();
    (*hints.borrow_mut()).ai_next = Ptr::<()>::null();
    (*hints.borrow_mut()).ai_family = unimplemented!();
    (*hints.borrow_mut()).ai_socktype = unimplemented!();
    let res: Value<Ptr<()>> = Rc::new(RefCell::new(Ptr::<()>::null()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut sin6: Ptr<()> = (*res.borrow())
        .with(|__s| __s.ai_addr.clone())
        .reinterpret_cast::<()>();
    unimplemented!();
    let port_be: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        (443_() / 256_()),
        (443_() % 256_()),
    ])));
    unimplemented!();
    let addr_be: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
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
        1_(),
    ])));
    unimplemented!();
    unimplemented!();
}
pub fn test_null_hints_2() {
    let res: Value<Ptr<()>> = Rc::new(RefCell::new(Ptr::<()>::null()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut sin: Ptr<()> = (*res.borrow())
        .with(|__s| __s.ai_addr.clone())
        .reinterpret_cast::<()>();
    let addr_be: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        127_(),
        unimplemented!(),
        unimplemented!(),
        1_(),
    ])));
    unimplemented!();
    unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test_ipv4_literal_0() });
    ({ test_ipv6_literal_1() });
    ({ test_null_hints_2() });
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
