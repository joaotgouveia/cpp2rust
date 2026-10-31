extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_bool__bool_ {}
pub unsafe fn testComparisonsComplete_1(
    t1: *const Vec<i32>,
    t2: *const Vec<i32>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= (1))
            && (!(c"at most one of isEqual, isLess, and isGreater can be true".as_ptr()).is_null())
    );
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    }
    return true;
}
pub unsafe fn testComparisonsComplete_2(
    t1: *const Vec<i32>,
    t2: *const Vec<i32>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= (1))
            && (!(c"at most one of isEqual, isLess, and isGreater can be true".as_ptr()).is_null())
    );
    if isEqual {
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
    } else if isLess {
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
        {}
        {
            return false;
        }
        {
            return false;
        }
        {}
    } else if isGreater {
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {}
        {
            return false;
        }
        {
            return false;
        }
        {}
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
    } else {
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
        {
            return false;
        }
    }
    return true;
}
pub unsafe fn testComparisonsComplete_3(
    t1: *const Vec<LessAndEqComp>,
    t2: *const Vec<LessAndEqComp>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= (1))
            && (!(c"at most one of isEqual, isLess, and isGreater can be true".as_ptr()).is_null())
    );
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
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
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
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
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
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
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
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
    t1: *const Vec<LessAndEqComp>,
    t2: *const Vec<LessAndEqComp>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= (1))
            && (!(c"at most one of isEqual, isLess, and isGreater can be true".as_ptr()).is_null())
    );
    if isEqual {
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
    } else if isLess {
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
        {}
        {
            return false;
        }
        {
            return false;
        }
        {}
    } else if isGreater {
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {}
        {
            return false;
        }
        {
            return false;
        }
        {}
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
    } else {
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
        {
            return false;
        }
        {}
        {}
        {
            return false;
        }
        {
            return false;
        }
    }
    return true;
}
pub unsafe fn testComparisons_5(
    t1: *const Vec<i32>,
    t2: *const Vec<i32>,
    mut isEqual: bool,
    mut isLess: bool,
) -> bool {
    assert!(
        (!((isEqual) && (isLess)))
            && (!(c"isEqual and isLess cannot be both true".as_ptr()).is_null())
    );
    let mut isGreater: bool = (!(isEqual)) && (!(isLess));
    return (unsafe {
        let _t1: *const Vec<i32> = t1;
        let _t2: *const Vec<i32> = t2;
        let _isEqual: bool = isEqual;
        let _isLess: bool = isLess;
        let _isGreater: bool = isGreater;
        testComparisonsComplete_1(_t1, _t2, _isEqual, _isLess, _isGreater)
    });
}
pub unsafe fn testComparisons_6(
    t1: *const Vec<i32>,
    t2: *const Vec<i32>,
    mut isEqual: bool,
    mut isLess: bool,
) -> bool {
    assert!(
        (!((isEqual) && (isLess)))
            && (!(c"isEqual and isLess cannot be both true".as_ptr()).is_null())
    );
    let mut isGreater: bool = (!(isEqual)) && (!(isLess));
    return (unsafe {
        let _t1: *const Vec<i32> = t1;
        let _t2: *const Vec<i32> = t2;
        let _isEqual: bool = isEqual;
        let _isLess: bool = isLess;
        let _isGreater: bool = isGreater;
        testComparisonsComplete_2(_t1, _t2, _isEqual, _isLess, _isGreater)
    });
}
pub unsafe fn testComparisons_7(
    t1: *const Vec<LessAndEqComp>,
    t2: *const Vec<LessAndEqComp>,
    mut isEqual: bool,
    mut isLess: bool,
) -> bool {
    assert!(
        (!((isEqual) && (isLess)))
            && (!(c"isEqual and isLess cannot be both true".as_ptr()).is_null())
    );
    let mut isGreater: bool = (!(isEqual)) && (!(isLess));
    return (unsafe {
        let _t1: *const Vec<LessAndEqComp> = t1;
        let _t2: *const Vec<LessAndEqComp> = t2;
        let _isEqual: bool = isEqual;
        let _isLess: bool = isLess;
        let _isGreater: bool = isGreater;
        testComparisonsComplete_3(_t1, _t2, _isEqual, _isLess, _isGreater)
    });
}
pub unsafe fn testComparisons_8(
    t1: *const Vec<LessAndEqComp>,
    t2: *const Vec<LessAndEqComp>,
    mut isEqual: bool,
    mut isLess: bool,
) -> bool {
    assert!(
        (!((isEqual) && (isLess)))
            && (!(c"isEqual and isLess cannot be both true".as_ptr()).is_null())
    );
    let mut isGreater: bool = (!(isEqual)) && (!(isLess));
    return (unsafe {
        let _t1: *const Vec<LessAndEqComp> = t1;
        let _t2: *const Vec<LessAndEqComp> = t2;
        let _isEqual: bool = isEqual;
        let _isLess: bool = isLess;
        let _isGreater: bool = isGreater;
        testComparisonsComplete_4(_t1, _t2, _isEqual, _isLess, _isGreater)
    });
}
pub unsafe fn AssertComparisonsReturnBool_9() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn AssertComparisonsReturnBool_10() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn operator_lt_11(lhs: *const LessAndEqComp, rhs: *const LessAndEqComp) -> bool {
    return (((*lhs).value) < ((*rhs).value));
}
pub unsafe fn operator_eq_12(lhs: *const LessAndEqComp, rhs: *const LessAndEqComp) -> bool {
    return (((*lhs).value) == ((*rhs).value));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct LessAndEqComp {
    pub value: i32,
}
impl LessAndEqComp {
    pub unsafe fn new(mut v: i32) -> Self {
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
pub unsafe fn operator_cmp_13(mut _a0: StrongOrder, mut _a1: StrongOrder) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&(_a0.value), &(_a1.value));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct StrongOrder {
    pub value: i32,
}
impl StrongOrder {
    pub unsafe fn new(mut v: i32) -> Self {
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
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct WeakOrder {
    pub value: i32,
}
impl WeakOrder {
    pub unsafe fn new(mut v: i32) -> Self {
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
    if ((lhs.value) == (i32::MIN)) || ((rhs.value) == (i32::MIN)) {
        return unimplemented!();
    }
    if ((lhs.value) == (i32::MAX)) || ((rhs.value) == (i32::MAX)) {
        return unimplemented!();
    }
    return unimplemented!();
}
pub unsafe fn operator_eq_16(mut lhs: PartialOrder, mut rhs: PartialOrder) -> bool {
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct PartialOrder {
    pub value: i32,
}
impl PartialOrder {
    pub unsafe fn new(mut v: i32) -> Self {
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
pub unsafe fn operator_eq_17(
    _a0: *const EqualityComparable,
    _a1: *const EqualityComparable,
) -> bool {
    return (((*_a0).value_) == ((*_a1).value_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct EqualityComparable {
    value_: i32,
}
impl EqualityComparable {
    pub unsafe fn new(mut value: i32) -> Self {
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
pub unsafe fn operator_eq_18(
    _a0: *const ThreeWayComparable,
    _a1: *const ThreeWayComparable,
) -> bool {
    return (((*_a0).value_) == ((*_a1).value_));
}
pub unsafe fn operator_cmp_19(
    _a0: *const ThreeWayComparable,
    _a1: *const ThreeWayComparable,
) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&((*_a0).value_), &((*_a1).value_));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct ThreeWayComparable {
    value_: i32,
}
impl ThreeWayComparable {
    pub unsafe fn new(mut value: i32) -> Self {
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
    pub x: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ImplicitBool_E2 {
    pub y: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(bool, to_bool_1)]
pub struct ImplicitBool {
    pub val: bool,
}
impl ImplicitBool {
    pub unsafe fn to_bool_1(&self) -> bool {
        return self.val;
    }
}
pub unsafe fn operator_eq_20(mut lhs: ImplicitBool_E1, mut rhs: ImplicitBool_E2) -> ImplicitBool {
    return ImplicitBool {
        val: ((lhs.x) == (rhs.y)),
    };
}
pub unsafe fn tests_21() -> bool {
    (unsafe { AssertComparisonsReturnBool_9() });
    {
        let c1: Vec<i32> = Vec::from([1, 2, 3]);
        let c2: Vec<i32> = Vec::from([1, 2, 3]);
        let c3: Vec<i32> = Vec::from([3, 2, 1]);
        let c4: Vec<i32> = Vec::from([1, 2, 1]);
        assert!((unsafe { testComparisons_5(&c1, &c2, true, false,) }));
        assert!((unsafe { testComparisons_5(&c1, &c3, false, true,) }));
        assert!((unsafe { testComparisons_5(&c1, &c4, false, false,) }));
    }
    {
        let c1: Vec<i32> = std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec();
        let c2: Vec<i32> = std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec();
        assert!((unsafe { testComparisons_6(&c1, &c2, true, false,) }));
    }
    (unsafe { AssertComparisonsReturnBool_10() });
    {
        let c1: Vec<LessAndEqComp> = Vec::from([
            LessAndEqComp::new({ 1 }),
            LessAndEqComp::new({ 2 }),
            LessAndEqComp::new({ 3 }),
        ]);
        let c2: Vec<LessAndEqComp> = Vec::from([
            LessAndEqComp::new({ 1 }),
            LessAndEqComp::new({ 2 }),
            LessAndEqComp::new({ 3 }),
        ]);
        let c3: Vec<LessAndEqComp> = Vec::from([
            LessAndEqComp::new({ 3 }),
            LessAndEqComp::new({ 2 }),
            LessAndEqComp::new({ 1 }),
        ]);
        let c4: Vec<LessAndEqComp> = Vec::from([
            LessAndEqComp::new({ 1 }),
            LessAndEqComp::new({ 2 }),
            LessAndEqComp::new({ 1 }),
        ]);
        assert!((unsafe { testComparisons_7(&c1, &c2, true, false,) }));
        assert!((unsafe { testComparisons_7(&c1, &c3, false, true,) }));
        assert!((unsafe { testComparisons_7(&c1, &c4, false, false,) }));
    }
    {
        let c1: Vec<LessAndEqComp> =
            std::array::from_fn::<_, 0, _>(|_| <LessAndEqComp>::default()).to_vec();
        let c2: Vec<LessAndEqComp> =
            std::array::from_fn::<_, 0, _>(|_| <LessAndEqComp>::default()).to_vec();
        assert!((unsafe { testComparisons_8(&c1, &c2, true, false,) }));
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
    (unsafe { tests_21() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
