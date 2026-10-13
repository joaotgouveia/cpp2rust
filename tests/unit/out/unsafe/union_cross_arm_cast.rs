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
pub struct shape_a {
    pub code: u16,
    pub pad: [libc::c_char; 14],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct shape_b {
    pub code: u16,
    pub lo: u16,
    pub mid: u32,
    pub fill: [u8; 16],
    pub tail: u32,
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_0 {
    pub a: shape_a,
    pub b: shape_b,
    pub raw_: [libc::c_char; 64],
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Container {
    pub len: u32,
    pub u: anon_0,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: Container = <Container>::default();
    (unsafe {
        let ___s: *mut ::libc::c_void = ((&mut c as *mut Container) as *mut ::libc::c_void);
        let ___n: usize = ::std::mem::size_of::<Container>();
        memset_1(___s, 0, ___n)
    });
    c.u.a.code = 10_u16;
    c.len = (::std::mem::size_of::<shape_b>() as u32);
    (*(((&mut c.u.a as *mut shape_a) as *mut ::libc::c_void) as *mut shape_b)).tail =
        3735928559_u32;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c.u.b.tail) == (3735928559_u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"c.u.b.tail == 0xDEADBEEF".as_ptr().cast_mut()).cast_const(),
                        (c"union_cross_arm_cast.c".as_ptr().cast_mut()).cast_const(),
                        37_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c.u.b.code as i32) == (10)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"c.u.b.code == 10".as_ptr().cast_mut()).cast_const(),
                        (c"union_cross_arm_cast.c".as_ptr().cast_mut()).cast_const(),
                        38_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    c.u.b.lo = 8080_u16;
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*((&mut c.u.raw_ as *mut [libc::c_char; 64]) as *mut u8).offset((2) as isize))
                as i32)
                == (144)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"((unsigned char *)&c.u.raw)[2] == 0x90"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"union_cross_arm_cast.c".as_ptr().cast_mut()).cast_const(),
                        41_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*((&mut c.u.raw_ as *mut [libc::c_char; 64]) as *mut u8).offset((3) as isize))
                as i32)
                == (31)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"((unsigned char *)&c.u.raw)[3] == 0x1F"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"union_cross_arm_cast.c".as_ptr().cast_mut()).cast_const(),
                        42_u32,
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
pub unsafe fn memset_1(
    mut __s: *mut ::libc::c_void,
    mut __c: i32,
    mut __n: usize,
) -> *mut ::libc::c_void {
    unimplemented!()
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
