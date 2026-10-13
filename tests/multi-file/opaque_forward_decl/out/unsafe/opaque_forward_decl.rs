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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct container {
    pub p: *mut opaque,
    pub x: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: container = container {
        p: std::ptr::null_mut(),
        x: 42,
    };
    (unsafe { touch_0((&mut c as *mut container)) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c.x) == (42)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"c.x == 42".as_ptr().cast_mut()).cast_const(),
                        (c"a.c".as_ptr().cast_mut()).cast_const(),
                        9_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c.p).is_null()) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"c.p == NULL".as_ptr().cast_mut()).cast_const(),
                        (c"a.c".as_ptr().cast_mut()).cast_const(),
                        10_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    return 0;
}
pub unsafe fn touch_0(mut c: *mut container) {
    &((*c).p);
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct opaque;
pub unsafe fn __assert_fail_1(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
