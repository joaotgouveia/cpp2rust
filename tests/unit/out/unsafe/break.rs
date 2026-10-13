extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn for_test_0(n: i32) -> i32 {
    let mut x: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    'loop_: while ((i) < (n)) {
        x += 1;
        if ((x) == (100)) {
            break;
        }
        let mut k: i32 = 0;
        let mut w: i32 = 0;
        'loop_: while ((w) < (j)) {
            break;
            {
                {
                    w += 1;
                    k += 1
                };
                i += k
            };
        }
        x = ((x) + (1));
        {
            j = i;
            i += 1
        };
    }
    return x;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (if ((unsafe { for_test_0(200) }) == (400)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"for_test(200) == 400".as_ptr(),
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
