extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn change_0(n: *mut std_unique_ptr_int__std_default_delete_int__) {
    let mut m: std_unique_ptr_int__std_default_delete_int__ = (unsafe {
        let mut ___args: () = 20_();
        make_unique_1(&mut ___args)
    });
    (*n) = (m).clone();
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut n: std_unique_ptr_int__std_default_delete_int__ = (unsafe {
        let mut ___args: () = 10_();
        make_unique_1(&mut ___args)
    });
    (unsafe { change_0(&mut n) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
