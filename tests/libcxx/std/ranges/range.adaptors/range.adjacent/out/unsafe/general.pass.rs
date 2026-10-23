extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_adjacent_pairs_0() {
    let mut v: () = unimplemented!();
    let mut expected_index: () = unimplemented!();
    let mut __tmp_0 : std_ranges_adjacent_view_std_ranges_ref_view_std_vector_int__std_allocator_int_____2_  = unimplemented!()  ;
    let __range1 : *mut std_ranges_adjacent_view_std_ranges_ref_view_std_vector_int__std_allocator_int_____2_ = & mut __tmp_0 ;
    'loop_: for __decomp_1 in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut __decomp_1: std_tuple_int_ref__int_ref_ = unimplemented!();
        let x: *mut () = (unsafe { get_2(&mut *(&raw mut __decomp_1).cast::<()>()) });
        let y: *mut () = (unsafe { get_3(&mut *(&raw mut __decomp_1).cast::<()>()) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        expected_index.first.prefix_inc();
        expected_index.second.prefix_inc();
    }
}
pub unsafe fn test_string_view_4() {
    let mut sv: () = unimplemented!();
    let mut v: std_ranges_adjacent_view_std_string_view__3_ = unimplemented!();
    let mut __decomp_5: std_tuple_const_char_ref__const_char_ref__const_char_ref_ =
        unimplemented!();
    let a: *const () = (unsafe { get_6(&mut *(&raw mut __decomp_5).cast::<()>()) });
    let b: *const () = (unsafe { get_7(&mut *(&raw mut __decomp_5).cast::<()>()) });
    let c: *const () = (unsafe { get_8(&mut *(&raw mut __decomp_5).cast::<()>()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_9() -> () {
    (unsafe { test_adjacent_pairs_0() });
    (unsafe { test_string_view_4() });
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
    (unsafe { test_9() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
