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
        "(std::is_same<std::basic_regex<char>::value_type, char>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_regex<char>::traits_type, std::regex_traits<char> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_regex<char>::string_type, std::basic_string<char> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_regex<char>::flag_type,\n                                std::regex_constants::syntax_option_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_regex<char>::locale_type, std::locale>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_regex<wchar_t>::value_type, wchar_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_regex<wchar_t>::traits_type, std::regex_traits<wchar_t> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_regex<wchar_t>::string_type, std::basic_string<wchar_t> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_regex<wchar_t>::flag_type,\n                                std::regex_constants::syntax_option_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_regex<wchar_t>::locale_type, std::locale>::value)"
    );;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
