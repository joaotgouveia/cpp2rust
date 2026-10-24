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
pub struct IntType {
    pub val: (),
}
impl IntType {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
    pub unsafe fn operator_eq(&self, rhs: *const IntType) -> () {
        return ((self.val) == ((*rhs).val));
    }
    pub unsafe fn to__4(&self) -> () {
        return self.val;
    }
    pub unsafe fn to__5(&self) -> () {
        return self.val;
    }
    pub unsafe fn to__6(&self) -> () {
        return self.val;
    }
}
impl std::cmp::PartialEq for IntType {
    fn eq(&self, other: &Self) -> bool {
        unsafe { IntType::operator_eq(self, other as *const IntType) }
    }
}
impl std::cmp::Eq for IntType {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IntTypeNC {
    pub val: (),
}
impl IntTypeNC {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
    pub unsafe fn operator_eq(&self, rhs: *const IntType) -> () {
        return ((self.val) == ((*rhs).val));
    }
    pub unsafe fn to__4(&mut self) -> () {
        return self.val;
    }
    pub unsafe fn to__5(&mut self) -> () {
        return self.val;
    }
    pub unsafe fn to__6(&mut self) -> () {
        return self.val;
    }
}
pub unsafe fn test_construction_0(mut e: (), mut s: ()) {
    const _: () = assert!(true, "");;
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!()) == (0_())) {
            expected_size = 0_();
            break;
        }
        expected_size += (((unimplemented!()) - (1_())) * ((*unimplemented!()) as ()));
        r.postfix_inc();
    }
    unimplemented!();
    let mut strides: () = unimplemented!();
    const _: () = assert!(true, "");;
    {}
}
pub unsafe fn test_construction_1(mut e: (), mut s: ()) {
    const _: () = assert!(true, "");;
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!()) == (0_())) {
            expected_size = 0_();
            break;
        }
        expected_size = (expected_size).wrapping_add(
            ((unimplemented!()).wrapping_sub(1_())).wrapping_mul((*unimplemented!())),
        );
        r.postfix_inc();
    }
    unimplemented!();
    let mut strides: () = unimplemented!();
    const _: () = assert!(true, "");;
    {
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            unimplemented!();
            unimplemented!();
            r.postfix_inc();
        }
    }
}
pub unsafe fn test_construction_2(mut e: (), mut s: ()) {
    const _: () = assert!(true, "");;
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!()) == (0_())) {
            expected_size = 0_();
            break;
        }
        expected_size = (expected_size).wrapping_add(
            ((unimplemented!()).wrapping_sub(1_())).wrapping_mul((*unimplemented!())),
        );
        r.postfix_inc();
    }
    unimplemented!();
    let mut strides: () = unimplemented!();
    const _: () = assert!(true, "");;
    {
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            unimplemented!();
            unimplemented!();
            r.postfix_inc();
        }
    }
}
pub unsafe fn test_construction_3(mut e: (), mut s: ()) {
    const _: () = assert!(true, "");;
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!()) == (0_())) {
            expected_size = 0_();
            break;
        }
        expected_size += (((unimplemented!()) - (1_()))
            * ((unsafe { IntType::to__4(&(*unimplemented!())) }) as ()));
        r.postfix_inc();
    }
    unimplemented!();
    let mut strides: () = unimplemented!();
    const _: () = assert!(true, "");;
    {
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            unimplemented!();
            unimplemented!();
            r.postfix_inc();
        }
    }
}
pub unsafe fn test_construction_4(mut e: (), mut s: ()) {
    const _: () = assert!(true, "");;
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!()) == (0_())) {
            expected_size = 0_();
            break;
        }
        expected_size = (expected_size).wrapping_add(
            ((unimplemented!()).wrapping_sub(1_())).wrapping_mul((*unimplemented!())),
        );
        r.postfix_inc();
    }
    unimplemented!();
    let mut strides: () = unimplemented!();
    const _: () = assert!(true, "");;
    {
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            unimplemented!();
            unimplemented!();
            r.postfix_inc();
        }
    }
}
pub unsafe fn test_construction_5(mut e: (), mut s: ()) {
    const _: () = assert!(true, "");;
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!()) == (0_())) {
            expected_size = 0_();
            break;
        }
        expected_size += (((unimplemented!()) - (1_())) * (*unimplemented!()));
        r.postfix_inc();
    }
    unimplemented!();
    let mut strides: () = unimplemented!();
    const _: () = assert!(true, "");;
    {
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            unimplemented!();
            unimplemented!();
            r.postfix_inc();
        }
    }
}
pub unsafe fn test_construction_6(mut e: (), mut s: ()) {
    const _: () = assert!(true, "");;
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!()) == (0_())) {
            expected_size = 0_();
            break;
        }
        expected_size += (((unimplemented!()) - (1_())) * (*unimplemented!()));
        r.postfix_inc();
    }
    unimplemented!();
    let mut strides: () = unimplemented!();
    const _: () = assert!(true, "");;
    {
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            unimplemented!();
            unimplemented!();
            r.postfix_inc();
        }
    }
}
pub unsafe fn test_7() -> () {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_8));
    {
        let mut s: () = unimplemented!();
        (unsafe { test_construction_0(unimplemented!(), unimplemented!()) });
    }
    {
        let mut s: () = unimplemented!();
        (unsafe { test_construction_1(unimplemented!(), unimplemented!()) });
    }
    {
        let mut s: () = unimplemented!();
        (unsafe { test_construction_1(unimplemented!(), unimplemented!()) });
    }
    {
        let mut s: () = unimplemented!();
        (unsafe { test_construction_2(unimplemented!(), unimplemented!()) });
    }
    {
        let mut s: () = unimplemented!();
        (unsafe { test_construction_3(unimplemented!(), unimplemented!()) });
    }
    {
        let mut s: () = unimplemented!();
        (unsafe { test_construction_4(unimplemented!(), unimplemented!()) });
    }
    {
        let mut s: () = unimplemented!();
        (unsafe { test_construction_5(unimplemented!(), unimplemented!()) });
    }
    {
        let mut s: () = unimplemented!();
        (unsafe { test_construction_5(unimplemented!(), unimplemented!()) });
        (unsafe { test_construction_5(unimplemented!(), unimplemented!()) });
        (unsafe { test_construction_5(unimplemented!(), unimplemented!()) });
    }
    {
        let mut s: () = unimplemented!();
        (unsafe { test_construction_6(unimplemented!(), unimplemented!()) });
        (unsafe { test_construction_6(unimplemented!(), unimplemented!()) });
        (unsafe { test_construction_6(unimplemented!(), unimplemented!()) });
        (unsafe { test_construction_6(unimplemented!(), unimplemented!()) });
        (unsafe { test_construction_6(unimplemented!(), unimplemented!()) });
        (unsafe { test_construction_6(unimplemented!(), unimplemented!()) });
        (unsafe { test_construction_6(unimplemented!(), unimplemented!()) });
        (unsafe { test_construction_6(unimplemented!(), unimplemented!()) });
        (unsafe { test_construction_6(unimplemented!(), unimplemented!()) });
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<mapping_t, std::dextents<int, 2>, std::span<int, 3>>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<mapping_t, std::dextents<int, 2>, std::span<int, 1>>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<mapping_t, std::dextents<int, 3>, std::span<int, 2>>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<mapping_t, std::dextents<int, 2>, std::span<IntType, 2>>"
        );;
    }
    {
        const _: () = assert!(true, "std::is_convertible_v<IntType, unsigned char>");;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<mapping_t, std::dextents<int, 2>, std::span<IntType, 2>>"
        );;
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
    (unsafe { test_7() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
