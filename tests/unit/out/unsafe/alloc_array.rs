extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn All_0(
    arr: *mut std_unique_ptr_intarrarr__std_default_delete_intarrarr__,
    mut N: (),
    mut element: (),
) {
    let mut all: std_unique_ptr_intarrarr__std_default_delete_intarrarr__ =
        (unsafe { make_unique_1(N) });
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        all[(i) as usize] = element;
        i.prefix_inc();
    }
    (*arr) = (all).clone();
}
pub unsafe fn Consume_2(
    mut arr: std_unique_ptr_intarrarr__std_default_delete_intarrarr__,
    mut N: (),
) -> () {
    let mut sum: () = 0_();
    let mut i: () = -1_();
    'loop_: while ((i.prefix_inc()) < (N)) {
        sum += arr[(i) as usize];
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
    let mut arr: std_unique_ptr_intarrarr__std_default_delete_intarrarr__ =
        (unsafe { make_unique_1(N) });
    (unsafe { All_0(&mut arr, N, 1_()) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
