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
    let fds: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..2).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    unimplemented!();
    unimplemented!();
    let pfd: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..2).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    (*pfd.borrow_mut())[(0_()) as usize].fd = (*fds.borrow())[(0_()) as usize];
    (*pfd.borrow_mut())[(0_()) as usize].events = 1_();
    (*pfd.borrow_mut())[(0_()) as usize].revents = 0_();
    (*pfd.borrow_mut())[(1_()) as usize].fd = -1_();
    (*pfd.borrow_mut())[(1_()) as usize].events = 1_();
    (*pfd.borrow_mut())[(1_()) as usize].revents = 42_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let ch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
