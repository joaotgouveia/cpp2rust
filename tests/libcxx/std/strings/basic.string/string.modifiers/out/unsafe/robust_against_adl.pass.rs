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
pub struct Charlike_Holder_Incomplete__ {
    pub ch_: (),
}
impl Charlike_Holder_Incomplete__ {
    pub unsafe fn new(mut ch: ()) -> Self {
        let mut this = Self { ch_: ch };
        this
    }
    pub unsafe fn to_(&self) -> () {
        return self.ch_;
    }
}
pub unsafe fn test_string_0() {
    let mut s: () = unimplemented!();
    let mut a: [Charlike_Holder_Incomplete__; 4] = [
        Charlike_Holder_Incomplete__::new({ ('m' as ()) }),
        Charlike_Holder_Incomplete__::new({ ('a' as ()) }),
        Charlike_Holder_Incomplete__::new({ ('h' as ()) }),
        Charlike_Holder_Incomplete__::new({ ('i' as ()) }),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_1() -> () {
    (unsafe { test_string_0() });
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
    (unsafe { test_1() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
