extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_ALPHA: anon_0 = 7;
pub unsafe fn a_value_1() -> i32 {
    let mut x: i32 = 0;
    x |= (anon_0_ALPHA as i32);
    return x;
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
            if ((((unsafe { a_value_1() }) == (7)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"a_value() == 7".as_ptr().cast_mut()).cast_const(),
                        (c"a.c".as_ptr().cast_mut()).cast_const(),
                        14_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { b_value_3() }) == (9)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"b_value() == 9".as_ptr().cast_mut()).cast_const(),
                        (c"a.c".as_ptr().cast_mut()).cast_const(),
                        15_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    return 0;
}
pub type anon_4 = u32;
pub const anon_4_BETA: anon_4 = 9;
pub unsafe fn b_value_3() -> i32 {
    let mut x: i32 = 0;
    x |= (anon_4_BETA as i32);
    return x;
}
pub unsafe fn __assert_fail_2(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
