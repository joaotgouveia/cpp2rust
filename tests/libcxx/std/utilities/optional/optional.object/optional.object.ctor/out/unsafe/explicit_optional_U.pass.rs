extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(rhs: *mut (), mut is_going_to_throw: Option<()>) {
    let mut is_going_to_throw: () = is_going_to_throw.unwrap_or_else(|| unsafe { false });
    const _: () = assert!(
        true,
        "!(std::is_convertible<optional<U>&&, optional<T>>::value)"
    );;
    let mut rhs_engaged: () = unimplemented!();
    if is_going_to_throw {
        return;
    }
    let mut lhs: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_1(rhs: *mut (), mut is_going_to_throw: Option<()>) {
    let mut is_going_to_throw: () = is_going_to_throw.unwrap_or_else(|| unsafe { false });
    const _: () = assert!(
        true,
        "!(std::is_convertible<optional<U>&&, optional<T>>::value)"
    );;
    let mut rhs_engaged: () = unimplemented!();
    if is_going_to_throw {
        return;
    }
    let mut lhs: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn operator_eq_2(x: *const X, y: *const X) -> () {
    return (((*x).i_) == ((*y).i_));
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, DestructorUnsafe, Default)]
pub struct X {
    i_: (),
}
impl X {
    pub unsafe fn new(mut i: ()) -> Self {
        let mut this = Self { i_: i };
        this
    }
    pub unsafe fn move_from(x: *mut X) -> Self {
        let mut this = Self { i_: (*x).i_ };
        (*x).i_ = 0_();
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.i_ = 0_();
    }
}
impl std::cmp::PartialEq for X {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_2(self as *const X, other as *const X) }
    }
}
impl std::cmp::Eq for X {}
pub static mut count_3: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Z {}
impl Z {
    pub unsafe fn new(mut _a0: ()) -> Self {
        let mut this = Self {};
        unimplemented!();
        this
    }
}
pub unsafe fn test_4() -> () {
    {
        let mut rhs: () = unimplemented!();
        (unsafe { test_0(&mut (*unimplemented!()), None) });
    }
    {
        let mut rhs: () = unimplemented!();
        (unsafe { test_0(&mut (*unimplemented!()), None) });
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
    const _: () = assert!(true, "test()");;
    (unsafe { test_4() });
    {
        let mut rhs: () = unimplemented!();
        (unsafe { test_1(&mut (*unimplemented!()), None) });
    }
    {
        let mut rhs: () = unimplemented!();
        (unsafe { test_1(&mut (*unimplemented!()), Some(true)) });
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_3);
}
