extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fatorial_0(mut n: ()) -> () {
    if ((n) == (unimplemented!())) {
        return 1_();
    }
    return ((n) * (unsafe { fatorial_0(((n) - (1_()))) }));
}
pub unsafe fn fatorial_by_ref_1(n: *mut ()) {
    if ((*n) == (1_())) {
        (*n) *= 1_();
        return;
    }
    let mut n_1: () = ((*n) - (1_()));
    (unsafe { fatorial_by_ref_1(&mut n_1) });
    (*n) *= n_1;
}
pub unsafe fn fatorial_by_ptr_2(mut n: *mut ()) {
    if ((*n) == (1_())) {
        (*n) *= 1_();
        return;
    }
    let mut n_1: () = ((*n) - (1_()));
    (unsafe { fatorial_by_ptr_2((&mut n_1 as *mut ())) });
    (*n) *= n_1;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut n1: () = 2_();
    (unsafe { fatorial_by_ptr_2((&mut n1 as *mut ())) });
    let mut n: () = ((n1) + (1_()));
    (unsafe { fatorial_by_ref_1(&mut n) });
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
