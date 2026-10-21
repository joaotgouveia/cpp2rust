extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn double_it_0(mut x: ()) -> () {
    return ((x) * (2_()));
}
pub unsafe fn triple_it_1(mut x: ()) -> () {
    return ((x) * (3_()));
}
pub static mut g_op_2: std::cell::LazyCell<Option<unsafe fn(()) -> ()>> =
    std::cell::LazyCell::new(|| unsafe { None });
pub unsafe fn set_op_3(mut fn_: Option<unsafe fn(()) -> ()>) {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut g_op_2)) = fn_;
}
pub unsafe fn call_op_4(mut x: ()) -> () {
    if !(*std::cell::LazyCell::force_mut(&mut *&raw mut g_op_2)).is_none() {
        return (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut g_op_2)).unwrap()(x) });
    }
    return x;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    (unsafe { set_op_3(Some(double_it_0)) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (unsafe { set_op_3(Some(triple_it_1)) });
    unimplemented!();
    unimplemented!();
    (unsafe { set_op_3(None) });
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const g_op_2);
}
