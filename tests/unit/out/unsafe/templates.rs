extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(mut x: ()) -> () {
    return x;
}
pub unsafe fn foo_1(mut x: ()) -> () {
    return x;
}
pub unsafe fn pick_2() -> () {
    return unimplemented!();
}
pub unsafe fn pick_3() -> () {
    return 1_();
}
pub unsafe fn bar_4(mut p: *mut (), mut flag: ()) -> *mut () {
    return if flag { p } else { std::ptr::null_mut() };
}
pub unsafe fn bar_5(mut p: *mut (), mut flag: ()) -> *mut () {
    return if flag { p } else { std::ptr::null_mut() };
}
pub unsafe fn func_6(mut x1: (), mut x2: (), mut x3: ()) -> () {
    return (((x1) + (x2)) + (x3));
}
pub unsafe fn func_7(mut x1: (), mut x2: (), mut x3: ()) -> () {
    return (((x1) + (x2)) + (x3));
}
pub static mut half_8: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { ((1_()) / (2_())) });
pub static mut half_9: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { ((1_()) / (2_())) });
pub static mut half_10: std::cell::LazyCell<*mut ()> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x: () = 10_();
    let mut y: () = x;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut half_8)) = 7_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut half_10)) = (&mut x as *mut ());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const half_8);
    std::cell::LazyCell::force(&*&raw const half_9);
    std::cell::LazyCell::force(&*&raw const half_10);
}
