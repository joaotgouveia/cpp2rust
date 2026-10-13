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
    let mut x: i32 = 1;
    let mut p: *mut i32 = (&mut x as *mut i32);
    (*p) += 1;
    (if ((x) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"x == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut a: [i32; 2] = [1, 2];
    p = (&mut a[(1) as usize] as *mut i32);
    (*p) += 1;
    (if ((a[(0) as usize]) == (1)) && ((a[(1) as usize]) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a[0] == 1 && a[1] == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    p.prefix_dec();
    (*p) += 1;
    (if ((a[(0) as usize]) == (2)) && ((a[(1) as usize]) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a[0] == 2 && a[1] == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    p = (&mut x as *mut i32);
    (*p) += 1;
    (if ((x) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"x == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut p2: *mut i32 = p;
    (*p2) += 1;
    (if ((x) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"x == 4".as_ptr(),
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
