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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    {
        const _: () = assert!(true, "(std::is_same<C::reference, T&>::value)");;
        const _: () = assert!(true, "(std::is_same<C::const_reference, const T&>::value)");;
        ({ test_iterators_0() });
        const _: () = assert!(true, "(std::is_same<C::pointer, T*>::value)");;
        const _: () = assert!(true, "(std::is_same<C::const_pointer, const T*>::value)");;
        const _: () = assert!(true, "(std::is_same<C::size_type, std::size_t>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<C::difference_type, std::ptrdiff_t>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::reverse_iterator, std::reverse_iterator<C::iterator> >::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::const_reverse_iterator, std::reverse_iterator<C::const_iterator> >::value)"
        );;
        const _: () = assert!(true, "(std::is_signed<typename C::difference_type>::value)");;
        const _: () = assert!(true, "(std::is_unsigned<typename C::size_type>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<typename C::difference_type,\n                                typename std::iterator_traits<typename C::iterator>::difference_type>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<typename C::difference_type,\n                                typename std::iterator_traits<typename C::const_iterator>::difference_type>::value)"
        );;
    }
    {
        const _: () = assert!(true, "(std::is_same<C::reference, T&>::value)");;
        const _: () = assert!(true, "(std::is_same<C::const_reference, const T&>::value)");;
        ({ test_iterators_1() });
        const _: () = assert!(true, "(std::is_same<C::pointer, T*>::value)");;
        const _: () = assert!(true, "(std::is_same<C::const_pointer, const T*>::value)");;
        const _: () = assert!(true, "(std::is_same<C::size_type, std::size_t>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<C::difference_type, std::ptrdiff_t>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::reverse_iterator, std::reverse_iterator<C::iterator> >::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::const_reverse_iterator, std::reverse_iterator<C::const_iterator> >::value)"
        );;
        const _: () = assert!(true, "(std::is_signed<typename C::difference_type>::value)");;
        const _: () = assert!(true, "(std::is_unsigned<typename C::size_type>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<typename C::difference_type,\n                                typename std::iterator_traits<typename C::iterator>::difference_type>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<typename C::difference_type,\n                                typename std::iterator_traits<typename C::const_iterator>::difference_type>::value)"
        );;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
