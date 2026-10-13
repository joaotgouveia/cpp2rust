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
pub struct node_a {
    pub n: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct node_b {
    pub data: *mut ::libc::c_void,
    pub next: *mut node_b,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: node_a = node_a { n: 123 };
    let mut ptr: anon_0 = <anon_0>::default();
    ptr.to_a = (&mut a as *mut node_a);
    let mut out: *mut node_b = ptr.to_b;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((out as *mut ::libc::c_void) == ((&mut a as *mut node_a) as *mut ::libc::c_void))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"(void *)out == (void *)&a".as_ptr().cast_mut()).cast_const(),
                        (c"union_pointer_pun_address.c".as_ptr().cast_mut()).cast_const(),
                        24_u32,
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
    pub to_a: *mut node_a,
    pub to_b: *mut node_b,
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
