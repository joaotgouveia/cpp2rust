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
pub struct Pair {
    pub first: i32,
    pub second: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut arr: [Pair; 3] = [
        Pair {
            first: 1,
            second: 2,
        },
        Pair {
            first: 3,
            second: 4,
        },
        Pair {
            first: 5,
            second: 6,
        },
    ];
    {
        let __range1: *mut [Pair; 3] = &mut arr;
        let mut __begin1: *mut Pair = (*__range1).as_mut_ptr();
        let mut __end1: *mut Pair = (*__range1).as_mut_ptr().offset((3_i64) as isize);
        'loop_: while ((__begin1) != (__end1)) {
            let __decomp_0: *mut Pair = &mut (*__begin1);
            (*__decomp_0).first += (*__decomp_0).second;
            __begin1.prefix_inc();
        }
    }
    (if ((arr[(0) as usize].first) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"arr[0].first == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((arr[(2) as usize].first) == (11)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"arr[2].first == 11".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    {
        let __range1: *mut [Pair; 3] = &mut arr;
        let mut __begin1: *mut Pair = (*__range1).as_mut_ptr();
        let mut __end1: *mut Pair = (*__range1).as_mut_ptr().offset((3_i64) as isize);
        'loop_: while ((__begin1) != (__end1)) {
            let mut __decomp_2: Pair = (*__begin1);
            __decomp_2.first = 0;
            __decomp_2.second = 0;
            __begin1.prefix_inc();
        }
    }
    (if ((arr[(1) as usize].first) == (7)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"arr[1].first == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((arr[(1) as usize].second) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"arr[1].second == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut sum: i32 = 0;
    {
        let __range1: *mut [Pair; 3] = &mut arr;
        let mut __begin1: *mut Pair = (*__range1).as_mut_ptr();
        let mut __end1: *mut Pair = (*__range1).as_mut_ptr().offset((3_i64) as isize);
        'loop_: while ((__begin1) != (__end1)) {
            let __decomp_3: *const Pair = &(*__begin1);
            sum += (((*__decomp_3).first) * ((*__decomp_3).second));
            __begin1.prefix_inc();
        }
    }
    (if ((sum) == ((((3) * (2)) + ((7) * (4))) + ((11) * (6)))) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"sum == 3 * 2 + 7 * 4 + 11 * 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
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
