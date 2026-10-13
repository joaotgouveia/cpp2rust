extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fill_row_0(mut row: *mut libc::c_char, mut c: libc::c_char) {
    (*row.offset((0) as isize)) = c;
    (*row.offset((1) as isize)) = (('\0' as i32) as libc::c_char);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut grid: [[libc::c_char; 6]; 3] = [[(0 as libc::c_char); 6]; 3];
    let mut i: i32 = 0;
    'loop_: while ((((i) < (3)) as i32) != 0) {
        (unsafe {
            let _row: *mut libc::c_char = grid[(i) as usize].as_mut_ptr();
            let _c: libc::c_char = ((('a' as i32) + (i)) as libc::c_char);
            fill_row_0(_row, _c)
        });
        i.postfix_inc();
    }
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((grid[(0) as usize][(0) as usize] as i32) == ('a' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"grid[0][0] == \'a\'".as_ptr().cast_mut()).cast_const(),
                        (c"array_2d_default.c".as_ptr().cast_mut()).cast_const(),
                        13_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((grid[(1) as usize][(0) as usize] as i32) == ('b' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"grid[1][0] == \'b\'".as_ptr().cast_mut()).cast_const(),
                        (c"array_2d_default.c".as_ptr().cast_mut()).cast_const(),
                        14_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((grid[(2) as usize][(0) as usize] as i32) == ('c' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"grid[2][0] == \'c\'".as_ptr().cast_mut()).cast_const(),
                        (c"array_2d_default.c".as_ptr().cast_mut()).cast_const(),
                        15_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((grid[(1) as usize][(1) as usize] as i32) == ('\0' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"grid[1][1] == \'\\0\'".as_ptr().cast_mut()).cast_const(),
                        (c"array_2d_default.c".as_ptr().cast_mut()).cast_const(),
                        16_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    grid[(2) as usize][(5) as usize] = (('z' as i32) as libc::c_char);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((grid[(2) as usize][(5) as usize] as i32) == ('z' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"grid[2][5] == \'z\'".as_ptr().cast_mut()).cast_const(),
                        (c"array_2d_default.c".as_ptr().cast_mut()).cast_const(),
                        18_u32,
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
