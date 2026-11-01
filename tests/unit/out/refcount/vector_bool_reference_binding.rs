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
    let bits: Value<Vec<bool>> = Rc::new(RefCell::new(
        (0..(2_usize) as usize)
            .map(|_| false)
            .collect::<Vec<bool>>(),
    ));
    let mut bit: Ptr<bool> = bits.decay().offset(0_usize as isize);
    let assigned: Ptr<bool> = ({
        let __a0 = (bit).clone();
        __a0.write(true);
        __a0
    });
    assert!(assigned.read());
    assert!(bits.decay().offset(0_usize as isize).read());
    let cleared: Ptr<bool> = {
        let __a0 = (bit).clone();
        __a0.write(false);
        __a0
    };
    assert!(!(cleared.read()));
    assert!(!(bits.decay().offset(0_usize as isize).read()));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
