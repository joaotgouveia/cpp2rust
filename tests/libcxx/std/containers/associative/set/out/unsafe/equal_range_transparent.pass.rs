extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Comp {}
impl Comp {
    pub unsafe fn operator_call_1(&self, lhs: *const (i32, i32), rhs: *const (i32, i32)) -> bool {
        return unimplemented!() == std::cmp::Ordering::Less;
    }
    pub unsafe fn operator_call_2(&self, lhs: *const (i32, i32), mut rhs: i32) -> bool {
        return (((*lhs).0) < (rhs));
    }
    pub unsafe fn operator_call_3(&self, mut lhs: i32, rhs: *const (i32, i32)) -> bool {
        return ((lhs) < ((*rhs).0));
    }
}
pub unsafe fn test_0() -> bool {
    let mut s: () = unimplemented!();
    let mut er: (
        UnsafeBidirectionalIterator<(i32, i32)>,
        UnsafeBidirectionalIterator<(i32, i32)>,
    ) = unimplemented!();
    let mut nels: i64 = 0_i64;
    let mut it: UnsafeBidirectionalIterator<(i32, i32)> = er.0.clone();
    'loop_: while it != er.1 {
        assert!((((*it.iter_deref()).0) == (1)));
        nels.postfix_inc();
        it.iter_post_inc();
    }
    assert!(((nels) == (3_i64)));
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
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
