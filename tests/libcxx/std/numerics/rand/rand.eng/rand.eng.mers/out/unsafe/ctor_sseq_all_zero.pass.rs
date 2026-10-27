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
pub struct all_zero_seed_seq {}
impl all_zero_seed_seq {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_2(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn size(&self) -> () {
        return 0_();
    }
    pub unsafe fn generate_4_unsigned_int_ptr(&mut self, mut rb: *mut (), mut re: *mut ()) {
        unimplemented!();
    }
}
impl Default for all_zero_seed_seq {
    fn default() -> Self {
        unsafe { all_zero_seed_seq::new_1() }
    }
}
pub unsafe fn test_0() {
    let state_size: () = 1_();
    let shift_size: () = 1_();
    let tempering_l: () = 3_();
    let mut q: all_zero_seed_seq = all_zero_seed_seq::new_1();
    let mut e: () = unimplemented!();
    let Xneg1: () = ((1_()) << ((3_() as ()).wrapping_sub(1_())));
    let Y: () = Xneg1;
    let X0: () = ((Xneg1) ^ ((Y) >> (1_())));
    unimplemented!();
}
pub unsafe fn test_1() {
    let state_size: () = 1_();
    let shift_size: () = 1_();
    let tempering_l: () = 33_();
    let mut q: all_zero_seed_seq = all_zero_seed_seq::new_1();
    let mut e: () = unimplemented!();
    let Xneg1: () = ((1_()) << ((33_() as ()).wrapping_sub(1_())));
    let Y: () = Xneg1;
    let X0: () = ((Xneg1) ^ ((Y) >> (1_())));
    unimplemented!();
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
    (unsafe { test_1() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
