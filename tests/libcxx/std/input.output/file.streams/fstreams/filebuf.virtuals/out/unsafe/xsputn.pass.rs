extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sputn_seekoff_0(
    mut buf: *mut (),
    buf_size: (),
    chunk_size1: (),
    offset1: (),
    chunk_size2: (),
) {
    let mut data: () = unimplemented!();
    let data_size: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result_size: () = ((((5_()) + (chunk_size1)) + (chunk_size2)) + (1_()));
    if ((offset1) > (0_())) {
        result_size = (result_size).wrapping_add(offset1);
    }
    let mut result: () = unimplemented!();
    {
        let mut f: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut chunk: () = unimplemented!();
        unimplemented!();
        let len1: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut p1: () = unimplemented!();
        let mut c: () = unimplemented!();
        if ((unsafe { p1() }) < (0_())) {
            p1 = unimplemented!();
            unimplemented!();
            c = ('^' as ());
        } else {
            unimplemented!();
            if ((unsafe { p1() }) > (data_size)) {
                c = ('_' as ());
            } else {
                c = (*unimplemented!());
            }
        }
        (*unimplemented!()) = c;
        unimplemented!();
        unimplemented!();
        (*unimplemented!()) = ('A' as ());
        unimplemented!();
        let mut end_pos: () = unimplemented!();
        unimplemented!();
        (*unimplemented!()) = ('Z' as ());
        unimplemented!();
        (*unimplemented!()) = (unsafe { toupper_1(c) });
        unimplemented!();
        let mut new_pos: () = unimplemented!();
        let mut p2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let len2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    let mut f: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut check: () = unimplemented!();
    let len: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (len)) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn sputn_not_open_2() {
    let mut data: () = unimplemented!();
    let mut f: () = unimplemented!();
    let mut len: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn sputn_not_open_wchar_3() {
    let mut data: () = unimplemented!();
    let mut f: () = unimplemented!();
    let mut len: () = unimplemented!();
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
    (unsafe { sputn_not_open_2() });
    (unsafe { sputn_not_open_wchar_3() });
    (unsafe { sputn_seekoff_0(std::ptr::null_mut(), 10_(), 21_(), -27_(), 1_()) });
    (unsafe { sputn_seekoff_0(std::ptr::null_mut(), 10_(), 1_(), -27_(), 1_()) });
    (unsafe { sputn_seekoff_0(std::ptr::null_mut(), 10_(), 10_(), 14_(), 12_()) });
    (unsafe { sputn_seekoff_0(std::ptr::null_mut(), 10_(), 1_(), -2_(), 1_()) });
    (unsafe { sputn_seekoff_0(std::ptr::null_mut(), 10_(), 10_(), -4_(), 12_()) });
    (unsafe { sputn_seekoff_0(std::ptr::null_mut(), 10_(), 11_(), -12_(), 3_()) });
    (unsafe { sputn_seekoff_0(std::ptr::null_mut(), 10_(), 7_(), 3_(), 8_()) });
    (unsafe { sputn_seekoff_0(std::ptr::null_mut(), 10_(), 5_(), -5_(), 12_()) });
    (unsafe { sputn_seekoff_0(std::ptr::null_mut(), 10_(), 1_(), 1_(), 1_()) });
    (unsafe { sputn_seekoff_0(std::ptr::null_mut(), 10_(), 9_(), 0_(), 1_()) });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
