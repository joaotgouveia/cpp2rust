extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_convertible_imp_0(mut _a0: i64) -> bool {
    return false;
}
pub unsafe fn test_convertible_1() -> bool {
    return (unsafe { test_convertible_imp_0(0_i64) });
}
pub unsafe fn make_2(mut n: i32) -> Vec<i32> {
    let mut c: Vec<i32> = Vec::new();
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        c.push(i);
        i.prefix_inc();
    }
    return std::mem::take(&mut c);
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
    let mut d: Vec<i32> = (unsafe { make_2(5) });
    let mut q: () = unimplemented!();
    assert!(((unimplemented!()) == (5_())));
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (unimplemented!())) {
        assert!(((*unimplemented!()) == (*unimplemented!())));
        unimplemented!();
        i.prefix_inc();
    }
    const _: () = assert!(true, "!test_convertible<Q, const Container&>()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
