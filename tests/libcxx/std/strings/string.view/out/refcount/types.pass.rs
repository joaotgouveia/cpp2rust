extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    const _: () = assert!(
        true,
        "(std::is_same<typename S::traits_type, Traits>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::value_type, typename Traits::char_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::size_type, std::size_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::difference_type, std::ptrdiff_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::reference, typename S::value_type&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::const_reference, const typename S::value_type&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::pointer, typename S::value_type*>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::const_pointer, const typename S::value_type*>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename std::iterator_traits<typename S::iterator>::iterator_category,\n                               std::random_access_iterator_tag>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename std::iterator_traits<typename S::const_iterator>::iterator_category,\n                               std::random_access_iterator_tag>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename S::reverse_iterator, std::reverse_iterator<typename S::iterator> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename S::const_reverse_iterator, std::reverse_iterator<typename S::const_iterator> >::value)"
    );;
    const _: () = assert!(true, "S::npos == -1");;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::iterator, typename S::const_iterator>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::reverse_iterator, typename S::const_reverse_iterator>::value)"
    );;
}
pub fn test_1() {
    const _: () = assert!(
        true,
        "(std::is_same<typename S::traits_type, Traits>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::value_type, typename Traits::char_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::size_type, std::size_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::difference_type, std::ptrdiff_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::reference, typename S::value_type&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::const_reference, const typename S::value_type&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::pointer, typename S::value_type*>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::const_pointer, const typename S::value_type*>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename std::iterator_traits<typename S::iterator>::iterator_category,\n                               std::random_access_iterator_tag>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename std::iterator_traits<typename S::const_iterator>::iterator_category,\n                               std::random_access_iterator_tag>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename S::reverse_iterator, std::reverse_iterator<typename S::iterator> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename S::const_reverse_iterator, std::reverse_iterator<typename S::const_iterator> >::value)"
    );;
    const _: () = assert!(true, "S::npos == -1");;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::iterator, typename S::const_iterator>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::reverse_iterator, typename S::const_reverse_iterator>::value)"
    );;
}
pub fn test_2() {
    const _: () = assert!(
        true,
        "(std::is_same<typename S::traits_type, Traits>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::value_type, typename Traits::char_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::size_type, std::size_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::difference_type, std::ptrdiff_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::reference, typename S::value_type&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::const_reference, const typename S::value_type&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::pointer, typename S::value_type*>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::const_pointer, const typename S::value_type*>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename std::iterator_traits<typename S::iterator>::iterator_category,\n                               std::random_access_iterator_tag>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename std::iterator_traits<typename S::const_iterator>::iterator_category,\n                               std::random_access_iterator_tag>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename S::reverse_iterator, std::reverse_iterator<typename S::iterator> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< typename S::const_reverse_iterator, std::reverse_iterator<typename S::const_iterator> >::value)"
    );;
    const _: () = assert!(true, "S::npos == -1");;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::iterator, typename S::const_iterator>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename S::reverse_iterator, typename S::const_reverse_iterator>::value)"
    );;
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
    ({ test_0() });
    ({ test_1() });
    ({ test_2() });
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_string_view<char>::traits_type, std::char_traits<char> >::value)"
    );;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
