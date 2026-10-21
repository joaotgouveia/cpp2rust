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
    let mut rows: [[(); 3]; 2] = [[1_(), 2_(), 3_()], [4_(), 5_(), 6_()]];
    return ((rows[(1_()) as usize].as_mut_ptr() as usize
        - ((&mut rows[(unimplemented!()) as usize][(unimplemented!()) as usize] as *mut ())
            .offset((2_()) as isize)) as usize)
        / ::std::mem::size_of::<()>()) as ();
}
pub unsafe fn __cpp2rust_init_globals() {}
