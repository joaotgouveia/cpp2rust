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
    let mut vals: [i16; 3] = [(-1_i32 as i16), 256_i16, (-32768_i32 as i16)];
    let mut uvals: *mut u16 = (vals.as_mut_ptr() as *mut u16);
    (if (((*uvals.offset((0) as isize)) as i32) == (65535)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"uvals[0] == 0xFFFF".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*uvals.offset((1) as isize)) as i32) == (256)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"uvals[1] == 256".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*uvals.offset((2) as isize)) as i32) == (32768)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"uvals[2] == 0x8000".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*uvals.offset((0) as isize)) = 42_u16;
    (if ((vals[(0) as usize] as i32) == (42)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"vals[0] == 42".as_ptr(),
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
