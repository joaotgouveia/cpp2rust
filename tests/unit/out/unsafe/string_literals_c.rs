extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_mut_0(mut str: *mut ()) {}
pub unsafe fn foo_const_1(mut str: *const ()) {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut mutable_strings: [*mut (); 3] = [
        c"a".as_ptr().cast_mut(),
        c"b".as_ptr().cast_mut(),
        c"c".as_ptr().cast_mut(),
    ];
    let mut immutable_strings: [*const (); 3] = [
        (c"a".as_ptr().cast_mut()).cast_const(),
        (c"b".as_ptr().cast_mut()).cast_const(),
        (c"c".as_ptr().cast_mut()).cast_const(),
    ];
    let mut mutable_string: *mut () = c"hello".as_ptr().cast_mut();
    let mut immutable_string: *const () = (c"hello".as_ptr().cast_mut()).cast_const();
    let mut mutable_string_arr: [(); 9] = std::mem::transmute(*b"papanasi\0");
    let immutable_string_arr: [(); 9] = std::mem::transmute(*b"papanasi\0");
    let mut mutable_empty: *mut () = c"".as_ptr().cast_mut();
    let mut immutable_empty: *const () = (c"".as_ptr().cast_mut()).cast_const();
    let mut mutable_empty_arr: [(); 1] = [0 as libc::c_char; 1];
    let immutable_empty_arr: [(); 1] = [0 as libc::c_char; 1];
    (unsafe { foo_mut_0(c"world".as_ptr().cast_mut()) });
    (unsafe { foo_mut_0(mutable_string) });
    (unsafe { foo_mut_0(mutable_string_arr.as_mut_ptr()) });
    (unsafe { foo_const_1((c"world".as_ptr().cast_mut()).cast_const()) });
    (unsafe { foo_const_1((mutable_string).cast_const()) });
    (unsafe { foo_const_1(immutable_string) });
    (unsafe { foo_const_1((mutable_string_arr.as_mut_ptr()).cast_const()) });
    (unsafe { foo_const_1(immutable_string_arr.as_ptr()) });
    (unsafe { foo_const_1((c"".as_ptr().cast_mut()).cast_const()) });
    (unsafe { foo_const_1((mutable_empty).cast_const()) });
    (unsafe { foo_const_1(immutable_empty) });
    (unsafe { foo_const_1((mutable_empty_arr.as_mut_ptr()).cast_const()) });
    (unsafe { foo_const_1(immutable_empty_arr.as_ptr()) });
    let inited_through_init_list: [(); 21] = std::mem::transmute(*b"papanasi cu smantana\0");
    (unsafe { foo_const_1(inited_through_init_list.as_ptr()) });
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
