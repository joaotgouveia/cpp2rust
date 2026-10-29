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
    const _: () = assert!(true, "std::is_enum<std::endian>::value");;
    const _: () = assert!(true, "!std::is_convertible<std::endian, UT>::value");;
    const _: () = assert!(true, "std::endian::little == std::endian::little");;
    const _: () = assert!(true, "std::endian::big    == std::endian::big");;
    const _: () = assert!(true, "std::endian::native == std::endian::native");;
    const _: () = assert!(true, "std::endian::little != std::endian::big");;
    const _: () = assert!(
        true,
        "std::endian::native == std::endian::little ||\n                   std::endian::native == std::endian::big"
    );;
    {
        let i: Value<()> = Rc::new(RefCell::new(16909060_()));
        let c: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        const _: () = assert!(
            (::std::mem::size_of::<()>() == ::std::mem::size_of::<[(); 4]>()),
            "sizeof(i) == sizeof(c)"
        );;
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
