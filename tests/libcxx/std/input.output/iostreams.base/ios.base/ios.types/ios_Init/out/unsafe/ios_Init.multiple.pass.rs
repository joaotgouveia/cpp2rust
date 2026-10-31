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
        let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
        let __s: *mut ::std::fs::File = __r;
        let _ = (*__s).write_all(::std::ffi::CStr::from_ptr(c"Hello!".as_ptr()).to_bytes());
        __s
    };
    let mut stock_flags: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::boolalpha_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(
                __s,
            )
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_bool(&mut *__s, true);
        __s
    };
    let mut ba_flags: () = unimplemented!();
    assert!(((stock_flags as i32) != (ba_flags as i32)));
    let mut init_streams: () = unimplemented!();
    let mut after_init: () = unimplemented!();
    assert!(((after_init as i32) == (ba_flags as i32)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
