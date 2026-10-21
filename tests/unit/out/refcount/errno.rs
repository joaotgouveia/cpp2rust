extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_errno_0() {
    unimplemented!().write(0_());
    unimplemented!();
    unimplemented!().write(42_());
    unimplemented!();
    let mut saved: () = (unimplemented!().read());
    unimplemented!();
    unimplemented!().write(0_());
}
pub fn test_errno_preserved_across_strdup_1() {
    unimplemented!().write(99_());
    let mut d: Ptr<()> = ({ strdup_2(Ptr::<()>::from_string_literal(b"hello")) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!().write(0_());
}
pub fn test_errno_from_fseek_3() {
    unimplemented!().write(0_());
    let mut r: () = libc::fseek(
        libcc2rs::stdin_unsafe(),
        0_() as ::libc::c_long,
        ::libc::SEEK_SET,
    );
    unimplemented!();
    unimplemented!();
    unimplemented!().write(0_());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test_errno_0() });
    ({ test_errno_preserved_across_strdup_1() });
    ({ test_errno_from_fseek_3() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
