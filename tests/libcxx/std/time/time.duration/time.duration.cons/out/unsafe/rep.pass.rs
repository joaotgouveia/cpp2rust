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
    data_: (),
}
impl Rep {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { data_: -1_() };
        this
    }
    pub unsafe fn new_2(mut i: ()) -> Self {
        let mut this = Self { data_: i };
        this
    }
    pub unsafe fn operator_eq_3(&self, mut i: ()) -> () {
        return ((self.data_) == (i));
    }
    pub unsafe fn operator_eq_4(&self, r: *const Rep) -> () {
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RepConstConvertibleLWG3050 {}
impl RepConstConvertibleLWG3050 {
    pub unsafe fn to__2(&self) -> () {
        return 2_();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NotValueConvertible {}
impl NotValueConvertible {
    pub unsafe fn to__2(&self) -> () {
        return 1_();
    }
}
pub unsafe fn check_0(mut r: ()) {
    let mut d: () = std_chrono_duration_int__std_ratio_1__1__::new_1({ &r });
    unimplemented!();
}
pub unsafe fn check_1(mut r: ()) {
    let mut d: () = std_chrono_duration_int__std_ratio_3__2__::new_1({ &r });
    unimplemented!();
}
pub unsafe fn check_2(mut r: Rep) {
    let mut d: () = std_chrono_duration_Rep__std_ratio_3__2__::new_1({ &r });
    unimplemented!();
}
pub unsafe fn check_3(mut r: ()) {
    let mut d: () = std_chrono_duration_double__std_ratio_2__3__::new_1({ &r });
    unimplemented!();
}
pub unsafe fn check_4(mut r: NotValueConvertible) {
    let mut d: () = std_chrono_duration_int__std_ratio_1__1__::new_2({ &r });
    unimplemented!();
}
pub unsafe fn test_5() -> () {
    (unsafe { check_0(5_()) });
    (unsafe { check_1(5_()) });
    (unsafe { check_2(Rep::new_2({ 3_() })) });
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_5() });
    const _: () = assert!(true, "test()");;
    {
        let d: () = {
            let mut __tmp_0: () = 5_();
            std_chrono_duration_int__std_ratio_1__1__::new_1({ &mut __tmp_0 })
        };
        const _: () = assert!(true, "d.count() == 5");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
