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
    let mut value: u32 = 67305985_u32;
    let mut bytes: *mut u8 = ((&mut value as *mut u32) as *mut u8);
    (if (((*bytes.offset((0) as isize)) as i32) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"bytes[0] == 0x01".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*bytes.offset((1) as isize)) as i32) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"bytes[1] == 0x02".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*bytes.offset((2) as isize)) as i32) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"bytes[2] == 0x03".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*bytes.offset((3) as isize)) as i32) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"bytes[3] == 0x04".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut arr: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
    let mut arr16: *mut u16 = (arr.as_mut_ptr() as *mut u16);
    (if (((*arr16.offset((0) as isize)) as i32) == (513)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"arr16[0] == 0x0201".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*arr16.offset((1) as isize)) as i32) == (1027)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"arr16[1] == 0x0403".as_ptr(),
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
