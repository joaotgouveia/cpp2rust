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
    pub hi: u32,
    pub fill: [libc::c_char; 8],
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_0 {
    pub a: shape_a,
    pub b: shape_b,
    pub raw_: [libc::c_char; 256],
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Container {
    pub view: anon_0,
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
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c.view.a.code as i32) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"c.view.a.code == 0".as_ptr().cast_mut()).cast_const(),
                        (c"union_memset_memcpy.c".as_ptr().cast_mut()).cast_const(),
                        30_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c.view.b.lo as i32) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"c.view.b.lo == 0".as_ptr().cast_mut()).cast_const(),
                        (c"union_memset_memcpy.c".as_ptr().cast_mut()).cast_const(),
                        31_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c.view.raw_[(0) as usize] as i32) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"c.view.raw[0] == 0".as_ptr().cast_mut()).cast_const(),
                        (c"union_memset_memcpy.c".as_ptr().cast_mut()).cast_const(),
                        32_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c.view.raw_[(255) as usize] as i32) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"c.view.raw[255] == 0".as_ptr().cast_mut()).cast_const(),
                        (c"union_memset_memcpy.c".as_ptr().cast_mut()).cast_const(),
                        33_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut src: [u8; 16] = [
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8,
    ];
    src[(0) as usize] = 2_u8;
    src[(2) as usize] = 80_u8;
    src[(3) as usize] = 0_u8;
    src[(4) as usize] = 127_u8;
    src[(5) as usize] = 0_u8;
    src[(6) as usize] = 0_u8;
    src[(7) as usize] = 1_u8;
    let mut len: usize = 16_usize;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((len) <= (::std::mem::size_of::<[libc::c_char; 256]>())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"len <= sizeof(c.view.raw)".as_ptr().cast_mut()).cast_const(),
                        (c"union_memset_memcpy.c".as_ptr().cast_mut()).cast_const(),
                        44_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    (unsafe {
        memcpy_3(
            ((&mut c.view.raw_ as *mut [libc::c_char; 256]) as *mut ::libc::c_void),
            (src.as_mut_ptr() as *const ::libc::c_void),
            len,
        )
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c.view.b.code as i32) == (2)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"c.view.b.code == 2".as_ptr().cast_mut()).cast_const(),
                        (c"union_memset_memcpy.c".as_ptr().cast_mut()).cast_const(),
                        47_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*((&mut c.view.b.lo as *mut u16) as *mut u8).offset((0) as isize)) as i32)
                == (80)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"((unsigned char *)&c.view.b.lo)[0] == 0x50"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"union_memset_memcpy.c".as_ptr().cast_mut()).cast_const(),
                        48_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    (unsafe {
        let ___s: *mut ::libc::c_void = ((&mut c as *mut Container) as *mut ::libc::c_void);
        let ___n: usize = ::std::mem::size_of::<Container>();
        memset_1(___s, 0, ___n)
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c.view.b.code as i32) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"c.view.b.code == 0".as_ptr().cast_mut()).cast_const(),
                        (c"union_memset_memcpy.c".as_ptr().cast_mut()).cast_const(),
                        51_u32,
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
pub unsafe fn memcpy_3(
    mut __dest: *mut ::libc::c_void,
    mut __src: *const ::libc::c_void,
    mut __n: usize,
) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
