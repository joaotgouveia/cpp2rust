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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp17_output_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
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
    it_: Ptr<()>,
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
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub fn operator_add_0(
    x: random_access_iterator_int_ptr_,
    mut n: (),
) -> random_access_iterator_int_ptr_ {
    let x: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_int_ptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return random_access_iterator_int_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_1(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn base_2(i: Ptr<random_access_iterator_int_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
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
    pub fn new_1(mut it: Ptr<()>) -> Self {
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
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
pub fn operator_add_3(x: contiguous_iterator_int_ptr_, mut n: ()) -> contiguous_iterator_int_ptr_ {
    let x: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(x));
    ({ contiguous_iterator_int_ptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return contiguous_iterator_int_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_4(
    mut x: contiguous_iterator_int_ptr_,
    mut y: contiguous_iterator_int_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn base_5(i: Ptr<contiguous_iterator_int_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<contiguous_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
pub fn base_6(i: Ptr<()>) -> Ptr<()> {
    let i: Value<Ptr<()>> = Rc::new(RefCell::new(i));
    return (*i.borrow()).clone();
}
pub fn base_7(i: Ptr<()>) -> Ptr<()> {
    let i: Value<Ptr<()>> = Rc::new(RefCell::new(i));
    return (*i.borrow()).clone();
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int__ {
    #[offset(0)]
    #[byte_size(1)]
    base_: InputIteratorNotDerivedFromGeneric_int_,
}
impl sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct sentinel_wrapper_InputIteratorNotIndirectlyReadable_ {
    #[offset(0)]
    #[byte_size(1)]
    base_: InputIteratorNotIndirectlyReadable,
}
impl sentinel_wrapper_InputIteratorNotIndirectlyReadable_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct sentinel_wrapper_InputIteratorNotInputOrOutputIterator_ {
    #[offset(0)]
    #[byte_size(1)]
    base_: InputIteratorNotInputOrOutputIterator,
}
impl sentinel_wrapper_InputIteratorNotInputOrOutputIterator_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct sentinel_wrapper_BidirectionalIteratorNotDerivedFrom_ {
    #[offset(0)]
    #[byte_size(1)]
    base_: BidirectionalIteratorNotDerivedFrom,
}
impl sentinel_wrapper_BidirectionalIteratorNotDerivedFrom_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct sentinel_wrapper_BidirectionalIteratorNotDecrementable_ {
    #[offset(0)]
    #[byte_size(1)]
    base_: BidirectionalIteratorNotDecrementable,
}
impl sentinel_wrapper_BidirectionalIteratorNotDecrementable_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct sentinel_wrapper_RandomAccessIteratorNotDerivedFrom_ {
    #[offset(0)]
    #[byte_size(1)]
    base_: RandomAccessIteratorNotDerivedFrom,
}
impl sentinel_wrapper_RandomAccessIteratorNotDerivedFrom_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct sentinel_wrapper_RandomAccessIteratorBadIndex_ {
    #[offset(0)]
    #[byte_size(1)]
    base_: RandomAccessIteratorBadIndex,
}
impl sentinel_wrapper_RandomAccessIteratorBadIndex_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_int_ptr_ {
    pub fn new_1(it: Ptr<Ptr<()>>) -> Self {
        Self {
            base_: ({ base_6((it.read())) }),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_NoComparator_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<NoComparator>,
}
impl sentinel_wrapper_NoComparator_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_random_access_iterator_int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_random_access_iterator_int_ptr__ {
    pub fn new_1(it: Ptr<random_access_iterator_int_ptr_>) -> Self {
        Self {
            base_: ({ base_2((it).clone()) }),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_contiguous_iterator_int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_contiguous_iterator_int_ptr__ {
    pub fn new_1(it: Ptr<contiguous_iterator_int_ptr_>) -> Self {
        Self {
            base_: ({ base_5((it).clone()) }),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_const_int_ptr_ {
    pub fn new_1(it: Ptr<Ptr<()>>) -> Self {
        Self {
            base_: ({ base_7((it.read())) }),
        }
    }
}
pub fn operator_eq_8(
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
impl CppEq for forward_sized_iterator_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_8(__lhs.clone(), __rhs.clone())
    }
}
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
pub fn operator_add_9(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_10(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_9(_i, n)
    });
}
pub fn iter_swap_11(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_12(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_13(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_14(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
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
impl CppLess for adl_Iterator {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_14(__lhs.clone(), __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for adl_Iterator {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_13(__lhs.clone(), __rhs.clone())
    }
}
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
    pub fn new(mut what: Ptr<()>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_int_ptr__int_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_InputIteratorNotDerivedFromGeneric_int___sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int___
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_InputIteratorNotIndirectlyReadable__sentinel_wrapper_InputIteratorNotIndirectlyReadable__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_InputIteratorNotInputOrOutputIterator__sentinel_wrapper_InputIteratorNotInputOrOutputIterator__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_BidirectionalIteratorNotDerivedFrom__sentinel_wrapper_BidirectionalIteratorNotDerivedFrom__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_BidirectionalIteratorNotDecrementable__sentinel_wrapper_BidirectionalIteratorNotDecrementable__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_RandomAccessIteratorNotDerivedFrom__sentinel_wrapper_RandomAccessIteratorNotDerivedFrom__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_RandomAccessIteratorBadIndex__sentinel_wrapper_RandomAccessIteratorBadIndex__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_int_ptr__sentinel_wrapper_int_ptr__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_NoComparator_ptr__sentinel_wrapper_NoComparator_ptr__ {}
const _: () = assert!(
    true,
    "std::ranges::contiguous_range<UncheckedRange<int*, int*>>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct InputIteratorNotDerivedFromGeneric_int_ {}
const _: () = assert!(
    true,
    "std::input_or_output_iterator<InputIteratorNotDerivedFrom>"
);
const _: () = assert!(
    true,
    "std::indirectly_readable<InputIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::input_iterator<InputIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::input_range<InputRangeNotDerivedFrom>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct InputIteratorNotIndirectlyReadable {}
const _: () = assert!(
    true,
    "std::input_or_output_iterator<InputIteratorNotIndirectlyReadable>"
);
const _: () = assert!(
    !(false),
    "!std::indirectly_readable<InputIteratorNotIndirectlyReadable>"
);
const _: () = assert!(
    !(false),
    "!std::input_iterator<InputIteratorNotIndirectlyReadable>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::input_range<InputRangeNotIndirectlyReadable>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct InputIteratorNotInputOrOutputIterator {}
const _: () = assert!(
    !(false),
    "!std::input_or_output_iterator<InputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(
    true,
    "std::indirectly_readable<InputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(
    !(false),
    "!std::input_iterator<InputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::input_range<InputRangeNotInputOrOutputIterator>"
);
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct IndirectUnaryPredicateNotCopyConstructible {}
const _: () = assert!(
    true,
    "std::predicate<IndirectUnaryPredicateNotCopyConstructible, int&>"
);
const _: () = assert!(
    !(false),
    "!std::indirect_unary_predicate<IndirectUnaryPredicateNotCopyConstructible, int*>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct IndirectUnaryPredicateNotPredicate {}
const _: () = assert!(
    !(false),
    "!std::predicate<IndirectUnaryPredicateNotPredicate, int&>"
);
const _: () = assert!(
    !(false),
    "!std::indirect_unary_predicate<IndirectUnaryPredicateNotPredicate, int*>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct SentinelForNotSemiregular {}
const _: () = assert!(
    true,
    "std::input_or_output_iterator<SentinelForNotSemiregular>"
);
const _: () = assert!(!(false), "!std::semiregular<SentinelForNotSemiregular>");
const _: () = assert!(
    !(false),
    "!std::sentinel_for<SentinelForNotSemiregular, cpp20_input_iterator<int*>>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct SentinelForNotWeaklyEqualityComparableWith {}
const _: () = assert!(
    true,
    "std::input_or_output_iterator<SentinelForNotWeaklyEqualityComparableWith>"
);
const _: () = assert!(
    true,
    "std::semiregular<SentinelForNotWeaklyEqualityComparableWith>"
);
const _: () = assert!(
    !(false),
    "!std::sentinel_for<SentinelForNotWeaklyEqualityComparableWith, cpp20_input_iterator<int*>>"
);
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct WeaklyIncrementableNotMovable {}
const _: () = assert!(!(false), "!std::movable<WeaklyIncrementableNotMovable>");
const _: () = assert!(
    !(false),
    "!std::weakly_incrementable<WeaklyIncrementableNotMovable>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ForwardIteratorNotDerivedFrom {}
impl CppEq for ForwardIteratorNotDerivedFrom {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        ForwardIteratorNotDerivedFromImpl::operator_eq(__lhs, __rhs.clone())
    }
}
const _: () = assert!(true, "std::input_iterator<ForwardIteratorNotDerivedFrom>");
const _: () = assert!(true, "std::incrementable<ForwardIteratorNotDerivedFrom>");
const _: () = assert!(
    true,
    "std::sentinel_for<ForwardIteratorNotDerivedFrom, ForwardIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::forward_iterator<ForwardIteratorNotDerivedFrom>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ForwardIteratorNotIncrementable {}
impl CppEq for ForwardIteratorNotIncrementable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        ForwardIteratorNotIncrementableImpl::operator_eq(__lhs, __rhs.clone())
    }
}
const _: () = assert!(true, "std::input_iterator<ForwardIteratorNotIncrementable>");
const _: () = assert!(
    !(false),
    "!std::incrementable<ForwardIteratorNotIncrementable>"
);
const _: () = assert!(
    true,
    "std::sentinel_for<ForwardIteratorNotIncrementable, ForwardIteratorNotIncrementable>"
);
const _: () = assert!(
    !(false),
    "!std::forward_iterator<ForwardIteratorNotIncrementable>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct BidirectionalIteratorNotDerivedFrom {}
impl CppEq for BidirectionalIteratorNotDerivedFrom {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        BidirectionalIteratorNotDerivedFromImpl::operator_eq(__lhs, __rhs.clone())
    }
}
const _: () = assert!(
    true,
    "std::forward_iterator<BidirectionalIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::bidirectional_iterator<BidirectionalIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::bidirectional_range<BidirectionalRangeNotDerivedFrom>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct BidirectionalIteratorNotDecrementable {}
impl CppEq for BidirectionalIteratorNotDecrementable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        BidirectionalIteratorNotDecrementableImpl::operator_eq(__lhs, __rhs.clone())
    }
}
const _: () = assert!(
    true,
    "std::forward_iterator<BidirectionalIteratorNotDecrementable>"
);
const _: () = assert!(
    !(false),
    "!std::bidirectional_iterator<BidirectionalIteratorNotDecrementable>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::bidirectional_range<BidirectionalRangeNotDecrementable>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct PermutableNotForwardIterator {}
const _: () = assert!(true, "std::input_iterator<PermutableNotForwardIterator>");
const _: () = assert!(
    !(false),
    "!std::forward_iterator<PermutableNotForwardIterator>"
);
const _: () = assert!(!(false), "!std::permutable<PermutableNotForwardIterator>");
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct PermutableNotSwappable_NotSwappable {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct PermutableNotSwappable {}
impl CppEq for PermutableNotSwappable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        PermutableNotSwappableImpl::operator_eq(__lhs, __rhs.clone())
    }
}
const _: () = assert!(true, "std::input_iterator<PermutableNotSwappable>");
const _: () = assert!(true, "std::forward_iterator<PermutableNotSwappable>");
const _: () = assert!(!(false), "!std::permutable<PermutableNotSwappable>");
const _: () = assert!(
    !(false),
    "!std::indirectly_swappable<PermutableNotSwappable>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct OutputIteratorNotInputOrOutputIterator {}
const _: () = assert!(
    !(false),
    "!std::input_or_output_iterator<OutputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(
    true,
    "std::indirectly_writable<OutputIteratorNotInputOrOutputIterator, int>"
);
const _: () = assert!(
    !(false),
    "!std::output_iterator<OutputIteratorNotInputOrOutputIterator, int>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::output_range<OutputRangeNotInputOrOutputIterator, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct OutputIteratorNotIndirectlyWritable {}
const _: () = assert!(
    true,
    "std::input_or_output_iterator<OutputIteratorNotIndirectlyWritable>"
);
const _: () = assert!(
    !(false),
    "!std::indirectly_writable<OutputIteratorNotIndirectlyWritable, int>"
);
const _: () = assert!(
    !(false),
    "!std::output_iterator<OutputIteratorNotIndirectlyWritable, int>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::output_range<OutputIteratorNotIndirectlyWritable, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct IndirectBinaryPredicateNotIndirectlyReadable {}
const _: () = assert!(
    !(false),
    "!std::indirect_binary_predicate<std::ranges::equal_to, IndirectBinaryPredicateNotIndirectlyReadable, int*>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RandomAccessIteratorNotDerivedFrom {}
impl CppLess for RandomAccessIteratorNotDerivedFrom {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        RandomAccessIteratorNotDerivedFromImpl::operator_cmp(__lhs, __rhs.clone())
            == std::cmp::Ordering::Less
    }
}
impl CppEq for RandomAccessIteratorNotDerivedFrom {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        RandomAccessIteratorNotDerivedFromImpl::operator_cmp(__lhs, __rhs.clone())
            == std::cmp::Ordering::Equal
    }
}
const _: () = assert!(
    true,
    "std::bidirectional_iterator<RandomAccessIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::random_access_iterator<RandomAccessIteratorNotDerivedFrom>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RandomAccessIteratorBadIndex {}
impl CppLess for RandomAccessIteratorBadIndex {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        RandomAccessIteratorBadIndexImpl::operator_cmp(__lhs, __rhs.clone())
            == std::cmp::Ordering::Less
    }
}
impl CppEq for RandomAccessIteratorBadIndex {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        RandomAccessIteratorBadIndexImpl::operator_cmp(__lhs, __rhs.clone())
            == std::cmp::Ordering::Equal
    }
}
const _: () = assert!(
    true,
    "std::bidirectional_iterator<RandomAccessIteratorBadIndex>"
);
const _: () = assert!(
    !(false),
    "!std::random_access_iterator<RandomAccessIteratorBadIndex>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RandomAccessIteratorBadDifferenceType {}
impl CppLess for RandomAccessIteratorBadDifferenceType {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        RandomAccessIteratorBadDifferenceTypeImpl::operator_cmp(__lhs, __rhs.clone())
            == std::cmp::Ordering::Less
    }
}
impl CppEq for RandomAccessIteratorBadDifferenceType {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        RandomAccessIteratorBadDifferenceTypeImpl::operator_cmp(__lhs, __rhs.clone())
            == std::cmp::Ordering::Equal
    }
}
const _: () = assert!(true, "std::regular<RandomAccessIteratorBadDifferenceType>");
const _: () = assert!(
    !(false),
    "!std::weakly_incrementable<RandomAccessIteratorBadDifferenceType>"
);
const _: () = assert!(
    !(false),
    "!std::random_access_iterator<RandomAccessIteratorBadDifferenceType>"
);
const _: () = assert!(true, "HasIsHeapUntilIter<int*, int*, std::ranges::less>");
const _: () = assert!(
    !(false),
    "!HasIsHeapUntilIter<RandomAccessIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!HasIsHeapUntilIter<RandomAccessIteratorBadIndex>"
);
const _: () = assert!(
    !(false),
    "!HasIsHeapUntilIter<int*, SentinelForNotSemiregular>"
);
const _: () = assert!(
    !(false),
    "!HasIsHeapUntilIter<int*, SentinelForNotWeaklyEqualityComparableWith>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoComparator {}
const _: () = assert!(
    !(false),
    "!HasIsHeapUntilIter<NoComparator*, NoComparator*>"
);
const _: () = assert!(true, "HasIsHeapUntilRange<R<int*>>");
const _: () = assert!(
    !(false),
    "!HasIsHeapUntilRange<RandomAccessRangeNotDerivedFrom>"
);
const _: () = assert!(!(false), "!HasIsHeapUntilRange<RandomAccessRangeBadIndex>");
const _: () = assert!(!(false), "!HasIsHeapUntilRange<R<NoComparator*>>");
pub fn test_one_15(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: random_access_iterator_int_ptr_ =
        random_access_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
        unimplemented!().offset((unimplemented!()) as isize)
    });
    {
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_17(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: random_access_iterator_int_ptr_ =
        random_access_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
        unimplemented!().offset((unimplemented!()) as isize)
    });
    {
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_18(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: random_access_iterator_int_ptr_ =
        random_access_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
        unimplemented!().offset((unimplemented!()) as isize)
    });
    {
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_19(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: random_access_iterator_int_ptr_ =
        random_access_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
        unimplemented!().offset((unimplemented!()) as isize)
    });
    {
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_20(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: random_access_iterator_int_ptr_ =
        random_access_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
        unimplemented!().offset((unimplemented!()) as isize)
    });
    {
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_21(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: random_access_iterator_int_ptr_ =
        random_access_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
        unimplemented!().offset((unimplemented!()) as isize)
    });
    {
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_22(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: random_access_iterator_int_ptr_ =
        random_access_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
        let __tmp_0: Value<random_access_iterator_int_ptr_> =
            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            })));
        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ __tmp_0.as_pointer() })
    };
    {
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_23(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: random_access_iterator_int_ptr_ =
        random_access_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
        let __tmp_1: Value<random_access_iterator_int_ptr_> =
            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            })));
        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ __tmp_1.as_pointer() })
    };
    {
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_24(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: random_access_iterator_int_ptr_ =
        random_access_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
        let __tmp_2: Value<random_access_iterator_int_ptr_> =
            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            })));
        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ __tmp_2.as_pointer() })
    };
    {
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_25(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: random_access_iterator_int_ptr_ =
        random_access_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
        let __tmp_3: Value<random_access_iterator_int_ptr_> =
            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            })));
        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ __tmp_3.as_pointer() })
    };
    {
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_26(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: random_access_iterator_int_ptr_ =
        random_access_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
        let __tmp_4: Value<random_access_iterator_int_ptr_> =
            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            })));
        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ __tmp_4.as_pointer() })
    };
    {
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_27(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: random_access_iterator_int_ptr_ =
        random_access_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
        let __tmp_5: Value<random_access_iterator_int_ptr_> =
            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            })));
        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ __tmp_5.as_pointer() })
    };
    {
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_28(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: contiguous_iterator_int_ptr_ =
        contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
        unimplemented!().offset((unimplemented!()) as isize)
    });
    {
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_29(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: contiguous_iterator_int_ptr_ =
        contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
        unimplemented!().offset((unimplemented!()) as isize)
    });
    {
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_30(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: contiguous_iterator_int_ptr_ =
        contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
        unimplemented!().offset((unimplemented!()) as isize)
    });
    {
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_31(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: contiguous_iterator_int_ptr_ =
        contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
        unimplemented!().offset((unimplemented!()) as isize)
    });
    {
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_32(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: contiguous_iterator_int_ptr_ =
        contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
        unimplemented!().offset((unimplemented!()) as isize)
    });
    {
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_33(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: contiguous_iterator_int_ptr_ =
        contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
        unimplemented!().offset((unimplemented!()) as isize)
    });
    {
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_34(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: contiguous_iterator_int_ptr_ =
        contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
        let __tmp_6: Value<contiguous_iterator_int_ptr_> =
            Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            })));
        sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_6.as_pointer() })
    };
    {
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_35(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: contiguous_iterator_int_ptr_ =
        contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
        let __tmp_7: Value<contiguous_iterator_int_ptr_> =
            Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            })));
        sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_7.as_pointer() })
    };
    {
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_36(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: contiguous_iterator_int_ptr_ =
        contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
        let __tmp_8: Value<contiguous_iterator_int_ptr_> =
            Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            })));
        sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_8.as_pointer() })
    };
    {
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_37(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: contiguous_iterator_int_ptr_ =
        contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
        let __tmp_9: Value<contiguous_iterator_int_ptr_> =
            Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            })));
        sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_9.as_pointer() })
    };
    {
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_38(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: contiguous_iterator_int_ptr_ =
        contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
        let __tmp_10: Value<contiguous_iterator_int_ptr_> =
            Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            })));
        sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_10.as_pointer() })
    };
    {
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_39(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: contiguous_iterator_int_ptr_ =
        contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
    let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
        let __tmp_11: Value<contiguous_iterator_int_ptr_> =
            Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            })));
        sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_11.as_pointer() })
    };
    {
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
            }),
        ));
        unimplemented!();
    }
}
pub fn test_one_40(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_one_41(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_one_42(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_one_43(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_one_44(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_one_45(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_one_46(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: sentinel_wrapper_int_ptr_ = {
        let __tmp_12: Value<Ptr<()>> = Rc::new(RefCell::new(
            unimplemented!().offset((unimplemented!()) as isize),
        ));
        sentinel_wrapper_int_ptr_::new_1({ __tmp_12.as_pointer() })
    };
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_one_47(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: sentinel_wrapper_int_ptr_ = {
        let __tmp_13: Value<Ptr<()>> = Rc::new(RefCell::new(
            unimplemented!().offset((unimplemented!()) as isize),
        ));
        sentinel_wrapper_int_ptr_::new_1({ __tmp_13.as_pointer() })
    };
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_one_48(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: sentinel_wrapper_int_ptr_ = {
        let __tmp_14: Value<Ptr<()>> = Rc::new(RefCell::new(
            unimplemented!().offset((unimplemented!()) as isize),
        ));
        sentinel_wrapper_int_ptr_::new_1({ __tmp_14.as_pointer() })
    };
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_one_49(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: sentinel_wrapper_int_ptr_ = {
        let __tmp_15: Value<Ptr<()>> = Rc::new(RefCell::new(
            unimplemented!().offset((unimplemented!()) as isize),
        ));
        sentinel_wrapper_int_ptr_::new_1({ __tmp_15.as_pointer() })
    };
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_one_50(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: sentinel_wrapper_int_ptr_ = {
        let __tmp_16: Value<Ptr<()>> = Rc::new(RefCell::new(
            unimplemented!().offset((unimplemented!()) as isize),
        ));
        sentinel_wrapper_int_ptr_::new_1({ __tmp_16.as_pointer() })
    };
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_one_51(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: sentinel_wrapper_int_ptr_ = {
        let __tmp_17: Value<Ptr<()>> = Rc::new(RefCell::new(
            unimplemented!().offset((unimplemented!()) as isize),
        ));
        sentinel_wrapper_int_ptr_::new_1({ __tmp_17.as_pointer() })
    };
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_one_52(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_one_53(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_one_54(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_one_55(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_one_56(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_one_57(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_one_58(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: sentinel_wrapper_const_int_ptr_ = {
        let __tmp_18: Value<Ptr<()>> = Rc::new(RefCell::new(
            unimplemented!().offset((unimplemented!()) as isize),
        ));
        sentinel_wrapper_const_int_ptr_::new_1({ __tmp_18.as_pointer() })
    };
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_one_59(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: sentinel_wrapper_const_int_ptr_ = {
        let __tmp_19: Value<Ptr<()>> = Rc::new(RefCell::new(
            unimplemented!().offset((unimplemented!()) as isize),
        ));
        sentinel_wrapper_const_int_ptr_::new_1({ __tmp_19.as_pointer() })
    };
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_one_60(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: sentinel_wrapper_const_int_ptr_ = {
        let __tmp_20: Value<Ptr<()>> = Rc::new(RefCell::new(
            unimplemented!().offset((unimplemented!()) as isize),
        ));
        sentinel_wrapper_const_int_ptr_::new_1({ __tmp_20.as_pointer() })
    };
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_one_61(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: sentinel_wrapper_const_int_ptr_ = {
        let __tmp_21: Value<Ptr<()>> = Rc::new(RefCell::new(
            unimplemented!().offset((unimplemented!()) as isize),
        ));
        sentinel_wrapper_const_int_ptr_::new_1({ __tmp_21.as_pointer() })
    };
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_one_62(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: sentinel_wrapper_const_int_ptr_ = {
        let __tmp_22: Value<Ptr<()>> = Rc::new(RefCell::new(
            unimplemented!().offset((unimplemented!()) as isize),
        ));
        sentinel_wrapper_const_int_ptr_::new_1({ __tmp_22.as_pointer() })
    };
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_one_63(input: (), mut until_index: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: sentinel_wrapper_const_int_ptr_ = {
        let __tmp_23: Value<Ptr<()>> = Rc::new(RefCell::new(
            unimplemented!().offset((unimplemented!()) as isize),
        ));
        sentinel_wrapper_const_int_ptr_::new_1({ __tmp_23.as_pointer() })
    };
    {
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(
                (begin).clone(),
                (end).clone(),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut result: Ptr<()> = ({
            (*is_heap_until_16.with(Value::clone).borrow_mut())(range.as_pointer(), None, None)
        });
        unimplemented!();
    }
}
pub fn test_iter_sent_64() {
    ({ test_one_15(unimplemented!(), 0_()) });
    ({ test_one_17(unimplemented!(), 1_()) });
    ({ test_one_18(unimplemented!(), 2_()) });
    ({ test_one_18(unimplemented!(), 1_()) });
    ({ test_one_19(unimplemented!(), 8_()) });
    ({ test_one_19(unimplemented!(), 7_()) });
    ({ test_one_20(unimplemented!(), 10_()) });
    ({ test_one_20(unimplemented!(), 3_()) });
    ({ test_one_21(unimplemented!(), 3_()) });
}
pub fn test_iter_sent_65() {
    ({ test_one_22(unimplemented!(), 0_()) });
    ({ test_one_23(unimplemented!(), 1_()) });
    ({ test_one_24(unimplemented!(), 2_()) });
    ({ test_one_24(unimplemented!(), 1_()) });
    ({ test_one_25(unimplemented!(), 8_()) });
    ({ test_one_25(unimplemented!(), 7_()) });
    ({ test_one_26(unimplemented!(), 10_()) });
    ({ test_one_26(unimplemented!(), 3_()) });
    ({ test_one_27(unimplemented!(), 3_()) });
}
pub fn test_iter_sent_66() {
    ({ test_one_28(unimplemented!(), 0_()) });
    ({ test_one_29(unimplemented!(), 1_()) });
    ({ test_one_30(unimplemented!(), 2_()) });
    ({ test_one_30(unimplemented!(), 1_()) });
    ({ test_one_31(unimplemented!(), 8_()) });
    ({ test_one_31(unimplemented!(), 7_()) });
    ({ test_one_32(unimplemented!(), 10_()) });
    ({ test_one_32(unimplemented!(), 3_()) });
    ({ test_one_33(unimplemented!(), 3_()) });
}
pub fn test_iter_sent_67() {
    ({ test_one_34(unimplemented!(), 0_()) });
    ({ test_one_35(unimplemented!(), 1_()) });
    ({ test_one_36(unimplemented!(), 2_()) });
    ({ test_one_36(unimplemented!(), 1_()) });
    ({ test_one_37(unimplemented!(), 8_()) });
    ({ test_one_37(unimplemented!(), 7_()) });
    ({ test_one_38(unimplemented!(), 10_()) });
    ({ test_one_38(unimplemented!(), 3_()) });
    ({ test_one_39(unimplemented!(), 3_()) });
}
pub fn test_iter_sent_68() {
    ({ test_one_40(unimplemented!(), 0_()) });
    ({ test_one_41(unimplemented!(), 1_()) });
    ({ test_one_42(unimplemented!(), 2_()) });
    ({ test_one_42(unimplemented!(), 1_()) });
    ({ test_one_43(unimplemented!(), 8_()) });
    ({ test_one_43(unimplemented!(), 7_()) });
    ({ test_one_44(unimplemented!(), 10_()) });
    ({ test_one_44(unimplemented!(), 3_()) });
    ({ test_one_45(unimplemented!(), 3_()) });
}
pub fn test_iter_sent_69() {
    ({ test_one_46(unimplemented!(), 0_()) });
    ({ test_one_47(unimplemented!(), 1_()) });
    ({ test_one_48(unimplemented!(), 2_()) });
    ({ test_one_48(unimplemented!(), 1_()) });
    ({ test_one_49(unimplemented!(), 8_()) });
    ({ test_one_49(unimplemented!(), 7_()) });
    ({ test_one_50(unimplemented!(), 10_()) });
    ({ test_one_50(unimplemented!(), 3_()) });
    ({ test_one_51(unimplemented!(), 3_()) });
}
pub fn test_iter_sent_70() {
    ({ test_one_52(unimplemented!(), 0_()) });
    ({ test_one_53(unimplemented!(), 1_()) });
    ({ test_one_54(unimplemented!(), 2_()) });
    ({ test_one_54(unimplemented!(), 1_()) });
    ({ test_one_55(unimplemented!(), 8_()) });
    ({ test_one_55(unimplemented!(), 7_()) });
    ({ test_one_56(unimplemented!(), 10_()) });
    ({ test_one_56(unimplemented!(), 3_()) });
    ({ test_one_57(unimplemented!(), 3_()) });
}
pub fn test_iter_sent_71() {
    ({ test_one_58(unimplemented!(), 0_()) });
    ({ test_one_59(unimplemented!(), 1_()) });
    ({ test_one_60(unimplemented!(), 2_()) });
    ({ test_one_60(unimplemented!(), 1_()) });
    ({ test_one_61(unimplemented!(), 8_()) });
    ({ test_one_61(unimplemented!(), 7_()) });
    ({ test_one_62(unimplemented!(), 10_()) });
    ({ test_one_62(unimplemented!(), 3_()) });
    ({ test_one_63(unimplemented!(), 3_()) });
}
pub fn test_iter_72() {
    ({ test_iter_sent_64() });
    ({ test_iter_sent_65() });
}
pub fn test_iter_73() {
    ({ test_iter_sent_66() });
    ({ test_iter_sent_67() });
}
pub fn test_iter_74() {
    ({ test_iter_sent_68() });
    ({ test_iter_sent_69() });
}
pub fn test_iter_75() {
    ({ test_iter_sent_70() });
    ({ test_iter_sent_71() });
}
pub fn test_iterators_76() {
    ({ test_iter_72() });
    ({ test_iter_73() });
    ({ test_iter_74() });
    ({ test_iter_75() });
}
pub fn test_77() -> () {
    ({ test_iterators_76() });
    {
        let ls: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let gt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let in_: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let mut result_default_comp: Ptr<()> = ({
                let ___first: Ptr<()> = unimplemented!();
                let ___last: Ptr<()> = unimplemented!();
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    ___first,
                    ___last,
                    Some(unimplemented!()),
                    None,
                )
            });
            unimplemented!();
            let mut result_custom_comp: Ptr<()> = ({
                let ___first: Ptr<()> = unimplemented!();
                let ___last: Ptr<()> = unimplemented!();
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    ___first,
                    ___last,
                    unimplemented!(),
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut result_default_comp: () = ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    in_.as_pointer(),
                    unimplemented!(),
                    None,
                )
            });
            unimplemented!();
            let mut result_custom_comp: () = ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    in_.as_pointer(),
                    unimplemented!(),
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let in_: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let negate: Value<FnPtr<fn(A_78) -> ()>> = Rc::new(RefCell::new(
            FnPtr::<fn(A_78) -> ()>::new(|mut a: A_78| -> () {
                {
                    return (a.x * -1_());
                }
            }),
        ));
        {
            let mut result_default_comp: Ptr<A_78> = ({
                let ___first: Ptr<A_78> = unimplemented!();
                let ___last: Ptr<A_78> = unimplemented!();
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    ___first,
                    ___last,
                    unimplemented!(),
                    None,
                )
            });
            unimplemented!();
            let mut result_custom_comp: Ptr<A_78> = ({
                let ___first: Ptr<A_78> = unimplemented!();
                let ___last: Ptr<A_78> = unimplemented!();
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    ___first,
                    ___last,
                    unimplemented!(),
                    (*negate.borrow()).copy_from(),
                )
            });
            unimplemented!();
        }
        {
            let mut result_default_comp: () = ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    in_.as_pointer(),
                    unimplemented!(),
                    None,
                )
            });
            unimplemented!();
            let mut result_custom_comp: () = ({
                (*is_heap_until_16.with(Value::clone).borrow_mut())(
                    in_.as_pointer(),
                    unimplemented!(),
                    (*negate.borrow()).copy_from(),
                )
            });
            unimplemented!();
        }
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct A_78 {
    #[offset(0)]
    pub x: (),
}
impl CppLess for A_78 {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        A_78Impl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for A_78 {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        A_78Impl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Equal
    }
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
    ({ test_77() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait A_78Impl {
    fn operator_cmp(&self, _a0: Ptr<A_78>) -> ();
    fn operator_eq(&self, _a0: Ptr<A_78>) -> ();
}
impl A_78Impl for Ptr<A_78> {
    fn operator_cmp(&self, _a0: Ptr<A_78>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.x)),
                &(_a0.with(|__s| __s.x)),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<A_78>) -> () {
        return ({ (*self).with(|__s| __s.x) } == { _a0.with(|__s| __s.x) });
    }
}
pub trait BidirectionalIteratorNotDecrementableImpl {
    fn operator_inc_1(&self) -> Ptr<BidirectionalIteratorNotDecrementable> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) -> BidirectionalIteratorNotDecrementable {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_eq(&self, _a0: Ptr<BidirectionalIteratorNotDecrementable>) -> ();
}
impl BidirectionalIteratorNotDecrementableImpl for Ptr<BidirectionalIteratorNotDecrementable> {
    fn operator_eq(&self, _a0: Ptr<BidirectionalIteratorNotDecrementable>) -> () {
        return true;
    }
}
pub trait BidirectionalIteratorNotDerivedFromImpl {
    fn operator_inc_1(&self) -> Ptr<BidirectionalIteratorNotDerivedFrom> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) -> BidirectionalIteratorNotDerivedFrom {
        unimplemented!()
    }
    fn operator_dec_3(&self) -> Ptr<BidirectionalIteratorNotDerivedFrom> {
        unimplemented!()
    }
    fn operator_post_dec_4(&self, mut _a0: ()) -> BidirectionalIteratorNotDerivedFrom {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_eq(&self, _a0: Ptr<BidirectionalIteratorNotDerivedFrom>) -> ();
}
impl BidirectionalIteratorNotDerivedFromImpl for Ptr<BidirectionalIteratorNotDerivedFrom> {
    fn operator_eq(&self, _a0: Ptr<BidirectionalIteratorNotDerivedFrom>) -> () {
        return true;
    }
}
pub trait ForwardIteratorNotDerivedFromImpl {
    fn operator_inc_1(&self) -> Ptr<ForwardIteratorNotDerivedFrom> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) -> ForwardIteratorNotDerivedFrom {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_eq(&self, _a0: Ptr<ForwardIteratorNotDerivedFrom>) -> ();
}
impl ForwardIteratorNotDerivedFromImpl for Ptr<ForwardIteratorNotDerivedFrom> {
    fn operator_eq(&self, _a0: Ptr<ForwardIteratorNotDerivedFrom>) -> () {
        return true;
    }
}
pub trait ForwardIteratorNotIncrementableImpl {
    fn operator_inc_1(&self) -> Ptr<ForwardIteratorNotIncrementable> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_eq(&self, _a0: Ptr<ForwardIteratorNotIncrementable>) -> ();
}
impl ForwardIteratorNotIncrementableImpl for Ptr<ForwardIteratorNotIncrementable> {
    fn operator_eq(&self, _a0: Ptr<ForwardIteratorNotIncrementable>) -> () {
        return true;
    }
}
pub trait IndirectBinaryPredicateNotIndirectlyReadableImpl {
    fn operator_inc_1(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl IndirectBinaryPredicateNotIndirectlyReadableImpl
    for Ptr<IndirectBinaryPredicateNotIndirectlyReadable>
{
}
pub trait IndirectUnaryPredicateNotCopyConstructibleImpl {
    fn operator_call(&self, mut _a0: ()) -> () {
        unimplemented!()
    }
}
impl IndirectUnaryPredicateNotCopyConstructibleImpl
    for Ptr<IndirectUnaryPredicateNotCopyConstructible>
{
}
pub trait IndirectUnaryPredicateNotPredicateImpl {
    fn operator_call(&self, _a0: Ptr<()>) -> () {
        unimplemented!()
    }
}
impl IndirectUnaryPredicateNotPredicateImpl for Ptr<IndirectUnaryPredicateNotPredicate> {}
pub trait InputIteratorNotDerivedFromGeneric_int_Impl {
    fn operator_inc_1(&self) -> Ptr<InputIteratorNotDerivedFromGeneric_int_> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl InputIteratorNotDerivedFromGeneric_int_Impl for Ptr<InputIteratorNotDerivedFromGeneric_int_> {}
pub trait InputIteratorNotIndirectlyReadableImpl {
    fn operator_inc_1(&self) -> Ptr<InputIteratorNotIndirectlyReadable> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl InputIteratorNotIndirectlyReadableImpl for Ptr<InputIteratorNotIndirectlyReadable> {}
pub trait InputIteratorNotInputOrOutputIteratorImpl {
    fn operator_inc_1(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl InputIteratorNotInputOrOutputIteratorImpl for Ptr<InputIteratorNotInputOrOutputIterator> {}
pub trait OutputIteratorNotIndirectlyWritableImpl {
    fn operator_inc_1(&self) -> Ptr<OutputIteratorNotIndirectlyWritable> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl OutputIteratorNotIndirectlyWritableImpl for Ptr<OutputIteratorNotIndirectlyWritable> {}
pub trait OutputIteratorNotInputOrOutputIteratorImpl {
    fn operator_inc_1(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl OutputIteratorNotInputOrOutputIteratorImpl for Ptr<OutputIteratorNotInputOrOutputIterator> {}
pub trait PermutableNotForwardIteratorImpl {
    fn operator_inc_1(&self) -> Ptr<PermutableNotForwardIterator> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl PermutableNotForwardIteratorImpl for Ptr<PermutableNotForwardIterator> {}
pub trait PermutableNotSwappableImpl {
    fn operator_inc_1(&self) -> Ptr<PermutableNotSwappable> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) -> PermutableNotSwappable {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<PermutableNotSwappable_NotSwappable> {
        unimplemented!()
    }
    fn operator_eq(&self, _a0: Ptr<PermutableNotSwappable>) -> ();
}
impl PermutableNotSwappableImpl for Ptr<PermutableNotSwappable> {
    fn operator_eq(&self, _a0: Ptr<PermutableNotSwappable>) -> () {
        return true;
    }
}
pub trait RandomAccessIteratorBadDifferenceTypeImpl {
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_index(&self, mut _a0: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_3(&self) -> Ptr<RandomAccessIteratorBadDifferenceType> {
        unimplemented!()
    }
    fn operator_dec_4(&self) -> Ptr<RandomAccessIteratorBadDifferenceType> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: ()) -> RandomAccessIteratorBadDifferenceType {
        unimplemented!()
    }
    fn operator_post_dec_6(&self, mut _a0: ()) -> RandomAccessIteratorBadDifferenceType {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut _a0: ()) -> Ptr<RandomAccessIteratorBadDifferenceType> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut _a0: ()) -> Ptr<RandomAccessIteratorBadDifferenceType> {
        unimplemented!()
    }
    fn operator_cmp(&self, _a0: Ptr<RandomAccessIteratorBadDifferenceType>) -> ();
    fn operator_eq(&self, _a0: Ptr<RandomAccessIteratorBadDifferenceType>) -> ();
}
impl RandomAccessIteratorBadDifferenceTypeImpl for Ptr<RandomAccessIteratorBadDifferenceType> {
    fn operator_cmp(&self, _a0: Ptr<RandomAccessIteratorBadDifferenceType>) -> () {
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<RandomAccessIteratorBadDifferenceType>) -> () {
        return true;
    }
}
pub trait RandomAccessIteratorBadIndexImpl {
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_index(&self, mut _a0: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_3(&self) -> Ptr<RandomAccessIteratorBadIndex> {
        unimplemented!()
    }
    fn operator_dec_4(&self) -> Ptr<RandomAccessIteratorBadIndex> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: ()) -> RandomAccessIteratorBadIndex {
        unimplemented!()
    }
    fn operator_post_dec_6(&self, mut _a0: ()) -> RandomAccessIteratorBadIndex {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut _a0: ()) -> Ptr<RandomAccessIteratorBadIndex> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut _a0: ()) -> Ptr<RandomAccessIteratorBadIndex> {
        unimplemented!()
    }
    fn operator_cmp(&self, _a0: Ptr<RandomAccessIteratorBadIndex>) -> ();
    fn operator_eq(&self, _a0: Ptr<RandomAccessIteratorBadIndex>) -> ();
}
impl RandomAccessIteratorBadIndexImpl for Ptr<RandomAccessIteratorBadIndex> {
    fn operator_cmp(&self, _a0: Ptr<RandomAccessIteratorBadIndex>) -> () {
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<RandomAccessIteratorBadIndex>) -> () {
        return true;
    }
}
pub trait RandomAccessIteratorNotDerivedFromImpl {
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_index(&self, mut _a0: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_3(&self) -> Ptr<RandomAccessIteratorNotDerivedFrom> {
        unimplemented!()
    }
    fn operator_dec_4(&self) -> Ptr<RandomAccessIteratorNotDerivedFrom> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: ()) -> RandomAccessIteratorNotDerivedFrom {
        unimplemented!()
    }
    fn operator_post_dec_6(&self, mut _a0: ()) -> RandomAccessIteratorNotDerivedFrom {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut _a0: ()) -> Ptr<RandomAccessIteratorNotDerivedFrom> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut _a0: ()) -> Ptr<RandomAccessIteratorNotDerivedFrom> {
        unimplemented!()
    }
    fn operator_cmp(&self, _a0: Ptr<RandomAccessIteratorNotDerivedFrom>) -> ();
    fn operator_eq(&self, _a0: Ptr<RandomAccessIteratorNotDerivedFrom>) -> ();
}
impl RandomAccessIteratorNotDerivedFromImpl for Ptr<RandomAccessIteratorNotDerivedFrom> {
    fn operator_cmp(&self, _a0: Ptr<RandomAccessIteratorNotDerivedFrom>) -> () {
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<RandomAccessIteratorNotDerivedFrom>) -> () {
        return true;
    }
}
pub trait SentinelForNotSemiregularImpl {
    fn operator_inc_2(&self) -> Ptr<SentinelForNotSemiregular> {
        unimplemented!()
    }
    fn operator_post_inc_3(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl SentinelForNotSemiregularImpl for Ptr<SentinelForNotSemiregular> {}
pub trait SentinelForNotWeaklyEqualityComparableWithImpl {
    fn operator_inc_1(&self) -> Ptr<SentinelForNotWeaklyEqualityComparableWith> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl SentinelForNotWeaklyEqualityComparableWithImpl
    for Ptr<SentinelForNotWeaklyEqualityComparableWith>
{
}
pub trait UncheckedRange_BidirectionalIteratorNotDecrementable__sentinel_wrapper_BidirectionalIteratorNotDecrementable__Impl
{
    fn begin(&self) -> BidirectionalIteratorNotDecrementable {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_BidirectionalIteratorNotDecrementable_ {
        unimplemented!()
    }
}
impl UncheckedRange_BidirectionalIteratorNotDecrementable__sentinel_wrapper_BidirectionalIteratorNotDecrementable__Impl for Ptr<UncheckedRange_BidirectionalIteratorNotDecrementable__sentinel_wrapper_BidirectionalIteratorNotDecrementable__> {
}
pub trait UncheckedRange_BidirectionalIteratorNotDerivedFrom__sentinel_wrapper_BidirectionalIteratorNotDerivedFrom__Impl
{
    fn begin(&self) -> BidirectionalIteratorNotDerivedFrom {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_BidirectionalIteratorNotDerivedFrom_ {
        unimplemented!()
    }
}
impl UncheckedRange_BidirectionalIteratorNotDerivedFrom__sentinel_wrapper_BidirectionalIteratorNotDerivedFrom__Impl for Ptr<UncheckedRange_BidirectionalIteratorNotDerivedFrom__sentinel_wrapper_BidirectionalIteratorNotDerivedFrom__> {
}
pub trait UncheckedRange_InputIteratorNotDerivedFromGeneric_int___sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int___Impl
{
    fn begin(&self) -> InputIteratorNotDerivedFromGeneric_int_ {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int__ {
        unimplemented!()
    }
}
impl UncheckedRange_InputIteratorNotDerivedFromGeneric_int___sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int___Impl for Ptr<UncheckedRange_InputIteratorNotDerivedFromGeneric_int___sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int___> {
}
pub trait UncheckedRange_InputIteratorNotIndirectlyReadable__sentinel_wrapper_InputIteratorNotIndirectlyReadable__Impl
{
    fn begin(&self) -> InputIteratorNotIndirectlyReadable {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_InputIteratorNotIndirectlyReadable_ {
        unimplemented!()
    }
}
impl UncheckedRange_InputIteratorNotIndirectlyReadable__sentinel_wrapper_InputIteratorNotIndirectlyReadable__Impl for Ptr<UncheckedRange_InputIteratorNotIndirectlyReadable__sentinel_wrapper_InputIteratorNotIndirectlyReadable__> {
}
pub trait UncheckedRange_InputIteratorNotInputOrOutputIterator__sentinel_wrapper_InputIteratorNotInputOrOutputIterator__Impl
{
    fn begin(&self) -> InputIteratorNotInputOrOutputIterator {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_InputIteratorNotInputOrOutputIterator_ {
        unimplemented!()
    }
}
impl UncheckedRange_InputIteratorNotInputOrOutputIterator__sentinel_wrapper_InputIteratorNotInputOrOutputIterator__Impl for Ptr<UncheckedRange_InputIteratorNotInputOrOutputIterator__sentinel_wrapper_InputIteratorNotInputOrOutputIterator__> {
}
pub trait UncheckedRange_NoComparator_ptr__sentinel_wrapper_NoComparator_ptr__Impl {
    fn begin(&self) -> Ptr<NoComparator> {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_NoComparator_ptr_ {
        unimplemented!()
    }
}
impl UncheckedRange_NoComparator_ptr__sentinel_wrapper_NoComparator_ptr__Impl
    for Ptr<UncheckedRange_NoComparator_ptr__sentinel_wrapper_NoComparator_ptr__>
{
}
pub trait UncheckedRange_RandomAccessIteratorBadIndex__sentinel_wrapper_RandomAccessIteratorBadIndex__Impl
{
    fn begin(&self) -> RandomAccessIteratorBadIndex {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_RandomAccessIteratorBadIndex_ {
        unimplemented!()
    }
}
impl UncheckedRange_RandomAccessIteratorBadIndex__sentinel_wrapper_RandomAccessIteratorBadIndex__Impl for Ptr<UncheckedRange_RandomAccessIteratorBadIndex__sentinel_wrapper_RandomAccessIteratorBadIndex__> {
}
pub trait UncheckedRange_RandomAccessIteratorNotDerivedFrom__sentinel_wrapper_RandomAccessIteratorNotDerivedFrom__Impl
{
    fn begin(&self) -> RandomAccessIteratorNotDerivedFrom {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_RandomAccessIteratorNotDerivedFrom_ {
        unimplemented!()
    }
}
impl UncheckedRange_RandomAccessIteratorNotDerivedFrom__sentinel_wrapper_RandomAccessIteratorNotDerivedFrom__Impl for Ptr<UncheckedRange_RandomAccessIteratorNotDerivedFrom__sentinel_wrapper_RandomAccessIteratorNotDerivedFrom__> {
}
pub trait UncheckedRange_int_ptr__int_ptr_Impl {
    fn begin(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl UncheckedRange_int_ptr__int_ptr_Impl for Ptr<UncheckedRange_int_ptr__int_ptr_> {}
pub trait UncheckedRange_int_ptr__sentinel_wrapper_int_ptr__Impl {
    fn begin(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_int_ptr_ {
        unimplemented!()
    }
}
impl UncheckedRange_int_ptr__sentinel_wrapper_int_ptr__Impl
    for Ptr<UncheckedRange_int_ptr__sentinel_wrapper_int_ptr__>
{
}
pub trait WeaklyIncrementableNotMovableImpl {
    fn operator_inc_1(&self) -> Ptr<WeaklyIncrementableNotMovable> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) {
        unimplemented!()
    }
}
impl WeaklyIncrementableNotMovableImpl for Ptr<WeaklyIncrementableNotMovable> {}
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
pub trait contiguous_iterator_int_ptr_Impl {
    fn base(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_deref(&self) -> ();
    fn operator_arrow(&self) -> () {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_int_ptr_>;
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: ()) -> contiguous_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: ()) -> contiguous_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: ()) -> Ptr<contiguous_iterator_int_ptr_>;
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_int_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_int_ptr_>,
    ) -> Ptr<contiguous_iterator_int_ptr_>;
}
impl contiguous_iterator_int_ptr_Impl for Ptr<contiguous_iterator_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_int_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_int_ptr_>,
    ) -> Ptr<contiguous_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
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
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp17_output_iterator_int_ptr_ {
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
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_random_access_iterator_int_ptr_>,
    ) -> Ptr<cpp20_random_access_iterator_int_ptr_>;
}
impl cpp20_random_access_iterator_int_ptr_Impl for Ptr<cpp20_random_access_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
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
    fn operator_deref(&self) -> ();
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptr_>;
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
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_int_ptr_>,
    ) -> Ptr<random_access_iterator_int_ptr_>;
}
impl random_access_iterator_int_ptr_Impl for Ptr<random_access_iterator_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<random_access_iterator_int_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
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
pub trait sentinel_wrapper_BidirectionalIteratorNotDecrementable_Impl {
    fn operator_eq(&self, other: Ptr<BidirectionalIteratorNotDecrementable>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_BidirectionalIteratorNotDecrementable_Impl
    for Ptr<sentinel_wrapper_BidirectionalIteratorNotDecrementable_>
{
}
pub trait sentinel_wrapper_BidirectionalIteratorNotDerivedFrom_Impl {
    fn operator_eq(&self, other: Ptr<BidirectionalIteratorNotDerivedFrom>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_BidirectionalIteratorNotDerivedFrom_Impl
    for Ptr<sentinel_wrapper_BidirectionalIteratorNotDerivedFrom_>
{
}
pub trait sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int__Impl {
    fn operator_eq(&self, other: Ptr<InputIteratorNotDerivedFromGeneric_int_>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int__Impl
    for Ptr<sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int__>
{
}
pub trait sentinel_wrapper_InputIteratorNotIndirectlyReadable_Impl {
    fn operator_eq(&self, other: Ptr<InputIteratorNotIndirectlyReadable>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_InputIteratorNotIndirectlyReadable_Impl
    for Ptr<sentinel_wrapper_InputIteratorNotIndirectlyReadable_>
{
}
pub trait sentinel_wrapper_InputIteratorNotInputOrOutputIterator_Impl {
    fn operator_eq(&self, other: Ptr<InputIteratorNotInputOrOutputIterator>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_InputIteratorNotInputOrOutputIterator_Impl
    for Ptr<sentinel_wrapper_InputIteratorNotInputOrOutputIterator_>
{
}
pub trait sentinel_wrapper_NoComparator_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<NoComparator>>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_NoComparator_ptr_Impl for Ptr<sentinel_wrapper_NoComparator_ptr_> {}
pub trait sentinel_wrapper_RandomAccessIteratorBadIndex_Impl {
    fn operator_eq(&self, other: Ptr<RandomAccessIteratorBadIndex>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_RandomAccessIteratorBadIndex_Impl
    for Ptr<sentinel_wrapper_RandomAccessIteratorBadIndex_>
{
}
pub trait sentinel_wrapper_RandomAccessIteratorNotDerivedFrom_Impl {
    fn operator_eq(&self, other: Ptr<RandomAccessIteratorNotDerivedFrom>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_RandomAccessIteratorNotDerivedFrom_Impl
    for Ptr<sentinel_wrapper_RandomAccessIteratorNotDerivedFrom_>
{
}
pub trait sentinel_wrapper_const_int_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> ();
}
impl sentinel_wrapper_const_int_ptr_Impl for Ptr<sentinel_wrapper_const_int_ptr_> {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> () {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_7((other.read())) }) });
    }
}
pub trait sentinel_wrapper_contiguous_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<contiguous_iterator_int_ptr_>) -> ();
}
impl sentinel_wrapper_contiguous_iterator_int_ptr__Impl
    for Ptr<sentinel_wrapper_contiguous_iterator_int_ptr__>
{
    fn operator_eq(&self, other: Ptr<contiguous_iterator_int_ptr_>) -> () {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_5((other).clone()) }) });
    }
}
pub trait sentinel_wrapper_int_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> ();
}
impl sentinel_wrapper_int_ptr_Impl for Ptr<sentinel_wrapper_int_ptr_> {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> () {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_6((other.read())) }) });
    }
}
pub trait sentinel_wrapper_random_access_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<random_access_iterator_int_ptr_>) -> ();
}
impl sentinel_wrapper_random_access_iterator_int_ptr__Impl
    for Ptr<sentinel_wrapper_random_access_iterator_int_ptr__>
{
    fn operator_eq(&self, other: Ptr<random_access_iterator_int_ptr_>) -> () {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_2((other).clone()) }) });
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
pub fn __cpp2rust_init_globals() {}
