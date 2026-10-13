extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut value_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 5 });
pub unsafe fn param_shadow_1(mut value: i32) -> i32 {
    return ((value) + (1));
}
pub unsafe fn local_shadow_2() -> i32 {
    let mut value: i32 = 99;
    return value;
}
pub unsafe fn read_global_3() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut value_0));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { param_shadow_1(10) }) == (11)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"param_shadow(10) == 11".as_ptr().cast_mut()).cast_const(),
                        (c"static_shadow.c".as_ptr().cast_mut()).cast_const(),
                        15_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { local_shadow_2() }) == (99)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"local_shadow() == 99".as_ptr().cast_mut()).cast_const(),
                        (c"static_shadow.c".as_ptr().cast_mut()).cast_const(),
                        16_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { read_global_3() }) == (5)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"read_global() == 5".as_ptr().cast_mut()).cast_const(),
                        (c"static_shadow.c".as_ptr().cast_mut()).cast_const(),
                        17_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
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
    std::cell::LazyCell::force(&*&raw const value_0);
}
