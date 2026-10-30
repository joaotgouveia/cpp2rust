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
pub fn operator_eq_0(
    x: Ptr<random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__>,
    y: Ptr<random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__>,
) -> () {
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(40)]
pub struct random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__ {
    #[offset(0)]
    #[byte_size(32)]
    it_: (),
    #[offset(32)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__ {
    pub fn new_1(it: ()) -> Self {
        let it: Value<()> = Rc::new(RefCell::new(it));
        Self {
            it_: (*it.borrow()).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(
        _a0: Ptr<random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__>,
    ) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppEq for random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_add_1(
    x: random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__,
    mut n: (),
) -> random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__ {
    let x: Value<random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__> =
        Rc::new(RefCell::new(x));
    ({
        random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__Impl :: operator_add_assign ( &x .as_pointer()  , n  , )
    });
    return random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__ :: move_from ( { x .as_pointer()  } , )    ;
}
pub fn operator_sub_2(
    mut x: random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__,
    mut y: random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__,
) -> () {
    return unimplemented!();
}
pub fn operator_ne_3(
    x: Ptr<random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__>,
    y: Ptr<random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__>,
) -> () {
    return !(unimplemented!());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(40)]
pub struct random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__ {
    #[offset(0)]
    #[byte_size(32)]
    it_: (),
    #[offset(32)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__ {
    pub fn new_1(it: ()) -> Self {
        let it: Value<()> = Rc::new(RefCell::new(it));
        Self {
            it_: (*it.borrow()).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(
        _a0: Ptr<random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__>,
    ) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
pub fn operator_eq_4(
    x : Ptr< random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  >,
    y : Ptr< random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  >,
) -> () {
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(40)]
pub struct random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____
{
    #[offset(0)]
    #[byte_size(32)]
    it_: (),
    #[offset(32)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____ { pub fn new_1 ( it : ()  , ) -> Self { let it : Value<() >  = Rc::new(RefCell::new(it)) ;
 Self { it_ : ((*it.borrow()) ).clone()  , tracker_ : support_double_move_tracker :: new ( )   , }  } pub fn move_from ( _a0 : Ptr< random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  > , ) -> Self { Self { it_ : { (*_a0.upgrade().deref()) . it_ .clone() }   , tracker_ : support_double_move_tracker :: move_from ( { field_ptr!(_a0 , tracker_)  } , )   , }  } }
impl CppEq for random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____ { fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool { operator_eq_4(__lhs.clone(), __rhs.clone()) } }
pub fn operator_ne_5(
    x : Ptr< random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  >,
    y : Ptr< random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  >,
) -> () {
    return !(unimplemented!());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(40)]
pub struct random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____
{
    #[offset(0)]
    #[byte_size(32)]
    it_: (),
    #[offset(32)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____ { pub fn new_1 ( it : ()  , ) -> Self { let it : Value<() >  = Rc::new(RefCell::new(it)) ;
 Self { it_ : ((*it.borrow()) ).clone()  , tracker_ : support_double_move_tracker :: new ( )   , }  } pub fn move_from ( _a0 : Ptr< random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  > , ) -> Self { Self { it_ : { (*_a0.upgrade().deref()) . it_ .clone() }   , tracker_ : support_double_move_tracker :: move_from ( { field_ptr!(_a0 , tracker_)  } , )   , }  } }
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
impl CppEq for forward_sized_iterator_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_6(__lhs.clone(), __rhs.clone())
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
impl CppLess for adl_Iterator {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_12(__lhs.clone(), __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for adl_Iterator {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_11(__lhs.clone(), __rhs.clone())
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
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_14.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_15.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_16.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_13.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_14.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_15.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_13: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_14: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_15: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_16: Value<()> = Rc::new(RefCell::new(false));
);
pub fn operator_sub_17(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_eq_18(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_19(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return !({
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_18(_x, y)
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
impl CppEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_18(__lhs.read(), __rhs.read())
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
pub fn operator_sub_20(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_lt_21(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } < { (y.ptr_).clone() });
}
pub fn operator_eq_22(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_23(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return !({
        let _x: min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__   = x   ;
        operator_eq_22(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>,
}
impl min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn new_1 ( mut p : Ptr< min_pointer_int__std_integral_constant_unsigned_long__0__  >  , ) -> Self { Self { ptr_ : (p ).clone() , }  } }
impl CppLess for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool { operator_lt_21(__lhs.read(), __rhs.read()) } }
impl CppEq for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool { operator_eq_22(__lhs.read(), __rhs.read()) } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub fn new_1(mut _a0: min_allocator_int_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
pub fn make_24(mut size: (), start: Option<()>) -> () {
    let mut start: () = start.unwrap_or_else(|| 0_());
    let mut b: () = (4096_()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut init: () = 0_();
    if (start > 0_()) {
        init = (((start + 1_()) / b) + (((start + 1_()) % b) != 0_()));
        init *= b;
        init.prefix_dec();
    }
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < (init - start)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < size) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < start) {
        unimplemented!();
        i.prefix_inc();
    }
    return unimplemented!();
}
pub fn make_25(mut size: (), start: Option<()>) -> () {
    let mut start: () = start.unwrap_or_else(|| 0_());
    let mut b: () = (4096_()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut init: () = 0_();
    if (start > 0_()) {
        init = (((start + 1_()) / b) + (((start + 1_()) % b) != 0_()));
        init *= b;
        init.prefix_dec();
    }
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < (init - start)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < size) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < start) {
        unimplemented!();
        i.prefix_inc();
    }
    return unimplemented!();
}
pub fn testN_26(mut start: (), mut N: ()) {
    let c1: Value<()> = Rc::new(RefCell::new(({ make_24(N, Some(start)) })));
    let c2: Value<()> = Rc::new(RefCell::new(({ make_24(N, None) })));
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn testN_27(mut start: (), mut N: ()) {
    let c1: Value<()> = Rc::new(RefCell::new(({ make_25(N, Some(start)) })));
    let c2: Value<()> = Rc::new(RefCell::new(({ make_25(N, None) })));
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
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
    {
        let mut rng: [(); 10] = [
            0_(),
            1_(),
            2_(),
            3_(),
            1023_(),
            1024_(),
            1025_(),
            2047_(),
            2048_(),
            2049_(),
        ];
        let mut N: () = (::std::mem::size_of::<[(); 10]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let mut i: () = 0_();
        'loop_: while (i < N) {
            let mut j: () = 0_();
            'loop_: while (j < N) {
                ({
                    let _start: () = rng[(i) as usize];
                    let _N: () = rng[(j) as usize];
                    testN_26(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut rng: [(); 10] = [
            0_(),
            1_(),
            2_(),
            3_(),
            1023_(),
            1024_(),
            1025_(),
            2047_(),
            2048_(),
            2049_(),
        ];
        let mut N: () = (::std::mem::size_of::<[(); 10]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let mut i: () = 0_();
        'loop_: while (i < N) {
            let mut j: () = 0_();
            'loop_: while (j < N) {
                ({
                    let _start: () = rng[(i) as usize];
                    let _N: () = rng[(j) as usize];
                    testN_27(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    return 0_();
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
pub trait min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___Impl {
    fn allocate ( &self , n : ()  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn deallocate(
        &self,
        p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        n: (),
    );
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___Impl
    for Ptr<min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___>
{
    fn allocate ( &self , mut n : ()  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()   } , )    ;
    }
    fn deallocate(
        &self,
        mut p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
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
}
pub trait min_pointer_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
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
pub trait min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl
{
    fn to_(&self) -> ();
    fn operator_deref(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_arrow(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_post_inc_9 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_post_dec_11 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_add_assign ( &self , n : ()  , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_sub_assign ( &self , n : ()  , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_add ( &self , n : ()  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn operator_sub ( &self , n : ()  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn operator_index(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
}
impl min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__> {
fn to_ ( &self , ) -> ()  { return ! ( ( (* self ) .with(|__s| __s . ptr_ .clone()) ).is_null() )   ;
 } fn operator_deref ( &self , ) -> Ptr< min_pointer_int__std_integral_constant_unsigned_long__0__  > { return (* self ) .with(|__s| __s . ptr_ .clone())    ;
 } fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { field!((* self ) , ptr_) .with_mut(|__v| __v. prefix_inc ()) ;
 return ((* self )  ).clone()  ;
 } fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { field!((* self ) , ptr_) .with_mut(|__v| __v. prefix_dec ()) ;
 return ((* self )  ).clone()  ;
 } fn operator_add_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { { let _ptr = field!((* self ) , ptr_); _ptr.write(_ptr.read() + n ) } ;
 return ((* self )  ).clone()  ;
 } fn operator_sub_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { { let _ptr = field!((* self ) , ptr_); _ptr.write(_ptr.read() - n ) } ;
 return ((* self )  ).clone()  ;
 } fn operator_add ( &self , mut n : ()  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  { let tmp : Value<min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ > = Rc::new(RefCell::new((( * (* self ) .upgrade().deref() )  ).clone() )) ;
  ;
 ;
 ( { min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , ) } )  ;
 return ((*tmp.borrow()) ).clone()   ;
 } fn operator_sub ( &self , mut n : ()  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  { let tmp : Value<min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ > = Rc::new(RefCell::new((( * (* self ) .upgrade().deref() )  ).clone() )) ;
  ;
 ;
 ( { min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl :: operator_sub_assign ( &tmp .as_pointer()  , n  , ) } )  ;
 return ((*tmp.borrow()) ).clone()   ;
 } }
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__Impl {
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_7(
        &self,
    ) -> Ptr<random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__>;
    fn operator_dec_8(
        &self,
    ) -> Ptr<random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__ {
        unimplemented!()
    }
    fn operator_post_dec_10(
        &self,
        mut _a0: (),
    ) -> random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: (),
    ) -> Ptr<random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__>;
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__>,
    ) -> Ptr<random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__>;
}
impl random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__Impl
    for Ptr<random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__>
{
    fn operator_deref(&self) -> Ptr<()> {
        return field_ptr!((*self), it_);
    }
    fn operator_inc_7(
        &self,
    ) -> Ptr<random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__> {
        unimplemented!();
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__> {
        unimplemented!();
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__>,
    ) -> Ptr<random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__> {
        unimplemented!();
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____Impl
{
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_7 ( &self , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  > ;
    fn operator_dec_8 ( &self , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  >{
        unimplemented!()
    }    fn operator_post_inc_9 ( &self , mut _a0 : ()  , ) -> random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____{
        unimplemented!()
    }    fn operator_post_dec_10 ( &self , mut _a0 : ()  , ) -> random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : ()  , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : ()  , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  >{
        unimplemented!()
    }
    fn move_assign ( &self , _a0 : Ptr< random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  > , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  > ;
}
impl random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____Impl for Ptr<random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____> {
fn operator_deref ( &self , ) -> Ptr< ()  > { return field_ptr!((* self )  , it_)   ;
 } fn operator_inc_7 ( &self , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  > { unimplemented!()  ;
 return ((* self )  ).clone()  ;
 } fn move_assign ( &self , _a0 : Ptr< random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  > , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  > { unimplemented!()  ;
 ( { let _other: Ptr< support_double_move_tracker  >  = field_ptr!(_a0 , tracker_)  ; support_double_move_trackerImpl :: move_assign ( &field_ptr!((* self )  , tracker_)  , _other , ) } )  ;
 return ((* self )  ).clone()  ;
 } }
pub trait random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__Impl {
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_7(
        &self,
    ) -> Ptr<random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__>;
    fn operator_dec_8(
        &self,
    ) -> Ptr<random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__ {
        unimplemented!()
    }
    fn operator_post_dec_10(
        &self,
        mut _a0: (),
    ) -> random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__>,
    ) -> Ptr<random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__>;
}
impl random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__Impl
    for Ptr<random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__>
{
    fn operator_deref(&self) -> Ptr<()> {
        return field_ptr!((*self), it_);
    }
    fn operator_inc_7(
        &self,
    ) -> Ptr<random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__> {
        unimplemented!();
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__>,
    ) -> Ptr<random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__> {
        unimplemented!();
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____Impl
{
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_7 ( &self , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  > ;
    fn operator_dec_8 ( &self , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  >{
        unimplemented!()
    }    fn operator_post_inc_9 ( &self , mut _a0 : ()  , ) -> random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____{
        unimplemented!()
    }    fn operator_post_dec_10 ( &self , mut _a0 : ()  , ) -> random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : ()  , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : ()  , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  >{
        unimplemented!()
    }
    fn move_assign ( &self , _a0 : Ptr< random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  > , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  > ;
}
impl random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____Impl for Ptr<random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____> {
fn operator_deref ( &self , ) -> Ptr< ()  > { return field_ptr!((* self )  , it_)   ;
 } fn operator_inc_7 ( &self , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  > { unimplemented!()  ;
 return ((* self )  ).clone()  ;
 } fn move_assign ( &self , _a0 : Ptr< random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  > , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  > { unimplemented!()  ;
 ( { let _other: Ptr< support_double_move_tracker  >  = field_ptr!(_a0 , tracker_)  ; support_double_move_trackerImpl :: move_assign ( &field_ptr!((* self )  , tracker_)  , _other , ) } )  ;
 return ((* self )  ).clone()  ;
 } }
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
    let _ = outstanding_bytes_13.with(|_| ());
    let _ = alloc_count_14.with(|_| ());
    let _ = dealloc_count_15.with(|_| ());
    let _ = disable_default_constructor_16.with(|_| ());
}
