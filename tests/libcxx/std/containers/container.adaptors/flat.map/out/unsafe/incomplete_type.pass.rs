extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct A {
    pub data: (),
    pub m: (),
    pub it: (),
    pub cit: (),
}
impl A {
    pub unsafe fn move_from(_a0: *mut A) -> Self {
        let mut this = Self {
            data: (*_a0).data,
            m: (*_a0).m,
            it: (*_a0).it,
            cit: (*_a0).cit,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut A) -> *mut A {
        self.data = (*_a0).data;
        self.m = ((*_a0).m).clone();
        unimplemented!();
        unimplemented!();
        return &mut (*(self as *mut A));
    }
}
impl std::cmp::Ord for A {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_0(self as *const A, other as *const A) {
                std::cmp::Ordering::Less
            } else if operator_lt_0(other as *const A, self as *const A) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for A {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for A {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_lt_0(self as *const A, other as *const A))
                && !(operator_lt_0(other as *const A, self as *const A))
        }
    }
}
impl std::cmp::Eq for A {}
pub unsafe fn operator_lt_0(L: *const A, R: *const A) -> () {
    return (((*L).data) < ((*R).data));
}
pub unsafe fn test_1() -> () {
    let mut a: A = <A>::default();
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_1() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
