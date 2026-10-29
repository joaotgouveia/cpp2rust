extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut buf: [(); 6] = [
        ('a' as ()),
        ('b' as ()),
        ('\0' as ()),
        ('c' as ()),
        ('d' as ()),
        ('\0' as ()),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s: std_basic_string_char__std_char_traits_char___std_allocator_char__ =
        std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { (buf.as_mut_ptr()).cast_const() },
            None,
        );
    unimplemented!();
    unimplemented!();
    let mut lit: *const () = (&[
        (120 as libc::c_char),
        (121 as libc::c_char),
        (0 as libc::c_char),
        (122 as libc::c_char),
        (119 as libc::c_char),
        (0 as libc::c_char),
    ])
        .as_ptr();
    let mut t: std_basic_string_char__std_char_traits_char___std_allocator_char__ =
        std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1({ lit }, None);
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
