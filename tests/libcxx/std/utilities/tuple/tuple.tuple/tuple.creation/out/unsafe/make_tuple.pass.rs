extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() -> bool {
    let mut i: i32 = 0;
    let mut j: f32 = 0_f32;
    let mut t: () = unimplemented!();
    assert!(((*unimplemented!()) == (1)));
    assert!(((*unimplemented!()) == (0)));
    assert!(((*unimplemented!()) == (0_f32)));
    i = 2;
    j = (3.5_f64 as f32);
    assert!(((*unimplemented!()) == (1)));
    assert!(((*unimplemented!()) == (2)));
    assert!((((*unimplemented!()) as f64) == (3.5_f64)));
    (*unimplemented!()) = 0;
    (*unimplemented!()) = 0_f32;
    assert!(((i) == (0)));
    assert!(((j) == (0_f32)));
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    {
        let t1: () = unimplemented!();
        let i1: i32 = ((*unimplemented!()) as i32);
        let d1: f64 = (*unimplemented!());
        const _: () = assert!(true, "i1 == 1");;
        const _: () = assert!(true, "d1 == 3.14");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
