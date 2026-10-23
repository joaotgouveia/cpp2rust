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
pub struct Y {
    pub i_: (),
}
impl Y {
    pub unsafe fn new(mut i: ()) -> Self {
        let mut this = Self { i_: i };
        this
    }
}
pub unsafe fn operator_eq_0(x: *const X, y: *const X) -> () {
    return (((*x).i_) == ((*y).i_));
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct X {
    pub i_: (),
}
impl X {
    pub unsafe fn new_1(mut i: ()) -> Self {
        let mut this = Self { i_: i };
        this
    }
    pub unsafe fn move_from(x: *mut X) -> Self {
        let mut this = Self { i_: (*x).i_ };
        (*x).i_ = 0_();
        this
    }
    pub unsafe fn new_2(y: *const Y) -> Self {
        let mut this = Self { i_: (*y).i_ };
        this
    }
    pub unsafe fn new_3(y: *mut Y) -> Self {
        let mut this = Self {
            i_: (((*y).i_) + (1_())),
        };
        this
    }
}
impl std::cmp::PartialEq for X {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const X, other as *const X) }
    }
}
impl std::cmp::Eq for X {}
pub unsafe fn operator_eq_1(z1: *const Z, z2: *const Z) -> () {
    return (((*z1).i_) == ((*z2).i_)) && (((*z1).j_) == ((*z2).j_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Z {
    pub i_: (),
    pub j_: (),
}
impl Z {
    pub unsafe fn new(mut i: (), mut j: ()) -> Self {
        let mut this = Self { i_: i, j_: j };
        this
    }
}
impl std::cmp::PartialEq for Z {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_1(self as *const Z, other as *const Z) }
    }
}
impl std::cmp::Eq for Z {}
pub unsafe fn test_2() -> () {
    {
        let mut opt: () = {
            let mut __tmp_0: () = 2_();
            std_optional_X_::new_1({ unimplemented!() }, { &mut __tmp_0 })
        };
        let mut y: Y = Y::new({ 3_() });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut opt: () = {
            let mut __tmp_1: () = 2_();
            std_optional_X_::new_1({ unimplemented!() }, { &mut __tmp_1 })
        };
        unimplemented!();
        unimplemented!();
    }
    {
        let mut opt: () = unimplemented!();
        let mut y: Y = Y::new({ 3_() });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut opt: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut opt: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut opt: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
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
    unimplemented!();
    const _: () = assert!(true, "test() == 0");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
