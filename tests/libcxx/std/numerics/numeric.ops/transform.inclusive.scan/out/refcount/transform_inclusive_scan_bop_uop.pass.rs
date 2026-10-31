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
    x: Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>,
    y: Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    pub fn new(mut it: Ptr<i32>) -> Self {
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
impl CppAdvance for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_5();
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub fn operator_ne_1(
    x: Ptr<forward_iterator_const_int_ptr_>,
    y: Ptr<forward_iterator_const_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct forward_iterator_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<forward_iterator_const_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppAdvance for forward_iterator_const_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
}
pub fn operator_ne_2(
    x: Ptr<bidirectional_iterator_const_int_ptr_>,
    y: Ptr<bidirectional_iterator_const_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct bidirectional_iterator_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_const_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<bidirectional_iterator_const_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppAdvance for bidirectional_iterator_const_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
}
pub fn operator_ne_3(
    x: Ptr<random_access_iterator_const_int_ptr_>,
    y: Ptr<random_access_iterator_const_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct random_access_iterator_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_const_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<random_access_iterator_const_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppAdvance for random_access_iterator_const_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_7();
    }
}
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
pub fn operator_eq_4(
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
        operator_eq_4(__lhs.clone(), __rhs.clone())
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
pub fn operator_add_5(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_6(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_5(_i, n)
    });
}
pub fn iter_swap_7(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap!((a.ptr_).clone(), (b.ptr_).clone());
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_8(iter: adl_Iterator) -> Ptr<i32> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_9(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> bool {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_10(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> std::cmp::Ordering {
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
        operator_cmp_10(__lhs.clone(), __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for adl_Iterator {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_9(__lhs.clone(), __rhs.clone())
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
pub struct add_one {}
pub fn test_11(
    mut first: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    bop: (),
    mut uop: add_one,
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_12(
    mut first: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    bop: (),
    mut uop: add_one,
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_13(
    mut first: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    bop: (),
    uop: (),
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    let uop: Value<()> = Rc::new(RefCell::new(uop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_14(
    mut first: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    bop: (),
    uop: (),
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    let uop: Value<()> = Rc::new(RefCell::new(uop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_15(
    mut first: forward_iterator_const_int_ptr_,
    mut last: forward_iterator_const_int_ptr_,
    bop: (),
    mut uop: add_one,
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_16(
    mut first: forward_iterator_const_int_ptr_,
    mut last: forward_iterator_const_int_ptr_,
    bop: (),
    mut uop: add_one,
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_17(
    mut first: forward_iterator_const_int_ptr_,
    mut last: forward_iterator_const_int_ptr_,
    bop: (),
    uop: (),
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    let uop: Value<()> = Rc::new(RefCell::new(uop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_18(
    mut first: forward_iterator_const_int_ptr_,
    mut last: forward_iterator_const_int_ptr_,
    bop: (),
    uop: (),
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    let uop: Value<()> = Rc::new(RefCell::new(uop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_19(
    mut first: bidirectional_iterator_const_int_ptr_,
    mut last: bidirectional_iterator_const_int_ptr_,
    bop: (),
    mut uop: add_one,
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_20(
    mut first: bidirectional_iterator_const_int_ptr_,
    mut last: bidirectional_iterator_const_int_ptr_,
    bop: (),
    mut uop: add_one,
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_21(
    mut first: bidirectional_iterator_const_int_ptr_,
    mut last: bidirectional_iterator_const_int_ptr_,
    bop: (),
    uop: (),
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    let uop: Value<()> = Rc::new(RefCell::new(uop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_22(
    mut first: bidirectional_iterator_const_int_ptr_,
    mut last: bidirectional_iterator_const_int_ptr_,
    bop: (),
    uop: (),
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    let uop: Value<()> = Rc::new(RefCell::new(uop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_23(
    mut first: random_access_iterator_const_int_ptr_,
    mut last: random_access_iterator_const_int_ptr_,
    bop: (),
    mut uop: add_one,
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_24(
    mut first: random_access_iterator_const_int_ptr_,
    mut last: random_access_iterator_const_int_ptr_,
    bop: (),
    mut uop: add_one,
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_25(
    mut first: random_access_iterator_const_int_ptr_,
    mut last: random_access_iterator_const_int_ptr_,
    bop: (),
    uop: (),
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    let uop: Value<()> = Rc::new(RefCell::new(uop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_26(
    mut first: random_access_iterator_const_int_ptr_,
    mut last: random_access_iterator_const_int_ptr_,
    bop: (),
    uop: (),
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    let uop: Value<()> = Rc::new(RefCell::new(uop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_27(
    mut first: Ptr<i32>,
    mut last: Ptr<i32>,
    bop: (),
    mut uop: add_one,
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_28(
    mut first: Ptr<i32>,
    mut last: Ptr<i32>,
    bop: (),
    mut uop: add_one,
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_29(
    mut first: Ptr<i32>,
    mut last: Ptr<i32>,
    bop: (),
    uop: (),
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    let uop: Value<()> = Rc::new(RefCell::new(uop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_30(
    mut first: Ptr<i32>,
    mut last: Ptr<i32>,
    bop: (),
    uop: (),
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    let uop: Value<()> = Rc::new(RefCell::new(uop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_31(
    mut first: Ptr<i32>,
    mut last: Ptr<i32>,
    bop: (),
    mut uop: add_one,
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_32(
    mut first: Ptr<i32>,
    mut last: Ptr<i32>,
    bop: (),
    mut uop: add_one,
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_33(
    mut first: Ptr<i32>,
    mut last: Ptr<i32>,
    bop: (),
    uop: (),
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    let uop: Value<()> = Rc::new(RefCell::new(uop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_34(
    mut first: Ptr<i32>,
    mut last: Ptr<i32>,
    bop: (),
    uop: (),
    mut rFirst: Ptr<i32>,
    mut rLast: Ptr<i32>,
) {
    let bop: Value<()> = Rc::new(RefCell::new(bop));
    let uop: Value<()> = Rc::new(RefCell::new(uop));
    assert!(((((rLast).clone() - (rFirst).clone()) as i64) <= 5_i64));
    let out: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let mut end: Ptr<i32> = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = (first).clone();
        let __last = (last).clone();
        let mut __out = (out.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let __rhs = unimplemented!();
    end = __rhs;
    assert!(unimplemented!());
}
pub fn test_35() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 7, 9])));
    let pResI0: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([2, 6, 12, 20, 30])));
    let mResI0: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([2, 8, 48, 384, 3840])));
    let pResN0: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        -1_i32, -4_i32, -9_i32, -16_i32, -25_i32,
    ])));
    let mResN0: Value<Box<[i32]>> =
        Rc::new(RefCell::new(Box::new([-1_i32, 3, -15_i32, 105, -945_i32])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    const _: () = assert!(true, "sa == sizeof(pResI0) / sizeof(pResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResI0) / sizeof(mResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(pResN0) / sizeof(pResN0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResN0) / sizeof(mResN0[0])");;
    let mut i: u32 = 0_u32;
    'loop_: while (i < sa) {
        ({
            let _first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                });
            let _last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>).offset((i) as isize)
                });
            let _rFirst: Ptr<i32> = (pResI0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (pResI0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_11(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                _rFirst,
                _rLast,
            )
        });
        ({
            let _first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                });
            let _last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>).offset((i) as isize)
                });
            let _rFirst: Ptr<i32> = (mResI0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (mResI0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_12(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                _rFirst,
                _rLast,
            )
        });
        ({
            let _first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                });
            let _last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>).offset((i) as isize)
                });
            let _rFirst: Ptr<i32> = (pResN0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (pResN0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_13(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                _rFirst,
                _rLast,
            )
        });
        ({
            let _first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>)
                });
            let _last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_pointer() as Ptr<i32>).offset((i) as isize)
                });
            let _rFirst: Ptr<i32> = (mResN0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (mResN0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_14(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                _rFirst,
                _rLast,
            )
        });
        i.prefix_inc();
    }
}
pub fn test_36() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 7, 9])));
    let pResI0: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([2, 6, 12, 20, 30])));
    let mResI0: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([2, 8, 48, 384, 3840])));
    let pResN0: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        -1_i32, -4_i32, -9_i32, -16_i32, -25_i32,
    ])));
    let mResN0: Value<Box<[i32]>> =
        Rc::new(RefCell::new(Box::new([-1_i32, 3, -15_i32, 105, -945_i32])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    const _: () = assert!(true, "sa == sizeof(pResI0) / sizeof(pResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResI0) / sizeof(mResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(pResN0) / sizeof(pResN0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResN0) / sizeof(mResN0[0])");;
    let mut i: u32 = 0_u32;
    'loop_: while (i < sa) {
        ({
            let _first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let _last: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((i) as isize)
            });
            let _rFirst: Ptr<i32> = (pResI0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (pResI0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_15(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                _rFirst,
                _rLast,
            )
        });
        ({
            let _first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let _last: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((i) as isize)
            });
            let _rFirst: Ptr<i32> = (mResI0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (mResI0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_16(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                _rFirst,
                _rLast,
            )
        });
        ({
            let _first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let _last: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((i) as isize)
            });
            let _rFirst: Ptr<i32> = (pResN0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (pResN0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_17(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                _rFirst,
                _rLast,
            )
        });
        ({
            let _first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let _last: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((i) as isize)
            });
            let _rFirst: Ptr<i32> = (mResN0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (mResN0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_18(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                _rFirst,
                _rLast,
            )
        });
        i.prefix_inc();
    }
}
pub fn test_37() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 7, 9])));
    let pResI0: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([2, 6, 12, 20, 30])));
    let mResI0: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([2, 8, 48, 384, 3840])));
    let pResN0: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        -1_i32, -4_i32, -9_i32, -16_i32, -25_i32,
    ])));
    let mResN0: Value<Box<[i32]>> =
        Rc::new(RefCell::new(Box::new([-1_i32, 3, -15_i32, 105, -945_i32])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    const _: () = assert!(true, "sa == sizeof(pResI0) / sizeof(pResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResI0) / sizeof(mResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(pResN0) / sizeof(pResN0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResN0) / sizeof(mResN0[0])");;
    let mut i: u32 = 0_u32;
    'loop_: while (i < sa) {
        ({
            let _first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let _last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((i) as isize)
                });
            let _rFirst: Ptr<i32> = (pResI0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (pResI0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_19(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                _rFirst,
                _rLast,
            )
        });
        ({
            let _first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let _last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((i) as isize)
                });
            let _rFirst: Ptr<i32> = (mResI0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (mResI0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_20(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                _rFirst,
                _rLast,
            )
        });
        ({
            let _first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let _last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((i) as isize)
                });
            let _rFirst: Ptr<i32> = (pResN0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (pResN0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_21(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                _rFirst,
                _rLast,
            )
        });
        ({
            let _first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let _last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((i) as isize)
                });
            let _rFirst: Ptr<i32> = (mResN0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (mResN0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_22(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                _rFirst,
                _rLast,
            )
        });
        i.prefix_inc();
    }
}
pub fn test_38() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 7, 9])));
    let pResI0: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([2, 6, 12, 20, 30])));
    let mResI0: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([2, 8, 48, 384, 3840])));
    let pResN0: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        -1_i32, -4_i32, -9_i32, -16_i32, -25_i32,
    ])));
    let mResN0: Value<Box<[i32]>> =
        Rc::new(RefCell::new(Box::new([-1_i32, 3, -15_i32, 105, -945_i32])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    const _: () = assert!(true, "sa == sizeof(pResI0) / sizeof(pResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResI0) / sizeof(mResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(pResN0) / sizeof(pResN0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResN0) / sizeof(mResN0[0])");;
    let mut i: u32 = 0_u32;
    'loop_: while (i < sa) {
        ({
            let _first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let _last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((i) as isize)
                });
            let _rFirst: Ptr<i32> = (pResI0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (pResI0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_23(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                _rFirst,
                _rLast,
            )
        });
        ({
            let _first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let _last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((i) as isize)
                });
            let _rFirst: Ptr<i32> = (mResI0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (mResI0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_24(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                _rFirst,
                _rLast,
            )
        });
        ({
            let _first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let _last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((i) as isize)
                });
            let _rFirst: Ptr<i32> = (pResN0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (pResN0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_25(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                _rFirst,
                _rLast,
            )
        });
        ({
            let _first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let _last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((i) as isize)
                });
            let _rFirst: Ptr<i32> = (mResN0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (mResN0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_26(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                _rFirst,
                _rLast,
            )
        });
        i.prefix_inc();
    }
}
pub fn test_39() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 7, 9])));
    let pResI0: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([2, 6, 12, 20, 30])));
    let mResI0: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([2, 8, 48, 384, 3840])));
    let pResN0: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        -1_i32, -4_i32, -9_i32, -16_i32, -25_i32,
    ])));
    let mResN0: Value<Box<[i32]>> =
        Rc::new(RefCell::new(Box::new([-1_i32, 3, -15_i32, 105, -945_i32])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    const _: () = assert!(true, "sa == sizeof(pResI0) / sizeof(pResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResI0) / sizeof(mResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(pResN0) / sizeof(pResN0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResN0) / sizeof(mResN0[0])");;
    let mut i: u32 = 0_u32;
    'loop_: while (i < sa) {
        ({
            let _first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let _last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((i) as isize);
            let _rFirst: Ptr<i32> = (pResI0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (pResI0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_27(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                _rFirst,
                _rLast,
            )
        });
        ({
            let _first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let _last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((i) as isize);
            let _rFirst: Ptr<i32> = (mResI0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (mResI0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_28(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                _rFirst,
                _rLast,
            )
        });
        ({
            let _first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let _last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((i) as isize);
            let _rFirst: Ptr<i32> = (pResN0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (pResN0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_29(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                _rFirst,
                _rLast,
            )
        });
        ({
            let _first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let _last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((i) as isize);
            let _rFirst: Ptr<i32> = (mResN0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (mResN0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_30(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                _rFirst,
                _rLast,
            )
        });
        i.prefix_inc();
    }
}
pub fn test_40() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 7, 9])));
    let pResI0: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([2, 6, 12, 20, 30])));
    let mResI0: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([2, 8, 48, 384, 3840])));
    let pResN0: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        -1_i32, -4_i32, -9_i32, -16_i32, -25_i32,
    ])));
    let mResN0: Value<Box<[i32]>> =
        Rc::new(RefCell::new(Box::new([-1_i32, 3, -15_i32, 105, -945_i32])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    const _: () = assert!(true, "sa == sizeof(pResI0) / sizeof(pResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResI0) / sizeof(mResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(pResN0) / sizeof(pResN0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResN0) / sizeof(mResN0[0])");;
    let mut i: u32 = 0_u32;
    'loop_: while (i < sa) {
        ({
            let _first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let _last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((i) as isize);
            let _rFirst: Ptr<i32> = (pResI0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (pResI0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_31(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                _rFirst,
                _rLast,
            )
        });
        ({
            let _first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let _last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((i) as isize);
            let _rFirst: Ptr<i32> = (mResI0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (mResI0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_32(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                _rFirst,
                _rLast,
            )
        });
        ({
            let _first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let _last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((i) as isize);
            let _rFirst: Ptr<i32> = (pResN0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (pResN0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_33(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                _rFirst,
                _rLast,
            )
        });
        ({
            let _first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let _last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((i) as isize);
            let _rFirst: Ptr<i32> = (mResN0.as_pointer() as Ptr<i32>);
            let _rLast: Ptr<i32> = (mResN0.as_pointer() as Ptr<i32>).offset((i) as isize);
            test_34(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                _rFirst,
                _rLast,
            )
        });
        i.prefix_inc();
    }
}
pub fn triangle_41(mut n: usize) -> usize {
    return ((n).wrapping_mul(((n).wrapping_add(1_usize)))).wrapping_div(2_usize);
}
pub fn basic_tests_42() {
    {
        let v: Value<Vec<u64>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 10, _>(|_| 0_u64).to_vec(),
        ));
        unimplemented!();
        unimplemented!();
        let mut i: usize = 0_usize;
        'loop_: while (i < (*v.borrow()).len()) {
            assert!(
                (((v.decay().offset(i as isize).read()) as usize)
                    == ((i).wrapping_add(1_usize)).wrapping_mul(4_usize))
            );
            i.prefix_inc();
        }
    }
    {
        let v: Value<Vec<u64>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 10, _>(|_| 0_u64).to_vec(),
        ));
        unimplemented!();
        unimplemented!();
        let mut i: usize = 0_usize;
        'loop_: while (i < (*v.borrow()).len()) {
            assert!(
                (((v.decay().offset(i as isize).read()) as usize)
                    == (({ triangle_41(i,) }).wrapping_add(i)).wrapping_add(1_usize))
            );
            i.prefix_inc();
        }
    }
    {
        let v: Value<Vec<u64>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 10, _>(|_| 0_u64).to_vec(),
        ));
        unimplemented!();
        unimplemented!();
        let mut i: usize = 0_usize;
        'loop_: while (i < (*v.borrow()).len()) {
            assert!(
                (((v.decay().offset(i as isize).read()) as usize)
                    == (({ triangle_41((i).wrapping_add(1_usize),) }).wrapping_add(i))
                        .wrapping_add(1_usize))
            );
            i.prefix_inc();
        }
    }
    {
        let v: Value<Vec<u64>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| 0_u64).to_vec(),
        ));
        let res: Value<Vec<u64>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| 0_u64).to_vec(),
        ));
        unimplemented!();
        assert!(unimplemented!());
    }
}
pub fn test_43() -> bool {
    ({ basic_tests_42() });
    ({ test_35() });
    ({ test_36() });
    ({ test_37() });
    ({ test_38() });
    ({ test_39() });
    ({ test_40() });
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_43() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait add_oneImpl {
    fn operator_call_1_unsigned_long(&self, x: u64) -> u64;
    fn operator_call_1_int(&self, x: i32) -> i32;
}
impl add_oneImpl for Ptr<add_one> {
    fn operator_call_1_unsigned_long(&self, mut x: u64) -> u64 {
        return (x).wrapping_add(1_u64);
    }
    fn operator_call_1_int(&self, mut x: i32) -> i32 {
        return (x + 1);
    }
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
pub trait bidirectional_iterator_const_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_const_int_ptr_>;
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_8(&self, mut _a0: i32) -> bidirectional_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_9(&self, mut _a0: i32) -> bidirectional_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_const_int_ptr_>,
    ) -> Ptr<bidirectional_iterator_const_int_ptr_>;
}
impl bidirectional_iterator_const_int_ptr_Impl for Ptr<bidirectional_iterator_const_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_const_int_ptr_>,
    ) -> Ptr<bidirectional_iterator_const_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait cpp17_input_iterator_const_int_ptr__const_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>;
    fn operator_post_inc_6(
        &self,
        mut _a0: i32,
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
    fn operator_deref(&self) -> Ptr<i32> {
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
pub trait forward_iterator_const_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_int_ptr_>;
    fn operator_post_inc_7(&self, mut _a0: i32) -> forward_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<forward_iterator_const_int_ptr_>,
    ) -> Ptr<forward_iterator_const_int_ptr_>;
}
impl forward_iterator_const_int_ptr_Impl for Ptr<forward_iterator_const_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<forward_iterator_const_int_ptr_>,
    ) -> Ptr<forward_iterator_const_int_ptr_> {
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
pub trait random_access_iterator_const_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_int_ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: i32) -> random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: i32) -> random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_const_int_ptr_>,
    ) -> Ptr<random_access_iterator_const_int_ptr_>;
}
impl random_access_iterator_const_int_ptr_Impl for Ptr<random_access_iterator_const_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_const_int_ptr_>,
    ) -> Ptr<random_access_iterator_const_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
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
pub fn __cpp2rust_init_globals() {}
