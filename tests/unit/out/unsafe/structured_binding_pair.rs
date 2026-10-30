extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn make_std_pair_0(mut x: i32) -> (i32, bool) {
    return (x, ((x) > (0)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut __decomp_1: (i32, bool) = (unsafe { make_std_pair_0(5) });
    let value: *mut i32 = <(i32, bool) as PairGet<0>>::pair_get_mut(&mut __decomp_1);
    let positive: *mut bool = <(i32, bool) as PairGet<1>>::pair_get_mut(&mut __decomp_1);
    assert!(((*value) == (5)));
    assert!(((*positive) as bool));
    let mut p: (i32, i32) = (1, 2);
    let mut __decomp_2: (i32, i32) = {
        let __a0 = (p).clone();
        ((*&__a0.0), (*&__a0.1))
    };
    let a: *mut i32 = <(i32, i32) as PairGet<0>>::pair_get_mut(&mut __decomp_2);
    let b: *mut i32 = <(i32, i32) as PairGet<1>>::pair_get_mut(&mut __decomp_2);
    (*a) = 10;
    assert!(((*a) == (10)));
    assert!(((*b) == (2)));
    assert!(((p.0) == (1)));
    let __decomp_3: *mut (i32, i32) = &mut p;
    let x: *mut i32 = <(i32, i32) as PairGet<0>>::pair_get_mut(&mut (*__decomp_3));
    let y: *mut i32 = <(i32, i32) as PairGet<1>>::pair_get_mut(&mut (*__decomp_3));
    (*x) = 3;
    (*y) += 4;
    assert!(((p.0) == (3)));
    assert!(((p.1) == (6)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
