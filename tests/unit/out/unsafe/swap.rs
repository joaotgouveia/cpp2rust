extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn identity_0(mut x: i32) -> i32 {
    return x;
}
pub unsafe fn swap_by_ptr_1(mut a: *mut i32, mut b: *mut i32) {
    let mut tmp: i32 = (*a);
    (*a) = (*b);
    (*b) = tmp;
}
pub unsafe fn swap_by_ref_2(a: *mut i32, b: *mut i32) {
    let mut tmp: i32 = (*a);
    (*a) = (*b);
    (*b) = tmp;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut local: i32 = 0;
    let mut a: i32 = 1;
    let mut b: i32 = 2;
    let mut c: i32 = (unsafe { identity_0(local) });
    let mut p: *mut i32 = (&mut a as *mut i32);
    p = (&mut (b) as *mut i32);
    p = (&mut a as *mut i32);
    (unsafe { swap_by_ptr_1(p, (&mut b as *mut i32)) });
    (unsafe { swap_by_ref_2(&mut a, &mut c) });
    (if ((c) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"c == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_3(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
