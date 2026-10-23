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
pub fn make_string_4(in_: Ptr<()>) -> () {
    return unimplemented!();
}
pub fn make_string_5(in_: Ptr<()>) -> () {
    return unimplemented!();
}
pub fn make_string_stream_6(in_: Ptr<()>) -> () {
    return unimplemented!();
}
pub fn make_string_stream_7(in_: Ptr<()>) -> () {
    return unimplemented!();
}
pub fn test_8() {
    let ints: Value<()> = Rc::new(RefCell::new(
        ({ make_string_stream_6(Ptr::<()>::from_string_literal(b"0 1  2   3     4")) }),
    ));
    let oss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut delimiter: () = ({ make_string_1(Ptr::<()>::from_string_literal(b"-")) });
    ({
        let ___r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*copy_9.with(Value::clone).borrow_mut())(___r.as_pointer(), unimplemented!())
    });
    let mut expected: () = ({ make_string_2(Ptr::<()>::from_string_literal(b"0-1-2-3-4-")) });
    unimplemented!();
}
pub fn test_10() {
    let ints: Value<()> = Rc::new(RefCell::new(
        ({ make_string_stream_7(Ptr::<()>::from_string_literal(b"0 1  2   3     4")) }),
    ));
    let oss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut delimiter: () = ({ make_string_4(Ptr::<()>::from_string_literal(b"-")) });
    ({
        let ___r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*copy_9.with(Value::clone).borrow_mut())(___r.as_pointer(), unimplemented!())
    });
    let mut expected: () = ({ make_string_5(Ptr::<()>::from_string_literal(b"0-1-2-3-4-")) });
    unimplemented!();
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
    ({ test_10() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
