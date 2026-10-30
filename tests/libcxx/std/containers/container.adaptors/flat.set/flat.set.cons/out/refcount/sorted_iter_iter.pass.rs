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
pub fn operator_ne_0(
    x: Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>,
    y: Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
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
    pub fn move_from(_a0: Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub fn operator_add_1(
    x: three_way_random_access_iterator_int_ptr_,
    mut n: (),
) -> three_way_random_access_iterator_int_ptr_ {
    let x: Value<three_way_random_access_iterator_int_ptr_> = Rc::new(RefCell::new(x));
    ({ three_way_random_access_iterator_int_ptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return three_way_random_access_iterator_int_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_add_2(
    mut n: (),
    x: three_way_random_access_iterator_int_ptr_,
) -> three_way_random_access_iterator_int_ptr_ {
    let x: Value<three_way_random_access_iterator_int_ptr_> = Rc::new(RefCell::new(x));
    ({ three_way_random_access_iterator_int_ptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return three_way_random_access_iterator_int_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_3(
    x: three_way_random_access_iterator_int_ptr_,
    mut n: (),
) -> three_way_random_access_iterator_int_ptr_ {
    let x: Value<three_way_random_access_iterator_int_ptr_> = Rc::new(RefCell::new(x));
    ({ three_way_random_access_iterator_int_ptr_Impl::operator_sub_assign(&x.as_pointer(), n) });
    return three_way_random_access_iterator_int_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_4(
    mut x: three_way_random_access_iterator_int_ptr_,
    mut y: three_way_random_access_iterator_int_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn operator_eq_5(
    x: Ptr<three_way_random_access_iterator_int_ptr_>,
    y: Ptr<three_way_random_access_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_lt_6(
    x: Ptr<three_way_random_access_iterator_int_ptr_>,
    y: Ptr<three_way_random_access_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } < { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct three_way_random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl three_way_random_access_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<three_way_random_access_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppLess for three_way_random_access_iterator_int_ptr_ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_6(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for three_way_random_access_iterator_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_5(__lhs.clone(), __rhs.clone())
    }
}
impl three_way_random_access_iterator_const_int_ptr_ {
    pub fn new_1(u: Ptr<three_way_random_access_iterator_int_ptr_>) -> Self {
        Self {
            it_: u.with(|__s| __s.it_.clone()),
            tracker_: u.with(|__s| __s.tracker_.clone()),
        }
    }
}
impl three_way_random_access_iterator_const_int_ptr_ {
    pub fn new_2(u: Ptr<three_way_random_access_iterator_int_ptr_>) -> Self {
        let __this: three_way_random_access_iterator_const_int_ptr_ = Self {
            it_: u.with(|__s| __s.it_.clone()),
            tracker_: support_double_move_tracker::move_from({ field_ptr!(u, tracker_) }),
        };
        field!(u, it_).write(Ptr::<()>::null());
        __this
    }
}
pub fn operator_add_7(
    x: three_way_random_access_iterator_const_int_ptr_,
    mut n: (),
) -> three_way_random_access_iterator_const_int_ptr_ {
    let x: Value<three_way_random_access_iterator_const_int_ptr_> = Rc::new(RefCell::new(x));
    ({
        three_way_random_access_iterator_const_int_ptr_Impl::operator_add_assign(&x.as_pointer(), n)
    });
    return three_way_random_access_iterator_const_int_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_8(
    mut x: three_way_random_access_iterator_const_int_ptr_,
    mut y: three_way_random_access_iterator_const_int_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn operator_eq_9(
    x: Ptr<three_way_random_access_iterator_const_int_ptr_>,
    y: Ptr<three_way_random_access_iterator_const_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct three_way_random_access_iterator_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl three_way_random_access_iterator_const_int_ptr_ {
    pub fn new_3(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<three_way_random_access_iterator_const_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppEq for three_way_random_access_iterator_const_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_9(__lhs.clone(), __rhs.clone())
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
pub fn operator_eq_10(
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
        operator_eq_10(__lhs.clone(), __rhs.clone())
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
pub fn operator_add_11(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_12(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_11(_i, n)
    });
}
pub fn iter_swap_13(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_14(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_15(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_16(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__
{
    #[offset(0)]
    #[byte_size(24)]
    data_: (),
}
impl MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__ { pub fn move_from ( _a0 : Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  > , ) -> Self { Self { data_ : unimplemented!()  , }  } }
const _: () = assert!(!(false), "!HasAllocatorType<MinSequenceContainer<int>>");
const _: () = assert!(!(false), "!HasEmplace<MinSequenceContainer<int>>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_18.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_19.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_20.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_17.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_18.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_19.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_17: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_18: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_19: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_20: Value<()> = Rc::new(RefCell::new(false));
);
pub fn operator_sub_21(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_lt_22(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } < { (y.ptr_).clone() });
}
pub fn operator_eq_23(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_24(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return !({
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_23(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
}
impl CppLess for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_22(__lhs.read(), __rhs.read())
    }
}
impl CppEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_23(__lhs.read(), __rhs.read())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub fn operator_sub_25(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_eq_26(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_27(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } != { (y.ptr_).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    pub fn new_1(mut p: min_pointer_int__std_integral_constant_unsigned_long__0__) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
}
impl CppEq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_26(__lhs.read(), __rhs.read())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct test_allocator_statistics {
    #[offset(0)]
    pub time_to_throw: (),
    #[offset(4)]
    pub throw_after: (),
    #[offset(8)]
    pub count: (),
    #[offset(12)]
    pub alloc_count: (),
    #[offset(16)]
    pub allocated_size: (),
    #[offset(20)]
    pub construct_count: (),
    #[offset(24)]
    pub destroy_count: (),
    #[offset(28)]
    pub copied: (),
    #[offset(32)]
    pub moved: (),
    #[offset(36)]
    pub converted: (),
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        {
            test_allocator_statistics {
                time_to_throw: 0_(),
                throw_after: 2147483647_(),
                count: 0_(),
                alloc_count: 0_(),
                allocated_size: 0_(),
                construct_count: 0_(),
                destroy_count: 0_(),
                copied: 0_(),
                moved: 0_(),
                converted: 0_(),
            }
        }
    }
}
thread_local!(
    pub static destructed_value_28: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_29: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_30(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_31(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_30(_x, _y)
    });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(16)]
pub struct test_allocator_void_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_void_ {
    pub fn new_1(mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: 0_(),
            id_: 0_(),
            stats_: (stats).clone(),
        }
    }
    pub fn new_2(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_3(mut data: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: (stats).clone(),
        }
    }
    pub fn new_4(mut data: (), mut id: ()) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_5(mut data: (), mut id: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: (stats).clone(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_void_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }
    }
}
impl CppEq for test_allocator_void_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_30(__lhs.clone(), __rhs.clone())
    }
}
impl Clone for test_allocator_void_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_void_> = Rc::new(RefCell::new(test_allocator_void_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_void_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_void_ {
    fn default() -> Self {
        {
            test_allocator_void_ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
pub fn operator_eq_32(x: Ptr<test_allocator_int_>, y: Ptr<test_allocator_int_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_int__rebind_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_int__rebind_int_ptr_ {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor)]
#[byte_size(16)]
pub struct test_allocator_int_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_int_ {
    pub fn new_1(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_int_>) -> Self {
        let __this: Value<test_allocator_int_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_int_> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(a: Ptr<test_allocator_int_>) -> Self {
        let __this: Value<test_allocator_int_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_int_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), moved).with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
        field!(a, id_).write(moved_value_29.with(|rc| *rc.borrow()));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppEq for test_allocator_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_32(__lhs.clone(), __rhs.clone())
    }
}
impl Clone for test_allocator_int_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_int_> = Rc::new(RefCell::new(test_allocator_int_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_int_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_int_ {
    fn default() -> Self {
        {
            test_allocator_int_ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
pub fn operator_eq_33(x: Ptr<test_allocator_long_>, y: Ptr<test_allocator_long_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_long__rebind_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_long__rebind_long_ptr_ {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor)]
#[byte_size(16)]
pub struct test_allocator_long_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_long_ {
    pub fn new_1(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_long_>) -> Self {
        let __this: Value<test_allocator_long_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_long_> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(a: Ptr<test_allocator_long_>) -> Self {
        let __this: Value<test_allocator_long_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_long_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), moved).with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
        field!(a, id_).write(moved_value_29.with(|rc| *rc.borrow()));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppEq for test_allocator_long_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_33(__lhs.clone(), __rhs.clone())
    }
}
impl Clone for test_allocator_long_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_long_> = Rc::new(RefCell::new(test_allocator_long_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_long_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_long_ {
    fn default() -> Self {
        {
            test_allocator_long_ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
impl test_allocator_int_ptr_ {
    pub fn new_1(a: Ptr<test_allocator_int_>) -> Self {
        let __this: Value<test_allocator_int_ptr_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_int_ptr_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), converted).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_int_ptr_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_int_ptr_ {}
impl Default for test_allocator_int_ptr_ {
    fn default() -> Self {
        {
            test_allocator_int_ptr_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
impl test_allocator_long_ptr_ {
    pub fn new_1(a: Ptr<test_allocator_long_>) -> Self {
        let __this: Value<test_allocator_long_ptr_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_long_ptr_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), converted).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_long_ptr_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_long_ptr_ {}
impl Default for test_allocator_long_ptr_ {
    fn default() -> Self {
        {
            test_allocator_long_ptr_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct other_allocator_int_ {
    #[offset(0)]
    data_: (),
}
impl other_allocator_int_ {}
impl Default for other_allocator_int_ {
    fn default() -> Self {
        {
            other_allocator_int_ {
                data_: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Ctor_Tag {}
#[derive(Record, ByteRepr, FnPtrArg, CopyAssign, MoveAssign, Default)]
#[byte_size(1)]
pub struct Tag_X {}
impl Tag_X {
    pub fn new_1(mut _a0: Ctor_Tag) -> Self {
        Self {}
    }
    pub fn new_2(mut _a0: Ctor_Tag, _a1: Ptr<Tag_X>) -> Self {
        Self {}
    }
    pub fn new_3(mut _a0: Ctor_Tag, _a1: Ptr<Tag_X>) -> Self {
        Self {}
    }
}
pub fn operator_eq_34(a: Ptr<test_less_int_>, b: Ptr<test_less_int_>) -> () {
    return ({ a.with(|__s| __s.data_) } == { b.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct test_less_int_ {
    #[offset(0)]
    pub data_: (),
}
impl test_less_int_ {
    pub fn new_1() -> Self {
        Self { data_: 0_() }
    }
    pub fn new_2(mut data: ()) -> Self {
        Self { data_: data }
    }
}
impl CppEq for test_less_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_34(__lhs.clone(), __rhs.clone())
    }
}
impl Default for test_less_int_ {
    fn default() -> Self {
        { test_less_int_::new_1() }
    }
}
pub fn test_35() {
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 4_(), 5_()])));
        let m: Value<()> = Rc::new(RefCell::new(
            std_flat_set_int__std_less_int___std_vector_int__std_allocator_int___::new_1(
                { unimplemented!() },
                {
                    cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                        (ar.as_pointer() as Ptr<()>)
                    })
                },
                {
                    cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                        (ar.as_pointer() as Ptr<()>).offset((4_()) as isize)
                    })
                },
                None,
            ),
        ));
        let expected: Value<()> = Rc::new(RefCell::new(
            std_flat_set_int__std_less_int___std_vector_int__std_allocator_int___::new_2(
                { unimplemented!() },
                None,
            ),
        ));
        unimplemented!();
        let m2: Value<()> = Rc::new(RefCell::new(
            std_flat_set_int__std_less_int___std_vector_int__std_allocator_int___::new_1(
                { unimplemented!() },
                {
                    cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                        (ar.as_pointer() as Ptr<()>)
                    })
                },
                {
                    cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                        (ar.as_pointer() as Ptr<()>).offset((4_()) as isize)
                    })
                },
                None,
            ),
        ));
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 4_(), 5_()])));
        let m: Value<()> = Rc::new(RefCell::new(
            std_flat_set_int__test_less_int___std_vector_int__std_allocator_int___::new_1(
                { unimplemented!() },
                { (ar.as_pointer() as Ptr<()>) },
                { (ar.as_pointer() as Ptr<()>).offset((4_()) as isize) },
                None,
            ),
        ));
        let expected: Value<()> = Rc::new(RefCell::new(
            std_flat_set_int__test_less_int___std_vector_int__std_allocator_int___::new_2(
                { unimplemented!() },
                None,
            ),
        ));
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 4_(), 5_()])));
        let m: Value<()> = Rc::new(RefCell::new({
            let __tmp_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_flat_set_int__std_function_bool__int__int____std_vector_int__std_allocator_int___ :: new_1 ( { unimplemented!()  } , { cpp17_input_iterator_const_int_ptr__const_int_ptr_ :: new ( { ( ar .as_pointer()  as Ptr::<()> )  } , )   } , { cpp17_input_iterator_const_int_ptr__const_int_ptr_ :: new ( { ( ar .as_pointer()  as Ptr::<()> ) . offset ( ( 4_() ) as isize )  } , )   } , { __tmp_0.as_pointer() } , )
        }));
        unimplemented!();
        unimplemented!();
        let m2: Value<()> = Rc::new(RefCell::new({
            let __tmp_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_flat_set_int__std_function_bool__int__int____std_vector_int__std_allocator_int___ :: new_1 ( { unimplemented!()  } , { cpp17_input_iterator_const_int_ptr__const_int_ptr_ :: new ( { ( ar .as_pointer()  as Ptr::<()> )  } , )   } , { cpp17_input_iterator_const_int_ptr__const_int_ptr_ :: new ( { ( ar .as_pointer()  as Ptr::<()> ) . offset ( ( 4_() ) as isize )  } , )   } , { __tmp_1.as_pointer() } , )
        }));
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([5_(), 4_(), 2_(), 1_()])));
        let m: Value<()> = Rc::new(RefCell::new({
            let __tmp_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_flat_set_int__std_greater_int___std_vector_int__std_allocator_int___::new_1(
                { unimplemented!() },
                {
                    cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                        (ar.as_pointer() as Ptr<()>)
                    })
                },
                {
                    cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                        (ar.as_pointer() as Ptr<()>).offset((4_()) as isize)
                    })
                },
                { __tmp_2.as_pointer() },
            )
        }));
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([42_()])));
        let m: Value<()> = Rc::new(RefCell::new({
            let __tmp_3: Value<test_less_int_> =
                Rc::new(RefCell::new(test_less_int_::new_2({ 5_() })));
            std_flat_set_int__test_less_int___std_vector_int__std_allocator_int___::new_1(
                { unimplemented!() },
                { (ar.as_pointer() as Ptr<()>) },
                { (ar.as_pointer() as Ptr<()>) },
                { Some(__tmp_3.as_pointer()) },
            )
        }));
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_36() {
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 4_(), 5_()])));
        let m: Value<()> = Rc::new(RefCell::new(
            std_flat_set_int__std_less_int___std_vector_int__min_allocator_int___::new_1(
                { unimplemented!() },
                {
                    cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                        (ar.as_pointer() as Ptr<()>)
                    })
                },
                {
                    cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                        (ar.as_pointer() as Ptr<()>).offset((4_()) as isize)
                    })
                },
                None,
            ),
        ));
        let expected: Value<()> = Rc::new(RefCell::new(
            std_flat_set_int__std_less_int___std_vector_int__min_allocator_int___::new_2(
                { unimplemented!() },
                None,
            ),
        ));
        unimplemented!();
        let m2: Value<()> = Rc::new(RefCell::new(
            std_flat_set_int__std_less_int___std_vector_int__min_allocator_int___::new_1(
                { unimplemented!() },
                {
                    cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                        (ar.as_pointer() as Ptr<()>)
                    })
                },
                {
                    cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                        (ar.as_pointer() as Ptr<()>).offset((4_()) as isize)
                    })
                },
                None,
            ),
        ));
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 4_(), 5_()])));
        let m: Value<()> = Rc::new(RefCell::new(
            std_flat_set_int__test_less_int___std_vector_int__min_allocator_int___::new_1(
                { unimplemented!() },
                { (ar.as_pointer() as Ptr<()>) },
                { (ar.as_pointer() as Ptr<()>).offset((4_()) as isize) },
                None,
            ),
        ));
        let expected: Value<()> = Rc::new(RefCell::new(
            std_flat_set_int__test_less_int___std_vector_int__min_allocator_int___::new_2(
                { unimplemented!() },
                None,
            ),
        ));
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 4_(), 5_()])));
        let m: Value<()> = Rc::new(RefCell::new({
            let __tmp_4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_flat_set_int__std_function_bool__int__int____std_vector_int__min_allocator_int___ :: new_1 ( { unimplemented!()  } , { cpp17_input_iterator_const_int_ptr__const_int_ptr_ :: new ( { ( ar .as_pointer()  as Ptr::<()> )  } , )   } , { cpp17_input_iterator_const_int_ptr__const_int_ptr_ :: new ( { ( ar .as_pointer()  as Ptr::<()> ) . offset ( ( 4_() ) as isize )  } , )   } , { __tmp_4.as_pointer() } , )
        }));
        unimplemented!();
        unimplemented!();
        let m2: Value<()> = Rc::new(RefCell::new({
            let __tmp_5: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_flat_set_int__std_function_bool__int__int____std_vector_int__min_allocator_int___ :: new_1 ( { unimplemented!()  } , { cpp17_input_iterator_const_int_ptr__const_int_ptr_ :: new ( { ( ar .as_pointer()  as Ptr::<()> )  } , )   } , { cpp17_input_iterator_const_int_ptr__const_int_ptr_ :: new ( { ( ar .as_pointer()  as Ptr::<()> ) . offset ( ( 4_() ) as isize )  } , )   } , { __tmp_5.as_pointer() } , )
        }));
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([5_(), 4_(), 2_(), 1_()])));
        let m: Value<()> = Rc::new(RefCell::new({
            let __tmp_6: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_flat_set_int__std_greater_int___std_vector_int__min_allocator_int___::new_1(
                { unimplemented!() },
                {
                    cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                        (ar.as_pointer() as Ptr<()>)
                    })
                },
                {
                    cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                        (ar.as_pointer() as Ptr<()>).offset((4_()) as isize)
                    })
                },
                { __tmp_6.as_pointer() },
            )
        }));
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([42_()])));
        let m: Value<()> = Rc::new(RefCell::new({
            let __tmp_7: Value<test_less_int_> =
                Rc::new(RefCell::new(test_less_int_::new_2({ 5_() })));
            std_flat_set_int__test_less_int___std_vector_int__min_allocator_int___::new_1(
                { unimplemented!() },
                { (ar.as_pointer() as Ptr<()>) },
                { (ar.as_pointer() as Ptr<()>) },
                { Some(__tmp_7.as_pointer()) },
            )
        }));
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_37() {
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 4_(), 5_()])));
        let m : Value<() > = Rc::new(RefCell::new(std_flat_set_int__std_less_int___MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr___ :: new_1 ( { unimplemented!()  } , { cpp17_input_iterator_const_int_ptr__const_int_ptr_ :: new ( { ( ar .as_pointer()  as Ptr::<()> )  } , )   } , { cpp17_input_iterator_const_int_ptr__const_int_ptr_ :: new ( { ( ar .as_pointer()  as Ptr::<()> ) . offset ( ( 4_() ) as isize )  } , )   } , None , )  )) ;
        let expected : Value<() > = Rc::new(RefCell::new(std_flat_set_int__std_less_int___MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr___ :: new_2 ( { unimplemented!()  } , None , )  )) ;
        unimplemented!();
        let m2 : Value<() > = Rc::new(RefCell::new(std_flat_set_int__std_less_int___MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr___ :: new_1 ( { unimplemented!()  } , { cpp17_input_iterator_const_int_ptr__const_int_ptr_ :: new ( { ( ar .as_pointer()  as Ptr::<()> )  } , )   } , { cpp17_input_iterator_const_int_ptr__const_int_ptr_ :: new ( { ( ar .as_pointer()  as Ptr::<()> ) . offset ( ( 4_() ) as isize )  } , )   } , None , )  )) ;
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 4_(), 5_()])));
        let m : Value<() > = Rc::new(RefCell::new(std_flat_set_int__test_less_int___MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr___ :: new_1 ( { unimplemented!()  } , { ( ar .as_pointer()  as Ptr::<()> )  } , { ( ar .as_pointer()  as Ptr::<()> ) . offset ( ( 4_() ) as isize )  } , None , )  )) ;
        let expected : Value<() > = Rc::new(RefCell::new(std_flat_set_int__test_less_int___MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr___ :: new_2 ( { unimplemented!()  } , None , )  )) ;
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 4_(), 5_()])));
        let m: Value<()> = Rc::new(RefCell::new({
            let __tmp_8: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_flat_set_int__std_function_bool__int__int____MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr___ :: new_1 ( { unimplemented!()  } , { cpp17_input_iterator_const_int_ptr__const_int_ptr_ :: new ( { ( ar .as_pointer()  as Ptr::<()> )  } , )   } , { cpp17_input_iterator_const_int_ptr__const_int_ptr_ :: new ( { ( ar .as_pointer()  as Ptr::<()> ) . offset ( ( 4_() ) as isize )  } , )   } , { __tmp_8.as_pointer() } , )
        }));
        unimplemented!();
        unimplemented!();
        let m2: Value<()> = Rc::new(RefCell::new({
            let __tmp_9: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_flat_set_int__std_function_bool__int__int____MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr___ :: new_1 ( { unimplemented!()  } , { cpp17_input_iterator_const_int_ptr__const_int_ptr_ :: new ( { ( ar .as_pointer()  as Ptr::<()> )  } , )   } , { cpp17_input_iterator_const_int_ptr__const_int_ptr_ :: new ( { ( ar .as_pointer()  as Ptr::<()> ) . offset ( ( 4_() ) as isize )  } , )   } , { __tmp_9.as_pointer() } , )
        }));
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([5_(), 4_(), 2_(), 1_()])));
        let m: Value<()> = Rc::new(RefCell::new({
            let __tmp_10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_flat_set_int__std_greater_int___MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr___ :: new_1 ( { unimplemented!()  } , { cpp17_input_iterator_const_int_ptr__const_int_ptr_ :: new ( { ( ar .as_pointer()  as Ptr::<()> )  } , )   } , { cpp17_input_iterator_const_int_ptr__const_int_ptr_ :: new ( { ( ar .as_pointer()  as Ptr::<()> ) . offset ( ( 4_() ) as isize )  } , )   } , { __tmp_10.as_pointer() } , )
        }));
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([42_()])));
        let m: Value<()> = Rc::new(RefCell::new({
            let __tmp_11: Value<test_less_int_> =
                Rc::new(RefCell::new(test_less_int_::new_2({ 5_() })));
            std_flat_set_int__test_less_int___MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr___ :: new_1 ( { unimplemented!()  } , { ( ar .as_pointer()  as Ptr::<()> )  } , { ( ar .as_pointer()  as Ptr::<()> )  } , { Some ( __tmp_11.as_pointer() ) } , )
        }));
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_38() {
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 4_(), 5_()])));
        let m: Value<()> = Rc::new(RefCell::new(
            std_flat_set_int__std_less_int___std_deque_int__std_allocator_int___::new_1(
                { unimplemented!() },
                {
                    cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                        (ar.as_pointer() as Ptr<()>)
                    })
                },
                {
                    cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                        (ar.as_pointer() as Ptr<()>).offset((4_()) as isize)
                    })
                },
                None,
            ),
        ));
        let expected: Value<()> = Rc::new(RefCell::new(
            std_flat_set_int__std_less_int___std_deque_int__std_allocator_int___::new_2(
                { unimplemented!() },
                None,
            ),
        ));
        unimplemented!();
        let m2: Value<()> = Rc::new(RefCell::new(
            std_flat_set_int__std_less_int___std_deque_int__std_allocator_int___::new_1(
                { unimplemented!() },
                {
                    cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                        (ar.as_pointer() as Ptr<()>)
                    })
                },
                {
                    cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                        (ar.as_pointer() as Ptr<()>).offset((4_()) as isize)
                    })
                },
                None,
            ),
        ));
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 4_(), 5_()])));
        let m: Value<()> = Rc::new(RefCell::new(
            std_flat_set_int__test_less_int___std_deque_int__std_allocator_int___::new_1(
                { unimplemented!() },
                { (ar.as_pointer() as Ptr<()>) },
                { (ar.as_pointer() as Ptr<()>).offset((4_()) as isize) },
                None,
            ),
        ));
        let expected: Value<()> = Rc::new(RefCell::new(
            std_flat_set_int__test_less_int___std_deque_int__std_allocator_int___::new_2(
                { unimplemented!() },
                None,
            ),
        ));
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 4_(), 5_()])));
        let m: Value<()> = Rc::new(RefCell::new({
            let __tmp_12: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_flat_set_int__std_function_bool__int__int____std_deque_int__std_allocator_int___ :: new_1 ( { unimplemented!()  } , { cpp17_input_iterator_const_int_ptr__const_int_ptr_ :: new ( { ( ar .as_pointer()  as Ptr::<()> )  } , )   } , { cpp17_input_iterator_const_int_ptr__const_int_ptr_ :: new ( { ( ar .as_pointer()  as Ptr::<()> ) . offset ( ( 4_() ) as isize )  } , )   } , { __tmp_12.as_pointer() } , )
        }));
        unimplemented!();
        unimplemented!();
        let m2: Value<()> = Rc::new(RefCell::new({
            let __tmp_13: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_flat_set_int__std_function_bool__int__int____std_deque_int__std_allocator_int___ :: new_1 ( { unimplemented!()  } , { cpp17_input_iterator_const_int_ptr__const_int_ptr_ :: new ( { ( ar .as_pointer()  as Ptr::<()> )  } , )   } , { cpp17_input_iterator_const_int_ptr__const_int_ptr_ :: new ( { ( ar .as_pointer()  as Ptr::<()> ) . offset ( ( 4_() ) as isize )  } , )   } , { __tmp_13.as_pointer() } , )
        }));
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([5_(), 4_(), 2_(), 1_()])));
        let m: Value<()> = Rc::new(RefCell::new({
            let __tmp_14: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_flat_set_int__std_greater_int___std_deque_int__std_allocator_int___::new_1(
                { unimplemented!() },
                {
                    cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                        (ar.as_pointer() as Ptr<()>)
                    })
                },
                {
                    cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                        (ar.as_pointer() as Ptr<()>).offset((4_()) as isize)
                    })
                },
                { __tmp_14.as_pointer() },
            )
        }));
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([42_()])));
        let m: Value<()> = Rc::new(RefCell::new({
            let __tmp_15: Value<test_less_int_> =
                Rc::new(RefCell::new(test_less_int_::new_2({ 5_() })));
            std_flat_set_int__test_less_int___std_deque_int__std_allocator_int___::new_1(
                { unimplemented!() },
                { (ar.as_pointer() as Ptr<()>) },
                { (ar.as_pointer() as Ptr<()>) },
                { Some(__tmp_15.as_pointer()) },
            )
        }));
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_alloc_39() {
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 4_(), 5_()])));
        let m: Value<()> = Rc::new(RefCell::new({
            let __tmp_16: Value<test_allocator_int_> =
                Rc::new(RefCell::new(test_allocator_int_::new_1({ 5_() })));
            std_flat_set_int__std_less_int___std_vector_int__test_allocator_int___::new_1(
                { unimplemented!() },
                { (ar.as_pointer() as Ptr<()>) },
                { (ar.as_pointer() as Ptr<()>).offset((4_()) as isize) },
                { __tmp_16.as_pointer() },
            )
        }));
        let expected: Value<()> = Rc::new(RefCell::new(
            std_flat_set_int__std_less_int___std_vector_int__test_allocator_int___::new_2(
                { unimplemented!() },
                None,
            ),
        ));
        unimplemented!();
        unimplemented!();
        let m2: Value<()> = Rc::new(RefCell::new({
            let __tmp_17: Value<test_allocator_int_> =
                Rc::new(RefCell::new(test_allocator_int_::new_1({ 5_() })));
            std_flat_set_int__std_less_int___std_vector_int__test_allocator_int___::new_1(
                { unimplemented!() },
                { (ar.as_pointer() as Ptr<()>) },
                { (ar.as_pointer() as Ptr<()>).offset((4_()) as isize) },
                { __tmp_17.as_pointer() },
            )
        }));
        unimplemented!();
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 4_(), 5_()])));
        let m: Value<()> = Rc::new(RefCell::new({
            let __tmp_18: Value<test_less_int_> =
                Rc::new(RefCell::new(test_less_int_::new_2({ 3_() })));
            let __tmp_19: Value<test_allocator_int_> =
                Rc::new(RefCell::new(test_allocator_int_::new_1({ 5_() })));
            std_flat_set_int__test_less_int___std_vector_int__test_allocator_int___::new_1(
                { unimplemented!() },
                { (ar.as_pointer() as Ptr<()>) },
                { (ar.as_pointer() as Ptr<()>).offset((4_()) as isize) },
                { __tmp_18.as_pointer() },
                { __tmp_19.as_pointer() },
            )
        }));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 4_(), 5_()])));
        let m: Value<()> = Rc::new(RefCell::new({
            let __tmp_20: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let __tmp_21: Value<test_allocator_long_> =
                Rc::new(RefCell::new(test_allocator_long_::new_1({ 5_() })));
            std_flat_set_long__std_less_int___std_vector_long__test_allocator_long___::new_1(
                { unimplemented!() },
                { (ar.as_pointer() as Ptr<()>) },
                { (ar.as_pointer() as Ptr<()>).offset((4_()) as isize) },
                { __tmp_20.as_pointer() },
                { __tmp_21.as_pointer() },
            )
        }));
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_alloc_40() {
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 4_(), 5_()])));
        let m: Value<()> = Rc::new(RefCell::new({
            let __tmp_22: Value<test_allocator_int_> =
                Rc::new(RefCell::new(test_allocator_int_::new_1({ 5_() })));
            std_flat_set_int__std_less_int___std_deque_int__test_allocator_int___::new_1(
                { unimplemented!() },
                { (ar.as_pointer() as Ptr<()>) },
                { (ar.as_pointer() as Ptr<()>).offset((4_()) as isize) },
                { __tmp_22.as_pointer() },
            )
        }));
        let expected: Value<()> = Rc::new(RefCell::new(
            std_flat_set_int__std_less_int___std_deque_int__test_allocator_int___::new_2(
                { unimplemented!() },
                None,
            ),
        ));
        unimplemented!();
        unimplemented!();
        let m2: Value<()> = Rc::new(RefCell::new({
            let __tmp_23: Value<test_allocator_int_> =
                Rc::new(RefCell::new(test_allocator_int_::new_1({ 5_() })));
            std_flat_set_int__std_less_int___std_deque_int__test_allocator_int___::new_1(
                { unimplemented!() },
                { (ar.as_pointer() as Ptr<()>) },
                { (ar.as_pointer() as Ptr<()>).offset((4_()) as isize) },
                { __tmp_23.as_pointer() },
            )
        }));
        unimplemented!();
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 4_(), 5_()])));
        let m: Value<()> = Rc::new(RefCell::new({
            let __tmp_24: Value<test_less_int_> =
                Rc::new(RefCell::new(test_less_int_::new_2({ 3_() })));
            let __tmp_25: Value<test_allocator_int_> =
                Rc::new(RefCell::new(test_allocator_int_::new_1({ 5_() })));
            std_flat_set_int__test_less_int___std_deque_int__test_allocator_int___::new_1(
                { unimplemented!() },
                { (ar.as_pointer() as Ptr<()>) },
                { (ar.as_pointer() as Ptr<()>).offset((4_()) as isize) },
                { __tmp_24.as_pointer() },
                { __tmp_25.as_pointer() },
            )
        }));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 4_(), 5_()])));
        let m: Value<()> = Rc::new(RefCell::new({
            let __tmp_26: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let __tmp_27: Value<test_allocator_long_> =
                Rc::new(RefCell::new(test_allocator_long_::new_1({ 5_() })));
            std_flat_set_long__std_less_int___std_deque_long__test_allocator_long___::new_1(
                { unimplemented!() },
                { (ar.as_pointer() as Ptr<()>) },
                { (ar.as_pointer() as Ptr<()>).offset((4_()) as isize) },
                { __tmp_26.as_pointer() },
                { __tmp_27.as_pointer() },
            )
        }));
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_41() -> () {
    {
        const _: () = assert!(
            true,
            "std::is_constructible_v<M1, std::sorted_unique_t, Iter1, Iter1, const A1&>"
        );;
        const _: () = assert!(
            true,
            "std::is_constructible_v<M2, std::sorted_unique_t, Iter2, Iter2, const A2&>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<M1, std::sorted_unique_t, Iter1, Iter1, const A2&>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<M2, std::sorted_unique_t, Iter2, Iter2, const A1&>"
        );;
        const _: () = assert!(
            true,
            "std::is_constructible_v<M1, std::sorted_unique_t, Iter1, Iter1, const C&, const A1&>"
        );;
        const _: () = assert!(
            true,
            "std::is_constructible_v<M2, std::sorted_unique_t, Iter2, Iter2, const C&, const A2&>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<M1, std::sorted_unique_t, Iter1, Iter1, const C&, const A2&>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<M2, std::sorted_unique_t, Iter2, Iter2, const C&, const A1&>"
        );;
    }
    ({ test_35() });
    ({ test_36() });
    ({ test_37() });
    ({ test_alloc_39() });
    {
        ({ test_38() });
        ({ test_alloc_40() });
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
    ({ test_41() });
    return 0_();
}
pub trait MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl
{
    fn operator_assign_6 ( &self , il : ()  , ) -> Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  >{
        unimplemented!()
    }
    fn assign_8(&self, il: ()) {
        unimplemented!()
    }
    fn assign_9(&self, mut n: (), mut t: ()) {
        unimplemented!()
    }
    fn begin_11(&self) -> three_way_random_access_iterator_int_ptr_;
    fn begin_12(&self) -> three_way_random_access_iterator_const_int_ptr_;
    fn cbegin(&self) -> three_way_random_access_iterator_const_int_ptr_;
    fn end_14(&self) -> three_way_random_access_iterator_int_ptr_;
    fn end_15(&self) -> three_way_random_access_iterator_const_int_ptr_;
    fn cend(&self) -> three_way_random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn size(&self) -> ();
    fn empty(&self) -> ();
    fn clear(&self);
    fn insert_21(
        &self,
        mut p: three_way_random_access_iterator_const_int_ptr_,
        mut value: (),
    ) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn insert_22(
        &self,
        mut p: three_way_random_access_iterator_const_int_ptr_,
        mut n: (),
        mut value: (),
    ) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn insert_23(
        &self,
        mut p: three_way_random_access_iterator_const_int_ptr_,
        il: (),
    ) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn erase_25(
        &self,
        first: three_way_random_access_iterator_const_int_ptr_,
        last: three_way_random_access_iterator_const_int_ptr_,
    ) -> three_way_random_access_iterator_int_ptr_;
    fn erase_26(
        &self,
        mut iter: three_way_random_access_iterator_const_int_ptr_,
    ) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn to_vector_iterator(&self, cit: three_way_random_access_iterator_const_int_ptr_) -> ();
    fn from_vector_iterator(&self, it: ()) -> three_way_random_access_iterator_int_ptr_;
    fn move_assign ( &self , _a0 : Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  > , ) -> Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  > ;
    fn insert_20_class_cpp17_input_iterator_const_int_ptr_(
        &self,
        p: three_way_random_access_iterator_const_int_ptr_,
        first: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
        last: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    ) -> three_way_random_access_iterator_int_ptr_;
    fn insert_20_const_int_ptr(
        &self,
        p: three_way_random_access_iterator_const_int_ptr_,
        first: Ptr<()>,
        last: Ptr<()>,
    ) -> three_way_random_access_iterator_int_ptr_;
    fn insert_20_int_ptr(
        &self,
        p: three_way_random_access_iterator_const_int_ptr_,
        first: Ptr<()>,
        last: Ptr<()>,
    ) -> three_way_random_access_iterator_int_ptr_;
}
impl MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl for Ptr<MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__> {
fn begin_11 ( &self , ) -> three_way_random_access_iterator_int_ptr_  { return three_way_random_access_iterator_int_ptr_ :: new_1 ( { unimplemented!()   } , )    ;
 } fn begin_12 ( &self , ) -> three_way_random_access_iterator_const_int_ptr_  { return three_way_random_access_iterator_const_int_ptr_ :: new_3 ( { unimplemented!()   } , )    ;
 } fn cbegin ( &self , ) -> three_way_random_access_iterator_const_int_ptr_  { return three_way_random_access_iterator_const_int_ptr_ :: new_3 ( { unimplemented!()   } , )    ;
 } fn end_14 ( &self , ) -> three_way_random_access_iterator_int_ptr_  { return ( { let _x: three_way_random_access_iterator_int_ptr_   = ( { MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl :: begin_11 ( self , ) } )   ; let _n: ()   = ( { MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl :: size ( self , ) } )   ; operator_add_1 ( _x , _n , ) } )    ;
 } fn end_15 ( &self , ) -> three_way_random_access_iterator_const_int_ptr_  { return ( { let _x: three_way_random_access_iterator_const_int_ptr_   = ( { MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl :: begin_12 ( self , ) } )   ; let _n: ()   = ( { MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl :: size ( self , ) } )   ; operator_add_7 ( _x , _n , ) } )    ;
 } fn size ( &self , ) -> ()  { return unimplemented!()    ;
 } fn empty ( &self , ) -> ()  { return unimplemented!()    ;
 } fn clear ( &self , ) { unimplemented!()  ;
 } fn erase_25 ( &self , mut first : three_way_random_access_iterator_const_int_ptr_  , mut last : three_way_random_access_iterator_const_int_ptr_  , ) -> three_way_random_access_iterator_int_ptr_  { return ( { let _it: ()   = unimplemented!()   ; MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl :: from_vector_iterator ( self , _it , ) } )    ;
 } fn to_vector_iterator ( &self , mut cit : three_way_random_access_iterator_const_int_ptr_  , ) -> ()  { return unimplemented!()    ;
 } fn from_vector_iterator ( &self , it : ()  , ) -> three_way_random_access_iterator_int_ptr_  { let it : Value<() >  = Rc::new(RefCell::new(it)) ;
 return ( { let _n: ()   = unimplemented!()   ; let _x: three_way_random_access_iterator_int_ptr_   = ( { MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl :: begin_11 ( self , ) } )   ; operator_add_2 ( _n , _x , ) } )    ;
 } fn move_assign ( &self , _a0 : Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  > , ) -> Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  > { unimplemented!()  ;
 return ((* self )  ).clone()  ;
 } fn insert_20_class_cpp17_input_iterator_const_int_ptr_ ( &self , mut p : three_way_random_access_iterator_const_int_ptr_  , mut first : cpp17_input_iterator_const_int_ptr__const_int_ptr_  , mut last : cpp17_input_iterator_const_int_ptr__const_int_ptr_  , ) -> three_way_random_access_iterator_int_ptr_  { return ( { let _it: ()   = unimplemented!()   ; MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl :: from_vector_iterator ( self , _it , ) } )    ;
 } fn insert_20_const_int_ptr ( &self , mut p : three_way_random_access_iterator_const_int_ptr_  , mut first : Ptr< ()  >  , mut last : Ptr< ()  >  , ) -> three_way_random_access_iterator_int_ptr_  { return ( { let _it: ()   = unimplemented!()   ; MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl :: from_vector_iterator ( self , _it , ) } )    ;
 } fn insert_20_int_ptr ( &self , mut p : three_way_random_access_iterator_const_int_ptr_  , mut first : Ptr< ()  >  , mut last : Ptr< ()  >  , ) -> three_way_random_access_iterator_int_ptr_  { return ( { let _it: ()   = unimplemented!()   ; MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl :: from_vector_iterator ( self , _it , ) } )    ;
 } }
pub trait Tag_XImpl {
    fn copy_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X>;
    fn move_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X>;
}
impl Tag_XImpl for Ptr<Tag_X> {
    fn copy_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X> {
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X> {
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
pub trait cpp17_input_iterator_const_int_ptr__const_int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>;
    fn operator_post_inc_6(
        &self,
        mut _a0: (),
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
    fn operator_deref(&self) -> () {
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
pub trait min_allocator_int_Impl {
    fn allocate(&self, n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_int__std_integral_constant_unsigned_long__0__, n: ());
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {
    fn allocate(&self, mut n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__>
{
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> ();
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_add(&self, n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn operator_sub(&self, n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>
{
    fn to_(&self) -> () {
        return !(((*self).with(|__s| __s.ptr_.clone())).is_null());
    }
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn operator_add(&self, mut n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_int__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_int__std_integral_constant_unsigned_long__0__Impl::operator_add_assign(
                &tmp.as_pointer(),
                n,
            )
        });
        return (*tmp.borrow()).clone();
    }
    fn operator_sub(&self, mut n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_int__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_int__std_integral_constant_unsigned_long__0__Impl::operator_sub_assign(
                &tmp.as_pointer(),
                n,
            )
        });
        return (*tmp.borrow()).clone();
    }
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait other_allocator_int_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!()
    }
    fn select_on_container_copy_construction(&self) -> other_allocator_int_ {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
}
impl other_allocator_int_Impl for Ptr<other_allocator_int_> {}
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
pub trait test_allocator_int_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn allocate(&self, n: (), _a1: Option<()>) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, s: ());
    fn max_size(&self) -> ();
    fn destroy(&self, p: Ptr<()>);
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
    fn construct_16_const_int_ref(&self, p: Ptr<()>, val: Ptr<()>);
    fn construct_16_int(&self, p: Ptr<()>, val: Ptr<()>);
    fn construct_16_int_ref(&self, p: Ptr<()>, val: Ptr<()>);
}
impl test_allocator_int_Impl for Ptr<test_allocator_int_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_28.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_28.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: (), _a1: Option<()>) -> Ptr<()> {
        let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            if ({
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.time_to_throw)
            } >= {
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.throw_after)
            }) {
                unimplemented!();
            }
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_add(n)
            });
        }
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_dec());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_sub(s)
            });
        }
        unimplemented!();
    }
    fn max_size(&self) -> () {
        return (((2147483647_()).wrapping_mul((2_() as ())) as ()).wrapping_add((1_() as ())))
            .wrapping_div((::std::mem::size_of::<()>() as ()));
    }
    fn destroy(&self, mut p: Ptr<()>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), destroy_count)
                .with_mut(|__v| __v.prefix_inc());
        };
    }
    fn construct_16_const_int_ref(&self, mut p: Ptr<()>, val: Ptr<()>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), construct_count)
                .with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
    }
    fn construct_16_int(&self, mut p: Ptr<()>, val: Ptr<()>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), construct_count)
                .with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
    }
    fn construct_16_int_ref(&self, mut p: Ptr<()>, val: Ptr<()>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), construct_count)
                .with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
    }
}
pub trait test_allocator_int_ptr_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn allocate(&self, n: (), _a1: Option<()>) -> Ptr<Ptr<()>>;
    fn deallocate(&self, p: Ptr<Ptr<()>>, s: ());
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<Ptr<()>>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_int_ptr_Impl for Ptr<test_allocator_int_ptr_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_28.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_28.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: (), _a1: Option<()>) -> Ptr<Ptr<()>> {
        let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            if ({
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.time_to_throw)
            } >= {
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.throw_after)
            }) {
                unimplemented!();
            }
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_add(n)
            });
        }
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<Ptr<()>>, mut s: ()) {
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_dec());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_sub(s)
            });
        }
        unimplemented!();
    }
}
pub trait test_allocator_long_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn allocate(&self, n: (), _a1: Option<()>) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, s: ());
    fn max_size(&self) -> ();
    fn destroy(&self, p: Ptr<()>);
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
    fn construct_16_long(&self, p: Ptr<()>, val: Ptr<()>);
    fn construct_16_int_ref(&self, p: Ptr<()>, val: Ptr<()>);
    fn construct_16_const_long_ref(&self, p: Ptr<()>, val: Ptr<()>);
}
impl test_allocator_long_Impl for Ptr<test_allocator_long_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_28.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_28.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: (), _a1: Option<()>) -> Ptr<()> {
        let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            if ({
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.time_to_throw)
            } >= {
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.throw_after)
            }) {
                unimplemented!();
            }
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_add(n)
            });
        }
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_dec());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_sub(s)
            });
        }
        unimplemented!();
    }
    fn max_size(&self) -> () {
        return (((2147483647_()).wrapping_mul((2_() as ())) as ()).wrapping_add((1_() as ())))
            .wrapping_div((::std::mem::size_of::<()>() as ()));
    }
    fn destroy(&self, mut p: Ptr<()>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), destroy_count)
                .with_mut(|__v| __v.prefix_inc());
        };
    }
    fn construct_16_long(&self, mut p: Ptr<()>, val: Ptr<()>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), construct_count)
                .with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
    }
    fn construct_16_int_ref(&self, mut p: Ptr<()>, val: Ptr<()>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), construct_count)
                .with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
    }
    fn construct_16_const_long_ref(&self, mut p: Ptr<()>, val: Ptr<()>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), construct_count)
                .with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
    }
}
pub trait test_allocator_long_ptr_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn allocate(&self, n: (), _a1: Option<()>) -> Ptr<Ptr<()>>;
    fn deallocate(&self, p: Ptr<Ptr<()>>, s: ());
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<Ptr<()>>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_long_ptr_Impl for Ptr<test_allocator_long_ptr_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_28.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_28.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: (), _a1: Option<()>) -> Ptr<Ptr<()>> {
        let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            if ({
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.time_to_throw)
            } >= {
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.throw_after)
            }) {
                unimplemented!();
            }
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_add(n)
            });
        }
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<Ptr<()>>, mut s: ()) {
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_dec());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_sub(s)
            });
        }
        unimplemented!();
    }
}
pub trait test_allocator_statisticsImpl {
    fn clear(&self);
    fn clear_ctor_counters(&self);
}
impl test_allocator_statisticsImpl for Ptr<test_allocator_statistics> {
    fn clear(&self) {
        unimplemented!();
        field!((*self), count).write(0_());
        field!((*self), time_to_throw).write(0_());
        field!((*self), alloc_count).write(0_());
        field!((*self), allocated_size).write(0_());
        field!((*self), construct_count).write(0_());
        field!((*self), destroy_count).write(0_());
        field!((*self), throw_after).write(2147483647_());
        ({ test_allocator_statisticsImpl::clear_ctor_counters(self) });
    }
    fn clear_ctor_counters(&self) {
        field!((*self), copied).write(0_());
        field!((*self), moved).write(0_());
        field!((*self), converted).write(0_());
    }
}
pub trait test_allocator_void_Impl {
    fn destructor(&self);
    fn get_id(&self) -> ();
    fn get_data(&self) -> ();
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_>;
}
impl test_allocator_void_Impl for Ptr<test_allocator_void_> {
    fn destructor(&self) {
        field!((*self), data_).write(destructed_value_28.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_28.with(|rc| *rc.borrow()));
    }
    fn get_id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
    fn get_data(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_> {
        field!((*self), data_).write({ _a0.with(|__s| __s.data_) });
        field!((*self), id_).write({ _a0.with(|__s| __s.id_) });
        field!((*self), stats_).write({ _a0.with(|__s| __s.stats_.clone()) });
        return (*self).clone();
    }
}
pub trait test_less_int_Impl {
    fn operator_call(&self, a: Ptr<()>, b: Ptr<()>) -> ();
}
impl test_less_int_Impl for Ptr<test_less_int_> {
    fn operator_call(&self, a: Ptr<()>, b: Ptr<()>) -> () {
        return ({ (a.read()) } < { (b.read()) });
    }
}
pub trait three_way_random_access_iterator_const_int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<three_way_random_access_iterator_const_int_ptr_>;
    fn operator_dec_8(&self) -> Ptr<three_way_random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: ()) -> three_way_random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> three_way_random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: ()) -> Ptr<three_way_random_access_iterator_const_int_ptr_>;
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<three_way_random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<three_way_random_access_iterator_const_int_ptr_>,
    ) -> Ptr<three_way_random_access_iterator_const_int_ptr_>;
}
impl three_way_random_access_iterator_const_int_ptr_Impl
    for Ptr<three_way_random_access_iterator_const_int_ptr_>
{
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<three_way_random_access_iterator_const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<three_way_random_access_iterator_const_int_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<three_way_random_access_iterator_const_int_ptr_>,
    ) -> Ptr<three_way_random_access_iterator_const_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait three_way_random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<three_way_random_access_iterator_int_ptr_>;
    fn operator_dec_8(&self) -> Ptr<three_way_random_access_iterator_int_ptr_>;
    fn operator_post_inc_9(&self, mut _a0: ()) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: ()) -> Ptr<three_way_random_access_iterator_int_ptr_>;
    fn operator_sub_assign(&self, n: ()) -> Ptr<three_way_random_access_iterator_int_ptr_>;
    fn move_assign(
        &self,
        _a0: Ptr<three_way_random_access_iterator_int_ptr_>,
    ) -> Ptr<three_way_random_access_iterator_int_ptr_>;
}
impl three_way_random_access_iterator_int_ptr_Impl
    for Ptr<three_way_random_access_iterator_int_ptr_>
{
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<three_way_random_access_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_8(&self) -> Ptr<three_way_random_access_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<three_way_random_access_iterator_int_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<three_way_random_access_iterator_int_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<three_way_random_access_iterator_int_ptr_>,
    ) -> Ptr<three_way_random_access_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_17.with(|_| ());
    let _ = alloc_count_18.with(|_| ());
    let _ = dealloc_count_19.with(|_| ());
    let _ = disable_default_constructor_20.with(|_| ());
    let _ = destructed_value_28.with(|_| ());
    let _ = moved_value_29.with(|_| ());
}
