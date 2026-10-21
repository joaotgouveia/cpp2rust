extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_fputc_0() {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_fputs_1() {
    unimplemented!();
    unimplemented!();
    let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"from variable");
    unimplemented!();
    unimplemented!();
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        ('b' as ()),
        ('u' as ()),
        ('f' as ()),
        ('\0' as ()),
    ])));
    unimplemented!();
    unimplemented!();
}
pub fn test_puts_2() {
    unimplemented!();
    let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"puts variable");
    unimplemented!();
}
pub fn test_fileno_3() {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut file: Ptr<()> = Ptr::<()>::from_string_literal(b"cpp2rust_fileno_test.tmp");
    let mut fp: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test_fputc_0() });
    ({ test_fputs_1() });
    ({ test_puts_2() });
    ({ test_fileno_3() });
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
