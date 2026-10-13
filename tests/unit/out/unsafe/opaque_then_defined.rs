extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct list {
    pub head: *mut node,
    pub size: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct node {
    pub value: i32,
    pub next: *mut node,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut n: node = node {
        value: 42,
        next: std::ptr::null_mut(),
    };
    let mut l: list = list {
        head: (&mut n as *mut node),
        size: 1,
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*l.head).value) == (42)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"l.head->value == 42".as_ptr().cast_mut()).cast_const(),
                        (c"opaque_then_defined.c".as_ptr().cast_mut()).cast_const(),
                        18_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((l.size) == (1)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"l.size == 1".as_ptr().cast_mut()).cast_const(),
                        (c"opaque_then_defined.c".as_ptr().cast_mut()).cast_const(),
                        19_u32,
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
