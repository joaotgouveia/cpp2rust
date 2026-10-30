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
fn main_0() -> () {
    let mut p: () = unimplemented!();
    {
        unimplemented!().with_mut(|__v| *__v = *__v + 5_())
    };
    {
        unimplemented!().with_mut(|__v| *__v = *__v - 3_())
    };
    {
        unimplemented!().with_mut(|__v| *__v = *__v * 2_())
    };
    let mut q: () = unimplemented!();
    let mut sum: () = ((unimplemented!().read()) + (unimplemented!().read()));
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
