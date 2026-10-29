extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fn_0(
    mut u: std_unique_ptr_int__std_default_delete_int__,
) -> std_unique_ptr_int__std_default_delete_int__ {
    (*u) = 10_();
    return u;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut f: std_unique_ptr_int__std_default_delete_int__ = (unsafe {
        let mut ___args: () = 8_();
        make_unique_1(&mut ___args)
    });
    (*f) = 9_();
    let mut f_ptr1: *mut () = (unsafe { f.get() });
    (*f_ptr1) = 10_();
    let mut f_ptr2: *mut () = (&mut (*f) as *mut ());
    (*f_ptr2) = 11_();
    f = (unsafe {
        let mut ___args: () = 9_();
        make_unique_1(&mut ___args)
    });
    f = (unsafe { fn_0(f) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
