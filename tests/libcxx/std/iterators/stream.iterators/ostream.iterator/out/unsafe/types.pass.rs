extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;

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
            "(std::is_same<I2::ostream_type, std::wostream>::value)"
        );;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
