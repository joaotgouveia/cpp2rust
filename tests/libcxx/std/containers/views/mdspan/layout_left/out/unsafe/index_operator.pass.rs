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
pub unsafe fn check_operator_constraints_0(mut m: (), mut idxs: ()) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_1(mut m: (), mut idxs: IntType) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_2(mut _a0: (), mut _a1: (), mut _a2: ()) -> () {
    return false;
}
pub unsafe fn check_operator_constraints_3(mut _a0: (), mut _a1: IntType) -> () {
    return false;
}
pub unsafe fn check_operator_constraints_4(mut _a0: (), mut _a1: IntType) -> () {
    return false;
}
pub unsafe fn iterate_left_5(mut m: (), count: *mut ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        (*count).postfix_inc();
    }
}
pub unsafe fn iterate_left_6(mut m: (), count: *mut ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe {
                let _m: () = unimplemented!();
                let _count: *mut () = count;
                let _args: () = i;
                iterate_left_7(_m, _count, _args)
            });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_left_7(mut m: (), count: *mut (), mut args: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (1));
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        (*count).postfix_inc();
    }
}
pub unsafe fn iterate_left_8(mut m: (), count: *mut ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe {
                let _m: () = unimplemented!();
                let _count: *mut () = count;
                let _args: () = i;
                iterate_left_9(_m, _count, _args)
            });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_left_9(mut m: (), count: *mut (), mut args: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (1));
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        (*count).postfix_inc();
    }
}
pub unsafe fn iterate_left_10(mut m: (), count: *mut ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe {
                let _m: () = unimplemented!();
                let _count: *mut () = count;
                let _args: () = i;
                iterate_left_11(_m, _count, _args)
            });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_left_11(mut m: (), count: *mut (), mut args: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (1));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe {
                let _m: () = unimplemented!();
                let _count: *mut () = count;
                let _args_2: () = i;
                let _args_3: () = args;
                iterate_left_12(_m, _count, _args_2, _args_3)
            });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_left_12(mut m: (), count: *mut (), mut args_2: (), mut args_3: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (2));
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        (*count).postfix_inc();
    }
}
pub unsafe fn iterate_left_13(mut m: (), count: *mut ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe {
                let _m: () = unimplemented!();
                let _count: *mut () = count;
                let _args: () = i;
                iterate_left_14(_m, _count, _args)
            });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_left_14(mut m: (), count: *mut (), mut args: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (1));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe {
                let _m: () = unimplemented!();
                let _count: *mut () = count;
                let _args_2: () = i;
                let _args_3: () = args;
                iterate_left_15(_m, _count, _args_2, _args_3)
            });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_left_15(mut m: (), count: *mut (), mut args_2: (), mut args_3: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (2));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe {
                let _m: () = unimplemented!();
                let _count: *mut () = count;
                let _args_2: () = i;
                let _args_3: () = args_2;
                let _args_4: () = args_3;
                iterate_left_16(_m, _count, _args_2, _args_3, _args_4)
            });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_left_16(
    mut m: (),
    count: *mut (),
    mut args_2: (),
    mut args_3: (),
    mut args_4: (),
) {
    let r: () = (((unimplemented!()) - (1_())) - (3));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe {
                let _m: () = unimplemented!();
                let _count: *mut () = count;
                let _args_2: () = i;
                let _args_3: () = args_2;
                let _args_4: () = args_3;
                let _args_5: () = args_4;
                iterate_left_17(_m, _count, _args_2, _args_3, _args_4, _args_5)
            });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_left_17(
    mut m: (),
    count: *mut (),
    mut args_2: (),
    mut args_3: (),
    mut args_4: (),
    mut args_5: (),
) {
    let r: () = (((unimplemented!()) - (1_())) - (4));
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        (*count).postfix_inc();
    }
}
pub unsafe fn iterate_left_18(mut m: (), count: *mut ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe {
                let _m: () = unimplemented!();
                let _count: *mut () = count;
                let _args: () = i;
                iterate_left_19(_m, _count, _args)
            });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_left_19(mut m: (), count: *mut (), mut args: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (1));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe {
                let _m: () = unimplemented!();
                let _count: *mut () = count;
                let _args_2: () = i;
                let _args_3: () = args;
                iterate_left_20(_m, _count, _args_2, _args_3)
            });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_left_20(mut m: (), count: *mut (), mut args_2: (), mut args_3: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (2));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe {
                let _m: () = unimplemented!();
                let _count: *mut () = count;
                let _args_2: () = i;
                let _args_3: () = args_2;
                let _args_4: () = args_3;
                iterate_left_21(_m, _count, _args_2, _args_3, _args_4)
            });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_left_21(
    mut m: (),
    count: *mut (),
    mut args_2: (),
    mut args_3: (),
    mut args_4: (),
) {
    let r: () = (((unimplemented!()) - (1_())) - (3));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe {
                let _m: () = unimplemented!();
                let _count: *mut () = count;
                let _args_2: () = i;
                let _args_3: () = args_2;
                let _args_4: () = args_3;
                let _args_5: () = args_4;
                iterate_left_22(_m, _count, _args_2, _args_3, _args_4, _args_5)
            });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_left_22(
    mut m: (),
    count: *mut (),
    mut args_2: (),
    mut args_3: (),
    mut args_4: (),
    mut args_5: (),
) {
    let r: () = (((unimplemented!()) - (1_())) - (4));
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        (*count).postfix_inc();
    }
}
pub unsafe fn iterate_left_23(mut m: (), count: *mut ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe {
                let _m: () = unimplemented!();
                let _count: *mut () = count;
                let _args: () = i;
                iterate_left_24(_m, _count, _args)
            });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_left_24(mut m: (), count: *mut (), mut args: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (1));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe {
                let _m: () = unimplemented!();
                let _count: *mut () = count;
                let _args_2: () = i;
                let _args_3: () = args;
                iterate_left_25(_m, _count, _args_2, _args_3)
            });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_left_25(mut m: (), count: *mut (), mut args_2: (), mut args_3: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (2));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe {
                let _m: () = unimplemented!();
                let _count: *mut () = count;
                let _args_2: () = i;
                let _args_3: () = args_2;
                let _args_4: () = args_3;
                iterate_left_26(_m, _count, _args_2, _args_3, _args_4)
            });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_left_26(
    mut m: (),
    count: *mut (),
    mut args_2: (),
    mut args_3: (),
    mut args_4: (),
) {
    let r: () = (((unimplemented!()) - (1_())) - (3));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe {
                let _m: () = unimplemented!();
                let _count: *mut () = count;
                let _args_2: () = i;
                let _args_3: () = args_2;
                let _args_4: () = args_3;
                let _args_5: () = args_4;
                iterate_left_27(_m, _count, _args_2, _args_3, _args_4, _args_5)
            });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_left_27(
    mut m: (),
    count: *mut (),
    mut args_2: (),
    mut args_3: (),
    mut args_4: (),
    mut args_5: (),
) {
    let r: () = (((unimplemented!()) - (1_())) - (4));
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        (*count).postfix_inc();
    }
}
pub unsafe fn test_iteration_28() {
    let mut m: () = unimplemented!();
    let mut count: () = 0_();
    (unsafe { iterate_left_5(unimplemented!(), &mut count) });
}
pub unsafe fn test_iteration_29(mut args: ()) {
    let mut m: () = unimplemented!();
    let mut count: () = 0_();
    (unsafe { iterate_left_6(unimplemented!(), &mut count) });
}
pub unsafe fn test_iteration_30() {
    let mut m: () = unimplemented!();
    let mut count: () = 0_();
    (unsafe { iterate_left_8(unimplemented!(), &mut count) });
}
pub unsafe fn test_iteration_31() {
    let mut m: () = unimplemented!();
    let mut count: () = 0_();
    (unsafe { iterate_left_10(unimplemented!(), &mut count) });
}
pub unsafe fn test_iteration_32(mut args_0: (), mut args_1: (), mut args_2: (), mut args_3: ()) {
    let mut m: () = unimplemented!();
    let mut count: () = 0_();
    (unsafe { iterate_left_13(unimplemented!(), &mut count) });
}
pub unsafe fn test_iteration_33(mut args_0: (), mut args_1: (), mut args_2: ()) {
    let mut m: () = unimplemented!();
    let mut count: () = 0_();
    (unsafe { iterate_left_18(unimplemented!(), &mut count) });
}
pub unsafe fn test_iteration_34(mut args_0: (), mut args_1: ()) {
    let mut m: () = unimplemented!();
    let mut count: () = 0_();
    (unsafe { iterate_left_23(unimplemented!(), &mut count) });
}
pub unsafe fn test_35() -> () {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_36));
    (unsafe { test_iteration_28() });
    (unsafe { test_iteration_29(1_()) });
    (unsafe { test_iteration_29(7_()) });
    (unsafe { test_iteration_30() });
    (unsafe { test_iteration_31() });
    (unsafe { test_iteration_32(1_(), 1_(), 1_(), 1_()) });
    const _: () = assert!(
        true,
        "check_operator_constraints(std::layout_left::mapping<std::extents<int, D>>(std::extents<int, D>(1)), 0)"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(std::layout_left::mapping<std::extents<int, D>>(std::extents<int, D>(1)), 0, 0)"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(std::layout_left::mapping<std::extents<int, D>>(std::extents<int, D>(1)), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::layout_left::mapping<std::extents<unsigned, D>>(std::extents<unsigned, D>(1)), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::layout_left::mapping<std::extents<unsigned char, D>>(std::extents<unsigned char, D>(1)), IntType(0))"
    );;
    return true;
}
pub unsafe fn test_large_37() -> () {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_36));
    (unsafe { test_iteration_33(7_(), 9_(), 10_()) });
    (unsafe { test_iteration_34(7_(), 10_()) });
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
    (unsafe { test_35() });
    const _: () = assert!(true, "test()");;
    (unsafe { test_large_37() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
