extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn make_string_0(in_: *const [(); 4]) -> () {
    return unimplemented!();
}
pub unsafe fn make_string_1(in_: *const [(); 4]) -> () {
    return unimplemented!();
}
pub unsafe fn make_string_stream_2(in_: *const [(); 4]) -> () {
    return unimplemented!();
}
pub unsafe fn make_string_stream_3(in_: *const [(); 4]) -> () {
    return unimplemented!();
}
const _: () = assert!(
    true,
    "std::constructible_from<std::ranges::istream_view<int>, std::istream&>"
);
const _: () = assert!(
    !(false),
    "!std::convertible_to<std::istream&, std::ranges::istream_view<int>>"
);
const _: () = assert!(
    true,
    "std::constructible_from<std::ranges::wistream_view<int>, std::wistream&>"
);
const _: () = assert!(
    !(false),
    "!std::convertible_to<std::wistream&, std::ranges::wistream_view<int>>"
);
pub unsafe fn test_4() {
    {
        let mut iss: () = (unsafe { make_string_stream_2(&std::mem::transmute(*b"123\0")) });
        let mut isv: () = unimplemented!();
        let mut it: std_ranges_basic_istream_view_int__char____iterator = unimplemented!();
        unimplemented!();
    }
    {
        let mut iss: () = (unsafe { make_string_stream_2(&std::mem::transmute(*b"123\0")) });
        let mut isv: () = unimplemented!();
        let mut iter: std_ranges_basic_istream_view_int__char____iterator =
            std_ranges_basic_istream_view_int__char____iterator::new({ &mut isv });
        unimplemented!();
    }
}
pub unsafe fn test_5() {
    {
        let mut iss: () = (unsafe { make_string_stream_3(&std::mem::transmute(*b"123\0")) });
        let mut isv: () = unimplemented!();
        let mut it: std_ranges_basic_istream_view_int__wchar_t____iterator = unimplemented!();
        unimplemented!();
    }
    {
        let mut iss: () = (unsafe { make_string_stream_3(&std::mem::transmute(*b"123\0")) });
        let mut isv: () = unimplemented!();
        let mut iter: std_ranges_basic_istream_view_int__wchar_t____iterator =
            std_ranges_basic_istream_view_int__wchar_t____iterator::new({ &mut isv });
        unimplemented!();
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_4() });
    (unsafe { test_5() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
