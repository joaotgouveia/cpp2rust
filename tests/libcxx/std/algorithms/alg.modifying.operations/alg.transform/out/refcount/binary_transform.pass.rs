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
pub fn base_0(i: Ptr<cpp17_output_iterator_int_ptr_>) -> Ptr<i32> {
    return i.with(|__s| __s.it_.clone());
}
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
    pub fn new(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
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
pub fn base_1(i: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>) -> Ptr<i32> {
    return i.with(|__s| __s.it_.clone());
}
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
    pub fn new(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
pub fn operator_ne_2(
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
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub fn base_3(i: Ptr<forward_iterator_int_ptr_>) -> Ptr<i32> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct forward_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl forward_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<forward_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
pub fn operator_ne_4(
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
pub fn base_5(i: Ptr<bidirectional_iterator_int_ptr_>) -> Ptr<i32> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct bidirectional_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<bidirectional_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
pub fn operator_ne_6(
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
pub fn base_7(i: Ptr<random_access_iterator_int_ptr_>) -> Ptr<i32> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<random_access_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
pub fn operator_ne_8(
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
pub fn base_9(i: Ptr<i32>) -> Ptr<i32> {
    let i: Value<Ptr<i32>> = Rc::new(RefCell::new(i));
    return (*i.borrow()).clone();
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
pub fn test_constexpr_17() -> bool {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 6, 7])));
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([2, 4, 7, 8])));
    let ic: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 0, 0])));
    let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([3, 7, 13, 15])));
    let mut it: Ptr<i32> = ({
        let ___first1: Ptr<i32> = ({ begin_18(ia.as_pointer()) });
        let ___last1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).to_end();
        let ___first2: Ptr<i32> = ({ begin_18(ib.as_pointer()) });
        transform_19(
            ___first1,
            ___last1,
            ___first2,
            ({ begin_20(ic.as_pointer()) }),
            <std_plus_int_>::default(),
        )
    });
    return (({ (it).clone() } == {
        (({ begin_20(ic.as_pointer()) }).offset(({ size_21(ia.as_pointer()) }) as isize))
    }) && ((it.read()) == 0))
        && ({
            let ___first1: Ptr<i32> = ({ begin_18(expected.as_pointer()) });
            let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
            equal_22(
                ___first1,
                ___last1,
                ({ begin_20(ic.as_pointer()) }),
                (it).clone(),
            )
        });
}
pub fn test_23() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_24(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_25() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_26(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_27() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_28(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_29() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_30(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_31() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_32(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_33() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_pointer() as Ptr<i32>)
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_34(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            }),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_35() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_36(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_37() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_38(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_39() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_40(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_41() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_42(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_43() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_44(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_45() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_pointer() as Ptr<i32>)
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_46(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_47() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_48(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_49() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_50(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_51() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_52(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_53() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_54(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_55() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_56(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_57() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_pointer() as Ptr<i32>)
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_58(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_59() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_60(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_61() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_62(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_63() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_64(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_65() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_66(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_67() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_68(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_69() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_pointer() as Ptr<i32>)
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_70(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_71() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_72(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_73() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_74(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_75() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_76(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_77() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_78(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_79() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>)
                });
            let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_80(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_81() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_pointer() as Ptr<i32>)
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_82(
            ___first1,
            ___last1,
            (ia.as_pointer() as Ptr<i32>),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_83() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_84(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_85() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_86(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_87() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_88(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_89() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_90(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_91() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_92(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_93() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        });
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_94(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            }),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_95() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_96(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_97() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_98(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_99() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_100(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_101() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_102(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_103() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_104(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_105() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        });
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_106(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_107() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_108(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_109() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_110(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_111() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_112(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_113() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_114(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_115() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_116(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_117() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        });
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_118(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_119() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_120(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_121() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_122(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_123() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_124(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_125() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_126(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_127() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_128(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_129() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        });
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_130(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_131() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_132(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_133() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_134(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_135() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_136(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_137() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_138(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_139() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_140(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_141() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        });
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_142(
            ___first1,
            ___last1,
            (ia.as_pointer() as Ptr<i32>),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_143() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_144(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_145() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_146(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_147() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_148(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_149() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_150(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_151() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_152(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_153() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_154(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            }),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_155() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_156(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_157() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_158(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_159() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_160(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_161() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_162(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_163() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_164(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_165() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_166(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_167() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_168(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_169() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_170(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_171() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_172(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_173() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_174(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_175() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_176(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_177() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_178(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_179() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_180(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_181() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_182(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_183() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_184(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_185() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_186(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_187() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_188(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_189() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_190(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_191() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_192(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_193() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_194(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_195() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_196(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_197() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_198(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_199() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_200(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_201() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_202(
            ___first1,
            ___last1,
            (ia.as_pointer() as Ptr<i32>),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_203() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_204(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_205() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_206(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_207() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_208(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_209() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_210(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_211() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_212(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_213() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_214(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            }),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_215() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_216(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_217() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_218(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_219() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_220(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_221() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_222(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_223() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_224(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_225() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_226(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_227() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_228(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_229() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_230(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_231() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_232(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_233() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_234(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_235() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_236(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_237() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_238(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_239() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_240(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_241() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_242(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_243() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_244(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_245() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_246(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_247() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_248(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_249() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_250(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_251() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_252(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_253() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_254(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_255() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_256(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_257() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_258(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_259() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            let ___last1: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
                });
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_260(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_261() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_262(
            ___first1,
            ___last1,
            (ia.as_pointer() as Ptr<i32>),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_263() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_264(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_265() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_266(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_267() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_268(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_269() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_270(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_271() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_272(
                ___first1,
                ___last1,
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_273() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_274(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            }),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_275() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_276(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_277() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_278(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_279() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_280(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_281() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_282(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_283() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_284(
                ___first1,
                ___last1,
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_285() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_286(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_287() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_288(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_289() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_290(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_291() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_292(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_293() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_294(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_295() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_296(
                ___first1,
                ___last1,
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_297() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_298(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_299() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_300(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_301() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_302(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_303() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_304(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_305() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_306(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_307() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_308(
                ___first1,
                ___last1,
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_309() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_310(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_311() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: cpp17_output_iterator_int_ptr_ =
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_312(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_0(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_313() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<cpp17_input_iterator_int_ptr__int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
                cpp17_input_iterator_int_ptr__int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) });
            transform_314(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_1(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_315() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_316(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_3(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_317() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_318(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_5(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_319() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        ({
            let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
            let ___result: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) });
            transform_320(
                ___first1,
                ___last1,
                (ia.as_pointer() as Ptr<i32>),
                ___result,
                <std_minus_int_>::default(),
            )
        }),
    ));
    assert!(
        (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
}
pub fn test_321() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut r: Ptr<i32> = ({
        let ___first1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        let ___last1: Ptr<i32> = (ib.as_pointer() as Ptr<i32>).offset((sa) as isize);
        let ___result: Ptr<i32> = (ib.as_pointer() as Ptr<i32>);
        transform_322(
            ___first1,
            ___last1,
            (ia.as_pointer() as Ptr<i32>),
            ___result,
            <std_minus_int_>::default(),
        )
    });
    assert!(
        ({ ({ base_9((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(((*ib.borrow())[(0) as usize] == 1));
    assert!(((*ib.borrow())[(1) as usize] == 1));
    assert!(((*ib.borrow())[(2) as usize] == 1));
    assert!(((*ib.borrow())[(3) as usize] == 1));
    assert!(((*ib.borrow())[(4) as usize] == 1));
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
    ({ test_23() });
    ({ test_25() });
    ({ test_27() });
    ({ test_29() });
    ({ test_31() });
    ({ test_33() });
    ({ test_35() });
    ({ test_37() });
    ({ test_39() });
    ({ test_41() });
    ({ test_43() });
    ({ test_45() });
    ({ test_47() });
    ({ test_49() });
    ({ test_51() });
    ({ test_53() });
    ({ test_55() });
    ({ test_57() });
    ({ test_59() });
    ({ test_61() });
    ({ test_63() });
    ({ test_65() });
    ({ test_67() });
    ({ test_69() });
    ({ test_71() });
    ({ test_73() });
    ({ test_75() });
    ({ test_77() });
    ({ test_79() });
    ({ test_81() });
    ({ test_83() });
    ({ test_85() });
    ({ test_87() });
    ({ test_89() });
    ({ test_91() });
    ({ test_93() });
    ({ test_95() });
    ({ test_97() });
    ({ test_99() });
    ({ test_101() });
    ({ test_103() });
    ({ test_105() });
    ({ test_107() });
    ({ test_109() });
    ({ test_111() });
    ({ test_113() });
    ({ test_115() });
    ({ test_117() });
    ({ test_119() });
    ({ test_121() });
    ({ test_123() });
    ({ test_125() });
    ({ test_127() });
    ({ test_129() });
    ({ test_131() });
    ({ test_133() });
    ({ test_135() });
    ({ test_137() });
    ({ test_139() });
    ({ test_141() });
    ({ test_143() });
    ({ test_145() });
    ({ test_147() });
    ({ test_149() });
    ({ test_151() });
    ({ test_153() });
    ({ test_155() });
    ({ test_157() });
    ({ test_159() });
    ({ test_161() });
    ({ test_163() });
    ({ test_165() });
    ({ test_167() });
    ({ test_169() });
    ({ test_171() });
    ({ test_173() });
    ({ test_175() });
    ({ test_177() });
    ({ test_179() });
    ({ test_181() });
    ({ test_183() });
    ({ test_185() });
    ({ test_187() });
    ({ test_189() });
    ({ test_191() });
    ({ test_193() });
    ({ test_195() });
    ({ test_197() });
    ({ test_199() });
    ({ test_201() });
    ({ test_203() });
    ({ test_205() });
    ({ test_207() });
    ({ test_209() });
    ({ test_211() });
    ({ test_213() });
    ({ test_215() });
    ({ test_217() });
    ({ test_219() });
    ({ test_221() });
    ({ test_223() });
    ({ test_225() });
    ({ test_227() });
    ({ test_229() });
    ({ test_231() });
    ({ test_233() });
    ({ test_235() });
    ({ test_237() });
    ({ test_239() });
    ({ test_241() });
    ({ test_243() });
    ({ test_245() });
    ({ test_247() });
    ({ test_249() });
    ({ test_251() });
    ({ test_253() });
    ({ test_255() });
    ({ test_257() });
    ({ test_259() });
    ({ test_261() });
    ({ test_263() });
    ({ test_265() });
    ({ test_267() });
    ({ test_269() });
    ({ test_271() });
    ({ test_273() });
    ({ test_275() });
    ({ test_277() });
    ({ test_279() });
    ({ test_281() });
    ({ test_283() });
    ({ test_285() });
    ({ test_287() });
    ({ test_289() });
    ({ test_291() });
    ({ test_293() });
    ({ test_295() });
    ({ test_297() });
    ({ test_299() });
    ({ test_301() });
    ({ test_303() });
    ({ test_305() });
    ({ test_307() });
    ({ test_309() });
    ({ test_311() });
    ({ test_313() });
    ({ test_315() });
    ({ test_317() });
    ({ test_319() });
    ({ test_321() });
    const _: () = assert!(true, "test_constexpr()");;
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
pub trait bidirectional_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_int_ptr_>;
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_8(&self, mut _a0: i32) -> bidirectional_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_9(&self, mut _a0: i32) -> bidirectional_iterator_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_int_ptr_>,
    ) -> Ptr<bidirectional_iterator_int_ptr_>;
}
impl bidirectional_iterator_int_ptr_Impl for Ptr<bidirectional_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_int_ptr_>,
    ) -> Ptr<bidirectional_iterator_int_ptr_> {
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
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_>;
    fn operator_post_inc_6(&self, mut _a0: i32) -> cpp17_input_iterator_int_ptr__int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>,
    ) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_>;
}
impl cpp17_input_iterator_int_ptr__int_ptr_Impl for Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
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
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_5(&self) -> Ptr<cpp17_output_iterator_int_ptr_>;
    fn operator_post_inc_6(&self, mut _a0: i32) -> cpp17_output_iterator_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_output_iterator_int_ptr_>,
    ) -> Ptr<cpp17_output_iterator_int_ptr_>;
}
impl cpp17_output_iterator_int_ptr_Impl for Ptr<cpp17_output_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_output_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
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
pub trait forward_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_6(&self) -> Ptr<forward_iterator_int_ptr_>;
    fn operator_post_inc_7(&self, mut _a0: i32) -> forward_iterator_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(&self, _a0: Ptr<forward_iterator_int_ptr_>) -> Ptr<forward_iterator_int_ptr_>;
}
impl forward_iterator_int_ptr_Impl for Ptr<forward_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<forward_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<forward_iterator_int_ptr_>) -> Ptr<forward_iterator_int_ptr_> {
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
pub trait random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: i32) -> random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: i32) -> random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_int_ptr_>,
    ) -> Ptr<random_access_iterator_int_ptr_>;
}
impl random_access_iterator_int_ptr_Impl for Ptr<random_access_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_int_ptr_>,
    ) -> Ptr<random_access_iterator_int_ptr_> {
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
pub fn __cpp2rust_init_globals() {}
