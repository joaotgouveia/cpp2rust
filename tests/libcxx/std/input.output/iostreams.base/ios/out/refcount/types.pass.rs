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
    const _: () = assert!(
        true,
        "(std::is_base_of<std::ios_base, std::basic_ios<char> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_ios<char>::char_type, char>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_ios<char>::traits_type, std::char_traits<char> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_ios<char>::int_type, std::char_traits<char>::int_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_ios<char>::pos_type, std::char_traits<char>::pos_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_ios<char>::off_type, std::char_traits<char>::off_type>::value)"
    );;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
