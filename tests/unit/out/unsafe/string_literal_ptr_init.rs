extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct label {
    pub name: *const libc::c_char,
    pub probe: Option<unsafe fn() -> i32>,
    pub mask: i32,
}
pub unsafe fn probe_two_0() -> i32 {
    return 1;
}
pub static mut table_1: std::cell::LazyCell<[label; 2]> = std::cell::LazyCell::new(|| unsafe {
    [
        label {
            name: ((c"first").as_ptr().cast_mut()).cast_const(),
            probe: None,
            mask: ((1) << (4)),
        },
        label {
            name: ((c"second").as_ptr().cast_mut()).cast_const(),
            probe: (Some(probe_two_0)),
            mask: ((1) << (5)),
        },
    ]
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
        {
            if ((((unsafe {
                ((*std::cell::LazyCell::force_mut(&mut *&raw mut table_1))[(1) as usize].probe)
                    .unwrap()()
            }) == (1)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"table[1].probe() == 1".as_ptr().cast_mut()).cast_const(),
                        (c"string_literal_ptr_init.c".as_ptr().cast_mut()).cast_const(),
                        26_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
    let mut tail: *const libc::c_char = (&mut (*c"ab.cd".as_ptr().cast_mut().offset((2) as isize))
        as *mut libc::c_char)
        .cast_const();
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*tail.offset((0) as isize)) as i32) == ('.' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"tail[0] == \'.\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literal_ptr_init.c".as_ptr().cast_mut()).cast_const(),
                        30_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*tail.offset((1) as isize)) as i32) == ('c' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"tail[1] == \'c\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literal_ptr_init.c".as_ptr().cast_mut()).cast_const(),
                        31_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*tail.offset((2) as isize)) as i32) == ('d' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"tail[2] == \'d\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literal_ptr_init.c".as_ptr().cast_mut()).cast_const(),
                        32_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut have: i32 = 0;
    let mut p: *mut ::libc::c_void = if (have != 0) {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut table_1))[(0) as usize].name
            as *mut ::libc::c_void)
    } else {
        (c"".as_ptr().cast_mut() as *mut ::libc::c_void)
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*(p as *const libc::c_char).offset((0) as isize)) as i32) == ('\0' as i32))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"((const char *)p)[0] == \'\\0\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literal_ptr_init.c".as_ptr().cast_mut()).cast_const(),
                        36_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    have = 1;
    p = if (have != 0) {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut table_1))[(0) as usize].name
            as *mut ::libc::c_void)
    } else {
        (c"".as_ptr().cast_mut() as *mut ::libc::c_void)
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*(p as *const libc::c_char).offset((0) as isize)) as i32) == ('f' as i32))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"((const char *)p)[0] == \'f\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literal_ptr_init.c".as_ptr().cast_mut()).cast_const(),
                        39_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    return 0;
}
pub unsafe fn __assert_fail_2(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const table_1);
}
