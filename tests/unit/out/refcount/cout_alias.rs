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
    let mut os1: Ptr<std_basic_ostream_char__std_char_traits_char__> = if cond {
        (cout_0.with(|v| v.as_pointer()))
    } else {
        (cerr_1.with(|v| v.as_pointer()))
    };
    write!(os1, "hello\n",);
    let os2: Ptr<std_basic_ostream_char__std_char_traits_char__> = if cond {
        (cout_0.with(|v| v.as_pointer()))
    } else {
        (cerr_1.with(|v| v.as_pointer()))
    };
    write!(os2, "hello\n",);
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
