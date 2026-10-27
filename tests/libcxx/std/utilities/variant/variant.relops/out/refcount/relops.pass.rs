extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_lt_0(lhs: Ptr<LessAndEqComp>, rhs: Ptr<LessAndEqComp>) -> () {
    return ({ lhs.with(|__s| __s.value) } < { rhs.with(|__s| __s.value) });
}
pub fn operator_eq_1(lhs: Ptr<LessAndEqComp>, rhs: Ptr<LessAndEqComp>) -> () {
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
            if operator_lt_0(
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
            } else if operator_lt_0(
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
            operator_eq_1(
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
pub fn operator_cmp_2(mut _a0: StrongOrder, mut _a1: StrongOrder) -> () {
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
        { operator_cmp_2(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for StrongOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for StrongOrder {
    fn eq(&self, other: &Self) -> bool {
        { operator_cmp_2(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for StrongOrder {}
pub fn operator_cmp_3(mut _a0: WeakOrder, mut _a1: WeakOrder) -> () {
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
        { operator_cmp_3(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for WeakOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for WeakOrder {
    fn eq(&self, other: &Self) -> bool {
        { operator_cmp_3(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for WeakOrder {}
pub fn operator_cmp_4(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    if (lhs.value == unimplemented!()) || (rhs.value == unimplemented!()) {
        return unimplemented!();
    }
    if (lhs.value == unimplemented!()) || (rhs.value == unimplemented!()) {
        return unimplemented!();
    }
    return unimplemented!();
}
pub fn operator_eq_5(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
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
        { operator_cmp_4(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for PartialOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PartialOrder {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_5(self.clone(), other.clone()) }
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
pub fn operator_eq_6(_a0: Ptr<EqualityComparable>, _a1: Ptr<EqualityComparable>) -> () {
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
            operator_eq_6(
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
pub fn operator_eq_7(_a0: Ptr<ThreeWayComparable>, _a1: Ptr<ThreeWayComparable>) -> () {
    return ({ _a0.with(|__s| __s.value_) } == { _a1.with(|__s| __s.value_) });
}
pub fn operator_cmp_8(_a0: Ptr<ThreeWayComparable>, _a1: Ptr<ThreeWayComparable>) -> () {
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
            operator_cmp_8(
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
            operator_eq_7(
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
pub fn operator_eq_9(mut lhs: ImplicitBool_E1, mut rhs: ImplicitBool_E2) -> ImplicitBool {
    return ImplicitBool {
        val: (lhs.x == rhs.y),
    };
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MyBool {
    #[offset(0)]
    pub value: (),
}
impl MyBool {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct ComparesToMyBool {
    #[offset(0)]
    pub value: (),
}
impl std::cmp::Ord for ComparesToMyBool {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if {
                let __cmp = &operator_lt_10(
                    Rc::new(RefCell::new(ComparesToMyBool {
                        value: self.value.clone(),
                    }))
                    .as_pointer(),
                    Rc::new(RefCell::new(ComparesToMyBool {
                        value: other.value.clone(),
                    }))
                    .as_pointer(),
                );
                MyBoolImpl::to_(
                    &Rc::new(RefCell::new(MyBool {
                        value: __cmp.value.clone(),
                    }))
                    .as_pointer(),
                )
            } {
                std::cmp::Ordering::Less
            } else if {
                let __cmp = &operator_lt_10(
                    Rc::new(RefCell::new(ComparesToMyBool {
                        value: other.value.clone(),
                    }))
                    .as_pointer(),
                    Rc::new(RefCell::new(ComparesToMyBool {
                        value: self.value.clone(),
                    }))
                    .as_pointer(),
                );
                MyBoolImpl::to_(
                    &Rc::new(RefCell::new(MyBool {
                        value: __cmp.value.clone(),
                    }))
                    .as_pointer(),
                )
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
        {
            {
                let __cmp = &operator_eq_11(
                    Rc::new(RefCell::new(ComparesToMyBool {
                        value: self.value.clone(),
                    }))
                    .as_pointer(),
                    Rc::new(RefCell::new(ComparesToMyBool {
                        value: other.value.clone(),
                    }))
                    .as_pointer(),
                );
                MyBoolImpl::to_(
                    &Rc::new(RefCell::new(MyBool {
                        value: __cmp.value.clone(),
                    }))
                    .as_pointer(),
                )
            }
        }
    }
}
impl std::cmp::Eq for ComparesToMyBool {}
impl Default for ComparesToMyBool {
    fn default() -> Self {
        { ComparesToMyBool { value: 0_() } }
    }
}
pub fn operator_eq_11(LHS: Ptr<ComparesToMyBool>, RHS: Ptr<ComparesToMyBool>) -> MyBool {
    return MyBool::new({ ({ LHS.with(|__s| __s.value) } == { RHS.with(|__s| __s.value) }) });
}
pub fn operator_ne_12(LHS: Ptr<ComparesToMyBool>, RHS: Ptr<ComparesToMyBool>) -> MyBool {
    return MyBool::new({ ({ LHS.with(|__s| __s.value) } != { RHS.with(|__s| __s.value) }) });
}
pub fn operator_lt_10(LHS: Ptr<ComparesToMyBool>, RHS: Ptr<ComparesToMyBool>) -> MyBool {
    return MyBool::new({ ({ LHS.with(|__s| __s.value) } < { RHS.with(|__s| __s.value) }) });
}
pub fn operator_le_13(LHS: Ptr<ComparesToMyBool>, RHS: Ptr<ComparesToMyBool>) -> MyBool {
    return MyBool::new({ ({ LHS.with(|__s| __s.value) } <= { RHS.with(|__s| __s.value) }) });
}
pub fn operator_gt_14(LHS: Ptr<ComparesToMyBool>, RHS: Ptr<ComparesToMyBool>) -> MyBool {
    return MyBool::new({ ({ LHS.with(|__s| __s.value) } > { RHS.with(|__s| __s.value) }) });
}
pub fn operator_ge_15(LHS: Ptr<ComparesToMyBool>, RHS: Ptr<ComparesToMyBool>) -> MyBool {
    return MyBool::new({ ({ LHS.with(|__s| __s.value) } >= { RHS.with(|__s| __s.value) }) });
}
pub fn test_equality_basic_16() {
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
}
pub fn test_equality_basic_17() {
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
}
pub fn test_equality_basic_18() {
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
}
pub fn test_equality_basic_19() {
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "!(v1 == v2)");;
        const _: () = assert!(true, "!(v2 == v1)");;
        const _: () = assert!(true, "v1 != v2");;
        const _: () = assert!(true, "v2 != v1");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "v1 == v2");;
        const _: () = assert!(true, "v2 == v1");;
        const _: () = assert!(true, "!(v1 != v2)");;
        const _: () = assert!(true, "!(v2 != v1)");;
    }
}
pub fn test_equality_20() {
    ({ test_equality_basic_16() });
    ({ test_equality_basic_17() });
    ({ test_equality_basic_18() });
    ({ test_equality_basic_19() });
}
pub fn test_less_21(l: Ptr<()>, r: Ptr<()>, mut expect_less: (), mut expect_greater: ()) -> () {
    const _: () = assert!(true, "std::is_same_v<decltype(l < r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l <= r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l > r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l >= r), bool>");;
    return ((({ (unimplemented!()) } == { expect_less })
        && ({ !(unimplemented!()) } == { expect_less }))
        && ({ (unimplemented!()) } == { expect_greater }))
        && ({ !(unimplemented!()) } == { expect_greater });
}
pub fn test_less_22(l: Ptr<()>, r: Ptr<()>, mut expect_less: (), mut expect_greater: ()) -> () {
    const _: () = assert!(true, "std::is_same_v<decltype(l < r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l <= r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l > r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l >= r), bool>");;
    return ((({ (unimplemented!()) } == { expect_less })
        && ({ !(unimplemented!()) } == { expect_less }))
        && ({ (unimplemented!()) } == { expect_greater }))
        && ({ !(unimplemented!()) } == { expect_greater });
}
pub fn test_less_23(l: Ptr<()>, r: Ptr<()>, mut expect_less: (), mut expect_greater: ()) -> () {
    const _: () = assert!(true, "std::is_same_v<decltype(l < r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l <= r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l > r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l >= r), bool>");;
    return ((({ (unimplemented!()) } == { expect_less })
        && ({ !(unimplemented!()) } == { expect_less }))
        && ({ (unimplemented!()) } == { expect_greater }))
        && ({ !(unimplemented!()) } == { expect_greater });
}
pub fn test_less_24(l: Ptr<()>, r: Ptr<()>, mut expect_less: (), mut expect_greater: ()) -> () {
    const _: () = assert!(true, "std::is_same_v<decltype(l < r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l <= r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l > r), bool>");;
    const _: () = assert!(true, "std::is_same_v<decltype(l >= r), bool>");;
    return ((({ (unimplemented!()) } == { expect_less })
        && ({ !(unimplemented!()) } == { expect_less }))
        && ({ (unimplemented!()) } == { expect_greater }))
        && ({ !(unimplemented!()) } == { expect_greater });
}
pub fn test_relational_basic_25() {
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "test_less(v1, v2, false, false)");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
}
pub fn test_relational_basic_26() {
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "test_less(v1, v2, false, false)");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
}
pub fn test_relational_basic_27() {
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "test_less(v1, v2, false, false)");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
}
pub fn test_relational_basic_28() {
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "test_less(v1, v2, false, false)");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "test_less(v1, v2, true, false)");;
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "test_less(v1, v2, false, true)");;
    }
}
pub fn test_relational_29() {
    ({ test_relational_basic_25() });
    ({ test_relational_basic_26() });
    ({ test_relational_basic_27() });
    ({ test_relational_basic_28() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A1 {}
impl std::cmp::Ord for A1 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if A1Impl::operator_lt(
                &Rc::new(RefCell::new(A1 {})).as_pointer(),
                Rc::new(RefCell::new(A1 {})).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if A1Impl::operator_lt(
                &Rc::new(RefCell::new(A1 {})).as_pointer(),
                Rc::new(RefCell::new(A1 {})).as_pointer(),
            ) {
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
        {
            A1Impl::operator_eq(
                &Rc::new(RefCell::new(A1 {})).as_pointer(),
                Rc::new(RefCell::new(A1 {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for A1 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A2 {}
impl std::cmp::Ord for A2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if A2Impl::operator_lt(
                &Rc::new(RefCell::new(A2 {})).as_pointer(),
                Rc::new(RefCell::new(A2 {})).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if A2Impl::operator_lt(
                &Rc::new(RefCell::new(A2 {})).as_pointer(),
                Rc::new(RefCell::new(A2 {})).as_pointer(),
            ) {
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
        {
            A2Impl::operator_eq(
                &Rc::new(RefCell::new(A2 {})).as_pointer(),
                Rc::new(RefCell::new(A2 {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for A2 {}
pub fn operator_eq_30(lhs: Ptr<Array>, rhs: Ptr<Array>) -> () {
    return unimplemented!();
}
pub fn operator_ne_31(lhs: Ptr<Array>, rhs: Ptr<Array>) -> () {
    return unimplemented!();
}
pub fn operator_lt_32(lhs: Ptr<Array>, rhs: Ptr<Array>) -> () {
    return unimplemented!();
}
pub fn operator_gt_33(lhs: Ptr<Array>, rhs: Ptr<Array>) -> () {
    return unimplemented!();
}
pub fn operator_le_34(lhs: Ptr<Array>, rhs: Ptr<Array>) -> () {
    return unimplemented!();
}
pub fn operator_ge_35(lhs: Ptr<Array>, rhs: Ptr<Array>) -> () {
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Array {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
}
impl std::cmp::Ord for Array {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_32(
                Rc::new(RefCell::new(Array {
                    ptr: self.ptr.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Array {
                    ptr: other.ptr.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_32(
                Rc::new(RefCell::new(Array {
                    ptr: other.ptr.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Array {
                    ptr: self.ptr.clone(),
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
impl std::cmp::PartialOrd for Array {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Array {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_30(
                Rc::new(RefCell::new(Array {
                    ptr: self.ptr.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Array {
                    ptr: other.ptr.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Array {}
pub fn test_recursive_36() {
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_equality_20() });
    ({ test_relational_29() });
    return 0_();
}
pub trait A1Impl {
    fn operator_eq(&self, _a0: Ptr<A1>) -> ();
    fn operator_ne(&self, _a0: Ptr<A1>) -> ();
    fn operator_lt(&self, _a0: Ptr<A1>) -> ();
    fn operator_gt(&self, _a0: Ptr<A1>) -> ();
    fn operator_le(&self, _a0: Ptr<A1>) -> ();
    fn operator_ge(&self, _a0: Ptr<A1>) -> ();
}
impl A1Impl for Ptr<A1> {
    fn operator_eq(&self, _a0: Ptr<A1>) -> () {
        return true;
    }
    fn operator_ne(&self, _a0: Ptr<A1>) -> () {
        return true;
    }
    fn operator_lt(&self, _a0: Ptr<A1>) -> () {
        return true;
    }
    fn operator_gt(&self, _a0: Ptr<A1>) -> () {
        return true;
    }
    fn operator_le(&self, _a0: Ptr<A1>) -> () {
        return true;
    }
    fn operator_ge(&self, _a0: Ptr<A1>) -> () {
        return true;
    }
}
pub trait A2Impl {
    fn operator_eq(&self, _a0: Ptr<A2>) -> ();
    fn operator_ne(&self, _a0: Ptr<A2>) -> ();
    fn operator_lt(&self, _a0: Ptr<A2>) -> ();
    fn operator_gt(&self, _a0: Ptr<A2>) -> ();
    fn operator_le(&self, _a0: Ptr<A2>) -> ();
    fn operator_ge(&self, _a0: Ptr<A2>) -> ();
}
impl A2Impl for Ptr<A2> {
    fn operator_eq(&self, _a0: Ptr<A2>) -> () {
        return true;
    }
    fn operator_ne(&self, _a0: Ptr<A2>) -> () {
        return true;
    }
    fn operator_lt(&self, _a0: Ptr<A2>) -> () {
        return true;
    }
    fn operator_gt(&self, _a0: Ptr<A2>) -> () {
        return true;
    }
    fn operator_le(&self, _a0: Ptr<A2>) -> () {
        return true;
    }
    fn operator_ge(&self, _a0: Ptr<A2>) -> () {
        return true;
    }
}
pub trait ImplicitBoolImpl {
    fn to__1(&self) -> ();
}
impl ImplicitBoolImpl for Ptr<ImplicitBool> {
    fn to__1(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
}
pub trait MyBoolImpl {
    fn to_(&self) -> ();
}
impl MyBoolImpl for Ptr<MyBool> {
    fn to_(&self) -> () {
        return (*self).with(|__s| __s.value);
    }
}
pub fn __cpp2rust_init_globals() {}
