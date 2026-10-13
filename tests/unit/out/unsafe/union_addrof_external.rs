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
pub struct record {
    pub code: u16,
    pub lo: u16,
    pub hi: u32,
    pub pad: [libc::c_char; 8],
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_0 {
    pub h: record,
    pub raw_: [libc::c_char; 128],
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
pub unsafe fn fill_1(mut out: *mut ::libc::c_void, mut cap: usize) {
    let mut src: [u8; 16] = [
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8,
    ];
    src[(0) as usize] = 2_u8;
    src[(1) as usize] = 0_u8;
    src[(2) as usize] = 0_u8;
    src[(3) as usize] = 80_u8;
    src[(4) as usize] = 127_u8;
    src[(5) as usize] = 0_u8;
    src[(6) as usize] = 0_u8;
    src[(7) as usize] = 1_u8;
    let mut n: usize = (if ((((::std::mem::size_of::<[u8; 16]>()) < (cap)) as i32) != 0) {
        (::std::mem::size_of::<[u8; 16]>() as u64)
    } else {
        (cap as u64)
    } as usize);
    (unsafe { memcpy_2(out, (src.as_mut_ptr() as *const ::libc::c_void), n) });
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
        memset_3(___s, 0, ___n)
    });
    (unsafe {
        let _out: *mut ::libc::c_void = ((&mut c.view as *mut anon_0) as *mut ::libc::c_void);
        let _cap: usize = ::std::mem::size_of::<anon_0>();
        fill_1(_out, _cap)
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c.view.h.code as i32) == (2)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"c.view.h.code == 2".as_ptr().cast_mut()).cast_const(),
                        (c"union_addrof_external.c".as_ptr().cast_mut()).cast_const(),
                        40_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*((&mut c.view.h.lo as *mut u16) as *mut u8).offset((0) as isize)) as i32)
                == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"((unsigned char *)&c.view.h.lo)[0] == 0x00"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"union_addrof_external.c".as_ptr().cast_mut()).cast_const(),
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
            if (((((*((&mut c.view.h.lo as *mut u16) as *mut u8).offset((1) as isize)) as i32)
                == (80)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"((unsigned char *)&c.view.h.lo)[1] == 0x50"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"union_addrof_external.c".as_ptr().cast_mut()).cast_const(),
                        42_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c.view.raw_[(0) as usize] as i32) == (2)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"c.view.raw[0] == 2".as_ptr().cast_mut()).cast_const(),
                        (c"union_addrof_external.c".as_ptr().cast_mut()).cast_const(),
                        44_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((c.view.raw_[(3) as usize] as u8) as i32) == (80)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"(unsigned char)c.view.raw[3] == 0x50".as_ptr().cast_mut()).cast_const(),
                        (c"union_addrof_external.c".as_ptr().cast_mut()).cast_const(),
                        45_u32,
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
pub unsafe fn memcpy_2(
    mut __dest: *mut ::libc::c_void,
    mut __src: *const ::libc::c_void,
    mut __n: usize,
) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn memset_3(
    mut __s: *mut ::libc::c_void,
    mut __c: i32,
    mut __n: usize,
) -> *mut ::libc::c_void {
    unimplemented!()
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
