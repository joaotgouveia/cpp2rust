extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn loopctl_0() -> i32 {
    let mut sum: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((((i) < (5)) as i32) != 0) {
        goto_block!({
            '__entry: {
                if ((((i) == (1)) as i32) != 0) {
                    i.postfix_inc();
                    continue 'loop_;
                }
                if ((((i) == (4)) as i32) != 0) {
                    break;
                }
                goto!('add);
            }
            'add: {
                sum += 1;
            }
        });
        i.postfix_inc();
    }
    return sum;
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
            if ((((unsafe { loopctl_0() }) == (3)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"loopctl() == 3".as_ptr().cast_mut()).cast_const(),
                        (c"goto_loop_control.c".as_ptr().cast_mut()).cast_const(),
                        20_u32,
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
