extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_required_span_size_0(e: (), strides: (), mut expected_size: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub fn test_required_span_size_1(e: (), strides: (), mut expected_size: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub fn test_required_span_size_2(e: (), strides: (), mut expected_size: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub fn test_required_span_size_3(e: (), strides: (), mut expected_size: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub fn test_required_span_size_4(e: (), strides: (), mut expected_size: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub fn test_required_span_size_5(e: (), strides: (), mut expected_size: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub fn test_required_span_size_6(e: (), strides: (), mut expected_size: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub fn test_7() -> () {
    let mut D: () = dynamic_extent_8.with(|rc| *rc.borrow());
    ({ test_required_span_size_0(unimplemented!(), unimplemented!(), 1_()) });
    ({ test_required_span_size_1(unimplemented!(), unimplemented!(), 0_()) });
    ({ test_required_span_size_1(unimplemented!(), unimplemented!(), 1_()) });
    ({ test_required_span_size_1(unimplemented!(), unimplemented!(), 31_()) });
    ({ test_required_span_size_2(unimplemented!(), unimplemented!(), 31_()) });
    ({ test_required_span_size_3(unimplemented!(), unimplemented!(), 135_()) });
    ({ test_required_span_size_4(unimplemented!(), unimplemented!(), 5040_()) });
    ({ test_required_span_size_5(unimplemented!(), unimplemented!(), 5034_()) });
    ({ test_required_span_size_6(unimplemented!(), unimplemented!(), 0_()) });
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
    ({ test_7() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
