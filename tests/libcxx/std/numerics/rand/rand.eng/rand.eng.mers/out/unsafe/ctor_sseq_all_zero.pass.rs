extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(Vec<u32 > , new_1, value)]
pub struct all_zero_seed_seq {}
impl all_zero_seed_seq {
    pub unsafe fn new_2() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_1(mut _a0: Vec<u32>) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn size(&self) -> usize {
        return 0_usize;
    }
    pub unsafe fn generate_4_unsigned_int_ptr(&mut self, mut rb: *mut u32, mut re: *mut u32) {
        unimplemented!();
    }
}
impl Default for all_zero_seed_seq {
    fn default() -> Self {
        unsafe { all_zero_seed_seq::new_2() }
    }
}
pub unsafe fn test_0() {
    let state_size: usize = 1_usize;
    let shift_size: usize = 1_usize;
    let tempering_l: usize = (3_u64 as usize);
    let mut q: all_zero_seed_seq = all_zero_seed_seq::new_2();
    let mut e: () = unimplemented!();
    let Xneg1: u16 = (((1_u16 as i32) << ((3_u64 as u64).wrapping_sub(1_u64))) as u16);
    let Y: u16 = Xneg1;
    let X0: u16 = (((Xneg1 as i32) ^ ((Y as i32) >> (1))) as u16);
    assert!(((unimplemented!() as i32) == (X0 as i32)));
}
pub unsafe fn test_1() {
    let state_size: usize = 1_usize;
    let shift_size: usize = 1_usize;
    let tempering_l: usize = (33_u64 as usize);
    let mut q: all_zero_seed_seq = all_zero_seed_seq::new_2();
    let mut e: () = unimplemented!();
    let Xneg1: u64 = ((1_u64) << ((33_u64 as u64).wrapping_sub(1_u64)));
    let Y: u64 = Xneg1;
    let X0: u64 = ((Xneg1) ^ ((Y) >> (1)));
    assert!(((unimplemented!()) == (X0)));
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_0() });
    (unsafe { test_1() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
