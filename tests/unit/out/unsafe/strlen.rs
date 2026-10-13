extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn strlen_0(mut ptr: *mut libc::c_char) -> u32 {
    let mut count: u32 = 0_u32;
    'loop_: while (((*ptr.postfix_inc()) as i32) != (('\0' as libc::c_char) as i32)) {
        count.prefix_inc();
    }
    return count;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut string: [libc::c_char; 6] = [
        ('h' as libc::c_char),
        ('e' as libc::c_char),
        ('l' as libc::c_char),
        ('l' as libc::c_char),
        ('o' as libc::c_char),
        ('\0' as libc::c_char),
    ];
    (if ((unsafe { strlen_0((&mut string[(0) as usize] as *mut libc::c_char)) }) == (5_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"strlen(&string[0]) == 5".as_ptr(),
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
