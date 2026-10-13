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
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { helper_0(42) }) == (43)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"helper(42) == 43".as_ptr().cast_mut()).cast_const(),
                        (c"a.c".as_ptr().cast_mut()).cast_const(),
                        6_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    return 0;
}
pub unsafe fn unrelated1_2() -> i32 {
    return 1;
}
pub unsafe fn unrelated2_3() -> i32 {
    return 2;
}
pub unsafe fn unrelated3_4() -> i32 {
    return 3;
}
pub unsafe fn helper_0(mut x: i32) -> i32 {
    &(unsafe { unrelated1_2() });
    &(unsafe { unrelated2_3() });
    &(unsafe { unrelated3_4() });
    return ((x) + (1));
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
