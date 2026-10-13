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
    let mut arr: [u8; 4] = [10_u8, 20_u8, 30_u8, 40_u8];
    let mut same: *mut u8 = arr.as_mut_ptr();
    (if (((*same.offset((0) as isize)) as i32) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"same[0] == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*same.offset((1) as isize)) as i32) == (20)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"same[1] == 20".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*same.offset((2) as isize)) as i32) == (30)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"same[2] == 30".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*same.offset((3) as isize)) as i32) == (40)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"same[3] == 40".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*same.offset((2) as isize)) = 99_u8;
    (if ((arr[(2) as usize] as i32) == (99)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"arr[2] == 99".as_ptr(),
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
