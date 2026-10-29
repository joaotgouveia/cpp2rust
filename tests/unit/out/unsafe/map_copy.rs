extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int___ =
        <std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int___>::default();
    a[(1_()) as usize] = 10_();
    let mut b: std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int___ =
        a.clone();
    b[(1_()) as usize] = 20_();
    unimplemented!();
    let mut x: std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int___ =
        <std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int___>::default();
    x[(2_()) as usize] = 3_();
    let mut y: std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int___ =
        x.clone();
    y[(2_()) as usize] = 4_();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
