extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo1_0() {
    let v1: Value<__gnu_cxx_throw_value_limit> =
        Rc::new(RefCell::new(<__gnu_cxx_throw_value_limit>::default()));
    let v2: Value<__gnu_cxx_throw_value_limit> = Rc::new(RefCell::new((*v1.borrow()).clone()));
    let v3: Value<__gnu_cxx_throw_value_limit> =
        Rc::new(RefCell::new(<__gnu_cxx_throw_value_limit>::default()));
}
pub fn foo2_1() -> () {
    let mut b: () = value_2.with(|rc| *rc.borrow());
    return b;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ foo1_0() });
    ({ foo2_1() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
