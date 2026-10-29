extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_back_insert_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_back_insert_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_back_insert_iterator_std_vector_int__std_allocator_int_____std_back_insert_iterator_std_vector_int__std_allocator_int____
{}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    {
        let mut s: () = unimplemented!();
        let mut it: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut v: () = unimplemented!();
        let mut it: () = unimplemented!();
        let mut copy: () = it.clone();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
