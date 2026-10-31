extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() -> bool {
    let mut a: [i32; 3] = [1, 2, 3];
    let mut b: [*mut i32; 3] = [
        (&mut a[(2) as usize] as *mut i32),
        (&mut a[(0) as usize] as *mut i32),
        (&mut a[(1) as usize] as *mut i32),
    ];
    let mut c: () = unimplemented!();
    assert!(((unimplemented!()) == ((b.as_mut_ptr().offset((0) as isize)).cast_const())));
    assert!(((unimplemented!()) == ((b.as_mut_ptr().offset((3) as isize)).cast_const())));
    let mut d: () = unimplemented!();
    assert!(((unimplemented!()) == (b.as_mut_ptr().offset((0) as isize))));
    assert!(((unimplemented!()) == (b.as_mut_ptr().offset((3) as isize))));
    let mut e: () = unimplemented!();
    assert!(((unimplemented!()) == ((b.as_mut_ptr().offset((0) as isize)).cast_const())));
    assert!(((unimplemented!()) == ((b.as_mut_ptr().offset((3) as isize)).cast_const())));
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
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
