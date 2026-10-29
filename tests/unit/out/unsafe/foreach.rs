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
    let mut v: std_vector_int__std_allocator_int__ =
        <std_vector_int__std_allocator_int__>::default();
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        (unsafe { v.push_back_57(&i) });
        i.prefix_inc();
    }
    let mut sum: () = 0_();
    'loop_: for x in IterRange::new((unsafe { v.begin_26() }), (unsafe { v.end_28() })) {
        let mut x: () = (*x);
        sum += x;
    }
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
