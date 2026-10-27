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
pub struct test_macros_detail_is_same_std_chrono_weekday_ref__std_chrono_weekday_ref_ {}
pub unsafe fn euclidian_addition_1(mut rhs: (), mut lhs: ()) -> () {
    let modulus: () = ((6_() as ()).wrapping_sub((0_() as ())) as ()).wrapping_add(1_());
    let mut ret: () = (rhs).wrapping_add(lhs);
    if ((ret) > (6_())) {
        ret = (ret).wrapping_sub(modulus);
    }
    return ret;
}
pub unsafe fn euclidian_subtraction_2(mut lhs: (), mut rhs: ()) -> () {
    let modulus: () = ((6_() as ()).wrapping_sub((0_() as ())) as ()).wrapping_add(1_());
    let mut ret: () = (lhs).wrapping_sub(rhs);
    if ((ret) < (0_())) {
        ret = (ret).wrapping_add(modulus);
    }
    if ((ret) > (6_())) {
        ret = (ret).wrapping_add(modulus);
    }
    return ret;
}
pub unsafe fn test_3() -> () {
    let mut i: () = 0_();
    'loop_: while ((i) <= (6_())) {
        let mut wd: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) <= (6_())) {
        let mut wd: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
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
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    (unsafe { test_3() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
