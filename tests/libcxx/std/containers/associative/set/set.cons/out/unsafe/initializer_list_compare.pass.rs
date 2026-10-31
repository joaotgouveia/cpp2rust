extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(a: *const test_less_int_, b: *const test_less_int_) -> bool {
    return (((*a).data_) == ((*b).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_less_int_ {
    pub data_: i32,
}
impl test_less_int_ {
    pub unsafe fn new_1(mut data: i32) -> Self {
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn operator_call(&self, a: *const i32, b: *const i32) -> bool {
        return ((*a) < (*b));
    }
}
impl std::cmp::PartialEq for test_less_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_0(
                self as *const test_less_int_,
                other as *const test_less_int_,
            )
        }
    }
}
impl std::cmp::Eq for test_less_int_ {}
impl Callable2<*const i32, *const i32, bool> for test_less_int_ {
    fn call(&self, a1: *const i32, a2: *const i32) -> bool {
        unsafe { test_less_int_::operator_call(self, a1, a2) }
    }
}
pub unsafe fn test_1() -> bool {
    let mut m: () = unimplemented!();
    assert!(((unimplemented!()) == (6_())));
    assert!(((unimplemented!()) == (6_isize)));
    let mut i: () = unimplemented!();
    assert!(((*i.iter_deref()) == (1 as ())));
    assert!(((*(*i.iter_inc()).iter_deref()) == (2 as ())));
    assert!(((*(*i.iter_inc()).iter_deref()) == (3 as ())));
    assert!(((*(*i.iter_inc()).iter_deref()) == (4 as ())));
    assert!(((*(*i.iter_inc()).iter_deref()) == (5 as ())));
    assert!(((*(*i.iter_inc()).iter_deref()) == (6 as ())));
    assert!(
        (unsafe {
            let mut _a: test_less_int_ = unimplemented!();
            let mut _b: test_less_int_ = test_less_int_::new_1({ 10 });
            operator_eq_0(&mut _a, &mut _b)
        })
    );
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
    (unsafe { test_1() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
