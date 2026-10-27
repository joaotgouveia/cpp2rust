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
        const _: () = assert!(true, "(std::is_same<SM::iterator, char*>::value)");;
        const _: () = assert!(true, "(std::is_same<SM::value_type, char>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<SM::difference_type, std::ptrdiff_t>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<SM::string_type, std::string>::value)");;
        const _: () = assert!(
            true,
            "(std::is_convertible<SM*, std::pair<char*, char*>*>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<std::csub_match, std::sub_match<const char*> >::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<std::ssub_match, std::sub_match<std::string::const_iterator> >::value)"
        );;
        let mut sm: () = unimplemented!();
        sm.first = std::ptr::null_mut();
        sm.second = std::ptr::null_mut();
        sm.matched = false;
    }
    {
        const _: () = assert!(true, "(std::is_same<SM::iterator, wchar_t*>::value)");;
        const _: () = assert!(true, "(std::is_same<SM::value_type, wchar_t>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<SM::difference_type, std::ptrdiff_t>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<SM::string_type, std::wstring>::value)");;
        const _: () = assert!(
            true,
            "(std::is_convertible<SM*, std::pair<wchar_t*, wchar_t*>*>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<std::wcsub_match, std::sub_match<const wchar_t*> >::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<std::wssub_match, std::sub_match<std::wstring::const_iterator> >::value)"
        );;
        let mut sm: () = unimplemented!();
        sm.first = std::ptr::null_mut();
        sm.second = std::ptr::null_mut();
        sm.matched = false;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
