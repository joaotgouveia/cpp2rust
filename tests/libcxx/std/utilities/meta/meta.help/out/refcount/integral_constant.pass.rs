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
    const _: () = assert!(true, "_5::value == 5");;
    const _: () = assert!(true, "(std::is_same<_5::value_type, int>::value)");;
    const _: () = assert!(true, "(std::is_same<_5::type, _5>::value)");;
    const _: () = assert!(true, "(_5() == 5)");;
    unimplemented!();
    const _: () = assert!(true, "_5{{}}() == 5");;
    const _: () = assert!(true, "std::true_type{{}}()");;
    const _: () = assert!(true, "std::false_type::value == false");;
    const _: () = assert!(
        true,
        "(std::is_same<std::false_type::value_type, bool>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::false_type::type, std::false_type>::value)"
    );;
    const _: () = assert!(true, "std::true_type::value == true");;
    const _: () = assert!(
        true,
        "(std::is_same<std::true_type::value_type, bool>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::true_type::type, std::true_type>::value)"
    );;
    let f1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let f2: Value<()> = Rc::new(RefCell::new((*f1.borrow()).clone()));
    unimplemented!();
    let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let t2: Value<()> = Rc::new(RefCell::new((*t1.borrow()).clone()));
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
