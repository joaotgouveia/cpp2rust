extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn switch_in_loop_0(mut n: i32) -> i32 {
    let mut r: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        'switch: {
            match { ((i) % (3)) } {
                __v if __v == 0 => {
                    r += 1;
                    break 'switch;
                }
                __v if __v == 1 => {
                    r += 2;
                    break 'switch;
                }
                _ => {
                    r += 3;
                    break 'switch;
                }
            }
        };
        r += 10;
        i.prefix_inc();
    }
    return r;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (if ((unsafe { switch_in_loop_0(6) }) == (72)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"switch_in_loop(6) == 72".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_1(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
