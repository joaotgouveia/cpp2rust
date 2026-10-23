extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct foo {
    val_: (),
}
impl foo {
    pub unsafe fn new() -> Self {
        let mut this = Self { val_: 3_() };
        this
    }
    pub unsafe fn get(&self) -> () {
        return self.val_;
    }
}
impl Default for foo {
    fn default() -> Self {
        unsafe { foo::new() }
    }
}
pub unsafe fn test_0() -> () {
    {
        let mut p1: () = (unsafe { make_unique_1(5_()) });
        let mut i: () = 0_();
        'loop_: while ((i) < (5_())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let mut p2: () = (unsafe { make_unique_2(5_()) });
        let mut i: () = 0_();
        'loop_: while ((i) < (5_())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let mut p3: () = (unsafe { make_unique_3(7_()) });
        let mut i: () = 0_();
        'loop_: while ((i) < (7_())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    return true;
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
