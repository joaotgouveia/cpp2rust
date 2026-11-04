extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test10_0() {
    write!(cout_1.with(|v| v.as_pointer()), "Press ENTER once\n",);
    ({ (*cin_2.with(Value::clone).borrow()).ignore_30(1_i64) });
    write!(
        cout_1.with(|v| v.as_pointer()),
        "_M_gcount: {:}\n",
        ({ (*cin_2.with(Value::clone).borrow()).gcount() }),
    );
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test10_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
