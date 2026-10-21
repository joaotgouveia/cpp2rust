extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn classify_0(mut x: *mut ()) -> () {
    {
        return (*x);
    }
    return 1_();
}
pub unsafe fn classify_1(mut x: ()) -> () {
    {
        {
            return 2_();
        }
    }
    return 1_();
}
pub unsafe fn classify_2(mut x: ()) -> () {
    {
        {}
    }
    return 1_();
}
pub unsafe fn keep_both_3(mut x: ()) -> () {
    {
        return ((x) + (1_()));
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn widen_4(mut v: ()) -> () {
    {}
    return v;
}
pub unsafe fn widen_5(mut v: ()) -> () {
    {
        let mut hi: () = v;
        hi <<= 32_();
        return hi;
    }
    return v;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut v: () = 7_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
