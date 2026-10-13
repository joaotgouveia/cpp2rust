extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn continue_inside_switch_0(mut n: i32) -> i32 {
    let mut r: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        'switch: {
            match { i } {
                __v if __v == 0 || __v == 2 || __v == 4 => {
                    i.prefix_inc();
                    continue 'loop_;
                }
                _ => {
                    r += i;
                    break 'switch;
                }
            }
        };
        r += 1000;
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
    (if ((unsafe { continue_inside_switch_0(6) }) == ((((1) + (3)) + (5)) + ((3) * (1000)))) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"continue_inside_switch(6) == (1 + 3 + 5) + 3 * 1000".as_ptr(),
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
