extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_bool__bool_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_partial_ordering__std_partial_ordering_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_strong_ordering__std_strong_ordering_ {}
pub unsafe fn testComparisonsComplete_3(
    t1: *const (),
    t2: *const (),
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub unsafe fn testComparisonsComplete_4(
    t1: *const (),
    t2: *const (),
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub unsafe fn AssertComparisonsReturnBool_5() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn AssertComparisonsReturnBool_6() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn AssertOrderReturn_7() {
    (unsafe { AssertComparisonsReturnBool_5() });
    const _: () = assert!(true, "");;
}
pub unsafe fn AssertOrderReturn_8() {
    (unsafe { AssertComparisonsReturnBool_6() });
    const _: () = assert!(true, "");;
}
pub unsafe fn testOrder_9(t1: *const (), t2: *const (), mut order: ()) -> () {
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && (unsafe {
            let _t1: *const () = t1;
            let _t2: *const () = t2;
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_3(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_10(t1: *const (), t2: *const (), mut order: ()) -> () {
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && (unsafe {
            let _t1: *const () = t1;
            let _t2: *const () = t2;
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_4(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn operator_lt_11(lhs: *const LessAndEqComp, rhs: *const LessAndEqComp) -> () {
    return (((*lhs).value) < ((*rhs).value));
}
pub unsafe fn operator_eq_12(lhs: *const LessAndEqComp, rhs: *const LessAndEqComp) -> () {
    return (((*lhs).value) == ((*rhs).value));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LessAndEqComp {
    pub value: (),
}
impl LessAndEqComp {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for LessAndEqComp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_11(self as *const LessAndEqComp, other as *const LessAndEqComp) {
                std::cmp::Ordering::Less
            } else if operator_lt_11(other as *const LessAndEqComp, self as *const LessAndEqComp) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for LessAndEqComp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for LessAndEqComp {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_12(self as *const LessAndEqComp, other as *const LessAndEqComp) }
    }
}
impl std::cmp::Eq for LessAndEqComp {}
pub unsafe fn operator_cmp_13(mut _a0: StrongOrder, mut _a1: StrongOrder) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&(_a0.value), &(_a1.value));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct StrongOrder {
    pub value: (),
}
impl StrongOrder {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for StrongOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_13(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for StrongOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for StrongOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_13(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for StrongOrder {}
pub unsafe fn operator_cmp_14(mut _a0: WeakOrder, mut _a1: WeakOrder) -> () {
    {
        let mut cmp: () = unimplemented!();
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WeakOrder {
    pub value: (),
}
impl WeakOrder {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for WeakOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_14(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for WeakOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for WeakOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_14(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for WeakOrder {}
pub unsafe fn operator_cmp_15(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    if ((lhs.value) == (unsafe { std_numeric_limits_int_::min() }))
        || ((rhs.value) == (unsafe { std_numeric_limits_int_::min() }))
    {
        return unimplemented!();
    }
    if ((lhs.value) == (unsafe { std_numeric_limits_int_::max() }))
        || ((rhs.value) == (unsafe { std_numeric_limits_int_::max() }))
    {
        return unimplemented!();
    }
    return unimplemented!();
}
pub unsafe fn operator_eq_16(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PartialOrder {
    pub value: (),
}
impl PartialOrder {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for PartialOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_15(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for PartialOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PartialOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_16(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for PartialOrder {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NonComparable {}
const _: () = assert!(!(false), "!std::equality_comparable<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorNotEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorSpaceship<NonComparable>");
pub unsafe fn operator_eq_17(_a0: *const EqualityComparable, _a1: *const EqualityComparable) -> () {
    return (((*_a0).value_) == ((*_a1).value_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EqualityComparable {
    value_: (),
}
impl EqualityComparable {
    pub unsafe fn new(mut value: ()) -> Self {
        let mut this = Self { value_: value };
        this
    }
}
impl std::cmp::PartialEq for EqualityComparable {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_17(
                self as *const EqualityComparable,
                other as *const EqualityComparable,
            )
        }
    }
}
impl std::cmp::Eq for EqualityComparable {}
const _: () = assert!(true, "std::equality_comparable<EqualityComparable>");
const _: () = assert!(true, "HasOperatorEqual<EqualityComparable>");
const _: () = assert!(true, "HasOperatorNotEqual<EqualityComparable>");
pub unsafe fn operator_eq_18(_a0: *const ThreeWayComparable, _a1: *const ThreeWayComparable) -> () {
    return (((*_a0).value_) == ((*_a1).value_));
}
pub unsafe fn operator_cmp_19(
    _a0: *const ThreeWayComparable,
    _a1: *const ThreeWayComparable,
) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&((*_a0).value_), &((*_a1).value_));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ThreeWayComparable {
    value_: (),
}
impl ThreeWayComparable {
    pub unsafe fn new(mut value: ()) -> Self {
        let mut this = Self { value_: value };
        this
    }
}
impl std::cmp::Ord for ThreeWayComparable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_19(
                self as *const ThreeWayComparable,
                other as *const ThreeWayComparable,
            )
        }
    }
}
impl std::cmp::PartialOrd for ThreeWayComparable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for ThreeWayComparable {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_18(
                self as *const ThreeWayComparable,
                other as *const ThreeWayComparable,
            )
        }
    }
}
impl std::cmp::Eq for ThreeWayComparable {}
const _: () = assert!(true, "std::equality_comparable<ThreeWayComparable>");
const _: () = assert!(true, "std::three_way_comparable<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorGreaterThan<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorGreaterThanEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorLessThan<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorLessThanEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorNotEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorSpaceship<ThreeWayComparable>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ImplicitBool_E1 {
    pub x: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ImplicitBool_E2 {
    pub y: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ImplicitBool {
    pub val: (),
}
impl ImplicitBool {
    pub unsafe fn to__1(&self) -> () {
        return self.val;
    }
}
pub unsafe fn operator_eq_20(mut lhs: ImplicitBool_E1, mut rhs: ImplicitBool_E2) -> ImplicitBool {
    return ImplicitBool {
        val: ((lhs.x) == (rhs.y)),
    };
}
pub unsafe fn test_with_types_21() -> () {
    (unsafe { AssertOrderReturn_7() });
    {
        let v1: () = {
            let mut __tmp_0: () = 1_();
            std_variant_int__double_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_22)).clone() },
                { &mut __tmp_0 },
            )
        };
        let v2: () = {
            let mut __tmp_1: () = 1_();
            std_variant_int__double_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_22)).clone() },
                { &mut __tmp_1 },
            )
        };
        unimplemented!();
    }
    {
        let v1: () = {
            let mut __tmp_2: () = 0_();
            std_variant_int__double_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_22)).clone() },
                { &mut __tmp_2 },
            )
        };
        let v2: () = {
            let mut __tmp_3: () = 1_();
            std_variant_int__double_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_22)).clone() },
                { &mut __tmp_3 },
            )
        };
        unimplemented!();
    }
    {
        let v1: () = {
            let mut __tmp_4: () = 1_();
            std_variant_int__double_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_22)).clone() },
                { &mut __tmp_4 },
            )
        };
        let v2: () = {
            let mut __tmp_5: () = 0_();
            std_variant_int__double_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_22)).clone() },
                { &mut __tmp_5 },
            )
        };
        unimplemented!();
    }
    {
        let v1: () = {
            let mut __tmp_6: () = 0_();
            std_variant_int__double_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_22)).clone() },
                { &mut __tmp_6 },
            )
        };
        let v2: () = {
            let mut __tmp_7: () = 0_();
            std_variant_int__double_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_23)).clone() },
                { &mut __tmp_7 },
            )
        };
        unimplemented!();
    }
    {
        let v1: () = {
            let mut __tmp_8: () = 0_();
            std_variant_int__double_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_23)).clone() },
                { &mut __tmp_8 },
            )
        };
        let v2: () = {
            let mut __tmp_9: () = 0_();
            std_variant_int__double_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_22)).clone() },
                { &mut __tmp_9 },
            )
        };
        unimplemented!();
    }
    return true;
}
pub unsafe fn test_with_types_24() -> () {
    (unsafe { AssertOrderReturn_8() });
    {
        let v1: () = {
            let mut __tmp_10: () = 1_();
            std_variant_int__long_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_22)).clone() },
                { &mut __tmp_10 },
            )
        };
        let v2: () = {
            let mut __tmp_11: () = 1_();
            std_variant_int__long_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_22)).clone() },
                { &mut __tmp_11 },
            )
        };
        unimplemented!();
    }
    {
        let v1: () = {
            let mut __tmp_12: () = 0_();
            std_variant_int__long_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_22)).clone() },
                { &mut __tmp_12 },
            )
        };
        let v2: () = {
            let mut __tmp_13: () = 1_();
            std_variant_int__long_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_22)).clone() },
                { &mut __tmp_13 },
            )
        };
        unimplemented!();
    }
    {
        let v1: () = {
            let mut __tmp_14: () = 1_();
            std_variant_int__long_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_22)).clone() },
                { &mut __tmp_14 },
            )
        };
        let v2: () = {
            let mut __tmp_15: () = 0_();
            std_variant_int__long_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_22)).clone() },
                { &mut __tmp_15 },
            )
        };
        unimplemented!();
    }
    {
        let v1: () = {
            let mut __tmp_16: () = 0_();
            std_variant_int__long_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_22)).clone() },
                { &mut __tmp_16 },
            )
        };
        let v2: () = {
            let mut __tmp_17: () = 0_();
            std_variant_int__long_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_23)).clone() },
                { &mut __tmp_17 },
            )
        };
        unimplemented!();
    }
    {
        let v1: () = {
            let mut __tmp_18: () = 0_();
            std_variant_int__long_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_23)).clone() },
                { &mut __tmp_18 },
            )
        };
        let v2: () = {
            let mut __tmp_19: () = 0_();
            std_variant_int__long_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_22)).clone() },
                { &mut __tmp_19 },
            )
        };
        unimplemented!();
    }
    return true;
}
pub unsafe fn test_three_way_25() -> () {
    unimplemented!();
    unimplemented!();
    {
        let nan: () = (unsafe { std_numeric_limits_double_::quiet_NaN() });
        {
            let v1: () = {
                let mut __tmp_20: () = 1_();
                std_variant_int__double_::new_3(
                    { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_type_26)).clone() },
                    { &mut __tmp_20 },
                )
            };
            let v2: () = std_variant_int__double_::new_4(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_type_27)).clone() },
                { &nan },
            );
            unimplemented!();
        }
        {
            let v1: () = std_variant_int__double_::new_4(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_type_27)).clone() },
                { &nan },
            );
            let v2: () = {
                let mut __tmp_21: () = 2_();
                std_variant_int__double_::new_3(
                    { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_type_26)).clone() },
                    { &mut __tmp_21 },
                )
            };
            unimplemented!();
        }
        {
            let v1: () = std_variant_int__double_::new_4(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_type_27)).clone() },
                { &nan },
            );
            let v2: () = std_variant_int__double_::new_4(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_type_27)).clone() },
                { &nan },
            );
            unimplemented!();
        }
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasSimpleOrdering {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasOnlySpaceship {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasFullOrdering {}
const _: () = assert!(!(false), "!has_three_way_op<HasSimpleOrdering>");
const _: () = assert!(
    !(false),
    "!has_three_way_op<std::variant<int, HasSimpleOrdering>>"
);
const _: () = assert!(!(false), "!three_way_comparable<HasSimpleOrdering>");
const _: () = assert!(
    !(false),
    "!three_way_comparable<std::variant<int, HasSimpleOrdering>>"
);
const _: () = assert!(true, "has_three_way_op<HasOnlySpaceship>");
const _: () = assert!(
    !(false),
    "!has_three_way_op<std::variant<int, HasOnlySpaceship>>"
);
const _: () = assert!(!(false), "!three_way_comparable<HasOnlySpaceship>");
const _: () = assert!(
    !(false),
    "!three_way_comparable<std::variant<int, HasOnlySpaceship>>"
);
const _: () = assert!(true, "has_three_way_op<HasFullOrdering>");
const _: () = assert!(true, "has_three_way_op<std::variant<int, HasFullOrdering>>");
const _: () = assert!(true, "three_way_comparable<HasFullOrdering>");
const _: () = assert!(
    true,
    "three_way_comparable<std::variant<int, HasFullOrdering>>"
);
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
    (unsafe { test_three_way_25() });
    const _: () = assert!(true, "test_three_way()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
