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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct header {
    pub tag: i32,
    pub size: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct view {
    pub tag: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let text: [libc::c_char; 3] = std::mem::transmute(*b"hi\0");
    let mut cp: *const libc::c_char = text.as_ptr();
    let mut u: *mut u8 = (cp as *mut u8);
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*u.offset((0) as isize)) as i32) == ('h' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"u[0] == \'h\'".as_ptr().cast_mut()).cast_const(),
                        (c"ptr_cast_init.c".as_ptr().cast_mut()).cast_const(),
                        17_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*u.offset((1) as isize)) as i32) == ('i' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"u[1] == \'i\'".as_ptr().cast_mut()).cast_const(),
                        (c"ptr_cast_init.c".as_ptr().cast_mut()).cast_const(),
                        18_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut h: header = header { tag: 7, size: 32 };
    let mut hp: *mut header = (&mut h as *mut header);
    let mut v: *mut view = (hp as *mut view);
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*v).tag) == (7)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"v->tag == 7".as_ptr().cast_mut()).cast_const(),
                        (c"ptr_cast_init.c".as_ptr().cast_mut()).cast_const(),
                        23_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut data: [libc::c_char; 3] = std::mem::transmute(*b"hi\0");
    let mut vp: *mut ::libc::c_void = (data.as_mut_ptr() as *mut ::libc::c_void);
    let mut n: i32 = 2;
    let mut sel: *mut libc::c_char = (if ((((n) < (100)) as i32) != 0) {
        vp
    } else {
        (0 as *mut ::libc::c_void)
    } as *mut libc::c_char);
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((sel).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"sel != 0".as_ptr().cast_mut()).cast_const(),
                        (c"ptr_cast_init.c".as_ptr().cast_mut()).cast_const(),
                        29_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*sel.offset((0) as isize)) as i32) == ('h' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"sel[0] == \'h\'".as_ptr().cast_mut()).cast_const(),
                        (c"ptr_cast_init.c".as_ptr().cast_mut()).cast_const(),
                        30_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    n = 200;
    sel = (if ((((n) < (100)) as i32) != 0) {
        vp
    } else {
        (0 as *mut ::libc::c_void)
    } as *mut libc::c_char);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((sel).is_null()) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"sel == 0".as_ptr().cast_mut()).cast_const(),
                        (c"ptr_cast_init.c".as_ptr().cast_mut()).cast_const(),
                        33_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    return 0;
}
pub unsafe fn __assert_fail_0(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
