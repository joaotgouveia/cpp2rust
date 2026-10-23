extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_ipv4_literal_0() {
    let hints: Value<addrinfo> = <Value<addrinfo>>::default();
    (*hints.borrow_mut()).ai_flags = 0_();
    (*hints.borrow_mut()).ai_protocol = 0_();
    (*hints.borrow_mut()).ai_addrlen = (0 as libc::socklen_t);
    (*hints.borrow_mut()).ai_addr = Ptr::<sockaddr>::null();
    (*hints.borrow_mut()).ai_canonname = Ptr::<()>::null();
    (*hints.borrow_mut()).ai_next = Ptr::<addrinfo>::null();
    (*hints.borrow_mut()).ai_family = libc::AF_INET;
    (*hints.borrow_mut()).ai_socktype = libc::SOCK_STREAM;
    let res: Value<Ptr<addrinfo>> = Rc::new(RefCell::new(Ptr::<addrinfo>::null()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut sin: Ptr<sockaddr_in> = (*res.borrow())
        .with(|__s| __s.ai_addr.clone())
        .reinterpret_cast::<sockaddr_in>();
    unimplemented!();
    let port_be: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        (8080_() / 256_()),
        (8080_() % 256_()),
    ])));
    unimplemented!();
    let addr_be: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([127_(), 0_(), 0_(), 1_()])));
    unimplemented!();
    unimplemented!();
}
pub fn test_ipv6_literal_1() {
    let hints: Value<addrinfo> = <Value<addrinfo>>::default();
    (*hints.borrow_mut()).ai_flags = 0_();
    (*hints.borrow_mut()).ai_protocol = 0_();
    (*hints.borrow_mut()).ai_addrlen = (0 as libc::socklen_t);
    (*hints.borrow_mut()).ai_addr = Ptr::<sockaddr>::null();
    (*hints.borrow_mut()).ai_canonname = Ptr::<()>::null();
    (*hints.borrow_mut()).ai_next = Ptr::<addrinfo>::null();
    (*hints.borrow_mut()).ai_family = libc::AF_INET6;
    (*hints.borrow_mut()).ai_socktype = libc::SOCK_STREAM;
    let res: Value<Ptr<addrinfo>> = Rc::new(RefCell::new(Ptr::<addrinfo>::null()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut sin6: Ptr<sockaddr_in6> = (*res.borrow())
        .with(|__s| __s.ai_addr.clone())
        .reinterpret_cast::<sockaddr_in6>();
    unimplemented!();
    let port_be: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        (443_() / 256_()),
        (443_() % 256_()),
    ])));
    unimplemented!();
    let addr_be: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
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
    unimplemented!();
    unimplemented!();
}
pub fn test_null_hints_2() {
    let res: Value<Ptr<addrinfo>> = Rc::new(RefCell::new(Ptr::<addrinfo>::null()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut sin: Ptr<sockaddr_in> = (*res.borrow())
        .with(|__s| __s.ai_addr.clone())
        .reinterpret_cast::<sockaddr_in>();
    let addr_be: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([127_(), 0_(), 0_(), 1_()])));
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
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
