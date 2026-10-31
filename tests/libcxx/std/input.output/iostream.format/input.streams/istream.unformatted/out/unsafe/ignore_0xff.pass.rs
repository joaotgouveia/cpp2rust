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
    let mut bad: i32 = -1_i32;
    let mut os: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*{
                    let __r: &mut ::std::fs::File = &mut os;
                    let __s: *mut ::std::fs::File = __r;
                    let _ = (*__s)
                        .write_all(::std::ffi::CStr::from_ptr(c"aaaabbbb".as_ptr()).to_bytes());
                    __s
                });
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(&[(bad as libc::c_char) as u8]);
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            let _ = (*__s).write_all(::std::ffi::CStr::from_ptr(c"ccccdddd".as_ptr()).to_bytes());
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        (libcc2rs::endl_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
    };
    let mut s: Vec<libc::c_char> = unimplemented!();
    let mut is: () = unimplemented!();
    let ignoreLen: u32 = 10_u32;
    let mut a: () = unimplemented!();
    unimplemented!();
    let mut b: () = unimplemented!();
    assert!(((unimplemented!()) == (ignoreLen as ())));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
