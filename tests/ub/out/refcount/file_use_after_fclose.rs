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
    let mut fp: *mut ::libc::FILE = libc::fopen(
        Ptr::<()>::from_string_literal(b"/tmp/cpp2rust_uafc_test.tmp"),
        Ptr::<()>::from_string_literal(b"wb"),
    );
    unimplemented!();
    libc::fclose(fp);
    return if (((libc::fputc(('x' as ()), fp) == ('x' as ())) as ()) != 0) {
        1_()
    } else {
        0_()
    };
}
pub fn __cpp2rust_init_globals() {}
