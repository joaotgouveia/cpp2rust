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
    let mut buf: [libc::c_char; 2] = [(0 as libc::c_char); 2];
    let mut p: *mut libc::c_char = buf.as_mut_ptr();
    let mut q: *mut libc::c_char = std::ptr::null_mut();
    q = {
        p = (p).wrapping_add((1 as i32) as usize);
        p
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((q) == (buf.as_mut_ptr().offset((1) as isize))) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"q == buf + 1".as_ptr().cast_mut()).cast_const(),
                        (c"assign_as_value.c".as_ptr().cast_mut()).cast_const(),
                        7_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut out: libc::c_char = (0 as libc::c_char);
    'switch: {
        match {
            (({
                out = (('x' as i32) as libc::c_char);
                out
            }) as i32)
        } {
            __v if __v == ('x' as i32) => {
                {
                    &(::std::mem::size_of::<i32>());
                    { {} }
                };
                break 'switch;
            }
            _ => {
                {
                    &(::std::mem::size_of::<i32>());
                    {
                        {
                            (unsafe {
                                __assert_fail_0(
                                    (c"0".as_ptr().cast_mut()).cast_const(),
                                    (c"assign_as_value.c".as_ptr().cast_mut()).cast_const(),
                                    15_u32,
                                    c"int main(void)".as_ptr(),
                                )
                            });
                        }
                    }
                };
                break 'switch;
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((out as i32) == ('x' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"out == \'x\'".as_ptr().cast_mut()).cast_const(),
                        (c"assign_as_value.c".as_ptr().cast_mut()).cast_const(),
                        18_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
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
