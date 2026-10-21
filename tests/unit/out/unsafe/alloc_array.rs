extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn All_0(arr: *mut (), mut N: (), mut element: ()) {
    let mut all: () = (unsafe { make_unique_1(N) });
    let mut i: () = unimplemented!();
    'loop_: while ((i) < (N)) {
        (*unimplemented!()) = element;
        i.prefix_inc();
    }
    unimplemented!();
}
pub unsafe fn Consume_2(mut arr: (), mut N: ()) -> () {
    let mut sum: () = unimplemented!();
    let mut i: () = -1_();
    'loop_: while ((i.prefix_inc()) < (N)) {
        sum += (*unimplemented!());
    }
    return sum;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let N: () = 10_();
    let mut arr: () = (unsafe { make_unique_1(N) });
    (unsafe { All_0(&mut arr, N, 1_()) });
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
