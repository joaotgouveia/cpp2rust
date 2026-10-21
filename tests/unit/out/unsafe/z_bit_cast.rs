extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn decay_cast_0(mut a1: *mut ()) {}
pub unsafe fn bit_cast_1(mut p: ()) {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a1: [(); 3] = [1_(), 2_(), 3_()];
    (unsafe { decay_cast_0(a1.as_mut_ptr()) });
    (unsafe { decay_cast_0((&mut a1[(unimplemented!()) as usize] as *mut ())) });
    (unsafe { bit_cast_1((a1.as_mut_ptr() as ())) });
    (unsafe { bit_cast_1(((&mut a1[(unimplemented!()) as usize] as *mut ()) as ())) });
    (unsafe { bit_cast_1(((&mut a1 as *mut [(); 3]) as ())) });
    let mut ptr: () = (a1.as_mut_ptr() as ());
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
