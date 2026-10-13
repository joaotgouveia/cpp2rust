extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
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
pub struct Inner {
    pub a: i32,
    pub b: i32,
}
pub unsafe fn sum_inner_0(mut i: *mut Inner) -> i32 {
    return (((*i).a) + ((*i).b));
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_1 {
    pub inner: Inner,
    pub raw_: [libc::c_char; 16],
}
impl Default for anon_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer {
    pub u: anon_1,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut standalone: Inner = <Inner>::default();
    standalone.a = 3;
    standalone.b = 4;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { sum_inner_0((&mut standalone as *mut Inner)) }) == (7)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"sum_inner(&standalone) == 7".as_ptr().cast_mut()).cast_const(),
                        (c"union_struct_dual_use.c".as_ptr().cast_mut()).cast_const(),
                        22_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut outer: Outer = <Outer>::default();
    (unsafe {
        let ___s: *mut ::libc::c_void = ((&mut outer as *mut Outer) as *mut ::libc::c_void);
        let ___n: usize = ::std::mem::size_of::<Outer>();
        memset_3(___s, 0, ___n)
    });
    outer.u.inner.a = 3;
    outer.u.inner.b = 4;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { sum_inner_0((&mut outer.u.inner as *mut Inner)) }) == (7)) as i32) != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"sum_inner(&outer.u.inner) == 7".as_ptr().cast_mut()).cast_const(),
                        (c"union_struct_dual_use.c".as_ptr().cast_mut()).cast_const(),
                        28_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((outer.u.raw_[(0) as usize] as u8) as i32) == (3)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"(unsigned char)outer.u.raw[0] == 3".as_ptr().cast_mut()).cast_const(),
                        (c"union_struct_dual_use.c".as_ptr().cast_mut()).cast_const(),
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
            if (((((outer.u.raw_[(4) as usize] as u8) as i32) == (4)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"(unsigned char)outer.u.raw[4] == 4".as_ptr().cast_mut()).cast_const(),
                        (c"union_struct_dual_use.c".as_ptr().cast_mut()).cast_const(),
                        31_u32,
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
pub unsafe fn __assert_fail_2(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn memset_3(
    mut __s: *mut ::libc::c_void,
    mut __c: i32,
    mut __n: usize,
) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
