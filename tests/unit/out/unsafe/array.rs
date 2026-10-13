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
    let N: i32 = 5;
    let mut arr1: [i32; 5] = [0, 0, 0, 0, 0];
    let arr2: [i32; 5] = [1, 1, 1, 1, 1];
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        arr1[(i) as usize] = ((i) + (arr2[(i) as usize]));
        i.prefix_inc();
    }
    let mut fatorial: i32 = 1;
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        fatorial *= arr1[(i) as usize];
        i.prefix_inc();
    }
    (if ((fatorial) == (120)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"fatorial == 120".as_ptr(),
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
