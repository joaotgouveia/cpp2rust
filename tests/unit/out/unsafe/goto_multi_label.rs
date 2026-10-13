extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn classify_0(mut n: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    goto_block!({
        '__entry: {
            ret = 0;
            if ((((n) < (0)) as i32) != 0) {
                goto!('error);
            }
            if ((((n) == (0)) as i32) != 0) {
                goto!('out);
            }
            ret = n;
            goto!('out);
        }
        'error: {
            ret = -1_i32;
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
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
            if ((((unsafe { classify_0(5) }) == (5)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"classify(5) == 5".as_ptr().cast_mut()).cast_const(),
                        (c"goto_multi_label.c".as_ptr().cast_mut()).cast_const(),
                        20_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { classify_0(0) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"classify(0) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"goto_multi_label.c".as_ptr().cast_mut()).cast_const(),
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
            if ((((unsafe { classify_0(-2_i32) }) == (-1_i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"classify(-2) == -1".as_ptr().cast_mut()).cast_const(),
                        (c"goto_multi_label.c".as_ptr().cast_mut()).cast_const(),
                        22_u32,
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
