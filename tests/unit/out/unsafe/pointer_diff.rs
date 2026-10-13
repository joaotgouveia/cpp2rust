extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    let mut p0: *const i32 = (&mut a[(0) as usize] as *mut i32).cast_const();
    let mut p1: *const i32 = (&mut a[(4) as usize] as *mut i32).cast_const();
    (if ((((p1 as usize - p0 as usize) / ::std::mem::size_of::<i32>()) as i64) == (4_i64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"p1 - p0 == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_0(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
