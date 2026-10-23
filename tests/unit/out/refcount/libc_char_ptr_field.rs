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
    let mut pw: Ptr<passwd> = unimplemented!();
    if !(!(pw).is_null()) {
        return 0_();
    }
    let mut home: Ptr<()> = pw.with(|__s| __s.pw_dir.clone());
    let mut d: Ptr<dirent> = libc::readdir(libc::opendir(Ptr::<()>::from_string_literal(b"/tmp")));
    let mut dname: Ptr<()> = (array_field_ptr!(d, d_name) as Ptr<()>);
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
