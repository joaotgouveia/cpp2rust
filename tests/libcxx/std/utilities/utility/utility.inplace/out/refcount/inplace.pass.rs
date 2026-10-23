extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn check_tag_0(_a0: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return (value_1.with(|rc| *rc.borrow())) && (value_1.with(|rc| *rc.borrow()));
}
pub fn check_tag_2(_a0: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return (value_1.with(|rc| *rc.borrow())) && (value_1.with(|rc| *rc.borrow()));
}
pub fn check_tag_3(_a0: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return (value_1.with(|rc| *rc.borrow())) && (value_1.with(|rc| *rc.borrow()));
}
pub fn check_tag_4(_a0: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return (value_1.with(|rc| *rc.borrow())) && (value_1.with(|rc| *rc.borrow()));
}
pub fn check_tag_5(_a0: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return (value_1.with(|rc| *rc.borrow())) && (value_1.with(|rc| *rc.borrow()));
}
pub fn check_tag_6(_a0: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return (value_1.with(|rc| *rc.borrow())) && (value_1.with(|rc| *rc.borrow()));
}
pub fn check_tag_7(_a0: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return (value_1.with(|rc| *rc.borrow())) && (value_1.with(|rc| *rc.borrow()));
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
    {
        const _: () = assert!(true, "check_tag<T>(std::in_place)");;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_same<T1, T2>::value && !std::is_same<T1, T3>::value"
        );;
        const _: () = assert!(true, "!std::is_same<T2, T3>::value");;
        const _: () = assert!(true, "check_tag<T1>(std::in_place_type<void>)");;
        const _: () = assert!(true, "check_tag<T2>(std::in_place_type<int>)");;
        const _: () = assert!(true, "check_tag<T3>(std::in_place_type<const int>)");;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_same<T1, T2>::value && !std::is_same<T1, T3>::value"
        );;
        const _: () = assert!(true, "!std::is_same<T2, T3>::value");;
        const _: () = assert!(true, "check_tag<T1>(std::in_place_index<0>)");;
        const _: () = assert!(true, "check_tag<T2>(std::in_place_index<1>)");;
        const _: () = assert!(
            true,
            "check_tag<T3>(std::in_place_index<static_cast<std::size_t>(-1)>)"
        );;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
