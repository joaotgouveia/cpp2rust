extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_1(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_2(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_3(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_4(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_5(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_6(_a0: Ptr<MoveOnly>, _a1: Ptr<MoveOnly>) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    data_: (),
}
impl MoveOnly {
    pub fn new(data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| 1_());
        Self { data_: data }
    }
    pub fn move_from(x: Ptr<MoveOnly>) -> Self {
        let __this: MoveOnly = Self {
            data_: x.with(|__s| __s.data_),
        };
        field!(x, data_).write(0_());
        __this
    }
}
impl std::cmp::Ord for MoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_6(
                Rc::new(RefCell::new(MoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(MoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_0(
                Rc::new(RefCell::new(MoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(MoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new(None) }
    }
}
pub fn operator_eq_7(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_8(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_9(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_10(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_11(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_12(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_13(_a0: Ptr<TrivialMoveOnly>, _a1: Ptr<TrivialMoveOnly>) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(4)]
pub struct TrivialMoveOnly {
    #[offset(0)]
    data_: (),
}
impl TrivialMoveOnly {
    pub fn new(data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| 1_());
        Self { data_: data }
    }
    pub fn move_from(_a0: Ptr<TrivialMoveOnly>) -> Self {
        Self {
            data_: { (*_a0.upgrade().deref()).data_ },
        }
    }
}
impl std::cmp::Ord for TrivialMoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_13(
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for TrivialMoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for TrivialMoveOnly {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_7(
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for TrivialMoveOnly {}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        { TrivialMoveOnly::new(None) }
    }
}
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
pub fn operator_add_14(
    x: random_access_iterator_int_ptr_,
    mut n: (),
) -> random_access_iterator_int_ptr_ {
    let x: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_int_ptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return random_access_iterator_int_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_15(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn operator_eq_16(
    x: Ptr<random_access_iterator_int_ptr_>,
    y: Ptr<random_access_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_17(
    x: Ptr<random_access_iterator_int_ptr_>,
    y: Ptr<random_access_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
pub fn base_18(i: Ptr<random_access_iterator_int_ptr_>) -> Ptr<()> {
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
impl std::cmp::PartialEq for random_access_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_16(
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
pub fn base_19(i: Ptr<()>) -> Ptr<()> {
    let i: Value<Ptr<()>> = Rc::new(RefCell::new(i));
    return (*i.borrow()).clone();
}
pub fn base_20(i: Ptr<cpp20_input_iterator_int_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
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
    pub fn new(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
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
impl sentinel_wrapper_int_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_OrderedConvertibleToInt_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<OrderedConvertibleToInt>,
}
impl sentinel_wrapper_OrderedConvertibleToInt_ptr_ {}
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
            base_: ({ base_19((it.read())) }),
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
pub struct sentinel_wrapper_MoveOnly_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<MoveOnly>,
}
impl sentinel_wrapper_MoveOnly_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
    pub fn new_1(it: Ptr<cpp20_input_iterator_int_ptr_>) -> Self {
        Self {
            base_: ({ base_20((it).clone()) }),
        }
    }
}
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
            base_: ({ base_18((it).clone()) }),
        }
    }
}
pub fn operator_eq_21(
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
            operator_eq_21(
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
pub fn operator_add_22(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_23(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_22(_i, n)
    });
}
pub fn iter_swap_24(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_25(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return unimplemented!();
}
pub fn operator_eq_26(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_27(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
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
            operator_cmp_27(
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
            operator_eq_26(
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
pub struct UncheckedRange_OrderedConvertibleToInt_ptr__sentinel_wrapper_OrderedConvertibleToInt_ptr__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_const_int_ptr__sentinel_wrapper_const_int_ptr__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_NoComparator_ptr__sentinel_wrapper_NoComparator_ptr__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_MoveOnly_ptr__sentinel_wrapper_MoveOnly_ptr__ {}
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
impl std::cmp::PartialEq for ForwardIteratorNotDerivedFrom {
    fn eq(&self, other: &Self) -> bool {
        {
            ForwardIteratorNotDerivedFromImpl::operator_eq(
                &Rc::new(RefCell::new(ForwardIteratorNotDerivedFrom {})).as_pointer(),
                Rc::new(RefCell::new(ForwardIteratorNotDerivedFrom {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for ForwardIteratorNotDerivedFrom {}
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
impl std::cmp::PartialEq for ForwardIteratorNotIncrementable {
    fn eq(&self, other: &Self) -> bool {
        {
            ForwardIteratorNotIncrementableImpl::operator_eq(
                &Rc::new(RefCell::new(ForwardIteratorNotIncrementable {})).as_pointer(),
                Rc::new(RefCell::new(ForwardIteratorNotIncrementable {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for ForwardIteratorNotIncrementable {}
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
impl std::cmp::PartialEq for BidirectionalIteratorNotDerivedFrom {
    fn eq(&self, other: &Self) -> bool {
        {
            BidirectionalIteratorNotDerivedFromImpl::operator_eq(
                &Rc::new(RefCell::new(BidirectionalIteratorNotDerivedFrom {})).as_pointer(),
                Rc::new(RefCell::new(BidirectionalIteratorNotDerivedFrom {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for BidirectionalIteratorNotDerivedFrom {}
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
impl std::cmp::PartialEq for BidirectionalIteratorNotDecrementable {
    fn eq(&self, other: &Self) -> bool {
        {
            BidirectionalIteratorNotDecrementableImpl::operator_eq(
                &Rc::new(RefCell::new(BidirectionalIteratorNotDecrementable {})).as_pointer(),
                Rc::new(RefCell::new(BidirectionalIteratorNotDecrementable {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for BidirectionalIteratorNotDecrementable {}
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
impl std::cmp::PartialEq for PermutableNotSwappable {
    fn eq(&self, other: &Self) -> bool {
        {
            PermutableNotSwappableImpl::operator_eq(
                &Rc::new(RefCell::new(PermutableNotSwappable {})).as_pointer(),
                Rc::new(RefCell::new(PermutableNotSwappable {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for PermutableNotSwappable {}
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
impl std::cmp::Ord for RandomAccessIteratorNotDerivedFrom {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            RandomAccessIteratorNotDerivedFromImpl::operator_cmp(
                &Rc::new(RefCell::new(RandomAccessIteratorNotDerivedFrom {})).as_pointer(),
                Rc::new(RefCell::new(RandomAccessIteratorNotDerivedFrom {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for RandomAccessIteratorNotDerivedFrom {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for RandomAccessIteratorNotDerivedFrom {
    fn eq(&self, other: &Self) -> bool {
        {
            RandomAccessIteratorNotDerivedFromImpl::operator_cmp(
                &Rc::new(RefCell::new(RandomAccessIteratorNotDerivedFrom {})).as_pointer(),
                Rc::new(RefCell::new(RandomAccessIteratorNotDerivedFrom {})).as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for RandomAccessIteratorNotDerivedFrom {}
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
impl std::cmp::Ord for RandomAccessIteratorBadIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            RandomAccessIteratorBadIndexImpl::operator_cmp(
                &Rc::new(RefCell::new(RandomAccessIteratorBadIndex {})).as_pointer(),
                Rc::new(RefCell::new(RandomAccessIteratorBadIndex {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for RandomAccessIteratorBadIndex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for RandomAccessIteratorBadIndex {
    fn eq(&self, other: &Self) -> bool {
        {
            RandomAccessIteratorBadIndexImpl::operator_cmp(
                &Rc::new(RefCell::new(RandomAccessIteratorBadIndex {})).as_pointer(),
                Rc::new(RefCell::new(RandomAccessIteratorBadIndex {})).as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for RandomAccessIteratorBadIndex {}
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
impl std::cmp::Ord for RandomAccessIteratorBadDifferenceType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            RandomAccessIteratorBadDifferenceTypeImpl::operator_cmp(
                &Rc::new(RefCell::new(RandomAccessIteratorBadDifferenceType {})).as_pointer(),
                Rc::new(RefCell::new(RandomAccessIteratorBadDifferenceType {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for RandomAccessIteratorBadDifferenceType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for RandomAccessIteratorBadDifferenceType {
    fn eq(&self, other: &Self) -> bool {
        {
            RandomAccessIteratorBadDifferenceTypeImpl::operator_cmp(
                &Rc::new(RefCell::new(RandomAccessIteratorBadDifferenceType {})).as_pointer(),
                Rc::new(RefCell::new(RandomAccessIteratorBadDifferenceType {})).as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for RandomAccessIteratorBadDifferenceType {}
const _: () = assert!(true, "std::regular<RandomAccessIteratorBadDifferenceType>");
const _: () = assert!(
    !(false),
    "!std::weakly_incrementable<RandomAccessIteratorBadDifferenceType>"
);
const _: () = assert!(
    !(false),
    "!std::random_access_iterator<RandomAccessIteratorBadDifferenceType>"
);
const _: () = assert!(
    true,
    "HasPartialSortCopyIter<int*, int*, int*, int*, std::ranges::less>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<InputIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<InputIteratorNotIndirectlyReadable>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<InputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<int*, SentinelForNotSemiregular>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<int*, SentinelForNotWeaklyEqualityComparableWith>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<int*, int*, RandomAccessIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<int*, int*, RandomAccessIteratorBadIndex>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<int*, int*, int*, SentinelForNotSemiregular>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<int*, int*, int*, SentinelForNotWeaklyEqualityComparableWith>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<int*, int*, int*, int*, IndirectUnaryPredicateNotPredicate>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<int*, int*, int*, int*, IndirectUnaryPredicateNotCopyConstructible>"
);
pub fn operator_cmp_28(_a0: Ptr<OrderedConvertibleToInt>, _a1: Ptr<OrderedConvertibleToInt>) -> () {
    return unimplemented!();
}
pub fn operator_eq_29(_a0: Ptr<OrderedConvertibleToInt>, _a1: Ptr<OrderedConvertibleToInt>) -> () {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct OrderedConvertibleToInt {}
impl std::cmp::Ord for OrderedConvertibleToInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_28(
                Rc::new(RefCell::new(OrderedConvertibleToInt {})).as_pointer(),
                Rc::new(RefCell::new(OrderedConvertibleToInt {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for OrderedConvertibleToInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for OrderedConvertibleToInt {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_cmp_28(
                Rc::new(RefCell::new(OrderedConvertibleToInt {})).as_pointer(),
                Rc::new(RefCell::new(OrderedConvertibleToInt {})).as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for OrderedConvertibleToInt {}
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<int*, int*, OrderedConvertibleToInt*, OrderedConvertibleToInt*>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<int*, int*, const int*, const int*>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoComparator {}
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<NoComparator*, NoComparator*, NoComparator*, NoComparator*>"
);
const _: () = assert!(
    true,
    "HasPartialSortCopyIter<int*, int*, MoveOnly*, MoveOnly*>"
);
const _: () = assert!(
    true,
    "HasPartialSortCopyRange<R<int*>, R<int*>, std::ranges::less>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyRange<InputRangeNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyRange<InputRangeNotIndirectlyReadable>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyRange<InputRangeNotInputOrOutputIterator>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyRange<R<int*>, RandomAccessRangeNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyRange<R<int*>, RandomAccessRangeBadIndex>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyRange<R<int*>, R<OrderedConvertibleToInt*>>"
);
const _: () = assert!(!(false), "!HasPartialSortCopyRange<R<int*>, R<const int*>>");
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyRange<R<NoComparator*>, R<NoComparator*>>"
);
const _: () = assert!(true, "HasPartialSortCopyRange<R<int*>, R<MoveOnly*>>");
const _: () = assert!(
    true,
    "std::is_same_v<std::ranges::partial_sort_copy_result<int, int>, std::ranges::in_out_result<int, int>>"
);
pub fn test_one_30(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (0_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    cpp20_input_iterator_int_ptr_::new({ (begin).clone() }),
                    {
                        let __tmp_0: Value<cpp20_input_iterator_int_ptr_> =
                            Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                                (end).clone()
                            })));
                        sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({
                            __tmp_0.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_33(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (1_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    cpp20_input_iterator_int_ptr_::new({ (begin).clone() }),
                    {
                        let __tmp_1: Value<cpp20_input_iterator_int_ptr_> =
                            Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                                (end).clone()
                            })));
                        sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({
                            __tmp_1.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_34(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (2_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    cpp20_input_iterator_int_ptr_::new({ (begin).clone() }),
                    {
                        let __tmp_2: Value<cpp20_input_iterator_int_ptr_> =
                            Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                                (end).clone()
                            })));
                        sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({
                            __tmp_2.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_35(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (3_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    cpp20_input_iterator_int_ptr_::new({ (begin).clone() }),
                    {
                        let __tmp_3: Value<cpp20_input_iterator_int_ptr_> =
                            Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                                (end).clone()
                            })));
                        sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({
                            __tmp_3.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_36(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (8_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    cpp20_input_iterator_int_ptr_::new({ (begin).clone() }),
                    {
                        let __tmp_4: Value<cpp20_input_iterator_int_ptr_> =
                            Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                                (end).clone()
                            })));
                        sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({
                            __tmp_4.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_37(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (7_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    cpp20_input_iterator_int_ptr_::new({ (begin).clone() }),
                    {
                        let __tmp_5: Value<cpp20_input_iterator_int_ptr_> =
                            Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                                (end).clone()
                            })));
                        sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({
                            __tmp_5.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_38(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (5_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    cpp20_input_iterator_int_ptr_::new({ (begin).clone() }),
                    {
                        let __tmp_6: Value<cpp20_input_iterator_int_ptr_> =
                            Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                                (end).clone()
                            })));
                        sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({
                            __tmp_6.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_39(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (0_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    cpp20_input_iterator_int_ptr_::new({ (begin).clone() }),
                    {
                        let __tmp_7: Value<cpp20_input_iterator_int_ptr_> =
                            Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                                (end).clone()
                            })));
                        sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({
                            __tmp_7.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_8: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_8.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_40(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (1_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    cpp20_input_iterator_int_ptr_::new({ (begin).clone() }),
                    {
                        let __tmp_9: Value<cpp20_input_iterator_int_ptr_> =
                            Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                                (end).clone()
                            })));
                        sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({
                            __tmp_9.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_10: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_10.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_41(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (2_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    cpp20_input_iterator_int_ptr_::new({ (begin).clone() }),
                    {
                        let __tmp_11: Value<cpp20_input_iterator_int_ptr_> =
                            Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                                (end).clone()
                            })));
                        sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({
                            __tmp_11.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_12: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_12.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_42(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (3_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    cpp20_input_iterator_int_ptr_::new({ (begin).clone() }),
                    {
                        let __tmp_13: Value<cpp20_input_iterator_int_ptr_> =
                            Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                                (end).clone()
                            })));
                        sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({
                            __tmp_13.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_14: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_14.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_43(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (8_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    cpp20_input_iterator_int_ptr_::new({ (begin).clone() }),
                    {
                        let __tmp_15: Value<cpp20_input_iterator_int_ptr_> =
                            Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                                (end).clone()
                            })));
                        sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({
                            __tmp_15.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_16: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_16.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_44(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (7_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    cpp20_input_iterator_int_ptr_::new({ (begin).clone() }),
                    {
                        let __tmp_17: Value<cpp20_input_iterator_int_ptr_> =
                            Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                                (end).clone()
                            })));
                        sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({
                            __tmp_17.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_18: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_18.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_45(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (5_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    cpp20_input_iterator_int_ptr_::new({ (begin).clone() }),
                    {
                        let __tmp_19: Value<cpp20_input_iterator_int_ptr_> =
                            Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                                (end).clone()
                            })));
                        sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({
                            __tmp_19.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_20: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_20.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_46(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (0_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (end).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_47(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (1_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (end).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_48(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (2_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (end).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_49(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (3_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (end).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_50(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (8_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (end).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_51(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (7_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (end).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_52(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (5_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (end).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_53(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (0_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (end).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_21: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_21.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_54(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (1_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (end).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_22: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_22.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_55(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (2_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (end).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_23: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_23.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_56(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (3_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (end).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_24: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_24.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_57(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (8_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (end).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_25: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_25.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_58(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (7_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (end).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_26: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_26.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_59(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (5_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (end).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_27: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_27.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_60(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (0_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    {
                        let __tmp_28: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_28.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_61(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (1_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    {
                        let __tmp_29: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_29.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_62(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (2_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    {
                        let __tmp_30: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_30.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_63(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (3_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    {
                        let __tmp_31: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_31.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_64(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (8_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    {
                        let __tmp_32: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_32.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_65(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (7_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    {
                        let __tmp_33: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_33.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_66(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (5_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    {
                        let __tmp_34: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_34.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_67(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (0_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    {
                        let __tmp_35: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_35.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_36: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_36.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_68(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (1_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    {
                        let __tmp_37: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_37.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_38: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_38.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_69(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (2_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    {
                        let __tmp_39: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_39.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_40: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_40.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_70(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (3_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    {
                        let __tmp_41: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_41.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_42: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_42.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_71(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (8_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    {
                        let __tmp_43: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_43.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_44: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_44.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_72(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (7_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    {
                        let __tmp_45: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_45.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_46: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_46.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_73(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (5_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::new_1({ (begin).clone() }),
                    {
                        let __tmp_47: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_47.as_pointer()
                        })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_48: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_48.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_74(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (0_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_75(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (1_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_76(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (2_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_77(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (3_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_78(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (8_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_79(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (7_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_80(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (5_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_81(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (0_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_49: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_49.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_82(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (1_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_50: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_50.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_83(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (2_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_51: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_51.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_84(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (3_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_52: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_52.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_85(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (8_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_53: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_53.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_86(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (7_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_54: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_54.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_87(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (5_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    (end).clone(),
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_55: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_55.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_88(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (0_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    {
                        let __tmp_56: Value<Ptr<()>> = Rc::new(RefCell::new((end).clone()));
                        sentinel_wrapper_const_int_ptr_::new_1({ __tmp_56.as_pointer() })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_89(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (1_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    {
                        let __tmp_57: Value<Ptr<()>> = Rc::new(RefCell::new((end).clone()));
                        sentinel_wrapper_const_int_ptr_::new_1({ __tmp_57.as_pointer() })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_90(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (2_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    {
                        let __tmp_58: Value<Ptr<()>> = Rc::new(RefCell::new((end).clone()));
                        sentinel_wrapper_const_int_ptr_::new_1({ __tmp_58.as_pointer() })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_91(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (3_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    {
                        let __tmp_59: Value<Ptr<()>> = Rc::new(RefCell::new((end).clone()));
                        sentinel_wrapper_const_int_ptr_::new_1({ __tmp_59.as_pointer() })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_92(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (8_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    {
                        let __tmp_60: Value<Ptr<()>> = Rc::new(RefCell::new((end).clone()));
                        sentinel_wrapper_const_int_ptr_::new_1({ __tmp_60.as_pointer() })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_93(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (7_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    {
                        let __tmp_61: Value<Ptr<()>> = Rc::new(RefCell::new((end).clone()));
                        sentinel_wrapper_const_int_ptr_::new_1({ __tmp_61.as_pointer() })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_94(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (5_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    {
                        let __tmp_62: Value<Ptr<()>> = Rc::new(RefCell::new((end).clone()));
                        sentinel_wrapper_const_int_ptr_::new_1({ __tmp_62.as_pointer() })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    random_access_iterator_int_ptr_::new_1({ (out_end).clone() }),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_95(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (0_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    {
                        let __tmp_63: Value<Ptr<()>> = Rc::new(RefCell::new((end).clone()));
                        sentinel_wrapper_const_int_ptr_::new_1({ __tmp_63.as_pointer() })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_64: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_64.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_96(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (1_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    {
                        let __tmp_65: Value<Ptr<()>> = Rc::new(RefCell::new((end).clone()));
                        sentinel_wrapper_const_int_ptr_::new_1({ __tmp_65.as_pointer() })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_66: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_66.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_97(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (2_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    {
                        let __tmp_67: Value<Ptr<()>> = Rc::new(RefCell::new((end).clone()));
                        sentinel_wrapper_const_int_ptr_::new_1({ __tmp_67.as_pointer() })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_68: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_68.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_98(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (3_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    {
                        let __tmp_69: Value<Ptr<()>> = Rc::new(RefCell::new((end).clone()));
                        sentinel_wrapper_const_int_ptr_::new_1({ __tmp_69.as_pointer() })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_70: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_70.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_99(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (8_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    {
                        let __tmp_71: Value<Ptr<()>> = Rc::new(RefCell::new((end).clone()));
                        sentinel_wrapper_const_int_ptr_::new_1({ __tmp_71.as_pointer() })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_72: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_72.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_100(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (7_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    {
                        let __tmp_73: Value<Ptr<()>> = Rc::new(RefCell::new((end).clone()));
                        sentinel_wrapper_const_int_ptr_::new_1({ __tmp_73.as_pointer() })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_74: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_74.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_one_101(input: (), input_size: (), output_size: (), sorted: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let input_size: Value<()> = Rc::new(RefCell::new(input_size));
    let output_size: Value<()> = Rc::new(RefCell::new(output_size));
    let sorted: Value<()> = Rc::new(RefCell::new(sorted));
    unimplemented!();
    unimplemented!();
    let mut OutputSize: () = (5_() as ()).wrapping_add(1_());
    let mut result_size: () = ({
        (*min_31.with(Value::clone).borrow_mut())(
            input_size.as_pointer(),
            output_size.as_pointer(),
            None,
            None,
        )
    });
    let mut begin: Ptr<()> = unimplemented!();
    let mut end: Ptr<()> = unimplemented!().offset((*input_size.borrow()) as isize);
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    (begin).clone(),
                    {
                        let __tmp_75: Value<Ptr<()>> = Rc::new(RefCell::new((end).clone()));
                        sentinel_wrapper_const_int_ptr_::new_1({ __tmp_75.as_pointer() })
                    },
                    random_access_iterator_int_ptr_::new_1({ (out_begin).clone() }),
                    {
                        let __tmp_76: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (out_end).clone()
                            })));
                        sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                            __tmp_76.as_pointer()
                        })
                    },
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut out_begin: Ptr<()> = unimplemented!();
        let mut out_end: Ptr<()> = unimplemented!().offset((*output_size.borrow()) as isize);
        let in_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let out_range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_range.as_pointer(),
                    out_range.as_pointer(),
                    None,
                    None,
                    None,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_all_subsequences_102(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (0_() as ()).wrapping_add(1_())) {
        ({
            test_one_30(
                (*input.borrow()).clone(),
                0_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_103(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (1_() as ()).wrapping_add(1_())) {
        ({
            test_one_33(
                (*input.borrow()).clone(),
                1_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_104(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (2_() as ()).wrapping_add(1_())) {
        ({
            test_one_34(
                (*input.borrow()).clone(),
                2_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_105(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (3_() as ()).wrapping_add(1_())) {
        ({
            test_one_35(
                (*input.borrow()).clone(),
                3_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_106(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (8_() as ()).wrapping_add(1_())) {
        ({
            test_one_36(
                (*input.borrow()).clone(),
                8_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_107(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (7_() as ()).wrapping_add(1_())) {
        ({
            test_one_37(
                (*input.borrow()).clone(),
                7_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_108(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (5_() as ()).wrapping_add(1_())) {
        ({
            test_one_38(
                (*input.borrow()).clone(),
                5_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_109(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (0_() as ()).wrapping_add(1_())) {
        ({
            test_one_39(
                (*input.borrow()).clone(),
                0_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_110(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (1_() as ()).wrapping_add(1_())) {
        ({
            test_one_40(
                (*input.borrow()).clone(),
                1_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_111(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (2_() as ()).wrapping_add(1_())) {
        ({
            test_one_41(
                (*input.borrow()).clone(),
                2_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_112(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (3_() as ()).wrapping_add(1_())) {
        ({
            test_one_42(
                (*input.borrow()).clone(),
                3_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_113(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (8_() as ()).wrapping_add(1_())) {
        ({
            test_one_43(
                (*input.borrow()).clone(),
                8_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_114(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (7_() as ()).wrapping_add(1_())) {
        ({
            test_one_44(
                (*input.borrow()).clone(),
                7_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_115(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (5_() as ()).wrapping_add(1_())) {
        ({
            test_one_45(
                (*input.borrow()).clone(),
                5_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_116(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (0_() as ()).wrapping_add(1_())) {
        ({
            test_one_46(
                (*input.borrow()).clone(),
                0_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_117(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (1_() as ()).wrapping_add(1_())) {
        ({
            test_one_47(
                (*input.borrow()).clone(),
                1_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_118(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (2_() as ()).wrapping_add(1_())) {
        ({
            test_one_48(
                (*input.borrow()).clone(),
                2_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_119(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (3_() as ()).wrapping_add(1_())) {
        ({
            test_one_49(
                (*input.borrow()).clone(),
                3_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_120(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (8_() as ()).wrapping_add(1_())) {
        ({
            test_one_50(
                (*input.borrow()).clone(),
                8_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_121(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (7_() as ()).wrapping_add(1_())) {
        ({
            test_one_51(
                (*input.borrow()).clone(),
                7_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_122(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (5_() as ()).wrapping_add(1_())) {
        ({
            test_one_52(
                (*input.borrow()).clone(),
                5_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_123(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (0_() as ()).wrapping_add(1_())) {
        ({
            test_one_53(
                (*input.borrow()).clone(),
                0_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_124(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (1_() as ()).wrapping_add(1_())) {
        ({
            test_one_54(
                (*input.borrow()).clone(),
                1_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_125(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (2_() as ()).wrapping_add(1_())) {
        ({
            test_one_55(
                (*input.borrow()).clone(),
                2_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_126(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (3_() as ()).wrapping_add(1_())) {
        ({
            test_one_56(
                (*input.borrow()).clone(),
                3_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_127(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (8_() as ()).wrapping_add(1_())) {
        ({
            test_one_57(
                (*input.borrow()).clone(),
                8_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_128(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (7_() as ()).wrapping_add(1_())) {
        ({
            test_one_58(
                (*input.borrow()).clone(),
                7_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_129(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (5_() as ()).wrapping_add(1_())) {
        ({
            test_one_59(
                (*input.borrow()).clone(),
                5_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_130(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (0_() as ()).wrapping_add(1_())) {
        ({
            test_one_60(
                (*input.borrow()).clone(),
                0_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_131(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (1_() as ()).wrapping_add(1_())) {
        ({
            test_one_61(
                (*input.borrow()).clone(),
                1_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_132(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (2_() as ()).wrapping_add(1_())) {
        ({
            test_one_62(
                (*input.borrow()).clone(),
                2_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_133(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (3_() as ()).wrapping_add(1_())) {
        ({
            test_one_63(
                (*input.borrow()).clone(),
                3_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_134(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (8_() as ()).wrapping_add(1_())) {
        ({
            test_one_64(
                (*input.borrow()).clone(),
                8_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_135(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (7_() as ()).wrapping_add(1_())) {
        ({
            test_one_65(
                (*input.borrow()).clone(),
                7_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_136(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (5_() as ()).wrapping_add(1_())) {
        ({
            test_one_66(
                (*input.borrow()).clone(),
                5_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_137(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (0_() as ()).wrapping_add(1_())) {
        ({
            test_one_67(
                (*input.borrow()).clone(),
                0_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_138(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (1_() as ()).wrapping_add(1_())) {
        ({
            test_one_68(
                (*input.borrow()).clone(),
                1_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_139(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (2_() as ()).wrapping_add(1_())) {
        ({
            test_one_69(
                (*input.borrow()).clone(),
                2_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_140(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (3_() as ()).wrapping_add(1_())) {
        ({
            test_one_70(
                (*input.borrow()).clone(),
                3_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_141(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (8_() as ()).wrapping_add(1_())) {
        ({
            test_one_71(
                (*input.borrow()).clone(),
                8_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_142(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (7_() as ()).wrapping_add(1_())) {
        ({
            test_one_72(
                (*input.borrow()).clone(),
                7_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_143(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (5_() as ()).wrapping_add(1_())) {
        ({
            test_one_73(
                (*input.borrow()).clone(),
                5_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_144(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (0_() as ()).wrapping_add(1_())) {
        ({
            test_one_74(
                (*input.borrow()).clone(),
                0_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_145(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (1_() as ()).wrapping_add(1_())) {
        ({
            test_one_75(
                (*input.borrow()).clone(),
                1_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_146(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (2_() as ()).wrapping_add(1_())) {
        ({
            test_one_76(
                (*input.borrow()).clone(),
                2_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_147(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (3_() as ()).wrapping_add(1_())) {
        ({
            test_one_77(
                (*input.borrow()).clone(),
                3_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_148(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (8_() as ()).wrapping_add(1_())) {
        ({
            test_one_78(
                (*input.borrow()).clone(),
                8_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_149(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (7_() as ()).wrapping_add(1_())) {
        ({
            test_one_79(
                (*input.borrow()).clone(),
                7_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_150(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (5_() as ()).wrapping_add(1_())) {
        ({
            test_one_80(
                (*input.borrow()).clone(),
                5_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_151(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (0_() as ()).wrapping_add(1_())) {
        ({
            test_one_81(
                (*input.borrow()).clone(),
                0_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_152(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (1_() as ()).wrapping_add(1_())) {
        ({
            test_one_82(
                (*input.borrow()).clone(),
                1_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_153(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (2_() as ()).wrapping_add(1_())) {
        ({
            test_one_83(
                (*input.borrow()).clone(),
                2_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_154(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (3_() as ()).wrapping_add(1_())) {
        ({
            test_one_84(
                (*input.borrow()).clone(),
                3_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_155(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (8_() as ()).wrapping_add(1_())) {
        ({
            test_one_85(
                (*input.borrow()).clone(),
                8_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_156(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (7_() as ()).wrapping_add(1_())) {
        ({
            test_one_86(
                (*input.borrow()).clone(),
                7_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_157(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (5_() as ()).wrapping_add(1_())) {
        ({
            test_one_87(
                (*input.borrow()).clone(),
                5_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_158(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (0_() as ()).wrapping_add(1_())) {
        ({
            test_one_88(
                (*input.borrow()).clone(),
                0_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_159(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (1_() as ()).wrapping_add(1_())) {
        ({
            test_one_89(
                (*input.borrow()).clone(),
                1_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_160(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (2_() as ()).wrapping_add(1_())) {
        ({
            test_one_90(
                (*input.borrow()).clone(),
                2_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_161(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (3_() as ()).wrapping_add(1_())) {
        ({
            test_one_91(
                (*input.borrow()).clone(),
                3_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_162(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (8_() as ()).wrapping_add(1_())) {
        ({
            test_one_92(
                (*input.borrow()).clone(),
                8_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_163(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (7_() as ()).wrapping_add(1_())) {
        ({
            test_one_93(
                (*input.borrow()).clone(),
                7_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_164(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (5_() as ()).wrapping_add(1_())) {
        ({
            test_one_94(
                (*input.borrow()).clone(),
                5_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_165(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (0_() as ()).wrapping_add(1_())) {
        ({
            test_one_95(
                (*input.borrow()).clone(),
                0_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_166(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (1_() as ()).wrapping_add(1_())) {
        ({
            test_one_96(
                (*input.borrow()).clone(),
                1_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_167(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (2_() as ()).wrapping_add(1_())) {
        ({
            test_one_97(
                (*input.borrow()).clone(),
                2_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_168(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (3_() as ()).wrapping_add(1_())) {
        ({
            test_one_98(
                (*input.borrow()).clone(),
                3_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_169(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (8_() as ()).wrapping_add(1_())) {
        ({
            test_one_99(
                (*input.borrow()).clone(),
                8_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_170(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (7_() as ()).wrapping_add(1_())) {
        ({
            test_one_100(
                (*input.borrow()).clone(),
                7_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_all_subsequences_171(input: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let sorted: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while (out_size <= (5_() as ()).wrapping_add(1_())) {
        ({
            test_one_101(
                (*input.borrow()).clone(),
                5_(),
                out_size,
                (*sorted.borrow()).clone(),
            )
        });
        out_size.prefix_inc();
    }
}
pub fn test_iterators_in_sent1_out_sent2_172() {
    ({ test_all_subsequences_102(unimplemented!()) });
    ({ test_all_subsequences_103(unimplemented!()) });
    ({ test_all_subsequences_104(unimplemented!()) });
    ({ test_all_subsequences_105(unimplemented!()) });
    ({ test_all_subsequences_106(unimplemented!()) });
    ({ test_all_subsequences_107(unimplemented!()) });
    ({ test_all_subsequences_105(unimplemented!()) });
    ({ test_all_subsequences_108(unimplemented!()) });
    ({ test_all_subsequences_108(unimplemented!()) });
}
pub fn test_iterators_in_sent1_out_sent2_173() {
    ({ test_all_subsequences_109(unimplemented!()) });
    ({ test_all_subsequences_110(unimplemented!()) });
    ({ test_all_subsequences_111(unimplemented!()) });
    ({ test_all_subsequences_112(unimplemented!()) });
    ({ test_all_subsequences_113(unimplemented!()) });
    ({ test_all_subsequences_114(unimplemented!()) });
    ({ test_all_subsequences_112(unimplemented!()) });
    ({ test_all_subsequences_115(unimplemented!()) });
    ({ test_all_subsequences_115(unimplemented!()) });
}
pub fn test_iterators_in_sent1_out_sent2_174() {
    ({ test_all_subsequences_116(unimplemented!()) });
    ({ test_all_subsequences_117(unimplemented!()) });
    ({ test_all_subsequences_118(unimplemented!()) });
    ({ test_all_subsequences_119(unimplemented!()) });
    ({ test_all_subsequences_120(unimplemented!()) });
    ({ test_all_subsequences_121(unimplemented!()) });
    ({ test_all_subsequences_119(unimplemented!()) });
    ({ test_all_subsequences_122(unimplemented!()) });
    ({ test_all_subsequences_122(unimplemented!()) });
}
pub fn test_iterators_in_sent1_out_sent2_175() {
    ({ test_all_subsequences_123(unimplemented!()) });
    ({ test_all_subsequences_124(unimplemented!()) });
    ({ test_all_subsequences_125(unimplemented!()) });
    ({ test_all_subsequences_126(unimplemented!()) });
    ({ test_all_subsequences_127(unimplemented!()) });
    ({ test_all_subsequences_128(unimplemented!()) });
    ({ test_all_subsequences_126(unimplemented!()) });
    ({ test_all_subsequences_129(unimplemented!()) });
    ({ test_all_subsequences_129(unimplemented!()) });
}
pub fn test_iterators_in_sent1_out_sent2_176() {
    ({ test_all_subsequences_130(unimplemented!()) });
    ({ test_all_subsequences_131(unimplemented!()) });
    ({ test_all_subsequences_132(unimplemented!()) });
    ({ test_all_subsequences_133(unimplemented!()) });
    ({ test_all_subsequences_134(unimplemented!()) });
    ({ test_all_subsequences_135(unimplemented!()) });
    ({ test_all_subsequences_133(unimplemented!()) });
    ({ test_all_subsequences_136(unimplemented!()) });
    ({ test_all_subsequences_136(unimplemented!()) });
}
pub fn test_iterators_in_sent1_out_sent2_177() {
    ({ test_all_subsequences_137(unimplemented!()) });
    ({ test_all_subsequences_138(unimplemented!()) });
    ({ test_all_subsequences_139(unimplemented!()) });
    ({ test_all_subsequences_140(unimplemented!()) });
    ({ test_all_subsequences_141(unimplemented!()) });
    ({ test_all_subsequences_142(unimplemented!()) });
    ({ test_all_subsequences_140(unimplemented!()) });
    ({ test_all_subsequences_143(unimplemented!()) });
    ({ test_all_subsequences_143(unimplemented!()) });
}
pub fn test_iterators_in_sent1_out_sent2_178() {
    ({ test_all_subsequences_144(unimplemented!()) });
    ({ test_all_subsequences_145(unimplemented!()) });
    ({ test_all_subsequences_146(unimplemented!()) });
    ({ test_all_subsequences_147(unimplemented!()) });
    ({ test_all_subsequences_148(unimplemented!()) });
    ({ test_all_subsequences_149(unimplemented!()) });
    ({ test_all_subsequences_147(unimplemented!()) });
    ({ test_all_subsequences_150(unimplemented!()) });
    ({ test_all_subsequences_150(unimplemented!()) });
}
pub fn test_iterators_in_sent1_out_sent2_179() {
    ({ test_all_subsequences_151(unimplemented!()) });
    ({ test_all_subsequences_152(unimplemented!()) });
    ({ test_all_subsequences_153(unimplemented!()) });
    ({ test_all_subsequences_154(unimplemented!()) });
    ({ test_all_subsequences_155(unimplemented!()) });
    ({ test_all_subsequences_156(unimplemented!()) });
    ({ test_all_subsequences_154(unimplemented!()) });
    ({ test_all_subsequences_157(unimplemented!()) });
    ({ test_all_subsequences_157(unimplemented!()) });
}
pub fn test_iterators_in_sent1_out_sent2_180() {
    ({ test_all_subsequences_158(unimplemented!()) });
    ({ test_all_subsequences_159(unimplemented!()) });
    ({ test_all_subsequences_160(unimplemented!()) });
    ({ test_all_subsequences_161(unimplemented!()) });
    ({ test_all_subsequences_162(unimplemented!()) });
    ({ test_all_subsequences_163(unimplemented!()) });
    ({ test_all_subsequences_161(unimplemented!()) });
    ({ test_all_subsequences_164(unimplemented!()) });
    ({ test_all_subsequences_164(unimplemented!()) });
}
pub fn test_iterators_in_sent1_out_sent2_181() {
    ({ test_all_subsequences_165(unimplemented!()) });
    ({ test_all_subsequences_166(unimplemented!()) });
    ({ test_all_subsequences_167(unimplemented!()) });
    ({ test_all_subsequences_168(unimplemented!()) });
    ({ test_all_subsequences_169(unimplemented!()) });
    ({ test_all_subsequences_170(unimplemented!()) });
    ({ test_all_subsequences_168(unimplemented!()) });
    ({ test_all_subsequences_171(unimplemented!()) });
    ({ test_all_subsequences_171(unimplemented!()) });
}
pub fn test_iterators_in_sent1_out_182() {
    ({ test_iterators_in_sent1_out_sent2_172() });
    ({ test_iterators_in_sent1_out_sent2_173() });
}
pub fn test_iterators_in_sent1_out_183() {
    ({ test_iterators_in_sent1_out_sent2_174() });
    ({ test_iterators_in_sent1_out_sent2_175() });
}
pub fn test_iterators_in_sent1_out_184() {
    ({ test_iterators_in_sent1_out_sent2_176() });
    ({ test_iterators_in_sent1_out_sent2_177() });
}
pub fn test_iterators_in_sent1_out_185() {
    ({ test_iterators_in_sent1_out_sent2_178() });
    ({ test_iterators_in_sent1_out_sent2_179() });
}
pub fn test_iterators_in_sent1_out_186() {
    ({ test_iterators_in_sent1_out_sent2_180() });
    ({ test_iterators_in_sent1_out_sent2_181() });
}
pub fn test_iterators_in_sent1_187() {
    ({ test_iterators_in_sent1_out_182() });
}
pub fn test_iterators_in_sent1_188() {
    ({ test_iterators_in_sent1_out_183() });
}
pub fn test_iterators_in_sent1_189() {
    ({ test_iterators_in_sent1_out_184() });
}
pub fn test_iterators_in_sent1_190() {
    ({ test_iterators_in_sent1_out_185() });
}
pub fn test_iterators_in_sent1_191() {
    ({ test_iterators_in_sent1_out_186() });
}
pub fn test_iterators_in_192() {
    {}
    ({ test_iterators_in_sent1_187() });
}
pub fn test_iterators_in_193() {
    {
        ({ test_iterators_in_sent1_188() });
    }
    ({ test_iterators_in_sent1_189() });
}
pub fn test_iterators_in_194() {
    {
        ({ test_iterators_in_sent1_190() });
    }
    ({ test_iterators_in_sent1_191() });
}
pub fn test_iterators_195() {
    ({ test_iterators_in_192() });
    ({ test_iterators_in_193() });
    ({ test_iterators_in_194() });
}
pub fn test_196() -> () {
    ({ test_iterators_195() });
    {
        let in_: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let comp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!();
                    let ___result_first: Ptr<()> = unimplemented!();
                    let ___result_last: Ptr<()> = unimplemented!();
                    (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        ___result_first,
                        ___result_last,
                        unimplemented!(),
                        None,
                        None,
                    )
                }),
            ));
            unimplemented!();
        }
        {
            let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                        in_.as_pointer(),
                        out.as_pointer(),
                        unimplemented!(),
                        None,
                        None,
                    )
                }),
            ));
            unimplemented!();
        }
    }
    {
        let in_: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let proj1: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
            |mut value: ()| -> () {
                {
                    return (value * -1_());
                }
            },
        )));
        let proj2: Value<FnPtr<fn(A_197) -> ()>> = Rc::new(RefCell::new(
            FnPtr::<fn(A_197) -> ()>::new(|mut value: A_197| -> () {
                {
                    return (value.a * -1_());
                }
            }),
        ));
        {
            let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!();
                    let ___result_first: Ptr<A_197> = unimplemented!();
                    let ___result_last: Ptr<A_197> = unimplemented!();
                    (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        ___result_first,
                        ___result_last,
                        unimplemented!(),
                        None,
                        None,
                    )
                }),
            ));
            unimplemented!();
            (*result.borrow_mut()) = ({
                let ___first: Ptr<()> = unimplemented!();
                let ___last: Ptr<()> = unimplemented!();
                let ___result_first: Ptr<A_197> = unimplemented!();
                let ___result_last: Ptr<A_197> = unimplemented!();
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    ___first,
                    ___last,
                    ___result_first,
                    ___result_last,
                    unimplemented!(),
                    (*proj1.borrow()).copy_from(),
                    (*proj2.borrow()).copy_from(),
                )
            });
            unimplemented!();
        }
        {
            let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                        in_.as_pointer(),
                        out.as_pointer(),
                        unimplemented!(),
                        None,
                        None,
                    )
                }),
            ));
            unimplemented!();
            (*result.borrow_mut()) = ({
                (*partial_sort_copy_32.with(Value::clone).borrow_mut())(
                    in_.as_pointer(),
                    out.as_pointer(),
                    unimplemented!(),
                    (*proj1.borrow()).copy_from(),
                    (*proj2.borrow()).copy_from(),
                )
            });
            unimplemented!();
        }
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct A_197 {
    #[offset(0)]
    pub a: (),
}
impl A_197 {
    pub fn new_1(mut value: ()) -> Self {
        Self { a: value }
    }
}
impl std::cmp::Ord for A_197 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            A_197Impl::operator_cmp(
                &Rc::new(RefCell::new(A_197 { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(A_197 { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for A_197 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for A_197 {
    fn eq(&self, other: &Self) -> bool {
        {
            A_197Impl::operator_cmp(
                &Rc::new(RefCell::new(A_197 { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(A_197 { a: other.a.clone() })).as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for A_197 {}
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
    ({ test_196() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait A_197Impl {
    fn operator_cmp(&self, _a0: Ptr<A_197>) -> ();
    fn operator_eq(&self, _a0: Ptr<A_197>) -> ();
}
impl A_197Impl for Ptr<A_197> {
    fn operator_cmp(&self, _a0: Ptr<A_197>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.a)),
                &(_a0.with(|__s| __s.a)),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<A_197>) -> () {
        return ({ (*self).with(|__s| __s.a) } == { _a0.with(|__s| __s.a) });
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
pub trait MoveOnlyImpl {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
    fn get(&self) -> ();
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly;
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        field!((*self), data_).write({ x.with(|__s| __s.data_) });
        field!(x, data_).write(0_());
        return (*self).clone();
    }
    fn get(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly {
        return MoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } + { x.with(|__s| __s.data_) }))
        });
    }
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly {
        return MoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } * { x.with(|__s| __s.data_) }))
        });
    }
}
pub trait OrderedConvertibleToIntImpl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl OrderedConvertibleToIntImpl for Ptr<OrderedConvertibleToInt> {}
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
pub trait TrivialMoveOnlyImpl {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly>;
    fn get(&self) -> ();
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
}
impl TrivialMoveOnlyImpl for Ptr<TrivialMoveOnly> {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly> {
        field!((*self), data_).write({ { (*_a0.upgrade().deref()).data_ } });
        return (*self).clone();
    }
    fn get(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } + { x.with(|__s| __s.data_) }))
        });
    }
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } * { x.with(|__s| __s.data_) }))
        });
    }
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
pub trait UncheckedRange_MoveOnly_ptr__sentinel_wrapper_MoveOnly_ptr__Impl {
    fn begin(&self) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_MoveOnly_ptr_ {
        unimplemented!()
    }
}
impl UncheckedRange_MoveOnly_ptr__sentinel_wrapper_MoveOnly_ptr__Impl
    for Ptr<UncheckedRange_MoveOnly_ptr__sentinel_wrapper_MoveOnly_ptr__>
{
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
pub trait UncheckedRange_OrderedConvertibleToInt_ptr__sentinel_wrapper_OrderedConvertibleToInt_ptr__Impl
{
    fn begin(&self) -> Ptr<OrderedConvertibleToInt> {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_OrderedConvertibleToInt_ptr_ {
        unimplemented!()
    }
}
impl UncheckedRange_OrderedConvertibleToInt_ptr__sentinel_wrapper_OrderedConvertibleToInt_ptr__Impl
    for Ptr<
        UncheckedRange_OrderedConvertibleToInt_ptr__sentinel_wrapper_OrderedConvertibleToInt_ptr__,
    >
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
pub trait UncheckedRange_const_int_ptr__sentinel_wrapper_const_int_ptr__Impl {
    fn begin(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_const_int_ptr_ {
        unimplemented!()
    }
}
impl UncheckedRange_const_int_ptr__sentinel_wrapper_const_int_ptr__Impl
    for Ptr<UncheckedRange_const_int_ptr__sentinel_wrapper_const_int_ptr__>
{
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
    fn operator_inc_5(&self) -> Ptr<cpp20_input_iterator_int_ptr_>;
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
    fn operator_inc_5(&self) -> Ptr<cpp20_input_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
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
    fn operator_deref(&self) -> ();
    fn operator_index(&self, n: ()) -> ();
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_int_ptr_>;
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
    fn operator_index(&self, mut n: ()) -> () {
        return (*self).with(|__s| __s.it_.clone()).offset((n) as isize);
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_dec());
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
pub trait sentinel_wrapper_MoveOnly_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<MoveOnly>>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_MoveOnly_ptr_Impl for Ptr<sentinel_wrapper_MoveOnly_ptr_> {}
pub trait sentinel_wrapper_NoComparator_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<NoComparator>>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_NoComparator_ptr_Impl for Ptr<sentinel_wrapper_NoComparator_ptr_> {}
pub trait sentinel_wrapper_OrderedConvertibleToInt_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<OrderedConvertibleToInt>>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_OrderedConvertibleToInt_ptr_Impl
    for Ptr<sentinel_wrapper_OrderedConvertibleToInt_ptr_>
{
}
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
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_19((other.read())) }) });
    }
}
pub trait sentinel_wrapper_cpp20_input_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<cpp20_input_iterator_int_ptr_>) -> ();
}
impl sentinel_wrapper_cpp20_input_iterator_int_ptr__Impl
    for Ptr<sentinel_wrapper_cpp20_input_iterator_int_ptr__>
{
    fn operator_eq(&self, other: Ptr<cpp20_input_iterator_int_ptr_>) -> () {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_20((other).clone()) }) });
    }
}
pub trait sentinel_wrapper_int_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_int_ptr_Impl for Ptr<sentinel_wrapper_int_ptr_> {}
pub trait sentinel_wrapper_random_access_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<random_access_iterator_int_ptr_>) -> ();
}
impl sentinel_wrapper_random_access_iterator_int_ptr__Impl
    for Ptr<sentinel_wrapper_random_access_iterator_int_ptr__>
{
    fn operator_eq(&self, other: Ptr<random_access_iterator_int_ptr_>) -> () {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_18((other).clone()) }) });
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
