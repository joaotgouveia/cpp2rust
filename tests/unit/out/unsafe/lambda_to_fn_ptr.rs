extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn apply_0(mut x: i32, mut fn_: Option<unsafe fn(i32) -> i32>) -> i32 {
    return (unsafe { (fn_).unwrap()(x) });
}
pub unsafe fn call_nttp_1(mut x: i32) -> i32 {
    return (unsafe { (Some(lambda_2::__invoke))(x) });
}
pub unsafe fn call_nttp_1(mut x: i32) -> i32 {
    return (unsafe { (Some(lambda_3::__invoke))(x) });
}
pub unsafe fn call_nttp_4(mut x: i32) -> i32 {
    return (unsafe { lambda_5::operator_call(x) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut fresh: Option<unsafe fn(i32) -> i32> =
        (unsafe { (<lambda_6>::default()).to_free_function() });
    assert!(((unsafe { (fresh).unwrap()(5,) }) == (-5_i32)));
    let mut twice: lambda_7 = (<lambda_7>::default());
    let mut named: Option<unsafe fn(i32) -> i32> = (unsafe { twice.to_free_function() });
    assert!(((unsafe { (named).unwrap()(5,) }) == (10)));
    assert!(((unsafe { apply_0(5, (unsafe { twice.to_free_function() }),) }) == (10)));
    named = fresh;
    assert!(((unsafe { (named).unwrap()(3,) }) == (-3_i32)));
    let mut p: Option<unsafe fn(i32) -> i32> =
        (unsafe { (<lambda_8>::default()).to_free_function() });
    assert!(((unsafe { (p).unwrap()(2,) }) == (6)));
    assert!(((unsafe { call_nttp_1(5,) }) == (4)));
    assert!(((unsafe { call_nttp_1(5,) }) == (50)));
    assert!(((unsafe { call_nttp_4(5,) }) == (105)));
    let mut sum: lambda_9 = (<lambda_9>::default());
    assert!(((unsafe { lambda_9::operator_call(2, &[(5).into(), (5).into(),]) }) == (10)));
    assert!(
        ((unsafe { lambda_9::operator_call(3, &[(1).into(), (2).into(), (3).into(),]) }) == (6))
    );
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_6 {}
impl lambda_6 {
    pub unsafe fn operator_call(mut x: i32) -> i32 {
        return -x;
    }
}
impl Callable1<i32, i32> for lambda_6 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_6::operator_call(a1) }
    }
}
impl lambda_6 {
    pub fn to_free_function(&self) -> Option<unsafe fn(i32) -> i32> {
        Some(lambda_6::operator_call)
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_7 {}
impl lambda_7 {
    pub unsafe fn operator_call(mut x: i32) -> i32 {
        return ((x) * (2));
    }
}
impl Callable1<i32, i32> for lambda_7 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_7::operator_call(a1) }
    }
}
impl lambda_7 {
    pub fn to_free_function(&self) -> Option<unsafe fn(i32) -> i32> {
        Some(lambda_7::operator_call)
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_8 {}
impl lambda_8 {
    pub unsafe fn operator_call(mut x: i32) -> i32 {
        return ((x) * (3));
    }
}
impl Callable1<i32, i32> for lambda_8 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_8::operator_call(a1) }
    }
}
impl lambda_8 {
    pub fn to_free_function(&self) -> Option<unsafe fn(i32) -> i32> {
        Some(lambda_8::operator_call)
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_9 {}
impl lambda_9 {
    pub unsafe fn operator_call(mut n: i32, __args: &[VaArg]) -> i32 {
        let mut ap: VaList = VaList::default();
        ap = VaList::new(__args);
        let mut s: i32 = 0;
        let mut i: i32 = 0;
        'loop_: while ((i) < (n)) {
            s += ap.arg::<i32>();
            i.postfix_inc();
        }
        return s;
    }
}
impl Callable1<i32, i32> for lambda_9 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_9::operator_call(a1) }
    }
}
impl lambda_9 {
    pub fn to_free_function(&self) -> Option<unsafe fn(i32) -> i32> {
        Some(lambda_9::operator_call)
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
