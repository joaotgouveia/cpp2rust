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
            "(std::is_same<I1::iterator_category, std::input_iterator_tag>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<I1::value_type, char>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<I1::difference_type, std::char_traits<char>::off_type>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<I1::pointer, char*>::value)");;
        const _: () = assert!(true, "(std::is_same<I1::reference, char>::value)");;
        const _: () = assert!(true, "(std::is_same<I1::char_type, char>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<I1::traits_type, std::char_traits<char> >::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<I1::int_type, I1::traits_type::int_type>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<I1::streambuf_type, std::streambuf>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<I1::istream_type, std::istream>::value)"
        );;
        const _: () = assert!(true, "(std::is_nothrow_default_constructible<I1>::value)");;
        const _: () = assert!(true, "(std::is_trivially_copy_constructible<I1>::value)");;
        const _: () = assert!(true, "(std::is_trivially_destructible<I1>::value)");;
    }
    {
        const _: () = assert!(
            true,
            "(std::is_same<I2::iterator_category, std::input_iterator_tag>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<I2::value_type, wchar_t>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<I2::difference_type, std::char_traits<wchar_t>::off_type>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<I2::pointer, wchar_t*>::value)");;
        const _: () = assert!(true, "(std::is_same<I2::reference, wchar_t>::value)");;
        const _: () = assert!(true, "(std::is_same<I2::char_type, wchar_t>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<I2::traits_type, std::char_traits<wchar_t> >::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<I2::int_type, I2::traits_type::int_type>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<I2::streambuf_type, std::wstreambuf>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<I2::istream_type, std::wistream>::value)"
        );;
        const _: () = assert!(true, "(std::is_nothrow_default_constructible<I2>::value)");;
        const _: () = assert!(true, "(std::is_trivially_copy_constructible<I2>::value)");;
        const _: () = assert!(true, "(std::is_trivially_destructible<I2>::value)");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
