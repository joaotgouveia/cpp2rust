extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: i64,
    pub __clang_max_align_nonce2: f64,
}
impl Clone for max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub unsafe fn write_ulong_0(mut p: *mut u64) {
    (*p) = 42_u64;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: usize = 0_usize;
    (unsafe { write_ulong_0(((&mut x as *mut usize) as *mut usize).cast::<u64>()) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((x) == (42_usize)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"x == 42".as_ptr().cast_mut()).cast_const(),
                        (c"implicit_ptr_cast.c".as_ptr().cast_mut()).cast_const(),
                        9_u32,
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
