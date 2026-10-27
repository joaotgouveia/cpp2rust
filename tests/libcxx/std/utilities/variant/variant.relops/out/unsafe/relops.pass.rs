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
    if ((lhs.value) == (unimplemented!())) || ((rhs.value) == (unimplemented!())) {
        return unimplemented!();
    }
    if ((lhs.value) == (unimplemented!())) || ((rhs.value) == (unimplemented!())) {
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
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
}
pub unsafe fn test_equality_basic_17() {
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
}
pub unsafe fn test_equality_basic_18() {
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
}
pub unsafe fn test_equality_basic_19() {
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
}
pub unsafe fn test_equality_20() {
    (unsafe { test_equality_basic_16() });
    (unsafe { test_equality_basic_17() });
    (unsafe { test_equality_basic_18() });
    (unsafe { test_equality_basic_19() });
}
pub unsafe fn test_less_21(
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
pub unsafe fn test_less_22(
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
pub unsafe fn test_relational_basic_25() {
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "test_less(v1, v2, false, false)");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
}
pub unsafe fn test_relational_basic_26() {
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "test_less(v1, v2, false, false)");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
}
pub unsafe fn test_relational_basic_27() {
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "test_less(v1, v2, false, false)");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
}
pub unsafe fn test_relational_basic_28() {
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "test_less(v1, v2, false, false)");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: () = unimplemented!();
        let v2: () = unimplemented!();
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
}
pub unsafe fn test_relational_29() {
    (unsafe { test_relational_basic_25() });
    (unsafe { test_relational_basic_26() });
    (unsafe { test_relational_basic_27() });
    (unsafe { test_relational_basic_28() });
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
pub unsafe fn operator_eq_30(lhs: *const Array, rhs: *const Array) -> () {
    return unimplemented!();
}
pub unsafe fn operator_ne_31(lhs: *const Array, rhs: *const Array) -> () {
    return unimplemented!();
}
pub unsafe fn operator_lt_32(lhs: *const Array, rhs: *const Array) -> () {
    return unimplemented!();
}
pub unsafe fn operator_gt_33(lhs: *const Array, rhs: *const Array) -> () {
    return unimplemented!();
}
pub unsafe fn operator_le_34(lhs: *const Array, rhs: *const Array) -> () {
    return unimplemented!();
}
pub unsafe fn operator_ge_35(lhs: *const Array, rhs: *const Array) -> () {
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
            if operator_lt_32(self as *const Array, other as *const Array) {
                std::cmp::Ordering::Less
            } else if operator_lt_32(other as *const Array, self as *const Array) {
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
        unsafe { operator_eq_30(self as *const Array, other as *const Array) }
    }
}
impl std::cmp::Eq for Array {}
pub unsafe fn test_recursive_36() {
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
    (unsafe { test_equality_20() });
    (unsafe { test_relational_29() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
