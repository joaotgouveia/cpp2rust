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
    it_: Ptr<i32>,
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
    x : Ptr< cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr___std__Deque_iterator_int__const_int_ref__const_int_ptr__  >,
    y : Ptr< cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr___std__Deque_iterator_int__const_int_ref__const_int_ptr__  >,
) -> bool {
    return ((*x.upgrade().deref()).it_ != (*y.upgrade().deref()).it_).clone();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(40)]
pub struct cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr___std__Deque_iterator_int__const_int_ref__const_int_ptr__
{
    #[offset(0)]
    #[byte_size(32)]
    it_: (),
    #[offset(32)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr___std__Deque_iterator_int__const_int_ref__const_int_ptr__ { pub fn new ( it : ()  , ) -> Self { let it : Value<() >  = Rc::new(RefCell::new(it)) ;
 Self { it_ : ((*it.borrow()) ).clone()  , tracker_ : support_double_move_tracker :: new ( )   , }  } pub fn move_from ( _a0 : Ptr< cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr___std__Deque_iterator_int__const_int_ref__const_int_ptr__  > , ) -> Self { Self { it_ : { (*_a0.upgrade().deref()) . it_ .clone() }   , tracker_ : support_double_move_tracker :: move_from ( { field_ptr!(_a0 , tracker_)  } , )   , }  } }
impl CppAdvance for cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr___std__Deque_iterator_int__const_int_ref__const_int_ptr__ { fn cpp_inc(__it: &Ptr<Self>) { __it.operator_inc_5(); } }
pub fn operator_ne_1(
    x : Ptr< cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0_____std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  >,
    y : Ptr< cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0_____std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  >,
) -> bool {
    return ((*x.upgrade().deref()).it_ != (*y.upgrade().deref()).it_).clone();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(40)]
pub struct cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0_____std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____
{
    #[offset(0)]
    #[byte_size(32)]
    it_: (),
    #[offset(32)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0_____std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____ { pub fn new ( it : ()  , ) -> Self { let it : Value<() >  = Rc::new(RefCell::new(it)) ;
 Self { it_ : ((*it.borrow()) ).clone()  , tracker_ : support_double_move_tracker :: new ( )   , }  } pub fn move_from ( _a0 : Ptr< cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0_____std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  > , ) -> Self { Self { it_ : { (*_a0.upgrade().deref()) . it_ .clone() }   , tracker_ : support_double_move_tracker :: move_from ( { field_ptr!(_a0 , tracker_)  } , )   , }  } }
