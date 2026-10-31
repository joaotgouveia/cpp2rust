extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
    let mut z: () = unimplemented!();
    let mut a: *mut f32 = ((&mut z as *mut ()) as *mut f32);
    assert!(((0_f32) == (unimplemented!())));
    assert!(((0_f32) == (unimplemented!())));
    assert!(((*a.offset((0) as isize)) == (unimplemented!())));
    assert!(((*a.offset((1) as isize)) == (unimplemented!())));
    (*a.offset((0) as isize)) = 5_f32;
    (*a.offset((1) as isize)) = 6_f32;
    assert!(((*a.offset((0) as isize)) == (unimplemented!())));
    assert!(((*a.offset((1) as isize)) == (unimplemented!())));
}
pub unsafe fn test_1() {
    let mut z: () = unimplemented!();
    let mut a: *mut f64 = ((&mut z as *mut ()) as *mut f64);
    assert!(((0_f64) == (unimplemented!())));
    assert!(((0_f64) == (unimplemented!())));
    assert!(((*a.offset((0) as isize)) == (unimplemented!())));
    assert!(((*a.offset((1) as isize)) == (unimplemented!())));
    (*a.offset((0) as isize)) = 5_f64;
    (*a.offset((1) as isize)) = 6_f64;
    assert!(((*a.offset((0) as isize)) == (unimplemented!())));
    assert!(((*a.offset((1) as isize)) == (unimplemented!())));
}
pub unsafe fn test_2() {
    let mut z: () = unimplemented!();
    let mut a: *mut f64 = ((&mut z as *mut ()) as *mut f64);
    assert!(((0_f64) == (unimplemented!())));
    assert!(((0_f64) == (unimplemented!())));
    assert!(((*a.offset((0) as isize)) == (unimplemented!())));
    assert!(((*a.offset((1) as isize)) == (unimplemented!())));
    (*a.offset((0) as isize)) = 5_f64;
    (*a.offset((1) as isize)) = 6_f64;
    assert!(((*a.offset((0) as isize)) == (unimplemented!())));
    assert!(((*a.offset((1) as isize)) == (unimplemented!())));
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
    (unsafe { test_1() });
    (unsafe { test_2() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
