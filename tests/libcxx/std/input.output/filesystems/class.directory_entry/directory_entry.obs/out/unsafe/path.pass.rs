extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_path_method_0() {
    let p: Vec<libc::c_char> = {
        let mut __v: Vec<libc::c_char> = c"foo/bar/baz.exe"
            .path_bytes()
            .into_iter()
            .map(|__b| __b as libc::c_char)
            .collect();
        __v.push(0);
        __v
    };
    let p2: Vec<libc::c_char> = {
        let mut __v: Vec<libc::c_char> = c"abc"
            .path_bytes()
            .into_iter()
            .map(|__b| __b as libc::c_char)
            .collect();
        __v.push(0);
        __v
    };
    {
        let mut nce: () = unimplemented!();
        let e: () = unimplemented!();
        const _: () = assert!(
            true,
            "std::is_same<decltype(e.path()),  const path&>::value"
        );;
        const _: () = assert!(
            true,
            "std::is_same<decltype(nce.path()), const path&>::value"
        );;
        const _: () = assert!(true, "noexcept(e.path()) && noexcept(nce.path())");;
    }
    {
        let mut e: () = unimplemented!();
        let pref: *const Vec<libc::c_char> = unimplemented!();
        assert!(unimplemented!());
        assert!(((pref) == (unimplemented!())));
        unimplemented!();
        assert!(unimplemented!());
        assert!(((pref) == (unimplemented!())));
    }
}
pub unsafe fn test_path_conversion_1() {
    let p: Vec<libc::c_char> = {
        let mut __v: Vec<libc::c_char> = c"foo/bar/baz.exe"
            .path_bytes()
            .into_iter()
            .map(|__b| __b as libc::c_char)
            .collect();
        __v.push(0);
        __v
    };
    let p2: Vec<libc::c_char> = {
        let mut __v: Vec<libc::c_char> = c"abc"
            .path_bytes()
            .into_iter()
            .map(|__b| __b as libc::c_char)
            .collect();
        __v.push(0);
        __v
    };
    {
        let mut nce: () = unimplemented!();
        let e: () = unimplemented!();
        const _: () = assert!(
            true,
            "std::is_convertible<directory_entry&,        path const&>::value"
        );;
        const _: () = assert!(
            true,
            "std::is_convertible<directory_entry const&,  path const&>::value"
        );;
        const _: () = assert!(
            true,
            "std::is_convertible<directory_entry &&,      path const&>::value"
        );;
        const _: () = assert!(
            true,
            "std::is_convertible<directory_entry const&&, path const&>::value"
        );;
        const _: () = assert!(
            true,
            "!std::is_convertible<directory_entry&,        path&>::value"
        );;
        const _: () = assert!(
            true,
            "!std::is_convertible<directory_entry const&,  path&>::value"
        );;
        const _: () = assert!(
            true,
            "!std::is_convertible<directory_entry &&,      path&>::value"
        );;
        const _: () = assert!(
            true,
            "!std::is_convertible<directory_entry const&&, path&>::value"
        );;
        const _: () = assert!(
            true,
            "noexcept(e.operator fs::path const&()) &&\n                  noexcept(e.operator fs::path const&())"
        );;
    }
    {
        let e: () = unimplemented!();
        let pref: *const Vec<libc::c_char> = unimplemented!();
        assert!(((pref) == (unimplemented!())));
    }
    {
        let mut e: () = unimplemented!();
        let pref: *const Vec<libc::c_char> = unimplemented!();
        assert!(((pref) == (unimplemented!())));
        unimplemented!();
        assert!(unimplemented!());
        assert!(((pref) == (unimplemented!())));
    }
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_path_method_0() });
    (unsafe { test_path_conversion_1() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
