extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_lt_0(lhs: Ptr<LessAndEqComp>, rhs: Ptr<LessAndEqComp>) -> bool {
    return ({ lhs.with(|__s| __s.value) } < { rhs.with(|__s| __s.value) });
}
pub fn operator_eq_1(lhs: Ptr<LessAndEqComp>, rhs: Ptr<LessAndEqComp>) -> bool {
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
        operator_lt_0(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for LessAndEqComp {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for LessAndEqComp {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_cmp_2(mut _a0: StrongOrder, mut _a1: StrongOrder) -> std::cmp::Ordering {
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
        operator_cmp_2(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Less
    }
}
impl CppEq for StrongOrder {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_2(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Equal
    }
}
impl CppMove for StrongOrder {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_cmp_3(mut _a0: WeakOrder, mut _a1: WeakOrder) -> () {
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
        operator_cmp_3(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Less
    }
}
impl CppEq for WeakOrder {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_3(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Equal
    }
}
impl CppMove for WeakOrder {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_cmp_4(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    if (lhs.value == i32::MIN) || (rhs.value == i32::MIN) {
        return unimplemented!();
    }
    if (lhs.value == i32::MAX) || (rhs.value == i32::MAX) {
        return unimplemented!();
    }
    return unimplemented!();
}
pub fn operator_eq_5(mut lhs: PartialOrder, mut rhs: PartialOrder) -> bool {
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
        operator_cmp_4(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Less
    }
}
impl CppEq for PartialOrder {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_5(__lhs.read(), __rhs.read())
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
pub fn operator_eq_6(_a0: Ptr<EqualityComparable>, _a1: Ptr<EqualityComparable>) -> bool {
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
        operator_eq_6(__lhs.clone(), __rhs.clone())
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
pub fn operator_eq_7(_a0: Ptr<ThreeWayComparable>, _a1: Ptr<ThreeWayComparable>) -> bool {
    return ({ _a0.with(|__s| __s.value_) } == { _a1.with(|__s| __s.value_) });
}
pub fn operator_cmp_8(
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
        operator_cmp_8(__lhs.clone(), __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for ThreeWayComparable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_7(__lhs.clone(), __rhs.clone())
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
pub fn operator_eq_9(mut lhs: ImplicitBool_E1, mut rhs: ImplicitBool_E2) -> ImplicitBool {
    return ImplicitBool {
        val: (lhs.x == rhs.y),
    };
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(1)]
pub struct support_double_move_tracker {
    #[offset(0)]
    moved_from_: bool,
}
impl support_double_move_tracker {
    pub fn new() -> Self {
        Self { moved_from_: false }
    }
    pub fn move_from(other: Ptr<support_double_move_tracker>) -> Self {
        let __this: support_double_move_tracker = Self { moved_from_: false };
        assert!(!(other.with(|__s| __s.moved_from_)));
        field!(other, moved_from_).write(true);
        __this
    }
}
impl CppMove for support_double_move_tracker {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for support_double_move_tracker {
    fn default() -> Self {
        { support_double_move_tracker::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp17_output_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {
    pub fn move_from(_a0: Ptr<cpp17_output_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for cpp17_output_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {
    pub fn move_from(_a0: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for cpp17_input_iterator_int_ptr__int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_eq_10(
    x: Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>,
    y: Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_11(
    x: Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>,
    y: Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    pub fn new(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_10(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {}
impl CppAdvance for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_5();
    }
}
impl CppIterator<i32> for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            cpp17_input_iterator_const_int_ptr__const_int_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            cpp17_input_iterator_const_int_ptr__const_int_ptr_Impl::operator_inc_5(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {}
impl CppMove for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub fn operator_eq_12(
    x: Ptr<forward_iterator_const_int_ptr_>,
    y: Ptr<forward_iterator_const_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_13(
    x: Ptr<forward_iterator_const_int_ptr_>,
    y: Ptr<forward_iterator_const_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct forward_iterator_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<forward_iterator_const_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for forward_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_12(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for forward_iterator_const_int_ptr_ {}
impl CppAdvance for forward_iterator_const_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
}
impl CppIterator<i32> for forward_iterator_const_int_ptr_ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            forward_iterator_const_int_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            forward_iterator_const_int_ptr_Impl::operator_inc_6(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for forward_iterator_const_int_ptr_ {}
impl CppMove for forward_iterator_const_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_eq_14(
    x: Ptr<bidirectional_iterator_const_int_ptr_>,
    y: Ptr<bidirectional_iterator_const_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_15(
    x: Ptr<bidirectional_iterator_const_int_ptr_>,
    y: Ptr<bidirectional_iterator_const_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct bidirectional_iterator_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_const_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<bidirectional_iterator_const_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_14(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_const_int_ptr_ {}
impl CppAdvance for bidirectional_iterator_const_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
}
impl CppIterator<i32> for bidirectional_iterator_const_int_ptr_ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            bidirectional_iterator_const_int_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            bidirectional_iterator_const_int_ptr_Impl::operator_inc_6(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for bidirectional_iterator_const_int_ptr_ {}
impl CppMove for bidirectional_iterator_const_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_eq_16(
    x: Ptr<random_access_iterator_const_int_ptr_>,
    y: Ptr<random_access_iterator_const_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_17(
    x: Ptr<random_access_iterator_const_int_ptr_>,
    y: Ptr<random_access_iterator_const_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct random_access_iterator_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_const_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<random_access_iterator_const_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for random_access_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_16(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for random_access_iterator_const_int_ptr_ {}
impl CppAdvance for random_access_iterator_const_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_7();
    }
}
impl CppIterator<i32> for random_access_iterator_const_int_ptr_ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            random_access_iterator_const_int_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            random_access_iterator_const_int_ptr_Impl::operator_inc_7(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for random_access_iterator_const_int_ptr_ {}
impl CppMove for random_access_iterator_const_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_random_access_iterator_int_ptr_ {
    pub fn move_from(_a0: Ptr<cpp20_random_access_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for cpp20_random_access_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
const _: () = assert!(
    true,
    "std::random_access_iterator<cpp20_random_access_iterator<int*>>"
);
pub fn operator_eq_18(
    x: Ptr<contiguous_iterator_const_int_ptr_>,
    y: Ptr<contiguous_iterator_const_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_19(
    x: Ptr<contiguous_iterator_const_int_ptr_>,
    y: Ptr<contiguous_iterator_const_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<contiguous_iterator_const_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for contiguous_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_18(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_const_int_ptr_ {}
impl CppAdvance for contiguous_iterator_const_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_9();
    }
}
impl CppIterator<i32> for contiguous_iterator_const_int_ptr_ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            contiguous_iterator_const_int_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            contiguous_iterator_const_int_ptr_Impl::operator_inc_9(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for contiguous_iterator_const_int_ptr_ {}
impl CppMove for contiguous_iterator_const_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_input_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_int_ptr_ {
    pub fn move_from(_a0: Ptr<cpp20_input_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for cpp20_input_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
const _: () = assert!(true, "std::input_iterator<cpp20_input_iterator<int*>>");
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_output_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_output_iterator_int_ptr_ {
    pub fn move_from(_a0: Ptr<cpp20_output_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for cpp20_output_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp20_output_iterator<int*>, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct IteratorOpCounts {
    #[offset(0)]
    pub increments: usize,
    #[offset(8)]
    pub decrements: usize,
    #[offset(16)]
    pub zero_moves: usize,
    #[offset(24)]
    pub equal_cmps: usize,
}
impl CppMove for IteratorOpCounts {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        {
            IteratorOpCounts {
                increments: 0_usize,
                decrements: 0_usize,
                zero_moves: 0_usize,
                equal_cmps: 0_usize,
            }
        }
    }
}
pub fn operator_eq_20(
    _a0: Ptr<forward_sized_iterator_int_ptr_>,
    _a1: Ptr<forward_sized_iterator_int_ptr_>,
) -> bool {
    return ({ _a0.with(|__s| __s.it_.clone()) } == { _a1.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct forward_sized_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    pub it_: Ptr<i32>,
}
impl forward_sized_iterator_int_ptr_ {}
impl CppEq for forward_sized_iterator_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_20(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for forward_sized_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for forward_sized_iterator_int_ptr_ {
    fn default() -> Self {
        {
            forward_sized_iterator_int_ptr_ {
                it_: Ptr::<i32>::null(),
            }
        }
    }
}
const _: () = assert!(true, "std::forward_iterator<forward_sized_iterator<>>");
const _: () = assert!(
    true,
    "std::sized_sentinel_for<forward_sized_iterator<>, forward_sized_iterator<>>"
);
pub fn operator_add_21(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_22(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_21(_i, n)
    });
}
pub fn iter_swap_23(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap!((a.ptr_).clone(), (b.ptr_).clone());
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| {
            __v.prefix_inc();
        });
    }
}
pub fn iter_move_24(iter: adl_Iterator) -> Ptr<i32> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| {
            __v.prefix_inc();
        });
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_25(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> bool {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_26(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> std::cmp::Ordering {
    return std::cmp::Ord::cmp(
        &(lhs.with(|__s| __s.ptr_.clone())),
        &(rhs.with(|__s| __s.ptr_.clone())),
    );
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct adl_Iterator {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    iter_moves_: Ptr<i32>,
    #[offset(16)]
    #[byte_size(8)]
    iter_swaps_: Ptr<i32>,
}
impl adl_Iterator {
    fn new_1(mut p: Ptr<i32>, mut iter_moves: Ptr<i32>, mut iter_swaps: Ptr<i32>) -> Self {
        Self {
            ptr_: (p).clone(),
            iter_moves_: (iter_moves).clone(),
            iter_swaps_: (iter_swaps).clone(),
        }
    }
    pub fn TrackMoves(mut p: Ptr<i32>, iter_moves: Ptr<i32>) -> adl_Iterator {
        return adl_Iterator::new_1({ (p).clone() }, { (iter_moves).clone() }, {
            Ptr::<i32>::null()
        });
    }
    pub fn TrackSwaps_4(iter_swaps: Ptr<i32>) -> adl_Iterator {
        return adl_Iterator::new_1({ Ptr::<i32>::null() }, { Ptr::<i32>::null() }, {
            (iter_swaps).clone()
        });
    }
    pub fn TrackSwaps_5(mut p: Ptr<i32>, iter_swaps: Ptr<i32>) -> adl_Iterator {
        return adl_Iterator::new_1({ (p).clone() }, { Ptr::<i32>::null() }, {
            (iter_swaps).clone()
        });
    }
}
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_cmp_26(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_25(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for adl_Iterator {}
impl CppAdvance for adl_Iterator {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_14();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_16();
    }
}
impl CppIterator<i32> for adl_Iterator {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            adl_IteratorImpl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            adl_IteratorImpl::operator_inc_14(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for adl_Iterator {}
impl CppBidirectionalIterator<i32> for adl_Iterator {
    fn iter_dec(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            adl_IteratorImpl::operator_dec_16(__it);
        });
        self
    }
}
impl CppRandomAccessIterator<i32> for adl_Iterator {
    fn iter_add_assign(&mut self, n: isize) -> Self {
        libcc2rs::with_record_ptr(self, |__it| {
            adl_IteratorImpl::operator_add_assign(__it, n as _);
        });
        self.clone()
    }
    fn iter_distance(&self, from: &Self) -> isize {
        libcc2rs::with_record_ptrs(self, from, |__lhs, __rhs| {
            (adl_IteratorImpl::operator_sub_11(__lhs, __rhs.read())) as isize
        })
    }
}
impl CppMove for adl_Iterator {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for adl_Iterator {
    fn default() -> Self {
        {
            adl_Iterator {
                ptr_: Ptr::<i32>::null(),
                iter_moves_: Ptr::<i32>::null(),
                iter_swaps_: Ptr::<i32>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct rvalue_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<Ptr<i32>>,
}
impl rvalue_iterator_int_ptr_ {}
impl CppLess for rvalue_iterator_int_ptr_ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        rvalue_iterator_int_ptr_Impl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for rvalue_iterator_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        rvalue_iterator_int_ptr_Impl::operator_cmp(__lhs, __rhs.clone())
            == std::cmp::Ordering::Equal
    }
}
impl CppMove for rvalue_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(true, "std::random_access_iterator<rvalue_iterator<int*>>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Ptr< i8  >  , new, value)]
#[byte_size(16)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub fn new(mut what: Ptr<i8>) -> Self {
        Self {}
    }
}
impl CppMove for util_iterator_error {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_lexicographical_compare_27(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_28(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_29(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_30(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_31(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_32(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_33(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_34(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_35(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_36(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_37(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_38(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_39(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_40(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_41(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_42(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_43(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_44(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_45(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_46(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_47(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_48(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_49(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_50(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_51(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_52(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_53(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_54(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_55(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_56(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_57(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_58(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_59(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_60(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_61(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_62(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_63(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_64(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_65(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_66(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_67(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_68(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_69(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_70(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_71(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_72(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_73(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_74(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_75(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_76(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_77(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_78(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_79(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_80(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_81(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_82(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_83(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_84(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_85(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_86(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_87(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_88(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_89(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_90(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_91(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_92(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_93(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_94(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_95(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_96(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_97(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_98(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_99(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_100(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_101(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_102(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_103(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_104(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_105(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_106(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_107(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_108(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_109(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_110(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_111(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_112(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_113(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_114(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_115(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_116(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_117(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_118(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_119(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_120(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_121(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_122(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_123(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_124(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_125(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_126(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_127(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_128(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_129(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_130(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_131(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_132(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_133(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_134(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_135(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_136(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_137(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_138(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_139(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_140(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_141(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_142(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_143(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_144(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_145(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_146(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_147(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_148(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_149(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_150(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_151(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_152(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_153(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_154(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_155(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_156(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_157(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_158(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_159(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_160(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_161(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_162(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_163(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_164(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_165(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_166(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_167(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_168(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_169(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_170(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_171(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_172(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_173(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_174(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_175(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_176(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_177(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_178(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_179(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_180(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_181(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_182(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_183(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_184(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_185(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_186(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_187(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_188(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_189(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_190(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_191(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_192(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_193(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_194(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_195(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_196(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_197(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_198(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_199(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_200(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_201(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_202(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_203(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_204(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_205(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_206(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_207(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_208(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_209(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_210(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_211(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_212(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_213(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_214(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_215(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_216(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_217(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_218(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_219(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_220(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_221(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_222(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_223(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_224(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_225(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_226(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_227(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_228(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_229(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_230(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_231(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_232(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_233(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_234(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_235(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_236(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_237(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_238(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_239(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_240(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_241(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_242(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_243(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_244(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_245(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_246(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_247(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_248(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_249(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_250(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_251(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_252(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_253(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_254(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_255(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_256(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_257(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_258(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_259(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_260(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_261(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_262(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_263(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_264(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_265(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_266(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_267(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_268(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_269(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_270(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_271(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_272(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_273(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_274(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_275(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_276(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_277(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_278(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_279(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_280(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_281(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_282(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_283(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_284(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_285(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_286(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_287(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_288(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_289(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_290(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_291(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_292(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_293(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_294(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_295(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_296(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_297(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_298(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_299(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_300(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_301(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_302(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_303(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_304(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_305(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_306(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_307(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_308(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_309(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_310(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_311(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_312(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_313(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_314(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_315(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_316(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_317(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_318(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_319(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_320(a: Vec<i32>, b: Vec<i32>, expected: std::cmp::Ordering) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_321(
    a: Vec<StrongOrder>,
    b: Vec<StrongOrder>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<StrongOrder>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<StrongOrder>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_322(a: Vec<WeakOrder>, b: Vec<WeakOrder>, expected: ()) {
    let a: Value<Vec<WeakOrder>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<WeakOrder>> = Rc::new(RefCell::new(b));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(unimplemented!());
}
pub fn test_lexicographical_compare_323(a: Vec<PartialOrder>, b: Vec<PartialOrder>, expected: ()) {
    let a: Value<Vec<PartialOrder>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<PartialOrder>> = Rc::new(RefCell::new(b));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(unimplemented!());
}
pub fn test_lexicographical_compare_324(
    a: Vec<StrongOrder>,
    b: Vec<StrongOrder>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<StrongOrder>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<StrongOrder>> = Rc::new(RefCell::new(b));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (*expected.borrow()) }) == ({ (*result.borrow()) }));
}
pub fn test_lexicographical_compare_325(a: Vec<WeakOrder>, b: Vec<WeakOrder>, expected: ()) {
    let a: Value<Vec<WeakOrder>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<WeakOrder>> = Rc::new(RefCell::new(b));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(unimplemented!());
}
pub fn test_lexicographical_compare_326(a: Vec<PartialOrder>, b: Vec<PartialOrder>, expected: ()) {
    let a: Value<Vec<PartialOrder>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<PartialOrder>> = Rc::new(RefCell::new(b));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(unimplemented!());
}
pub fn test_given_iterator_types_327() {
    ({
        test_lexicographical_compare_27(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_28(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_29(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_30(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_30(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_30(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_30(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_31(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_32(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_328() {
    ({
        test_lexicographical_compare_33(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_34(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_35(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_36(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_36(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_36(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_36(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_37(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_38(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_329() {
    ({
        test_lexicographical_compare_39(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_40(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_41(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_42(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_42(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_42(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_42(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_43(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_44(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_330() {
    ({
        test_lexicographical_compare_45(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_46(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_47(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_48(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_48(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_48(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_48(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_49(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_50(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_331() {
    ({
        test_lexicographical_compare_51(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_52(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_53(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_54(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_54(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_54(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_54(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_55(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_56(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_332() {
    ({
        test_lexicographical_compare_57(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_58(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_59(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_60(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_60(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_60(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_60(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_61(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_62(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_333() {
    ({
        test_lexicographical_compare_63(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_64(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_65(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_66(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_66(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_66(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_66(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_67(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_68(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_334() {
    ({
        test_lexicographical_compare_69(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_70(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_71(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_72(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_72(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_72(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_72(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_73(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_74(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_335() {
    ({
        test_lexicographical_compare_75(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_76(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_77(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_78(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_78(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_78(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_78(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_79(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_80(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_336() {
    ({
        test_lexicographical_compare_81(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_82(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_83(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_84(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_84(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_84(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_84(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_85(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_86(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_337() {
    ({
        test_lexicographical_compare_87(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_88(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_89(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_90(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_90(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_90(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_90(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_91(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_92(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_338() {
    ({
        test_lexicographical_compare_93(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_94(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_95(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_96(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_96(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_96(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_96(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_97(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_98(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_339() {
    ({
        test_lexicographical_compare_99(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_100(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_101(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_102(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_102(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_102(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_102(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_103(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_104(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_340() {
    ({
        test_lexicographical_compare_105(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_106(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_107(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_108(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_108(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_108(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_108(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_109(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_110(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_341() {
    ({
        test_lexicographical_compare_111(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_112(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_113(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_114(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_114(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_114(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_114(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_115(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_116(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_342() {
    ({
        test_lexicographical_compare_117(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_118(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_119(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_120(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_120(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_120(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_120(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_121(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_122(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_343() {
    ({
        test_lexicographical_compare_123(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_124(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_125(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_126(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_126(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_126(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_126(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_127(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_128(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_344() {
    ({
        test_lexicographical_compare_129(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_130(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_131(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_132(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_132(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_132(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_132(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_133(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_134(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_345() {
    ({
        test_lexicographical_compare_135(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_136(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_137(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_138(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_138(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_138(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_138(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_139(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_140(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_346() {
    ({
        test_lexicographical_compare_141(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_142(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_143(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_144(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_144(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_144(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_144(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_145(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_146(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_347() {
    ({
        test_lexicographical_compare_147(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_148(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_149(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_150(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_150(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_150(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_150(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_151(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_152(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_348() {
    ({
        test_lexicographical_compare_153(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_154(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_155(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_156(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_156(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_156(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_156(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_157(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_158(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_349() {
    ({
        test_lexicographical_compare_159(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_160(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_161(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_162(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_162(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_162(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_162(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_163(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_164(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_350() {
    ({
        test_lexicographical_compare_165(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_166(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_167(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_168(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_168(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_168(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_168(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_169(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_170(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_351() {
    ({
        test_lexicographical_compare_171(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_172(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_173(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_174(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_174(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_174(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_174(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_175(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_176(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_352() {
    ({
        test_lexicographical_compare_177(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_178(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_179(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_180(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_180(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_180(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_180(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_181(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_182(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_353() {
    ({
        test_lexicographical_compare_183(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_184(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_185(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_186(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_186(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_186(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_186(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_187(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_188(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_354() {
    ({
        test_lexicographical_compare_189(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_190(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_191(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_192(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_192(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_192(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_192(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_193(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_194(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_355() {
    ({
        test_lexicographical_compare_195(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_196(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_197(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_198(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_198(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_198(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_198(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_199(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_200(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_356() {
    ({
        test_lexicographical_compare_201(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_202(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_203(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_204(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_204(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_204(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_204(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_205(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_206(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_357() {
    ({
        test_lexicographical_compare_207(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_208(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_209(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_210(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_210(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_210(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_210(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_211(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_212(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_358() {
    ({
        test_lexicographical_compare_213(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_214(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_215(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_216(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_216(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_216(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_216(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_217(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_218(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_359() {
    ({
        test_lexicographical_compare_219(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_220(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_221(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_222(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_222(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_222(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_222(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_223(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_224(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_360() {
    ({
        test_lexicographical_compare_225(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_226(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_227(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_228(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_228(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_228(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_228(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_229(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_230(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_361() {
    ({
        test_lexicographical_compare_231(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_232(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_233(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_234(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_234(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_234(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_234(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_235(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_236(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_362() {
    ({
        test_lexicographical_compare_237(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_238(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_239(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_240(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_240(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_240(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_240(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_241(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_242(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_363() {
    ({
        test_lexicographical_compare_243(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_244(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_245(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_246(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_246(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_246(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_246(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_247(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_248(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_364() {
    ({
        test_lexicographical_compare_249(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_250(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_251(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_252(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_252(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_252(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_252(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_253(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_254(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_365() {
    ({
        test_lexicographical_compare_255(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_256(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_257(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_258(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_258(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_258(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_258(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_259(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_260(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_366() {
    ({
        test_lexicographical_compare_261(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_262(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_263(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_264(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_264(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_264(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_264(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_265(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_266(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_367() {
    ({
        test_lexicographical_compare_267(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_268(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_269(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_270(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_270(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_270(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_270(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_271(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_272(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_368() {
    ({
        test_lexicographical_compare_273(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_274(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_275(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_276(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_276(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_276(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_276(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_277(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_278(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_369() {
    ({
        test_lexicographical_compare_279(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_280(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_281(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_282(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_282(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_282(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_282(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_283(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_284(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_370() {
    ({
        test_lexicographical_compare_285(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_286(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_287(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_288(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_288(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_288(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_288(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_289(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_290(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_371() {
    ({
        test_lexicographical_compare_291(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_292(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_293(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_294(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_294(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_294(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_294(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_295(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_296(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_372() {
    ({
        test_lexicographical_compare_297(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_298(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_299(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_300(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_300(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_300(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_300(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_301(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_302(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_373() {
    ({
        test_lexicographical_compare_303(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_304(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_305(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_306(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_306(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_306(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_306(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_307(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_308(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_374() {
    ({
        test_lexicographical_compare_309(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_310(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_311(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_312(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_312(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_312(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_312(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_313(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_314(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_given_iterator_types_375() {
    ({
        test_lexicographical_compare_315(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_316(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_317(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_318(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_318(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_318(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_318(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_319(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_320(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
        )
    });
}
pub fn test_iterator_types1_376() {
    ({ test_given_iterator_types_327() });
    ({ test_given_iterator_types_328() });
    ({ test_given_iterator_types_329() });
    ({ test_given_iterator_types_330() });
    ({ test_given_iterator_types_331() });
    ({ test_given_iterator_types_332() });
    ({ test_given_iterator_types_333() });
}
pub fn test_iterator_types1_377() {
    ({ test_given_iterator_types_334() });
    ({ test_given_iterator_types_335() });
    ({ test_given_iterator_types_336() });
    ({ test_given_iterator_types_337() });
    ({ test_given_iterator_types_338() });
    ({ test_given_iterator_types_339() });
    ({ test_given_iterator_types_340() });
}
pub fn test_iterator_types1_378() {
    ({ test_given_iterator_types_341() });
    ({ test_given_iterator_types_342() });
    ({ test_given_iterator_types_343() });
    ({ test_given_iterator_types_344() });
    ({ test_given_iterator_types_345() });
    ({ test_given_iterator_types_346() });
    ({ test_given_iterator_types_347() });
}
pub fn test_iterator_types1_379() {
    ({ test_given_iterator_types_348() });
    ({ test_given_iterator_types_349() });
    ({ test_given_iterator_types_350() });
    ({ test_given_iterator_types_351() });
    ({ test_given_iterator_types_352() });
    ({ test_given_iterator_types_353() });
    ({ test_given_iterator_types_354() });
}
pub fn test_iterator_types1_380() {
    ({ test_given_iterator_types_355() });
    ({ test_given_iterator_types_356() });
    ({ test_given_iterator_types_357() });
    ({ test_given_iterator_types_358() });
    ({ test_given_iterator_types_359() });
    ({ test_given_iterator_types_360() });
    ({ test_given_iterator_types_361() });
}
pub fn test_iterator_types1_381() {
    ({ test_given_iterator_types_362() });
    ({ test_given_iterator_types_363() });
    ({ test_given_iterator_types_364() });
    ({ test_given_iterator_types_365() });
    ({ test_given_iterator_types_366() });
    ({ test_given_iterator_types_367() });
    ({ test_given_iterator_types_368() });
}
pub fn test_iterator_types1_382() {
    ({ test_given_iterator_types_369() });
    ({ test_given_iterator_types_370() });
    ({ test_given_iterator_types_371() });
    ({ test_given_iterator_types_372() });
    ({ test_given_iterator_types_373() });
    ({ test_given_iterator_types_374() });
    ({ test_given_iterator_types_375() });
}
pub fn test_iterator_types_383() {
    ({ test_iterator_types1_376() });
    ({ test_iterator_types1_377() });
    ({ test_iterator_types1_378() });
    ({ test_iterator_types1_379() });
    ({ test_iterator_types1_380() });
    ({ test_iterator_types1_381() });
    ({ test_iterator_types1_382() });
}
pub fn test_comparison_categories_384() {
    ({
        test_lexicographical_compare_321(
            Vec::from([StrongOrder::new({ 0 }), StrongOrder::new({ 1 })]),
            Vec::from([StrongOrder::new({ 1 }), StrongOrder::new({ 1 })]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_322(
            Vec::from([WeakOrder::new({ 0 }), WeakOrder::new({ 1 })]),
            Vec::from([WeakOrder::new({ 1 }), WeakOrder::new({ 1 })]),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_323(
            Vec::from([PartialOrder::new({ 0 }), PartialOrder::new({ 1 })]),
            Vec::from([PartialOrder::new({ 1 }), PartialOrder::new({ 1 })]),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_324(
            Vec::from([StrongOrder::new({ 0 }), StrongOrder::new({ 1 })]),
            Vec::from([
                StrongOrder::new({ 0 }),
                StrongOrder::new({ 1 }),
                StrongOrder::new({ 2 }),
            ]),
            ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
        )
    });
    ({
        test_lexicographical_compare_325(
            Vec::from([WeakOrder::new({ 0 }), WeakOrder::new({ 1 })]),
            Vec::from([
                WeakOrder::new({ 0 }),
                WeakOrder::new({ 1 }),
                WeakOrder::new({ 2 }),
            ]),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_326(
            Vec::from([PartialOrder::new({ 0 }), PartialOrder::new({ 1 })]),
            Vec::from([
                PartialOrder::new({ 0 }),
                PartialOrder::new({ 1 }),
                PartialOrder::new({ 2 }),
            ]),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_326(
            Vec::from([PartialOrder::new({ i32::MIN }), PartialOrder::new({ 1 })]),
            Vec::from([
                PartialOrder::new({ 0 }),
                PartialOrder::new({ 1 }),
                PartialOrder::new({ 2 }),
            ]),
            unimplemented!(),
        )
    });
}
pub fn test_proxy_iterators_385() {
    let vec_: Value<Vec<bool>> = Rc::new(RefCell::new(
        (0..(10_usize) as usize)
            .map(|_| true)
            .collect::<Vec<bool>>(),
    ));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ({ (*result.borrow()) }) == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) })
    );
}
pub fn test_386() -> bool {
    ({ test_iterator_types_383() });
    ({ test_comparison_categories_384() });
    ({ test_proxy_iterators_385() });
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
    ({ test_386() });
    const _: () = assert!(true, "test()");;
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
pub trait adl_IteratorImpl {
    fn iter_moves(&self) -> i32;
    fn iter_swaps(&self) -> i32;
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_index(&self, n: isize) -> Ptr<i32>;
    fn operator_sub_10(&self, n: isize) -> adl_Iterator;
    fn operator_sub_11(&self, rhs: adl_Iterator) -> isize;
    fn operator_add_assign(&self, n: isize) -> Ptr<adl_Iterator>;
    fn operator_sub_assign(&self, n: isize) -> Ptr<adl_Iterator>;
    fn operator_inc_14(&self) -> Ptr<adl_Iterator>;
    fn operator_post_inc_15(&self, _a0: i32) -> adl_Iterator;
    fn operator_dec_16(&self) -> Ptr<adl_Iterator>;
    fn operator_post_dec_17(&self, _a0: i32) -> adl_Iterator;
}
impl adl_IteratorImpl for Ptr<adl_Iterator> {
    fn iter_moves(&self) -> i32 {
        assert!(!((*self).with(|__s| __s.iter_moves_.clone())).is_null());
        return ((*self).with(|__s| __s.iter_moves_.clone()).read());
    }
    fn iter_swaps(&self) -> i32 {
        assert!(!((*self).with(|__s| __s.iter_swaps_.clone())).is_null());
        return ((*self).with(|__s| __s.iter_swaps_.clone()).read());
    }
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
    fn operator_sub_10(&self, mut n: isize) -> adl_Iterator {
        return adl_Iterator::new_1(
            { (*self).with(|__s| __s.ptr_.clone()).offset(-((n) as isize)) },
            { (*self).with(|__s| __s.iter_moves_.clone()) },
            { (*self).with(|__s| __s.iter_swaps_.clone()) },
        );
    }
    fn operator_sub_11(&self, mut rhs: adl_Iterator) -> isize {
        return (((*self).with(|__s| __s.ptr_.clone()) - (rhs.ptr_).clone()) as i64 as isize);
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<adl_Iterator> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<adl_Iterator> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn operator_inc_14(&self) -> Ptr<adl_Iterator> {
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_post_inc_15(&self, mut _a0: i32) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return prev;
    }
    fn operator_dec_16(&self) -> Ptr<adl_Iterator> {
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_dec();
        });
        return (*self).clone();
    }
    fn operator_post_dec_17(&self, mut _a0: i32) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_dec();
        });
        return prev;
    }
}
pub trait bidirectional_iterator_const_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_const_int_ptr_>;
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_8(&self, mut _a0: i32) -> bidirectional_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_9(&self, mut _a0: i32) -> bidirectional_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_const_int_ptr_>,
    ) -> Ptr<bidirectional_iterator_const_int_ptr_>;
}
impl bidirectional_iterator_const_int_ptr_Impl for Ptr<bidirectional_iterator_const_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_const_int_ptr_>,
    ) -> Ptr<bidirectional_iterator_const_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait contiguous_iterator_const_int_ptr_Impl {
    fn base(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_arrow(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_const_int_ptr_>;
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: i32) -> contiguous_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: i32) -> contiguous_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<contiguous_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<contiguous_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_const_int_ptr_>,
    ) -> Ptr<contiguous_iterator_const_int_ptr_>;
}
impl contiguous_iterator_const_int_ptr_Impl for Ptr<contiguous_iterator_const_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_const_int_ptr_>,
    ) -> Ptr<contiguous_iterator_const_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait cpp17_input_iterator_const_int_ptr__const_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>;
    fn operator_post_inc_6(
        &self,
        mut _a0: i32,
    ) -> cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>,
    ) -> Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>;
}
impl cpp17_input_iterator_const_int_ptr__const_int_ptr_Impl
    for Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>
{
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>,
    ) -> Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait cpp17_input_iterator_int_ptr__int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: i32) -> cpp17_input_iterator_int_ptr__int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>,
    ) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_>;
}
impl cpp17_input_iterator_int_ptr__int_ptr_Impl for Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>,
    ) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait cpp17_output_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: i32) -> cpp17_output_iterator_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_output_iterator_int_ptr_>,
    ) -> Ptr<cpp17_output_iterator_int_ptr_>;
}
impl cpp17_output_iterator_int_ptr_Impl for Ptr<cpp17_output_iterator_int_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_output_iterator_int_ptr_>,
    ) -> Ptr<cpp17_output_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait cpp20_input_iterator_int_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_input_iterator_int_ptr_>,
    ) -> Ptr<cpp20_input_iterator_int_ptr_>;
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_5(&self) -> Ptr<cpp20_input_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: i32) {
        unimplemented!()
    }
}
impl cpp20_input_iterator_int_ptr_Impl for Ptr<cpp20_input_iterator_int_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_input_iterator_int_ptr_>,
    ) -> Ptr<cpp20_input_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait cpp20_output_iterator_int_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_output_iterator_int_ptr_>,
    ) -> Ptr<cpp20_output_iterator_int_ptr_>;
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_5(&self) -> Ptr<cpp20_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: i32) -> cpp20_output_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl cpp20_output_iterator_int_ptr_Impl for Ptr<cpp20_output_iterator_int_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_output_iterator_int_ptr_>,
    ) -> Ptr<cpp20_output_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait cpp20_random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_index(&self, n: isize) -> Ptr<i32>;
    fn operator_inc_7(&self) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_dec_8(&self) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: i32) -> cpp20_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: i32) -> cpp20_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_random_access_iterator_int_ptr_>,
    ) -> Ptr<cpp20_random_access_iterator_int_ptr_>;
}
impl cpp20_random_access_iterator_int_ptr_Impl for Ptr<cpp20_random_access_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone()).offset((n) as isize);
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_random_access_iterator_int_ptr_>,
    ) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait forward_iterator_const_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_int_ptr_>;
    fn operator_post_inc_7(&self, mut _a0: i32) -> forward_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<forward_iterator_const_int_ptr_>,
    ) -> Ptr<forward_iterator_const_int_ptr_>;
}
impl forward_iterator_const_int_ptr_Impl for Ptr<forward_iterator_const_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<forward_iterator_const_int_ptr_>,
    ) -> Ptr<forward_iterator_const_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait forward_sized_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_4(&self) -> Ptr<forward_sized_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: i32) -> forward_sized_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl forward_sized_iterator_int_ptr_Impl for Ptr<forward_sized_iterator_int_ptr_> {}
pub trait random_access_iterator_const_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_int_ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: i32) -> random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: i32) -> random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_const_int_ptr_>,
    ) -> Ptr<random_access_iterator_const_int_ptr_>;
}
impl random_access_iterator_const_int_ptr_Impl for Ptr<random_access_iterator_const_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_const_int_ptr_>,
    ) -> Ptr<random_access_iterator_const_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait rvalue_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn operator_inc_4(&self) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: i32) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_dec_6(&self) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_dec_7(&self, mut _a0: i32) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add(&self, mut n: isize) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_sub_9(&self, mut n: isize) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_sub_10(&self, other: Ptr<rvalue_iterator_int_ptr_>) -> isize {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn operator_cmp(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> std::cmp::Ordering;
    fn operator_eq(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> bool;
}
impl rvalue_iterator_int_ptr_Impl for Ptr<rvalue_iterator_int_ptr_> {
    fn operator_cmp(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.it_.clone())),
                &(_a0.with(|__s| __s.it_.clone())),
            )));
            if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
                return ({ (*cmp.borrow()) });
            }
        }
        return ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) });
    }
    fn operator_eq(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> bool {
        return ({ (*self).with(|__s| __s.it_.clone()) } == { _a0.with(|__s| __s.it_.clone()) });
    }
}
pub trait support_double_move_trackerImpl {
    fn move_assign(
        &self,
        other: Ptr<support_double_move_tracker>,
    ) -> Ptr<support_double_move_tracker>;
}
impl support_double_move_trackerImpl for Ptr<support_double_move_tracker> {
    fn move_assign(
        &self,
        other: Ptr<support_double_move_tracker>,
    ) -> Ptr<support_double_move_tracker> {
        assert!(!(other.with(|__s| __s.moved_from_)));
        field!(other, moved_from_).write(true);
        field!((*self), moved_from_).write(false);
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