impl CppAdvance for cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0_____std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____ { fn cpp_inc(__it: &Ptr<Self>) { __it.operator_inc_5(); } }
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub fn operator_eq_2(
    x: Ptr<random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__>,
    y: Ptr<random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__>,
) -> bool {
    return (*x.upgrade().deref()).it_ == (*y.upgrade().deref()).it_;
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
        operator_eq_2(__lhs.clone(), __rhs.clone())
    }
}
impl CppAdvance for random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_7();
    }
}
pub fn operator_add_3(
    x: random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__,
    mut n: isize,
) -> random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__ {
    let x: Value<random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__> =
        Rc::new(RefCell::new(x));
    ({
        random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__Impl :: operator_add_assign ( &x .as_pointer()  , n  , )
    });
    return random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__ :: move_from ( { x .as_pointer()  } , )    ;
}
pub fn operator_sub_4(
    mut x: random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__,
    mut y: random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__,
) -> isize {
    return (x.it_.iter_distance(&y.it_.clone().into()) as i64 as isize);
}
pub fn operator_ne_5(
    x: Ptr<random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__>,
    y: Ptr<random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__>,
) -> bool {
    return ((*x.upgrade().deref()).it_ != (*y.upgrade().deref()).it_).clone();
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
impl CppAdvance for random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_7();
    }
}
pub fn operator_eq_6(
    x : Ptr< random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  >,
    y : Ptr< random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  >,
) -> bool {
    return (*x.upgrade().deref()).it_ == (*y.upgrade().deref()).it_;
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
impl CppEq for random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____ { fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool { operator_eq_6(__lhs.clone(), __rhs.clone()) } }
impl CppAdvance for random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____ { fn cpp_inc(__it: &Ptr<Self>) { __it.operator_inc_7(); } }
pub fn operator_ne_7(
    x : Ptr< random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  >,
    y : Ptr< random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  >,
) -> bool {
    return ((*x.upgrade().deref()).it_ != (*y.upgrade().deref()).it_).clone();
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
impl CppAdvance for random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____ { fn cpp_inc(__it: &Ptr<Self>) { __it.operator_inc_7(); } }
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
pub fn operator_eq_8(
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
        operator_eq_8(__lhs.clone(), __rhs.clone())
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
pub fn operator_add_9(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_10(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_9(_i, n)
    });
}
pub fn iter_swap_11(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap!((a.ptr_).clone(), (b.ptr_).clone());
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_12(iter: adl_Iterator) -> Ptr<i32> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_13(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> bool {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_14(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> std::cmp::Ordering {
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
        operator_cmp_14(__lhs.clone(), __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for adl_Iterator {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_13(__lhs.clone(), __rhs.clone())
    }
}
impl CppAdvance for adl_Iterator {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_14();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_16();
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Ptr< i8  >  , new, value)]
#[byte_size(16)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub fn new(mut what: Ptr<i8>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!(
            (alloc_count_16.with(|rc| *rc.borrow()) >= dealloc_count_17.with(|rc| *rc.borrow()))
        );
        return ((alloc_count_16.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_17.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_18.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_15.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_16.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_17.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
thread_local!(
    pub static outstanding_bytes_15: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_16: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_17: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_18: Value<bool> = Rc::new(RefCell::new(false));
);
pub fn operator_sub_19(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_eq_20(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_21(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_20(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<i32>,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<i32>) -> Self {
        Self { ptr_: (p).clone() }
    }
}
impl CppEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_20(__lhs.read(), __rhs.read())
    }
}
impl CppAdvance for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_8();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_10();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: AnyPtr,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<i32>,
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
pub fn operator_sub_22(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_lt_23(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } < { (y.ptr_).clone() });
}
pub fn operator_eq_24(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_25(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__   = x   ;
        operator_eq_24(_x, y)
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
impl CppLess for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool { operator_lt_23(__lhs.read(), __rhs.read()) } }
impl CppEq for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool { operator_eq_24(__lhs.read(), __rhs.read()) } }
impl CppAdvance for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn cpp_inc(__it: &Ptr<Self>) { __it.operator_inc_8(); } fn cpp_dec(__it: &Ptr<Self>) { __it.operator_dec_10(); } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub fn new_1(mut _a0: min_allocator_int_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(min_allocator_int_, new_1, value)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
pub fn make_26(mut size: i32, start: Option<i32>) -> Vec<i32> {
    let mut start: i32 = start.unwrap_or_else(|| 0);
    let mut b: i32 = (((4096_usize).wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
    let mut init: i32 = 0;
    if (start > 0) {
        init = (((start + 1) / b) + ((((start + 1) % b) != 0) as i32));
        init *= b;
        init.prefix_dec();
    }
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: i32 = 0;
    'loop_: while (i < (init - start)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < size) {
        (*c.borrow_mut()).push(i);
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < start) {
        (*c.borrow_mut()).remove(0);
        i.prefix_inc();
    }
    return std::mem::take(&mut (*c.borrow_mut()));
}
pub fn make_27(mut size: i32, start: Option<i32>) -> Vec<i32> {
    let mut start: i32 = start.unwrap_or_else(|| 0);
    let mut b: i32 = (((4096_usize).wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
    let mut init: i32 = 0;
    if (start > 0) {
        init = (((start + 1) / b) + ((((start + 1) % b) != 0) as i32));
        init *= b;
        init.prefix_dec();
    }
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: i32 = 0;
    'loop_: while (i < (init - start)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < size) {
        (*c.borrow_mut()).push(i);
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < start) {
        (*c.borrow_mut()).remove(0);
        i.prefix_inc();
    }
    return std::mem::take(&mut (*c.borrow_mut()));
}
pub fn testN_28(mut start: i32, mut N: i32) {
    let c1: Value<Vec<i32>> = Rc::new(RefCell::new(({ make_26(N, Some(start)) })));
    let c2: Value<Vec<i32>> = Rc::new(RefCell::new(({ make_26(N, None) })));
    assert!(
        {
            let mut __first = unimplemented!();
            let __last = unimplemented!();
            let mut __out = unimplemented!();
            while __first != __last {
                __out.iter_deref().write((__first.iter_deref().read()));
                __first.iter_inc();
                __out.iter_inc();
            }
            __out
        } == unimplemented!()
    );
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(
        {
            let mut __first = unimplemented!();
            let __last = unimplemented!();
            let mut __out = unimplemented!();
            while __first != __last {
                __out.iter_deref().write((__first.iter_deref().read()));
                __first.iter_inc();
                __out.iter_inc();
            }
            __out
        } == unimplemented!()
    );
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(
        ({
            let _x: Value<random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__> =
                Rc::new(RefCell::new({
                    let mut __first = unimplemented!();
                    let __last = unimplemented!();
                    let mut __out =
                        random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__::new_1(
                            { unimplemented!() },
                        );
                    while __first != __last {
                        __out.iter_deref().write((__first.iter_deref().read()));
                        __first.iter_inc();
                        __out.iter_inc();
                    }
                    __out
                }));
            let _y: Value<random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__> =
                Rc::new(RefCell::new(
                    random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__::new_1({
                        unimplemented!()
                    }),
                ));
            operator_eq_2(_x.as_pointer(), _y.as_pointer())
        })
    );
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(
        ({
            let _x: Value<random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__> =
                Rc::new(RefCell::new({
                    let mut __first = unimplemented!();
                    let __last = unimplemented!();
                    let mut __out =
                        random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__::new_1(
                            { unimplemented!() },
                        );
                    while __first != __last {
                        __out.iter_deref().write((__first.iter_deref().read()));
                        __first.iter_inc();
                        __out.iter_inc();
                    }
                    __out
                }));
            let _y: Value<random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__> =
                Rc::new(RefCell::new(
                    random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__::new_1({
                        unimplemented!()
                    }),
                ));
            operator_eq_2(_x.as_pointer(), _y.as_pointer())
        })
    );
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(
        {
            let mut __first = random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__ :: new_1 ( { unimplemented!()   } , )  ;
            let __last = random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__ :: new_1 ( { unimplemented!()   } , )  ;
            let mut __out = unimplemented!();
            while __first != __last {
                __out.iter_deref().write((__first.iter_deref().read()));
                __first.iter_inc();
                __out.iter_inc();
            }
            __out
        } == unimplemented!()
    );
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(
        {
            let mut __first = cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr___std__Deque_iterator_int__const_int_ref__const_int_ptr__ :: new ( { unimplemented!()   } , )  ;
            let __last = cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr___std__Deque_iterator_int__const_int_ref__const_int_ptr__ :: new ( { unimplemented!()   } , )  ;
            let mut __out = unimplemented!();
            while __first != __last {
                __out.iter_deref().write((__first.iter_deref().read()));
                __first.iter_inc();
                __out.iter_inc();
            }
            __out
        } == unimplemented!()
    );
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn testN_29(mut start: i32, mut N: i32) {
    let c1: Value<Vec<i32>> = Rc::new(RefCell::new(({ make_27(N, Some(start)) })));
    let c2: Value<Vec<i32>> = Rc::new(RefCell::new(({ make_27(N, None) })));
    assert!(
        {
            let mut __first = unimplemented!();
            let __last = unimplemented!();
            let mut __out = unimplemented!();
            while __first != __last {
                __out.iter_deref().write((__first.iter_deref().read()));
                __first.iter_inc();
                __out.iter_inc();
            }
            __out
        } == unimplemented!()
    );
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(
        {
            let mut __first = unimplemented!();
            let __last = unimplemented!();
            let mut __out = unimplemented!();
            while __first != __last {
                __out.iter_deref().write((__first.iter_deref().read()));
                __first.iter_inc();
                __out.iter_inc();
            }
            __out
        } == unimplemented!()
    );
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(
        ({
            let _x : Value<random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____ > = Rc::new(RefCell::new({let mut __first = unimplemented!()  ;
    let __last = unimplemented!()  ;
    let mut __out = random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____ :: new_1 ( { unimplemented!()   } , )  ;
    while __first != __last {
        __out.iter_deref().write((__first.iter_deref().read()));
        __first.iter_inc();
        __out.iter_inc();
    }
    __out}  ));
            let _y : Value<random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____ > = Rc::new(RefCell::new(random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____ :: new_1 ( { unimplemented!()   } , )  ));
            operator_eq_6(_x.as_pointer(), _y.as_pointer())
        })
    );
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(
        ({
            let _x : Value<random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____ > = Rc::new(RefCell::new({let mut __first = unimplemented!()  ;
    let __last = unimplemented!()  ;
    let mut __out = random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____ :: new_1 ( { unimplemented!()   } , )  ;
    while __first != __last {
        __out.iter_deref().write((__first.iter_deref().read()));
        __first.iter_inc();
        __out.iter_inc();
    }
    __out}  ));
            let _y : Value<random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____ > = Rc::new(RefCell::new(random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____ :: new_1 ( { unimplemented!()   } , )  ));
            operator_eq_6(_x.as_pointer(), _y.as_pointer())
        })
    );
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(
        {
            let mut __first = random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____ :: new_1 ( { unimplemented!()   } , )  ;
            let __last = random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____ :: new_1 ( { unimplemented!()   } , )  ;
            let mut __out = unimplemented!();
            while __first != __last {
                __out.iter_deref().write((__first.iter_deref().read()));
                __first.iter_inc();
                __out.iter_inc();
            }
            __out
        } == unimplemented!()
    );
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(
        {
            let mut __first = cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0_____std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____ :: new ( { unimplemented!()   } , )  ;
            let __last = cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0_____std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____ :: new ( { unimplemented!()   } , )  ;
            let mut __out = unimplemented!();
            while __first != __last {
                __out.iter_deref().write((__first.iter_deref().read()));
                __first.iter_inc();
                __out.iter_inc();
            }
            __out
        } == unimplemented!()
    );
    assert!(unimplemented!());
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    {
        let mut rng: [i32; 10] = [0, 1, 2, 3, 1023, 1024, 1025, 2047, 2048, 2049];
        let mut N: i32 = (((::std::mem::size_of::<[i32; 10]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            let mut j: i32 = 0;
            'loop_: while (j < N) {
                ({
                    let _start: i32 = rng[(i) as usize];
                    let _N: i32 = rng[(j) as usize];
                    testN_28(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut rng: [i32; 10] = [0, 1, 2, 3, 1023, 1024, 1025, 2047, 2048, 2049];
        let mut N: i32 = (((::std::mem::size_of::<[i32; 10]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            let mut j: i32 = 0;
            'loop_: while (j < N) {
                ({
                    let _start: i32 = rng[(i) as usize];
                    let _N: i32 = rng[(j) as usize];
                    testN_29(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
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
pub trait cpp17_input_iterator_int_ptr__int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: i32) -> cpp17_input_iterator_int_ptr__int_ptr_ {
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
pub trait cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr___std__Deque_iterator_int__const_int_ref__const_int_ptr__Impl
{
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_5 ( &self , ) -> Ptr< cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr___std__Deque_iterator_int__const_int_ref__const_int_ptr__  > ;
    fn operator_post_inc_6 ( &self , mut _a0 : i32  , ) -> cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr___std__Deque_iterator_int__const_int_ref__const_int_ptr__{
        unimplemented!()
    }
    fn move_assign ( &self , _a0 : Ptr< cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr___std__Deque_iterator_int__const_int_ref__const_int_ptr__  > , ) -> Ptr< cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr___std__Deque_iterator_int__const_int_ref__const_int_ptr__  > ;
}
impl cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr___std__Deque_iterator_int__const_int_ref__const_int_ptr__Impl for Ptr<cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr___std__Deque_iterator_int__const_int_ref__const_int_ptr__> {
fn operator_deref ( &self , ) -> Ptr< i32  > { return field_ptr!((* self )  , it_)   ;
 } fn operator_inc_5 ( &self , ) -> Ptr< cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr___std__Deque_iterator_int__const_int_ref__const_int_ptr__  > { field!((* self ) , it_).with_mut(|__v: &mut ()  | __v.iter_inc())  ;
 return ((* self )  ).clone()  ;
 } fn move_assign ( &self , _a0 : Ptr< cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr___std__Deque_iterator_int__const_int_ref__const_int_ptr__  > , ) -> Ptr< cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr___std__Deque_iterator_int__const_int_ref__const_int_ptr__  > { field!((* self ) , it_).with_mut(|__b0| (*__b0) = (*_a0.upgrade().deref()) . it_  .clone())  ;
 ( { let _other: Ptr< support_double_move_tracker  >  = field_ptr!(_a0 , tracker_)  ; support_double_move_trackerImpl :: move_assign ( &field_ptr!((* self )  , tracker_)  , _other , ) } )  ;
 return ((* self )  ).clone()  ;
 } }
pub trait cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0_____std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____Impl
{
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_5 ( &self , ) -> Ptr< cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0_____std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  > ;
    fn operator_post_inc_6 ( &self , mut _a0 : i32  , ) -> cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0_____std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____{
        unimplemented!()
    }
    fn move_assign ( &self , _a0 : Ptr< cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0_____std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  > , ) -> Ptr< cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0_____std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  > ;
}
impl cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0_____std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____Impl for Ptr<cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0_____std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____> {
fn operator_deref ( &self , ) -> Ptr< i32  > { return field_ptr!((* self )  , it_)   ;
 } fn operator_inc_5 ( &self , ) -> Ptr< cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0_____std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  > { field!((* self ) , it_).with_mut(|__v: &mut ()  | __v.iter_inc())  ;
 return ((* self )  ).clone()  ;
 } fn move_assign ( &self , _a0 : Ptr< cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0_____std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  > , ) -> Ptr< cpp17_input_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0_____std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  > { field!((* self ) , it_).with_mut(|__b0| (*__b0) = (*_a0.upgrade().deref()) . it_  .clone())  ;
 ( { let _other: Ptr< support_double_move_tracker  >  = field_ptr!(_a0 , tracker_)  ; support_double_move_trackerImpl :: move_assign ( &field_ptr!((* self )  , tracker_)  , _other , ) } )  ;
 return ((* self )  ).clone()  ;
 } }
pub trait cpp17_output_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: i32) -> cpp17_output_iterator_int_ptr_ {
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
pub trait min_allocator_int_Impl {
    fn allocate(&self, n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_int__std_integral_constant_unsigned_long__0__, n: usize);
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {
    fn allocate(&self, mut n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___Impl {
    fn allocate ( &self , n : usize  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn deallocate(
        &self,
        p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___Impl
    for Ptr<min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___>
{
    fn allocate ( &self , mut n : usize  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()   } , )    ;
    }
    fn deallocate(
        &self,
        mut p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
pub trait min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: i32,
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
        mut _a0: i32,
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_arrow(&self) -> Ptr<i32>;
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_post_dec_11(
        &self,
        mut _a0: i32,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_add(&self, n: isize) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn operator_sub(&self, n: isize) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>
{
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<i32> {
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
        mut n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
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
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
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
    fn to_bool(&self) -> bool;
    fn operator_deref(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_arrow(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_post_inc_9 ( &self , mut _a0 : i32  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_post_dec_11 ( &self , mut _a0 : i32  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_add_assign ( &self , n : isize  , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_sub_assign ( &self , n : isize  , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_add ( &self , n : isize  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn operator_sub ( &self , n : isize  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn operator_index(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
}
impl min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__> {
fn to_bool ( &self , ) -> bool  { return ! ( ( (* self ) .with(|__s| __s . ptr_ .clone()) ).is_null() )   ;
 } fn operator_deref ( &self , ) -> Ptr< min_pointer_int__std_integral_constant_unsigned_long__0__  > { return (* self ) .with(|__s| __s . ptr_ .clone())    ;
 } fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { field!((* self ) , ptr_) .with_mut(|__v| __v. prefix_inc ()) ;
 return ((* self )  ).clone()  ;
 } fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { field!((* self ) , ptr_) .with_mut(|__v| __v. prefix_dec ()) ;
 return ((* self )  ).clone()  ;
 } fn operator_add_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { { let _ptr = field!((* self ) , ptr_); _ptr.write(_ptr.read() + n ) } ;
 return ((* self )  ).clone()  ;
 } fn operator_sub_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { { let _ptr = field!((* self ) , ptr_); _ptr.write(_ptr.read() - n ) } ;
 return ((* self )  ).clone()  ;
 } fn operator_add ( &self , mut n : isize  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  { let tmp : Value<min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ > = Rc::new(RefCell::new((( * (* self ) .upgrade().deref() )  ).clone() )) ;
  ;
 ;
 ( { min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , ) } )  ;
 return ((*tmp.borrow()) ).clone()   ;
 } fn operator_sub ( &self , mut n : isize  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  { let tmp : Value<min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ > = Rc::new(RefCell::new((( * (* self ) .upgrade().deref() )  ).clone() )) ;
  ;
 ;
 ( { min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl :: operator_sub_assign ( &tmp .as_pointer()  , n  , ) } )  ;
 return ((*tmp.borrow()) ).clone()   ;
 } }
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
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
        mut _a0: i32,
    ) -> random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__ {
        unimplemented!()
    }
    fn operator_post_dec_10(
        &self,
        mut _a0: i32,
    ) -> random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: isize,
    ) -> Ptr<random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__>;
    fn operator_sub_assign(
        &self,
        mut n: isize,
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
    fn operator_deref(&self) -> Ptr<i32> {
        return field_ptr!((*self), it_);
    }
    fn operator_inc_7(
        &self,
    ) -> Ptr<random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__> {
        field!((*self), it_).with_mut(|__v: &mut ()| __v.iter_inc());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__> {
        field!((*self), it_).with_mut(|__v: &mut ()| __v.iter_add_assign(n));
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__>,
    ) -> Ptr<random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__> {
        field!((*self), it_).with_mut(|__b0| (*__b0) = (*_a0.upgrade().deref()).it_.clone());
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____Impl
{
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_7 ( &self , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  > ;
    fn operator_dec_8 ( &self , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  >{
        unimplemented!()
    }    fn operator_post_inc_9 ( &self , mut _a0 : i32  , ) -> random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____{
        unimplemented!()
    }    fn operator_post_dec_10 ( &self , mut _a0 : i32  , ) -> random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : isize  , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : isize  , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  >{
        unimplemented!()
    }
    fn move_assign ( &self , _a0 : Ptr< random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  > , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  > ;
}
impl random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____Impl for Ptr<random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____> {
fn operator_deref ( &self , ) -> Ptr< i32  > { return field_ptr!((* self )  , it_)   ;
 } fn operator_inc_7 ( &self , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  > { field!((* self ) , it_).with_mut(|__v: &mut ()  | __v.iter_inc())  ;
 return ((* self )  ).clone()  ;
 } fn move_assign ( &self , _a0 : Ptr< random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  > , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  > { field!((* self ) , it_).with_mut(|__b0| (*__b0) = (*_a0.upgrade().deref()) . it_  .clone())  ;
 ( { let _other: Ptr< support_double_move_tracker  >  = field_ptr!(_a0 , tracker_)  ; support_double_move_trackerImpl :: move_assign ( &field_ptr!((* self )  , tracker_)  , _other , ) } )  ;
 return ((* self )  ).clone()  ;
 } }
pub trait random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
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
        mut _a0: i32,
    ) -> random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__ {
        unimplemented!()
    }
    fn operator_post_dec_10(
        &self,
        mut _a0: i32,
    ) -> random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
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
    fn operator_deref(&self) -> Ptr<i32> {
        return field_ptr!((*self), it_);
    }
    fn operator_inc_7(
        &self,
    ) -> Ptr<random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__> {
        field!((*self), it_).with_mut(|__v: &mut ()| __v.iter_inc());
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__>,
    ) -> Ptr<random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__> {
        field!((*self), it_).with_mut(|__b0| (*__b0) = (*_a0.upgrade().deref()).it_.clone());
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____Impl
{
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_7 ( &self , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  > ;
    fn operator_dec_8 ( &self , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  >{
        unimplemented!()
    }    fn operator_post_inc_9 ( &self , mut _a0 : i32  , ) -> random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____{
        unimplemented!()
    }    fn operator_post_dec_10 ( &self , mut _a0 : i32  , ) -> random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : isize  , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : isize  , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  >{
        unimplemented!()
    }
    fn move_assign ( &self , _a0 : Ptr< random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  > , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  > ;
}
impl random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____Impl for Ptr<random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____> {
fn operator_deref ( &self , ) -> Ptr< i32  > { return field_ptr!((* self )  , it_)   ;
 } fn operator_inc_7 ( &self , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  > { field!((* self ) , it_).with_mut(|__v: &mut ()  | __v.iter_inc())  ;
 return ((* self )  ).clone()  ;
 } fn move_assign ( &self , _a0 : Ptr< random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  > , ) -> Ptr< random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  > { field!((* self ) , it_).with_mut(|__b0| (*__b0) = (*_a0.upgrade().deref()) . it_  .clone())  ;
 ( { let _other: Ptr< support_double_move_tracker  >  = field_ptr!(_a0 , tracker_)  ; support_double_move_trackerImpl :: move_assign ( &field_ptr!((* self )  , tracker_)  , _other , ) } )  ;
 return ((* self )  ).clone()  ;
 } }
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
                return (*cmp.borrow());
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
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_15.with(|_| ());
    let _ = alloc_count_16.with(|_| ());
    let _ = dealloc_count_17.with(|_| ());
    let _ = disable_default_constructor_18.with(|_| ());
}
