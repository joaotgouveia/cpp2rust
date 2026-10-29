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
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..32).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*buf.borrow_mut())[(0_()) as usize] = ('Z' as ());
    unimplemented!();
    unimplemented!();
    let fmt: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..8).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    (*fmt.borrow_mut())[(0_()) as usize] = ('%' as ());
    (*fmt.borrow_mut())[(1_()) as usize] = ('5' as ());
    (*fmt.borrow_mut())[(2_()) as usize] = ('.' as ());
    (*fmt.borrow_mut())[(3_()) as usize] = ('1' as ());
    (*fmt.borrow_mut())[(4_()) as usize] = ('f' as ());
    (*fmt.borrow_mut())[(5_()) as usize] = 0_();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
