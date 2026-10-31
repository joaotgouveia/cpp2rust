extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(rhs: *mut (), mut is_going_to_throw: Option<bool>) {
    let mut is_going_to_throw: bool = is_going_to_throw.unwrap_or_else(|| unsafe { false });
    const _: () = assert!(
        true,
        "!(std::is_convertible<optional<U>&&, optional<T>>::value)"
    );;
    let mut rhs_engaged: bool = unimplemented!();
    if is_going_to_throw {
        return;
    }
    let mut lhs: () = unimplemented!();
    assert!(((unimplemented!() as i32) == (rhs_engaged as i32)));
}
pub unsafe fn test_1(rhs: *mut (), mut is_going_to_throw: Option<bool>) {
    let mut is_going_to_throw: bool = is_going_to_throw.unwrap_or_else(|| unsafe { false });
    const _: () = assert!(
        true,
        "!(std::is_convertible<optional<U>&&, optional<T>>::value)"
    );;
    let mut rhs_engaged: bool = unimplemented!();
    if is_going_to_throw {
        return;
    }
    let mut lhs: () = unimplemented!();
    assert!(((unimplemented!() as i32) == (rhs_engaged as i32)));
}
pub unsafe fn operator_eq_2(x: *const X, y: *const X) -> bool {
    return (((*x).i_) == ((*y).i_));
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, DestructorUnsafe, Default)]
pub struct X {
    i_: i32,
}
impl X {
    pub unsafe fn new(mut i: i32) -> Self {
        let mut this = Self { i_: i };
        this
    }
    pub unsafe fn move_from(x: *mut X) -> Self {
        let mut this = Self { i_: (*x).i_ };
        (*x).i_ = 0;
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.i_ = 0;
    }
}
impl std::cmp::PartialEq for X {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_2(self as *const X, other as *const X) }
    }
}
impl std::cmp::Eq for X {}
pub static mut count_3: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Z {}
impl Z {
    pub unsafe fn new(mut _a0: i32) -> Self {
        let mut this = Self {};
        std::process::abort();
        this
    }
}
pub unsafe fn test_4() -> bool {
    {
        let mut rhs: () = unimplemented!();
        (unsafe { test_0(&mut rhs, None) });
    }
    {
        let mut rhs: () = unimplemented!();
        (unsafe { test_0(&mut rhs, None) });
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    const _: () = assert!(true, "test()");;
    (unsafe { test_4() });
    {
        let mut rhs: () = unimplemented!();
        (unsafe { test_1(&mut rhs, None) });
    }
    {
        let mut rhs: () = unimplemented!();
        (unsafe { test_1(&mut rhs, Some(true)) });
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_3);
}
