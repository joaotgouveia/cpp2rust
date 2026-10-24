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
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_int__int_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_int__unsigned_int_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_signed_char__signed_char_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_long_long__long_long_ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
pub unsafe fn test_runtime_observers_5(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_6(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_7(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_8(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_9(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_10(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_11(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_12(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_13(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_14(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_15(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_16(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_17(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_18(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_19(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_20(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_21(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_22(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_23(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_24(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_25(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_26(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_27(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_28(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_29(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_30(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_31(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_32(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_33(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_34(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_35(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_36(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_37(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_38(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_39(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_40(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_41(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_42(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_43(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_44(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_45(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_46(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_47(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_48(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_49(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_50(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_51(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_52(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_53(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_54(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_55(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_56(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_57(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_58(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_59(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_60(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_61(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_62(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_63(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_64(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_65(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_66(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_67(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_68(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_69(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_70(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_71(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_72(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_73(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_74(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_75(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_76(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_77(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_78(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_79(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_80(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_81(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_82(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_83(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_84(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_85(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_86(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_87(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_88(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_89(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_90(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_91(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_92(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_93(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_94(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_95(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_96(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_97(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_98(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_99(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_100(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_101(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_102(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_103(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_104(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_105(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_106(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_107(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_108(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_109(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_110(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_111(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_112(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_113(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_114(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_115(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_116(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_117(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_118(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_119(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_120(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_121(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_122(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_123(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_124(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_125(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_126(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_127(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_128(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_129(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_130(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_131(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_132(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_133(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_134(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_135(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_136(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_137(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_138(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_139(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_140(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_141(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_142(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_143(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_144(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_145(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_146(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_147(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_148(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_149(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_150(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_151(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_152(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_153(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_154(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_155(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_156(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_157(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_158(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_159(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_160(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_161(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_162(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_163(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_164(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_165(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_runtime_observers_166(mut ext: (), mut expected: ()) {
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub unsafe fn test_construction_167(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__struct_std_array_int__0__struct_std_array_int__0_ ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__struct_std_array_int__0__struct_std_array_int__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_169(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__struct_std_array_int__1__struct_std_array_int__1__0 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__struct_std_array_int__1__struct_std_array_int__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_170(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__struct_std_array_int__1__struct_std_array_int__1__0 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__struct_std_array_int__1__struct_std_array_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_171(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__7__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__7__struct_std_array_int__2__struct_std_array_int__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_172(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_173(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_174(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_175(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__7__9__struct_std_array_int__3__struct_std_array_int__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_176(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_177(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_178(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_179(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_180(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_181(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_182(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_183(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_184(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__4__0_1_2_3 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_185(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_186(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__struct_std_array_unsigned_long__0__struct_std_array_unsigned_long__0_ ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__struct_std_array_unsigned_long__0__struct_std_array_unsigned_long__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_187(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_188(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_189(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_190(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_191(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_192(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_193(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_194(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_195(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_196(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_197(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_198(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_199(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_200(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_201(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_202(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__4__0_1_2_3 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_203(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_204(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__struct_std_array_int__0__struct_std_array_int__0_ ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__struct_std_array_int__0__struct_std_array_int__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_205(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__3__struct_std_array_int__1__struct_std_array_int__1__0 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__3__struct_std_array_int__1__struct_std_array_int__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_206(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__struct_std_array_int__1__struct_std_array_int__1__0 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__struct_std_array_int__1__struct_std_array_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_207(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__3__7__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__3__7__struct_std_array_int__2__struct_std_array_int__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_208(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_209(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_210(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_211(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__3__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__3__7__9__struct_std_array_int__3__struct_std_array_int__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_212(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_213(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_214(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_215(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_216(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_217(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_218(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_219(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_220(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__4__0_1_2_3 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_221(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_222(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__struct_std_array_unsigned_long__0__struct_std_array_unsigned_long__0_ ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__struct_std_array_unsigned_long__0__struct_std_array_unsigned_long__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_223(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_224(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_225(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_226(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_227(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_228(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_229(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_230(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_231(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_232(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_233(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_234(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_235(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_236(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_237(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_238(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__4__0_1_2_3 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_239(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_240(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__struct_std_array_unsigned_int__0__struct_std_array_unsigned_int__0_ ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__struct_std_array_unsigned_int__0__struct_std_array_unsigned_int__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_241(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__3__struct_std_array_unsigned_int__1__struct_std_array_unsigned_int__1__0 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__3__struct_std_array_unsigned_int__1__struct_std_array_unsigned_int__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_242(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__struct_std_array_unsigned_int__1__struct_std_array_unsigned_int__1__0 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__struct_std_array_unsigned_int__1__struct_std_array_unsigned_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_243(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__3__7__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__3__7__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_244(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__3__18446744073709551615__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__3__18446744073709551615__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_245(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__7__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__7__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_246(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_247(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__3__7__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__3__7__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_248(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__3__7__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__3__7__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_249(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_250(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_251(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_252(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__3__18446744073709551615__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__3__18446744073709551615__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_253(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_254(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__7__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__7__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_255(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_int__9__struct_std_array_unsigned_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_int__9__struct_std_array_unsigned_int__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_256(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_int__9__struct_std_array_unsigned_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_int__9__struct_std_array_unsigned_int__4__0_1_2_3 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_257(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__9__struct_std_array_unsigned_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__9__struct_std_array_unsigned_int__9__0_1_2_3_4_5_6_7_8 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_258(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__struct_std_array_int__0__struct_std_array_int__0_ ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__struct_std_array_int__0__struct_std_array_int__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_259(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__struct_std_array_int__1__struct_std_array_int__1__0 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__struct_std_array_int__1__struct_std_array_int__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_260(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__struct_std_array_int__1__struct_std_array_int__1__0 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__struct_std_array_int__1__struct_std_array_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_261(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__struct_std_array_int__2__struct_std_array_int__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_262(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_263(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_264(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_265(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__9__struct_std_array_int__3__struct_std_array_int__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_266(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_267(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_268(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_269(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_270(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_271(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_272(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_273(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_274(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__4__0_1_2_3 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_275(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_276(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__struct_std_array_unsigned_long__0__struct_std_array_unsigned_long__0_ ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__struct_std_array_unsigned_long__0__struct_std_array_unsigned_long__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_277(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_278(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_279(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_280(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_281(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_282(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_283(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_284(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_285(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_286(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_287(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_288(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_289(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_290(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_291(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_292(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__4__0_1_2_3 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_293(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_294(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__struct_std_array_struct_IntType__0__struct_std_array_struct_IntType__0_ ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__struct_std_array_struct_IntType__0__struct_std_array_struct_IntType__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_295(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__1__0 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_296(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__1__0 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_297(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_298(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_299(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_300(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_301(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_302(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_303(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_304(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_305(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_306(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_307(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_308(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_309(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_310(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__4__0_1_2_3 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_311(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_312(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__struct_std_array_struct_IntType__0__struct_std_array_struct_IntType__0_ ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__struct_std_array_struct_IntType__0__struct_std_array_struct_IntType__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_313(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__1__0 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_314(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__1__0 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_315(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_316(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_317(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_318(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_319(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_320(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_321(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_322(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_323(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_324(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_325(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_326(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_327(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__1__2__3__4__5__6__7__8__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__1__2__3__4__5__6__7__8__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__0_ ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_328(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__4__0_1_2_3 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_construction_329(mut all_ext: ()) {
    (unsafe {
        let _all_ext: () = all_ext.clone();
        let _ext: () = all_ext.clone();
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _ext , unimplemented!() , )
    });
    let mut dyn_ext: () = unimplemented!();
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        if ((unimplemented!())
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168)))
        {
            (*unimplemented!()) = (*unimplemented!());
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    (unsafe {
        IntegralCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8 ( all_ext .clone() , dyn_ext .clone() , unimplemented!() , )
    });
}
pub unsafe fn test_330() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168));
    (unsafe { test_construction_167(unimplemented!()) });
    (unsafe { test_construction_169(unimplemented!()) });
    (unsafe { test_construction_170(unimplemented!()) });
    (unsafe { test_construction_171(unimplemented!()) });
    (unsafe { test_construction_172(unimplemented!()) });
    (unsafe { test_construction_173(unimplemented!()) });
    (unsafe { test_construction_174(unimplemented!()) });
    (unsafe { test_construction_175(unimplemented!()) });
    (unsafe { test_construction_176(unimplemented!()) });
    (unsafe { test_construction_177(unimplemented!()) });
    (unsafe { test_construction_178(unimplemented!()) });
    (unsafe { test_construction_179(unimplemented!()) });
    (unsafe { test_construction_180(unimplemented!()) });
    (unsafe { test_construction_181(unimplemented!()) });
    (unsafe { test_construction_182(unimplemented!()) });
    (unsafe { test_construction_183(unimplemented!()) });
    (unsafe { test_construction_184(unimplemented!()) });
    (unsafe { test_construction_185(unimplemented!()) });
}
pub unsafe fn test_331() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168));
    (unsafe { test_construction_186(unimplemented!()) });
    (unsafe { test_construction_187(unimplemented!()) });
    (unsafe { test_construction_188(unimplemented!()) });
    (unsafe { test_construction_189(unimplemented!()) });
    (unsafe { test_construction_190(unimplemented!()) });
    (unsafe { test_construction_191(unimplemented!()) });
    (unsafe { test_construction_192(unimplemented!()) });
    (unsafe { test_construction_193(unimplemented!()) });
    (unsafe { test_construction_194(unimplemented!()) });
    (unsafe { test_construction_195(unimplemented!()) });
    (unsafe { test_construction_196(unimplemented!()) });
    (unsafe { test_construction_197(unimplemented!()) });
    (unsafe { test_construction_198(unimplemented!()) });
    (unsafe { test_construction_199(unimplemented!()) });
    (unsafe { test_construction_200(unimplemented!()) });
    (unsafe { test_construction_201(unimplemented!()) });
    (unsafe { test_construction_202(unimplemented!()) });
    (unsafe { test_construction_203(unimplemented!()) });
}
pub unsafe fn test_332() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168));
    (unsafe { test_construction_204(unimplemented!()) });
    (unsafe { test_construction_205(unimplemented!()) });
    (unsafe { test_construction_206(unimplemented!()) });
    (unsafe { test_construction_207(unimplemented!()) });
    (unsafe { test_construction_208(unimplemented!()) });
    (unsafe { test_construction_209(unimplemented!()) });
    (unsafe { test_construction_210(unimplemented!()) });
    (unsafe { test_construction_211(unimplemented!()) });
    (unsafe { test_construction_212(unimplemented!()) });
    (unsafe { test_construction_213(unimplemented!()) });
    (unsafe { test_construction_214(unimplemented!()) });
    (unsafe { test_construction_215(unimplemented!()) });
    (unsafe { test_construction_216(unimplemented!()) });
    (unsafe { test_construction_217(unimplemented!()) });
    (unsafe { test_construction_218(unimplemented!()) });
    (unsafe { test_construction_219(unimplemented!()) });
    (unsafe { test_construction_220(unimplemented!()) });
    (unsafe { test_construction_221(unimplemented!()) });
}
pub unsafe fn test_333() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168));
    (unsafe { test_construction_222(unimplemented!()) });
    (unsafe { test_construction_223(unimplemented!()) });
    (unsafe { test_construction_224(unimplemented!()) });
    (unsafe { test_construction_225(unimplemented!()) });
    (unsafe { test_construction_226(unimplemented!()) });
    (unsafe { test_construction_227(unimplemented!()) });
    (unsafe { test_construction_228(unimplemented!()) });
    (unsafe { test_construction_229(unimplemented!()) });
    (unsafe { test_construction_230(unimplemented!()) });
    (unsafe { test_construction_231(unimplemented!()) });
    (unsafe { test_construction_232(unimplemented!()) });
    (unsafe { test_construction_233(unimplemented!()) });
    (unsafe { test_construction_234(unimplemented!()) });
    (unsafe { test_construction_235(unimplemented!()) });
    (unsafe { test_construction_236(unimplemented!()) });
    (unsafe { test_construction_237(unimplemented!()) });
    (unsafe { test_construction_238(unimplemented!()) });
    (unsafe { test_construction_239(unimplemented!()) });
}
pub unsafe fn test_334() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168));
    (unsafe { test_construction_240(unimplemented!()) });
    (unsafe { test_construction_241(unimplemented!()) });
    (unsafe { test_construction_242(unimplemented!()) });
    (unsafe { test_construction_243(unimplemented!()) });
    (unsafe { test_construction_244(unimplemented!()) });
    (unsafe { test_construction_245(unimplemented!()) });
    (unsafe { test_construction_246(unimplemented!()) });
    (unsafe { test_construction_247(unimplemented!()) });
    (unsafe { test_construction_248(unimplemented!()) });
    (unsafe { test_construction_249(unimplemented!()) });
    (unsafe { test_construction_250(unimplemented!()) });
    (unsafe { test_construction_251(unimplemented!()) });
    (unsafe { test_construction_252(unimplemented!()) });
    (unsafe { test_construction_253(unimplemented!()) });
    (unsafe { test_construction_254(unimplemented!()) });
    (unsafe { test_construction_255(unimplemented!()) });
    (unsafe { test_construction_256(unimplemented!()) });
    (unsafe { test_construction_257(unimplemented!()) });
}
pub unsafe fn test_335() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168));
    (unsafe { test_construction_258(unimplemented!()) });
    (unsafe { test_construction_259(unimplemented!()) });
    (unsafe { test_construction_260(unimplemented!()) });
    (unsafe { test_construction_261(unimplemented!()) });
    (unsafe { test_construction_262(unimplemented!()) });
    (unsafe { test_construction_263(unimplemented!()) });
    (unsafe { test_construction_264(unimplemented!()) });
    (unsafe { test_construction_265(unimplemented!()) });
    (unsafe { test_construction_266(unimplemented!()) });
    (unsafe { test_construction_267(unimplemented!()) });
    (unsafe { test_construction_268(unimplemented!()) });
    (unsafe { test_construction_269(unimplemented!()) });
    (unsafe { test_construction_270(unimplemented!()) });
    (unsafe { test_construction_271(unimplemented!()) });
    (unsafe { test_construction_272(unimplemented!()) });
    (unsafe { test_construction_273(unimplemented!()) });
    (unsafe { test_construction_274(unimplemented!()) });
    (unsafe { test_construction_275(unimplemented!()) });
}
pub unsafe fn test_336() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168));
    (unsafe { test_construction_276(unimplemented!()) });
    (unsafe { test_construction_277(unimplemented!()) });
    (unsafe { test_construction_278(unimplemented!()) });
    (unsafe { test_construction_279(unimplemented!()) });
    (unsafe { test_construction_280(unimplemented!()) });
    (unsafe { test_construction_281(unimplemented!()) });
    (unsafe { test_construction_282(unimplemented!()) });
    (unsafe { test_construction_283(unimplemented!()) });
    (unsafe { test_construction_284(unimplemented!()) });
    (unsafe { test_construction_285(unimplemented!()) });
    (unsafe { test_construction_286(unimplemented!()) });
    (unsafe { test_construction_287(unimplemented!()) });
    (unsafe { test_construction_288(unimplemented!()) });
    (unsafe { test_construction_289(unimplemented!()) });
    (unsafe { test_construction_290(unimplemented!()) });
    (unsafe { test_construction_291(unimplemented!()) });
    (unsafe { test_construction_292(unimplemented!()) });
    (unsafe { test_construction_293(unimplemented!()) });
}
pub unsafe fn test_337() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168));
    (unsafe { test_construction_294(unimplemented!()) });
    (unsafe { test_construction_295(unimplemented!()) });
    (unsafe { test_construction_296(unimplemented!()) });
    (unsafe { test_construction_297(unimplemented!()) });
    (unsafe { test_construction_298(unimplemented!()) });
    (unsafe { test_construction_299(unimplemented!()) });
    (unsafe { test_construction_300(unimplemented!()) });
    (unsafe { test_construction_301(unimplemented!()) });
    (unsafe { test_construction_302(unimplemented!()) });
    (unsafe { test_construction_303(unimplemented!()) });
    (unsafe { test_construction_304(unimplemented!()) });
    (unsafe { test_construction_305(unimplemented!()) });
    (unsafe { test_construction_306(unimplemented!()) });
    (unsafe { test_construction_307(unimplemented!()) });
    (unsafe { test_construction_308(unimplemented!()) });
    (unsafe { test_construction_309(unimplemented!()) });
    (unsafe { test_construction_310(unimplemented!()) });
    (unsafe { test_construction_311(unimplemented!()) });
}
pub unsafe fn test_338() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168));
    (unsafe { test_construction_312(unimplemented!()) });
    (unsafe { test_construction_313(unimplemented!()) });
    (unsafe { test_construction_314(unimplemented!()) });
    (unsafe { test_construction_315(unimplemented!()) });
    (unsafe { test_construction_316(unimplemented!()) });
    (unsafe { test_construction_317(unimplemented!()) });
    (unsafe { test_construction_318(unimplemented!()) });
    (unsafe { test_construction_319(unimplemented!()) });
    (unsafe { test_construction_320(unimplemented!()) });
    (unsafe { test_construction_321(unimplemented!()) });
    (unsafe { test_construction_322(unimplemented!()) });
    (unsafe { test_construction_323(unimplemented!()) });
    (unsafe { test_construction_324(unimplemented!()) });
    (unsafe { test_construction_325(unimplemented!()) });
    (unsafe { test_construction_326(unimplemented!()) });
    (unsafe { test_construction_327(unimplemented!()) });
    (unsafe { test_construction_328(unimplemented!()) });
    (unsafe { test_construction_329(unimplemented!()) });
}
pub unsafe fn test_index_type_combo_339() -> () {
    (unsafe { test_330() });
    (unsafe { test_331() });
    (unsafe { test_332() });
    (unsafe { test_333() });
    (unsafe { test_334() });
    (unsafe { test_335() });
    (unsafe { test_336() });
    (unsafe { test_337() });
    (unsafe { test_338() });
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RValueInt {
    pub val: (),
}
impl RValueInt {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
    pub unsafe fn to_(&mut self) -> () {
        return self.val;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IntegralCtorTest {}
impl IntegralCtorTest {
    pub unsafe fn test_construction_1_class_std_extents_int__struct_std_array_int__0__struct_std_array_int__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_5(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__struct_std_array_int__1__struct_std_array_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_6(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__struct_std_array_int__1__struct_std_array_int__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_6(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__struct_std_array_int__1__struct_std_array_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_7(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__7__struct_std_array_int__2__struct_std_array_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_8(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__7__struct_std_array_int__2__struct_std_array_int__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_8(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_9(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_9(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_10(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_10(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_11(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_12(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__7__9__struct_std_array_int__3__struct_std_array_int__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_12(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_13(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_13(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_14(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_14(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_15(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_15(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_16(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_17(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_17(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_18(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_18(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_19(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_19(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_20(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_20(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_21(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__4__0_1_2_3(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_21(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_22(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__struct_std_array_unsigned_long__0__struct_std_array_unsigned_long__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_23(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_24(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_24(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_25(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_26(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_26(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_27(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_27(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_28(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_28(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_29(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_30(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_30(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_31(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_31(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_32(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_32(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_33(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_33(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_34(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_35(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_35(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_36(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_36(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_37(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_37(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_38(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_38(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_39(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__4__0_1_2_3(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_39(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_40(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__struct_std_array_int__0__struct_std_array_int__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_41(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__3__struct_std_array_int__1__struct_std_array_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_42(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__3__struct_std_array_int__1__struct_std_array_int__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_42(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__struct_std_array_int__1__struct_std_array_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_43(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__3__7__struct_std_array_int__2__struct_std_array_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_44(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__3__7__struct_std_array_int__2__struct_std_array_int__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_44(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_45(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_45(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_46(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_46(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_47(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__3__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_48(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__3__7__9__struct_std_array_int__3__struct_std_array_int__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_48(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_49(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_49(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_50(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_50(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_51(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_51(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_52(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_53(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_53(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_54(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_54(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_55(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_55(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_56(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_56(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_57(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__4__0_1_2_3(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_57(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_58(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__struct_std_array_unsigned_long__0__struct_std_array_unsigned_long__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_59(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_60(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_60(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_61(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_62(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_62(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_63(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_63(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_64(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_64(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_65(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_66(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_66(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_67(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_67(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_68(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_68(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_69(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_69(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_70(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_71(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_71(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_72(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_72(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_73(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_73(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_74(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_74(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_75(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__4__0_1_2_3(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_75(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_76(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__struct_std_array_unsigned_int__0__struct_std_array_unsigned_int__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_77(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__3__struct_std_array_unsigned_int__1__struct_std_array_unsigned_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_78(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__3__struct_std_array_unsigned_int__1__struct_std_array_unsigned_int__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_78(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__18446744073709551615__struct_std_array_unsigned_int__1__struct_std_array_unsigned_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_79(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__3__7__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_80(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__3__7__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_80(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__3__18446744073709551615__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_81(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__3__18446744073709551615__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_81(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__18446744073709551615__7__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_82(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__18446744073709551615__7__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_82(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_83(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__3__7__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_84(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__3__7__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_84(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__3__7__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_85(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__3__7__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_85(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_86(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_86(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_87(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_87(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_88(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__3__18446744073709551615__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_89(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__3__18446744073709551615__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_89(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_90(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_90(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__18446744073709551615__7__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_91(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__18446744073709551615__7__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_91(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_int__9__struct_std_array_unsigned_int__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_92(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_int__9__struct_std_array_unsigned_int__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_92(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_int__9__struct_std_array_unsigned_int__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_93(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_int__9__struct_std_array_unsigned_int__4__0_1_2_3(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_93(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__9__struct_std_array_unsigned_int__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_94(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__struct_std_array_int__0__struct_std_array_int__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_95(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__struct_std_array_int__1__struct_std_array_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_96(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__struct_std_array_int__1__struct_std_array_int__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_96(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__struct_std_array_int__1__struct_std_array_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_97(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__7__struct_std_array_int__2__struct_std_array_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_98(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__7__struct_std_array_int__2__struct_std_array_int__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_98(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_99(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_99(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_100(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_100(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_101(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_102(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__7__9__struct_std_array_int__3__struct_std_array_int__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_102(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_103(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_103(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_104(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_104(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_105(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_105(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_106(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_107(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_107(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_108(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_108(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_109(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_109(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_110(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_110(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_111(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__4__0_1_2_3(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_111(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_112(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__struct_std_array_unsigned_long__0__struct_std_array_unsigned_long__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_113(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_114(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_114(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_115(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_116(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_116(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_117(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_117(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_118(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_118(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_119(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_120(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_120(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_121(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_121(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_122(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_122(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_123(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_123(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_124(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_125(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_125(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_126(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_126(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_127(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_127(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_128(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_128(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_129(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__4__0_1_2_3(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_129(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_130(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__struct_std_array_struct_IntType__0__struct_std_array_struct_IntType__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_131(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_132(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_132(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_133(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_134(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_134(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_135(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_135(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_136(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_136(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_137(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_138(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_138(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_139(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_139(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_140(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_140(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_141(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_141(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_142(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_143(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_143(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_144(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_144(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_145(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_145(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_146(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_146(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_147(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__4__0_1_2_3(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_147(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_148(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__struct_std_array_struct_IntType__0__struct_std_array_struct_IntType__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_149(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_150(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_150(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_151(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_152(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_152(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_153(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_153(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_154(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_154(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_155(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_156(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_156(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_157(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_157(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_158(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_158(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_159(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_159(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_160(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_161(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_161(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_162(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_162(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_163(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_163(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__1__2__3__4__5__6__7__8__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_164(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__1__2__3__4__5__6__7__8__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__0_(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_164(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_165(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__4__0_1_2_3(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_165(unimplemented!(), all_ext.clone()) });
    }
    pub unsafe fn test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8(
        mut all_ext: (),
        mut ext: (),
        mut _a2: (),
    ) {
        const _: () = assert!(true, "");;
        (unsafe { test_runtime_observers_166(unimplemented!(), all_ext.clone()) });
    }
}
pub unsafe fn test_ctor_from_bool_340() -> () {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168));
    let mut e: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_ctor_from_rvalue_convertible_341() -> () {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168));
    let mut e: () = unimplemented!();
    unimplemented!();
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
    (unsafe { test_index_type_combo_339() });
    const _: () = assert!(true, "test_index_type_combo<IntegralCtorTest>()");;
    (unsafe { test_ctor_from_bool_340() });
    const _: () = assert!(true, "test_ctor_from_bool()");;
    (unsafe { test_ctor_from_rvalue_convertible_341() });
    const _: () = assert!(true, "test_ctor_from_rvalue_convertible()");;
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_168));
    const _: () = assert!(true, "!std::is_constructible_v<E, int>");;
    const _: () = assert!(true, "!std::is_constructible_v<E, int, int, int>");;
    const _: () = assert!(true, "!std::is_constructible_v<E, int, int, int, int, int>");;
    const _: () = assert!(true, "std::is_convertible_v<IntType, int>");;
    const _: () = assert!(
        true,
        "!std::is_constructible_v< std::extents<unsigned long, D>, IntType>"
    );;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
