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
pub struct Y {
    pub i_: i32,
}
impl Y {
    pub unsafe fn new(mut i: i32) -> Self {
        let mut this = Self { i_: i };
        this
    }
}
pub unsafe fn operator_eq_0(x: *const X, y: *const X) -> bool {
    return (((*x).i_) == ((*y).i_));
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[cpp_from(i32, new_1, value)]
#[cpp_from(Y, new_2, ptr)]
#[cpp_from_move(Y, new_3, ptr)]
pub struct X {
    pub i_: i32,
}
impl X {
    pub unsafe fn new_1(mut i: i32) -> Self {
        let mut this = Self { i_: i };
        this
    }
    pub unsafe fn move_from(x: *mut X) -> Self {
        let mut this = Self { i_: (*x).i_ };
        (*x).i_ = 0;
        this
    }
    pub unsafe fn new_2(y: *const Y) -> Self {
        let mut this = Self { i_: (*y).i_ };
        this
    }
    pub unsafe fn new_3(y: *mut Y) -> Self {
        let mut this = Self {
            i_: (((*y).i_) + (1)),
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
pub unsafe fn operator_eq_1(z1: *const Z, z2: *const Z) -> bool {
    return (((*z1).i_) == ((*z2).i_)) && (((*z1).j_) == ((*z2).j_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Z {
    pub i_: i32,
    pub j_: i32,
}
impl Z {
    pub unsafe fn new(mut i: i32, mut j: i32) -> Self {
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
pub unsafe fn test_2() -> i32 {
    {
        let mut opt: () = unimplemented!();
        let mut y: Y = Y::new({ 3 });
        assert!(
            (unsafe {
                let mut _x: X = unimplemented!();
                let mut _y: X = X::new_1({ 2 });
                operator_eq_0(&mut _x, &mut _y)
            })
        );
        assert!(
            (unsafe {
                let _x: *const X = &(*unimplemented!());
                let mut _y: X = X::new_1({ 0 });
                operator_eq_0(_x, &mut _y)
            })
        );
    }
    {
        let mut opt: () = unimplemented!();
        assert!(
            (unsafe {
                let mut _x: X = unimplemented!();
                let mut _y: X = X::new_1({ 2 });
                operator_eq_0(&mut _x, &mut _y)
            })
        );
        assert!(
            (unsafe {
                let _x: *const X = &(*unimplemented!());
                let mut _y: X = X::new_1({ 0 });
                operator_eq_0(_x, &mut _y)
            })
        );
    }
    {
        let mut opt: () = unimplemented!();
        let mut y: Y = Y::new({ 3 });
        assert!(
            (unsafe {
                let mut _x: X = unimplemented!();
                let mut _y: X = X::new_1({ 3 });
                operator_eq_0(&mut _x, &mut _y)
            })
        );
        assert!(!(unimplemented!()));
    }
    {
        let mut opt: () = unimplemented!();
        assert!(
            (unsafe {
                let mut _x: X = unimplemented!();
                let mut _y: X = X::new_1({ 4 });
                operator_eq_0(&mut _x, &mut _y)
            })
        );
        assert!(!(unimplemented!()));
    }
    {
        let mut opt: () = unimplemented!();
        assert!(
            (unsafe {
                let mut _x: X = unimplemented!();
                let mut _y: X = X::new_1({ 4 });
                operator_eq_0(&mut _x, &mut _y)
            })
        );
        assert!(!(unimplemented!()));
    }
    {
        let mut opt: () = unimplemented!();
        assert!(
            (unsafe {
                let mut _z1: Z = unimplemented!();
                let mut _z2: Z = Z::new({ 2 }, { 3 });
                operator_eq_1(&mut _z1, &mut _z2)
            })
        );
        assert!(!(unimplemented!()));
    }
    return 0;
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
    assert!(((unsafe { test_2() }) == (0)));
    const _: () = assert!(true, "test() == 0");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
