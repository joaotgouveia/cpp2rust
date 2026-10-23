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
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        ('a' as ()),
        ('b' as ()),
        ('\0' as ()),
        ('c' as ()),
        ('d' as ()),
        ('\0' as ()),
    ])));
    libc::fputs((buf.as_pointer() as Ptr<()>), libcc2rs::stdout_unsafe());
    libc::fputc(('|' as ()), libcc2rs::stdout_unsafe());
    libc::puts((buf.as_pointer() as Ptr<()>));
    let mut s: () = unimplemented!();
    println!("{}", unimplemented!());
    println!(
        "{} {}",
        ({ strlen_0((buf.as_pointer() as Ptr::<()>),) }),
        unimplemented!()
    );
    let mut lit: Ptr<()> = Ptr::<()>::from_string_literal(b"xy\0zw");
    let mut t: () = unimplemented!();
    println!("{}", unimplemented!());
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
