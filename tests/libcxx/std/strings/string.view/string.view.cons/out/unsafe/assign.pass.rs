extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(mut sv0: ()) -> () {
    let mut sv1: () = unimplemented!();
    unimplemented!();
    return ((unimplemented!()) == (unimplemented!()))
        && ((unimplemented!()) == (unimplemented!()));
}
pub unsafe fn test_1(mut sv0: ()) -> () {
    let mut sv1: () = unimplemented!();
    unimplemented!();
    return ((unimplemented!()) == (unimplemented!()))
        && ((unimplemented!()) == (unimplemented!()));
}
pub unsafe fn test_2(mut sv0: ()) -> () {
    let mut sv1: () = unimplemented!();
    unimplemented!();
    return ((unimplemented!()) == (unimplemented!()))
        && ((unimplemented!()) == (unimplemented!()));
}
pub unsafe fn test_3(mut sv0: ()) -> () {
    let mut sv1: () = unimplemented!();
    unimplemented!();
    return ((unimplemented!()) == (unimplemented!()))
        && ((unimplemented!()) == (unimplemented!()));
}
pub unsafe fn test_4(mut sv0: ()) -> () {
    let mut sv1: () = unimplemented!();
    unimplemented!();
    return ((unimplemented!()) == (unimplemented!()))
        && ((unimplemented!()) == (unimplemented!()));
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "test<std::string_view>({{\"abc\", 3}})");;
    const _: () = assert!(true, "test<std::u8string_view>({{u8\"abc\", 3}})");;
    const _: () = assert!(true, "test<std::u16string_view>({{u\"abc\", 3}})");;
    const _: () = assert!(true, "test<std::u32string_view>({{U\"abc\", 3}})");;
    const _: () = assert!(true, "test<std::wstring_view>({{L\"abc\", 3}})");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
