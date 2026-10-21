extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static value_0: Value<()> = Rc::new(RefCell::new(5_()));
);
pub fn param_shadow_1(mut value: ()) -> () {
    return (value + 1_());
}
pub fn local_shadow_2() -> () {
    let mut value: () = 99_();
    return value;
}
pub fn read_global_3() -> () {
    return value_0.with(|rc| *rc.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = value_0.with(|_| ());
}
