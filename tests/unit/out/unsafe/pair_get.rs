extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn second_of_0(p: *const (f64, i32)) -> i32 {
    return (*<(f64, i32) as PairGet<1>>::pair_get(&(*p)));
}
pub unsafe fn first_of_1(p: *const (f64, i32)) -> f64 {
    return (*<(f64, i32) as PairGet<0>>::pair_get(&(*p)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: (f64, i32) = (1.5_f64, 2);
    assert!(((*<(f64, i32) as PairGet<0>>::pair_get_mut(&mut p)) == (1.5_f64)));
    assert!(((*<(f64, i32) as PairGet<1>>::pair_get_mut(&mut p)) == (2)));
    assert!(((unsafe { first_of_1(&p,) }) == (1.5_f64)));
    assert!(((unsafe { second_of_0(&p,) }) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
