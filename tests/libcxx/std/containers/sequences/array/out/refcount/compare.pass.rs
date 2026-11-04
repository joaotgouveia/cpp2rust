extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_bool__bool_ {}
impl CppMove for test_macros_detail_is_same_bool__bool_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn testComparisonsComplete_1(
    t1: Ptr<Vec<i32>>,
    t2: Ptr<Vec<i32>>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= 1)
            && (!(Ptr::<i8>::from_string_literal(
                b"at most one of isEqual, isLess, and isGreater can be true"
            ))
            .is_null())
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
pub fn testComparisonsComplete_2(
    t1: Ptr<Vec<i32>>,
    t2: Ptr<Vec<i32>>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= 1)
            && (!(Ptr::<i8>::from_string_literal(
                b"at most one of isEqual, isLess, and isGreater can be true"
            ))
            .is_null())
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
pub fn testComparisonsComplete_3(
    t1: Ptr<Vec<LessAndEqComp>>,
    t2: Ptr<Vec<LessAndEqComp>>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= 1)
            && (!(Ptr::<i8>::from_string_literal(
                b"at most one of isEqual, isLess, and isGreater can be true"
            ))
            .is_null())
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
pub fn testComparisonsComplete_4(
    t1: Ptr<Vec<LessAndEqComp>>,
    t2: Ptr<Vec<LessAndEqComp>>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= 1)
            && (!(Ptr::<i8>::from_string_literal(
                b"at most one of isEqual, isLess, and isGreater can be true"
            ))
            .is_null())
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
pub fn testComparisons_5(
    t1: Ptr<Vec<i32>>,
    t2: Ptr<Vec<i32>>,
    mut isEqual: bool,
    mut isLess: bool,
) -> bool {
    assert!(
        (!((isEqual) && (isLess)))
            && (!(Ptr::<i8>::from_string_literal(b"isEqual and isLess cannot be both true"))
                .is_null())
    );
    let mut isGreater: bool = (!(isEqual)) && (!(isLess));
    return ({
        let _t1: Ptr<Vec<i32>> = (t1).clone();
        let _t2: Ptr<Vec<i32>> = (t2).clone();
        let _isEqual: bool = isEqual;
        let _isLess: bool = isLess;
        let _isGreater: bool = isGreater;
        testComparisonsComplete_1(_t1, _t2, _isEqual, _isLess, _isGreater)
    });
}
pub fn testComparisons_6(
    t1: Ptr<Vec<i32>>,
    t2: Ptr<Vec<i32>>,
    mut isEqual: bool,
    mut isLess: bool,
) -> bool {
    assert!(
        (!((isEqual) && (isLess)))
            && (!(Ptr::<i8>::from_string_literal(b"isEqual and isLess cannot be both true"))
                .is_null())
    );
    let mut isGreater: bool = (!(isEqual)) && (!(isLess));
    return ({
        let _t1: Ptr<Vec<i32>> = (t1).clone();
        let _t2: Ptr<Vec<i32>> = (t2).clone();
        let _isEqual: bool = isEqual;
        let _isLess: bool = isLess;
        let _isGreater: bool = isGreater;
        testComparisonsComplete_2(_t1, _t2, _isEqual, _isLess, _isGreater)
    });
}
pub fn testComparisons_7(
    t1: Ptr<Vec<LessAndEqComp>>,
    t2: Ptr<Vec<LessAndEqComp>>,
    mut isEqual: bool,
    mut isLess: bool,
) -> bool {
    assert!(
        (!((isEqual) && (isLess)))
            && (!(Ptr::<i8>::from_string_literal(b"isEqual and isLess cannot be both true"))
                .is_null())
    );
    let mut isGreater: bool = (!(isEqual)) && (!(isLess));
    return ({
        let _t1: Ptr<Vec<LessAndEqComp>> = (t1).clone();
        let _t2: Ptr<Vec<LessAndEqComp>> = (t2).clone();
        let _isEqual: bool = isEqual;
        let _isLess: bool = isLess;
        let _isGreater: bool = isGreater;
        testComparisonsComplete_3(_t1, _t2, _isEqual, _isLess, _isGreater)
    });
}
pub fn testComparisons_8(
    t1: Ptr<Vec<LessAndEqComp>>,
    t2: Ptr<Vec<LessAndEqComp>>,
    mut isEqual: bool,
    mut isLess: bool,
) -> bool {
    assert!(
        (!((isEqual) && (isLess)))
            && (!(Ptr::<i8>::from_string_literal(b"isEqual and isLess cannot be both true"))
                .is_null())
    );
    let mut isGreater: bool = (!(isEqual)) && (!(isLess));
    return ({
        let _t1: Ptr<Vec<LessAndEqComp>> = (t1).clone();
        let _t2: Ptr<Vec<LessAndEqComp>> = (t2).clone();
        let _isEqual: bool = isEqual;
        let _isLess: bool = isLess;
        let _isGreater: bool = isGreater;
        testComparisonsComplete_4(_t1, _t2, _isEqual, _isLess, _isGreater)
    });
}
pub fn AssertComparisonsReturnBool_9() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsReturnBool_10() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn operator_lt_11(lhs: Ptr<LessAndEqComp>, rhs: Ptr<LessAndEqComp>) -> bool {
    return ({ lhs.with(|__s| __s.value) } < { rhs.with(|__s| __s.value) });
}
pub fn operator_eq_12(lhs: Ptr<LessAndEqComp>, rhs: Ptr<LessAndEqComp>) -> bool {
    return ({ lhs.with(|__s| __s.value) } == { rhs.with(|__s| __s.value) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct LessAndEqComp {
    #[offset(0)]
    pub value: i32,
}
impl LessAndEqComp {
    pub fn new(mut v: i32) -> Self {
        Self { value: v }
    }
}
impl CppLess for LessAndEqComp {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_11(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for LessAndEqComp {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_12(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for LessAndEqComp {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_cmp_13(mut _a0: StrongOrder, mut _a1: StrongOrder) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> =
            Rc::new(RefCell::new(std::cmp::Ord::cmp(&(_a0.value), &(_a1.value))));
        if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
            return ({ (*cmp.borrow()) });
        }
    }
    return ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct StrongOrder {
    #[offset(0)]
    pub value: i32,
}
impl StrongOrder {
    pub fn new(mut v: i32) -> Self {
        Self { value: v }
    }
}
impl CppLess for StrongOrder {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_13(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Less
    }
}
impl CppEq for StrongOrder {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_13(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Equal
    }
}
impl CppMove for StrongOrder {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_cmp_14(mut _a0: WeakOrder, mut _a1: WeakOrder) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct WeakOrder {
    #[offset(0)]
    pub value: i32,
}
impl WeakOrder {
    pub fn new(mut v: i32) -> Self {
        Self { value: v }
    }
}
impl CppLess for WeakOrder {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_14(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Less
    }
}
impl CppEq for WeakOrder {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_14(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Equal
    }
}
impl CppMove for WeakOrder {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_cmp_15(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    if (lhs.value == i32::MIN) || (rhs.value == i32::MIN) {
        return unimplemented!();
    }
    if (lhs.value == i32::MAX) || (rhs.value == i32::MAX) {
        return unimplemented!();
    }
    return unimplemented!();
}
pub fn operator_eq_16(mut lhs: PartialOrder, mut rhs: PartialOrder) -> bool {
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct PartialOrder {
    #[offset(0)]
    pub value: i32,
}
impl PartialOrder {
    pub fn new(mut v: i32) -> Self {
        Self { value: v }
    }
}
impl CppLess for PartialOrder {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_15(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Less
    }
}
impl CppEq for PartialOrder {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_16(__lhs.read(), __rhs.read())
    }
}
impl CppMove for PartialOrder {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NonComparable {}
impl CppMove for NonComparable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(!(false), "!std::equality_comparable<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorNotEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorSpaceship<NonComparable>");
pub fn operator_eq_17(_a0: Ptr<EqualityComparable>, _a1: Ptr<EqualityComparable>) -> bool {
    return ({ _a0.with(|__s| __s.value_) } == { _a1.with(|__s| __s.value_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct EqualityComparable {
    #[offset(0)]
    value_: i32,
}
impl EqualityComparable {
    pub fn new(mut value: i32) -> Self {
        Self { value_: value }
    }
}
impl CppEq for EqualityComparable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_17(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for EqualityComparable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(true, "std::equality_comparable<EqualityComparable>");
const _: () = assert!(true, "HasOperatorEqual<EqualityComparable>");
const _: () = assert!(true, "HasOperatorNotEqual<EqualityComparable>");
pub fn operator_eq_18(_a0: Ptr<ThreeWayComparable>, _a1: Ptr<ThreeWayComparable>) -> bool {
    return ({ _a0.with(|__s| __s.value_) } == { _a1.with(|__s| __s.value_) });
}
pub fn operator_cmp_19(
    _a0: Ptr<ThreeWayComparable>,
    _a1: Ptr<ThreeWayComparable>,
) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.value_)),
            &(_a1.with(|__s| __s.value_)),
        )));
        if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
            return ({ (*cmp.borrow()) });
        }
    }
    return ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct ThreeWayComparable {
    #[offset(0)]
    value_: i32,
}
impl ThreeWayComparable {
    pub fn new(mut value: i32) -> Self {
        Self { value_: value }
    }
}
impl CppLess for ThreeWayComparable {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_19(__lhs.clone(), __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for ThreeWayComparable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_18(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for ThreeWayComparable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(true, "std::equality_comparable<ThreeWayComparable>");
const _: () = assert!(true, "std::three_way_comparable<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorGreaterThan<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorGreaterThanEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorLessThan<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorLessThanEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorNotEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorSpaceship<ThreeWayComparable>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct ImplicitBool_E1 {
    #[offset(0)]
    pub x: i32,
}
impl CppMove for ImplicitBool_E1 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct ImplicitBool_E2 {
    #[offset(0)]
    pub y: i32,
}
impl CppMove for ImplicitBool_E2 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(bool, to_bool_1)]
#[byte_size(1)]
pub struct ImplicitBool {
    #[offset(0)]
    pub val: bool,
}
impl CppMove for ImplicitBool {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_20(mut lhs: ImplicitBool_E1, mut rhs: ImplicitBool_E2) -> ImplicitBool {
    return ImplicitBool {
        val: (lhs.x == rhs.y),
    };
}
pub fn tests_21() -> bool {
    ({ AssertComparisonsReturnBool_9() });
    {
        let c1: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3])));
        let c2: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3])));
        let c3: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([3, 2, 1])));
        let c4: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 1])));
        assert!(({ testComparisons_5(c1.as_pointer(), c2.as_pointer(), true, false,) }));
        assert!(({ testComparisons_5(c1.as_pointer(), c3.as_pointer(), false, true,) }));
        assert!(({ testComparisons_5(c1.as_pointer(), c4.as_pointer(), false, false,) }));
    }
    {
        let c1: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
        ));
        let c2: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
        ));
        assert!(({ testComparisons_6(c1.as_pointer(), c2.as_pointer(), true, false,) }));
    }
    ({ AssertComparisonsReturnBool_10() });
    {
        let c1: Value<Vec<LessAndEqComp>> = Rc::new(RefCell::new(Vec::from([
            LessAndEqComp::new({ 1 }),
            LessAndEqComp::new({ 2 }),
            LessAndEqComp::new({ 3 }),
        ])));
        let c2: Value<Vec<LessAndEqComp>> = Rc::new(RefCell::new(Vec::from([
            LessAndEqComp::new({ 1 }),
            LessAndEqComp::new({ 2 }),
            LessAndEqComp::new({ 3 }),
        ])));
        let c3: Value<Vec<LessAndEqComp>> = Rc::new(RefCell::new(Vec::from([
            LessAndEqComp::new({ 3 }),
            LessAndEqComp::new({ 2 }),
            LessAndEqComp::new({ 1 }),
        ])));
        let c4: Value<Vec<LessAndEqComp>> = Rc::new(RefCell::new(Vec::from([
            LessAndEqComp::new({ 1 }),
            LessAndEqComp::new({ 2 }),
            LessAndEqComp::new({ 1 }),
        ])));
        assert!(({ testComparisons_7(c1.as_pointer(), c2.as_pointer(), true, false,) }));
        assert!(({ testComparisons_7(c1.as_pointer(), c3.as_pointer(), false, true,) }));
        assert!(({ testComparisons_7(c1.as_pointer(), c4.as_pointer(), false, false,) }));
    }
    {
        let c1: Value<Vec<LessAndEqComp>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| <LessAndEqComp>::default()).to_vec(),
        ));
        let c2: Value<Vec<LessAndEqComp>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| <LessAndEqComp>::default()).to_vec(),
        ));
        assert!(({ testComparisons_8(c1.as_pointer(), c2.as_pointer(), true, false,) }));
    }
    return true;
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ tests_21() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub trait ImplicitBoolImpl {
    fn to_bool_1(&self) -> bool;
}
impl ImplicitBoolImpl for Ptr<ImplicitBool> {
    fn to_bool_1(&self) -> bool {
        return (*self).with(|__s| __s.val);
    }
}
pub fn __cpp2rust_init_globals() {}
