extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_iterators_0() {
    const _: () = assert!(
        true,
        "(std::is_same<typename ItT::iterator_category, std::random_access_iterator_tag>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename ItT::value_type, typename C::value_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename ItT::reference, typename C::reference>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename ItT::pointer, typename C::pointer>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename ItT::difference_type, typename C::difference_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename CItT::iterator_category, std::random_access_iterator_tag>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename CItT::value_type, typename C::value_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename CItT::reference, typename C::const_reference>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename CItT::pointer, typename C::const_pointer>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename CItT::difference_type, typename C::difference_type>::value)"
    );;
}
pub unsafe fn test_iterators_1() {
    const _: () = assert!(
        true,
        "(std::is_same<typename ItT::iterator_category, std::random_access_iterator_tag>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename ItT::value_type, typename C::value_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename ItT::reference, typename C::reference>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename ItT::pointer, typename C::pointer>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename ItT::difference_type, typename C::difference_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename CItT::iterator_category, std::random_access_iterator_tag>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename CItT::value_type, typename C::value_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename CItT::reference, typename C::const_reference>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename CItT::pointer, typename C::const_pointer>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename CItT::difference_type, typename C::difference_type>::value)"
    );;
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    {
        const _: () = assert!(true, "(std::is_same<C::reference, T&>::value)");;
        const _: () = assert!(true, "(std::is_same<C::const_reference, const T&>::value)");;
        (unsafe { test_iterators_0() });
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
        (unsafe { test_iterators_1() });
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
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
