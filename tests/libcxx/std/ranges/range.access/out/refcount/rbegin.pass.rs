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
pub struct test_macros_detail_is_same_std_reverse_iterator_int_ptrptr___std_reverse_iterator_int_ptrptr__
{}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_reverse_iterator_int_ptrconst_ptr___std_reverse_iterator_int_ptrconst_ptr__
{}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_reverse_iterator_int__ptr_arr2arr___std_reverse_iterator_int__ptr_arr2arr__
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_reverse_iterator_const_int__ptr_arr2arr___std_reverse_iterator_const_int__ptr_arr2arr__
{}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_char_ptr__char_ptr_ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_short_ptr__short_ptr_ {}
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct forward_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl forward_iterator_int_ptr_ {}
pub fn base_6(i: Ptr<bidirectional_iterator_int_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct bidirectional_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
}
pub fn base_7(i: Ptr<bidirectional_iterator_const_char_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct bidirectional_iterator_const_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_const_char_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
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
impl bidirectional_iterator_const_int_ptr_ {}
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
pub type ThrowingIterator_int__ThrowingAction = ();
pub const ThrowingIterator_int__ThrowingAction_TAIncrement: ThrowingIterator_int__ThrowingAction =
    0;
pub const ThrowingIterator_int__ThrowingAction_TADecrement: ThrowingIterator_int__ThrowingAction =
    1;
pub const ThrowingIterator_int__ThrowingAction_TADereference: ThrowingIterator_int__ThrowingAction =
    2;
pub const ThrowingIterator_int__ThrowingAction_TAAssignment: ThrowingIterator_int__ThrowingAction =
    3;
pub const ThrowingIterator_int__ThrowingAction_TAComparison: ThrowingIterator_int__ThrowingAction =
    4;
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(32)]
pub struct ThrowingIterator_int_ {
    #[offset(0)]
    #[byte_size(8)]
    begin_: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    end_: Ptr<()>,
    #[offset(16)]
    #[byte_size(8)]
    current_: Ptr<()>,
    #[offset(24)]
    action_: ThrowingIterator_int__ThrowingAction,
    #[offset(28)]
    index_: (),
}
impl ThrowingIterator_int_ {
    pub fn copy_from(rhs: Ptr<ThrowingIterator_int_>) -> Self {
        Self {
            begin_: rhs.with(|__s| __s.begin_.clone()),
            end_: rhs.with(|__s| __s.end_.clone()),
            current_: rhs.with(|__s| __s.current_.clone()),
            action_: rhs.with(|__s| __s.action_),
            index_: rhs.with(|__s| __s.index_),
        }
    }
}
impl Clone for ThrowingIterator_int_ {
    fn clone(&self) -> Self {
        let __src: Value<ThrowingIterator_int_> = Rc::new(RefCell::new(ThrowingIterator_int_ {
            begin_: self.begin_.clone(),
            end_: self.end_.clone(),
            current_: self.current_.clone(),
            action_: self.action_.clone(),
            index_: self.index_.clone(),
        }));
        ThrowingIterator_int_::copy_from(__src.as_pointer())
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
impl std::cmp::PartialEq for forward_sized_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_8(
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
pub fn iter_swap_11(a: adl_Iterator, b: adl_Iterator) {
    let a: Value<adl_Iterator> = Rc::new(RefCell::new(a));
    let b: Value<adl_Iterator> = Rc::new(RefCell::new(b));
    ({
        let ___x: Ptr<Ptr<()>> = field_ptr!(a.as_pointer(), ptr_);
        let ___y: Ptr<Ptr<()>> = field_ptr!(b.as_pointer(), ptr_);
        swap_12(___x, ___y)
    });
    if !({ (*a.borrow()).iter_swaps_.clone() }).is_null() {
        { (*a.borrow()).iter_swaps_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_13(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) }).reinterpret_cast::<()>();
}
pub fn operator_eq_14(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_15(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
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
            operator_cmp_15(
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
            operator_eq_14(
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
thread_local!(
    pub static globalBuff_16: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..8).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
);
const _: () = assert!(true, "!std::is_invocable_v<RangeRBeginT, int (&&)[10]>");
const _: () = assert!(true, "std::is_invocable_v<RangeRBeginT, int (&)[10]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeRBeginT, int (&&)[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeRBeginT, int (&)[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCRBeginT, int (&&)[10]>");
const _: () = assert!(true, "std::is_invocable_v<RangeCRBeginT, int (&)[10]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCRBeginT, int (&&)[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCRBeginT, int (&)[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeRBeginT, Incomplete(&&)[]>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, const Incomplete(&&)[]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, Incomplete(&&)[]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, const Incomplete(&&)[]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, Incomplete(&&)[10]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, const Incomplete(&&)[10]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, Incomplete(&&)[10]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, const Incomplete(&&)[10]>"
);
const _: () = assert!(true, "!std::is_invocable_v<RangeRBeginT, Incomplete(&)[]>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, const Incomplete(&)[]>"
);
const _: () = assert!(true, "!std::is_invocable_v<RangeCRBeginT, Incomplete(&)[]>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, const Incomplete(&)[]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, Incomplete(&)[10]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, const Incomplete(&)[10]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, Incomplete(&)[10]>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, const Incomplete(&)[10]>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct RBeginMember {
    #[offset(0)]
    pub x: (),
}
const _: () = assert!(true, "std::is_invocable_v<RangeRBeginT, RBeginMember &>");
const _: () = assert!(true, "!std::is_invocable_v<RangeRBeginT, RBeginMember &&>");
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeRBeginT, RBeginMember const&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, RBeginMember const&&>"
);
const _: () = assert!(true, "std::is_invocable_v<RangeCRBeginT, RBeginMember &>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCRBeginT, RBeginMember &&>");
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeCRBeginT, RBeginMember const&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, RBeginMember const&&>"
);
pub fn testReturnTypes_17() -> () {
    {
        let x: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(
            (0..2)
                .map(|_| Ptr::<()>::null())
                .collect::<Box<[Ptr<()>]>>(),
        ));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let x: Value<Box<[Box<[()]>]>> = Rc::new(RefCell::new(
            (0..2)
                .map(|_| (0..2).map(|_| unimplemented!()).collect::<Box<[()]>>())
                .collect::<Box<[Box<[()]>]>>(),
        ));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let x: Value<Different_18> = Rc::new(RefCell::new(<Different_18>::default()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Different_18 {}
pub fn testArray_19() -> () {
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..2).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    unimplemented!();
    unimplemented!();
    let b: Value<Box<[Box<[()]>]>> = Rc::new(RefCell::new(
        (0..2)
            .map(|_| (0..2).map(|_| unimplemented!()).collect::<Box<[()]>>())
            .collect::<Box<[Box<[()]>]>>(),
    ));
    unimplemented!();
    unimplemented!();
    let c: Value<Box<[RBeginMember]>> = Rc::new(RefCell::new(
        (0..2)
            .map(|_| <RBeginMember>::default())
            .collect::<Box<[RBeginMember]>>(),
    ));
    unimplemented!();
    unimplemented!();
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RBeginMemberReturnsInt {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, RBeginMemberReturnsInt const&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RBeginMemberReturnsVoidPtr {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, RBeginMemberReturnsVoidPtr const&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct PtrConvertibleRBeginMember_iterator {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct PtrConvertibleRBeginMember {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, PtrConvertibleRBeginMember const&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct NonConstRBeginMember {
    #[offset(0)]
    pub x: (),
}
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeRBeginT,  NonConstRBeginMember &>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT,  NonConstRBeginMember const&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, NonConstRBeginMember &>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, NonConstRBeginMember const&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct EnabledBorrowingRBeginMember {}
thread_local!(
    pub static enable_borrowed_range_20: Value<()> = Rc::new(RefCell::new(true));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct RBeginMemberFunction {
    #[offset(0)]
    pub x: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct EmptyPtrRBeginMember_Empty {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct EmptyPtrRBeginMember {
    #[offset(0)]
    #[byte_size(1)]
    pub x: EmptyPtrRBeginMember_Empty,
}
pub fn testRBeginMember_21() -> () {
    let a: Value<RBeginMember> = Rc::new(RefCell::new(<RBeginMember>::default()));
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "!std::is_invocable_v<RangeRBeginT, RBeginMember&&>");;
    const _: () = assert!(true, "!std::is_invocable_v<RangeCRBeginT, RBeginMember&&>");;
    let b: Value<NonConstRBeginMember> = Rc::new(RefCell::new(<NonConstRBeginMember>::default()));
    unimplemented!();
    const _: () = assert!(
        true,
        "!std::is_invocable_v<RangeCRBeginT, NonConstRBeginMember&>"
    );;
    let c: Value<EnabledBorrowingRBeginMember> =
        Rc::new(RefCell::new(<EnabledBorrowingRBeginMember>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let d: Value<RBeginMemberFunction> = Rc::new(RefCell::new(<RBeginMemberFunction>::default()));
    unimplemented!();
    unimplemented!();
    let e: Value<EmptyPtrRBeginMember> = Rc::new(RefCell::new(<EmptyPtrRBeginMember>::default()));
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn rbegin_22(bf: Ptr<RBeginFunction>) -> Ptr<()> {
    return (field_ptr!(bf, x));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct RBeginFunction {
    #[offset(0)]
    pub x: (),
}
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeRBeginT,  RBeginFunction const&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT,  RBeginFunction &&>"
);
const _: () = assert!(true, "std::is_invocable_v<RangeRBeginT, RBeginFunction&>");
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeCRBeginT, RBeginFunction const&>"
);
const _: () = assert!(true, "std::is_invocable_v<RangeCRBeginT, RBeginFunction &>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RBeginFunctionReturnsInt {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, RBeginFunctionReturnsInt const&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RBeginFunctionReturnsVoidPtr {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, RBeginFunctionReturnsVoidPtr const&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RBeginFunctionReturnsEmpty_Empty {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RBeginFunctionReturnsEmpty {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, RBeginFunctionReturnsEmpty const&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RBeginFunctionReturnsPtrConvertible_iterator {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RBeginFunctionReturnsPtrConvertible {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, RBeginFunctionReturnsPtrConvertible const&>"
);
pub fn rbegin_23(mut _a0: RBeginFunctionByValue) -> Ptr<()> {
    return (globalBuff_16.with(|v| v.as_pointer()) as Ptr<()>).offset((1_()) as isize);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RBeginFunctionByValue {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, RBeginFunctionByValue>"
);
pub fn rbegin_24(mut _a0: RBeginFunctionEnabledBorrowing) -> Ptr<()> {
    return (globalBuff_16.with(|v| v.as_pointer()) as Ptr<()>).offset((2_()) as isize);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RBeginFunctionEnabledBorrowing {}
thread_local!(
    pub static enable_borrowed_range_25: Value<()> = Rc::new(RefCell::new(true));
);
pub fn rbegin_26(
    bf: Ptr<RBeginFunctionReturnsEmptyPtr>,
) -> Ptr<RBeginFunctionReturnsEmptyPtr_Empty> {
    return (field_ptr!(bf, x));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RBeginFunctionReturnsEmptyPtr_Empty {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RBeginFunctionReturnsEmptyPtr {
    #[offset(0)]
    #[byte_size(1)]
    pub x: RBeginFunctionReturnsEmptyPtr_Empty,
}
pub fn rbegin_27(bf: Ptr<RBeginFunctionWithDataMember>) -> Ptr<()> {
    return (field_ptr!(bf, x));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct RBeginFunctionWithDataMember {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub rbegin: (),
}
pub fn rbegin_28(bf: Ptr<RBeginFunctionWithPrivateBeginMember>) -> Ptr<()> {
    return (field_ptr!(bf, y));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct RBeginFunctionWithPrivateBeginMember {
    #[offset(0)]
    pub y: (),
}
pub fn testRBeginFunction_29() -> () {
    let a: Value<RBeginFunction> = Rc::new(RefCell::new(<RBeginFunction>::default()));
    let aa: Value<RBeginFunction> = Rc::new(RefCell::new(<RBeginFunction>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let b: Value<RBeginFunctionByValue> = Rc::new(RefCell::new(<RBeginFunctionByValue>::default()));
    let bb: Value<RBeginFunctionByValue> =
        Rc::new(RefCell::new(<RBeginFunctionByValue>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let c: Value<RBeginFunctionEnabledBorrowing> =
        Rc::new(RefCell::new(<RBeginFunctionEnabledBorrowing>::default()));
    let cc: Value<RBeginFunctionEnabledBorrowing> =
        Rc::new(RefCell::new(<RBeginFunctionEnabledBorrowing>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let d: Value<RBeginFunctionReturnsEmptyPtr> =
        Rc::new(RefCell::new(<RBeginFunctionReturnsEmptyPtr>::default()));
    let dd: Value<RBeginFunctionReturnsEmptyPtr> =
        Rc::new(RefCell::new(<RBeginFunctionReturnsEmptyPtr>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let e: Value<RBeginFunctionWithDataMember> =
        Rc::new(RefCell::new(<RBeginFunctionWithDataMember>::default()));
    let ee: Value<RBeginFunctionWithDataMember> =
        Rc::new(RefCell::new(<RBeginFunctionWithDataMember>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let f: Value<RBeginFunctionWithPrivateBeginMember> = Rc::new(RefCell::new(
        <RBeginFunctionWithPrivateBeginMember>::default(),
    ));
    let ff: Value<RBeginFunctionWithPrivateBeginMember> = Rc::new(RefCell::new(
        <RBeginFunctionWithPrivateBeginMember>::default(),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct MemberBeginEnd {
    #[offset(0)]
    pub b: (),
    #[offset(4)]
    pub e: (),
    #[offset(8)]
    pub cb: (),
    #[offset(9)]
    pub ce: (),
}
const _: () = assert!(true, "std::is_invocable_v<RangeRBeginT, MemberBeginEnd&>");
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeRBeginT, MemberBeginEnd const&>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeCRBeginT, MemberBeginEnd const&>"
);
pub fn begin_30(v: Ptr<FunctionBeginEnd>) -> bidirectional_iterator_int_ptr_ {
    return bidirectional_iterator_int_ptr_::new_1({ (field_ptr!(v, b)) });
}
pub fn end_31(v: Ptr<FunctionBeginEnd>) -> bidirectional_iterator_int_ptr_ {
    return bidirectional_iterator_int_ptr_::new_1({ (field_ptr!(v, e)) });
}
pub fn begin_32(v: Ptr<FunctionBeginEnd>) -> bidirectional_iterator_const_char_ptr_ {
    return bidirectional_iterator_const_char_ptr_::new_1({ (field_ptr!(v, cb)) });
}
pub fn end_33(v: Ptr<FunctionBeginEnd>) -> bidirectional_iterator_const_char_ptr_ {
    return bidirectional_iterator_const_char_ptr_::new_1({ (field_ptr!(v, ce)) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct FunctionBeginEnd {
    #[offset(0)]
    pub b: (),
    #[offset(4)]
    pub e: (),
    #[offset(8)]
    pub cb: (),
    #[offset(9)]
    pub ce: (),
}
const _: () = assert!(true, "std::is_invocable_v<RangeRBeginT, FunctionBeginEnd&>");
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeRBeginT, FunctionBeginEnd const&>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeCRBeginT, FunctionBeginEnd const&>"
);
pub fn end_34(v: Ptr<MemberBeginFunctionEnd>) -> bidirectional_iterator_int_ptr_ {
    return bidirectional_iterator_int_ptr_::new_1({ (field_ptr!(v, e)) });
}
pub fn end_35(v: Ptr<MemberBeginFunctionEnd>) -> bidirectional_iterator_const_char_ptr_ {
    return bidirectional_iterator_const_char_ptr_::new_1({ (field_ptr!(v, ce)) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct MemberBeginFunctionEnd {
    #[offset(0)]
    pub b: (),
    #[offset(4)]
    pub e: (),
    #[offset(8)]
    pub cb: (),
    #[offset(9)]
    pub ce: (),
}
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeRBeginT, MemberBeginFunctionEnd&>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeRBeginT, MemberBeginFunctionEnd const&>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeCRBeginT, MemberBeginFunctionEnd const&>"
);
pub fn begin_36(v: Ptr<FunctionBeginMemberEnd>) -> bidirectional_iterator_int_ptr_ {
    return bidirectional_iterator_int_ptr_::new_1({ (field_ptr!(v, b)) });
}
pub fn begin_37(v: Ptr<FunctionBeginMemberEnd>) -> bidirectional_iterator_const_char_ptr_ {
    return bidirectional_iterator_const_char_ptr_::new_1({ (field_ptr!(v, cb)) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct FunctionBeginMemberEnd {
    #[offset(0)]
    pub b: (),
    #[offset(4)]
    pub e: (),
    #[offset(8)]
    pub cb: (),
    #[offset(9)]
    pub ce: (),
}
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeRBeginT, FunctionBeginMemberEnd&>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeRBeginT, FunctionBeginMemberEnd const&>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeCRBeginT, FunctionBeginMemberEnd const&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MemberBeginEndDifferentTypes {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, MemberBeginEndDifferentTypes&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, MemberBeginEndDifferentTypes&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct FunctionBeginEndDifferentTypes {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, FunctionBeginEndDifferentTypes&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, FunctionBeginEndDifferentTypes&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MemberBeginEndForwardIterators {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, MemberBeginEndForwardIterators&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, MemberBeginEndForwardIterators&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct FunctionBeginEndForwardIterators {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, FunctionBeginEndForwardIterators&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, FunctionBeginEndForwardIterators&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MemberBeginOnly {}
const _: () = assert!(true, "!std::is_invocable_v<RangeRBeginT, MemberBeginOnly&>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, MemberBeginOnly&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct FunctionBeginOnly {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, FunctionBeginOnly&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, FunctionBeginOnly&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MemberEndOnly {}
const _: () = assert!(true, "!std::is_invocable_v<RangeRBeginT, MemberEndOnly&>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCRBeginT, MemberEndOnly&>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct FunctionEndOnly {}
const _: () = assert!(true, "!std::is_invocable_v<RangeRBeginT, FunctionEndOnly&>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, FunctionEndOnly&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MemberBeginAndRBegin {}
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeRBeginT, MemberBeginAndRBegin&>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<RangeCRBeginT, MemberBeginAndRBegin&>"
);
const _: () = assert!(
    true,
    "std::same_as<std::invoke_result_t<RangeRBeginT, MemberBeginAndRBegin&>, int*>"
);
const _: () = assert!(
    true,
    "std::same_as<std::invoke_result_t<RangeCRBeginT, MemberBeginAndRBegin&>, int*>"
);
pub fn testBeginEnd_38() -> () {
    let a: Value<MemberBeginEnd> = Rc::new(RefCell::new(<MemberBeginEnd>::default()));
    let aa: Value<MemberBeginEnd> = Rc::new(RefCell::new(<MemberBeginEnd>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let b: Value<FunctionBeginEnd> = Rc::new(RefCell::new(<FunctionBeginEnd>::default()));
    let bb: Value<FunctionBeginEnd> = Rc::new(RefCell::new(<FunctionBeginEnd>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let c: Value<MemberBeginFunctionEnd> =
        Rc::new(RefCell::new(<MemberBeginFunctionEnd>::default()));
    let cc: Value<MemberBeginFunctionEnd> =
        Rc::new(RefCell::new(<MemberBeginFunctionEnd>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let d: Value<FunctionBeginMemberEnd> =
        Rc::new(RefCell::new(<FunctionBeginMemberEnd>::default()));
    let dd: Value<FunctionBeginMemberEnd> =
        Rc::new(RefCell::new(<FunctionBeginMemberEnd>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
const _: () = assert!(true, "");
const _: () = assert!(true, "");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoThrowMemberRBegin {}
thread_local!(
    pub static ntmb_39: Value<NoThrowMemberRBegin> =
        Rc::new(RefCell::new(<NoThrowMemberRBegin>::default()));
);
const _: () = assert!(true, "noexcept(std::ranges::rbegin(ntmb))");
const _: () = assert!(true, "noexcept(std::ranges::crbegin(ntmb))");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoThrowADLRBegin {}
thread_local!(
    pub static ntab_40: Value<NoThrowADLRBegin> =
        Rc::new(RefCell::new(<NoThrowADLRBegin>::default()));
);
const _: () = assert!(true, "noexcept(std::ranges::rbegin(ntab))");
const _: () = assert!(true, "noexcept(std::ranges::crbegin(ntab))");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoThrowMemberRBeginReturnsRef {}
thread_local!(
    pub static ntmbrr_41: Value<NoThrowMemberRBeginReturnsRef> =
        Rc::new(RefCell::new(<NoThrowMemberRBeginReturnsRef>::default()));
);
const _: () = assert!(true, "!noexcept(std::ranges::rbegin(ntmbrr))");
const _: () = assert!(true, "!noexcept(std::ranges::crbegin(ntmbrr))");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RBeginReturnsArrayRef {}
thread_local!(
    pub static brar_42: Value<RBeginReturnsArrayRef> =
        Rc::new(RefCell::new(<RBeginReturnsArrayRef>::default()));
);
const _: () = assert!(true, "noexcept(std::ranges::rbegin(brar))");
const _: () = assert!(true, "noexcept(std::ranges::crbegin(brar))");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoThrowBeginThrowingEnd {}
thread_local!(
    pub static ntbte_43: Value<NoThrowBeginThrowingEnd> =
        Rc::new(RefCell::new(<NoThrowBeginThrowingEnd>::default()));
);
const _: () = assert!(true, "!noexcept(std::ranges::rbegin(ntbte))");
const _: () = assert!(true, "!noexcept(std::ranges::crbegin(ntbte))");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoThrowEndThrowingBegin {}
thread_local!(
    pub static ntetb_44: Value<NoThrowEndThrowingBegin> =
        Rc::new(RefCell::new(<NoThrowEndThrowingBegin>::default()));
);
const _: () = assert!(true, "noexcept(std::ranges::rbegin(ntetb))");
const _: () = assert!(true, "noexcept(std::ranges::crbegin(ntetb))");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, Holder<Incomplete>*>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeRBeginT, Holder<Incomplete>*&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, Holder<Incomplete>*>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCRBeginT, Holder<Incomplete>*&>"
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
    const _: () = assert!(true, "testReturnTypes()");;
    ({ testArray_19() });
    const _: () = assert!(true, "testArray()");;
    ({ testRBeginMember_21() });
    const _: () = assert!(true, "testRBeginMember()");;
    ({ testRBeginFunction_29() });
    const _: () = assert!(true, "testRBeginFunction()");;
    ({ testBeginEnd_38() });
    const _: () = assert!(true, "testBeginEnd()");;
    return 0_();
}
pub trait Different_18Impl {
    fn rbegin_1(&self) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn rbegin_2(&self) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
}
impl Different_18Impl for Ptr<Different_18> {}
pub trait EmptyPtrRBeginMemberImpl {
    fn rbegin(&self) -> Ptr<EmptyPtrRBeginMember_Empty>;
}
impl EmptyPtrRBeginMemberImpl for Ptr<EmptyPtrRBeginMember> {
    fn rbegin(&self) -> Ptr<EmptyPtrRBeginMember_Empty> {
        return (field_ptr!((*self), x));
    }
}
pub trait EnabledBorrowingRBeginMemberImpl {
    fn rbegin(&self) -> Ptr<()>;
}
impl EnabledBorrowingRBeginMemberImpl for Ptr<EnabledBorrowingRBeginMember> {
    fn rbegin(&self) -> Ptr<()> {
        return (globalBuff_16.with(|v| v.as_pointer()) as Ptr<()>);
    }
}
pub trait FunctionBeginMemberEndImpl {
    fn end_1(&self) -> bidirectional_iterator_int_ptr_;
    fn end_2(&self) -> bidirectional_iterator_const_char_ptr_;
}
impl FunctionBeginMemberEndImpl for Ptr<FunctionBeginMemberEnd> {
    fn end_1(&self) -> bidirectional_iterator_int_ptr_ {
        return bidirectional_iterator_int_ptr_::new_1({ (field_ptr!((*self), e)) });
    }
    fn end_2(&self) -> bidirectional_iterator_const_char_ptr_ {
        return bidirectional_iterator_const_char_ptr_::new_1({ (field_ptr!((*self), ce)) });
    }
}
pub trait MemberBeginAndRBeginImpl {
    fn begin(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn rbegin(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn rend(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl MemberBeginAndRBeginImpl for Ptr<MemberBeginAndRBegin> {}
pub trait MemberBeginEndImpl {
    fn begin_1(&self) -> bidirectional_iterator_int_ptr_;
    fn end_2(&self) -> bidirectional_iterator_int_ptr_;
    fn begin_3(&self) -> bidirectional_iterator_const_char_ptr_;
    fn end_4(&self) -> bidirectional_iterator_const_char_ptr_;
}
impl MemberBeginEndImpl for Ptr<MemberBeginEnd> {
    fn begin_1(&self) -> bidirectional_iterator_int_ptr_ {
        return bidirectional_iterator_int_ptr_::new_1({ (field_ptr!((*self), b)) });
    }
    fn end_2(&self) -> bidirectional_iterator_int_ptr_ {
        return bidirectional_iterator_int_ptr_::new_1({ (field_ptr!((*self), e)) });
    }
    fn begin_3(&self) -> bidirectional_iterator_const_char_ptr_ {
        return bidirectional_iterator_const_char_ptr_::new_1({ (field_ptr!((*self), cb)) });
    }
    fn end_4(&self) -> bidirectional_iterator_const_char_ptr_ {
        return bidirectional_iterator_const_char_ptr_::new_1({ (field_ptr!((*self), ce)) });
    }
}
pub trait MemberBeginEndDifferentTypesImpl {
    fn begin(&self) -> bidirectional_iterator_int_ptr_ {
        unimplemented!()
    }
    fn end(&self) -> bidirectional_iterator_const_int_ptr_ {
        unimplemented!()
    }
}
impl MemberBeginEndDifferentTypesImpl for Ptr<MemberBeginEndDifferentTypes> {}
pub trait MemberBeginEndForwardIteratorsImpl {
    fn begin(&self) -> forward_iterator_int_ptr_ {
        unimplemented!()
    }
    fn end(&self) -> forward_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl MemberBeginEndForwardIteratorsImpl for Ptr<MemberBeginEndForwardIterators> {}
pub trait MemberBeginFunctionEndImpl {
    fn begin_1(&self) -> bidirectional_iterator_int_ptr_;
    fn begin_2(&self) -> bidirectional_iterator_const_char_ptr_;
}
impl MemberBeginFunctionEndImpl for Ptr<MemberBeginFunctionEnd> {
    fn begin_1(&self) -> bidirectional_iterator_int_ptr_ {
        return bidirectional_iterator_int_ptr_::new_1({ (field_ptr!((*self), b)) });
    }
    fn begin_2(&self) -> bidirectional_iterator_const_char_ptr_ {
        return bidirectional_iterator_const_char_ptr_::new_1({ (field_ptr!((*self), cb)) });
    }
}
pub trait MemberBeginOnlyImpl {
    fn begin(&self) -> bidirectional_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl MemberBeginOnlyImpl for Ptr<MemberBeginOnly> {}
pub trait MemberEndOnlyImpl {
    fn end(&self) -> bidirectional_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl MemberEndOnlyImpl for Ptr<MemberEndOnly> {}
pub trait NoThrowBeginThrowingEndImpl {
    fn begin(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl NoThrowBeginThrowingEndImpl for Ptr<NoThrowBeginThrowingEnd> {}
pub trait NoThrowEndThrowingBeginImpl {
    fn begin(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl NoThrowEndThrowingBeginImpl for Ptr<NoThrowEndThrowingBegin> {}
pub trait NoThrowMemberRBeginImpl {
    fn rbegin(&self) -> ThrowingIterator_int_ {
        unimplemented!()
    }
}
impl NoThrowMemberRBeginImpl for Ptr<NoThrowMemberRBegin> {}
pub trait NoThrowMemberRBeginReturnsRefImpl {
    fn rbegin(&self) -> Ptr<ThrowingIterator_int_> {
        unimplemented!()
    }
}
impl NoThrowMemberRBeginReturnsRefImpl for Ptr<NoThrowMemberRBeginReturnsRef> {}
pub trait NonConstRBeginMemberImpl {
    fn rbegin(&self) -> Ptr<()>;
}
impl NonConstRBeginMemberImpl for Ptr<NonConstRBeginMember> {
    fn rbegin(&self) -> Ptr<()> {
        return (field_ptr!((*self), x));
    }
}
pub trait PtrConvertibleRBeginMemberImpl {
    fn rbegin(&self) -> PtrConvertibleRBeginMember_iterator {
        unimplemented!()
    }
}
impl PtrConvertibleRBeginMemberImpl for Ptr<PtrConvertibleRBeginMember> {}
pub trait PtrConvertibleRBeginMember_iteratorImpl {
    fn to_pmut(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl PtrConvertibleRBeginMember_iteratorImpl for Ptr<PtrConvertibleRBeginMember_iterator> {}
pub trait RBeginFunctionReturnsPtrConvertible_iteratorImpl {
    fn to_pmut(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl RBeginFunctionReturnsPtrConvertible_iteratorImpl
    for Ptr<RBeginFunctionReturnsPtrConvertible_iterator>
{
}
pub trait RBeginFunctionWithPrivateBeginMemberImpl {
    fn rbegin(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl RBeginFunctionWithPrivateBeginMemberImpl for Ptr<RBeginFunctionWithPrivateBeginMember> {}
pub trait RBeginMemberImpl {
    fn rbegin(&self) -> Ptr<()>;
}
impl RBeginMemberImpl for Ptr<RBeginMember> {
    fn rbegin(&self) -> Ptr<()> {
        return (field_ptr!((*self), x));
    }
}
pub trait RBeginMemberFunctionImpl {
    fn rbegin(&self) -> Ptr<()>;
}
impl RBeginMemberFunctionImpl for Ptr<RBeginMemberFunction> {
    fn rbegin(&self) -> Ptr<()> {
        return (field_ptr!((*self), x));
    }
}
pub trait RBeginMemberReturnsIntImpl {
    fn rbegin(&self) -> () {
        unimplemented!()
    }
}
impl RBeginMemberReturnsIntImpl for Ptr<RBeginMemberReturnsInt> {}
pub trait RBeginMemberReturnsVoidPtrImpl {
    fn rbegin(&self) -> () {
        unimplemented!()
    }
}
impl RBeginMemberReturnsVoidPtrImpl for Ptr<RBeginMemberReturnsVoidPtr> {}
pub trait RBeginReturnsArrayRefImpl {
    fn rbegin(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl RBeginReturnsArrayRefImpl for Ptr<RBeginReturnsArrayRef> {}
pub trait ThrowingIterator_int_Impl {
    fn copy_assign(&self, rhs: Ptr<ThrowingIterator_int_>) -> Ptr<ThrowingIterator_int_> {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<ThrowingIterator_int_> {
        unimplemented!()
    }
    fn operator_post_inc_7(&self, mut _a0: ()) -> ThrowingIterator_int_ {
        unimplemented!()
    }
    fn operator_dec_8(&self) -> Ptr<ThrowingIterator_int_> {
        unimplemented!()
    }
    fn operator_post_dec_9(&self, mut _a0: ()) -> ThrowingIterator_int_ {
        unimplemented!()
    }
}
impl ThrowingIterator_int_Impl for Ptr<ThrowingIterator_int_> {}
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
pub trait bidirectional_iterator_const_char_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_const_char_ptr_> {
        unimplemented!()
    }
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_const_char_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_8(&self, mut _a0: ()) -> bidirectional_iterator_const_char_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_9(&self, mut _a0: ()) -> bidirectional_iterator_const_char_ptr_ {
        unimplemented!()
    }
}
impl bidirectional_iterator_const_char_ptr_Impl for Ptr<bidirectional_iterator_const_char_ptr_> {}
pub trait bidirectional_iterator_const_int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_const_int_ptr_> {
        unimplemented!()
    }
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
impl bidirectional_iterator_const_int_ptr_Impl for Ptr<bidirectional_iterator_const_int_ptr_> {}
pub trait bidirectional_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_8(&self, mut _a0: ()) -> bidirectional_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_9(&self, mut _a0: ()) -> bidirectional_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl bidirectional_iterator_int_ptr_Impl for Ptr<bidirectional_iterator_int_ptr_> {}
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
pub trait forward_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<forward_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_7(&self, mut _a0: ()) -> forward_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl forward_iterator_int_ptr_Impl for Ptr<forward_iterator_int_ptr_> {}
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
pub fn __cpp2rust_init_globals() {
    let _ = globalBuff_16.with(|_| ());
    let _ = enable_borrowed_range_20.with(|_| ());
    let _ = enable_borrowed_range_25.with(|_| ());
    let _ = ntmb_39.with(|_| ());
    let _ = ntab_40.with(|_| ());
    let _ = ntmbrr_41.with(|_| ());
    let _ = brar_42.with(|_| ());
    let _ = ntbte_43.with(|_| ());
    let _ = ntetb_44.with(|_| ());
}
