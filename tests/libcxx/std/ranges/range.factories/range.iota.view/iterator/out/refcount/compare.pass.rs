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
    moved_from_: (),
}
impl support_double_move_tracker {
    pub fn new() -> Self {
        Self { moved_from_: false }
    }
    pub fn move_from(other: Ptr<support_double_move_tracker>) -> Self {
        let __this: support_double_move_tracker = Self { moved_from_: false };
        unimplemented!();
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp17_output_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub fn operator_eq_0(
    x: Ptr<random_access_iterator_int_ptr_>,
    y: Ptr<random_access_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_lt_1(
    x: Ptr<random_access_iterator_int_ptr_>,
    y: Ptr<random_access_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } < { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor)]
#[byte_size(16)]
pub struct random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptr_ {
    pub fn new_1() -> Self {
        Self {
            it_: Ptr::<()>::null(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn new_2(mut it: Ptr<()>) -> Self {
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
impl std::cmp::Ord for random_access_iterator_int_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_1(
                Rc::new(RefCell::new(random_access_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: self.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
                Rc::new(RefCell::new(random_access_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: other.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_1(
                Rc::new(RefCell::new(random_access_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: other.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
                Rc::new(RefCell::new(random_access_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: self.tracker_.moved_from_.clone(),
                    },
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
impl std::cmp::PartialOrd for random_access_iterator_int_ptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for random_access_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_0(
                Rc::new(RefCell::new(random_access_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: self.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
                Rc::new(RefCell::new(random_access_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: other.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_int_ptr_ {}
impl Default for random_access_iterator_int_ptr_ {
    fn default() -> Self {
        { random_access_iterator_int_ptr_::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_random_access_iterator_int_ptr_ {}
const _: () = assert!(
    true,
    "std::random_access_iterator<cpp20_random_access_iterator<int*>>"
);
pub fn operator_cmp_2(
    x: Ptr<three_way_contiguous_iterator_int_ptr_>,
    y: Ptr<three_way_contiguous_iterator_int_ptr_>,
) -> () {
    return std::cmp::Ord::cmp(
        &(x.with(|__s| __s.it_.clone())),
        &(y.with(|__s| __s.it_.clone())),
    );
}
pub fn operator_eq_3(
    x: Ptr<three_way_contiguous_iterator_int_ptr_>,
    y: Ptr<three_way_contiguous_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor)]
#[byte_size(16)]
pub struct three_way_contiguous_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl three_way_contiguous_iterator_int_ptr_ {
    pub fn new_1() -> Self {
        Self {
            it_: Ptr::<()>::null(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn new_2(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<three_way_contiguous_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::Ord for three_way_contiguous_iterator_int_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_2(
                Rc::new(RefCell::new(three_way_contiguous_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: self.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
                Rc::new(RefCell::new(three_way_contiguous_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: other.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for three_way_contiguous_iterator_int_ptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for three_way_contiguous_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_3(
                Rc::new(RefCell::new(three_way_contiguous_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: self.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
                Rc::new(RefCell::new(three_way_contiguous_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: other.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for three_way_contiguous_iterator_int_ptr_ {}
impl Default for three_way_contiguous_iterator_int_ptr_ {
    fn default() -> Self {
        { three_way_contiguous_iterator_int_ptr_::new_1() }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_input_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
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
    it_: Ptr<()>,
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
    pub increments: (),
    #[offset(8)]
    pub decrements: (),
    #[offset(16)]
    pub zero_moves: (),
    #[offset(24)]
    pub equal_cmps: (),
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        {
            IteratorOpCounts {
                increments: 0_(),
                decrements: 0_(),
                zero_moves: 0_(),
                equal_cmps: 0_(),
            }
        }
    }
}
pub fn operator_eq_4(
    _a0: Ptr<forward_sized_iterator_int_ptr_>,
    _a1: Ptr<forward_sized_iterator_int_ptr_>,
) -> () {
    return ({ _a0.with(|__s| __s.it_.clone()) } == { _a1.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct forward_sized_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    pub it_: Ptr<()>,
}
impl forward_sized_iterator_int_ptr_ {}
impl std::cmp::PartialEq for forward_sized_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_4(
                Rc::new(RefCell::new(forward_sized_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(forward_sized_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for forward_sized_iterator_int_ptr_ {}
impl Default for forward_sized_iterator_int_ptr_ {
    fn default() -> Self {
        {
            forward_sized_iterator_int_ptr_ {
                it_: Ptr::<()>::null(),
            }
        }
    }
}
const _: () = assert!(true, "std::forward_iterator<forward_sized_iterator<>>");
const _: () = assert!(
    true,
    "std::sized_sentinel_for<forward_sized_iterator<>, forward_sized_iterator<>>"
);
pub fn operator_add_5(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_6(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_5(_i, n)
    });
}
pub fn iter_swap_7(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_8(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return (unimplemented!()).reinterpret_cast::<()>();
}
pub fn operator_eq_9(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_10(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
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
    ptr_: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    iter_moves_: Ptr<()>,
    #[offset(16)]
    #[byte_size(8)]
    iter_swaps_: Ptr<()>,
}
impl adl_Iterator {
    fn new_1(mut p: Ptr<()>, mut iter_moves: Ptr<()>, mut iter_swaps: Ptr<()>) -> Self {
        Self {
            ptr_: (p).clone(),
            iter_moves_: (iter_moves).clone(),
            iter_swaps_: (iter_swaps).clone(),
        }
    }
    pub fn TrackMoves(mut p: Ptr<()>, iter_moves: Ptr<()>) -> adl_Iterator {
        return adl_Iterator::new_1({ (p).clone() }, { (iter_moves).clone() }, {
            Ptr::<()>::null()
        });
    }
    pub fn TrackSwaps_4(iter_swaps: Ptr<()>) -> adl_Iterator {
        return adl_Iterator::new_1({ Ptr::<()>::null() }, { Ptr::<()>::null() }, {
            (iter_swaps).clone()
        });
    }
    pub fn TrackSwaps_5(mut p: Ptr<()>, iter_swaps: Ptr<()>) -> adl_Iterator {
        return adl_Iterator::new_1({ (p).clone() }, { Ptr::<()>::null() }, {
            (iter_swaps).clone()
        });
    }
}
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_10(
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: self.ptr_.clone(),
                    iter_moves_: self.iter_moves_.clone(),
                    iter_swaps_: self.iter_swaps_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: other.ptr_.clone(),
                    iter_moves_: other.iter_moves_.clone(),
                    iter_swaps_: other.iter_swaps_.clone(),
                }))
                .as_pointer(),
            )
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
            operator_eq_9(
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: self.ptr_.clone(),
                    iter_moves_: self.iter_moves_.clone(),
                    iter_swaps_: self.iter_swaps_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: other.ptr_.clone(),
                    iter_moves_: other.iter_moves_.clone(),
                    iter_swaps_: other.iter_swaps_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for adl_Iterator {}
impl Default for adl_Iterator {
    fn default() -> Self {
        {
            adl_Iterator {
                ptr_: Ptr::<()>::null(),
                iter_moves_: Ptr::<()>::null(),
                iter_swaps_: Ptr::<()>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct rvalue_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<Ptr<()>>,
}
impl rvalue_iterator_int_ptr_ {}
impl std::cmp::Ord for rvalue_iterator_int_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            rvalue_iterator_int_ptr_Impl::operator_cmp(
                &Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for rvalue_iterator_int_ptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for rvalue_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            rvalue_iterator_int_ptr_Impl::operator_cmp(
                &Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for rvalue_iterator_int_ptr_ {}
const _: () = assert!(true, "std::random_access_iterator<rvalue_iterator<int*>>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub fn new(mut what: Ptr<()>) -> Self {
        Self {}
    }
}
pub fn operator_add_assign_11(lhs: Ptr<SomeInt>, rhs: Ptr<SomeInt>) -> Ptr<SomeInt> {
    {
        let __rhs = { rhs.with(|__s| __s.value_) };
        field!(lhs, value_).with_mut(|__v| *__v = *__v + __rhs)
    };
    return (lhs).clone();
}
pub fn operator_sub_assign_12(lhs: Ptr<SomeInt>, rhs: Ptr<SomeInt>) -> Ptr<SomeInt> {
    {
        let __rhs = { rhs.with(|__s| __s.value_) };
        field!(lhs, value_).with_mut(|__v| *__v = *__v - __rhs)
    };
    return (lhs).clone();
}
pub fn operator_add_assign_13(lhs: Ptr<SomeInt>, mut rhs: ()) -> Ptr<SomeInt> {
    {
        let __rhs = rhs;
        field!(lhs, value_).with_mut(|__v| *__v = *__v + __rhs)
    };
    return (lhs).clone();
}
pub fn operator_sub_assign_14(lhs: Ptr<SomeInt>, mut rhs: ()) -> Ptr<SomeInt> {
    {
        let __rhs = rhs;
        field!(lhs, value_).with_mut(|__v| *__v = *__v - __rhs)
    };
    return (lhs).clone();
}
pub fn operator_add_15(mut lhs: SomeInt, mut rhs: SomeInt) -> SomeInt {
    return SomeInt::new({ Some((lhs.value_ + rhs.value_)) });
}
pub fn operator_sub_16(mut lhs: SomeInt, mut rhs: SomeInt) -> () {
    return (lhs.value_ - rhs.value_);
}
pub fn operator_add_17(mut lhs: SomeInt, mut rhs: ()) -> SomeInt {
    return SomeInt::new({ Some((lhs.value_ + rhs)) });
}
pub fn operator_sub_18(mut lhs: SomeInt, mut rhs: ()) -> () {
    return (lhs.value_ - rhs);
}
pub fn operator_add_19(mut lhs: (), mut rhs: SomeInt) -> SomeInt {
    return SomeInt::new({ Some((lhs + rhs.value_)) });
}
pub fn operator_sub_20(mut lhs: (), mut rhs: SomeInt) -> () {
    return (lhs - rhs.value_);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct SomeInt {
    #[offset(0)]
    pub value_: (),
}
impl SomeInt {
    pub fn new(value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| 0_());
        Self { value_: value }
    }
}
impl std::cmp::Ord for SomeInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            SomeIntImpl::operator_cmp(
                &Rc::new(RefCell::new(SomeInt {
                    value_: self.value_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(SomeInt {
                    value_: other.value_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for SomeInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for SomeInt {
    fn eq(&self, other: &Self) -> bool {
        {
            SomeIntImpl::operator_cmp(
                &Rc::new(RefCell::new(SomeInt {
                    value_: self.value_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(SomeInt {
                    value_: other.value_.clone(),
                }))
                .as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for SomeInt {}
impl Default for SomeInt {
    fn default() -> Self {
        { SomeInt::new(None) }
    }
}
pub fn operator_add_assign_21(
    lhs: Ptr<NotIncrementable>,
    rhs: Ptr<NotIncrementable>,
) -> Ptr<NotIncrementable> {
    {
        let __rhs = { rhs.with(|__s| __s.value_) };
        field!(lhs, value_).with_mut(|__v| *__v = *__v + __rhs)
    };
    return (lhs).clone();
}
pub fn operator_sub_assign_22(
    lhs: Ptr<NotIncrementable>,
    rhs: Ptr<NotIncrementable>,
) -> Ptr<NotIncrementable> {
    {
        let __rhs = { rhs.with(|__s| __s.value_) };
        field!(lhs, value_).with_mut(|__v| *__v = *__v - __rhs)
    };
    return (lhs).clone();
}
pub fn operator_add_23(mut lhs: NotIncrementable, mut rhs: NotIncrementable) -> NotIncrementable {
    return NotIncrementable::new({ Some((lhs.value_ + rhs.value_)) });
}
pub fn operator_sub_24(mut lhs: NotIncrementable, mut rhs: NotIncrementable) -> () {
    return (lhs.value_ - rhs.value_);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct NotIncrementable {
    #[offset(0)]
    pub value_: (),
}
impl NotIncrementable {
    pub fn new(value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| 0_());
        Self { value_: value }
    }
}
impl std::cmp::PartialEq for NotIncrementable {
    fn eq(&self, other: &Self) -> bool {
        {
            NotIncrementableImpl::operator_eq(
                &Rc::new(RefCell::new(NotIncrementable {
                    value_: self.value_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(NotIncrementable {
                    value_: other.value_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for NotIncrementable {}
impl Default for NotIncrementable {
    fn default() -> Self {
        { NotIncrementable::new(None) }
    }
}
const _: () = assert!(!(false), "!std::incrementable<NotIncrementable>");
pub fn operator_add_assign_25(
    lhs: Ptr<NotDecrementable>,
    rhs: Ptr<NotDecrementable>,
) -> Ptr<NotDecrementable> {
    {
        let __rhs = { rhs.with(|__s| __s.value_) };
        field!(lhs, value_).with_mut(|__v| *__v = *__v + __rhs)
    };
    return (lhs).clone();
}
pub fn operator_sub_assign_26(
    lhs: Ptr<NotDecrementable>,
    rhs: Ptr<NotDecrementable>,
) -> Ptr<NotDecrementable> {
    {
        let __rhs = { rhs.with(|__s| __s.value_) };
        field!(lhs, value_).with_mut(|__v| *__v = *__v - __rhs)
    };
    return (lhs).clone();
}
pub fn operator_add_27(mut lhs: NotDecrementable, mut rhs: NotDecrementable) -> NotDecrementable {
    return NotDecrementable::new({ Some((lhs.value_ + rhs.value_)) });
}
pub fn operator_sub_28(mut lhs: NotDecrementable, mut rhs: NotDecrementable) -> () {
    return (lhs.value_ - rhs.value_);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct NotDecrementable {
    #[offset(0)]
    pub value_: (),
}
impl NotDecrementable {
    pub fn new(value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| 0_());
        Self { value_: value }
    }
}
impl std::cmp::PartialEq for NotDecrementable {
    fn eq(&self, other: &Self) -> bool {
        {
            NotDecrementableImpl::operator_eq(
                &Rc::new(RefCell::new(NotDecrementable {
                    value_: self.value_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(NotDecrementable {
                    value_: other.value_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for NotDecrementable {}
impl Default for NotDecrementable {
    fn default() -> Self {
        { NotDecrementable::new(None) }
    }
}
pub type CtorKind = ();
pub const CtorKind_DefaultTo42: CtorKind = 0;
pub const CtorKind_ValueCtor: CtorKind = 1;
pub fn test_29() -> () {
    {
        const _: () = assert!(
            true,
            "std::three_way_comparable<std::ranges::iterator_t<R>>"
        );;
        let r: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<()> = Rc::new(RefCell::new(42_()));
                (*iota_30.with(Value::clone).borrow_mut())(___start.as_pointer())
            })
            .clone(),
        ));
        let iter1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let iter2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "std::three_way_comparable<It>");;
        const _: () = assert!(
            true,
            "std::three_way_comparable<std::ranges::iterator_t<R>>"
        );;
        let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
        let r: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<three_way_contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(three_way_contiguous_iterator_int_ptr_::new_2(
                        { (a.as_pointer() as Ptr<()>) },
                    )));
                (*iota_30.with(Value::clone).borrow_mut())(___start.as_pointer())
            })
            .clone(),
        ));
        let iter1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let iter2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(!(false), "!std::three_way_comparable<It>");;
        const _: () = assert!(
            !(false),
            "!std::three_way_comparable<std::ranges::iterator_t<R>>"
        );;
        let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
        let r: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_2({
                        (a.as_pointer() as Ptr<()>)
                    })));
                (*iota_30.with(Value::clone).borrow_mut())(___start.as_pointer())
            })
            .clone(),
        ));
        let iter1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let iter2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_29() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait NotDecrementableImpl {
    fn operator_eq(&self, _a0: Ptr<NotDecrementable>) -> ();
    fn operator_inc_3(&self) -> Ptr<NotDecrementable>;
    fn operator_post_inc_4(&self, _a0: ());
}
impl NotDecrementableImpl for Ptr<NotDecrementable> {
    fn operator_eq(&self, _a0: Ptr<NotDecrementable>) -> () {
        return ({ (*self).with(|__s| __s.value_) } == { _a0.with(|__s| __s.value_) });
    }
    fn operator_inc_3(&self) -> Ptr<NotDecrementable> {
        field!((*self), value_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_post_inc_4(&self, mut _a0: ()) {
        field!((*self), value_).with_mut(|__v| __v.prefix_inc());
    }
}
pub trait NotIncrementableImpl {
    fn operator_eq(&self, _a0: Ptr<NotIncrementable>) -> ();
    fn operator_inc_3(&self) -> Ptr<NotIncrementable>;
    fn operator_post_inc_4(&self, _a0: ());
    fn operator_dec(&self) -> Ptr<NotIncrementable>;
}
impl NotIncrementableImpl for Ptr<NotIncrementable> {
    fn operator_eq(&self, _a0: Ptr<NotIncrementable>) -> () {
        return ({ (*self).with(|__s| __s.value_) } == { _a0.with(|__s| __s.value_) });
    }
    fn operator_inc_3(&self) -> Ptr<NotIncrementable> {
        field!((*self), value_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_post_inc_4(&self, mut _a0: ()) {
        field!((*self), value_).with_mut(|__v| __v.prefix_inc());
    }
    fn operator_dec(&self) -> Ptr<NotIncrementable> {
        field!((*self), value_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
}
pub trait SomeIntImpl {
    fn operator_cmp(&self, _a0: Ptr<SomeInt>) -> ();
    fn operator_inc_3(&self) -> Ptr<SomeInt>;
    fn operator_post_inc_4(&self, _a0: ()) -> SomeInt;
    fn operator_dec_5(&self) -> Ptr<SomeInt>;
    fn operator_post_dec_6(&self, _a0: ()) -> SomeInt;
    fn operator_eq(&self, _a0: Ptr<SomeInt>) -> ();
}
impl SomeIntImpl for Ptr<SomeInt> {
    fn operator_cmp(&self, _a0: Ptr<SomeInt>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.value_)),
                &(_a0.with(|__s| __s.value_)),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    fn operator_inc_3(&self) -> Ptr<SomeInt> {
        field!((*self), value_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_post_inc_4(&self, mut _a0: ()) -> SomeInt {
        let mut tmp: SomeInt = (*(*self).upgrade().deref()).clone();
        field!((*self), value_).with_mut(|__v| __v.prefix_inc());
        return tmp;
    }
    fn operator_dec_5(&self) -> Ptr<SomeInt> {
        field!((*self), value_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_post_dec_6(&self, mut _a0: ()) -> SomeInt {
        let mut tmp: SomeInt = (*(*self).upgrade().deref()).clone();
        field!((*self), value_).with_mut(|__v| __v.prefix_dec());
        return tmp;
    }
    fn operator_eq(&self, _a0: Ptr<SomeInt>) -> () {
        return ({ (*self).with(|__s| __s.value_) } == { _a0.with(|__s| __s.value_) });
    }
}
pub trait adl_IteratorImpl {
    fn iter_moves(&self) -> ();
    fn iter_swaps(&self) -> ();
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_index(&self, n: ()) -> Ptr<()>;
    fn operator_sub_10(&self, n: ()) -> adl_Iterator;
    fn operator_sub_11(&self, rhs: adl_Iterator) -> ();
    fn operator_add_assign(&self, n: ()) -> Ptr<adl_Iterator>;
    fn operator_sub_assign(&self, n: ()) -> Ptr<adl_Iterator>;
    fn operator_inc_14(&self) -> Ptr<adl_Iterator>;
    fn operator_post_inc_15(&self, _a0: ()) -> adl_Iterator;
    fn operator_dec_16(&self) -> Ptr<adl_Iterator>;
    fn operator_post_dec_17(&self, _a0: ()) -> adl_Iterator;
}
impl adl_IteratorImpl for Ptr<adl_Iterator> {
    fn iter_moves(&self) -> () {
        unimplemented!();
        return ((*self).with(|__s| __s.iter_moves_.clone()).read());
    }
    fn iter_swaps(&self) -> () {
        unimplemented!();
        return ((*self).with(|__s| __s.iter_swaps_.clone()).read());
    }
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
    fn operator_sub_10(&self, mut n: ()) -> adl_Iterator {
        return adl_Iterator::new_1(
            { (*self).with(|__s| __s.ptr_.clone()).offset(-((n) as isize)) },
            { (*self).with(|__s| __s.iter_moves_.clone()) },
            { (*self).with(|__s| __s.iter_swaps_.clone()) },
        );
    }
    fn operator_sub_11(&self, mut rhs: adl_Iterator) -> () {
        return (((*self).with(|__s| __s.ptr_.clone()) - (rhs.ptr_).clone()) as () as ());
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<adl_Iterator> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<adl_Iterator> {
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
    fn operator_post_inc_15(&self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return prev;
    }
    fn operator_dec_16(&self) -> Ptr<adl_Iterator> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_post_dec_17(&self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return prev;
    }
}
pub trait cpp17_input_iterator_int_ptr__int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp17_input_iterator_int_ptr__int_ptr_ {
        unimplemented!()
    }
}
impl cpp17_input_iterator_int_ptr__int_ptr_Impl for Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {}
pub trait cpp17_output_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp17_output_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl cpp17_output_iterator_int_ptr_Impl for Ptr<cpp17_output_iterator_int_ptr_> {}
pub trait cpp20_input_iterator_int_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_input_iterator_int_ptr_>,
    ) -> Ptr<cpp20_input_iterator_int_ptr_>;
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_inc_5(&self) -> Ptr<cpp20_input_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) {
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
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait cpp20_output_iterator_int_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_output_iterator_int_ptr_>,
    ) -> Ptr<cpp20_output_iterator_int_ptr_>;
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_inc_5(&self) -> Ptr<cpp20_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp20_output_iterator_int_ptr_ {
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
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait cpp20_random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_index(&self, n: ()) -> Ptr<()>;
    fn operator_inc_7(&self) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_dec_8(&self) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: ()) -> cpp20_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> cpp20_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
}
impl cpp20_random_access_iterator_int_ptr_Impl for Ptr<cpp20_random_access_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone()).offset((n) as isize);
    }
}
pub trait forward_sized_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_4(&self) -> Ptr<forward_sized_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: ()) -> forward_sized_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl forward_sized_iterator_int_ptr_Impl for Ptr<forward_sized_iterator_int_ptr_> {}
pub trait random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: ()) -> random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: ()) -> Ptr<random_access_iterator_int_ptr_>;
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<random_access_iterator_int_ptr_> {
        unimplemented!()
    }
}
impl random_access_iterator_int_ptr_Impl for Ptr<random_access_iterator_int_ptr_> {
    fn operator_add_assign(&self, mut n: ()) -> Ptr<random_access_iterator_int_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
}
pub trait rvalue_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn operator_inc_4(&self) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_dec_6(&self) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_dec_7(&self, mut _a0: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add(&self, mut n: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_sub_9(&self, mut n: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_sub_10(&self, other: Ptr<rvalue_iterator_int_ptr_>) -> () {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn operator_cmp(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> ();
    fn operator_eq(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> ();
}
impl rvalue_iterator_int_ptr_Impl for Ptr<rvalue_iterator_int_ptr_> {
    fn operator_cmp(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.it_.clone())),
                &(_a0.with(|__s| __s.it_.clone())),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> () {
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
        unimplemented!();
        field!(other, moved_from_).write(true);
        field!((*self), moved_from_).write(false);
        return (*self).clone();
    }
}
pub trait three_way_contiguous_iterator_int_ptr_Impl {
    fn base(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<three_way_contiguous_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<three_way_contiguous_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: ()) -> three_way_contiguous_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: ()) -> three_way_contiguous_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: ()) -> Ptr<three_way_contiguous_iterator_int_ptr_>;
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<three_way_contiguous_iterator_int_ptr_> {
        unimplemented!()
    }
}
impl three_way_contiguous_iterator_int_ptr_Impl for Ptr<three_way_contiguous_iterator_int_ptr_> {
    fn operator_add_assign(&self, mut n: ()) -> Ptr<three_way_contiguous_iterator_int_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
