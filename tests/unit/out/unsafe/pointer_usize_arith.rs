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
pub type std_byte = u8;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut arr: [i32; 8] = [10, 11, 12, 13, 14, 15, 16, 17];
    let mut p: *mut i32 = arr.as_mut_ptr();
    let mut q: *mut i32 = p.offset((1) as isize);
    (if ((*q) == (11)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"*q == 11".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut r: *mut i32 = p.offset((3) as isize);
    (if ((*r) == (13)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"*r == 13".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut s: *mut i32 = r.offset(-((2) as isize));
    (if ((*s) == (11)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"*s == 11".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut diff: i64 = ((r as usize - p as usize) / ::std::mem::size_of::<i32>()) as i64;
    (if ((diff) == (3_i64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"diff == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut idx: usize =
        ((((r as usize - p as usize) / ::std::mem::size_of::<i32>()) as i64) as usize);
    (if ((idx) == (3_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"idx == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut q2: *mut i32 = p;
    q2.prefix_inc();
    (if ((*q2) == (11)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"*q2 == 11".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    q2.postfix_inc();
    (if ((*q2) == (12)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"*q2 == 12".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    q2.prefix_dec();
    (if ((*q2) == (11)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"*q2 == 11".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    q2.postfix_dec();
    (if ((*q2) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"*q2 == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((q2) == (p)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"q2 == p".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut q3: *mut i32 = p;
    q3 = (q3).wrapping_add((4 as i32) as usize);
    (if ((*q3) == (14)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"*q3 == 14".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    q3 = (q3).wrapping_sub((2 as i32) as usize);
    (if ((*q3) == (12)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"*q3 == 12".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut step: usize = 2_usize;
    let mut q4: *mut i32 = p.offset((step) as isize);
    (if ((*q4) == (12)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"*q4 == 12".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut v: i32 = (*p.offset((3) as isize));
    (if ((v) == (13)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"v == 13".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut v2: i32 = (*(p.offset((4) as isize)));
    (if ((v2) == (14)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"v2 == 14".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*(p.offset((5) as isize))) = 99;
    (if ((*p.offset((5) as isize)) == (99)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"p[5] == 99".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((arr[(5) as usize]) == (99)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"arr[5] == 99".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut end: *mut i32 = p.offset((8) as isize);
    let mut sum: i32 = 0;
    let mut it: *mut i32 = p;
    'loop_: while ((it) != (end)) {
        sum += (*it);
        it.prefix_inc();
    }
    (if ((sum) == ((((((((10) + (11)) + (12)) + (13)) + (14)) + (99)) + (16)) + (17))) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"sum == 10 + 11 + 12 + 13 + 14 + 99 + 16 + 17".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut bytes: [u8; 8] = [0_u8, 1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8];
    let mut bp: *mut u8 = bytes.as_mut_ptr();
    let mut bq: *mut u8 = bp.offset((4) as isize);
    (if (((*bq) as i32) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"*bq == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut bdiff: i64 = ((bq as usize - bp as usize) / ::std::mem::size_of::<u8>()) as i64;
    (if ((bdiff) == (4_i64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"bdiff == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut cp: *const i32 = (arr.as_mut_ptr()).cast_const();
    let mut cq: *const i32 = cp.offset((2) as isize);
    (if ((*cq) == (12)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"*cq == 12".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut cdiff: i64 = ((cq as usize - cp as usize) / ::std::mem::size_of::<i32>()) as i64;
    (if ((cdiff) == (2_i64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"cdiff == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut n: usize = 3_usize;
    let mut q5: *mut i32 = arr.as_mut_ptr().offset((n) as isize);
    (if ((*q5) == (13)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"*q5 == 13".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut q6: *mut i32 = (&mut arr[(n)] as *mut i32);
    (if ((q6) == (q5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"q6 == q5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut matrix: [[i32; 4]; 3] = [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11]];
    let mut row1: *mut i32 = (&mut matrix[(1) as usize][(0) as usize] as *mut i32);
    (if ((*row1.offset((2) as isize)) == (6)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"row1[2] == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut back: *mut i32 = end.offset(-((1) as isize));
    (if ((*back) == (17)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"*back == 17".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
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
