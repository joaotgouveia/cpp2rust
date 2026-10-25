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
pub struct test_macros_detail_is_same_int_ptrptr__int_ptrptr_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int_ptrconst_ptr__int_ptrconst_ptr_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int__ptr_arr2arr__int__ptr_arr2arr_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int__ptr_arr2arr__const_int__ptr_arr2arr_ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_sentinel_wrapper_char_ptr___sentinel_wrapper_char_ptr__ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_sentinel_wrapper_short_ptr___sentinel_wrapper_short_ptr__ {}
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_char_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_short_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_short_ptr_ {}
pub fn operator_eq_6(
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
            operator_eq_6(
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
pub fn operator_add_7(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_8(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_7(_i, n)
    });
}
pub fn iter_swap_9(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_10(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_11(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_12(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
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
            operator_cmp_12(
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
            operator_eq_11(
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
    pub static globalBuff_13: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..8).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
);
const _: () = assert!(true, "!std::is_invocable_v<RangeEndT, int (&&)[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeEndT, int (&)[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeEndT, int (&&)[10]>");
const _: () = assert!(true, "std::is_invocable_v<RangeEndT, int (&)[10]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCEndT, int (&&)[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCEndT, int (&)[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCEndT, int (&&)[10]>");
const _: () = assert!(true, "std::is_invocable_v<RangeCEndT, int (&)[10]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeEndT, Incomplete(&&)[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeEndT, Incomplete(&&)[42]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCEndT, Incomplete(&&)[]>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCEndT, Incomplete(&&)[42]>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct EndMember {
    #[offset(0)]
    pub x: (),
}
const _: () = assert!(true, "std::is_invocable_v<RangeEndT, EndMember &>");
const _: () = assert!(true, "!std::is_invocable_v<RangeEndT, EndMember &&>");
const _: () = assert!(true, "std::is_invocable_v<RangeEndT, EndMember const&>");
const _: () = assert!(true, "!std::is_invocable_v<RangeEndT, EndMember const&&>");
const _: () = assert!(true, "std::is_invocable_v<RangeCEndT, EndMember &>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCEndT, EndMember &&>");
const _: () = assert!(true, "std::is_invocable_v<RangeCEndT, EndMember const&>");
const _: () = assert!(true, "!std::is_invocable_v<RangeCEndT, EndMember const&&>");
pub fn testReturnTypes_14() -> () {
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
        let x: Value<Different_15> = Rc::new(RefCell::new(<Different_15>::default()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Different_15 {}
pub fn testArray_16() -> () {
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
    let c: Value<Box<[EndMember]>> = Rc::new(RefCell::new(
        (0..2)
            .map(|_| <EndMember>::default())
            .collect::<Box<[EndMember]>>(),
    ));
    unimplemented!();
    unimplemented!();
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct EndMemberReturnsInt {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT, EndMemberReturnsInt const&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct EndMemberReturnsVoidPtr {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT, EndMemberReturnsVoidPtr const&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct PtrConvertible {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct PtrConvertibleEndMember {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT, PtrConvertibleEndMember const&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoBeginMember {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT, NoBeginMember const&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct NonConstEndMember {
    #[offset(0)]
    pub x: (),
}
const _: () = assert!(true, "std::is_invocable_v<RangeEndT,  NonConstEndMember &>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT,  NonConstEndMember const&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCEndT, NonConstEndMember &>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCEndT, NonConstEndMember const&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct EnabledBorrowingEndMember {}
thread_local!(
    pub static enable_borrowed_range_17: Value<()> = Rc::new(RefCell::new(true));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct EndMemberFunction {
    #[offset(0)]
    pub x: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Empty {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct EmptyEndMember {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT, EmptyEndMember const&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct EmptyPtrEndMember {
    #[offset(0)]
    #[byte_size(1)]
    pub x: Empty,
}
pub fn testEndMember_18() -> () {
    let a: Value<EndMember> = Rc::new(RefCell::new(<EndMember>::default()));
    unimplemented!();
    unimplemented!();
    let b: Value<NonConstEndMember> = Rc::new(RefCell::new(<NonConstEndMember>::default()));
    unimplemented!();
    const _: () = assert!(true, "!std::is_invocable_v<RangeCEndT, decltype((b))>");;
    let mut c: EnabledBorrowingEndMember = <EnabledBorrowingEndMember>::default();
    unimplemented!();
    unimplemented!();
    let d: Value<EndMemberFunction> = Rc::new(RefCell::new(<EndMemberFunction>::default()));
    unimplemented!();
    unimplemented!();
    let e: Value<EmptyPtrEndMember> = Rc::new(RefCell::new(<EmptyPtrEndMember>::default()));
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn begin_19(_a0: Ptr<EndFunction>) -> Ptr<()> {
    return Ptr::<()>::null();
}
pub fn end_20(bf: Ptr<EndFunction>) -> Ptr<()> {
    return (field_ptr!(bf, x));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct EndFunction {
    #[offset(0)]
    pub x: (),
}
const _: () = assert!(true, "std::is_invocable_v<RangeEndT, EndFunction const&>");
const _: () = assert!(true, "!std::is_invocable_v<RangeEndT, EndFunction &&>");
const _: () = assert!(true, "std::is_invocable_v<RangeEndT,  EndFunction const&>");
const _: () = assert!(true, "!std::is_invocable_v<RangeEndT,  EndFunction &&>");
const _: () = assert!(true, "std::is_invocable_v<RangeEndT, EndFunction&>");
const _: () = assert!(true, "std::is_invocable_v<RangeCEndT, EndFunction const&>");
const _: () = assert!(true, "std::is_invocable_v<RangeCEndT, EndFunction &>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct EndFunctionReturnsInt {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT, EndFunctionReturnsInt const&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct EndFunctionReturnsVoidPtr {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT, EndFunctionReturnsVoidPtr const&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct EndFunctionReturnsEmpty {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT, EndFunctionReturnsEmpty const&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct EndFunctionReturnsPtrConvertible {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT, EndFunctionReturnsPtrConvertible const&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoBeginFunction {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT, NoBeginFunction const&>"
);
pub fn begin_21(mut _a0: EndFunctionByValue) -> Ptr<()> {
    return Ptr::<()>::null();
}
pub fn end_22(mut _a0: EndFunctionByValue) -> Ptr<()> {
    return ((globalBuff_13.with(|v| v.as_pointer()) as Ptr<()>).offset(1_()));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct EndFunctionByValue {}
const _: () = assert!(true, "!std::is_invocable_v<RangeCEndT, EndFunctionByValue>");
pub fn begin_23(mut _a0: EndFunctionEnabledBorrowing) -> Ptr<()> {
    return Ptr::<()>::null();
}
pub fn end_24(mut _a0: EndFunctionEnabledBorrowing) -> Ptr<()> {
    return ((globalBuff_13.with(|v| v.as_pointer()) as Ptr<()>).offset(2_()));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct EndFunctionEnabledBorrowing {}
thread_local!(
    pub static enable_borrowed_range_25: Value<()> = Rc::new(RefCell::new(true));
);
pub fn begin_26(_a0: Ptr<EndFunctionReturnsEmptyPtr>) -> Ptr<Empty> {
    return Ptr::<Empty>::null();
}
pub fn end_27(bf: Ptr<EndFunctionReturnsEmptyPtr>) -> Ptr<Empty> {
    return (field_ptr!(bf, x));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct EndFunctionReturnsEmptyPtr {
    #[offset(0)]
    #[byte_size(1)]
    pub x: Empty,
}
pub fn begin_28(_a0: Ptr<EndFunctionWithDataMember>) -> Ptr<()> {
    return Ptr::<()>::null();
}
pub fn end_29(bf: Ptr<EndFunctionWithDataMember>) -> Ptr<()> {
    return (field_ptr!(bf, x));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct EndFunctionWithDataMember {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub end: (),
}
pub fn begin_30(_a0: Ptr<EndFunctionWithPrivateEndMember>) -> Ptr<()> {
    return Ptr::<()>::null();
}
pub fn end_31(bf: Ptr<EndFunctionWithPrivateEndMember>) -> Ptr<()> {
    return (field_ptr!(bf, y));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct EndFunctionWithPrivateEndMember {
    #[offset(0)]
    pub y: (),
}
pub fn end_32(bf: Ptr<BeginMemberEndFunction>) -> Ptr<()> {
    return (field_ptr!(bf, x));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct BeginMemberEndFunction {
    #[offset(0)]
    pub x: (),
}
pub fn testEndFunction_33() -> () {
    let a: Value<EndFunction> = Rc::new(RefCell::new(<EndFunction>::default()));
    unimplemented!();
    unimplemented!();
    let aa: Value<EndFunction> = Rc::new(RefCell::new(<EndFunction>::default()));
    unimplemented!();
    unimplemented!();
    let b: Value<EndFunctionByValue> = Rc::new(RefCell::new(<EndFunctionByValue>::default()));
    unimplemented!();
    unimplemented!();
    let mut c: EndFunctionEnabledBorrowing = <EndFunctionEnabledBorrowing>::default();
    unimplemented!();
    unimplemented!();
    let d: Value<EndFunctionReturnsEmptyPtr> =
        Rc::new(RefCell::new(<EndFunctionReturnsEmptyPtr>::default()));
    unimplemented!();
    unimplemented!();
    let dd: Value<EndFunctionReturnsEmptyPtr> =
        Rc::new(RefCell::new(<EndFunctionReturnsEmptyPtr>::default()));
    unimplemented!();
    unimplemented!();
    let e: Value<EndFunctionWithDataMember> =
        Rc::new(RefCell::new(<EndFunctionWithDataMember>::default()));
    unimplemented!();
    unimplemented!();
    let ee: Value<EndFunctionWithDataMember> =
        Rc::new(RefCell::new(<EndFunctionWithDataMember>::default()));
    unimplemented!();
    unimplemented!();
    let f: Value<EndFunctionWithPrivateEndMember> =
        Rc::new(RefCell::new(<EndFunctionWithPrivateEndMember>::default()));
    unimplemented!();
    unimplemented!();
    let ff: Value<EndFunctionWithPrivateEndMember> =
        Rc::new(RefCell::new(<EndFunctionWithPrivateEndMember>::default()));
    unimplemented!();
    unimplemented!();
    let g: Value<BeginMemberEndFunction> =
        Rc::new(RefCell::new(<BeginMemberEndFunction>::default()));
    unimplemented!();
    unimplemented!();
    let gg: Value<BeginMemberEndFunction> =
        Rc::new(RefCell::new(<BeginMemberEndFunction>::default()));
    unimplemented!();
    unimplemented!();
    return true;
}
const _: () = assert!(true, "");
const _: () = assert!(true, "");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoThrowMemberEnd {}
thread_local!(
    pub static ntme_34: Value<NoThrowMemberEnd> =
        Rc::new(RefCell::new(<NoThrowMemberEnd>::default()));
);
const _: () = assert!(true, "noexcept(std::ranges::end(ntme))");
const _: () = assert!(true, "noexcept(std::ranges::cend(ntme))");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoThrowADLEnd {}
thread_local!(
    pub static ntae_35: Value<NoThrowADLEnd> = Rc::new(RefCell::new(<NoThrowADLEnd>::default()));
);
const _: () = assert!(true, "noexcept(std::ranges::end(ntae))");
const _: () = assert!(true, "noexcept(std::ranges::cend(ntae))");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoThrowMemberEndReturnsRef {}
thread_local!(
    pub static ntmerr_36: Value<NoThrowMemberEndReturnsRef> =
        Rc::new(RefCell::new(<NoThrowMemberEndReturnsRef>::default()));
);
const _: () = assert!(true, "!noexcept(std::ranges::end(ntmerr))");
const _: () = assert!(true, "!noexcept(std::ranges::cend(ntmerr))");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct EndReturnsArrayRef {}
thread_local!(
    pub static erar_37: Value<EndReturnsArrayRef> =
        Rc::new(RefCell::new(<EndReturnsArrayRef>::default()));
);
const _: () = assert!(true, "noexcept(std::ranges::end(erar))");
const _: () = assert!(true, "noexcept(std::ranges::cend(erar))");
const _: () = assert!(true, "!std::is_invocable_v<RangeEndT, Holder<Incomplete>*>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeEndT, Holder<Incomplete>*&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCEndT, Holder<Incomplete>*>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<RangeCEndT, Holder<Incomplete>*&>"
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
    ({ testArray_16() });
    const _: () = assert!(true, "testArray()");;
    ({ testEndMember_18() });
    const _: () = assert!(true, "testEndMember()");;
    ({ testEndFunction_33() });
    const _: () = assert!(true, "testEndFunction()");;
    return 0_();
}
pub trait BeginMemberEndFunctionImpl {
    fn begin(&self) -> Ptr<()>;
}
impl BeginMemberEndFunctionImpl for Ptr<BeginMemberEndFunction> {
    fn begin(&self) -> Ptr<()> {
        return Ptr::<()>::null();
    }
}
pub trait Different_15Impl {
    fn begin_1(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end_2(&self) -> Ptr<sentinel_wrapper_char_ptr_> {
        unimplemented!()
    }
    fn begin_3(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end_4(&self) -> Ptr<sentinel_wrapper_short_ptr_> {
        unimplemented!()
    }
}
impl Different_15Impl for Ptr<Different_15> {}
pub trait EmptyEndMemberImpl {
    fn begin(&self) -> Empty {
        unimplemented!()
    }
    fn end(&self) -> Empty {
        unimplemented!()
    }
}
impl EmptyEndMemberImpl for Ptr<EmptyEndMember> {}
pub trait EmptyPtrEndMemberImpl {
    fn begin(&self) -> Ptr<Empty>;
    fn end(&self) -> Ptr<Empty>;
}
impl EmptyPtrEndMemberImpl for Ptr<EmptyPtrEndMember> {
    fn begin(&self) -> Ptr<Empty> {
        return Ptr::<Empty>::null();
    }
    fn end(&self) -> Ptr<Empty> {
        return (field_ptr!((*self), x));
    }
}
pub trait EnabledBorrowingEndMemberImpl {
    fn begin(&self) -> Ptr<()>;
    fn end(&self) -> Ptr<()>;
}
impl EnabledBorrowingEndMemberImpl for Ptr<EnabledBorrowingEndMember> {
    fn begin(&self) -> Ptr<()> {
        return Ptr::<()>::null();
    }
    fn end(&self) -> Ptr<()> {
        return ((globalBuff_13.with(|v| v.as_pointer()) as Ptr<()>).offset(0_()));
    }
}
pub trait EndFunctionWithPrivateEndMemberImpl {
    fn end(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl EndFunctionWithPrivateEndMemberImpl for Ptr<EndFunctionWithPrivateEndMember> {}
pub trait EndMemberImpl {
    fn begin(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end(&self) -> Ptr<()>;
}
impl EndMemberImpl for Ptr<EndMember> {
    fn end(&self) -> Ptr<()> {
        return (field_ptr!((*self), x));
    }
}
pub trait EndMemberFunctionImpl {
    fn begin(&self) -> Ptr<()>;
    fn end(&self) -> Ptr<()>;
}
impl EndMemberFunctionImpl for Ptr<EndMemberFunction> {
    fn begin(&self) -> Ptr<()> {
        return Ptr::<()>::null();
    }
    fn end(&self) -> Ptr<()> {
        return (field_ptr!((*self), x));
    }
}
pub trait EndMemberReturnsIntImpl {
    fn begin(&self) -> () {
        unimplemented!()
    }
    fn end(&self) -> () {
        unimplemented!()
    }
}
impl EndMemberReturnsIntImpl for Ptr<EndMemberReturnsInt> {}
pub trait EndMemberReturnsVoidPtrImpl {
    fn begin(&self) -> () {
        unimplemented!()
    }
    fn end(&self) -> () {
        unimplemented!()
    }
}
impl EndMemberReturnsVoidPtrImpl for Ptr<EndMemberReturnsVoidPtr> {}
pub trait EndReturnsArrayRefImpl {
    fn begin(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl EndReturnsArrayRefImpl for Ptr<EndReturnsArrayRef> {}
pub trait NoBeginMemberImpl {
    fn end(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl NoBeginMemberImpl for Ptr<NoBeginMember> {}
pub trait NoThrowADLEndImpl {
    fn begin(&self) -> ThrowingIterator_int_ {
        unimplemented!()
    }
}
impl NoThrowADLEndImpl for Ptr<NoThrowADLEnd> {}
pub trait NoThrowMemberEndImpl {
    fn begin(&self) -> ThrowingIterator_int_ {
        unimplemented!()
    }
    fn end(&self) -> ThrowingIterator_int_ {
        unimplemented!()
    }
}
impl NoThrowMemberEndImpl for Ptr<NoThrowMemberEnd> {}
pub trait NoThrowMemberEndReturnsRefImpl {
    fn begin(&self) -> ThrowingIterator_int_ {
        unimplemented!()
    }
    fn end(&self) -> Ptr<ThrowingIterator_int_> {
        unimplemented!()
    }
}
impl NoThrowMemberEndReturnsRefImpl for Ptr<NoThrowMemberEndReturnsRef> {}
pub trait NonConstEndMemberImpl {
    fn begin(&self) -> Ptr<()>;
    fn end(&self) -> Ptr<()>;
}
impl NonConstEndMemberImpl for Ptr<NonConstEndMember> {
    fn begin(&self) -> Ptr<()> {
        return Ptr::<()>::null();
    }
    fn end(&self) -> Ptr<()> {
        return (field_ptr!((*self), x));
    }
}
pub trait PtrConvertibleImpl {
    fn to_pmut(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl PtrConvertibleImpl for Ptr<PtrConvertible> {}
pub trait PtrConvertibleEndMemberImpl {
    fn begin(&self) -> PtrConvertible {
        unimplemented!()
    }
    fn end(&self) -> PtrConvertible {
        unimplemented!()
    }
}
impl PtrConvertibleEndMemberImpl for Ptr<PtrConvertibleEndMember> {}
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
pub trait sentinel_wrapper_char_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_char_ptr_Impl for Ptr<sentinel_wrapper_char_ptr_> {}
pub trait sentinel_wrapper_short_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_short_ptr_Impl for Ptr<sentinel_wrapper_short_ptr_> {}
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
    let _ = globalBuff_13.with(|_| ());
    let _ = enable_borrowed_range_17.with(|_| ());
    let _ = enable_borrowed_range_25.with(|_| ());
    let _ = ntme_34.with(|_| ());
    let _ = ntae_35.with(|_| ());
    let _ = ntmerr_36.with(|_| ());
    let _ = erar_37.with(|_| ());
}
