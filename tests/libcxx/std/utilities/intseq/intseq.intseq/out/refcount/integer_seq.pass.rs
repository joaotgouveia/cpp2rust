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
    const _: () = assert!(true, "std::is_same<int3::value_type, int>::value");;
    const _: () = assert!(true, "int3::size() == 3");;
    const _: () = assert!(true, "std::is_same<size1::value_type, std::size_t>::value");;
    const _: () = assert!(true, "size1::size() == 1");;
    const _: () = assert!(
        true,
        "std::is_same<ushort2::value_type, unsigned short>::value"
    );;
    const _: () = assert!(true, "ushort2::size() == 2");;
    const _: () = assert!(true, "std::is_same<bool0::value_type, bool>::value");;
    const _: () = assert!(true, "bool0::size() == 0");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
