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
pub fn operator_ne_14(
    x: Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>,
    y: Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    pub fn new(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
}
pub fn operator_ne_15(
    x: Ptr<cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_>,
    y: Ptr<cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ {
    pub fn new(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub fn operator_ne_16(
    x: Ptr<forward_iterator_const_unsigned_int_ptr_>,
    y: Ptr<forward_iterator_const_unsigned_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct forward_iterator_const_unsigned_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_unsigned_int_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
}
pub fn operator_ne_17(
    x: Ptr<forward_iterator_const_int_ptr_>,
    y: Ptr<forward_iterator_const_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct forward_iterator_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_int_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
}
pub fn operator_ne_18(
    x: Ptr<bidirectional_iterator_const_unsigned_int_ptr_>,
    y: Ptr<bidirectional_iterator_const_unsigned_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct bidirectional_iterator_const_unsigned_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_const_unsigned_int_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
}
pub fn operator_ne_19(
    x: Ptr<bidirectional_iterator_const_int_ptr_>,
    y: Ptr<bidirectional_iterator_const_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct bidirectional_iterator_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_const_int_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
}
pub fn operator_ne_20(
    x: Ptr<random_access_iterator_const_unsigned_int_ptr_>,
    y: Ptr<random_access_iterator_const_unsigned_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct random_access_iterator_const_unsigned_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_const_unsigned_int_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
}
pub fn operator_ne_21(
    x: Ptr<random_access_iterator_const_int_ptr_>,
    y: Ptr<random_access_iterator_const_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct random_access_iterator_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_const_int_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
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
pub fn operator_eq_22(
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
            operator_eq_22(
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
pub fn operator_add_23(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_24(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_23(_i, n)
    });
}
pub fn iter_swap_25(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_26(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return (unimplemented!()).reinterpret_cast::<()>();
}
pub fn operator_eq_27(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_28(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
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
            operator_cmp_28(
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
            operator_eq_27(
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
pub fn test_29(
    mut first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut first2: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_30(
    mut first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_31(
    mut first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut first2: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_32(
    mut first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_33(
    mut first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut first2: forward_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_34(
    mut first1: forward_iterator_const_unsigned_int_ptr_,
    mut last1: forward_iterator_const_unsigned_int_ptr_,
    mut first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_35(
    mut first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut first2: forward_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_36(
    mut first1: forward_iterator_const_unsigned_int_ptr_,
    mut last1: forward_iterator_const_unsigned_int_ptr_,
    mut first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_37(
    mut first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut first2: bidirectional_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_38(
    mut first1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut last1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_39(
    mut first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut first2: bidirectional_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_40(
    mut first1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut last1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_41(
    mut first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut first2: random_access_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_42(
    mut first1: random_access_iterator_const_unsigned_int_ptr_,
    mut last1: random_access_iterator_const_unsigned_int_ptr_,
    mut first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_43(
    mut first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut first2: random_access_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_44(
    mut first1: random_access_iterator_const_unsigned_int_ptr_,
    mut last1: random_access_iterator_const_unsigned_int_ptr_,
    mut first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_45(
    mut first1: forward_iterator_const_int_ptr_,
    mut last1: forward_iterator_const_int_ptr_,
    mut first2: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_46(
    mut first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut first2: forward_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_47(
    mut first1: forward_iterator_const_int_ptr_,
    mut last1: forward_iterator_const_int_ptr_,
    mut first2: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_48(
    mut first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut first2: forward_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_49(
    mut first1: forward_iterator_const_int_ptr_,
    mut last1: forward_iterator_const_int_ptr_,
    mut first2: forward_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_50(
    mut first1: forward_iterator_const_unsigned_int_ptr_,
    mut last1: forward_iterator_const_unsigned_int_ptr_,
    mut first2: forward_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_51(
    mut first1: forward_iterator_const_int_ptr_,
    mut last1: forward_iterator_const_int_ptr_,
    mut first2: forward_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_52(
    mut first1: forward_iterator_const_unsigned_int_ptr_,
    mut last1: forward_iterator_const_unsigned_int_ptr_,
    mut first2: forward_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_53(
    mut first1: forward_iterator_const_int_ptr_,
    mut last1: forward_iterator_const_int_ptr_,
    mut first2: bidirectional_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_54(
    mut first1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut last1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut first2: forward_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_55(
    mut first1: forward_iterator_const_int_ptr_,
    mut last1: forward_iterator_const_int_ptr_,
    mut first2: bidirectional_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_56(
    mut first1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut last1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut first2: forward_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_57(
    mut first1: forward_iterator_const_int_ptr_,
    mut last1: forward_iterator_const_int_ptr_,
    mut first2: random_access_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_58(
    mut first1: random_access_iterator_const_unsigned_int_ptr_,
    mut last1: random_access_iterator_const_unsigned_int_ptr_,
    mut first2: forward_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_59(
    mut first1: forward_iterator_const_int_ptr_,
    mut last1: forward_iterator_const_int_ptr_,
    mut first2: random_access_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_60(
    mut first1: random_access_iterator_const_unsigned_int_ptr_,
    mut last1: random_access_iterator_const_unsigned_int_ptr_,
    mut first2: forward_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_61(
    mut first1: bidirectional_iterator_const_int_ptr_,
    mut last1: bidirectional_iterator_const_int_ptr_,
    mut first2: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_62(
    mut first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut first2: bidirectional_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_63(
    mut first1: bidirectional_iterator_const_int_ptr_,
    mut last1: bidirectional_iterator_const_int_ptr_,
    mut first2: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_64(
    mut first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut first2: bidirectional_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_65(
    mut first1: bidirectional_iterator_const_int_ptr_,
    mut last1: bidirectional_iterator_const_int_ptr_,
    mut first2: forward_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_66(
    mut first1: forward_iterator_const_unsigned_int_ptr_,
    mut last1: forward_iterator_const_unsigned_int_ptr_,
    mut first2: bidirectional_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_67(
    mut first1: bidirectional_iterator_const_int_ptr_,
    mut last1: bidirectional_iterator_const_int_ptr_,
    mut first2: forward_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_68(
    mut first1: forward_iterator_const_unsigned_int_ptr_,
    mut last1: forward_iterator_const_unsigned_int_ptr_,
    mut first2: bidirectional_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_69(
    mut first1: bidirectional_iterator_const_int_ptr_,
    mut last1: bidirectional_iterator_const_int_ptr_,
    mut first2: bidirectional_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_70(
    mut first1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut last1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut first2: bidirectional_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_71(
    mut first1: bidirectional_iterator_const_int_ptr_,
    mut last1: bidirectional_iterator_const_int_ptr_,
    mut first2: bidirectional_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_72(
    mut first1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut last1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut first2: bidirectional_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_73(
    mut first1: bidirectional_iterator_const_int_ptr_,
    mut last1: bidirectional_iterator_const_int_ptr_,
    mut first2: random_access_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_74(
    mut first1: random_access_iterator_const_unsigned_int_ptr_,
    mut last1: random_access_iterator_const_unsigned_int_ptr_,
    mut first2: bidirectional_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_75(
    mut first1: bidirectional_iterator_const_int_ptr_,
    mut last1: bidirectional_iterator_const_int_ptr_,
    mut first2: random_access_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_76(
    mut first1: random_access_iterator_const_unsigned_int_ptr_,
    mut last1: random_access_iterator_const_unsigned_int_ptr_,
    mut first2: bidirectional_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_77(
    mut first1: random_access_iterator_const_int_ptr_,
    mut last1: random_access_iterator_const_int_ptr_,
    mut first2: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_78(
    mut first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut first2: random_access_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_79(
    mut first1: random_access_iterator_const_int_ptr_,
    mut last1: random_access_iterator_const_int_ptr_,
    mut first2: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_80(
    mut first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut first2: random_access_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_81(
    mut first1: random_access_iterator_const_int_ptr_,
    mut last1: random_access_iterator_const_int_ptr_,
    mut first2: forward_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_82(
    mut first1: forward_iterator_const_unsigned_int_ptr_,
    mut last1: forward_iterator_const_unsigned_int_ptr_,
    mut first2: random_access_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_83(
    mut first1: random_access_iterator_const_int_ptr_,
    mut last1: random_access_iterator_const_int_ptr_,
    mut first2: forward_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_84(
    mut first1: forward_iterator_const_unsigned_int_ptr_,
    mut last1: forward_iterator_const_unsigned_int_ptr_,
    mut first2: random_access_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_85(
    mut first1: random_access_iterator_const_int_ptr_,
    mut last1: random_access_iterator_const_int_ptr_,
    mut first2: bidirectional_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_86(
    mut first1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut last1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut first2: random_access_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_87(
    mut first1: random_access_iterator_const_int_ptr_,
    mut last1: random_access_iterator_const_int_ptr_,
    mut first2: bidirectional_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_88(
    mut first1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut last1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut first2: random_access_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_89(
    mut first1: random_access_iterator_const_int_ptr_,
    mut last1: random_access_iterator_const_int_ptr_,
    mut first2: random_access_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_90(
    mut first1: random_access_iterator_const_unsigned_int_ptr_,
    mut last1: random_access_iterator_const_unsigned_int_ptr_,
    mut first2: random_access_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_91(
    mut first1: random_access_iterator_const_int_ptr_,
    mut last1: random_access_iterator_const_int_ptr_,
    mut first2: random_access_iterator_const_unsigned_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_92(
    mut first1: random_access_iterator_const_unsigned_int_ptr_,
    mut last1: random_access_iterator_const_unsigned_int_ptr_,
    mut first2: random_access_iterator_const_int_ptr_,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_93(
    mut first1: Ptr<()>,
    mut last1: Ptr<()>,
    mut first2: Ptr<()>,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_94(
    mut first1: Ptr<()>,
    mut last1: Ptr<()>,
    mut first2: Ptr<()>,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_95(
    mut first1: Ptr<()>,
    mut last1: Ptr<()>,
    mut first2: Ptr<()>,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_96(
    mut first1: Ptr<()>,
    mut last1: Ptr<()>,
    mut first2: Ptr<()>,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_97(
    mut first1: Ptr<()>,
    mut last1: Ptr<()>,
    mut first2: Ptr<()>,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_98(
    mut first1: Ptr<()>,
    mut last1: Ptr<()>,
    mut first2: Ptr<()>,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_99(
    mut first1: Ptr<()>,
    mut last1: Ptr<()>,
    mut first2: Ptr<()>,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_100(
    mut first1: Ptr<()>,
    mut last1: Ptr<()>,
    mut first2: Ptr<()>,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_101(
    mut first1: Ptr<()>,
    mut last1: Ptr<()>,
    mut first2: Ptr<()>,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_102(
    mut first1: Ptr<()>,
    mut last1: Ptr<()>,
    mut first2: Ptr<()>,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_103(
    mut first1: Ptr<()>,
    mut last1: Ptr<()>,
    mut first2: Ptr<()>,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_104(
    mut first1: Ptr<()>,
    mut last1: Ptr<()>,
    mut first2: Ptr<()>,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_105(
    mut first1: Ptr<()>,
    mut last1: Ptr<()>,
    mut first2: Ptr<()>,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_106(
    mut first1: Ptr<()>,
    mut last1: Ptr<()>,
    mut first2: Ptr<()>,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_107(
    mut first1: Ptr<()>,
    mut last1: Ptr<()>,
    mut first2: Ptr<()>,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_108(
    mut first1: Ptr<()>,
    mut last1: Ptr<()>,
    mut first2: Ptr<()>,
    mut init: (),
    op1: (),
    op2: (),
    mut x: (),
) {
    let op1: Value<()> = Rc::new(RefCell::new(op1));
    let op2: Value<()> = Rc::new(RefCell::new(op2));
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n         decltype(std::transform_reduce(first1, last1, first2, init, op1, op2))>"
    );;
    unimplemented!();
}
pub fn test_109() {
    let ia: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_(), 6_()])));
    let ua: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        2_(),
        4_(),
        6_(),
        8_(),
        10_(),
        12_(),
    ])));
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            });
        test_29(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            });
        test_30(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_31(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_32(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_29(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_30(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_31(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_32(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub fn test_110() {
    let ia: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_(), 6_()])));
    let ua: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        2_(),
        4_(),
        6_(),
        8_(),
        10_(),
        12_(),
    ])));
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            });
        test_33(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        test_34(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_35(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_36(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_33(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_34(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_35(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_36(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub fn test_111() {
    let ia: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_(), 6_()])));
    let ua: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        2_(),
        4_(),
        6_(),
        8_(),
        10_(),
        12_(),
    ])));
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            });
        test_37(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        test_38(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_39(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_40(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_37(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_38(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_39(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_40(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub fn test_112() {
    let ia: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_(), 6_()])));
    let ua: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        2_(),
        4_(),
        6_(),
        8_(),
        10_(),
        12_(),
    ])));
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            });
        test_41(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        test_42(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_43(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_44(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_41(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_42(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_43(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_44(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<()>)
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub fn test_113() {
    let ia: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_(), 6_()])));
    let ua: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        2_(),
        4_(),
        6_(),
        8_(),
        10_(),
        12_(),
    ])));
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        test_45(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            });
        test_46(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
        });
        test_47(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_48(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
        });
        test_45(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_46(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
        });
        test_47(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_48(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub fn test_114() {
    let ia: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_(), 6_()])));
    let ua: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        2_(),
        4_(),
        6_(),
        8_(),
        10_(),
        12_(),
    ])));
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        test_49(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        test_50(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
        });
        test_51(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_52(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
        });
        test_49(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_50(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
        });
        test_51(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_52(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub fn test_115() {
    let ia: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_(), 6_()])));
    let ua: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        2_(),
        4_(),
        6_(),
        8_(),
        10_(),
        12_(),
    ])));
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        test_53(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        test_54(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
        });
        test_55(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_56(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
        });
        test_53(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_54(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
        });
        test_55(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_56(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub fn test_116() {
    let ia: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_(), 6_()])));
    let ua: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        2_(),
        4_(),
        6_(),
        8_(),
        10_(),
        12_(),
    ])));
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        test_57(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        test_58(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
        });
        test_59(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_60(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
        });
        test_57(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_58(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
        });
        test_59(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_60(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub fn test_117() {
    let ia: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_(), 6_()])));
    let ua: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        2_(),
        4_(),
        6_(),
        8_(),
        10_(),
        12_(),
    ])));
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        test_61(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            });
        test_62(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_63(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_64(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_61(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_62(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_63(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_64(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub fn test_118() {
    let ia: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_(), 6_()])));
    let ua: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        2_(),
        4_(),
        6_(),
        8_(),
        10_(),
        12_(),
    ])));
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        test_65(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        test_66(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_67(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_68(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_65(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_66(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_67(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_68(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub fn test_119() {
    let ia: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_(), 6_()])));
    let ua: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        2_(),
        4_(),
        6_(),
        8_(),
        10_(),
        12_(),
    ])));
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        test_69(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        test_70(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_71(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_72(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_69(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_70(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_71(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_72(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub fn test_120() {
    let ia: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_(), 6_()])));
    let ua: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        2_(),
        4_(),
        6_(),
        8_(),
        10_(),
        12_(),
    ])));
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        test_73(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        test_74(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_75(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_76(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_73(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_74(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_75(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_76(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub fn test_121() {
    let ia: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_(), 6_()])));
    let ua: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        2_(),
        4_(),
        6_(),
        8_(),
        10_(),
        12_(),
    ])));
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        test_77(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            });
        test_78(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_79(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_80(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_77(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_78(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_79(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_80(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub fn test_122() {
    let ia: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_(), 6_()])));
    let ua: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        2_(),
        4_(),
        6_(),
        8_(),
        10_(),
        12_(),
    ])));
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        test_81(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        test_82(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_83(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_84(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_81(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_82(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_83(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_84(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub fn test_123() {
    let ia: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_(), 6_()])));
    let ua: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        2_(),
        4_(),
        6_(),
        8_(),
        10_(),
        12_(),
    ])));
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        test_85(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        test_86(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_87(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_88(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_85(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_86(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_87(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_88(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub fn test_124() {
    let ia: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_(), 6_()])));
    let ua: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        2_(),
        4_(),
        6_(),
        8_(),
        10_(),
        12_(),
    ])));
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        test_89(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        test_90(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_91(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        test_92(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_89(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        test_90(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_91(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<()>) });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        test_92(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) }),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub fn test_125() {
    let ia: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_(), 6_()])));
    let ua: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        2_(),
        4_(),
        6_(),
        8_(),
        10_(),
        12_(),
    ])));
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    ({
        let _first1: Ptr<()> = (ia.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ia.as_pointer() as Ptr<()>);
        test_93(
            _first1,
            _last1,
            (ua.as_pointer() as Ptr<()>),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ua.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ua.as_pointer() as Ptr<()>);
        test_94(
            _first1,
            _last1,
            (ia.as_pointer() as Ptr<()>),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ia.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ia.as_pointer() as Ptr<()>).offset((1_()) as isize);
        test_95(
            _first1,
            _last1,
            (ua.as_pointer() as Ptr<()>),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ua.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ua.as_pointer() as Ptr<()>).offset((1_()) as isize);
        test_96(
            _first1,
            _last1,
            (ia.as_pointer() as Ptr<()>),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ia.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ia.as_pointer() as Ptr<()>).offset((2_()) as isize);
        test_93(
            _first1,
            _last1,
            (ua.as_pointer() as Ptr<()>),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ua.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ua.as_pointer() as Ptr<()>).offset((2_()) as isize);
        test_94(
            _first1,
            _last1,
            (ia.as_pointer() as Ptr<()>),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ia.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ia.as_pointer() as Ptr<()>).offset((sa) as isize);
        test_95(
            _first1,
            _last1,
            (ua.as_pointer() as Ptr<()>),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ua.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ua.as_pointer() as Ptr<()>).offset((sa) as isize);
        test_96(
            _first1,
            _last1,
            (ia.as_pointer() as Ptr<()>),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub fn test_126() {
    let ia: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_(), 6_()])));
    let ua: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        2_(),
        4_(),
        6_(),
        8_(),
        10_(),
        12_(),
    ])));
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    ({
        let _first1: Ptr<()> = (ia.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ia.as_pointer() as Ptr<()>);
        test_97(
            _first1,
            _last1,
            (ua.as_pointer() as Ptr<()>),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ua.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ua.as_pointer() as Ptr<()>);
        test_98(
            _first1,
            _last1,
            (ia.as_pointer() as Ptr<()>),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ia.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ia.as_pointer() as Ptr<()>).offset((1_()) as isize);
        test_99(
            _first1,
            _last1,
            (ua.as_pointer() as Ptr<()>),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ua.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ua.as_pointer() as Ptr<()>).offset((1_()) as isize);
        test_100(
            _first1,
            _last1,
            (ia.as_pointer() as Ptr<()>),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ia.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ia.as_pointer() as Ptr<()>).offset((2_()) as isize);
        test_97(
            _first1,
            _last1,
            (ua.as_pointer() as Ptr<()>),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ua.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ua.as_pointer() as Ptr<()>).offset((2_()) as isize);
        test_98(
            _first1,
            _last1,
            (ia.as_pointer() as Ptr<()>),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ia.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ia.as_pointer() as Ptr<()>).offset((sa) as isize);
        test_99(
            _first1,
            _last1,
            (ua.as_pointer() as Ptr<()>),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ua.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ua.as_pointer() as Ptr<()>).offset((sa) as isize);
        test_100(
            _first1,
            _last1,
            (ia.as_pointer() as Ptr<()>),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub fn test_127() {
    let ia: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_(), 6_()])));
    let ua: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        2_(),
        4_(),
        6_(),
        8_(),
        10_(),
        12_(),
    ])));
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    ({
        let _first1: Ptr<()> = (ia.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ia.as_pointer() as Ptr<()>);
        test_101(
            _first1,
            _last1,
            (ua.as_pointer() as Ptr<()>),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ua.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ua.as_pointer() as Ptr<()>);
        test_102(
            _first1,
            _last1,
            (ia.as_pointer() as Ptr<()>),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ia.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ia.as_pointer() as Ptr<()>).offset((1_()) as isize);
        test_103(
            _first1,
            _last1,
            (ua.as_pointer() as Ptr<()>),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ua.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ua.as_pointer() as Ptr<()>).offset((1_()) as isize);
        test_104(
            _first1,
            _last1,
            (ia.as_pointer() as Ptr<()>),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ia.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ia.as_pointer() as Ptr<()>).offset((2_()) as isize);
        test_101(
            _first1,
            _last1,
            (ua.as_pointer() as Ptr<()>),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ua.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ua.as_pointer() as Ptr<()>).offset((2_()) as isize);
        test_102(
            _first1,
            _last1,
            (ia.as_pointer() as Ptr<()>),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ia.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ia.as_pointer() as Ptr<()>).offset((sa) as isize);
        test_103(
            _first1,
            _last1,
            (ua.as_pointer() as Ptr<()>),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ua.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ua.as_pointer() as Ptr<()>).offset((sa) as isize);
        test_104(
            _first1,
            _last1,
            (ia.as_pointer() as Ptr<()>),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub fn test_128() {
    let ia: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_(), 6_()])));
    let ua: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        2_(),
        4_(),
        6_(),
        8_(),
        10_(),
        12_(),
    ])));
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    ({
        let _first1: Ptr<()> = (ia.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ia.as_pointer() as Ptr<()>);
        test_105(
            _first1,
            _last1,
            (ua.as_pointer() as Ptr<()>),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ua.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ua.as_pointer() as Ptr<()>);
        test_106(
            _first1,
            _last1,
            (ia.as_pointer() as Ptr<()>),
            1_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ia.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ia.as_pointer() as Ptr<()>).offset((1_()) as isize);
        test_107(
            _first1,
            _last1,
            (ua.as_pointer() as Ptr<()>),
            0_(),
            unimplemented!(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ua.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ua.as_pointer() as Ptr<()>).offset((1_()) as isize);
        test_108(
            _first1,
            _last1,
            (ia.as_pointer() as Ptr<()>),
            2_(),
            unimplemented!(),
            unimplemented!(),
            4_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ia.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ia.as_pointer() as Ptr<()>).offset((2_()) as isize);
        test_105(
            _first1,
            _last1,
            (ua.as_pointer() as Ptr<()>),
            0_(),
            unimplemented!(),
            unimplemented!(),
            10_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ua.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ua.as_pointer() as Ptr<()>).offset((2_()) as isize);
        test_106(
            _first1,
            _last1,
            (ia.as_pointer() as Ptr<()>),
            3_(),
            unimplemented!(),
            unimplemented!(),
            54_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ia.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ia.as_pointer() as Ptr<()>).offset((sa) as isize);
        test_107(
            _first1,
            _last1,
            (ua.as_pointer() as Ptr<()>),
            4_(),
            unimplemented!(),
            unimplemented!(),
            2099520_(),
        )
    });
    ({
        let _first1: Ptr<()> = (ua.as_pointer() as Ptr<()>);
        let _last1: Ptr<()> = (ua.as_pointer() as Ptr<()>).offset((sa) as isize);
        test_108(
            _first1,
            _last1,
            (ia.as_pointer() as Ptr<()>),
            4_(),
            unimplemented!(),
            unimplemented!(),
            186_(),
        )
    });
}
pub fn test_return_type_129() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    const _: () = assert!(
        true,
        "std::is_same_v<Init,\n       decltype(std::transform_reduce(p, p, p, Init{{}}, std::plus<>(), std::multiplies<>()))>"
    );;
}
pub fn test_return_type_130() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    const _: () = assert!(
        true,
        "std::is_same_v<Init,\n       decltype(std::transform_reduce(p, p, p, Init{{}}, std::plus<>(), std::multiplies<>()))>"
    );;
}
pub fn test_return_type_131() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    const _: () = assert!(
        true,
        "std::is_same_v<Init,\n       decltype(std::transform_reduce(p, p, p, Init{{}}, std::plus<>(), std::multiplies<>()))>"
    );;
}
pub fn test_return_type_132() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    const _: () = assert!(
        true,
        "std::is_same_v<Init,\n       decltype(std::transform_reduce(p, p, p, Init{{}}, std::plus<>(), std::multiplies<>()))>"
    );;
}
pub fn test_return_type_133() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    const _: () = assert!(
        true,
        "std::is_same_v<Init,\n       decltype(std::transform_reduce(p, p, p, Init{{}}, std::plus<>(), std::multiplies<>()))>"
    );;
}
pub fn test_return_type_134() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    const _: () = assert!(
        true,
        "std::is_same_v<Init,\n       decltype(std::transform_reduce(p, p, p, Init{{}}, std::plus<>(), std::multiplies<>()))>"
    );;
}
pub fn test_return_type_135() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    const _: () = assert!(
        true,
        "std::is_same_v<Init,\n       decltype(std::transform_reduce(p, p, p, Init{{}}, std::plus<>(), std::multiplies<>()))>"
    );;
}
pub fn test_move_only_types_136() {
    let mut ia: [MoveOnly; 3] = [
        MoveOnly::new({ Some(1_()) }),
        MoveOnly::new({ Some(2_()) }),
        MoveOnly::new({ Some(3_()) }),
    ];
    let mut ib: [MoveOnly; 3] = [
        MoveOnly::new({ Some(1_()) }),
        MoveOnly::new({ Some(2_()) }),
        MoveOnly::new({ Some(3_()) }),
    ];
    unimplemented!();
}
pub fn test_137() -> () {
    ({ test_return_type_129() });
    ({ test_return_type_130() });
    ({ test_return_type_131() });
    ({ test_return_type_132() });
    ({ test_return_type_133() });
    ({ test_return_type_134() });
    ({ test_return_type_135() });
    ({ test_109() });
    ({ test_110() });
    ({ test_111() });
    ({ test_112() });
    ({ test_113() });
    ({ test_114() });
    ({ test_115() });
    ({ test_116() });
    ({ test_117() });
    ({ test_118() });
    ({ test_119() });
    ({ test_120() });
    ({ test_121() });
    ({ test_122() });
    ({ test_123() });
    ({ test_124() });
    ({ test_125() });
    ({ test_126() });
    ({ test_127() });
    ({ test_128() });
    ({ test_move_only_types_136() });
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
    ({ test_137() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
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
pub trait bidirectional_iterator_const_int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_const_int_ptr_>;
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_8(&self, mut _a0: ()) -> bidirectional_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_9(&self, mut _a0: ()) -> bidirectional_iterator_const_int_ptr_ {
        unimplemented!()
    }
}
impl bidirectional_iterator_const_int_ptr_Impl for Ptr<bidirectional_iterator_const_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait bidirectional_iterator_const_unsigned_int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_const_unsigned_int_ptr_>;
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_const_unsigned_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_8(&self, mut _a0: ()) -> bidirectional_iterator_const_unsigned_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_9(&self, mut _a0: ()) -> bidirectional_iterator_const_unsigned_int_ptr_ {
        unimplemented!()
    }
}
impl bidirectional_iterator_const_unsigned_int_ptr_Impl
    for Ptr<bidirectional_iterator_const_unsigned_int_ptr_>
{
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_const_unsigned_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait cpp17_input_iterator_const_int_ptr__const_int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>;
    fn operator_post_inc_6(
        &self,
        mut _a0: (),
    ) -> cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
        unimplemented!()
    }
}
impl cpp17_input_iterator_const_int_ptr__const_int_ptr_Impl
    for Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>
{
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_5(
        &self,
    ) -> Ptr<cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_>;
    fn operator_post_inc_6(
        &self,
        mut _a0: (),
    ) -> cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ {
        unimplemented!()
    }
}
impl cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_Impl
    for Ptr<cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_>
{
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_5(
        &self,
    ) -> Ptr<cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
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
pub trait forward_iterator_const_int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_int_ptr_>;
    fn operator_post_inc_7(&self, mut _a0: ()) -> forward_iterator_const_int_ptr_ {
        unimplemented!()
    }
}
impl forward_iterator_const_int_ptr_Impl for Ptr<forward_iterator_const_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait forward_iterator_const_unsigned_int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_unsigned_int_ptr_>;
    fn operator_post_inc_7(&self, mut _a0: ()) -> forward_iterator_const_unsigned_int_ptr_ {
        unimplemented!()
    }
}
impl forward_iterator_const_unsigned_int_ptr_Impl
    for Ptr<forward_iterator_const_unsigned_int_ptr_>
{
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_unsigned_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
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
pub trait random_access_iterator_const_int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_int_ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: ()) -> random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
}
impl random_access_iterator_const_int_ptr_Impl for Ptr<random_access_iterator_const_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait random_access_iterator_const_unsigned_int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_unsigned_int_ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_const_unsigned_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: ()) -> random_access_iterator_const_unsigned_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> random_access_iterator_const_unsigned_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<random_access_iterator_const_unsigned_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<random_access_iterator_const_unsigned_int_ptr_> {
        unimplemented!()
    }
}
impl random_access_iterator_const_unsigned_int_ptr_Impl
    for Ptr<random_access_iterator_const_unsigned_int_ptr_>
{
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_unsigned_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
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
pub fn __cpp2rust_init_globals() {}
