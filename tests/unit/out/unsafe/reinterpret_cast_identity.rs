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
    let mut val: u32 = 42_u32;
    let mut original: *mut u32 = (&mut val as *mut u32);
    let mut as_u16: *mut u16 = (original as *mut u16);
    let mut back: *mut u32 = (as_u16 as *mut u32);
    (if ((back) == (original)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"back == original".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*back) == (42_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"*back == 42".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut as_u8: *mut u8 = (original as *mut u8);
    let mut back2: *mut u32 = (as_u8 as *mut u32);
    (if ((back2) == (original)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"back2 == original".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*back2) == (42_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"*back2 == 42".as_ptr(),
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
