extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let flil: Value<std_forward_list_double__std_allocator_double__> = Rc::new(RefCell::new(
        std_forward_list_double__std_allocator_double__::new_1(
            { Vec::from([1_f64, 2_f64, 3_f64, 4_f64, 5_f64]) },
            None,
        ),
    ));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test01_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
