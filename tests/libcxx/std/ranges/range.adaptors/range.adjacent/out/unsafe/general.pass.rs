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
    let mut __tmp_0: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_0;
    'loop_: for __decomp_1 in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut __decomp_1: () = unimplemented!();
        let x: *mut () = (unsafe { get_2(&mut __decomp_1) });
        let y: *mut () = (unsafe { get_3(&mut __decomp_1) });
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
    let mut v: () = unimplemented!();
    let mut __decomp_5: () = unimplemented!();
    let a: *const () = (unsafe { get_6(&mut __decomp_5) });
    let b: *const () = (unsafe { get_7(&mut __decomp_5) });
    let c: *const () = (unsafe { get_8(&mut __decomp_5) });
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
