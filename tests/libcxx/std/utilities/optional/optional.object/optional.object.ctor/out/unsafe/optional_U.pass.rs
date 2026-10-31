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
    let mut rhs_engaged: bool = unimplemented!();
    if is_going_to_throw {
        return;
    }
    let mut lhs: () = unimplemented!();
    assert!(((unimplemented!() as i32) == (rhs_engaged as i32)));
}
pub unsafe fn test_1(rhs: *mut (), mut is_going_to_throw: Option<bool>) {
    let mut is_going_to_throw: bool = is_going_to_throw.unwrap_or_else(|| unsafe { false });
    let mut rhs_engaged: bool = unimplemented!();
    if is_going_to_throw {
        return;
    }
    let mut lhs: () = unimplemented!();
    assert!(((unimplemented!() as i32) == (rhs_engaged as i32)));
}
pub unsafe fn test_2(rhs: *mut (), mut is_going_to_throw: Option<bool>) {
    let mut is_going_to_throw: bool = is_going_to_throw.unwrap_or_else(|| unsafe { false });
    let mut rhs_engaged: bool = unimplemented!();
    if is_going_to_throw {
        return;
    }
    let mut lhs: () = unimplemented!();
    assert!(((unimplemented!() as i32) == (rhs_engaged as i32)));
}
pub unsafe fn operator_eq_3(x: *const X, y: *const X) -> bool {
    return (((*x).i_) == ((*y).i_));
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, DestructorUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct X {
    i_: i32,
}
impl X {
    pub unsafe fn new(mut i: i32) -> Self {
        let mut this = Self { i_: i };
        this
    }
    pub unsafe fn move_from(x: *mut X) -> Self {
        let mut this = Self {
            i_: unimplemented!(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.i_ = 0;
    }
}
impl std::cmp::PartialEq for X {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_3(self as *const X, other as *const X) }
    }
}
impl std::cmp::Eq for X {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct Z {}
impl Z {
    pub unsafe fn new(mut _a0: i32) -> Self {
        let mut this = Self {};
        std::process::abort();
        this
    }
}
pub unsafe fn test_all_4() -> bool {
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
pub unsafe fn test_all_5() -> bool {
    {
        let mut rhs: () = unimplemented!();
        (unsafe { test_1(&mut rhs, None) });
    }
    {
        let mut rhs: () = unimplemented!();
        (unsafe { test_1(&mut rhs, None) });
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
    (unsafe { test_all_4() });
    (unsafe { test_all_5() });
    const _: () = assert!(true, "test_all<short, int>()");;
    const _: () = assert!(true, "test_all<int, X>()");;
    {
        let mut rhs: () = unimplemented!();
        (unsafe { test_2(&mut rhs, None) });
    }
    {
        let mut rhs: () = unimplemented!();
        (unsafe { test_2(&mut rhs, Some(true)) });
    }
    const _: () = assert!(
        true,
        "!(std::is_constructible<optional<X>, optional<Z>>::value)"
    );;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
