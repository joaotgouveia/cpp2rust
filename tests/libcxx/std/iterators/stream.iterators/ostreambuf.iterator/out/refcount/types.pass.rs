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
    {
        const _: () = assert!(
            true,
            "(std::is_same<I1::iterator_category, std::output_iterator_tag>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<I1::value_type, void>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<I1::difference_type, std::ptrdiff_t>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<I1::pointer, void>::value)");;
        const _: () = assert!(true, "(std::is_same<I1::reference, void>::value)");;
        const _: () = assert!(true, "(std::is_same<I1::char_type, char>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<I1::traits_type, std::char_traits<char> >::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<I1::streambuf_type, std::streambuf>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<I1::ostream_type, std::ostream>::value)"
        );;
    }
    {
        const _: () = assert!(
            true,
            "(std::is_same<I2::iterator_category, std::output_iterator_tag>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<I2::value_type, void>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<I2::difference_type, std::ptrdiff_t>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<I2::pointer, void>::value)");;
        const _: () = assert!(true, "(std::is_same<I2::reference, void>::value)");;
        const _: () = assert!(true, "(std::is_same<I2::char_type, wchar_t>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<I2::traits_type, std::char_traits<wchar_t> >::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<I2::streambuf_type, std::wstreambuf>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<I2::ostream_type, std::wostream>::value)"
        );;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
