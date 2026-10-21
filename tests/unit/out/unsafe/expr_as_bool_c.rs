extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn cmp_eq_0(mut rc: ()) -> () {
    return (((rc) == (-1_())) as ());
}
pub unsafe fn cmp_or_ptr_1(mut p: *const (), mut q: *const ()) -> () {
    return (((!(p).is_null()) || (!(q).is_null())) as ());
}
pub unsafe fn both_null_2(mut s1: *const (), mut s2: *const ()) -> () {
    return ((((((s1).is_null()) as ()) != 0) && ((((s2).is_null()) as ()) != 0)) as ());
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: () = 0_();
    let mut b: () = unimplemented!();
    if ({
        b = a;
        b
    } != 0)
    {}
    'loop_: while (((({
        b = a;
        b
    }) != (0_())) as ())
        != 0)
    {}
    if (a != 0) {}
    if ((((a) == (b)) as ()) != 0) {}
    if ((((a) < (b)) as ()) != 0) {}
    unimplemented!();
    unimplemented!();
    let mut c: () = unimplemented!();
    c = ({
        a = b;
        a
    } != 0);
    c = (((({
        b = a;
        b
    }) != (0_())) as ())
        != 0);
    c = (a != 0);
    c = ((((a) == (b)) as ()) != 0);
    c = ((((a) < (b)) as ()) != 0);
    let mut x: () = 5_();
    let mut y: () = 5_();
    let mut eq: () = (((x) == (y)) as ());
    let mut lt: () = (((x) < (y)) as ());
    let mut neq: () = (((x) != (y)) as ());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p1: *const () = (c"hi".as_ptr().cast_mut()).cast_const();
    let mut p2: *const () = std::ptr::null();
    let mut either: () = (((!(p1).is_null()) || (!(p2).is_null())) as ());
    let mut both: () = (((!(p1).is_null()) && (!(p2).is_null())) as ());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
