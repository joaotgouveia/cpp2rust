extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn tests_0() -> () {
    {
        let arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "std::is_same_v<decltype(arr), std::array<int, 3>>");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = 42_();
        let arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "std::is_same_v<decltype(arr)::value_type, long>");;
        const _: () = assert!(true, "arr.size() == 4");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let source: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let arr: Value<()> = Rc::new(RefCell::new((*source.borrow()).clone()));
        const _: () = assert!(true, "std::is_same_v<decltype(arr), decltype(source)>");;
        const _: () = assert!(true, "std::is_same_v<decltype(arr), std::array<double, 2>>");;
        unimplemented!();
        unimplemented!();
    }
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
    ({ tests_0() });
    const _: () = assert!(true, "tests()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
