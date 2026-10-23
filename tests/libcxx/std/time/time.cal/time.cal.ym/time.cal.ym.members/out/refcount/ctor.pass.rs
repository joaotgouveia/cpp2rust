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
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let ym0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "ym0.year()  == year{{}}");;
    const _: () = assert!(true, "ym0.month() == month{{}}");;
    const _: () = assert!(true, "!ym0.ok()");;
    let ym1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "ym1.year()  == year{{2018}}");;
    const _: () = assert!(true, "ym1.month() == std::chrono::January");;
    const _: () = assert!(true, "ym1.ok()");;
    let ym2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "ym2.year()  == year{{2018}}");;
    const _: () = assert!(true, "ym2.month() == month{{}}");;
    const _: () = assert!(true, "!ym2.ok()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
