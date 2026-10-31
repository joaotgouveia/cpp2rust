extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Rep {
    data_: i32,
}
impl Rep {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { data_: -1_i32 };
        this
    }
    pub unsafe fn new_2(mut i: i32) -> Self {
        let mut this = Self { data_: i };
        this
    }
    pub unsafe fn operator_eq_3(&self, mut i: i32) -> bool {
        return ((self.data_) == (i));
    }
    pub unsafe fn operator_eq_4(&self, r: *const Rep) -> bool {
        return ((self.data_) == ((*r).data_));
    }
    pub unsafe fn operator_mul_assign(&mut self, mut x: Rep) -> *mut Rep {
        self.data_ *= x.data_;
        return &mut (*(self as *mut Rep));
    }
    pub unsafe fn operator_div_assign(&mut self, mut x: Rep) -> *mut Rep {
        self.data_ /= x.data_;
        return &mut (*(self as *mut Rep));
    }
}
impl std::cmp::PartialEq for Rep {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Rep::operator_eq_4(self, other as *const Rep) }
    }
}
impl std::cmp::Eq for Rep {}
impl Default for Rep {
    fn default() -> Self {
        unsafe { Rep::new_1() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NotARep {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(i64, to_i64_2)]
pub struct RepConstConvertibleLWG3050 {}
impl RepConstConvertibleLWG3050 {
    pub unsafe fn to_i64_2(&self) -> i64 {
        return 2_i64;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(i32, to_i32_2)]
pub struct NotValueConvertible {}
impl NotValueConvertible {
    pub unsafe fn to_i32_2(&self) -> i32 {
        return 1;
    }
}
pub unsafe fn check_0(mut r: i32) {
    let mut d: () = unimplemented!();
    assert!(((unimplemented!()) == (r)));
}
pub unsafe fn check_1(mut r: i32) {
    let mut d: () = unimplemented!();
    assert!(((unimplemented!()) == (r)));
}
pub unsafe fn check_2(mut r: Rep) {
    let mut d: () = unimplemented!();
    assert!((unsafe { Rep::operator_eq_4(&unimplemented!(), &r,) }));
}
pub unsafe fn check_3(mut r: f64) {
    let mut d: () = unimplemented!();
    assert!(((unimplemented!()) == (r)));
}
pub unsafe fn check_4(mut r: NotValueConvertible) {
    let mut d: () = unimplemented!();
    assert!(((unimplemented!()) == (unsafe { NotValueConvertible::to_i32_2(&r,) })));
}
pub unsafe fn test_5() -> bool {
    (unsafe { check_0(5) });
    (unsafe { check_1(5) });
    (unsafe { check_2(Rep::new_2({ 3 })) });
    (unsafe { check_3(5.5_f64) });
    (unsafe { check_4(<NotValueConvertible>::default()) });
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
    (unsafe { test_5() });
    const _: () = assert!(true, "test()");;
    {
        let d: () = unimplemented!();
        const _: () = assert!(true, "d.count() == 5");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
