extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct S {
    pub i_: i32,
}
impl S {
    pub unsafe fn new(mut i: i32) -> Self {
        let mut this = Self { i_: i };
        this
    }
    pub unsafe fn operator_lt(&self, mut lhs: S) -> bool {
        return ((lhs.i_) < (self.i_));
    }
}
impl std::cmp::Ord for S {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if S::operator_lt(self, other.clone()) {
                std::cmp::Ordering::Less
            } else if S::operator_lt(other, self.clone()) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for S {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { !(S::operator_lt(self, other.clone())) && !(S::operator_lt(other, self.clone())) }
    }
}
impl std::cmp::Eq for S {}
pub unsafe fn test_0(container: *mut UnsafeMap<*mut S, (*mut S, *mut ::libc::c_void)>) {
    {
        let c: *mut UnsafeMap<*mut S, (*mut S, *mut ::libc::c_void)> = container;
        let mut v: S = S::new({ 1 });
        assert!((*c).find(&&mut v) == (*c).end());
        assert!(((unimplemented!()) == (0_usize)));
        assert!(unimplemented!() == (*c).end());
        assert!(unimplemented!() == (*c).end());
        assert!(unimplemented!().0 == (*c).end());
        assert!(unimplemented!().1 == (*c).end());
        assert!(!(unimplemented!()));
    }
    {
        let c: *const UnsafeMap<*mut S, (*mut S, *mut ::libc::c_void)> = container;
        let mut v: S = S::new({ 1 });
        assert!((*c).find(&&mut v) == (*c).end());
        assert!(((unimplemented!()) == (0_usize)));
        assert!(unimplemented!() == (*c).end());
        assert!(unimplemented!() == (*c).end());
        assert!(unimplemented!().0 == (*c).end());
        assert!(unimplemented!().1 == (*c).end());
        assert!(!(unimplemented!()));
    }
}
pub unsafe fn test_1(container: *mut ()) {
    {
        let c: *mut () = container;
        let mut v: S = S::new({ 1 });
        assert!(unimplemented!() == unimplemented!());
        assert!(((unimplemented!()) == (0_())));
        assert!(unimplemented!() == unimplemented!());
        assert!(unimplemented!() == unimplemented!());
        assert!(unimplemented!().0 == unimplemented!());
        assert!(unimplemented!().1 == unimplemented!());
        assert!(!(unimplemented!()));
    }
    {
        let c: *const () = container;
        let mut v: S = S::new({ 1 });
        assert!(unimplemented!() == unimplemented!());
        assert!(((unimplemented!()) == (0_())));
        assert!(unimplemented!() == unimplemented!());
        assert!(unimplemented!() == unimplemented!());
        assert!(unimplemented!().0 == unimplemented!());
        assert!(unimplemented!().1 == unimplemented!());
        assert!(!(unimplemented!()));
    }
}
pub unsafe fn test_2(container: *mut ()) {
    {
        let c: *mut () = container;
        let mut v: S = S::new({ 1 });
        assert!(unimplemented!() == unimplemented!());
        assert!(((unimplemented!()) == (0_())));
        assert!(unimplemented!() == unimplemented!());
        assert!(unimplemented!() == unimplemented!());
        assert!(unimplemented!().0 == unimplemented!());
        assert!(unimplemented!().1 == unimplemented!());
        assert!(!(unimplemented!()));
    }
    {
        let c: *const () = container;
        let mut v: S = S::new({ 1 });
        assert!(unimplemented!() == unimplemented!());
        assert!(((unimplemented!()) == (0_())));
        assert!(unimplemented!() == unimplemented!());
        assert!(unimplemented!() == unimplemented!());
        assert!(unimplemented!().0 == unimplemented!());
        assert!(unimplemented!().1 == unimplemented!());
        assert!(!(unimplemented!()));
    }
}
pub unsafe fn test_3(container: *mut ()) {
    {
        let c: *mut () = container;
        let mut v: S = S::new({ 1 });
        assert!(unimplemented!() == unimplemented!());
        assert!(((unimplemented!()) == (0_())));
        assert!(unimplemented!() == unimplemented!());
        assert!(unimplemented!() == unimplemented!());
        assert!(unimplemented!().0 == unimplemented!());
        assert!(unimplemented!().1 == unimplemented!());
        assert!(!(unimplemented!()));
    }
    {
        let c: *const () = container;
        let mut v: S = S::new({ 1 });
        assert!(unimplemented!() == unimplemented!());
        assert!(((unimplemented!()) == (0_())));
        assert!(unimplemented!() == unimplemented!());
        assert!(unimplemented!() == unimplemented!());
        assert!(unimplemented!().0 == unimplemented!());
        assert!(unimplemented!().1 == unimplemented!());
        assert!(!(unimplemented!()));
    }
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
    {
        let mut m: UnsafeMap<*mut S, (*mut S, *mut ::libc::c_void)> = UnsafeMap::new();
        (unsafe { test_0(&mut m) });
    }
    {
        let mut m: () = unimplemented!();
        (unsafe { test_1(&mut m) });
    }
    {
        let mut s: () = unimplemented!();
        (unsafe { test_2(&mut s) });
    }
    {
        let mut s: () = unimplemented!();
        (unsafe { test_3(&mut s) });
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
