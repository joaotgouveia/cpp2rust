extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    let January: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let Tuesday: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    let mwdl1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "mwdl1.month() == month{{}}");;
    const _: () = assert!(true, "mwdl1.weekday_last() == weekday_last{{Tuesday}}");;
    const _: () = assert!(true, "!mwdl1.ok()");;
    let mwdl2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "mwdl2.month() == January");;
    const _: () = assert!(
        true,
        "mwdl2.weekday_last() == weekday_last{{weekday{{16}}}}"
    );;
    const _: () = assert!(true, "!mwdl2.ok()");;
    let mwdl3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "mwdl3.month() == January");;
    const _: () = assert!(true, "mwdl3.weekday_last() == weekday_last{{weekday{{4}}}}");;
    const _: () = assert!(true, "mwdl3.ok()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
