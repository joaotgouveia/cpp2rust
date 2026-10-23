extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;

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
        let mut f: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut f: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut f: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut f: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut f: () = unimplemented!();
        (unsafe { f.clear_5(None) });
        unimplemented!();
        (unsafe { f.clear_5(None) });
        unimplemented!();
    }
    {
        let mut f: () = unimplemented!();
        (unsafe {
            f.clear_5(Some(
                (*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_relaxed_0)),
            ))
        });
        unimplemented!();
        (unsafe {
            f.clear_5(Some(
                (*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_relaxed_0)),
            ))
        });
        unimplemented!();
    }
    {
        let mut f: () = unimplemented!();
        (unsafe {
            f.clear_5(Some(
                (*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_1)),
            ))
        });
        unimplemented!();
        (unsafe {
            f.clear_5(Some(
                (*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_1)),
            ))
        });
        unimplemented!();
    }
    {
        let mut f: () = unimplemented!();
        (unsafe {
            f.clear_5(Some(
                (*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_seq_cst_2)),
            ))
        });
        unimplemented!();
        (unsafe {
            f.clear_5(Some(
                (*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_seq_cst_2)),
            ))
        });
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
