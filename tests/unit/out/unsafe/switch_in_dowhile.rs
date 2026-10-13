extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn switch_in_dowhile_0(mut n: i32) -> i32 {
    let mut r: i32 = 0;
    let mut i: i32 = 0;
    let mut __do_while = true;
    'loop_: while __do_while || ((i) < (n)) {
        __do_while = false;
        'switch: {
            match { i } {
                __v if __v == 0 => {
                    r += 1;
                    break 'switch;
                }
                __v if __v == 1 => {
                    r += 10;
                    break 'switch;
                }
                _ => {
                    r += 100;
                    break 'switch;
                }
            }
        };
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
    (if ((unsafe { switch_in_dowhile_0(1) }) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"switch_in_dowhile(1) == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { switch_in_dowhile_0(3) }) == (((1) + (10)) + (100))) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"switch_in_dowhile(3) == 1 + 10 + 100".as_ptr(),
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
