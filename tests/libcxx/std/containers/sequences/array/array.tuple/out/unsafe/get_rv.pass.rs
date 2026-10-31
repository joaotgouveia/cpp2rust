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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let mut c: Vec<Option<Box<f64>>> = Vec::from([Some(Box::from_raw(
            (Box::leak(Box::new(3.5_f64)) as *mut f64),
        ))]);
        let mut t: Option<Box<f64>> = (*unimplemented!()).take();
        assert!(
            ((*t.as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const f64 as *mut f64))
                == (3.5_f64))
        );
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
