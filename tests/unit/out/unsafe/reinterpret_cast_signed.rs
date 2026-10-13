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
    let mut neg: i32 = -1_i32;
    let mut words: *mut u16 = ((&mut neg as *mut i32) as *mut u16);
    (if (((*words.offset((0) as isize)) as i32) == (65535)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"words[0] == 0xFFFF".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*words.offset((1) as isize)) as i32) == (65535)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"words[1] == 0xFFFF".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut neg64: i64 = (-256_i32 as i64);
    let mut quarters: *mut i16 = ((&mut neg64 as *mut i64) as *mut i16);
    (if (((*quarters.offset((0) as isize)) as i32) == (-256_i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"quarters[0] == -256".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*quarters.offset((1) as isize)) as i32) == (-1_i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"quarters[1] == -1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*quarters.offset((2) as isize)) as i32) == (-1_i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"quarters[2] == -1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*quarters.offset((3) as isize)) as i32) == (-1_i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"quarters[3] == -1".as_ptr(),
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
