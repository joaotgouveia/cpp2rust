extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct __fsid_t {
    pub __val: [i32; 2],
}
impl Clone for __fsid_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __fsid_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const u16,
    pub __ctype_tolower: *const i32,
    pub __ctype_toupper: *const i32,
    pub __names: [*const libc::c_char; 13],
}
impl Clone for __locale_struct {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __locale_struct {
    fn default() -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct point {
    pub x: i32,
    pub y: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut src: point = point { x: 3, y: 7 };
    let mut buf: [u8; 8] = [0_u8; 8];
    (unsafe {
        let ___dest: *mut ::libc::c_void = (buf.as_mut_ptr() as *mut ::libc::c_void);
        let ___n: usize = ::std::mem::size_of::<[u8; 8]>();
        memcpy_0(
            ___dest,
            ((&mut src as *mut point) as *const ::libc::c_void),
            ___n,
        )
    });
    let mut dst: point = <point>::default();
    (unsafe {
        let ___dest: *mut ::libc::c_void = ((&mut dst as *mut point) as *mut ::libc::c_void);
        let ___n: usize = ::std::mem::size_of::<point>();
        memcpy_0(___dest, (buf.as_mut_ptr() as *const ::libc::c_void), ___n)
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((dst.x) == (3)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"dst.x == 3".as_ptr().cast_mut()).cast_const(),
                        (c"memcpy_struct_bytes.c".as_ptr().cast_mut()).cast_const(),
                        19_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((dst.y) == (7)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"dst.y == 7".as_ptr().cast_mut()).cast_const(),
                        (c"memcpy_struct_bytes.c".as_ptr().cast_mut()).cast_const(),
                        20_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct __locale_data;
pub unsafe fn memcpy_0(
    mut __dest: *mut ::libc::c_void,
    mut __src: *const ::libc::c_void,
    mut __n: usize,
) -> *mut ::libc::c_void {
    unimplemented!()
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
