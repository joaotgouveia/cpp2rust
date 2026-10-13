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
    let mut arr: [u32; 2] = [67305985_u32, 134678021_u32];
    let mut bytes: *mut u8 = (arr.as_mut_ptr() as *mut u8);
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
    (if (((*bytes.offset((4) as isize)) as i32) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"bytes[4] == 0x05".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*bytes.offset((3) as isize)) = 255_u8;
    (if ((arr[(0) as usize]) == (4278387201_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"arr[0] == 0xFF030201".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*bytes.offset((4) as isize)) = 170_u8;
    (if ((arr[(1) as usize]) == (134678186_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"arr[1] == 0x080706AA".as_ptr(),
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
