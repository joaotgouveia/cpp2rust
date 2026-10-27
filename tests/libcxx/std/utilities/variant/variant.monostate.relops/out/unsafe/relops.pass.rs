extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn testComparisonsComplete_0(
    t1: *const (),
    t2: *const (),
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
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
pub unsafe fn testComparisons_1(
    t1: *const (),
    t2: *const (),
    mut isEqual: (),
    mut isLess: (),
) -> () {
    unimplemented!();
    let mut isGreater: () = (!(isEqual)) && (!(isLess));
    return (unsafe {
        let _t1: *const () = t1;
        let _t2: *const () = t2;
        let _isEqual: () = isEqual;
        let _isLess: () = isLess;
        let _isGreater: () = isGreater;
        testComparisonsComplete_0(_t1, _t2, _isEqual, _isLess, _isGreater)
    });
}
pub unsafe fn AssertComparisonsAreNoexcept_2() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn AssertOrderAreNoexcept_3() {
    (unsafe { AssertComparisonsAreNoexcept_2() });
    const _: () = assert!(true, "");;
}
pub unsafe fn testOrder_4(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_0(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn operator_lt_5(lhs: *const LessAndEqComp, rhs: *const LessAndEqComp) -> () {
    return (((*lhs).value) < ((*rhs).value));
}
pub unsafe fn operator_eq_6(lhs: *const LessAndEqComp, rhs: *const LessAndEqComp) -> () {
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
            if operator_lt_5(self as *const LessAndEqComp, other as *const LessAndEqComp) {
                std::cmp::Ordering::Less
            } else if operator_lt_5(other as *const LessAndEqComp, self as *const LessAndEqComp) {
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
        unsafe { operator_eq_6(self as *const LessAndEqComp, other as *const LessAndEqComp) }
    }
}
impl std::cmp::Eq for LessAndEqComp {}
pub unsafe fn operator_cmp_7(mut _a0: StrongOrder, mut _a1: StrongOrder) -> () {
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
        unsafe { operator_cmp_7(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for StrongOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for StrongOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_7(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for StrongOrder {}
pub unsafe fn operator_cmp_8(mut _a0: WeakOrder, mut _a1: WeakOrder) -> () {
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
        unsafe { operator_cmp_8(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for WeakOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for WeakOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_8(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for WeakOrder {}
pub unsafe fn operator_cmp_9(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    if ((lhs.value) == (unimplemented!())) || ((rhs.value) == (unimplemented!())) {
        return unimplemented!();
    }
    if ((lhs.value) == (unimplemented!())) || ((rhs.value) == (unimplemented!())) {
        return unimplemented!();
    }
    return unimplemented!();
}
pub unsafe fn operator_eq_10(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
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
        unsafe { operator_cmp_9(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for PartialOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PartialOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_10(self.clone(), other.clone()) }
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
pub unsafe fn operator_eq_11(_a0: *const EqualityComparable, _a1: *const EqualityComparable) -> () {
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
            operator_eq_11(
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
pub unsafe fn operator_eq_12(_a0: *const ThreeWayComparable, _a1: *const ThreeWayComparable) -> () {
    return (((*_a0).value_) == ((*_a1).value_));
}
pub unsafe fn operator_cmp_13(
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
            operator_cmp_13(
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
            operator_eq_12(
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
pub unsafe fn operator_eq_14(mut lhs: ImplicitBool_E1, mut rhs: ImplicitBool_E2) -> ImplicitBool {
    return ImplicitBool {
        val: ((lhs.x) == (rhs.y)),
    };
}
pub unsafe fn test_15() -> () {
    let m1: () = unimplemented!();
    let m2: () = unimplemented!();
    unimplemented!();
    (unsafe { AssertComparisonsAreNoexcept_2() });
    unimplemented!();
    (unsafe { AssertOrderAreNoexcept_3() });
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
    (unsafe { test_15() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
