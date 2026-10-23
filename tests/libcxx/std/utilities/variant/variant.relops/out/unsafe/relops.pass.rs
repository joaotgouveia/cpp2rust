extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_lt_0(lhs: *const LessAndEqComp, rhs: *const LessAndEqComp) -> () {
    return (((*lhs).value) < ((*rhs).value));
}
pub unsafe fn operator_eq_1(lhs: *const LessAndEqComp, rhs: *const LessAndEqComp) -> () {
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
            if operator_lt_0(self as *const LessAndEqComp, other as *const LessAndEqComp) {
                std::cmp::Ordering::Less
            } else if operator_lt_0(other as *const LessAndEqComp, self as *const LessAndEqComp) {
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
        unsafe { operator_eq_1(self as *const LessAndEqComp, other as *const LessAndEqComp) }
    }
}
impl std::cmp::Eq for LessAndEqComp {}
pub unsafe fn operator_cmp_2(mut _a0: StrongOrder, mut _a1: StrongOrder) -> () {
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
        unsafe { operator_cmp_2(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for StrongOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for StrongOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_2(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for StrongOrder {}
pub unsafe fn operator_cmp_3(mut _a0: WeakOrder, mut _a1: WeakOrder) -> () {
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
        unsafe { operator_cmp_3(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for WeakOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for WeakOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_3(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for WeakOrder {}
pub unsafe fn operator_cmp_4(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
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
pub unsafe fn operator_eq_5(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
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
        unsafe { operator_cmp_4(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for PartialOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PartialOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_5(self.clone(), other.clone()) }
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
pub unsafe fn operator_eq_6(_a0: *const EqualityComparable, _a1: *const EqualityComparable) -> () {
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
            operator_eq_6(
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
pub unsafe fn operator_eq_7(_a0: *const ThreeWayComparable, _a1: *const ThreeWayComparable) -> () {
    return (((*_a0).value_) == ((*_a1).value_));
}
pub unsafe fn operator_cmp_8(_a0: *const ThreeWayComparable, _a1: *const ThreeWayComparable) -> () {
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
            operator_cmp_8(
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
            operator_eq_7(
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
pub unsafe fn operator_eq_9(mut lhs: ImplicitBool_E1, mut rhs: ImplicitBool_E2) -> ImplicitBool {
    return ImplicitBool {
        val: ((lhs.x) == (rhs.y)),
    };
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MyBool {
    pub value: (),
}
impl MyBool {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
    pub unsafe fn to_(&self) -> () {
        return self.value;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ComparesToMyBool {
    pub value: (),
}
impl std::cmp::Ord for ComparesToMyBool {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if {
                let __cmp = &operator_lt_10(
                    self as *const ComparesToMyBool,
                    other as *const ComparesToMyBool,
                );
                MyBool::to_(__cmp)
            } {
                std::cmp::Ordering::Less
            } else if {
                let __cmp = &operator_lt_10(
                    other as *const ComparesToMyBool,
                    self as *const ComparesToMyBool,
                );
                MyBool::to_(__cmp)
            } {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for ComparesToMyBool {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for ComparesToMyBool {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            {
                let __cmp = &operator_eq_11(
                    self as *const ComparesToMyBool,
                    other as *const ComparesToMyBool,
                );
                MyBool::to_(__cmp)
            }
        }
    }
}
impl std::cmp::Eq for ComparesToMyBool {}
impl Default for ComparesToMyBool {
    fn default() -> Self {
        unsafe { ComparesToMyBool { value: 0_() } }
    }
}
pub unsafe fn operator_eq_11(LHS: *const ComparesToMyBool, RHS: *const ComparesToMyBool) -> MyBool {
    return MyBool::new({ (((*LHS).value) == ((*RHS).value)) });
}
pub unsafe fn operator_ne_12(LHS: *const ComparesToMyBool, RHS: *const ComparesToMyBool) -> MyBool {
    return MyBool::new({ (((*LHS).value) != ((*RHS).value)) });
}
pub unsafe fn operator_lt_10(LHS: *const ComparesToMyBool, RHS: *const ComparesToMyBool) -> MyBool {
    return MyBool::new({ (((*LHS).value) < ((*RHS).value)) });
}
pub unsafe fn operator_le_13(LHS: *const ComparesToMyBool, RHS: *const ComparesToMyBool) -> MyBool {
    return MyBool::new({ (((*LHS).value) <= ((*RHS).value)) });
}
pub unsafe fn operator_gt_14(LHS: *const ComparesToMyBool, RHS: *const ComparesToMyBool) -> MyBool {
    return MyBool::new({ (((*LHS).value) > ((*RHS).value)) });
}
pub unsafe fn operator_ge_15(LHS: *const ComparesToMyBool, RHS: *const ComparesToMyBool) -> MyBool {
    return MyBool::new({ (((*LHS).value) >= ((*RHS).value)) });
}
pub unsafe fn test_equality_basic_16() {
    {
        let v1: () = {
            let mut __tmp_0: () = 42_();
            std_variant_int__long_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_0 },
            )
        };
        let v2: () = {
            let mut __tmp_1: () = 42_();
            std_variant_int__long_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_1 },
            )
        };
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
    {
        let v1: () = {
            let mut __tmp_2: () = 42_();
            std_variant_int__long_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_2 },
            )
        };
        let v2: () = {
            let mut __tmp_3: () = 43_();
            std_variant_int__long_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_3 },
            )
        };
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: () = {
            let mut __tmp_4: () = 42_();
            std_variant_int__long_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_4 },
            )
        };
        let v2: () = {
            let mut __tmp_5: () = 42_();
            std_variant_int__long_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_18)).clone() },
                { &mut __tmp_5 },
            )
        };
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: () = {
            let mut __tmp_6: () = 42_();
            std_variant_int__long_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_18)).clone() },
                { &mut __tmp_6 },
            )
        };
        let v2: () = {
            let mut __tmp_7: () = 42_();
            std_variant_int__long_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_18)).clone() },
                { &mut __tmp_7 },
            )
        };
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
}
pub unsafe fn test_equality_basic_19() {
    {
        let v1: () = {
            let mut __tmp_8: ComparesToMyBool = ComparesToMyBool { value: 42_() };
            std_variant_ComparesToMyBool__int_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_8 },
            )
        };
        let v2: () = {
            let mut __tmp_9: ComparesToMyBool = ComparesToMyBool { value: 42_() };
            std_variant_ComparesToMyBool__int_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_9 },
            )
        };
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
    {
        let v1: () = {
            let mut __tmp_10: ComparesToMyBool = ComparesToMyBool { value: 42_() };
            std_variant_ComparesToMyBool__int_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_10 },
            )
        };
        let v2: () = {
            let mut __tmp_11: ComparesToMyBool = ComparesToMyBool { value: 43_() };
            std_variant_ComparesToMyBool__int_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_11 },
            )
        };
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: () = {
            let mut __tmp_12: ComparesToMyBool = ComparesToMyBool { value: 42_() };
            std_variant_ComparesToMyBool__int_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_12 },
            )
        };
        let v2: () = {
            let mut __tmp_13: () = 42_();
            std_variant_ComparesToMyBool__int_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_18)).clone() },
                { &mut __tmp_13 },
            )
        };
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: () = {
            let mut __tmp_14: () = 42_();
            std_variant_ComparesToMyBool__int_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_18)).clone() },
                { &mut __tmp_14 },
            )
        };
        let v2: () = {
            let mut __tmp_15: () = 42_();
            std_variant_ComparesToMyBool__int_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_18)).clone() },
                { &mut __tmp_15 },
            )
        };
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
}
pub unsafe fn test_equality_basic_20() {
    {
        let v1: () = {
            let mut __tmp_16: () = 42_();
            std_variant_int__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_16 },
            )
        };
        let v2: () = {
            let mut __tmp_17: () = 42_();
            std_variant_int__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_17 },
            )
        };
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
    {
        let v1: () = {
            let mut __tmp_18: () = 42_();
            std_variant_int__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_18 },
            )
        };
        let v2: () = {
            let mut __tmp_19: () = 43_();
            std_variant_int__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_19 },
            )
        };
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: () = {
            let mut __tmp_20: () = 42_();
            std_variant_int__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_20 },
            )
        };
        let v2: () = {
            let mut __tmp_21: ComparesToMyBool = ComparesToMyBool { value: 42_() };
            std_variant_int__ComparesToMyBool_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_18)).clone() },
                { &mut __tmp_21 },
            )
        };
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: () = {
            let mut __tmp_22: ComparesToMyBool = ComparesToMyBool { value: 42_() };
            std_variant_int__ComparesToMyBool_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_18)).clone() },
                { &mut __tmp_22 },
            )
        };
        let v2: () = {
            let mut __tmp_23: ComparesToMyBool = ComparesToMyBool { value: 42_() };
            std_variant_int__ComparesToMyBool_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_18)).clone() },
                { &mut __tmp_23 },
            )
        };
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
}
pub unsafe fn test_equality_basic_21() {
    {
        let v1: () = {
            let mut __tmp_24: ComparesToMyBool = ComparesToMyBool { value: 42_() };
            std_variant_ComparesToMyBool__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_24 },
            )
        };
        let v2: () = {
            let mut __tmp_25: ComparesToMyBool = ComparesToMyBool { value: 42_() };
            std_variant_ComparesToMyBool__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_25 },
            )
        };
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
    {
        let v1: () = {
            let mut __tmp_26: ComparesToMyBool = ComparesToMyBool { value: 42_() };
            std_variant_ComparesToMyBool__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_26 },
            )
        };
        let v2: () = {
            let mut __tmp_27: ComparesToMyBool = ComparesToMyBool { value: 43_() };
            std_variant_ComparesToMyBool__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_27 },
            )
        };
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: () = {
            let mut __tmp_28: ComparesToMyBool = ComparesToMyBool { value: 42_() };
            std_variant_ComparesToMyBool__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_28 },
            )
        };
        let v2: () = {
            let mut __tmp_29: ComparesToMyBool = ComparesToMyBool { value: 42_() };
            std_variant_ComparesToMyBool__ComparesToMyBool_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_18)).clone() },
                { &mut __tmp_29 },
            )
        };
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: () = {
            let mut __tmp_30: ComparesToMyBool = ComparesToMyBool { value: 42_() };
            std_variant_ComparesToMyBool__ComparesToMyBool_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_18)).clone() },
                { &mut __tmp_30 },
            )
        };
        let v2: () = {
            let mut __tmp_31: ComparesToMyBool = ComparesToMyBool { value: 42_() };
            std_variant_ComparesToMyBool__ComparesToMyBool_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_18)).clone() },
                { &mut __tmp_31 },
            )
        };
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
}
pub unsafe fn test_equality_22() {
    (unsafe { test_equality_basic_16() });
    (unsafe { test_equality_basic_19() });
    (unsafe { test_equality_basic_20() });
    (unsafe { test_equality_basic_21() });
}
pub unsafe fn test_less_23(
    l: *const (),
    r: *const (),
    mut expect_less: (),
    mut expect_greater: (),
) -> () {
    const _: () = assert!(true, "std::is_same_v<decltype(l < r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l <= r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l > r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l >= r), bool>");;
    return ((((unimplemented!()) == (expect_less)) && ((!(unimplemented!())) == (expect_less)))
        && ((unimplemented!()) == (expect_greater)))
        && ((!(unimplemented!())) == (expect_greater));
}
pub unsafe fn test_less_24(
    l: *const (),
    r: *const (),
    mut expect_less: (),
    mut expect_greater: (),
) -> () {
    const _: () = assert!(true, "std::is_same_v<decltype(l < r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l <= r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l > r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l >= r), bool>");;
    return ((((unimplemented!()) == (expect_less)) && ((!(unimplemented!())) == (expect_less)))
        && ((unimplemented!()) == (expect_greater)))
        && ((!(unimplemented!())) == (expect_greater));
}
pub unsafe fn test_less_25(
    l: *const (),
    r: *const (),
    mut expect_less: (),
    mut expect_greater: (),
) -> () {
    const _: () = assert!(true, "std::is_same_v<decltype(l < r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l <= r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l > r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l >= r), bool>");;
    return ((((unimplemented!()) == (expect_less)) && ((!(unimplemented!())) == (expect_less)))
        && ((unimplemented!()) == (expect_greater)))
        && ((!(unimplemented!())) == (expect_greater));
}
pub unsafe fn test_less_26(
    l: *const (),
    r: *const (),
    mut expect_less: (),
    mut expect_greater: (),
) -> () {
    const _: () = assert!(true, "std::is_same_v<decltype(l < r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l <= r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l > r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l >= r), bool>");;
    return ((((unimplemented!()) == (expect_less)) && ((!(unimplemented!())) == (expect_less)))
        && ((unimplemented!()) == (expect_greater)))
        && ((!(unimplemented!())) == (expect_greater));
}
pub unsafe fn test_relational_basic_27() {
    {
        let v1: () = {
            let mut __tmp_32: () = 1_();
            std_variant_int__long_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_32 },
            )
        };
        let v2: () = {
            let mut __tmp_33: () = 1_();
            std_variant_int__long_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_33 },
            )
        };
        const _: () = assert!(true, "test_less(v1, v2, false, false)");;
    }
    {
        let v1: () = {
            let mut __tmp_34: () = 0_();
            std_variant_int__long_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_34 },
            )
        };
        let v2: () = {
            let mut __tmp_35: () = 1_();
            std_variant_int__long_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_35 },
            )
        };
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: () = {
            let mut __tmp_36: () = 1_();
            std_variant_int__long_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_36 },
            )
        };
        let v2: () = {
            let mut __tmp_37: () = 0_();
            std_variant_int__long_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_37 },
            )
        };
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
    {
        let v1: () = {
            let mut __tmp_38: () = 0_();
            std_variant_int__long_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_38 },
            )
        };
        let v2: () = {
            let mut __tmp_39: () = 0_();
            std_variant_int__long_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_18)).clone() },
                { &mut __tmp_39 },
            )
        };
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: () = {
            let mut __tmp_40: () = 0_();
            std_variant_int__long_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_18)).clone() },
                { &mut __tmp_40 },
            )
        };
        let v2: () = {
            let mut __tmp_41: () = 0_();
            std_variant_int__long_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_41 },
            )
        };
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
}
pub unsafe fn test_relational_basic_28() {
    {
        let v1: () = {
            let mut __tmp_42: ComparesToMyBool = ComparesToMyBool { value: 1_() };
            std_variant_ComparesToMyBool__int_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_42 },
            )
        };
        let v2: () = {
            let mut __tmp_43: ComparesToMyBool = ComparesToMyBool { value: 1_() };
            std_variant_ComparesToMyBool__int_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_43 },
            )
        };
        const _: () = assert!(true, "test_less(v1, v2, false, false)");;
    }
    {
        let v1: () = {
            let mut __tmp_44: ComparesToMyBool = ComparesToMyBool { value: 0_() };
            std_variant_ComparesToMyBool__int_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_44 },
            )
        };
        let v2: () = {
            let mut __tmp_45: ComparesToMyBool = ComparesToMyBool { value: 1_() };
            std_variant_ComparesToMyBool__int_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_45 },
            )
        };
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: () = {
            let mut __tmp_46: ComparesToMyBool = ComparesToMyBool { value: 1_() };
            std_variant_ComparesToMyBool__int_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_46 },
            )
        };
        let v2: () = {
            let mut __tmp_47: ComparesToMyBool = ComparesToMyBool { value: 0_() };
            std_variant_ComparesToMyBool__int_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_47 },
            )
        };
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
    {
        let v1: () = {
            let mut __tmp_48: ComparesToMyBool = ComparesToMyBool { value: 0_() };
            std_variant_ComparesToMyBool__int_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_48 },
            )
        };
        let v2: () = {
            let mut __tmp_49: () = 0_();
            std_variant_ComparesToMyBool__int_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_18)).clone() },
                { &mut __tmp_49 },
            )
        };
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: () = {
            let mut __tmp_50: () = 0_();
            std_variant_ComparesToMyBool__int_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_18)).clone() },
                { &mut __tmp_50 },
            )
        };
        let v2: () = {
            let mut __tmp_51: ComparesToMyBool = ComparesToMyBool { value: 0_() };
            std_variant_ComparesToMyBool__int_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_51 },
            )
        };
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
}
pub unsafe fn test_relational_basic_29() {
    {
        let v1: () = {
            let mut __tmp_52: () = 1_();
            std_variant_int__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_52 },
            )
        };
        let v2: () = {
            let mut __tmp_53: () = 1_();
            std_variant_int__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_53 },
            )
        };
        const _: () = assert!(true, "test_less(v1, v2, false, false)");;
    }
    {
        let v1: () = {
            let mut __tmp_54: () = 0_();
            std_variant_int__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_54 },
            )
        };
        let v2: () = {
            let mut __tmp_55: () = 1_();
            std_variant_int__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_55 },
            )
        };
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: () = {
            let mut __tmp_56: () = 1_();
            std_variant_int__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_56 },
            )
        };
        let v2: () = {
            let mut __tmp_57: () = 0_();
            std_variant_int__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_57 },
            )
        };
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
    {
        let v1: () = {
            let mut __tmp_58: () = 0_();
            std_variant_int__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_58 },
            )
        };
        let v2: () = {
            let mut __tmp_59: ComparesToMyBool = ComparesToMyBool { value: 0_() };
            std_variant_int__ComparesToMyBool_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_18)).clone() },
                { &mut __tmp_59 },
            )
        };
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: () = {
            let mut __tmp_60: ComparesToMyBool = ComparesToMyBool { value: 0_() };
            std_variant_int__ComparesToMyBool_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_18)).clone() },
                { &mut __tmp_60 },
            )
        };
        let v2: () = {
            let mut __tmp_61: () = 0_();
            std_variant_int__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_61 },
            )
        };
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
}
pub unsafe fn test_relational_basic_30() {
    {
        let v1: () = {
            let mut __tmp_62: ComparesToMyBool = ComparesToMyBool { value: 1_() };
            std_variant_ComparesToMyBool__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_62 },
            )
        };
        let v2: () = {
            let mut __tmp_63: ComparesToMyBool = ComparesToMyBool { value: 1_() };
            std_variant_ComparesToMyBool__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_63 },
            )
        };
        const _: () = assert!(true, "test_less(v1, v2, false, false)");;
    }
    {
        let v1: () = {
            let mut __tmp_64: ComparesToMyBool = ComparesToMyBool { value: 0_() };
            std_variant_ComparesToMyBool__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_64 },
            )
        };
        let v2: () = {
            let mut __tmp_65: ComparesToMyBool = ComparesToMyBool { value: 1_() };
            std_variant_ComparesToMyBool__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_65 },
            )
        };
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: () = {
            let mut __tmp_66: ComparesToMyBool = ComparesToMyBool { value: 1_() };
            std_variant_ComparesToMyBool__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_66 },
            )
        };
        let v2: () = {
            let mut __tmp_67: ComparesToMyBool = ComparesToMyBool { value: 0_() };
            std_variant_ComparesToMyBool__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_67 },
            )
        };
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
    {
        let v1: () = {
            let mut __tmp_68: ComparesToMyBool = ComparesToMyBool { value: 0_() };
            std_variant_ComparesToMyBool__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_68 },
            )
        };
        let v2: () = {
            let mut __tmp_69: ComparesToMyBool = ComparesToMyBool { value: 0_() };
            std_variant_ComparesToMyBool__ComparesToMyBool_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_18)).clone() },
                { &mut __tmp_69 },
            )
        };
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: () = {
            let mut __tmp_70: ComparesToMyBool = ComparesToMyBool { value: 0_() };
            std_variant_ComparesToMyBool__ComparesToMyBool_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_18)).clone() },
                { &mut __tmp_70 },
            )
        };
        let v2: () = {
            let mut __tmp_71: ComparesToMyBool = ComparesToMyBool { value: 0_() };
            std_variant_ComparesToMyBool__ComparesToMyBool_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_17)).clone() },
                { &mut __tmp_71 },
            )
        };
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
}
pub unsafe fn test_relational_31() {
    (unsafe { test_relational_basic_27() });
    (unsafe { test_relational_basic_28() });
    (unsafe { test_relational_basic_29() });
    (unsafe { test_relational_basic_30() });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A1 {}
impl A1 {
    pub unsafe fn operator_eq(&self, _a0: *const A1) -> () {
        return true;
    }
    pub unsafe fn operator_ne(&self, _a0: *const A1) -> () {
        return true;
    }
    pub unsafe fn operator_lt(&self, _a0: *const A1) -> () {
        return true;
    }
    pub unsafe fn operator_gt(&self, _a0: *const A1) -> () {
        return true;
    }
    pub unsafe fn operator_le(&self, _a0: *const A1) -> () {
        return true;
    }
    pub unsafe fn operator_ge(&self, _a0: *const A1) -> () {
        return true;
    }
}
impl std::cmp::Ord for A1 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if A1::operator_lt(self, other as *const A1) {
                std::cmp::Ordering::Less
            } else if A1::operator_lt(other, self as *const A1) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for A1 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for A1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { A1::operator_eq(self, other as *const A1) }
    }
}
impl std::cmp::Eq for A1 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A2 {}
impl A2 {
    pub unsafe fn operator_eq(&self, _a0: *const A2) -> () {
        return true;
    }
    pub unsafe fn operator_ne(&self, _a0: *const A2) -> () {
        return true;
    }
    pub unsafe fn operator_lt(&self, _a0: *const A2) -> () {
        return true;
    }
    pub unsafe fn operator_gt(&self, _a0: *const A2) -> () {
        return true;
    }
    pub unsafe fn operator_le(&self, _a0: *const A2) -> () {
        return true;
    }
    pub unsafe fn operator_ge(&self, _a0: *const A2) -> () {
        return true;
    }
}
impl std::cmp::Ord for A2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if A2::operator_lt(self, other as *const A2) {
                std::cmp::Ordering::Less
            } else if A2::operator_lt(other, self as *const A2) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for A2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for A2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { A2::operator_eq(self, other as *const A2) }
    }
}
impl std::cmp::Eq for A2 {}
pub unsafe fn operator_eq_32(lhs: *const Array, rhs: *const Array) -> () {
    return unimplemented!();
}
pub unsafe fn operator_ne_33(lhs: *const Array, rhs: *const Array) -> () {
    return unimplemented!();
}
pub unsafe fn operator_lt_34(lhs: *const Array, rhs: *const Array) -> () {
    return unimplemented!();
}
pub unsafe fn operator_gt_35(lhs: *const Array, rhs: *const Array) -> () {
    return unimplemented!();
}
pub unsafe fn operator_le_36(lhs: *const Array, rhs: *const Array) -> () {
    return unimplemented!();
}
pub unsafe fn operator_ge_37(lhs: *const Array, rhs: *const Array) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Array {
    pub ptr: *mut (),
}
impl std::cmp::Ord for Array {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_34(self as *const Array, other as *const Array) {
                std::cmp::Ordering::Less
            } else if operator_lt_34(other as *const Array, self as *const Array) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Array {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Array {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_32(self as *const Array, other as *const Array) }
    }
}
impl std::cmp::Eq for Array {}
pub unsafe fn test_recursive_38() {
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
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
    (unsafe { test_equality_22() });
    (unsafe { test_relational_31() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
