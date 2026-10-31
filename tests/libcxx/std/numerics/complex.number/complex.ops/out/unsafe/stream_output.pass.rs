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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let c: () = unimplemented!();
        let mut os: () = unimplemented!();
        unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c"(1,2)".as_ptr(),
                    libcc2rs::c_str_len(c"(1,2)".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut width: i32 = 0;
        'loop_: while ((width) <= (5)) {
            let c: () = unimplemented!();
            let mut os: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (0_())));
            assert!(
                unimplemented!()
                    == std::slice::from_raw_parts(
                        c"(1,2)".as_ptr(),
                        libcc2rs::c_str_len(c"(1,2)".as_ptr()) + 1
                    )
                    .to_vec()
            );
            width.prefix_inc();
        }
        {
            let c: () = unimplemented!();
            let mut os: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (0_())));
            assert!(
                unimplemented!()
                    == std::slice::from_raw_parts(
                        c"_(1,2)".as_ptr(),
                        libcc2rs::c_str_len(c"_(1,2)".as_ptr()) + 1
                    )
                    .to_vec()
            );
        }
        {
            let c: () = unimplemented!();
            let mut os: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (0_())));
            assert!(
                unimplemented!()
                    == std::slice::from_raw_parts(
                        c"__(1,2)".as_ptr(),
                        libcc2rs::c_str_len(c"__(1,2)".as_ptr()) + 1
                    )
                    .to_vec()
            );
        }
        {
            let c: () = unimplemented!();
            let mut os: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (0_())));
            assert!(
                unimplemented!()
                    == std::slice::from_raw_parts(
                        c"___(1,2)".as_ptr(),
                        libcc2rs::c_str_len(c"___(1,2)".as_ptr()) + 1
                    )
                    .to_vec()
            );
        }
        {
            let c: () = unimplemented!();
            let mut os: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (0_())));
            {
                let __r: &mut ::std::fs::File = &mut os;
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(::std::ffi::CStr::from_ptr(c"hello".as_ptr()).to_bytes());
                __s
            };
            assert!(
                unimplemented!()
                    == std::slice::from_raw_parts(
                        c"___(1,2)hello".as_ptr(),
                        libcc2rs::c_str_len(c"___(1,2)hello".as_ptr()) + 1
                    )
                    .to_vec()
            );
        }
        {
            let c: () = unimplemented!();
            let mut os: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (0_())));
            assert!(
                unimplemented!()
                    == std::slice::from_raw_parts(
                        c"(123,456)".as_ptr(),
                        libcc2rs::c_str_len(c"(123,456)".as_ptr()) + 1
                    )
                    .to_vec()
            );
        }
        {
            let c: () = unimplemented!();
            let mut os: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (0_())));
            assert!(
                unimplemented!()
                    == std::slice::from_raw_parts(
                        c"___(123,456)".as_ptr(),
                        libcc2rs::c_str_len(c"___(123,456)".as_ptr()) + 1
                    )
                    .to_vec()
            );
            {
                let __r: &mut ::std::fs::File = &mut os;
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(::std::ffi::CStr::from_ptr(c"hello".as_ptr()).to_bytes());
                __s
            };
            assert!(
                unimplemented!()
                    == std::slice::from_raw_parts(
                        c"___(123,456)hello".as_ptr(),
                        libcc2rs::c_str_len(c"___(123,456)hello".as_ptr()) + 1
                    )
                    .to_vec()
            );
        }
        {
            let c: () = unimplemented!();
            let mut os: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (0_())));
            assert!(
                unimplemented!()
                    == std::slice::from_raw_parts(
                        c"(123,456)___".as_ptr(),
                        libcc2rs::c_str_len(c"(123,456)___".as_ptr()) + 1
                    )
                    .to_vec()
            );
            {
                let __r: &mut ::std::fs::File = &mut os;
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(::std::ffi::CStr::from_ptr(c"xy".as_ptr()).to_bytes());
                __s
            };
            assert!(
                unimplemented!()
                    == std::slice::from_raw_parts(
                        c"(123,456)___xy".as_ptr(),
                        libcc2rs::c_str_len(c"(123,456)___xy".as_ptr()) + 1
                    )
                    .to_vec()
            );
        }
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
