extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type check_unqualified_lookup_unscoped_enum = ();
pub const check_unqualified_lookup_unscoped_enum_a: check_unqualified_lookup_unscoped_enum = 0;
pub const check_unqualified_lookup_unscoped_enum_b: check_unqualified_lookup_unscoped_enum = 1;
pub const check_unqualified_lookup_unscoped_enum_c: check_unqualified_lookup_unscoped_enum = 2;
pub fn iter_move_0(
    e: Ptr<check_unqualified_lookup_unscoped_enum>,
) -> check_unqualified_lookup_unscoped_enum {
    return (e.read());
}
pub type check_unqualified_lookup_scoped_enum = ();
pub const check_unqualified_lookup_scoped_enum_a: check_unqualified_lookup_scoped_enum = 0;
pub const check_unqualified_lookup_scoped_enum_b: check_unqualified_lookup_scoped_enum = 1;
pub const check_unqualified_lookup_scoped_enum_c: check_unqualified_lookup_scoped_enum = 2;
pub fn iter_move_1(
    _a0: Ptr<check_unqualified_lookup_scoped_enum>,
) -> Ptr<check_unqualified_lookup_scoped_enum> {
    return Ptr::<check_unqualified_lookup_scoped_enum>::null();
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(8)]
pub struct check_unqualified_lookup_some_union {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Box<[u8]>,
}
impl check_unqualified_lookup_some_union {
    pub fn x(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
    pub fn y(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
}
impl Default for check_unqualified_lookup_some_union {
    fn default() -> Self {
        check_unqualified_lookup_some_union {
            __bytes: Box::from([0u8; 8]),
        }
    }
}
pub fn iter_move_2(u: Ptr<check_unqualified_lookup_some_union>) -> () {
    return (check_unqualified_lookup_some_union::x(u.clone()).read());
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(4)]
pub struct move_tracker {
    #[offset(0)]
    moves_: (),
}
impl move_tracker {
    pub fn move_from(other: Ptr<move_tracker>) -> Self {
        let __this: move_tracker = Self {
            moves_: (other.with(|__s| __s.moves_) + 1_()),
        };
        field!(other, moves_).write(0_());
        __this
    }
}
impl Default for move_tracker {
    fn default() -> Self {
        { move_tracker { moves_: 0_() } }
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
impl cpp17_input_iterator_int_ptr__int_ptr_ {
    pub fn new(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
}
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
impl forward_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
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
pub fn operator_eq_3(
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
            operator_eq_3(
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
pub fn operator_add_4(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_5(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_4(_i, n)
    });
}
pub fn iter_swap_6(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_7(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_8(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_9(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
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
            operator_cmp_9(
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
            operator_eq_8(
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
pub fn iter_swap_10(a: Ptr<HasIterSwap>, b: Ptr<HasIterSwap>) {
    a.with(|__s| __s.value_.clone()).write(1_());
    b.with(|__s| __s.value_.clone()).write(1_());
}
pub fn iter_swap_11(a: Ptr<HasIterSwap>, b: Ptr<()>) {
    a.with(|__s| __s.value_.clone()).write(2_());
    b.write(2_());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct HasIterSwap {
    #[offset(0)]
    #[byte_size(8)]
    pub value_: Ptr<()>,
}
impl HasIterSwap {
    pub fn new(value: Ptr<()>) -> Self {
        let __this: HasIterSwap = Self {
            value_: (value).clone(),
        };
        unimplemented!();
        __this
    }
}
const _: () = assert!(
    true,
    "std::is_invocable_v<IterSwapT, HasIterSwap&, HasIterSwap&>"
);
const _: () = assert!(true, "std::is_invocable_v<IterSwapT, HasIterSwap&, int&>");
const _: () = assert!(true, "!std::is_invocable_v<IterSwapT, int&, HasIterSwap&>");
const _: () = assert!(
    true,
    "std::is_invocable_v<IterSwapT&, HasIterSwap&, HasIterSwap&>"
);
const _: () = assert!(true, "std::is_invocable_v<IterSwapT&, HasIterSwap&, int&>");
const _: () = assert!(true, "!std::is_invocable_v<IterSwapT&, int&, HasIterSwap&>");
const _: () = assert!(
    true,
    "std::is_invocable_v<IterSwapT&&, HasIterSwap&, HasIterSwap&>"
);
const _: () = assert!(true, "std::is_invocable_v<IterSwapT&&, HasIterSwap&, int&>");
const _: () = assert!(
    true,
    "!std::is_invocable_v<IterSwapT&&, int&, HasIterSwap&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct StructWithNotMoreSpecializedIterSwap {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<IterSwapT, StructWithNotMoreSpecializedIterSwap&, StructWithNotMoreSpecializedIterSwap&>"
);
pub fn iter_swap_12(_a0: Ptr<NodiscardIterSwap>, _a1: Ptr<NodiscardIterSwap>) -> () {
    return 0_();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NodiscardIterSwap {}
pub fn ensureVoidCast_13(a: Ptr<NodiscardIterSwap>, b: Ptr<NodiscardIterSwap>) {
    ({
        let ___x: Ptr<NodiscardIterSwap> = (a).clone();
        let ___y: Ptr<NodiscardIterSwap> = (b).clone();
        (*iter_swap_14.with(Value::clone).borrow_mut())(___x, ___y)
    });
}
pub fn swap_15(a: Ptr<HasRangesSwap>, b: Ptr<HasRangesSwap>) {
    a.with(|__s| __s.value_.clone()).write(1_());
    b.with(|__s| __s.value_.clone()).write(1_());
}
pub fn swap_16(a: Ptr<HasRangesSwap>, b: Ptr<()>) {
    a.with(|__s| __s.value_.clone()).write(2_());
    b.write(2_());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct HasRangesSwap {
    #[offset(0)]
    #[byte_size(8)]
    pub value_: Ptr<()>,
}
impl HasRangesSwap {
    pub fn new(value: Ptr<()>) -> Self {
        let __this: HasRangesSwap = Self {
            value_: (value).clone(),
        };
        unimplemented!();
        __this
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct HasRangesSwapWrapper {
    #[offset(0)]
    #[byte_size(8)]
    pub value_: Ptr<HasRangesSwap>,
}
impl HasRangesSwapWrapper {
    pub fn new(value: Ptr<HasRangesSwap>) -> Self {
        Self {
            value_: (value).clone(),
        }
    }
}
const _: () = assert!(
    true,
    "std::is_invocable_v<IterSwapT, HasRangesSwapWrapper&, HasRangesSwapWrapper&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<IterSwapT, HasRangesSwapWrapper&, int&>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<IterSwapT, int&, HasRangesSwapWrapper&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct A {
    #[offset(0)]
    pub value: (),
}
impl Default for A {
    fn default() -> Self {
        { A { value: false } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct B {
    #[offset(0)]
    pub value: (),
}
impl Default for B {
    fn default() -> Self {
        { B { value: false } }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(1)]
pub struct MoveOnly1 {
    #[offset(0)]
    pub value: (),
}
impl MoveOnly1 {
    pub fn move_from(_a0: Ptr<MoveOnly1>) -> Self {
        Self {
            value: { (*_a0.upgrade().deref()).value },
        }
    }
}
impl Default for MoveOnly1 {
    fn default() -> Self {
        { MoveOnly1 { value: false } }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(1)]
pub struct MoveOnly2 {
    #[offset(0)]
    pub value: (),
}
impl MoveOnly2 {
    pub fn move_from(_a0: Ptr<MoveOnly2>) -> Self {
        Self {
            value: { (*_a0.upgrade().deref()).value },
        }
    }
}
impl Default for MoveOnly2 {
    fn default() -> Self {
        { MoveOnly2 { value: false } }
    }
}
pub fn test_17() -> () {
    {
        let value1: Value<()> = Rc::new(RefCell::new(0_()));
        let value2: Value<()> = Rc::new(RefCell::new(0_()));
        let a: Value<HasIterSwap> =
            Rc::new(RefCell::new(HasIterSwap::new({ value1.as_pointer() })));
        let b: Value<HasIterSwap> =
            Rc::new(RefCell::new(HasIterSwap::new({ value2.as_pointer() })));
        ({ (*iter_swap_14.with(Value::clone).borrow_mut())(a.as_pointer(), b.as_pointer()) });
        unimplemented!();
    }
    {
        let value1: Value<()> = Rc::new(RefCell::new(0_()));
        let value2: Value<()> = Rc::new(RefCell::new(0_()));
        let c: Value<HasRangesSwap> =
            Rc::new(RefCell::new(HasRangesSwap::new({ value1.as_pointer() })));
        let d: Value<HasRangesSwap> =
            Rc::new(RefCell::new(HasRangesSwap::new({ value2.as_pointer() })));
        let cWrapper: Value<HasRangesSwapWrapper> =
            Rc::new(RefCell::new(HasRangesSwapWrapper::new({ c.as_pointer() })));
        let dWrapper: Value<HasRangesSwapWrapper> =
            Rc::new(RefCell::new(HasRangesSwapWrapper::new({ d.as_pointer() })));
        ({
            (*iter_swap_14.with(Value::clone).borrow_mut())(
                cWrapper.as_pointer(),
                dWrapper.as_pointer(),
            )
        });
        unimplemented!();
    }
    {
        let value1: Value<()> = Rc::new(RefCell::new(0_()));
        let value2: Value<()> = Rc::new(RefCell::new(0_()));
        let c: Value<HasRangesSwap> =
            Rc::new(RefCell::new(HasRangesSwap::new({ value1.as_pointer() })));
        let d: Value<HasRangesSwap> =
            Rc::new(RefCell::new(HasRangesSwap::new({ value2.as_pointer() })));
        ({
            let ___x: Value<HasRangesSwapWrapper> =
                Rc::new(RefCell::new(HasRangesSwapWrapper::new({ c.as_pointer() })));
            let ___y: Value<HasRangesSwapWrapper> =
                Rc::new(RefCell::new(HasRangesSwapWrapper::new({ d.as_pointer() })));
            (*iter_swap_14.with(Value::clone).borrow_mut())(___x.as_pointer(), ___y.as_pointer())
        });
        unimplemented!();
    }
    {
        let e: Value<A> = Rc::new(RefCell::new(<A>::default()));
        let f: Value<B> = Rc::new(RefCell::new(<B>::default()));
        let ePtr: Value<Ptr<A>> = Rc::new(RefCell::new((e.as_pointer())));
        let fPtr: Value<Ptr<B>> = Rc::new(RefCell::new((f.as_pointer())));
        ({ (*iter_swap_14.with(Value::clone).borrow_mut())(ePtr.as_pointer(), fPtr.as_pointer()) });
        unimplemented!();
    }
    {
        let g: Value<MoveOnly1> = Rc::new(RefCell::new(<MoveOnly1>::default()));
        let h: Value<MoveOnly2> = Rc::new(RefCell::new(<MoveOnly2>::default()));
        ({
            let ___x: Value<Ptr<MoveOnly1>> = Rc::new(RefCell::new((g.as_pointer())));
            let ___y: Value<Ptr<MoveOnly2>> = Rc::new(RefCell::new((h.as_pointer())));
            (*iter_swap_14.with(Value::clone).borrow_mut())(___x.as_pointer(), ___y.as_pointer())
        });
        unimplemented!();
    }
    {
        let arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let ___x: Value<Ptr<move_tracker>> = Rc::new(RefCell::new(unimplemented!()));
            let ___y: Value<Ptr<move_tracker>> =
                Rc::new(RefCell::new(unimplemented!().offset((1_()) as isize)));
            (*iter_swap_14.with(Value::clone).borrow_mut())(___x.as_pointer(), ___y.as_pointer())
        });
        {
            unimplemented!();
        }
    }
    {
        let buff: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_()])));
        ({
            let ___x: Value<Ptr<()>> = Rc::new(RefCell::new(
                (buff.as_pointer() as Ptr<()>).offset((0_()) as isize),
            ));
            let ___y: Value<Ptr<()>> = Rc::new(RefCell::new(
                (buff.as_pointer() as Ptr<()>).offset((1_()) as isize),
            ));
            (*iter_swap_14.with(Value::clone).borrow_mut())(___x.as_pointer(), ___y.as_pointer())
        });
        unimplemented!();
        ({
            let ___x: Value<cpp20_input_iterator_int_ptr_> =
                Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                    (buff.as_pointer() as Ptr<()>)
                })));
            let ___y: Value<cpp20_input_iterator_int_ptr_> =
                Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                    (buff.as_pointer() as Ptr<()>).offset((1_()) as isize)
                })));
            (*iter_swap_14.with(Value::clone).borrow_mut())(___x.as_pointer(), ___y.as_pointer())
        });
        unimplemented!();
        ({
            let ___x: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                    (buff.as_pointer() as Ptr<()>)
                })));
            let ___y: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                    (buff.as_pointer() as Ptr<()>).offset((1_()) as isize)
                })));
            (*iter_swap_14.with(Value::clone).borrow_mut())(___x.as_pointer(), ___y.as_pointer())
        });
        unimplemented!();
        ({
            let ___x: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    (buff.as_pointer() as Ptr<()>)
                })));
            let ___y: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    (buff.as_pointer() as Ptr<()>).offset((1_()) as isize)
                })));
            (*iter_swap_14.with(Value::clone).borrow_mut())(___x.as_pointer(), ___y.as_pointer())
        });
        unimplemented!();
        ({
            let ___x: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    (buff.as_pointer() as Ptr<()>)
                })));
            let ___y: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    (buff.as_pointer() as Ptr<()>).offset((1_()) as isize)
                })));
            (*iter_swap_14.with(Value::clone).borrow_mut())(___x.as_pointer(), ___y.as_pointer())
        });
        unimplemented!();
        ({
            let ___x: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    (buff.as_pointer() as Ptr<()>)
                })));
            let ___y: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    (buff.as_pointer() as Ptr<()>).offset((1_()) as isize)
                })));
            (*iter_swap_14.with(Value::clone).borrow_mut())(___x.as_pointer(), ___y.as_pointer())
        });
        unimplemented!();
        ({
            let ___x: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    (buff.as_pointer() as Ptr<()>)
                })));
            let ___y: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    (buff.as_pointer() as Ptr<()>).offset((1_()) as isize)
                })));
            (*iter_swap_14.with(Value::clone).borrow_mut())(___x.as_pointer(), ___y.as_pointer())
        });
        unimplemented!();
    }
    return true;
}
const _: () = assert!(true, "!std::is_invocable_v<IterSwapT, int*>");
const _: () = assert!(true, "!std::is_invocable_v<IterSwapT, int*, int*, int*>");
const _: () = assert!(true, "!std::is_invocable_v<IterSwapT, int, int*>");
const _: () = assert!(true, "!std::is_invocable_v<IterSwapT, int*, int>");
const _: () = assert!(true, "!std::is_invocable_v<IterSwapT, void*, void*>");
const _: () = assert!(
    true,
    "std::is_invocable_v<IterSwapT, Holder<Incomplete>**, Holder<Incomplete>**>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<IterSwapT, Holder<Incomplete>**, Holder<Incomplete>**&>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<IterSwapT, Holder<Incomplete>**&, Holder<Incomplete>**>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<IterSwapT, Holder<Incomplete>**&, Holder<Incomplete>**&>"
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
    ({ test_17() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait AImpl {
    fn operator_assign_1(&self, _a0: Ptr<B>) -> Ptr<A>;
}
impl AImpl for Ptr<A> {
    fn operator_assign_1(&self, _a0: Ptr<B>) -> Ptr<A> {
        field!((*self), value).write(true);
        return (*self).clone();
    }
}
pub trait BImpl {
    fn operator_assign_1(&self, _a0: Ptr<A>) -> Ptr<B>;
}
impl BImpl for Ptr<B> {
    fn operator_assign_1(&self, _a0: Ptr<A>) -> Ptr<B> {
        field!((*self), value).write(true);
        return (*self).clone();
    }
}
pub trait HasRangesSwapWrapperImpl {
    fn operator_deref(&self) -> Ptr<HasRangesSwap>;
}
impl HasRangesSwapWrapperImpl for Ptr<HasRangesSwapWrapper> {
    fn operator_deref(&self) -> Ptr<HasRangesSwap> {
        return ((*self).with(|__s| __s.value_.clone())).clone();
    }
}
pub trait MoveOnly1Impl {
    fn move_assign(&self, _a0: Ptr<MoveOnly1>) -> Ptr<MoveOnly1>;
    fn operator_assign_6(&self, _a0: Ptr<MoveOnly2>) -> Ptr<MoveOnly1>;
}
impl MoveOnly1Impl for Ptr<MoveOnly1> {
    fn move_assign(&self, _a0: Ptr<MoveOnly1>) -> Ptr<MoveOnly1> {
        field!((*self), value).write({ { (*_a0.upgrade().deref()).value } });
        return (*self).clone();
    }
    fn operator_assign_6(&self, _a0: Ptr<MoveOnly2>) -> Ptr<MoveOnly1> {
        field!((*self), value).write(true);
        return (*self).clone();
    }
}
pub trait MoveOnly2Impl {
    fn move_assign(&self, _a0: Ptr<MoveOnly2>) -> Ptr<MoveOnly2>;
    fn operator_assign_6(&self, _a0: Ptr<MoveOnly1>) -> Ptr<MoveOnly2>;
}
impl MoveOnly2Impl for Ptr<MoveOnly2> {
    fn move_assign(&self, _a0: Ptr<MoveOnly2>) -> Ptr<MoveOnly2> {
        field!((*self), value).write({ { (*_a0.upgrade().deref()).value } });
        return (*self).clone();
    }
    fn operator_assign_6(&self, _a0: Ptr<MoveOnly1>) -> Ptr<MoveOnly2> {
        field!((*self), value).write(true);
        return (*self).clone();
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
pub trait bidirectional_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> ();
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
impl bidirectional_iterator_int_ptr_Impl for Ptr<bidirectional_iterator_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
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
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: ()) -> contiguous_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: ()) -> contiguous_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_int_ptr_> {
        unimplemented!()
    }
}
impl contiguous_iterator_int_ptr_Impl for Ptr<contiguous_iterator_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait cpp17_input_iterator_int_ptr__int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp17_input_iterator_int_ptr__int_ptr_ {
        unimplemented!()
    }
}
impl cpp17_input_iterator_int_ptr__int_ptr_Impl for Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
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
    fn operator_deref(&self) -> ();
    fn operator_inc_6(&self) -> Ptr<forward_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_7(&self, mut _a0: ()) -> forward_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl forward_iterator_int_ptr_Impl for Ptr<forward_iterator_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
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
pub trait move_trackerImpl {
    fn move_assign(&self, other: Ptr<move_tracker>) -> Ptr<move_tracker>;
    fn moves(&self) -> ();
}
impl move_trackerImpl for Ptr<move_tracker> {
    fn move_assign(&self, other: Ptr<move_tracker>) -> Ptr<move_tracker> {
        field!((*self), moves_).write({ (other.with(|__s| __s.moves_) + 1_()) });
        field!(other, moves_).write(0_());
        return (*self).clone();
    }
    fn moves(&self) -> () {
        return (*self).with(|__s| __s.moves_);
    }
}
pub trait random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> ();
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
    fn operator_add_assign(&self, mut n: ()) -> Ptr<random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<random_access_iterator_int_ptr_> {
        unimplemented!()
    }
}
impl random_access_iterator_int_ptr_Impl for Ptr<random_access_iterator_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
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
