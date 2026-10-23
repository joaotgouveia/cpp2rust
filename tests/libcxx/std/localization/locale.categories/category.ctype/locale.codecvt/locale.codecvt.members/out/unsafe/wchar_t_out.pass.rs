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
    let mut l: () = unimplemented!();
    let f: *const () = (unsafe { use_facet_0(&l) });
    {
        let from: () =
            std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
                {
                    [
                        115 as (), 111 as (), 109 as (), 101 as (), 32 as (), 116 as (), 101 as (),
                        120 as (), 116 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                None,
            );
        let mut to: () = unimplemented!();
        let mut mbs: () = unimplemented!();
        let mut from_next: *const () = std::ptr::null();
        let mut to_next: *mut () = std::ptr::null_mut();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut from: () =
            std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
                {
                    [
                        115 as (), 111 as (), 109 as (), 101 as (), 32 as (), 116 as (), 101 as (),
                        120 as (), 116 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                None,
            );
        (*unimplemented!()) = ('\0' as ());
        let mut to: () = unimplemented!();
        let mut mbs: () = unimplemented!();
        let mut from_next: *const () = std::ptr::null();
        let mut to_next: *mut () = std::ptr::null_mut();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut from: () =
            std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
                {
                    [
                        115 as (), 111 as (), 109 as (), 101 as (), 32 as (), 116 as (), 101 as (),
                        120 as (), 116 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                None,
            );
        let mut to: () = unimplemented!();
        let mut mbs: () = unimplemented!();
        let mut from_next: *const () = std::ptr::null();
        let mut to_next: *mut () = std::ptr::null_mut();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
