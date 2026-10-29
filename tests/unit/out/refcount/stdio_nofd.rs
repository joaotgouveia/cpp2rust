extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_fputc_fputs_0() {
    let mut path: Ptr<()> = Ptr::<()>::from_string_literal(b"cpp2rust_stdio_nofd_puts.tmp");
    let mut fp: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    fp = unimplemented!();
    unimplemented!();
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_puts_1() {
    unimplemented!();
}
pub fn test_fgets_getc_2() {
    let mut path: Ptr<()> = Ptr::<()>::from_string_literal(b"cpp2rust_stdio_nofd_gets.tmp");
    let mut fp: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    fp = unimplemented!();
    unimplemented!();
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..8).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_freopen_3() {
    let mut path: Ptr<()> = Ptr::<()>::from_string_literal(b"cpp2rust_stdio_nofd_reopen.tmp");
    let mut fp: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut fp2: () = unimplemented!();
    unimplemented!();
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_fseeko_4() {
    let mut path: Ptr<()> = Ptr::<()>::from_string_literal(b"cpp2rust_stdio_nofd_seek.tmp");
    let mut fp: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    fp = unimplemented!();
    unimplemented!();
    unimplemented!();
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_rename_5() {
    let mut from: Ptr<()> = Ptr::<()>::from_string_literal(b"cpp2rust_stdio_nofd_from.tmp");
    let mut to: Ptr<()> = Ptr::<()>::from_string_literal(b"cpp2rust_stdio_nofd_to.tmp");
    let mut fp: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    fp = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_setvbuf_6() {
    let mut path: Ptr<()> = Ptr::<()>::from_string_literal(b"cpp2rust_stdio_nofd_vbuf.tmp");
    let mut fp: () = unimplemented!();
    unimplemented!();
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
    ({ test_fputc_fputs_0() });
    ({ test_puts_1() });
    ({ test_fgets_getc_2() });
    ({ test_freopen_3() });
    ({ test_fseeko_4() });
    ({ test_rename_5() });
    ({ test_setvbuf_6() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
