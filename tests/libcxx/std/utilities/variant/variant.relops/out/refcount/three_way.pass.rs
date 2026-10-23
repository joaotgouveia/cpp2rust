extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_bool__bool_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_partial_ordering__std_partial_ordering_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_strong_ordering__std_strong_ordering_ {}
pub fn testComparisonsComplete_3(
    t1: Ptr<()>,
    t2: Ptr<()>,
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
pub fn testComparisonsComplete_4(
    t1: Ptr<()>,
    t2: Ptr<()>,
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
pub fn AssertComparisonsReturnBool_5() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsReturnBool_6() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertOrderReturn_7() {
    ({ AssertComparisonsReturnBool_5() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderReturn_8() {
    ({ AssertComparisonsReturnBool_6() });
    const _: () = assert!(true, "");;
}
pub fn testOrder_9(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_3(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_10(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_4(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn operator_lt_11(lhs: Ptr<LessAndEqComp>, rhs: Ptr<LessAndEqComp>) -> () {
    return ({ lhs.with(|__s| __s.value) } < { rhs.with(|__s| __s.value) });
}
pub fn operator_eq_12(lhs: Ptr<LessAndEqComp>, rhs: Ptr<LessAndEqComp>) -> () {
    return ({ lhs.with(|__s| __s.value) } == { rhs.with(|__s| __s.value) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct LessAndEqComp {
    #[offset(0)]
    pub value: (),
}
impl LessAndEqComp {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
}
impl std::cmp::Ord for LessAndEqComp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_11(
                Rc::new(RefCell::new(LessAndEqComp {
                    value: self.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(LessAndEqComp {
                    value: other.value.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_11(
                Rc::new(RefCell::new(LessAndEqComp {
                    value: other.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(LessAndEqComp {
                    value: self.value.clone(),
                }))
                .as_pointer(),
            ) {
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
        {
            operator_eq_12(
                Rc::new(RefCell::new(LessAndEqComp {
                    value: self.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(LessAndEqComp {
                    value: other.value.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for LessAndEqComp {}
pub fn operator_cmp_13(mut _a0: StrongOrder, mut _a1: StrongOrder) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(&(_a0.value), &(_a1.value))));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct StrongOrder {
    #[offset(0)]
    pub value: (),
}
impl StrongOrder {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
}
impl std::cmp::Ord for StrongOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        { operator_cmp_13(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for StrongOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for StrongOrder {
    fn eq(&self, other: &Self) -> bool {
        { operator_cmp_13(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for StrongOrder {}
pub fn operator_cmp_14(mut _a0: WeakOrder, mut _a1: WeakOrder) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct WeakOrder {
    #[offset(0)]
    pub value: (),
}
impl WeakOrder {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
}
impl std::cmp::Ord for WeakOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        { operator_cmp_14(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for WeakOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for WeakOrder {
    fn eq(&self, other: &Self) -> bool {
        { operator_cmp_14(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for WeakOrder {}
pub fn operator_cmp_15(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    if (lhs.value == unimplemented!()) || (rhs.value == unimplemented!()) {
        return unimplemented!();
    }
    if (lhs.value == unimplemented!()) || (rhs.value == unimplemented!()) {
        return unimplemented!();
    }
    return unimplemented!();
}
pub fn operator_eq_16(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct PartialOrder {
    #[offset(0)]
    pub value: (),
}
impl PartialOrder {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
}
impl std::cmp::Ord for PartialOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        { operator_cmp_15(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for PartialOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PartialOrder {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_16(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for PartialOrder {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NonComparable {}
const _: () = assert!(!(false), "!std::equality_comparable<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorNotEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorSpaceship<NonComparable>");
pub fn operator_eq_17(_a0: Ptr<EqualityComparable>, _a1: Ptr<EqualityComparable>) -> () {
    return ({ _a0.with(|__s| __s.value_) } == { _a1.with(|__s| __s.value_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct EqualityComparable {
    #[offset(0)]
    value_: (),
}
impl EqualityComparable {
    pub fn new(mut value: ()) -> Self {
        Self { value_: value }
    }
}
impl std::cmp::PartialEq for EqualityComparable {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_17(
                Rc::new(RefCell::new(EqualityComparable {
                    value_: self.value_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(EqualityComparable {
                    value_: other.value_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for EqualityComparable {}
const _: () = assert!(true, "std::equality_comparable<EqualityComparable>");
const _: () = assert!(true, "HasOperatorEqual<EqualityComparable>");
const _: () = assert!(true, "HasOperatorNotEqual<EqualityComparable>");
pub fn operator_eq_18(_a0: Ptr<ThreeWayComparable>, _a1: Ptr<ThreeWayComparable>) -> () {
    return ({ _a0.with(|__s| __s.value_) } == { _a1.with(|__s| __s.value_) });
}
pub fn operator_cmp_19(_a0: Ptr<ThreeWayComparable>, _a1: Ptr<ThreeWayComparable>) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.value_)),
            &(_a1.with(|__s| __s.value_)),
        )));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct ThreeWayComparable {
    #[offset(0)]
    value_: (),
}
impl ThreeWayComparable {
    pub fn new(mut value: ()) -> Self {
        Self { value_: value }
    }
}
impl std::cmp::Ord for ThreeWayComparable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_19(
                Rc::new(RefCell::new(ThreeWayComparable {
                    value_: self.value_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(ThreeWayComparable {
                    value_: other.value_.clone(),
                }))
                .as_pointer(),
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
        {
            operator_eq_18(
                Rc::new(RefCell::new(ThreeWayComparable {
                    value_: self.value_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(ThreeWayComparable {
                    value_: other.value_.clone(),
                }))
                .as_pointer(),
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct ImplicitBool_E1 {
    #[offset(0)]
    pub x: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct ImplicitBool_E2 {
    #[offset(0)]
    pub y: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ImplicitBool {
    #[offset(0)]
    pub val: (),
}
pub fn operator_eq_20(mut lhs: ImplicitBool_E1, mut rhs: ImplicitBool_E2) -> ImplicitBool {
    return ImplicitBool {
        val: (lhs.x == rhs.y),
    };
}
pub fn test_with_types_21() -> () {
    ({ AssertOrderReturn_7() });
    {
        let v1: Value<()> = Rc::new(RefCell::new({
            let __tmp_0: Value<()> = Rc::new(RefCell::new(1_()));
            std_variant_int__double_::new_1(
                { (*in_place_index_22.with(Value::clone).borrow()).clone() },
                { __tmp_0.as_pointer() },
            )
        }));
        let v2: Value<()> = Rc::new(RefCell::new({
            let __tmp_1: Value<()> = Rc::new(RefCell::new(1_()));
            std_variant_int__double_::new_1(
                { (*in_place_index_22.with(Value::clone).borrow()).clone() },
                { __tmp_1.as_pointer() },
            )
        }));
        unimplemented!();
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new({
            let __tmp_2: Value<()> = Rc::new(RefCell::new(0_()));
            std_variant_int__double_::new_1(
                { (*in_place_index_22.with(Value::clone).borrow()).clone() },
                { __tmp_2.as_pointer() },
            )
        }));
        let v2: Value<()> = Rc::new(RefCell::new({
            let __tmp_3: Value<()> = Rc::new(RefCell::new(1_()));
            std_variant_int__double_::new_1(
                { (*in_place_index_22.with(Value::clone).borrow()).clone() },
                { __tmp_3.as_pointer() },
            )
        }));
        unimplemented!();
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new({
            let __tmp_4: Value<()> = Rc::new(RefCell::new(1_()));
            std_variant_int__double_::new_1(
                { (*in_place_index_22.with(Value::clone).borrow()).clone() },
                { __tmp_4.as_pointer() },
            )
        }));
        let v2: Value<()> = Rc::new(RefCell::new({
            let __tmp_5: Value<()> = Rc::new(RefCell::new(0_()));
            std_variant_int__double_::new_1(
                { (*in_place_index_22.with(Value::clone).borrow()).clone() },
                { __tmp_5.as_pointer() },
            )
        }));
        unimplemented!();
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new({
            let __tmp_6: Value<()> = Rc::new(RefCell::new(0_()));
            std_variant_int__double_::new_1(
                { (*in_place_index_22.with(Value::clone).borrow()).clone() },
                { __tmp_6.as_pointer() },
            )
        }));
        let v2: Value<()> = Rc::new(RefCell::new({
            let __tmp_7: Value<()> = Rc::new(RefCell::new(0_()));
            std_variant_int__double_::new_2(
                { (*in_place_index_23.with(Value::clone).borrow()).clone() },
                { __tmp_7.as_pointer() },
            )
        }));
        unimplemented!();
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new({
            let __tmp_8: Value<()> = Rc::new(RefCell::new(0_()));
            std_variant_int__double_::new_2(
                { (*in_place_index_23.with(Value::clone).borrow()).clone() },
                { __tmp_8.as_pointer() },
            )
        }));
        let v2: Value<()> = Rc::new(RefCell::new({
            let __tmp_9: Value<()> = Rc::new(RefCell::new(0_()));
            std_variant_int__double_::new_1(
                { (*in_place_index_22.with(Value::clone).borrow()).clone() },
                { __tmp_9.as_pointer() },
            )
        }));
        unimplemented!();
    }
    return true;
}
pub fn test_with_types_24() -> () {
    ({ AssertOrderReturn_8() });
    {
        let v1: Value<()> = Rc::new(RefCell::new({
            let __tmp_10: Value<()> = Rc::new(RefCell::new(1_()));
            std_variant_int__long_::new_1(
                { (*in_place_index_22.with(Value::clone).borrow()).clone() },
                { __tmp_10.as_pointer() },
            )
        }));
        let v2: Value<()> = Rc::new(RefCell::new({
            let __tmp_11: Value<()> = Rc::new(RefCell::new(1_()));
            std_variant_int__long_::new_1(
                { (*in_place_index_22.with(Value::clone).borrow()).clone() },
                { __tmp_11.as_pointer() },
            )
        }));
        unimplemented!();
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new({
            let __tmp_12: Value<()> = Rc::new(RefCell::new(0_()));
            std_variant_int__long_::new_1(
                { (*in_place_index_22.with(Value::clone).borrow()).clone() },
                { __tmp_12.as_pointer() },
            )
        }));
        let v2: Value<()> = Rc::new(RefCell::new({
            let __tmp_13: Value<()> = Rc::new(RefCell::new(1_()));
            std_variant_int__long_::new_1(
                { (*in_place_index_22.with(Value::clone).borrow()).clone() },
                { __tmp_13.as_pointer() },
            )
        }));
        unimplemented!();
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new({
            let __tmp_14: Value<()> = Rc::new(RefCell::new(1_()));
            std_variant_int__long_::new_1(
                { (*in_place_index_22.with(Value::clone).borrow()).clone() },
                { __tmp_14.as_pointer() },
            )
        }));
        let v2: Value<()> = Rc::new(RefCell::new({
            let __tmp_15: Value<()> = Rc::new(RefCell::new(0_()));
            std_variant_int__long_::new_1(
                { (*in_place_index_22.with(Value::clone).borrow()).clone() },
                { __tmp_15.as_pointer() },
            )
        }));
        unimplemented!();
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new({
            let __tmp_16: Value<()> = Rc::new(RefCell::new(0_()));
            std_variant_int__long_::new_1(
                { (*in_place_index_22.with(Value::clone).borrow()).clone() },
                { __tmp_16.as_pointer() },
            )
        }));
        let v2: Value<()> = Rc::new(RefCell::new({
            let __tmp_17: Value<()> = Rc::new(RefCell::new(0_()));
            std_variant_int__long_::new_2(
                { (*in_place_index_23.with(Value::clone).borrow()).clone() },
                { __tmp_17.as_pointer() },
            )
        }));
        unimplemented!();
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new({
            let __tmp_18: Value<()> = Rc::new(RefCell::new(0_()));
            std_variant_int__long_::new_2(
                { (*in_place_index_23.with(Value::clone).borrow()).clone() },
                { __tmp_18.as_pointer() },
            )
        }));
        let v2: Value<()> = Rc::new(RefCell::new({
            let __tmp_19: Value<()> = Rc::new(RefCell::new(0_()));
            std_variant_int__long_::new_1(
                { (*in_place_index_22.with(Value::clone).borrow()).clone() },
                { __tmp_19.as_pointer() },
            )
        }));
        unimplemented!();
    }
    return true;
}
pub fn test_three_way_25() -> () {
    unimplemented!();
    unimplemented!();
    {
        let nan: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let v1: Value<()> = Rc::new(RefCell::new({
                let __tmp_20: Value<()> = Rc::new(RefCell::new(1_()));
                std_variant_int__double_::new_3(
                    { (*in_place_type_26.with(Value::clone).borrow()).clone() },
                    { __tmp_20.as_pointer() },
                )
            }));
            let v2: Value<()> = Rc::new(RefCell::new(std_variant_int__double_::new_4(
                { (*in_place_type_27.with(Value::clone).borrow()).clone() },
                { nan.as_pointer() },
            )));
            unimplemented!();
        }
        {
            let v1: Value<()> = Rc::new(RefCell::new(std_variant_int__double_::new_4(
                { (*in_place_type_27.with(Value::clone).borrow()).clone() },
                { nan.as_pointer() },
            )));
            let v2: Value<()> = Rc::new(RefCell::new({
                let __tmp_21: Value<()> = Rc::new(RefCell::new(2_()));
                std_variant_int__double_::new_3(
                    { (*in_place_type_26.with(Value::clone).borrow()).clone() },
                    { __tmp_21.as_pointer() },
                )
            }));
            unimplemented!();
        }
        {
            let v1: Value<()> = Rc::new(RefCell::new(std_variant_int__double_::new_4(
                { (*in_place_type_27.with(Value::clone).borrow()).clone() },
                { nan.as_pointer() },
            )));
            let v2: Value<()> = Rc::new(RefCell::new(std_variant_int__double_::new_4(
                { (*in_place_type_27.with(Value::clone).borrow()).clone() },
                { nan.as_pointer() },
            )));
            unimplemented!();
        }
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct HasSimpleOrdering {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct HasOnlySpaceship {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_three_way_25() });
    const _: () = assert!(true, "test_three_way()");;
    return 0_();
}
pub trait HasFullOrderingImpl {
    fn operator_eq(&self, _a0: Ptr<HasFullOrdering>) -> () {
        unimplemented!()
    }
    fn operator_cmp(&self, _a0: Ptr<HasFullOrdering>) -> () {
        unimplemented!()
    }
}
impl HasFullOrderingImpl for Ptr<HasFullOrdering> {}
pub trait HasOnlySpaceshipImpl {
    fn operator_cmp(&self, _a0: Ptr<HasOnlySpaceship>) -> () {
        unimplemented!()
    }
}
impl HasOnlySpaceshipImpl for Ptr<HasOnlySpaceship> {}
pub trait HasSimpleOrderingImpl {
    fn operator_eq(&self, _a0: Ptr<HasSimpleOrdering>) -> () {
        unimplemented!()
    }
    fn operator_lt(&self, _a0: Ptr<HasSimpleOrdering>) -> () {
        unimplemented!()
    }
}
impl HasSimpleOrderingImpl for Ptr<HasSimpleOrdering> {}
pub trait ImplicitBoolImpl {
    fn to__1(&self) -> ();
}
impl ImplicitBoolImpl for Ptr<ImplicitBool> {
    fn to__1(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
}
pub fn __cpp2rust_init_globals() {}
