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
unsafe fn main_0() -> i32 {
    let mut two: (i32, f64) = <(i32, f64)>::tuple_from((1, 2.5_f64));
    assert!(((*two.get_mut::<0>()) == (1)));
    assert!(((*two.get_mut::<1>()) == (2.5_f64)));
    *two.get_mut::<0>() = 7;
    assert!(((*two.get_mut::<0>()) == (7)));
    let mut made: (i32, f64, libc::c_char) = (1, 2.5_f64, ('c' as libc::c_char));
    assert!(((*made.get_mut::<2>() as i32) == (('c' as libc::c_char) as i32)));
    assert!(made == (1, 2.5_f64, ('c' as libc::c_char),));
    assert!(!(made == (2, 2.5_f64, ('c' as libc::c_char),)));
    assert!(!((1, 2,) == (2, 1,)));
    let mut single: (i64,) = <(i64,)>::tuple_from((3,));
    assert!(((*single.get_mut::<0>()) == (3_i64)));
    let mut converted: (i64, f64) =
        <(i64, f64)>::tuple_from(<(i32, f32)>::tuple_from((4, 1.5_f32)));
    assert!(((*converted.get_mut::<0>()) == (4_i64)));
    assert!(((*converted.get_mut::<1>()) == (1.5_f64)));
    let mut empty: () = ();
    assert!(empty == ());
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
