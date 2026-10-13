extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sm_0(mut n: i32) -> i32 {
    let mut steps: i32 = 0;
    switch!(match n {
        __v if __v == 0 => 'target: {
            steps += 1;
            break;
        }
        __v if __v == 1 => {
            steps += 10;
            goto!('target);
        }
        _ => {
            steps = -1_i32;
            break;
        }
    });
    return steps;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { sm_0(0) }) == (1)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"sm(0) == 1".as_ptr().cast_mut()).cast_const(),
                        (c"goto_switch_self_case.c".as_ptr().cast_mut()).cast_const(),
                        21_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { sm_0(1) }) == (11)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"sm(1) == 11".as_ptr().cast_mut()).cast_const(),
                        (c"goto_switch_self_case.c".as_ptr().cast_mut()).cast_const(),
                        22_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { sm_0(7) }) == (-1_i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"sm(7) == -1".as_ptr().cast_mut()).cast_const(),
                        (c"goto_switch_self_case.c".as_ptr().cast_mut()).cast_const(),
                        23_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
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
