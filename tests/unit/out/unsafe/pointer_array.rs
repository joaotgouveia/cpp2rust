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
pub struct StackArray {
    pub arr: [*mut i32; 3],
}
pub unsafe fn IncrementAll_0(s: *mut StackArray) {
    let mut i: i32 = 0;
    'loop_: while ((i) < (3)) {
        (*(*s).arr[(i) as usize]) += 1;
        i.prefix_inc();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 0;
    let mut s: StackArray = StackArray {
        arr: [
            (&mut x as *mut i32),
            (&mut x as *mut i32),
            (&mut x as *mut i32),
        ],
    };
    (unsafe { IncrementAll_0(&mut s) });
    (if ((x) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"x == 3".as_ptr(),
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
