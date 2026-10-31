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
        let mut sb: () = unimplemented!();
        assert!(((unimplemented!()) == (-1_i32 as ())));
        assert!(((unimplemented!()) == (-1_i32 as ())));
        assert!(((unimplemented!()) == (3_())));
        assert!(((unimplemented!()) == (('3' as libc::c_char) as i32)));
    }
    {
        let mut sb: () = unimplemented!();
        assert!(((unimplemented!()) == (-1_i32 as ())));
        assert!(((unimplemented!()) == (-1_i32 as ())));
        assert!(((unimplemented!()) == (3_())));
        assert!(((unimplemented!()) == (('a' as libc::c_char) as i32)));
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c"012a456789".as_ptr(),
                    libcc2rs::c_str_len(c"012a456789".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut sb: () = unimplemented!();
        assert!(((unimplemented!()) == (3_())));
        assert!(((unimplemented!()) == (('3' as libc::c_char) as i32)));
        assert!(((unimplemented!()) == (3_())));
        assert!(((unimplemented!()) == (('3' as libc::c_char) as i32)));
        assert!(((unimplemented!()) == (('a' as libc::c_char) as i32)));
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c"012a456789".as_ptr(),
                    libcc2rs::c_str_len(c"012a456789".as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(((unimplemented!()) == (3_())));
        assert!(((unimplemented!()) == (('3' as libc::c_char) as i32)));
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c"0123456789".as_ptr(),
                    libcc2rs::c_str_len(c"0123456789".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut sb: () = unimplemented!();
        assert!(((unimplemented!()) == (-1_i32 as ())));
        assert!(((unimplemented!()) == (-1_i32 as ())));
        assert!(((unimplemented!()) == (3_())));
        assert!(((unimplemented!()) == ((51 as i32) as ())));
    }
    {
        let mut sb: () = unimplemented!();
        assert!(((unimplemented!()) == (-1_i32 as ())));
        assert!(((unimplemented!()) == (-1_i32 as ())));
        assert!(((unimplemented!()) == (3_())));
        assert!(((unimplemented!()) == ((97 as i32) as ())));
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    [
                        48 as i32, 49 as i32, 50 as i32, 97 as i32, 52 as i32, 53 as i32,
                        54 as i32, 55 as i32, 56 as i32, 57 as i32, 0 as i32,
                    ]
                    .as_ptr(),
                    libcc2rs::c_str_len(
                        [
                            48 as i32, 49 as i32, 50 as i32, 97 as i32, 52 as i32, 53 as i32,
                            54 as i32, 55 as i32, 56 as i32, 57 as i32, 0 as i32,
                        ]
                        .as_ptr()
                    ) + 1
                )
                .to_vec()
        );
    }
    {
        let mut sb: () = unimplemented!();
        assert!(((unimplemented!()) == (3_())));
        assert!(((unimplemented!()) == ((51 as i32) as ())));
        assert!(((unimplemented!()) == (3_())));
        assert!(((unimplemented!()) == ((51 as i32) as ())));
        assert!(((unimplemented!()) == ((97 as i32) as ())));
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    [
                        48 as i32, 49 as i32, 50 as i32, 97 as i32, 52 as i32, 53 as i32,
                        54 as i32, 55 as i32, 56 as i32, 57 as i32, 0 as i32,
                    ]
                    .as_ptr(),
                    libcc2rs::c_str_len(
                        [
                            48 as i32, 49 as i32, 50 as i32, 97 as i32, 52 as i32, 53 as i32,
                            54 as i32, 55 as i32, 56 as i32, 57 as i32, 0 as i32,
                        ]
                        .as_ptr()
                    ) + 1
                )
                .to_vec()
        );
        assert!(((unimplemented!()) == (3_())));
        assert!(((unimplemented!()) == ((51 as i32) as ())));
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    [
                        48 as i32, 49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32,
                        54 as i32, 55 as i32, 56 as i32, 57 as i32, 0 as i32,
                    ]
                    .as_ptr(),
                    libcc2rs::c_str_len(
                        [
                            48 as i32, 49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32,
                            54 as i32, 55 as i32, 56 as i32, 57 as i32, 0 as i32,
                        ]
                        .as_ptr()
                    ) + 1
                )
                .to_vec()
        );
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
