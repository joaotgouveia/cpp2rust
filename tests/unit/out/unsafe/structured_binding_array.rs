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
    let mut arr: [(); 3] = [1_(), 2_(), 3_()];
    let mut __decomp_0: [(); 3] = std::array::from_fn::<_, 3, _>(|__i: usize| arr[(__i) as usize]);
    __decomp_0[(0_()) as usize] = 10_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let __decomp_1: *mut [(); 3] = &mut arr;
    (*__decomp_1)[(0_()) as usize] = 7_();
    (*__decomp_1)[(2_()) as usize] += (*__decomp_1)[(1_()) as usize];
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
