extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fn_0(mut v: ()) -> () {
    return unimplemented!();
}
pub unsafe fn fn2_1(v: *const ()) -> *const () {
    return v;
}
pub unsafe fn log_to_2(mut out: (), mut fmt: *const (), mut args_2: *const (), mut args_3: ()) {
    unimplemented!();
}
pub unsafe fn log_to_3(mut out: (), mut fmt: *const (), mut args: ()) {
    unimplemented!();
}
pub unsafe fn log_fmt_4(mut fmt: *const (), mut v: ()) {
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut in_: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (unsafe {
        log_to_2(
            unimplemented!(),
            c"%s %d\n".as_ptr(),
            c"runtime".as_ptr(),
            4_(),
        )
    });
    (unsafe { log_to_3(unimplemented!(), c"%d\n".as_ptr(), 5_()) });
    (unsafe { log_fmt_4(c"%d\n".as_ptr(), 6_()) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
