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
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_strong_ordering__std_strong_ordering_ {}
pub fn testComparisonsComplete_2(
    t1: Ptr<()>,
    t2: Ptr<()>,
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
pub fn testComparisonsValues_3(val1: Vec<i8>, val2: Vec<i8>) -> bool {
    let val1: Value<Vec<i8>> = Rc::new(RefCell::new(val1));
    let val2: Value<Vec<i8>> = Rc::new(RefCell::new(val2));
    let mut isEqual: bool = unimplemented!();
    let mut isLess: bool = unimplemented!() == std::cmp::Ordering::Less;
    let mut isGreater: bool = unimplemented!() == std::cmp::Ordering::Greater;
    return ({
        let _t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        testComparisonsComplete_2(
            _t1.as_pointer(),
            _t2.as_pointer(),
            isEqual,
            isLess,
            isGreater,
        )
    });
}
pub fn AssertComparisonsAreNoexcept_4() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsReturnBool_5() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertOrderAreNoexcept_6() {
    ({ AssertComparisonsAreNoexcept_4() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderReturn_7() {
    ({ AssertComparisonsReturnBool_5() });
    const _: () = assert!(true, "");;
}
pub fn testOrder_8(t1: Ptr<()>, t2: Ptr<()>, order: std::cmp::Ordering) -> bool {
    let order: Value<std::cmp::Ordering> = Rc::new(RefCell::new(order));
    let mut equal: bool = (*order.borrow()) == std::cmp::Ordering::Equal;
    let mut less: bool = (*order.borrow()) == std::cmp::Ordering::Less;
    let mut greater: bool = (*order.borrow()) == std::cmp::Ordering::Greater;
    return (unimplemented!() == (*order.borrow()))
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: bool = equal;
            let _isLess: bool = less;
            let _isGreater: bool = greater;
            testComparisonsComplete_2(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrderValues_9(val1: Vec<i8>, val2: Vec<i8>) -> bool {
    let val1: Value<Vec<i8>> = Rc::new(RefCell::new(val1));
    let val2: Value<Vec<i8>> = Rc::new(RefCell::new(val2));
    return ({
        let _t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _order: std::cmp::Ordering = unimplemented!();
        testOrder_8(_t1.as_pointer(), _t2.as_pointer(), _order)
    });
}
pub fn operator_lt_10(lhs: Ptr<LessAndEqComp>, rhs: Ptr<LessAndEqComp>) -> bool {
    return ({ lhs.with(|__s| __s.value) } < { rhs.with(|__s| __s.value) });
}
pub fn operator_eq_11(lhs: Ptr<LessAndEqComp>, rhs: Ptr<LessAndEqComp>) -> bool {
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
        operator_lt_10(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for LessAndEqComp {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_11(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_cmp_12(mut _a0: StrongOrder, mut _a1: StrongOrder) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> =
            Rc::new(RefCell::new(std::cmp::Ord::cmp(&(_a0.value), &(_a1.value))));
        if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
            return (*cmp.borrow());
        }
    }
    return std::cmp::Ordering::Equal;
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
        operator_cmp_12(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Less
    }
}
impl CppEq for StrongOrder {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_12(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Equal
    }
}
pub fn operator_cmp_13(mut _a0: WeakOrder, mut _a1: WeakOrder) -> () {
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
        operator_cmp_13(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Less
    }
}
impl CppEq for WeakOrder {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_13(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Equal
    }
}
pub fn operator_cmp_14(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    if (lhs.value == i32::MIN) || (rhs.value == i32::MIN) {
        return unimplemented!();
    }
    if (lhs.value == i32::MAX) || (rhs.value == i32::MAX) {
        return unimplemented!();
    }
    return unimplemented!();
}
pub fn operator_eq_15(mut lhs: PartialOrder, mut rhs: PartialOrder) -> bool {
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
        operator_cmp_14(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Less
    }
}
impl CppEq for PartialOrder {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_15(__lhs.read(), __rhs.read())
    }
}
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
pub fn operator_eq_16(_a0: Ptr<EqualityComparable>, _a1: Ptr<EqualityComparable>) -> bool {
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
        operator_eq_16(__lhs.clone(), __rhs.clone())
    }
}
const _: () = assert!(true, "std::equality_comparable<EqualityComparable>");
const _: () = assert!(true, "HasOperatorEqual<EqualityComparable>");
const _: () = assert!(true, "HasOperatorNotEqual<EqualityComparable>");
pub fn operator_eq_17(_a0: Ptr<ThreeWayComparable>, _a1: Ptr<ThreeWayComparable>) -> bool {
    return ({ _a0.with(|__s| __s.value_) } == { _a1.with(|__s| __s.value_) });
}
pub fn operator_cmp_18(
    _a0: Ptr<ThreeWayComparable>,
    _a1: Ptr<ThreeWayComparable>,
) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.value_)),
            &(_a1.with(|__s| __s.value_)),
        )));
        if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
            return (*cmp.borrow());
        }
    }
    return std::cmp::Ordering::Equal;
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
        operator_cmp_18(__lhs.clone(), __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for ThreeWayComparable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_17(__lhs.clone(), __rhs.clone())
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct ImplicitBool_E2 {
    #[offset(0)]
    pub y: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(bool, to_bool_1)]
#[byte_size(1)]
pub struct ImplicitBool {
    #[offset(0)]
    pub val: bool,
}
pub fn operator_eq_19(mut lhs: ImplicitBool_E1, mut rhs: ImplicitBool_E2) -> ImplicitBool {
    return ImplicitBool {
        val: (lhs.x == rhs.y),
    };
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
    ({ AssertComparisonsAreNoexcept_4() });
    ({ AssertComparisonsReturnBool_5() });
    ({ AssertOrderAreNoexcept_6() });
    ({ AssertOrderReturn_7() });
    let TestCases: Value<Box<[(Value<Vec<i8>>, Value<Vec<i8>>)]>> =
        Rc::new(RefCell::new(Box::new([
            (
                Rc::new(RefCell::new({
                    let mut __v: Vec<i8> = (Ptr::<i8>::from_string_literal(b"") as Ptr<i8>)
                        .path_bytes()
                        .into_iter()
                        .map(|__b| __b as i8)
                        .collect();
                    __v.push(0);
                    __v
                })),
                Rc::new(RefCell::new({
                    let mut __v: Vec<i8> = (Ptr::<i8>::from_string_literal(b"") as Ptr<i8>)
                        .path_bytes()
                        .into_iter()
                        .map(|__b| __b as i8)
                        .collect();
                    __v.push(0);
                    __v
                })),
            ),
            (
                Rc::new(RefCell::new({
                    let mut __v: Vec<i8> = (Ptr::<i8>::from_string_literal(b"") as Ptr<i8>)
                        .path_bytes()
                        .into_iter()
                        .map(|__b| __b as i8)
                        .collect();
                    __v.push(0);
                    __v
                })),
                Rc::new(RefCell::new({
                    let mut __v: Vec<i8> = (Ptr::<i8>::from_string_literal(b"a") as Ptr<i8>)
                        .path_bytes()
                        .into_iter()
                        .map(|__b| __b as i8)
                        .collect();
                    __v.push(0);
                    __v
                })),
            ),
            (
                Rc::new(RefCell::new({
                    let mut __v: Vec<i8> = (Ptr::<i8>::from_string_literal(b"a") as Ptr<i8>)
                        .path_bytes()
                        .into_iter()
                        .map(|__b| __b as i8)
                        .collect();
                    __v.push(0);
                    __v
                })),
                Rc::new(RefCell::new({
                    let mut __v: Vec<i8> = (Ptr::<i8>::from_string_literal(b"a") as Ptr<i8>)
                        .path_bytes()
                        .into_iter()
                        .map(|__b| __b as i8)
                        .collect();
                    __v.push(0);
                    __v
                })),
            ),
            (
                Rc::new(RefCell::new({
                    let mut __v: Vec<i8> = (Ptr::<i8>::from_string_literal(b"a") as Ptr<i8>)
                        .path_bytes()
                        .into_iter()
                        .map(|__b| __b as i8)
                        .collect();
                    __v.push(0);
                    __v
                })),
                Rc::new(RefCell::new({
                    let mut __v: Vec<i8> = (Ptr::<i8>::from_string_literal(b"b") as Ptr<i8>)
                        .path_bytes()
                        .into_iter()
                        .map(|__b| __b as i8)
                        .collect();
                    __v.push(0);
                    __v
                })),
            ),
            (
                Rc::new(RefCell::new({
                    let mut __v: Vec<i8> = (Ptr::<i8>::from_string_literal(b"foo/bar/baz")
                        as Ptr<i8>)
                        .path_bytes()
                        .into_iter()
                        .map(|__b| __b as i8)
                        .collect();
                    __v.push(0);
                    __v
                })),
                Rc::new(RefCell::new({
                    let mut __v: Vec<i8> = (Ptr::<i8>::from_string_literal(b"foo/bar/baz/")
                        as Ptr<i8>)
                        .path_bytes()
                        .into_iter()
                        .map(|__b| __b as i8)
                        .collect();
                    __v.push(0);
                    __v
                })),
            ),
        ])));
    'loop_: for TC in IterRange::new(
        (TestCases.as_pointer() as Ptr<(Value<Vec<i8>>, Value<Vec<i8>>)>),
        (TestCases.as_pointer() as Ptr<(Value<Vec<i8>>, Value<Vec<i8>>)>).offset((5_i64) as isize),
    ) {
        let TC: Ptr<(Value<Vec<i8>>, Value<Vec<i8>>)> = (TC).clone();
        assert!(
            ({
                let _val1: Vec<i8> = unimplemented!();
                let _val2: Vec<i8> = unimplemented!();
                testComparisonsValues_3(_val1, _val2)
            })
        );
        assert!(
            ({
                let _val1: Vec<i8> = unimplemented!();
                let _val2: Vec<i8> = unimplemented!();
                testComparisonsValues_3(_val1, _val2)
            })
        );
        assert!(
            ({
                let _val1: Vec<i8> = unimplemented!();
                let _val2: Vec<i8> = unimplemented!();
                testOrderValues_9(_val1, _val2)
            })
        );
        assert!(
            ({
                let _val1: Vec<i8> = unimplemented!();
                let _val2: Vec<i8> = unimplemented!();
                testOrderValues_9(_val1, _val2)
            })
        );
    }
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
