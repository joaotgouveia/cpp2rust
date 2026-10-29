extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
impl Default for support_double_move_tracker {
    fn default() -> Self {
        { support_double_move_tracker::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
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
pub fn operator_eq_0(
    x: Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>,
    y: Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_1(
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
impl CppEq for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub fn operator_eq_2(
    x: Ptr<forward_iterator_const_int_ptr_>,
    y: Ptr<forward_iterator_const_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_3(
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
impl CppEq for forward_iterator_const_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_2(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_eq_4(
    x: Ptr<bidirectional_iterator_const_int_ptr_>,
    y: Ptr<bidirectional_iterator_const_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_5(
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
impl CppEq for bidirectional_iterator_const_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_4(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_eq_6(
    x: Ptr<random_access_iterator_const_int_ptr_>,
    y: Ptr<random_access_iterator_const_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_7(
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
impl CppEq for random_access_iterator_const_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_6(__lhs.clone(), __rhs.clone())
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
const _: () = assert!(
    true,
    "std::random_access_iterator<cpp20_random_access_iterator<int*>>"
);
pub fn operator_eq_8(
    x: Ptr<contiguous_iterator_const_int_ptr_>,
    y: Ptr<contiguous_iterator_const_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_9(
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
impl CppEq for contiguous_iterator_const_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_8(__lhs.clone(), __rhs.clone())
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
pub fn operator_eq_10(
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
        operator_eq_10(__lhs.clone(), __rhs.clone())
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
pub fn operator_add_11(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_12(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_11(_i, n)
    });
}
pub fn iter_swap_13(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap!((a.ptr_).clone(), (b.ptr_).clone());
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_14(iter: adl_Iterator) -> Ptr<i32> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_15(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> bool {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_16(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> std::cmp::Ordering {
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
impl CppLess for adl_Iterator {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_16(__lhs.clone(), __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for adl_Iterator {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_15(__lhs.clone(), __rhs.clone())
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
const _: () = assert!(true, "std::random_access_iterator<rvalue_iterator<int*>>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub fn new(mut what: Ptr<i8>) -> Self {
        Self {}
    }
}
thread_local!(
    pub static compare_last_digit_strong_17: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> =
        Rc::new(RefCell::new(
            FnPtr::<fn(i32, i32) -> std::cmp::Ordering>::new(
                |mut a: i32, mut b: i32| -> std::cmp::Ordering {
                    {
                        return std::cmp::Ord::cmp(&(a % 10), &(b % 10));
                    }
                },
            ),
        ));
);
thread_local!(
    pub static compare_last_digit_weak_18: Value<FnPtr<fn(i32, i32) -> std_weak_ordering>> =
        Rc::new(RefCell::new(
            FnPtr::<fn(i32, i32) -> std_weak_ordering>::new(
                |mut a: i32, mut b: i32| -> std_weak_ordering {
                    {
                        return ({ std::cmp::Ord::cmp(&(a % 10), &(b % 10))() });
                    }
                },
            ),
        ));
);
thread_local!(
    pub static compare_last_digit_partial_19: Value<FnPtr<fn(i32, i32) -> std_partial_ordering>> =
        Rc::new(RefCell::new(
            FnPtr::<fn(i32, i32) -> std_partial_ordering>::new(
                |mut a: i32, mut b: i32| -> std_partial_ordering {
                    {
                        if (a == <i32>::MIN) || (b == <i32>::MIN) {
                            return (*unordered_20.with(Value::clone).borrow()).clone();
                        }
                        return ({ std::cmp::Ord::cmp(&(a % 10), &(b % 10))() });
                    }
                },
            ),
        ));
);
thread_local!(
    pub static compare_int_result_21: Value<FnPtr<fn(i32, i32) -> i32>> = Rc::new(RefCell::new(
        FnPtr::<fn(i32, i32) -> i32>::new(|mut a: i32, mut b: i32| -> i32 {
            {
                return ((a % 10) - (b % 10));
            }
        }),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct StructWithoutCallOperator {}
const _: () = assert!(
    true,
    "has_lexicographical_compare<decltype(compare_last_digit_strong)>"
);
const _: () = assert!(
    true,
    "has_lexicographical_compare<decltype(compare_last_digit_weak)>"
);
const _: () = assert!(
    true,
    "has_lexicographical_compare<decltype(compare_last_digit_partial)>"
);
const _: () = assert!(
    !(false),
    "!has_lexicographical_compare<StructWithoutCallOperator>"
);
const _: () = assert!(
    true,
    "has_lexicographical_compare<decltype(compare_int_result)>"
);
pub fn test_lexicographical_compare_22(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_23(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_24(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_23(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_25(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_23(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_26(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_23(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_27(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_23(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_28(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_23(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_29(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_30(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_31(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_30(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_32(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_30(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_33(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_30(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_34(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_30(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_35(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_30(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_36(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_37(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_38(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_37(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_39(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_37(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_40(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_37(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_41(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_37(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_42(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_37(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_43(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_44(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_45(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_44(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_46(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_44(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_47(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_44(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_48(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_44(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_49(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_44(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_50(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_51(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_52(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_51(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_53(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_51(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_54(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_51(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_55(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_51(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_56(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_51(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_57(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_58(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_59(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_58(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_60(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_58(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_61(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_58(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_62(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_58(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_63(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_58(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_64(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_65(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_66(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_65(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_67(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_65(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_68(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_65(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_69(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_65(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_70(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_65(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_71(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_72(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_73(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_72(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_74(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_72(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_75(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_72(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_76(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_72(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_77(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_72(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_78(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_79(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_80(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_79(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_81(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_79(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_82(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_79(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_83(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_79(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_84(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_79(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_85(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_86(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_87(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_86(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_88(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_86(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_89(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_86(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_90(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_86(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_91(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_86(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_92(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_93(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_94(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_93(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_95(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_93(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_96(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_93(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_97(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_93(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_98(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_93(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_99(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_100(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_101(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_100(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_102(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_100(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_103(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_100(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_104(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_100(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_105(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_100(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_106(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_107(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_108(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_107(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_109(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_107(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_110(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_107(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_111(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_107(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_112(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_107(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_113(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_114(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_115(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_114(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_116(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_114(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_117(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_114(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_118(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_114(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_119(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_114(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_120(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_121(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_122(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_121(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_123(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_121(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_124(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_121(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_125(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_121(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_126(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_121(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_127(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_128(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_129(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_128(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_130(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_128(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_131(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_128(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_132(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_128(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_133(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_128(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_134(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_135(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_136(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_135(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_137(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_135(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_138(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_135(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_139(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_135(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_140(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_135(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_141(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_142(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_143(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_142(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_144(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_142(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_145(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_142(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_146(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_142(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_147(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_142(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_148(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_149(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_150(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_149(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_151(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_149(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_152(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_149(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_153(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_149(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_154(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_149(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_155(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_156(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_157(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_156(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_158(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_156(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_159(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_156(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_160(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_156(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_161(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_156(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_162(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_163(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_164(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_163(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_165(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_163(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_166(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_163(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_167(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_163(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_168(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ a.decay() });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_163(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_169(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_170(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_171(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_170(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_172(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_170(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_173(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_170(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_174(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_170(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_175(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_170(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_176(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_177(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_178(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_177(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_179(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_177(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_180(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_177(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_181(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_177(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_182(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_177(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_183(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_184(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_185(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_184(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_186(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_184(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_187(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_184(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_188(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_184(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_189(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_184(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_190(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_191(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_192(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_191(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_193(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_191(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_194(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_191(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_195(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_191(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_196(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_191(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_197(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_198(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_199(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_198(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_200(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_198(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_201(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_198(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_202(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_198(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_203(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_198(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_204(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_205(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_206(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_205(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_207(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_205(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_208(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_205(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_209(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_205(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_210(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_205(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_211(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_212(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_213(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_212(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_214(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_212(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_215(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_212(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_216(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_212(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_217(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_212(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_218(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_219(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_220(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_219(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_221(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_219(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_222(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_219(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_223(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_219(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_224(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_219(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_225(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_226(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_227(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_226(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_228(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_226(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_229(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_226(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_230(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_226(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_231(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_226(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_232(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_233(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_234(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_233(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_235(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_233(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_236(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_233(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_237(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_233(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_238(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_233(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_239(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_240(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_241(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_240(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_242(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_240(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_243(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_240(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_244(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_240(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_245(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_240(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_246(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_247(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_248(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_247(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_249(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_247(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_250(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_247(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_251(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_247(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_252(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_247(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_253(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_254(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_255(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_254(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_256(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_254(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_257(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_254(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_258(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_254(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_259(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_254(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_260(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_261(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_262(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_261(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_263(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_261(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_264(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_261(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_265(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_261(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_266(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_261(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_267(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_268(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_269(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_268(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_270(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_268(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_271(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_268(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_272(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_268(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_273(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_268(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_274(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_275(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_276(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_275(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_277(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_275(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_278(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_275(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_279(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_275(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_280(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_275(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_281(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_282(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_283(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_282(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_284(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_282(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_285(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_282(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_286(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_282(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_287(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_282(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_288(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_289(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_290(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_289(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_291(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_289(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_292(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_289(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_293(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_289(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_294(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_289(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_295(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_296(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_297(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_296(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_298(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_296(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_299(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_296(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_300(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_296(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_301(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_296(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_302(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_303(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_304(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_303(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_305(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_303(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_306(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_303(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_307(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_303(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_308(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_303(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_309(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_310(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_311(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_310(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_312(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_310(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_313(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_310(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_314(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_310(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_315(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_310(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_316(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_317(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_318(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_317(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_319(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_317(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_320(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_317(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_321(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_317(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_322(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_317(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_323(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_324(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_325(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_324(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_326(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_324(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_327(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_324(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_328(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_324(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_329(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_324(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_330(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_331(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_332(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_331(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_333(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_331(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_334(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_331(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_335(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_331(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_336(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ b.decay() });
            let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_331(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_337(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_338(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_339(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_338(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_340(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_338(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_341(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_338(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_342(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_338(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_343(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_338(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_344(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_345(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_346(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_345(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_347(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_345(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_348(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_345(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_349(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_345(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_350(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_345(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_351(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_352(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_353(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_352(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_354(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_352(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_355(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_352(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_356(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_352(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_357(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_352(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_358(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_359(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_360(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_359(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_361(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_359(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_362(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_359(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_363(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_359(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_364(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ a.decay() });
            let ___last1: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    a.decay().offset(((*a.borrow()).len()) as isize)
                });
            let ___first2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ b.decay() });
            let ___last2: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    b.decay().offset(((*b.borrow()).len()) as isize)
                });
            lexicographical_compare_three_way_359(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_365(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_79(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_366(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_367(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_lexicographical_compare_368(
    a: Vec<i32>,
    b: Vec<i32>,
    comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    expected: std::cmp::Ordering,
) {
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Vec<i32>> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(comp));
    let expected: Value<std::cmp::Ordering> = Rc::new(RefCell::new(expected));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = a.decay();
            let ___last1: Ptr<i32> = a.decay().offset(((*a.borrow()).len()) as isize);
            let ___first2: Ptr<i32> = b.decay();
            let ___last2: Ptr<i32> = b.decay().offset(((*b.borrow()).len()) as isize);
            lexicographical_compare_three_way_367(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*comp.borrow()).copy_from(),
            )
        }),
    ));
    assert!((*expected.borrow()) == (*result.borrow()));
}
pub fn test_given_iterator_types_369() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_22(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_24(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_25(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_26(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_26(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_26(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_26(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_27(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_28(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_26(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_370() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_29(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_31(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_32(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_33(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_33(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_33(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_33(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_34(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_35(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_33(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_371() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_36(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_38(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_39(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_40(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_40(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_40(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_40(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_41(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_42(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_40(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_372() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_43(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_45(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_46(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_47(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_47(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_47(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_47(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_48(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_49(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_47(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_373() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_50(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_52(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_53(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_54(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_54(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_54(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_54(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_55(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_56(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_54(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_374() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_57(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_59(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_60(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_61(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_61(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_61(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_61(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_62(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_63(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_61(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_375() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_64(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_66(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_67(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_68(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_68(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_68(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_68(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_69(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_70(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_68(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_376() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_71(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_73(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_74(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_75(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_75(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_75(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_75(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_76(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_77(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_75(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_377() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_78(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_80(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_81(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_82(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_82(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_82(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_82(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_83(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_84(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_82(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_378() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_85(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_87(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_88(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_89(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_89(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_89(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_89(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_90(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_91(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_89(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_379() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_92(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_94(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_95(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_96(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_96(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_96(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_96(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_97(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_98(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_96(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_380() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_99(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_101(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_102(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_103(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_103(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_103(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_103(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_104(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_105(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_103(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_381() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_106(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_108(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_109(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_110(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_110(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_110(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_110(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_111(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_112(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_110(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_382() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_113(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_115(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_116(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_117(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_117(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_117(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_117(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_118(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_119(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_117(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_383() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_120(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_122(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_123(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_124(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_124(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_124(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_124(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_125(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_126(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_124(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_384() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_127(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_129(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_130(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_131(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_131(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_131(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_131(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_132(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_133(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_131(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_385() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_134(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_136(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_137(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_138(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_138(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_138(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_138(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_139(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_140(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_138(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_386() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_141(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_143(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_144(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_145(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_145(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_145(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_145(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_146(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_147(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_145(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_387() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_148(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_150(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_151(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_152(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_152(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_152(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_152(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_153(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_154(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_152(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_388() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_155(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_157(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_158(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_159(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_159(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_159(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_159(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_160(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_161(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_159(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_389() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_162(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_164(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_165(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_166(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_166(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_166(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_166(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_167(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_168(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_166(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_390() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_169(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_171(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_172(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_173(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_173(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_173(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_173(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_174(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_175(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_173(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_391() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_176(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_178(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_179(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_180(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_180(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_180(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_180(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_181(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_182(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_180(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_392() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_183(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_185(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_186(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_187(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_187(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_187(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_187(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_188(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_189(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_187(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_393() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_190(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_192(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_193(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_194(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_194(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_194(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_194(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_195(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_196(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_194(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_394() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_197(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_199(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_200(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_201(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_201(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_201(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_201(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_202(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_203(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_201(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_395() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_204(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_206(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_207(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_208(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_208(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_208(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_208(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_209(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_210(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_208(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_396() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_211(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_213(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_214(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_215(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_215(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_215(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_215(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_216(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_217(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_215(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_397() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_218(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_220(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_221(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_222(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_222(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_222(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_222(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_223(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_224(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_222(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_398() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_225(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_227(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_228(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_229(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_229(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_229(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_229(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_230(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_231(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_229(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_399() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_232(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_234(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_235(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_236(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_236(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_236(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_236(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_237(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_238(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_236(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_400() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_239(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_241(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_242(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_243(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_243(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_243(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_243(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_244(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_245(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_243(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_401() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_246(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_248(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_249(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_250(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_250(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_250(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_250(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_251(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_252(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_250(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_402() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_253(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_255(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_256(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_257(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_257(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_257(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_257(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_258(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_259(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_257(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_403() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_260(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_262(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_263(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_264(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_264(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_264(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_264(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_265(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_266(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_264(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_404() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_267(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_269(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_270(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_271(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_271(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_271(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_271(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_272(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_273(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_271(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_405() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_274(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_276(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_277(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_278(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_278(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_278(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_278(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_279(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_280(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_278(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_406() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_281(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_283(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_284(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_285(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_285(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_285(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_285(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_286(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_287(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_285(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_407() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_288(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_290(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_291(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_292(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_292(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_292(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_292(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_293(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_294(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_292(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_408() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_295(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_297(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_298(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_299(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_299(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_299(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_299(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_300(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_301(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_299(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_409() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_302(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_304(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_305(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_306(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_306(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_306(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_306(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_307(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_308(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_306(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_410() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_309(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_311(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_312(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_313(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_313(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_313(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_313(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_314(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_315(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_313(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_411() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_316(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_318(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_319(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_320(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_320(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_320(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_320(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_321(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_322(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_320(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_412() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_323(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_325(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_326(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_327(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_327(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_327(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_327(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_328(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_329(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_327(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_413() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_330(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_332(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_333(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_334(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_334(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_334(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_334(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_335(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_336(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_334(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_414() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_337(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_339(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_340(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_341(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_341(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_341(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_341(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_342(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_343(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_341(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_415() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_344(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_346(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_347(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_348(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_348(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_348(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_348(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_349(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_350(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_348(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_416() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_351(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_353(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_354(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_355(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_355(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_355(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_355(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_356(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_357(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_355(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_given_iterator_types_417() {
    let cmp: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_358(
            <Vec<i32>>::default(),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_360(
            <Vec<i32>>::default(),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_361(
            Vec::from([0, 1]),
            <Vec<i32>>::default(),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_362(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    ({
        test_lexicographical_compare_362(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_362(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_362(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_363(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_364(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    ({
        test_lexicographical_compare_362(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            (*cmp.borrow()).copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub fn test_iterator_types1_418() {
    ({ test_given_iterator_types_369() });
    ({ test_given_iterator_types_370() });
    ({ test_given_iterator_types_371() });
    ({ test_given_iterator_types_372() });
    ({ test_given_iterator_types_373() });
    ({ test_given_iterator_types_374() });
    ({ test_given_iterator_types_375() });
}
pub fn test_iterator_types1_419() {
    ({ test_given_iterator_types_376() });
    ({ test_given_iterator_types_377() });
    ({ test_given_iterator_types_378() });
    ({ test_given_iterator_types_379() });
    ({ test_given_iterator_types_380() });
    ({ test_given_iterator_types_381() });
    ({ test_given_iterator_types_382() });
}
pub fn test_iterator_types1_420() {
    ({ test_given_iterator_types_383() });
    ({ test_given_iterator_types_384() });
    ({ test_given_iterator_types_385() });
    ({ test_given_iterator_types_386() });
    ({ test_given_iterator_types_387() });
    ({ test_given_iterator_types_388() });
    ({ test_given_iterator_types_389() });
}
pub fn test_iterator_types1_421() {
    ({ test_given_iterator_types_390() });
    ({ test_given_iterator_types_391() });
    ({ test_given_iterator_types_392() });
    ({ test_given_iterator_types_393() });
    ({ test_given_iterator_types_394() });
    ({ test_given_iterator_types_395() });
    ({ test_given_iterator_types_396() });
}
pub fn test_iterator_types1_422() {
    ({ test_given_iterator_types_397() });
    ({ test_given_iterator_types_398() });
    ({ test_given_iterator_types_399() });
    ({ test_given_iterator_types_400() });
    ({ test_given_iterator_types_401() });
    ({ test_given_iterator_types_402() });
    ({ test_given_iterator_types_403() });
}
pub fn test_iterator_types1_423() {
    ({ test_given_iterator_types_404() });
    ({ test_given_iterator_types_405() });
    ({ test_given_iterator_types_406() });
    ({ test_given_iterator_types_407() });
    ({ test_given_iterator_types_408() });
    ({ test_given_iterator_types_409() });
    ({ test_given_iterator_types_410() });
}
pub fn test_iterator_types1_424() {
    ({ test_given_iterator_types_411() });
    ({ test_given_iterator_types_412() });
    ({ test_given_iterator_types_413() });
    ({ test_given_iterator_types_414() });
    ({ test_given_iterator_types_415() });
    ({ test_given_iterator_types_416() });
    ({ test_given_iterator_types_417() });
}
pub fn test_iterator_types_425() {
    ({ test_iterator_types1_418() });
    ({ test_iterator_types1_419() });
    ({ test_iterator_types1_420() });
    ({ test_iterator_types1_421() });
    ({ test_iterator_types1_422() });
    ({ test_iterator_types1_423() });
    ({ test_iterator_types1_424() });
}
pub fn test_comparison_categories_426() {
    ({
        test_lexicographical_compare_427(
            Vec::from([0, 1]),
            Vec::from([10, 11]),
            (*compare_last_digit_weak_18.with(Value::clone).borrow()).copy_from(),
            (*equivalent_428.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_lexicographical_compare_429(
            Vec::from([0, 1]),
            Vec::from([20, 11]),
            (*compare_last_digit_partial_19.with(Value::clone).borrow()).copy_from(),
            (*equivalent_430.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_lexicographical_compare_365(
            Vec::from([0]),
            Vec::from([0, 1]),
            (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    ({
        test_lexicographical_compare_431(
            Vec::from([0]),
            Vec::from([0, 1]),
            (*compare_last_digit_weak_18.with(Value::clone).borrow()).copy_from(),
            (*less_432.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_lexicographical_compare_433(
            Vec::from([0]),
            Vec::from([0, 1]),
            (*compare_last_digit_partial_19.with(Value::clone).borrow()).copy_from(),
            (*less_434.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_lexicographical_compare_435(
            Vec::from([<i32>::MIN, 1]),
            Vec::from([0, 1, 2]),
            (*compare_last_digit_partial_19.with(Value::clone).borrow()).copy_from(),
            (*unordered_20.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_comparator_invocation_count_436() {
    let compare_invocation_count: Value<i32> = Rc::new(RefCell::new(0));
    let compare_last_digit_counting: Value<FnPtr<fn(i32, i32) -> std::cmp::Ordering>> =
        Rc::new(RefCell::new(lambda!(
            {
                let compare_invocation_count: Ptr<i32> = compare_invocation_count.as_pointer();
            },
            |mut a: i32, mut b: i32| -> std::cmp::Ordering {
                compare_invocation_count.with_mut(|__v| __v.prefix_inc());
                return std::cmp::Ord::cmp(&(a % 10), &(b % 10));
            }
        )));
    (*compare_invocation_count.borrow_mut()) = 0;
    ({
        test_lexicographical_compare_366(
            Vec::from([0, 1, 2, 3]),
            <Vec<i32>>::default(),
            (*compare_last_digit_counting.borrow()).copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    assert!(((*compare_invocation_count.borrow()) == 0));
    ({
        test_lexicographical_compare_368(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1, 2, 3]),
            (*compare_last_digit_counting.borrow()).copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    assert!(((*compare_invocation_count.borrow()) <= 6));
}
pub fn test_proxy_iterators_437() {
    let vec_: Value<Vec<bool>> = Rc::new(RefCell::new(
        (0..(10_usize) as usize)
            .map(|_| true)
            .collect::<Vec<bool>>(),
    ));
    let result: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        ({
            let ___first1: RandomAccessIterator<bool> = vec_.decay().begin();
            let ___last1: RandomAccessIterator<bool> = vec_.decay().end();
            let ___first2: RandomAccessIterator<bool> = vec_.decay().begin();
            let ___last2: RandomAccessIterator<bool> = vec_.decay().end();
            lexicographical_compare_three_way_438(
                ___first1,
                ___last1,
                ___first2,
                ___last2,
                (*compare_last_digit_strong_17.with(Value::clone).borrow()).copy_from(),
            )
        }),
    ));
    assert!((*result.borrow()) == std::cmp::Ordering::Equal);
}
pub fn test_439() -> bool {
    ({ test_iterator_types_425() });
    ({ test_comparison_categories_426() });
    ({ test_comparator_invocation_count_436() });
    ({ test_proxy_iterators_437() });
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
    ({ test_439() });
    const _: () = assert!(true, "test()");;
    return 0;
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
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_post_inc_15(&self, mut _a0: i32) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return prev;
    }
    fn operator_dec_16(&self) -> Ptr<adl_Iterator> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_post_dec_17(&self, mut _a0: i32) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
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
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
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
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
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
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
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
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
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
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
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
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut());
            }
        }
        return std::cmp::Ordering::Equal;
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
pub fn __cpp2rust_init_globals() {
    let _ = compare_last_digit_strong_17.with(|_| ());
    let _ = compare_last_digit_weak_18.with(|_| ());
    let _ = compare_last_digit_partial_19.with(|_| ());
    let _ = compare_int_result_21.with(|_| ());
}
