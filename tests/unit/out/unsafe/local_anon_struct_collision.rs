extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn first_0() -> i32 {
    let mut p: anon_1 = <anon_1>::default();
    p.x = 1;
    p.y = 2;
    return ((p.x) + (p.y));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_1 {
    pub x: i32,
    pub y: i32,
}
pub unsafe fn second_2() -> i32 {
    let mut q: anon_3 = <anon_3>::default();
    q.a = 10_i64;
    q.b = 20_i64;
    return (((q.a) + (q.b)) as i32);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_3 {
    pub a: i64,
    pub b: i64,
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
            if ((((unsafe { first_0() }) == (3)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"first() == 3".as_ptr().cast_mut()).cast_const(),
                        (c"local_anon_struct_collision.c".as_ptr().cast_mut()).cast_const(),
                        24_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { second_2() }) == (30)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"second() == 30".as_ptr().cast_mut()).cast_const(),
                        (c"local_anon_struct_collision.c".as_ptr().cast_mut()).cast_const(),
                        25_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    return 0;
}
pub unsafe fn __assert_fail_4(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
