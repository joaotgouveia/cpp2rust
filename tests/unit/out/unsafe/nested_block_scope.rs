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
    let mut x: i32 = 1;
    {
        let mut x: i32 = 2;
        (if ((x) == (2)) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_0(
                    c"x == 2".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        {
            let mut x: i32 = 3;
            (if ((x) == (3)) {
                &(0);
            } else {
                (unsafe {
                    __assert_fail_0(
                        c"x == 3".as_ptr(),
                        concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                        line!() as u32,
                        c"int main()".as_ptr(),
                    )
                });
            });
        }
        (if ((x) == (2)) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_0(
                    c"x == 2".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
    }
    (if ((x) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"x == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut sum: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (3)) {
        let mut y: i32 = i;
        {
            let mut y: i32 = 10;
            sum += y;
        }
        sum += y;
        i.postfix_inc();
    }
    (if ((sum) == (33)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"sum == 33".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    if ((x) == (1)) {
        let mut x: i32 = 5;
        {
            let mut x: i32 = 6;
            (if ((x) == (6)) {
                &(0);
            } else {
                (unsafe {
                    __assert_fail_0(
                        c"x == 6".as_ptr(),
                        concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                        line!() as u32,
                        c"int main()".as_ptr(),
                    )
                });
            });
        }
        (if ((x) == (5)) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_0(
                    c"x == 5".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
    }
    (if ((x) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"x == 1".as_ptr(),
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
