extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test1_0() {
    let mut re: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { c"\\{a\\}".as_ptr() },
        None,
    );
    let mut target: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { c"{a}".as_ptr() },
        None,
    );
    let mut regex: () = unimplemented!();
    let mut smatch: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test2_1() {
    let mut re: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { c"\\{a\\}".as_ptr() },
        None,
    );
    let mut target: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { c"{a}".as_ptr() },
        None,
    );
    let mut regex: () = unimplemented!();
    let mut smatch: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test3_2() {
    let mut re: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { c"\\{a\\}".as_ptr() },
        None,
    );
    let mut target: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { c"{a}".as_ptr() },
        None,
    );
    let mut regex: () = unimplemented!();
    let mut smatch: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test4_3() {
    let mut re: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { c"\\{a\\}".as_ptr() },
        None,
    );
    let mut target: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { c"{a}".as_ptr() },
        None,
    );
    let mut regex: () = unimplemented!();
    let mut smatch: () = unimplemented!();
    unimplemented!();
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
    (unsafe { test1_0() });
    (unsafe { test2_1() });
    (unsafe { test3_2() });
    (unsafe { test4_3() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
