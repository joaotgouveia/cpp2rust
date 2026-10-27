extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_reset_all_0() {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn test_reset_all_1() {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn test_reset_all_2() {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn test_reset_all_3() {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn test_reset_all_4() {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn test_reset_all_5() {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn test_reset_all_6() {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn test_reset_all_7() {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn test_reset_all_8() {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn test_9() -> () {
    ({ test_reset_all_0() });
    ({ test_reset_all_1() });
    ({ test_reset_all_2() });
    ({ test_reset_all_3() });
    ({ test_reset_all_4() });
    ({ test_reset_all_5() });
    ({ test_reset_all_6() });
    ({ test_reset_all_7() });
    ({ test_reset_all_8() });
    return true;
}
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
    ({ test_9() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
