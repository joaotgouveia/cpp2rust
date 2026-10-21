extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_fputc_0() {
    libc::fputc(('H' as ()), libcc2rs::stdout_unsafe());
    libc::fputc(('i' as ()), libcc2rs::stdout_unsafe());
    libc::fputc(('\n' as ()), libcc2rs::stdout_unsafe());
}
pub fn test_fputs_1() {
    libc::fputs(
        Ptr::<()>::from_string_literal(b"hello"),
        libcc2rs::stdout_unsafe(),
    );
    libc::fputc(('\n' as ()), libcc2rs::stdout_unsafe());
    let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"from variable");
    libc::fputs(s, libcc2rs::stdout_unsafe());
    libc::fputc(('\n' as ()), libcc2rs::stdout_unsafe());
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        ('b' as ()),
        ('u' as ()),
        ('f' as ()),
        ('\0' as ()),
    ])));
    libc::fputs((buf.as_pointer() as Ptr<()>), libcc2rs::stdout_unsafe());
    libc::fputc(('\n' as ()), libcc2rs::stdout_unsafe());
}
pub fn test_puts_2() {
    libc::puts(Ptr::<()>::from_string_literal(b"puts hello"));
    let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"puts variable");
    libc::puts(s);
}
pub fn test_fileno_3() {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut file: Ptr<()> = Ptr::<()>::from_string_literal(b"cpp2rust_fileno_test.tmp");
    let mut fp: *mut ::libc::FILE = libc::fopen(file, Ptr::<()>::from_string_literal(b"wb"));
    unimplemented!();
    unimplemented!();
    libc::fclose(fp);
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
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
