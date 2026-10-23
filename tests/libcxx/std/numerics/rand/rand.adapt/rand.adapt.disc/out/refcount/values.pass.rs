extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn where_0(_a0: Ptr<()>) {}
pub fn test1_1() {
    const _: () = assert!(true, "(E::block_size == 223)");;
    const _: () = assert!(true, "(E::used_block == 23)");;
    const _: () = assert!(true, "(E::min() == 0)");;
    const _: () = assert!(true, "(E::max() == 0xFFFFFF)");;
    ({ where_0(block_size_2.with(|v| v.as_pointer())) });
    ({ where_0(used_block_3.with(|v| v.as_pointer())) });
}
pub fn test2_4() {
    const _: () = assert!(true, "(E::block_size == 389)");;
    const _: () = assert!(true, "(E::used_block == 11)");;
    const _: () = assert!(true, "(E::min() == 0)");;
    const _: () = assert!(true, "(E::max() == 0xFFFFFFFFFFFFull)");;
    ({ where_0(block_size_5.with(|v| v.as_pointer())) });
    ({ where_0(used_block_6.with(|v| v.as_pointer())) });
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
    ({ test1_1() });
    ({ test2_4() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
