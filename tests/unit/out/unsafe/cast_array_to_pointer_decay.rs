extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn deref_0(mut p: *mut i32) -> i32 {
    return (*p);
}
pub unsafe fn strlen_1(mut s: *mut libc::c_char) -> i32 {
    let mut c: i32 = 0;
    'loop_: while ((*s.postfix_inc()) != 0) {
        c.prefix_inc();
    }
    return c;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: [i32; 2] = [1, 2];
    let mut s: [libc::c_char; 4] = [
        ('a' as libc::c_char),
        ('b' as libc::c_char),
        ('c' as libc::c_char),
        ('\0' as libc::c_char),
    ];
    (if (((unsafe { deref_0(a.as_mut_ptr()) }) + (unsafe { strlen_1(s.as_mut_ptr()) })) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"deref(a) + strlen(s) == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_2(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
