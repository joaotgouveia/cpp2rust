extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut ca_allocs_0: std::cell::LazyCell<Vec<i32>> =
    std::cell::LazyCell::new(|| unsafe { Vec::new() });

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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let __a1 = 0;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut ca_allocs_0)).push(__a1)
    };
    'loop_: for a in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut ca_allocs_0)).begin(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut ca_allocs_0)).end(),
    ) {
        let a: *const i32 = &(*a.iter_deref());
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const ca_allocs_0);
}
