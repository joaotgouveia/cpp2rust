extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(rhs: *const (), mut is_going_to_throw: Option<()>) {
    let mut is_going_to_throw: () = is_going_to_throw.unwrap_or_else(|| unsafe { false });
    let mut rhs_engaged: () = (unsafe { (*rhs)() });
    if is_going_to_throw {
        return;
    }
    let mut lhs: () = unimplemented!();
    unimplemented!();
    if rhs_engaged {
        unimplemented!();
    }
}
pub unsafe fn test_1(rhs: *const (), mut is_going_to_throw: Option<()>) {
    let mut is_going_to_throw: () = is_going_to_throw.unwrap_or_else(|| unsafe { false });
    let mut rhs_engaged: () = (unsafe { (*rhs)() });
    if is_going_to_throw {
        return;
    }
    let mut lhs: () = unimplemented!();
    unimplemented!();
    if rhs_engaged {
        unimplemented!();
    }
}
pub unsafe fn test_2(rhs: *const (), mut is_going_to_throw: Option<()>) {
    let mut is_going_to_throw: () = is_going_to_throw.unwrap_or_else(|| unsafe { false });
    let mut rhs_engaged: () = (unsafe { (*rhs)() });
    if is_going_to_throw {
        return;
    }
    let mut lhs: () = unimplemented!();
    unimplemented!();
    if rhs_engaged {
        unimplemented!();
    }
}
pub unsafe fn test_3(rhs: *const (), mut is_going_to_throw: Option<()>) {
    let mut is_going_to_throw: () = is_going_to_throw.unwrap_or_else(|| unsafe { false });
    let mut rhs_engaged: () = (unsafe { (*rhs)() });
    if is_going_to_throw {
        return;
    }
    let mut lhs: () = unimplemented!();
    unimplemented!();
    if rhs_engaged {
        unimplemented!();
    }
}
pub unsafe fn operator_eq_4(x: *const X, y: *const X) -> () {
    return (((*x).i_) == ((*y).i_));
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe, Default)]
pub struct X {
    i_: (),
}
impl X {
    pub unsafe fn new(mut i: ()) -> Self {
        let mut this = Self { i_: i };
        this
    }
    pub unsafe fn copy_from(x: *const X) -> Self {
        let mut this = Self { i_: (*x).i_ };
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.i_ = 0_();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const X) -> *mut X {
        self.i_ = (*_a0).i_;
        return &mut (*(self as *mut X));
    }
}
impl std::cmp::PartialEq for X {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_4(self as *const X, other as *const X) }
    }
}
impl std::cmp::Eq for X {}
impl Clone for X {
    fn clone(&self) -> Self {
        unsafe { X::copy_from(self as *const X) }
    }
}
pub unsafe fn operator_eq_5(x: *const Y, y: *const Y) -> () {
    return (((*x).i_) == ((*y).i_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Y {
    i_: (),
}
impl Y {
    pub unsafe fn new(mut i: ()) -> Self {
        let mut this = Self { i_: i };
        this
    }
}
impl std::cmp::PartialEq for Y {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_5(self as *const Y, other as *const Y) }
    }
}
impl std::cmp::Eq for Y {}
pub static mut count_6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn operator_eq_7(x: *const Z, y: *const Z) -> () {
    return (((*x).i_) == ((*y).i_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Z {
    i_: (),
}
impl Z {
    pub unsafe fn new(mut i: ()) -> Self {
        let mut this = Self { i_: i };
        unimplemented!();
        this
    }
}
impl std::cmp::PartialEq for Z {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_7(self as *const Z, other as *const Z) }
    }
}
impl std::cmp::Eq for Z {}
pub unsafe fn test_all_8() -> () {
    {
        let mut rhs: () = unimplemented!();
        (unsafe { test_0(&rhs, None) });
    }
    {
        let mut rhs: () = unimplemented!();
        (unsafe { test_0(&rhs, None) });
    }
    return true;
}
pub unsafe fn test_all_9() -> () {
    {
        let mut rhs: () = unimplemented!();
        (unsafe { test_1(&rhs, None) });
    }
    {
        let mut rhs: () = unimplemented!();
        (unsafe { test_1(&rhs, None) });
    }
    return true;
}
pub unsafe fn test_all_10() -> () {
    {
        let mut rhs: () = unimplemented!();
        (unsafe { test_2(&rhs, None) });
    }
    {
        let mut rhs: () = unimplemented!();
        (unsafe { test_2(&rhs, None) });
    }
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
    (unsafe { test_all_8() });
    (unsafe { test_all_9() });
    (unsafe { test_all_10() });
    const _: () = assert!(true, "test_all<int, short>()");;
    const _: () = assert!(true, "test_all<X, int>()");;
    const _: () = assert!(true, "test_all<Y, int>()");;
    {
        let mut rhs: () = unimplemented!();
        (unsafe { test_3(&rhs, None) });
    }
    {
        let mut rhs: () = unimplemented!();
        (unsafe { test_3(&rhs, Some(true)) });
    }
    const _: () = assert!(
        true,
        "!(std::is_constructible<optional<X>, const optional<Y>&>::value)"
    );;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_6);
}
