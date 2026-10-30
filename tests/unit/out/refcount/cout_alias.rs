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
    let mut cond: () = true;
    let mut os1: () = if cond {
        (unimplemented!())
    } else {
        (unimplemented!())
    };
    write!(os1, "hello\n",);
    let os2: () = if cond {
        (unimplemented!())
    } else {
        (unimplemented!())
    };
    write!(os2, "hello\n",);
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
