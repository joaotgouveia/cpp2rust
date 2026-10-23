extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
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
pub unsafe fn test_1() {
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
pub unsafe fn test_2() {
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
    (unsafe { test_0() });
    (unsafe { test_1() });
    (unsafe { test_2() });
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_string_view<char>::traits_type, std::char_traits<char> >::value)"
    );;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
