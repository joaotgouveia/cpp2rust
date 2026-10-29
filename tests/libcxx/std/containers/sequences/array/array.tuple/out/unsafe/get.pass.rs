extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn tempArray_0(mut args_0: (), mut args_1: (), mut args_2: ()) -> () {
    return unimplemented!();
}
pub unsafe fn tests_1() -> () {
    {
        let mut array: () = unimplemented!();
        unimplemented!();
        (*unimplemented!()) = 99.1_f64;
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        (*unimplemented!()) = 99.1_f64;
        (*unimplemented!()) = 99.2_f64;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (*unimplemented!()) = 99.2_f64;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        const _: () = assert!(
            true,
            "std::is_same<double&, decltype(std::get<0>(array))>::value"
        );;
    }
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
    (unsafe { tests_1() });
    const _: () = assert!(true, "tests()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
