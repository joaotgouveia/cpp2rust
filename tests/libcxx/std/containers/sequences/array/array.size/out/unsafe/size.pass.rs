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
        let mut c: Vec<f64> = Vec::from([1_f64, 2_f64, 3.5_f64]);
        assert!(((c.len()) == (3_usize)));
        assert!(((unimplemented!()) == (3_usize)));
        assert!(!(unimplemented!()));
    }
    {
        let mut c: Vec<f64> = std::array::from_fn::<_, 0, _>(|_| 0_f64).to_vec();
        assert!(((c.len()) == (0_usize)));
        assert!(((unimplemented!()) == (0_usize)));
        assert!(unimplemented!());
    }
    {
        let c: Vec<f64> = Vec::from([1_f64, 2_f64, 3.5_f64]);
        const _: () = assert!(true, "c.size() == 3");;
        const _: () = assert!(true, "c.max_size() == 3");;
        const _: () = assert!(true, "!c.empty()");;
    }
    {
        let c: Vec<f64> = std::array::from_fn::<_, 0, _>(|_| 0_f64).to_vec();
        const _: () = assert!(true, "c.size() == 0");;
        const _: () = assert!(true, "c.max_size() == 0");;
        const _: () = assert!(true, "c.empty()");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
