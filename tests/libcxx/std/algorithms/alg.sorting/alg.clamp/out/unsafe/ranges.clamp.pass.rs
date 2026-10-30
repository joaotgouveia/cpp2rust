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
pub struct NoComp {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CreateNoComp {}
impl CreateNoComp {
    pub unsafe fn operator_call(&self, mut _a0: ()) -> NoComp {
        return <NoComp>::default();
    }
}
const _: () = assert!(true, "HasClamp<int, std::ranges::less, std::identity>");
const _: () = assert!(!(false), "!HasClamp<NoComp>");
const _: () = assert!(!(false), "!HasClamp<int, NoComp>");
const _: () = assert!(!(false), "!HasClamp<int, std::ranges::less, CreateNoComp>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EnsureValueCategoryComp {}
impl EnsureValueCategoryComp {
    pub unsafe fn operator_call_1(&self, x: *const (), y: *const ()) -> () {
        return ((*x) < (*y));
    }
    pub unsafe fn operator_call_2(&self, x: *const (), y: *mut ()) -> () {
        return ((*x) < (*y));
    }
    pub unsafe fn operator_call_3(&self, x: *mut (), y: *const ()) -> () {
        return ((*x) < (*y));
    }
    pub unsafe fn operator_call_4(&self, x: *mut (), y: *mut ()) -> () {
        return ((*x) < (*y));
    }
}
pub unsafe fn test_0() -> () {
    {
        let mut val: () = 2_();
        let mut low: () = 1_();
        let mut high: () = 3_();
        let ret: *const () = &(unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut clamp_1))(&val, &low, &high, None, None)
        });
        unimplemented!();
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let mut val: () = 10_();
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let mut val: S_2 = S_2 { i: 10_() };
        let mut low: S_2 = S_2 { i: 20_() };
        let mut high: S_2 = S_2 { i: 30_() };
        let mut proj: FnPtr<fn(*const S_2) -> *const ()> =
            FnPtr::<fn(*const S_2) -> *const ()>::new(|s: *const S_2| -> *const () {
                unsafe {
                    return &(*s).i;
                }
            });
        unimplemented!();
    }
    {
        const _: () = assert!(
            true,
            "std::indirect_strict_weak_order<EnsureValueCategoryComp, std::projected<const int*, MoveProj>>"
        );;
        unimplemented!();
    }
    {
        let mut counter: () = 0_();
        let mut projection_function: FnPtr<fn(()) -> ()> = lambda_unsafe!(
            {
                let counter: *mut () = &mut counter;
            },
            |value: ()| -> () {
                (*counter).postfix_inc();
                return value;
            }
        );
        unimplemented!();
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_2 {
    pub i: (),
}
impl S_2 {
    pub unsafe fn operator_eq(&self, other: *const S_2) -> () {
        return ((self.i) == ((*other).i));
    }
}
impl std::cmp::PartialEq for S_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { S_2::operator_eq(self, other as *const S_2) }
    }
}
impl std::cmp::Eq for S_2 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MoveProj_3 {}
impl MoveProj_3 {
    pub unsafe fn operator_call(&self, x: *const ()) -> *const () {
        return x;
    }
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
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
