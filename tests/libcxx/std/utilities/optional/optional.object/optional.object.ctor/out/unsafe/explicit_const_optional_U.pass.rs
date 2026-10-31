extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(rhs: *const (), mut is_going_to_throw: Option<bool>) {
    let mut is_going_to_throw: bool = is_going_to_throw.unwrap_or_else(|| unsafe { false });
    const _: () = assert!(
        true,
        "!(std::is_convertible<const optional<U>&, optional<T>>::value)"
    );;
    let mut rhs_engaged: bool = unimplemented!();
    if is_going_to_throw {
        return;
    }
    let mut lhs: () = unimplemented!();
    assert!(((unimplemented!() as i32) == (rhs_engaged as i32)));
    if rhs_engaged {
        assert!(
            (unsafe {
                let _x: *const X = &(*unimplemented!());
                let mut _y: X = X::new({ (*unimplemented!()) });
                operator_eq_1(_x, &mut _y)
            })
        );
    }
}
pub unsafe fn test_2(rhs: *const (), mut is_going_to_throw: Option<bool>) {
    let mut is_going_to_throw: bool = is_going_to_throw.unwrap_or_else(|| unsafe { false });
    const _: () = assert!(
        true,
        "!(std::is_convertible<const optional<U>&, optional<T>>::value)"
    );;
    let mut rhs_engaged: bool = unimplemented!();
    if is_going_to_throw {
        return;
    }
    let mut lhs: () = unimplemented!();
    assert!(((unimplemented!() as i32) == (rhs_engaged as i32)));
    if rhs_engaged {
        assert!(
            (unsafe {
                let _x: *const Y = &(*unimplemented!());
                let mut _y: Y = Y::new({ (*unimplemented!()) });
                operator_eq_3(_x, &mut _y)
            })
        );
    }
}
pub unsafe fn test_4(rhs: *const (), mut is_going_to_throw: Option<bool>) {
    let mut is_going_to_throw: bool = is_going_to_throw.unwrap_or_else(|| unsafe { false });
    const _: () = assert!(
        true,
        "!(std::is_convertible<const optional<U>&, optional<T>>::value)"
    );;
    let mut rhs_engaged: bool = unimplemented!();
    if is_going_to_throw {
        return;
    }
    let mut lhs: () = unimplemented!();
    assert!(((unimplemented!() as i32) == (rhs_engaged as i32)));
    if rhs_engaged {
        assert!(
            (unsafe {
                let _x: *const Z = &(*unimplemented!());
                let mut _y: Z = Z::new({ (*unimplemented!()) });
                operator_eq_5(_x, &mut _y)
            })
        );
    }
}
pub unsafe fn operator_eq_1(x: *const X, y: *const X) -> bool {
    return (((*x).i_) == ((*y).i_));
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe, Default)]
pub struct X {
    i_: i32,
}
impl X {
    pub unsafe fn new(mut i: i32) -> Self {
        let mut this = Self { i_: i };
        this
    }
    pub unsafe fn copy_from(x: *const X) -> Self {
        let mut this = Self { i_: (*x).i_ };
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.i_ = 0;
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const X) -> *mut X {
        self.i_ = (*_a0).i_;
        return &mut (*(self as *mut X));
    }
}
impl std::cmp::PartialEq for X {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_1(self as *const X, other as *const X) }
    }
}
impl std::cmp::Eq for X {}
impl Clone for X {
    fn clone(&self) -> Self {
        unsafe { X::copy_from(self as *const X) }
    }
}
pub unsafe fn operator_eq_3(x: *const Y, y: *const Y) -> bool {
    return (((*x).i_) == ((*y).i_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Y {
    i_: i32,
}
impl Y {
    pub unsafe fn new(mut i: i32) -> Self {
        let mut this = Self { i_: i };
        this
    }
}
impl std::cmp::PartialEq for Y {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_3(self as *const Y, other as *const Y) }
    }
}
impl std::cmp::Eq for Y {}
pub static mut count_6: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn operator_eq_5(x: *const Z, y: *const Z) -> bool {
    return (((*x).i_) == ((*y).i_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Z {
    i_: i32,
}
impl Z {
    pub unsafe fn new(mut i: i32) -> Self {
        let mut this = Self { i_: i };
        std::process::abort();
        this
    }
}
impl std::cmp::PartialEq for Z {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_5(self as *const Z, other as *const Z) }
    }
}
impl std::cmp::Eq for Z {}
pub unsafe fn test_all_7() -> bool {
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
pub unsafe fn test_all_8() -> bool {
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_all_7() });
    (unsafe { test_all_8() });
    const _: () = assert!(true, "test_all<X, int>()");;
    const _: () = assert!(true, "test_all<Y, int>()");;
    {
        let mut rhs: () = unimplemented!();
        (unsafe { test_4(&rhs, None) });
    }
    {
        let mut rhs: () = unimplemented!();
        (unsafe { test_4(&rhs, Some(true)) });
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_6);
}
