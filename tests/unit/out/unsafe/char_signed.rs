extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn to_int_0(mut c: ()) -> () {
    return c;
}
pub unsafe fn is_negative_1(mut s: *const ()) -> () {
    return ((*s.offset((0_()) as isize)) < (0_()));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut c: () = -56_();
    let mut widened: () = c;
    (unsafe {
        libc::printf(
            c"%d %d\n".as_ptr() as *const libc::c_char,
            (widened),
            (unsafe { to_int_0((b'\xff' as ())) }),
        )
    });
    (unsafe { libc::printf(c"%d\n".as_ptr() as *const libc::c_char, ((c) < (0_()))) });
    let mut lit: [(); 4] = std::mem::transmute(*b"\xe9t\xe9\0");
    let mut ulit: [(); 4] = std::mem::transmute(*b"\xe9t\xe9\0");
    (unsafe {
        libc::printf(
            c"%d %d\n".as_ptr() as *const libc::c_char,
            (lit[(0_()) as usize]),
            (ulit[(0_()) as usize]),
        )
    });
    (unsafe {
        libc::printf(
            c"%d\n".as_ptr() as *const libc::c_char,
            (unsafe { is_negative_1((lit.as_mut_ptr()).cast_const()) }),
        )
    });
    let mut u8s: *const () = c"\xc3\xa9".as_ptr();
    (unsafe {
        libc::printf(
            c"%d %d\n".as_ptr() as *const libc::c_char,
            (*u8s.offset((0_()) as isize)),
            (*u8s.offset((1_()) as isize)),
        )
    });
    let mut p: *const () = c"\x80".as_ptr();
    (unsafe {
        libc::printf(
            c"%d %d\n".as_ptr() as *const libc::c_char,
            (*p.offset((0_()) as isize)),
            (*p.offset((0_()) as isize)),
        )
    });
    (unsafe {
        libc::printf(
            c"%d\n".as_ptr() as *const libc::c_char,
            ((unsafe { strcmp_2(c"\x80".as_ptr(), c"a".as_ptr()) }) > (0_())),
        )
    });
    let mut s: () = unimplemented!();
    (unsafe {
        libc::printf(
            c"%d %d\n".as_ptr() as *const libc::c_char,
            (*unimplemented!()),
            (*unimplemented!()),
        )
    });
    let mut sum: () = ((c) + (c));
    (unsafe { libc::printf(c"%d\n".as_ptr() as *const libc::c_char, (sum)) });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
