extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0() -> () {
    return true;
}
pub fn bar_1() -> () {
    return true;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut i1: () = 0_();
    let mut i2: () = 1_();
    write!(cout_2.with(|v| v.as_pointer()), "{:}\n", (true as u8),);
    write!(cout_2.with(|v| v.as_pointer()), "{:}\n", (false as u8),);
    write!(cout_2.with(|v| v.as_pointer()), "{:}\n", ((i1 != i2) as u8),);
    write!(cout_2.with(|v| v.as_pointer()), "{:}\n", ((i1 == i2) as u8),);
    write!(
        cout_2.with(|v| v.as_pointer()),
        "{:}\n",
        (({ foo_0() }) as u8),
    );
    write!(
        cout_2.with(|v| v.as_pointer()),
        "{:}\n",
        (({ bar_1() }) as u8),
    );
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
