extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_layout_mapping_stride_0(ext: (), strides: (), mut exhaustive: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_layout_mapping_stride_1(ext: (), strides: (), mut exhaustive: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_layout_mapping_stride_2(ext: (), strides: (), mut exhaustive: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_layout_mapping_stride_3(ext: (), strides: (), mut exhaustive: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_layout_mapping_stride_4(ext: (), strides: (), mut exhaustive: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_layout_mapping_stride_5(ext: (), strides: (), mut exhaustive: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_6() -> () {
    let mut D: () = dynamic_extent_7.with(|rc| *rc.borrow());
    ({ test_layout_mapping_stride_0(unimplemented!(), unimplemented!(), true) });
    ({ test_layout_mapping_stride_1(unimplemented!(), unimplemented!(), true) });
    ({ test_layout_mapping_stride_2(unimplemented!(), unimplemented!(), true) });
    ({ test_layout_mapping_stride_3(unimplemented!(), unimplemented!(), true) });
    ({ test_layout_mapping_stride_3(unimplemented!(), unimplemented!(), true) });
    ({ test_layout_mapping_stride_4(unimplemented!(), unimplemented!(), true) });
    ({ test_layout_mapping_stride_5(unimplemented!(), unimplemented!(), true) });
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
    ({ test_6() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
