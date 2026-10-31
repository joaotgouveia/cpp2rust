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
        let mut ss0: () = unimplemented!();
        let mut ss: () = unimplemented!();
        unimplemented!();
        assert!(!((unimplemented!()).is_null()));
        assert!(unimplemented!());
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c" 123 456 ".as_ptr(),
                    libcc2rs::c_str_len(c" 123 456 ".as_ptr()) + 1
                )
                .to_vec()
        );
        let mut i: i32 = 0;
        unimplemented!();
        assert!(((i) == (123)));
        unimplemented!();
        assert!(((i) == (456)));
        {
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*{
                    let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
                    let __s: *mut ::std::fs::File = __r;
                    libcc2rs::write_int(&mut *__s, i);
                    __s
                });
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(&[(' ' as libc::c_char) as u8]);
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            libcc2rs::write_int(&mut *__s, 123);
            __s
        };
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c"456 1236 ".as_ptr(),
                    libcc2rs::c_str_len(c"456 1236 ".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut ss0: () = unimplemented!();
        let mut ss: () = unimplemented!();
        unimplemented!();
        assert!(!((unimplemented!()).is_null()));
        assert!(unimplemented!());
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    [
                        32 as i32, 49 as i32, 50 as i32, 51 as i32, 32 as i32, 52 as i32,
                        53 as i32, 54 as i32, 32 as i32, 0 as i32,
                    ]
                    .as_ptr(),
                    libcc2rs::c_str_len(
                        [
                            32 as i32, 49 as i32, 50 as i32, 51 as i32, 32 as i32, 52 as i32,
                            53 as i32, 54 as i32, 32 as i32, 0 as i32,
                        ]
                        .as_ptr()
                    ) + 1
                )
                .to_vec()
        );
        let mut i: i32 = 0;
        unimplemented!();
        assert!(((i) == (123)));
        unimplemented!();
        assert!(((i) == (456)));
        {
            let __r: &mut ::std::fs::File = &mut (*unimplemented!());
            let __s: *mut ::std::fs::File = __r;
            libcc2rs::write_int(&mut *__s, 123);
            __s
        };
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    [
                        52 as i32, 53 as i32, 54 as i32, 32 as i32, 49 as i32, 50 as i32,
                        51 as i32, 54 as i32, 32 as i32, 0 as i32,
                    ]
                    .as_ptr(),
                    libcc2rs::c_str_len(
                        [
                            52 as i32, 53 as i32, 54 as i32, 32 as i32, 49 as i32, 50 as i32,
                            51 as i32, 54 as i32, 32 as i32, 0 as i32,
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
