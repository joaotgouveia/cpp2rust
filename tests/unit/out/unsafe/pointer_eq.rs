extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct __fsid_t {
    pub __val: [i32; 2],
}
impl Clone for __fsid_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __fsid_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 5;
    let mut p1: *const i32 = (&mut x as *mut i32).cast_const();
    let mut p2: *const i32 = (&mut x as *mut i32).cast_const();
    (if ((p1) == (p2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"p1 == p2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut y: i32 = 5;
    let mut p3: *const i32 = (&mut y as *mut i32).cast_const();
    (if ((p1) != (p3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"p1 != p3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut arr: [i32; 3] = [1, 2, 3];
    let mut p: *mut i32 = arr.as_mut_ptr();
    (if ((p) == (arr.as_mut_ptr())) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"p == arr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((p.offset((1) as isize)) == (&mut arr[(1) as usize] as *mut i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"p + 1 == &arr[1]".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((p.offset((2) as isize)) == (&mut arr[(2) as usize] as *mut i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"p + 2 == &arr[2]".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut val: i32 = 42;
    let mut orig: *mut i32 = (&mut val as *mut i32);
    let mut as_bytes: *mut u8 = (orig as *mut u8);
    let mut back: *mut i32 = (as_bytes as *mut i32);
    (if ((orig) == (back)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"orig == back".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut arr_bytes: *mut u8 = (arr.as_mut_ptr() as *mut u8);
    let mut arr_back: *mut i32 = (arr_bytes as *mut i32);
    (if ((arr_back) == (arr.as_mut_ptr())) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"arr_back == arr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((arr_back.offset((1) as isize)) == (&mut arr[(1) as usize] as *mut i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"arr_back + 1 == &arr[1]".as_ptr(),
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
