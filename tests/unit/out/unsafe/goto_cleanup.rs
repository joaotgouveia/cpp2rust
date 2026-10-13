extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn early_0(mut n: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    let mut intentionally_const_var: i32 = 0_i32;
    goto_block!({
        '__entry: {
            ret = 0;
            if ((((n) < (0)) as i32) != 0) {
                ret = -1_i32;
                goto!('out);
            }
            ret = 100;
            intentionally_const_var = 22;
        }
        'out: {
            return (((ret) + (intentionally_const_var)) - (intentionally_const_var));
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn from_loop_1(mut n: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    goto_block!({
        '__entry: {
            ret = 0;
            let mut i: i32 = 0;
            'loop_: while ((((i) < (n)) as i32) != 0) {
                if ((((i) == (3)) as i32) != 0) {
                    ret = 7;
                    goto!('out);
                }
                ret += i;
                i.postfix_inc();
            }
            ret = 999;
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn from_switch_2(mut n: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    goto_block!({
        '__entry: {
            ret = 0;
            'switch: {
                match { n } {
                    __v if __v == 1 => {
                        ret = 10;
                        goto!('out);
                    }
                    _ => {
                        ret = 20;
                        break 'switch;
                    }
                }
            };
            ret = 999;
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct wrapper {
    pub item: *mut i32,
}
pub unsafe fn via_pointer_3(mut w: *mut wrapper, mut fail: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    let mut item: *mut i32 = std::ptr::null_mut();
    goto_block!({
        '__entry: {
            ret = 0;
            item = (*w).item;
            if (fail != 0) {
                ret = -1_i32;
                goto!('out);
            }
            ret = (*item);
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn via_arrays_4(mut fail: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    let mut remain: [u8; 4] = [0_u8; 4];
    let mut name: [libc::c_char; 5] = [(0 as libc::c_char); 5];
    goto_block!({
        '__entry: {
            ret = 0;
            remain = [0_u8, 0_u8, 0_u8, 0_u8];
            name = std::mem::transmute(*b"wxyz\0");
            if (fail != 0) {
                ret = -1_i32;
                goto!('out);
            }
            remain[(1) as usize] = 9_u8;
            ret = ((((remain[(0) as usize] as i32) + (remain[(1) as usize] as i32))
                + (((name[(0) as usize] as i32) == ('w' as i32)) as i32))
                + (((name[(4) as usize] as i32) == ('\0' as i32)) as i32));
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
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
            if ((((unsafe { early_0(-1_i32) }) == (-1_i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_5(
                        (c"early(-1) == -1".as_ptr().cast_mut()).cast_const(),
                        (c"goto_cleanup.c".as_ptr().cast_mut()).cast_const(),
                        75_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { early_0(5) }) == (100)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_5(
                        (c"early(5) == 100".as_ptr().cast_mut()).cast_const(),
                        (c"goto_cleanup.c".as_ptr().cast_mut()).cast_const(),
                        76_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { from_loop_1(2) }) == (999)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_5(
                        (c"from_loop(2) == 999".as_ptr().cast_mut()).cast_const(),
                        (c"goto_cleanup.c".as_ptr().cast_mut()).cast_const(),
                        77_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { from_loop_1(10) }) == (7)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_5(
                        (c"from_loop(10) == 7".as_ptr().cast_mut()).cast_const(),
                        (c"goto_cleanup.c".as_ptr().cast_mut()).cast_const(),
                        78_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { from_switch_2(1) }) == (10)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_5(
                        (c"from_switch(1) == 10".as_ptr().cast_mut()).cast_const(),
                        (c"goto_cleanup.c".as_ptr().cast_mut()).cast_const(),
                        79_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { from_switch_2(2) }) == (999)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_5(
                        (c"from_switch(2) == 999".as_ptr().cast_mut()).cast_const(),
                        (c"goto_cleanup.c".as_ptr().cast_mut()).cast_const(),
                        80_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut value: i32 = 42;
    let mut w: wrapper = wrapper {
        item: (&mut value as *mut i32),
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { via_pointer_3((&mut w as *mut wrapper), 0) }) == (42)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_5(
                        (c"via_pointer(&w, 0) == 42".as_ptr().cast_mut()).cast_const(),
                        (c"goto_cleanup.c".as_ptr().cast_mut()).cast_const(),
                        83_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { via_pointer_3((&mut w as *mut wrapper), 1) }) == (-1_i32)) as i32) != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_5(
                        (c"via_pointer(&w, 1) == -1".as_ptr().cast_mut()).cast_const(),
                        (c"goto_cleanup.c".as_ptr().cast_mut()).cast_const(),
                        84_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { via_arrays_4(0) }) == (11)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_5(
                        (c"via_arrays(0) == 11".as_ptr().cast_mut()).cast_const(),
                        (c"goto_cleanup.c".as_ptr().cast_mut()).cast_const(),
                        85_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { via_arrays_4(1) }) == (-1_i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_5(
                        (c"via_arrays(1) == -1".as_ptr().cast_mut()).cast_const(),
                        (c"goto_cleanup.c".as_ptr().cast_mut()).cast_const(),
                        86_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    return 0;
}
pub unsafe fn __assert_fail_5(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
