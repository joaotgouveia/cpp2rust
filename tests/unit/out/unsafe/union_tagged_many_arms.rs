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
pub type Tag_enum = u32;
pub const Tag_enum_T_NUM_S: Tag_enum = 0;
pub const Tag_enum_T_NUM_U: Tag_enum = 1;
pub const Tag_enum_T_TEXT: Tag_enum = 2;
pub const Tag_enum_T_FLOAT: Tag_enum = 3;
pub const Tag_enum_T_REF: Tag_enum = 4;
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_0 {
    pub text: *const libc::c_char,
    pub handle: *mut ::libc::c_void,
    pub signed_n: i64,
    pub unsigned_n: u64,
    pub f: f64,
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Slot {
    pub tag: Tag_enum,
    pub payload: anon_0,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Slot = <Slot>::default();
    a.tag = Tag_enum_T_NUM_S;
    a.payload.signed_n = (-7_i32 as i64);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((a.payload.signed_n) == (-7_i32 as i64)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"a.payload.signed_n == -7".as_ptr().cast_mut()).cast_const(),
                        (c"union_tagged_many_arms.c".as_ptr().cast_mut()).cast_const(),
                        28_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut b: Slot = <Slot>::default();
    b.tag = Tag_enum_T_NUM_U;
    b.payload.unsigned_n = 3735928559_u64;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((b.payload.unsigned_n) == (3735928559_u64)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"b.payload.unsigned_n == 0xDEADBEEFu".as_ptr().cast_mut()).cast_const(),
                        (c"union_tagged_many_arms.c".as_ptr().cast_mut()).cast_const(),
                        33_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut c: Slot = <Slot>::default();
    c.tag = Tag_enum_T_TEXT;
    c.payload.text = (c"hello".as_ptr().cast_mut()).cast_const();
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*c.payload.text.offset((0) as isize)) as i32) == ('h' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"c.payload.text[0] == \'h\'".as_ptr().cast_mut()).cast_const(),
                        (c"union_tagged_many_arms.c".as_ptr().cast_mut()).cast_const(),
                        38_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut d: Slot = <Slot>::default();
    d.tag = Tag_enum_T_FLOAT;
    d.payload.f = 1.5E+0;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((d.payload.f) == (1.5E+0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"d.payload.f == 1.5".as_ptr().cast_mut()).cast_const(),
                        (c"union_tagged_many_arms.c".as_ptr().cast_mut()).cast_const(),
                        43_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut x: i32 = 0;
    let mut e: Slot = <Slot>::default();
    e.tag = Tag_enum_T_REF;
    e.payload.handle = ((&mut x as *mut i32) as *mut ::libc::c_void);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((e.payload.handle) == ((&mut x as *mut i32) as *mut ::libc::c_void)) as i32) != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"e.payload.handle == &x".as_ptr().cast_mut()).cast_const(),
                        (c"union_tagged_many_arms.c".as_ptr().cast_mut()).cast_const(),
                        49_u32,
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
