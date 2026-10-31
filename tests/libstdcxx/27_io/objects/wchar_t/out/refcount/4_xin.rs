extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test10_0() {
    unimplemented!();
    unimplemented!();
    {
        let __s = {
            let __s = unimplemented!();
            write_int_refcount(&__s, (unimplemented!() as i64));
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
    ({ test10_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
