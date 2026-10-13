extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut n: i32 = 3;
    let mut zero: i32 = 0;
    let mut u: u32 = 4_u32;
    let mut ul: u64 = 5_u64;
    let mut ll: i64 = 6_i64;
    let mut ch: libc::c_char = (('a' as i32) as libc::c_char);
    if (n != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    if !(n != 0) {
        {
            &(::std::mem::size_of::<i32>());
            {
                {
                    (unsafe {
                        __assert_fail_0(
                            (c"false".as_ptr().cast_mut()).cast_const(),
                            (c"bool_condition_int_c.c".as_ptr().cast_mut()).cast_const(),
                            16_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
    }
    if (zero != 0) {
        {
            &(::std::mem::size_of::<i32>());
            {
                {
                    (unsafe {
                        __assert_fail_0(
                            (c"false".as_ptr().cast_mut()).cast_const(),
                            (c"bool_condition_int_c.c".as_ptr().cast_mut()).cast_const(),
                            19_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
    }
    if !(zero != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    if (u != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    if (ul != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    if (ll != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    if (ch != 0) {
        {
            &(::std::mem::size_of::<i32>());
            { {} }
        };
    }
    let mut loop_count: i32 = 0;
    let mut counter: i32 = 3;
    'loop_: while (counter != 0) {
        counter.prefix_dec();
        loop_count.prefix_inc();
    }
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((loop_count) == (3)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"loop_count == 3".as_ptr().cast_mut()).cast_const(),
                        (c"bool_condition_int_c.c".as_ptr().cast_mut()).cast_const(),
                        44_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut i: i32 = 5;
    'loop_: while (i != 0) {
        loop_count.prefix_inc();
        i.prefix_dec();
    }
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((loop_count) == (8)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"loop_count == 8".as_ptr().cast_mut()).cast_const(),
                        (c"bool_condition_int_c.c".as_ptr().cast_mut()).cast_const(),
                        49_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut t: i32 = if (n != 0) { 100 } else { 200 };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((t) == (100)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"t == 100".as_ptr().cast_mut()).cast_const(),
                        (c"bool_condition_int_c.c".as_ptr().cast_mut()).cast_const(),
                        52_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut t2: i32 = if (zero != 0) { 100 } else { 200 };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((t2) == (200)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"t2 == 200".as_ptr().cast_mut()).cast_const(),
                        (c"bool_condition_int_c.c".as_ptr().cast_mut()).cast_const(),
                        54_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut t7: i32 = (!(n != 0) as i32);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((t7) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"t7 == 0".as_ptr().cast_mut()).cast_const(),
                        (c"bool_condition_int_c.c".as_ptr().cast_mut()).cast_const(),
                        56_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut t8: i32 = (!(zero != 0) as i32);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((t8) == (1)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"t8 == 1".as_ptr().cast_mut()).cast_const(),
                        (c"bool_condition_int_c.c".as_ptr().cast_mut()).cast_const(),
                        58_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut b1: bool = (n != 0);
    {
        &(::std::mem::size_of::<i32>());
        {
            if b1 {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"b1".as_ptr().cast_mut()).cast_const(),
                        (c"bool_condition_int_c.c".as_ptr().cast_mut()).cast_const(),
                        61_u32,
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
