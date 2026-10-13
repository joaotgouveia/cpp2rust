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
pub struct record {
    pub code: u16,
    pub pad: [libc::c_char; 14],
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
pub struct inner {
    pub view: anon_0,
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_1 {
    pub h: record,
    pub nested: inner,
}
impl Default for anon_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer {
    pub kind: i32,
    pub level: i32,
    pub variant: i32,
    pub len: u32,
    pub body: anon_1,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut ex: Outer = <Outer>::default();
    (unsafe {
        let ___s: *mut ::libc::c_void = ((&mut ex as *mut Outer) as *mut ::libc::c_void);
        let ___n: usize = ::std::mem::size_of::<Outer>();
        memset_2(___s, 0, ___n)
    });
    ex.kind = 2;
    ex.level = 1;
    ex.variant = 6;
    ex.len = (::std::mem::size_of::<record>() as u32);
    ex.body.h.code = 2_u16;
    ex.body.h.pad[(0) as usize] = (('X' as i32) as libc::c_char);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((ex.body.h.code as i32) == (2)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"ex.body.h.code == 2".as_ptr().cast_mut()).cast_const(),
                        (c"union_nested.c".as_ptr().cast_mut()).cast_const(),
                        39_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((ex.body.h.pad[(0) as usize] as i32) == ('X' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"ex.body.h.pad[0] == \'X\'".as_ptr().cast_mut()).cast_const(),
                        (c"union_nested.c".as_ptr().cast_mut()).cast_const(),
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
            if ((((ex.body.nested.view.h.code as i32) == (2)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"ex.body.nested.view.h.code == 2".as_ptr().cast_mut()).cast_const(),
                        (c"union_nested.c".as_ptr().cast_mut()).cast_const(),
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
pub unsafe fn memset_2(
    mut __s: *mut ::libc::c_void,
    mut __c: i32,
    mut __n: usize,
) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn __assert_fail_3(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
