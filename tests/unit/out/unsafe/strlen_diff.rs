extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: i64,
    pub __clang_max_align_nonce2: f64,
}
impl Clone for max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type std_byte = u8;
pub unsafe fn strlen_0(mut s: *const libc::c_char) -> usize {
    let mut begin: *const libc::c_char = s;
    'loop_: while ((*s) != 0) {
        s.prefix_inc();
    }
    return ((((s as usize - begin as usize) / ::std::mem::size_of::<libc::c_char>()) as i64)
        as usize);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let s: [libc::c_char; 7] = [
        ('s' as libc::c_char),
        ('t' as libc::c_char),
        ('r' as libc::c_char),
        ('i' as libc::c_char),
        ('n' as libc::c_char),
        ('g' as libc::c_char),
        ('\0' as libc::c_char),
    ];
    (if ((unsafe { strlen_0((&s[(0) as usize] as *const libc::c_char)) }) == (6_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"strlen(&s[0]) == 6".as_ptr(),
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
