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
    let mut x: i64 = (-1_i32 as i64);
    let mut pp: anon_0 = <anon_0>::default();
    pp.as_signed = (&mut x as *mut i64);
    (*pp.as_unsigned) = 42_u64;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((x) == (42_i64)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"x == 42".as_ptr().cast_mut()).cast_const(),
                        (c"union_pointer_pun_writethrough.c".as_ptr().cast_mut()).cast_const(),
                        15_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_0 {
    pub as_unsigned: *mut u64,
    pub as_signed: *mut i64,
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
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
