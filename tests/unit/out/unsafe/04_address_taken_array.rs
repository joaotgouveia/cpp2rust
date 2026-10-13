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
    let mut arr2: [i32; 2] = [2, 2];
    arr2[(0) as usize] = 3;
    arr2[(1) as usize] = 4;
    let mut arr2_ptr: *mut i32 = arr2.as_mut_ptr();
    (*arr2_ptr.offset((0) as isize)) = 5;
    (*arr2_ptr.offset((1) as isize)) = 6;
    let arr2_ref1: *mut i32 = &mut arr2[(1) as usize];
    (*arr2_ref1) = 7;
    (if (((arr2[(0) as usize]) + (arr2[(1) as usize])) == (12)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"arr2[0] + arr2[1] == 12".as_ptr(),
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
