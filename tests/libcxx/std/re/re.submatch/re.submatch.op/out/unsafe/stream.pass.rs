extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(s: *const ()) {
    let mut sm: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    sm.matched = true;
    let mut os: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_1(s: *const ()) {
    let mut sm: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    sm.matched = true;
    let mut os: () = unimplemented!();
    unimplemented!();
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
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"123".as_ptr() },
            None,
        );
        test_0(&mut _s)
    });
    (unsafe {
        let mut _s: () =
            std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                None,
            );
        test_1(&mut _s)
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
