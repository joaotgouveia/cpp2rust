extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut cinc1_2: std::cell::LazyCell<*const IncompleteType> =
    std::cell::LazyCell::new(|| unsafe {
        &(*std::cell::LazyCell::force_mut(&mut *&raw mut inc1_0))
    });
pub static mut cinc2_3: std::cell::LazyCell<*const IncompleteType> =
    std::cell::LazyCell::new(|| unsafe {
        &(*std::cell::LazyCell::force_mut(&mut *&raw mut inc2_1))
    });

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
    {
        let mut t: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IncompleteType {}
pub static mut inc1_0: std::cell::LazyCell<IncompleteType> =
    std::cell::LazyCell::new(|| unsafe { <IncompleteType>::default() });
pub static mut inc2_1: std::cell::LazyCell<IncompleteType> =
    std::cell::LazyCell::new(|| unsafe { <IncompleteType>::default() });
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const cinc1_2);
    std::cell::LazyCell::force(&*&raw const cinc2_3);
    std::cell::LazyCell::force(&*&raw const inc1_0);
    std::cell::LazyCell::force(&*&raw const inc2_1);
}
