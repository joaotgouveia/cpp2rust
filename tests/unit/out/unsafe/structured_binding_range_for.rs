extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Pair {
    pub first: (),
    pub second: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut arr: [Pair; 3] = [
        Pair {
            first: 1_(),
            second: 2_(),
        },
        Pair {
            first: 3_(),
            second: 4_(),
        },
        Pair {
            first: 5_(),
            second: 6_(),
        },
    ];
    'loop_: for __decomp_0 in
        IterRange::new(arr.as_mut_ptr(), arr.as_mut_ptr().offset((3_()) as isize))
    {
        let __decomp_0: *mut Pair = &mut (*__decomp_0);
        (*__decomp_0).first += (*__decomp_0).second;
    }
    unimplemented!();
    unimplemented!();
    'loop_: for __decomp_1 in
        IterRange::new(arr.as_mut_ptr(), arr.as_mut_ptr().offset((3_()) as isize))
    {
        let mut __decomp_1: Pair = (*__decomp_1);
        __decomp_1.first = unimplemented!();
        __decomp_1.second = unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    let mut sum: () = unimplemented!();
    'loop_: for __decomp_2 in
        IterRange::new(arr.as_mut_ptr(), arr.as_mut_ptr().offset((3_()) as isize))
    {
        let __decomp_2: *const Pair = &(*__decomp_2);
        sum += (((*__decomp_2).first) * ((*__decomp_2).second));
    }
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
