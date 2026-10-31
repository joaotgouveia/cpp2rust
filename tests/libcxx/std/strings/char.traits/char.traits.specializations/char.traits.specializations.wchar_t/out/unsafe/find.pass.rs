extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_constexpr_0() -> bool {
    let p: *const i32 = [49 as i32, 50 as i32, 51 as i32, 0 as i32].as_ptr();
    return ((((unimplemented!()) == (p)) && ((unimplemented!()) == (p.offset((1) as isize))))
        && ((unimplemented!()) == (p.offset((2) as isize))))
        && ((unimplemented!()).is_null());
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
    let mut s1: [i32; 3] = [1_i32, 2_i32, 3_i32];
    assert!(((unimplemented!()) == ((s1.as_mut_ptr()).cast_const())));
    assert!(((unimplemented!()) == ((s1.as_mut_ptr().offset((1) as isize)).cast_const())));
    assert!(((unimplemented!()) == ((s1.as_mut_ptr().offset((2) as isize)).cast_const())));
    assert!((unimplemented!()).is_null());
    assert!((unimplemented!()).is_null());
    assert!((unimplemented!()).is_null());
    const _: () = assert!(true, "test_constexpr()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
