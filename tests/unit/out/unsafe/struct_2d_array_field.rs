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
pub struct table {
    pub rows: [[libc::c_char; 10]; 3],
    pub count: usize,
}
pub static mut T1_0: std::cell::LazyCell<table> = std::cell::LazyCell::new(|| unsafe {
    table {
        rows: [std::mem::transmute(*b"alpha\0\0\0\0\0"), [0; 10], [0; 10]],
        count: 1_usize,
    }
});
pub static mut T2_1: std::cell::LazyCell<table> = std::cell::LazyCell::new(|| unsafe {
    table {
        rows: [
            std::mem::transmute(*b"alpha\0\0\0\0\0"),
            std::mem::transmute(*b"beta\0\0\0\0\0\0"),
            [0; 10],
        ],
        count: 2_usize,
    }
});
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
    let mut local: table = table {
        rows: [
            std::mem::transmute(*b"one\0\0\0\0\0\0\0"),
            std::mem::transmute(*b"two\0\0\0\0\0\0\0"),
            std::mem::transmute(*b"three\0\0\0\0\0"),
        ],
        count: 3_usize,
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strcmp_2(
                    (local.rows[(2) as usize].as_mut_ptr()).cast_const(),
                    (c"three".as_ptr().cast_mut()).cast_const(),
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"strcmp(local.rows[2], \"three\") == 0".as_ptr().cast_mut()).cast_const(),
                        (c"struct_2d_array_field.c".as_ptr().cast_mut()).cast_const(),
                        24_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    local.rows[(1) as usize][(0) as usize] = (('T' as i32) as libc::c_char);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strcmp_2(
                    (local.rows[(1) as usize].as_mut_ptr()).cast_const(),
                    (c"Two".as_ptr().cast_mut()).cast_const(),
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"strcmp(local.rows[1], \"Two\") == 0".as_ptr().cast_mut()).cast_const(),
                        (c"struct_2d_array_field.c".as_ptr().cast_mut()).cast_const(),
                        26_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strcmp_2(
                    (local.rows[(0) as usize].as_mut_ptr()).cast_const(),
                    (c"one".as_ptr().cast_mut()).cast_const(),
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"strcmp(local.rows[0], \"one\") == 0".as_ptr().cast_mut()).cast_const(),
                        (c"struct_2d_array_field.c".as_ptr().cast_mut()).cast_const(),
                        27_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut p: *const libc::c_char = (local.rows[(2) as usize].as_mut_ptr()).cast_const();
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*p.offset((0) as isize)) as i32) == ('t' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"p[0] == \'t\'".as_ptr().cast_mut()).cast_const(),
                        (c"struct_2d_array_field.c".as_ptr().cast_mut()).cast_const(),
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
            if ((((local.count) == (3_usize)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"local.count == 3".as_ptr().cast_mut()).cast_const(),
                        (c"struct_2d_array_field.c".as_ptr().cast_mut()).cast_const(),
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
pub unsafe fn strcmp_2(mut __s1: *const libc::c_char, mut __s2: *const libc::c_char) -> i32 {
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
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const T1_0);
    std::cell::LazyCell::force(&*&raw const T2_1);
}
