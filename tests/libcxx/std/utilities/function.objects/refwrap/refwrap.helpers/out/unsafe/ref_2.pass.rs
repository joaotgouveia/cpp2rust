extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CppFromUnsafe, DestructorUnsafe, Default)]
#[cpp_from(Option<unsafe fn(i32 ,) -> bool > , new, value)]
pub struct unary_counting_predicate_bool__ptr__int___int_ {
    p_: Option<unsafe fn(i32) -> bool>,
    count_: usize,
}
impl unary_counting_predicate_bool__ptr__int___int_ {
    pub unsafe fn new(mut p: Option<unsafe fn(i32) -> bool>) -> Self {
        let mut this = Self {
            p_: p,
            count_: 0_usize,
        };
        this
    }
    pub unsafe fn destructor(&mut self) {}
    pub unsafe fn operator_call(&self, a: *const i32) -> bool {
        (*(&raw const self.count_).cast_mut()).prefix_inc();
        return (unsafe { (self.p_).unwrap()((*a)) });
    }
    pub unsafe fn count(&self) -> usize {
        return self.count_;
    }
}
pub unsafe fn is5_0(mut i: i32) -> bool {
    return ((i) == (5));
}
pub unsafe fn call_pred_1(mut pred: unary_counting_predicate_bool__ptr__int___int_) -> bool {
    return (unsafe {
        let mut _a: i32 = 5;
        unary_counting_predicate_bool__ptr__int___int_::operator_call(&pred, &mut _a)
    });
}
pub unsafe fn call_pred_2(mut pred: *mut unary_counting_predicate_bool__ptr__int___int_) -> bool {
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct adl_A {}
pub unsafe fn ref_3(mut _a0: adl_A) {}
pub unsafe fn test_4() -> bool {
    {
        let mut i: i32 = 0;
        let mut r1: *mut i32 = unimplemented!();
        let mut r2: *mut i32 = unimplemented!();
        assert!(((r2) == (&mut i as *mut i32)));
    }
    {
        let mut a: adl_A = <adl_A>::default();
        let mut a1: *mut adl_A = unimplemented!();
        let mut a2: *mut adl_A = unimplemented!();
        assert!(((a2) == (&mut a as *mut adl_A)));
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
    (unsafe { test_4() });
    const _: () = assert!(true, "test()");;
    {
        let mut cp: unary_counting_predicate_bool__ptr__int___int_ =
            unary_counting_predicate_bool__ptr__int___int_::new({ Some(is5_0) });
        let _dtor_cp = ScopedDestructorUnsafe::new(
            &raw mut cp,
            unary_counting_predicate_bool__ptr__int___int_::destructor,
        );
        assert!(
            !(unsafe {
                let mut _a: i32 = 6;
                unary_counting_predicate_bool__ptr__int___int_::operator_call(&cp, &mut _a)
            })
        );
        assert!(
            ((unsafe { unary_counting_predicate_bool__ptr__int___int_::count(&cp,) }) == (1_usize))
        );
        assert!((unsafe { call_pred_1(cp.clone(),) }));
        assert!(
            ((unsafe { unary_counting_predicate_bool__ptr__int___int_::count(&cp,) }) == (1_usize))
        );
        assert!((unsafe { call_pred_2(unimplemented!(),) }));
        assert!(
            ((unsafe { unary_counting_predicate_bool__ptr__int___int_::count(&cp,) }) == (2_usize))
        );
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
