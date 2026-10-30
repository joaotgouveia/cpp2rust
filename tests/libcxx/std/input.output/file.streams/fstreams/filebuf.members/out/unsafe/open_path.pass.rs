extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn get_temp_file_name_0() -> () {
    let mut Name: () = unimplemented!();
    let mut FD: () = unimplemented!();
    if ((FD) == (-1_())) {
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn glibc_version_less_than_1(mut version: *const ()) -> () {
    let mut test_version: () = unimplemented!();
    let mut n: () = unimplemented!();
    let mut current_version: *mut () =
        Box::leak((0..n).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
    unimplemented!();
    let mut result: () = ((unimplemented!()) < (0_()));
    {
        let __p = current_version;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<()>(),
            )))
        }
    };
    return result;
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
    let mut p: () = unimplemented!();
    {
        let mut f: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut f: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    {
        let mut f: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut f: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
