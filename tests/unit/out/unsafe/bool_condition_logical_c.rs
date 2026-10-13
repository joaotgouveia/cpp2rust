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
pub type Code = u32;
pub const Code_CODE_OK: Code = 0;
pub const Code_CODE_ERR: Code = 1;
pub const Code_CODE_FATAL: Code = 2;
pub static mut side_effect_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn observe_1(mut v: i32) -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut side_effect_0)).prefix_inc();
    return v;
}
pub unsafe fn returns_one_2() -> i32 {
    return 1;
}
pub unsafe fn returns_zero_3() -> i32 {
    return 0;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut n: i32 = 3;
    let mut zero: i32 = 0;
    let mut storage: i32 = 7;
    let mut p: *mut i32 = (&mut storage as *mut i32);
    let mut np: *mut i32 = std::ptr::null_mut();
    let mut u: u32 = 4_u32;
    let mut code: Code = Code_CODE_OK;
    if ((((n != 0) && (!(p).is_null())) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    if ((((n != 0) && (!(np).is_null())) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            {
                {
                    (unsafe {
                        __assert_fail_4(
                            (c"false".as_ptr().cast_mut()).cast_const(),
                            (c"bool_condition_logical_c.c".as_ptr().cast_mut()).cast_const(),
                            29_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
    }
    if ((((zero != 0) || (!(p).is_null())) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    if ((((zero != 0) || (!(np).is_null())) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            {
                {
                    (unsafe {
                        __assert_fail_4(
                            (c"false".as_ptr().cast_mut()).cast_const(),
                            (c"bool_condition_logical_c.c".as_ptr().cast_mut()).cast_const(),
                            35_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
    }
    if ((((((((((n != 0) && (u != 0)) as i32) != 0) && (!(p).is_null())) as i32) != 0)
        && ((((code as u32) == ((Code_CODE_OK as i32) as u32)) as i32) != 0)) as i32)
        != 0)
    {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    (*std::cell::LazyCell::force_mut(&mut *&raw mut side_effect_0)) = 0;
    if ((((zero != 0) && ((unsafe { observe_1(1) }) != 0)) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            {
                {
                    (unsafe {
                        __assert_fail_4(
                            (c"false".as_ptr().cast_mut()).cast_const(),
                            (c"bool_condition_logical_c.c".as_ptr().cast_mut()).cast_const(),
                            43_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
    }
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((*std::cell::LazyCell::force_mut(&mut *&raw mut side_effect_0)) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"side_effect == 0".as_ptr().cast_mut()).cast_const(),
                        (c"bool_condition_logical_c.c".as_ptr().cast_mut()).cast_const(),
                        45_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    if ((((n != 0) || ((unsafe { observe_1(1) }) != 0)) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((*std::cell::LazyCell::force_mut(&mut *&raw mut side_effect_0)) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"side_effect == 0".as_ptr().cast_mut()).cast_const(),
                        (c"bool_condition_logical_c.c".as_ptr().cast_mut()).cast_const(),
                        50_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut x: i32 = 5;
    let mut y: i32 = 3;
    let mut flags: u32 = 2_u32;
    if (((((((x) > (y)) as i32) != 0) || (((flags) & (1_u32)) != 0)) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    if (((((((x) < (y)) as i32) != 0) || (((flags) & (1_u32)) != 0)) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            {
                {
                    (unsafe {
                        __assert_fail_4(
                            (c"false".as_ptr().cast_mut()).cast_const(),
                            (c"bool_condition_logical_c.c".as_ptr().cast_mut()).cast_const(),
                            59_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
    }
    let mut a: u32 = 1_u32;
    let mut b: u32 = 2_u32;
    let mut c: u32 = 3_u32;
    if (((((((a) != (c)) as i32) != 0) && ((((b) != (c)) as i32) != 0)) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    let mut s: i32 = -1_i32;
    if ((((((!((p).is_null())) as i32) != 0) && ((((s) < (0)) as i32) != 0)) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    let mut k: u32 = 2_u32;
    let mut done: bool = (0 != 0);
    if (((((((k) > (1_u32)) as i32) != 0) || (!(done))) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    if (((((((x) > (y)) as i32) != 0) || (((flags) & (4_u32)) != 0)) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    let mut ull: u64 = 7_u64;
    if ((((((!((p).is_null())) as i32) != 0) && (ull != 0)) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    if (((((((x) > (y)) as i32) != 0) && (ull != 0)) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    let mut mask: i64 = (((1_i64) << (4)) | ((1_i64) << (5)));
    let mut bits: i64 = ((1_i64) << (4));
    if (((((((n) != (0)) as i32) != 0) && (((bits) & (mask)) != 0)) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    if (((((((n) != (0)) as i32) != 0) || (((bits) & (256_i64)) != 0)) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    let mut cp: *const libc::c_char = (c"hi".as_ptr().cast_mut()).cast_const();
    let mut cnp: *const libc::c_char = std::ptr::null();
    if (((((((x) > (y)) as i32) != 0) && (!(cp).is_null())) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    if (((((((x) < (y)) as i32) != 0) || (!(cnp).is_null())) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            {
                {
                    (unsafe {
                        __assert_fail_4(
                            (c"false".as_ptr().cast_mut()).cast_const(),
                            (c"bool_condition_logical_c.c".as_ptr().cast_mut()).cast_const(),
                            107_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
    }
    if (((((((x) > (y)) as i32) != 0) && ((((n != 0) && (!(cp).is_null())) as i32) != 0)) as i32)
        != 0)
    {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    if (((((((x) > (y)) as i32) != 0) && ((unsafe { returns_one_2() }) != 0)) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    if (((((((x) > (y)) as i32) != 0) && (!((unsafe { returns_zero_3() }) != 0))) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    if (((((((x) < (y)) as i32) != 0) || ((unsafe { returns_one_2() }) != 0)) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    if (((((((x) < (y)) as i32) != 0) || (!((unsafe { returns_one_2() }) != 0))) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            {
                {
                    (unsafe {
                        __assert_fail_4(
                            (c"false".as_ptr().cast_mut()).cast_const(),
                            (c"bool_condition_logical_c.c".as_ptr().cast_mut()).cast_const(),
                            123_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
    }
    if (((((((((!((p).is_null())) as i32) != 0) && ((unsafe { returns_one_2() }) != 0)) as i32)
        != 0)
        && ((((n) != (0)) as i32) != 0)) as i32)
        != 0)
    {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    return 0;
}
pub unsafe fn __assert_fail_4(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const side_effect_0);
}
