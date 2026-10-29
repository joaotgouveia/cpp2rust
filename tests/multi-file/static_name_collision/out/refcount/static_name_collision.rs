extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static same_name_different_type_0: Value<()> = Rc::new(RefCell::new(1_()));
);
thread_local!(
    pub static same_name_same_type_1: Value<()> = Rc::new(RefCell::new(5_()));
);
pub fn a_foo_2() -> () {
    return same_name_different_type_0.with(|rc| *rc.borrow());
}
pub fn a_bar_3() -> () {
    return same_name_same_type_1.with(|rc| *rc.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
thread_local!(
    pub static same_name_different_type_4: Value<()> = Rc::new(RefCell::new(1_f32));
);
thread_local!(
    pub static same_name_same_type_5: Value<()> = Rc::new(RefCell::new(6_()));
);
pub fn b_foo_6() -> () {
    return same_name_different_type_4.with(|rc| *rc.borrow());
}
pub fn b_bar_7() -> () {
    return same_name_same_type_5.with(|rc| *rc.borrow());
}
pub fn __cpp2rust_init_globals() {
    let _ = same_name_different_type_0.with(|_| ());
    let _ = same_name_same_type_1.with(|_| ());
    let _ = same_name_different_type_4.with(|_| ());
    let _ = same_name_same_type_5.with(|_| ());
}
