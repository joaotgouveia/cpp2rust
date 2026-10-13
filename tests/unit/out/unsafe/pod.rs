extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct POD {
    pub x1: i32,
    pub x2: i32,
    pub x3: i32,
}
pub unsafe fn PODIncrement_0(pod: *mut POD) {
    (*pod).x1 += 1;
    (*pod).x2 += 2;
    (*pod).x3 += 3;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p1: POD = POD {
        x1: 10,
        x2: 11,
        x3: 12,
    };
    let mut p2: POD = POD {
        x1: p1.x1,
        x2: p1.x2,
        x3: p1.x3,
    };
    (unsafe { PODIncrement_0(&mut p2) });
    (if ((((p2.x1) + (p2.x2)) + (p2.x3)) == (39)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"p2.x1 + p2.x2 + p2.x3 == 39".as_ptr(),
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
