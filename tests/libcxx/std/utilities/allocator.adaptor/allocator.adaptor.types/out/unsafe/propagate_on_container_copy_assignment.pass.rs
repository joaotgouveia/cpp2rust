extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct A1_int_ {
    id_: (),
}
impl A1_int_ {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct A2_int_ {
    id_: (),
}
impl A2_int_ {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct A3_int_ {
    id_: (),
}
impl A3_int_ {}
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
    const _: () = assert!(
        true,
        "(std::is_same< std::scoped_allocator_adaptor<A1<int>>::propagate_on_container_copy_assignment,\n                               std::false_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< std::scoped_allocator_adaptor<A1<int>, A2<int>>::propagate_on_container_copy_assignment,\n                               std::false_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< std::scoped_allocator_adaptor<A1<int>, A2<int>, A3<int>>::propagate_on_container_copy_assignment,\n                     std::true_type>::value)"
    );;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
