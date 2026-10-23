extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct unary_counting_predicate_bool__ptr__int___int_ {
    p_: Option<unsafe fn(()) -> ()>,
    count_: (),
}
impl unary_counting_predicate_bool__ptr__int___int_ {
    pub unsafe fn new(mut p: Option<unsafe fn(()) -> ()>) -> Self {
        let mut this = Self {
            p_: p,
            count_: 0_(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {}
    pub unsafe fn operator_call(&self, a: *const ()) -> () {
        (*(&raw const self.count_).cast_mut()).prefix_inc();
        return (unsafe { (self.p_).unwrap()((*a)) });
    }
    pub unsafe fn count(&self) -> () {
        return self.count_;
    }
}
pub unsafe fn is5_0(mut i: ()) -> () {
    return ((i) == (5_()));
}
pub unsafe fn call_pred_1(mut pred: unary_counting_predicate_bool__ptr__int___int_) -> () {
    return (unsafe {
        let mut _a: () = 5_();
        unary_counting_predicate_bool__ptr__int___int_::operator_call(&pred, &mut _a)
    });
}
pub unsafe fn call_pred_2(mut pred: ()) -> () {
    return ((unsafe {
        let mut ___args: () = 5_();
        pred(&mut ___args)
    }) as ());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct adl_A {}
pub unsafe fn ref_3(mut _a0: adl_A) {}
pub unsafe fn test_4() -> () {
    {
        let mut i: () = 0_();
        let mut r1: () = unimplemented!();
        let mut r2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut a: adl_A = <adl_A>::default();
        let mut a1: () = unimplemented!();
        let mut a2: () = unimplemented!();
        unimplemented!();
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
    (unsafe { test_4() });
    const _: () = assert!(true, "test()");;
    {
        let mut cp: unary_counting_predicate_bool__ptr__int___int_ =
            unary_counting_predicate_bool__ptr__int___int_::new({ Some(is5_0) });
        let _dtor_cp = ScopedDestructorUnsafe::new(
            &raw mut cp,
            unary_counting_predicate_bool__ptr__int___int_::destructor,
        );
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
