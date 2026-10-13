extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn strlen_0(mut s: *const libc::c_char, mut n: i32) -> i32 {
    return if ((*s) != 0) {
        (unsafe { strlen_0(s.offset((1) as isize), ((n) + (1))) })
    } else {
        n
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let s: [libc::c_char; 4] = [
        ('s' as libc::c_char),
        ('t' as libc::c_char),
        ('r' as libc::c_char),
        ('\0' as libc::c_char),
    ];
    (if ((unsafe { strlen_0((&s[(0) as usize] as *const libc::c_char), 0) }) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"strlen(&s[0], 0) == 3".as_ptr(),
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
