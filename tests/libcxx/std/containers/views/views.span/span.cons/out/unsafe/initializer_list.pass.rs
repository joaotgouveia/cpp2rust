extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Sink {}
impl Sink {
    pub unsafe fn new_1(mut _a0: *mut Sink) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn count_0(mut sp: ()) -> () {
    return unimplemented!();
}
pub unsafe fn count_n_1(mut sp: ()) -> () {
    return unimplemented!();
}
pub unsafe fn test_2() -> () {
    let mut a: [Sink; 10] = [<Sink>::default(); 10];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn three_3(mut sp: ()) -> () {
    return unimplemented!();
}
pub unsafe fn four_4(mut sp: ()) -> () {
    return unimplemented!();
}
pub unsafe fn test_P2447R4_annex_c_examples_5() -> () {
    {
        let mut a: [(); 10] = [unimplemented!(); 10];
        unimplemented!();
    }
    {
        let mut a: [(); 10] = unimplemented!();
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
    unimplemented!();
    const _: () = assert!(true, "test()");;
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
