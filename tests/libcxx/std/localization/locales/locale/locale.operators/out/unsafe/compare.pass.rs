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
        let mut l: std_locale = <std_locale>::default();
        {
            let mut s2: () =
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"aaaaaaA".as_ptr() },
                    None,
                );
            let mut s3: () =
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"BaaaaaA".as_ptr() },
                    None,
                );
            unimplemented!();
        }
        {
            let mut s2: () =
                std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
                    {
                        [
                            97 as (), 97 as (), 97 as (), 97 as (), 97 as (), 97 as (), 65 as (),
                            0 as (),
                        ]
                        .as_ptr()
                    },
                    None,
                );
            let mut s3: () =
                std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
                    {
                        [
                            66 as (), 97 as (), 97 as (), 97 as (), 97 as (), 97 as (), 65 as (),
                            0 as (),
                        ]
                        .as_ptr()
                    },
                    None,
                );
            unimplemented!();
        }
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
