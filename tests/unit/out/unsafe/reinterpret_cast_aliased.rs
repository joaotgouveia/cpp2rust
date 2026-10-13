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
    let mut val: u64 = 578437695752307201_u64;
    let mut view1: *mut u32 = ((&mut val as *mut u64) as *mut u32);
    let mut view2: *mut u32 = ((&mut val as *mut u64) as *mut u32);
    (*view1.offset((0) as isize)) = 3721182122_u32;
    (if ((*view2.offset((0) as isize)) == (3721182122_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"view2[0] == 0xDDCCBBAA".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((val) == (578437699406183338_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"val == 0x08070605DDCCBBAAULL".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*view2.offset((1) as isize)) = 4293844428_u32;
    (if ((*view1.offset((1) as isize)) == (4293844428_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"view1[1] == 0xFFEEDDCC".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((val) == (18441921396093008810_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"val == 0xFFEEDDCCDDCCBBAAULL".as_ptr(),
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
