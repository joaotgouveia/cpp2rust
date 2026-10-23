extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn second_of_0(p: Ptr<()>) -> () {
    return (({ get_1((p).clone()) }).read());
}
pub fn first_of_2(p: Ptr<()>) -> () {
    return (({ get_3((p).clone()) }).read());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let p: Value<()> = Rc::new(RefCell::new({
        let __tmp_0: Value<()> = Rc::new(RefCell::new(1.5_f64));
        let __tmp_1: Value<()> = Rc::new(RefCell::new(2_()));
        std_pair_double__int_::new({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
    }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
