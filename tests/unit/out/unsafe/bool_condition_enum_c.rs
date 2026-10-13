extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Code = u32;
pub const Code_CODE_OK: Code = 0;
pub const Code_CODE_ERR: Code = 1;
pub const Code_CODE_FATAL: Code = 2;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut code: Code = Code_CODE_OK;
    let mut err: Code = Code_CODE_ERR;
    if (code != 0) {
        {
            &(::std::mem::size_of::<i32>());
            {
                {
                    (unsafe {
                        __assert_fail_0(
                            (c"false".as_ptr().cast_mut()).cast_const(),
                            (c"bool_condition_enum_c.c".as_ptr().cast_mut()).cast_const(),
                            11_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
    }
    if !(code != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    if (err != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    if !(err != 0) {
        {
            &(::std::mem::size_of::<i32>());
            {
                {
                    (unsafe {
                        __assert_fail_0(
                            (c"false".as_ptr().cast_mut()).cast_const(),
                            (c"bool_condition_enum_c.c".as_ptr().cast_mut()).cast_const(),
                            20_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
    }
    let mut t9: i32 = (!(code != 0) as i32);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((t9) == (1)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"t9 == 1".as_ptr().cast_mut()).cast_const(),
                        (c"bool_condition_enum_c.c".as_ptr().cast_mut()).cast_const(),
                        24_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut b4: bool = (code != 0);
    {
        &(::std::mem::size_of::<i32>());
        {
            if !(b4) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"!b4".as_ptr().cast_mut()).cast_const(),
                        (c"bool_condition_enum_c.c".as_ptr().cast_mut()).cast_const(),
                        27_u32,
                        c"int main()".as_ptr(),
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
