extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Kind_enum = u32;
pub const Kind_enum_KIND_NONE: Kind_enum = 0;
pub const Kind_enum_KIND_DONE: Kind_enum = 1;
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_0 {
    pub obj: *mut ::libc::c_void,
    pub code: i32,
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Event {
    pub kind: Kind_enum,
    pub handle: *mut ::libc::c_void,
    pub payload: anon_0,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut dummy: i32 = 0;
    let mut m1: Event = <Event>::default();
    m1.kind = Kind_enum_KIND_DONE;
    m1.handle = ((&mut dummy as *mut i32) as *mut ::libc::c_void);
    m1.payload.code = 42;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((m1.kind as u32) == ((Kind_enum_KIND_DONE as i32) as u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"m1.kind == KIND_DONE".as_ptr().cast_mut()).cast_const(),
                        (c"union_tagged_simple.c".as_ptr().cast_mut()).cast_const(),
                        25_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((m1.payload.code) == (42)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"m1.payload.code == 42".as_ptr().cast_mut()).cast_const(),
                        (c"union_tagged_simple.c".as_ptr().cast_mut()).cast_const(),
                        26_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut m2: Event = <Event>::default();
    m2.kind = Kind_enum_KIND_NONE;
    m2.handle = ((&mut dummy as *mut i32) as *mut ::libc::c_void);
    m2.payload.obj = ((&mut dummy as *mut i32) as *mut ::libc::c_void);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((m2.payload.obj) == ((&mut dummy as *mut i32) as *mut ::libc::c_void)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"m2.payload.obj == &dummy".as_ptr().cast_mut()).cast_const(),
                        (c"union_tagged_simple.c".as_ptr().cast_mut()).cast_const(),
                        32_u32,
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
