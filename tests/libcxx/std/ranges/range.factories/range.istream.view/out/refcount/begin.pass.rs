extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn make_string_0(in_: Ptr<()>) -> () {
    return unimplemented!();
}
pub fn make_string_1(in_: Ptr<()>) -> () {
    return unimplemented!();
}
pub fn make_string_2(in_: Ptr<()>) -> () {
    return unimplemented!();
}
pub fn make_string_3(in_: Ptr<()>) -> () {
    return unimplemented!();
}
pub fn make_string_stream_4(in_: Ptr<()>) -> () {
    return unimplemented!();
}
pub fn make_string_stream_5(in_: Ptr<()>) -> () {
    return unimplemented!();
}
pub fn make_string_stream_6(in_: Ptr<()>) -> () {
    return unimplemented!();
}
pub fn make_string_stream_7(in_: Ptr<()>) -> () {
    return unimplemented!();
}
const _: () = assert!(true, "HasBegin<std::ranges::istream_view<int>>");
const _: () = assert!(!(false), "!HasBegin<const std::ranges::istream_view<int>>");
const _: () = assert!(true, "HasBegin<std::ranges::wistream_view<int>>");
const _: () = assert!(!(false), "!HasBegin<const std::ranges::wistream_view<int>>");
pub fn test_8() {
    {
        let iss: Value<()> = Rc::new(RefCell::new(
            ({ make_string_stream_4(Ptr::<()>::from_string_literal(b"12    3")) }),
        ));
        let isv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let iss: Value<()> = Rc::new(RefCell::new(
            ({ make_string_stream_5(Ptr::<()>::from_string_literal(b"")) }),
        ));
        let isv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_9() {
    {
        let iss: Value<()> = Rc::new(RefCell::new(
            ({ make_string_stream_6(Ptr::<()>::from_string_literal(b"12    3")) }),
        ));
        let isv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let iss: Value<()> = Rc::new(RefCell::new(
            ({ make_string_stream_7(Ptr::<()>::from_string_literal(b"")) }),
        ));
        let isv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
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
    ({ test_8() });
    ({ test_9() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
