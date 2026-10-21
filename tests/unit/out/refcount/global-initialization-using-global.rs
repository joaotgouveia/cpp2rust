extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static first_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
thread_local!(
    pub static second_1: Value<()> =
        Rc::new(RefCell::new((first_0.with(|rc| *rc.borrow()) + 1_())));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {
    let _ = first_0.with(|_| ());
    let _ = second_1.with(|_| ());
}
