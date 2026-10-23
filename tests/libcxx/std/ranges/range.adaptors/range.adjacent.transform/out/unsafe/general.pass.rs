extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() -> () {
    let mut v: () = unimplemented!();
    let mut expected: () = unimplemented!();
    {
        let mut expected_index: () = 0_();
        let mut __tmp_0 : std_ranges_adjacent_transform_view_std_ranges_ref_view_std_vector_int__std_allocator_int_____std_multiplies_void___2_  = unimplemented!()  ;
        let __range1 : *mut std_ranges_adjacent_transform_view_std_ranges_ref_view_std_vector_int__std_allocator_int_____std_multiplies_void___2_ = & mut __tmp_0 ;
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut expected_index: () = 0_();
        let mut __tmp_1 : std_ranges_adjacent_transform_view_std_ranges_ref_view_std_vector_int__std_allocator_int_____std_multiplies_void___2_  = unimplemented!()  ;
        let __range1 : *mut std_ranges_adjacent_transform_view_std_ranges_ref_view_std_vector_int__std_allocator_int_____std_multiplies_void___2_ = & mut __tmp_1 ;
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = unimplemented!();
            unimplemented!();
        }
    }
    return true;
}
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
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
