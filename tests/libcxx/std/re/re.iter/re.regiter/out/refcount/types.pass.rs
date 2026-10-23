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
        "(std::is_same<typename I::regex_type, std::basic_regex<CharT> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename I::value_type, std::match_results<const CharT*> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename I::difference_type, std::ptrdiff_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename I::pointer, const std::match_results<const CharT*>*>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename I::reference, const std::match_results<const CharT*>&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename I::iterator_category, std::forward_iterator_tag>::value)"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<typename I::iterator_concept, std::input_iterator_tag>"
    );;
}
pub fn test_1() {
    const _: () = assert!(
        true,
        "(std::is_same<typename I::regex_type, std::basic_regex<CharT> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename I::value_type, std::match_results<const CharT*> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename I::difference_type, std::ptrdiff_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename I::pointer, const std::match_results<const CharT*>*>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename I::reference, const std::match_results<const CharT*>&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename I::iterator_category, std::forward_iterator_tag>::value)"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<typename I::iterator_concept, std::input_iterator_tag>"
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
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
