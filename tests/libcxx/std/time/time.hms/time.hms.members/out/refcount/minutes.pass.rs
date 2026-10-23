extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_chrono_duration_long__std_ratio_60__1____std_chrono_duration_long__std_ratio_60__1___
{}
pub fn check_minutes_1(d: ()) -> () {
    let d: Value<()> = Rc::new(RefCell::new(d));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return unimplemented!();
}
pub fn check_minutes_2(d: ()) -> () {
    let d: Value<()> = Rc::new(RefCell::new(d));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return unimplemented!();
}
pub fn check_minutes_3(d: ()) -> () {
    let d: Value<()> = Rc::new(RefCell::new(d));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return unimplemented!();
}
pub fn check_minutes_4(d: ()) -> () {
    let d: Value<()> = Rc::new(RefCell::new(d));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return unimplemented!();
}
pub fn check_minutes_5(d: ()) -> () {
    let d: Value<()> = Rc::new(RefCell::new(d));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return unimplemented!();
}
pub fn check_minutes_6(d: ()) -> () {
    let d: Value<()> = Rc::new(RefCell::new(d));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return unimplemented!();
}
pub fn check_minutes_7(d: ()) -> () {
    let d: Value<()> = Rc::new(RefCell::new(d));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return unimplemented!();
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
    const _: () = assert!(true, "check_minutes(std::chrono::minutes( 1)) == 1");;
    const _: () = assert!(true, "check_minutes(std::chrono::minutes(-1)) == 1");;
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
