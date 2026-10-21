extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn identity_0(mut x: ()) -> () {
    return x;
}
pub unsafe fn swap_by_ptr_1(mut a: *mut (), mut b: *mut ()) {
    let mut tmp: () = (*a);
    (*a) = (*b);
    (*b) = tmp;
}
pub unsafe fn swap_by_ref_2(a: *mut (), b: *mut ()) {
    let mut tmp: () = (*a);
    (*a) = (*b);
    (*b) = tmp;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut local: () = unimplemented!();
    let mut a: () = 1_();
    let mut b: () = 2_();
    let mut c: () = (unsafe { identity_0(local) });
    let mut p: *mut () = (&mut a as *mut ());
    p = (&mut (b) as *mut ());
    p = (&mut a as *mut ());
    (unsafe { swap_by_ptr_1(p, (&mut b as *mut ())) });
    (unsafe { swap_by_ref_2(&mut a, &mut c) });
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
