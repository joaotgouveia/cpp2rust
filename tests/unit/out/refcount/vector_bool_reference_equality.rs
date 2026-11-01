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
fn main_0() -> i32 {
    let v1: Value<Vec<bool>> = Rc::new(RefCell::new(
        (0..(2_usize) as usize)
            .map(|_| false)
            .collect::<Vec<bool>>(),
    ));
    let v2: Value<Vec<bool>> = Rc::new(RefCell::new(
        (0..(2_usize) as usize).map(|_| true).collect::<Vec<bool>>(),
    ));
    assert!(
        v1.decay().offset(0_usize as isize).read() == v1.decay().offset(1_usize as isize).read()
    );
    assert!(
        v2.decay().offset(0_usize as isize).read() == v2.decay().offset(1_usize as isize).read()
    );
    assert!(
        !(v1.decay().offset(0_usize as isize).read() == v2.decay().offset(0_usize as isize).read())
    );
    assert!(
        !(v1.decay().offset(1_usize as isize).read() == v2.decay().offset(1_usize as isize).read())
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
