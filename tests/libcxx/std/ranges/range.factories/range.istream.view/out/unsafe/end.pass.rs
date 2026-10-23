extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn make_string_0(in_: *const [(); 3]) -> () {
    return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { (*in_).as_ptr().offset((0_()) as isize) },
        {
            (*in_)
                .as_ptr()
                .offset(((3_() as ()).wrapping_sub(1_())) as isize)
        },
        None,
    );
}
pub unsafe fn make_string_1(in_: *const [(); 3]) -> () {
    return std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
        { (*in_).as_ptr().offset((0_()) as isize) },
        {
            (*in_)
                .as_ptr()
                .offset(((3_() as ()).wrapping_sub(1_())) as isize)
        },
        None,
    );
}
pub unsafe fn make_string_stream_2(in_: *const [(); 3]) -> () {
    return unimplemented!();
}
pub unsafe fn make_string_stream_3(in_: *const [(); 3]) -> () {
    return unimplemented!();
}
const _: () = assert!(true, "NoexceptEnd<std::ranges::istream_view<int>>");
const _: () = assert!(true, "NoexceptEnd<const std::ranges::istream_view<int>>");
const _: () = assert!(true, "NoexceptEnd<std::ranges::wistream_view<int>>");
const _: () = assert!(true, "NoexceptEnd<const std::ranges::wistream_view<int>>");
pub unsafe fn test_4() {
    let mut iss: () = (unsafe { make_string_stream_2(&std::mem::transmute(*b"12\0")) });
    let mut isv: () = unimplemented!();
    let mut sent: () = unimplemented!();
}
pub unsafe fn test_5() {
    let mut iss: () = (unsafe { make_string_stream_3(&std::mem::transmute(*b"12\0")) });
    let mut isv: () = unimplemented!();
    let mut sent: () = unimplemented!();
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
