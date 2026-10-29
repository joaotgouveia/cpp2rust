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
    let mut p: std_unique_ptr_int__std_default_delete_int__ = (unsafe {
        let mut ___args: () = 10_();
        make_unique_0(&mut ___args)
    });
    (*p) += 5_();
    (*p) -= 3_();
    (*p) *= 2_();
    let mut q: std_unique_ptr_int__std_default_delete_int__ = (unsafe {
        let mut ___args: () = 1_();
        make_unique_0(&mut ___args)
    });
    let mut sum: () = ((*p) + (*q));
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
