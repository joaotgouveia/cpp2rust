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
    let mut arr: [i32; 3] = [1, 2, 3];
    let mut __decomp_0: [i32; 3] = std::array::from_fn::<_, 3, _>(|__i: usize| arr[(__i)]);
    __decomp_0[(0) as usize] = 10;
    (if ((__decomp_0[(0) as usize]) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"a == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((__decomp_0[(1) as usize]) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"b == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((__decomp_0[(2) as usize]) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"c == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((arr[(0) as usize]) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"arr[0] == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let __decomp_2: *mut [i32; 3] = &mut arr;
    (*__decomp_2)[(0) as usize] = 7;
    (*__decomp_2)[(2) as usize] += (*__decomp_2)[(1) as usize];
    (if ((arr[(0) as usize]) == (7)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"arr[0] == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((arr[(2) as usize]) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"arr[2] == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut darr: [f64; 2] = [1.5E+0, 2.5E+0];
    let mut __decomp_3: [f64; 2] = std::array::from_fn::<_, 2, _>(|__i: usize| darr[(__i)]);
    __decomp_3[(0) as usize] = 4.0E+0;
    (if (((__decomp_3[(0) as usize]) + (__decomp_3[(1) as usize])) == (6.5E+0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"d0 + d1 == 6.5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((darr[(0) as usize]) == (1.5E+0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"darr[0] == 1.5".as_ptr(),
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
