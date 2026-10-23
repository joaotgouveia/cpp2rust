extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct A {
    pub data_: (),
}
impl A {
    pub unsafe fn new(mut i: Option<()>) -> Self {
        let mut i: () = i.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { data_: i };
        this
    }
}
impl std::cmp::PartialEq for A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const A, other as *const A) }
    }
}
impl std::cmp::Eq for A {}
impl Default for A {
    fn default() -> Self {
        unsafe { A::new(None) }
    }
}
pub unsafe fn operator_eq_0(x: *const A, y: *const A) -> () {
    return (((*x).data_) == ((*y).data_));
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
    const _: () = assert!(true, "std::numeric_limits<A>::is_specialized == false");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "std::numeric_limits<A>::digits == 0");;
    const _: () = assert!(true, "std::numeric_limits<A>::digits10 == 0");;
    const _: () = assert!(true, "std::numeric_limits<A>::max_digits10 == 0");;
    const _: () = assert!(true, "std::numeric_limits<A>::is_signed == false");;
    const _: () = assert!(true, "std::numeric_limits<A>::is_integer == false");;
    const _: () = assert!(true, "std::numeric_limits<A>::is_exact == false");;
    const _: () = assert!(true, "std::numeric_limits<A>::radix == 0");;
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "std::numeric_limits<A>::min_exponent == 0");;
    const _: () = assert!(true, "std::numeric_limits<A>::min_exponent10 == 0");;
    const _: () = assert!(true, "std::numeric_limits<A>::max_exponent == 0");;
    const _: () = assert!(true, "std::numeric_limits<A>::max_exponent10 == 0");;
    const _: () = assert!(true, "std::numeric_limits<A>::has_infinity == false");;
    const _: () = assert!(true, "std::numeric_limits<A>::has_quiet_NaN == false");;
    const _: () = assert!(true, "std::numeric_limits<A>::has_signaling_NaN == false");;
    const _: () = assert!(
        true,
        "std::numeric_limits<A>::has_denorm == std::denorm_absent"
    );;
    const _: () = assert!(true, "std::numeric_limits<A>::has_denorm_loss == false");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "std::numeric_limits<A>::is_iec559 == false");;
    const _: () = assert!(true, "std::numeric_limits<A>::is_bounded == false");;
    const _: () = assert!(true, "std::numeric_limits<A>::is_modulo == false");;
    const _: () = assert!(true, "std::numeric_limits<A>::traps == false");;
    const _: () = assert!(true, "std::numeric_limits<A>::tinyness_before == false");;
    const _: () = assert!(
        true,
        "std::numeric_limits<A>::round_style == std::round_toward_zero"
    );;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
