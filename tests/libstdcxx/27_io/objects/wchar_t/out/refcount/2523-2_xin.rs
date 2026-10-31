extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test03_0() {
    unimplemented!();
    let mut i: i32 = 0_i32;
    unimplemented!();
    {
        let __s = {
            let __s = unimplemented!();
            write_int_refcount(&__s, i);
            __s
        };
        libcc2rs::endl_refcount.call(__s)
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test03_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
