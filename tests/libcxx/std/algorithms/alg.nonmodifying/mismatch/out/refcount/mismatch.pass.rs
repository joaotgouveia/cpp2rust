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
pub struct types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__
{}
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
#[byte_size(1)]
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr____types_type_list_bidirectional_iterator_int_ptr___
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr____types_type_list_forward_iterator_int_ptr___
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr____types_type_list_cpp17_input_iterator_int_ptr__int_ptr___
{}
pub fn for_each_0(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__,
    f: Test,
) {
    let f: Value<Test> = Rc::new(RefCell::new(f));
    ({
        let _arg0: i32 = {
            ({ TestImpl::operator_call_1_int_ptr(&f.as_pointer()) });
            0
        };
        let _arg1: i32 = {
            ({ TestImpl::operator_call_1_class_contiguous_iterator_int_ptr_(&f.as_pointer()) });
            0
        };
        let _arg2: i32 = {
            ({ TestImpl::operator_call_1_class_random_access_iterator_int_ptr_(&f.as_pointer()) });
            0
        };
        let _arg3: i32 = {
            ({ TestImpl::operator_call_1_class_bidirectional_iterator_int_ptr_(&f.as_pointer()) });
            0
        };
        let _arg4: i32 = {
            ({ TestImpl::operator_call_1_class_forward_iterator_int_ptr_(&f.as_pointer()) });
            0
        };
        let _arg5: i32 = {
            ({ TestImpl::operator_call_1_class_cpp17_input_iterator_int_ptr_(&f.as_pointer()) });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub fn swallow_1(
    mut _a0: i32,
    mut _a1: i32,
    mut _a2: i32,
    mut _a3: i32,
    mut _a4: i32,
    mut _a5: i32,
) {
}
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
pub fn operator_eq_2(
    x: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>,
    y: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_3(
    x: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>,
    y: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
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
    pub fn new(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppEq for cpp17_input_iterator_int_ptr__int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_2(__lhs.clone(), __rhs.clone())
    }
}
impl CppAdvance for cpp17_input_iterator_int_ptr__int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_5();
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub fn operator_eq_4(x: Ptr<forward_iterator_int_ptr_>, y: Ptr<forward_iterator_int_ptr_>) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_5(x: Ptr<forward_iterator_int_ptr_>, y: Ptr<forward_iterator_int_ptr_>) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct forward_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl forward_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<forward_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppEq for forward_iterator_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_4(__lhs.clone(), __rhs.clone())
    }
}
impl CppAdvance for forward_iterator_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
}
pub fn operator_eq_6(
    x: Ptr<bidirectional_iterator_int_ptr_>,
    y: Ptr<bidirectional_iterator_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_7(
    x: Ptr<bidirectional_iterator_int_ptr_>,
    y: Ptr<bidirectional_iterator_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct bidirectional_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<bidirectional_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppEq for bidirectional_iterator_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_6(__lhs.clone(), __rhs.clone())
    }
}
impl CppAdvance for bidirectional_iterator_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
}
pub fn operator_eq_8(
    x: Ptr<random_access_iterator_int_ptr_>,
    y: Ptr<random_access_iterator_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_9(
    x: Ptr<random_access_iterator_int_ptr_>,
    y: Ptr<random_access_iterator_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
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
impl CppEq for random_access_iterator_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_8(__lhs.clone(), __rhs.clone())
    }
}
impl CppAdvance for random_access_iterator_int_ptr_ {
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
pub fn operator_eq_10(
    x: Ptr<contiguous_iterator_int_ptr_>,
    y: Ptr<contiguous_iterator_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_11(
    x: Ptr<contiguous_iterator_int_ptr_>,
    y: Ptr<contiguous_iterator_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
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
impl CppEq for contiguous_iterator_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_10(__lhs.clone(), __rhs.clone())
    }
}
impl CppAdvance for contiguous_iterator_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_9();
    }
}
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
pub fn operator_eq_12(
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
        operator_eq_12(__lhs.clone(), __rhs.clone())
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
pub fn operator_add_13(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_14(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_13(_i, n)
    });
}
pub fn iter_swap_15(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap!((a.ptr_).clone(), (b.ptr_).clone());
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_16(iter: adl_Iterator) -> Ptr<i32> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_17(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> bool {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_18(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> std::cmp::Ordering {
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
        operator_cmp_18(__lhs.clone(), __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for adl_Iterator {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_17(__lhs.clone(), __rhs.clone())
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
pub fn check_19(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_20(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_21(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_22(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_23(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_24(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_25(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_26(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_27(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_28(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_29(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_30(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_31(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_32(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_33(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_34(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_35(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_36(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_37(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_38(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_39(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_40(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_41(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_42(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_43(lhs: Vec<NonTrivialMod4Comp>, rhs: Vec<NonTrivialMod4Comp>, mut offset: usize) {
    let lhs: Value<Vec<NonTrivialMod4Comp>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<NonTrivialMod4Comp>> = Rc::new(RefCell::new(rhs));
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_44(
    lhs: Vec<TriviallyEqualityComparable>,
    rhs: Vec<TriviallyEqualityComparable>,
    mut offset: usize,
) {
    let lhs: Value<Vec<TriviallyEqualityComparable>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<TriviallyEqualityComparable>> = Rc::new(RefCell::new(rhs));
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_45(lhs: Vec<i8>, rhs: Vec<i8>, mut offset: usize) {
    let lhs: Value<Vec<i8>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i8>> = Rc::new(RefCell::new(rhs));
    if ((*lhs.borrow()).len() == (*rhs.borrow()).len()) {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn check_46(lhs: Vec<i32>, rhs: Vec<i32>, mut offset: usize) {
    let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(lhs));
    let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(rhs));
    if ((*lhs.borrow()).len() == (*rhs.borrow()).len()) {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn operator_eq_47(lhs: Ptr<NonTrivialMod4Comp>, rhs: Ptr<NonTrivialMod4Comp>) -> bool {
    return ({ (lhs.with(|__s| __s.i_) % 4) } == { (rhs.with(|__s| __s.i_) % 4) });
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct NonTrivialMod4Comp {
    #[offset(0)]
    pub i_: i32,
}
impl NonTrivialMod4Comp {
    pub fn new(mut i: i32) -> Self {
        Self { i_: i }
    }
    pub fn move_from(other: Ptr<NonTrivialMod4Comp>) -> Self {
        let __this: NonTrivialMod4Comp = Self {
            i_: other.with(|__s| __s.i_),
        };
        field!(other, i_).write(0);
        __this
    }
}
impl CppEq for NonTrivialMod4Comp {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_47(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_eq_48(
    mut _a0: TriviallyEqualityComparable,
    mut _a1: TriviallyEqualityComparable,
) -> bool {
    return (_a0.i_ == _a1.i_);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct TriviallyEqualityComparable {
    #[offset(0)]
    pub i_: i32,
}
impl TriviallyEqualityComparable {
    pub fn new(mut i: i32) -> Self {
        Self { i_: i }
    }
}
impl CppEq for TriviallyEqualityComparable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_48(__lhs.read(), __rhs.read())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ModTwoComp {}
pub fn test_49() -> bool {
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
        ));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
        ));
        ({ check_19((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 0_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 3, 0, 1, 2, 3])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 3, 0, 1, 2, 3])));
        ({ check_20((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 8_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 2, 0, 1, 2, 3])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 3, 0, 1, 2, 3])));
        ({ check_20((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 3_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 2, 0, 1, 2, 3])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1])));
        ({ check_21((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 2_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 2, 0, 1, 2, 3])));
        ({ check_22((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 2_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 2, 3, 4])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 0, 4, 4])));
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    return true;
}
pub fn test_50() -> bool {
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
        ));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
        ));
        ({ check_23((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 0_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 3, 0, 1, 2, 3])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 3, 0, 1, 2, 3])));
        ({ check_24((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 8_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 2, 0, 1, 2, 3])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 3, 0, 1, 2, 3])));
        ({ check_24((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 3_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 2, 0, 1, 2, 3])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1])));
        ({ check_25((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 2_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 2, 0, 1, 2, 3])));
        ({ check_26((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 2_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 2, 3, 4])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 0, 4, 4])));
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    return true;
}
pub fn test_51() -> bool {
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
        ));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
        ));
        ({ check_27((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 0_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 3, 0, 1, 2, 3])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 3, 0, 1, 2, 3])));
        ({ check_28((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 8_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 2, 0, 1, 2, 3])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 3, 0, 1, 2, 3])));
        ({ check_28((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 3_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 2, 0, 1, 2, 3])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1])));
        ({ check_29((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 2_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 2, 0, 1, 2, 3])));
        ({ check_30((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 2_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 2, 3, 4])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 0, 4, 4])));
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    return true;
}
pub fn test_52() -> bool {
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
        ));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
        ));
        ({ check_31((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 0_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 3, 0, 1, 2, 3])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 3, 0, 1, 2, 3])));
        ({ check_32((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 8_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 2, 0, 1, 2, 3])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 3, 0, 1, 2, 3])));
        ({ check_32((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 3_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 2, 0, 1, 2, 3])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1])));
        ({ check_33((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 2_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 2, 0, 1, 2, 3])));
        ({ check_34((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 2_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 2, 3, 4])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 0, 4, 4])));
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    return true;
}
pub fn test_53() -> bool {
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
        ));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
        ));
        ({ check_35((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 0_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 3, 0, 1, 2, 3])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 3, 0, 1, 2, 3])));
        ({ check_36((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 8_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 2, 0, 1, 2, 3])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 3, 0, 1, 2, 3])));
        ({ check_36((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 3_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 2, 0, 1, 2, 3])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1])));
        ({ check_37((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 2_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 2, 0, 1, 2, 3])));
        ({ check_38((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 2_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 2, 3, 4])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 0, 4, 4])));
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    return true;
}
pub fn test_54() -> bool {
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
        ));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
        ));
        ({ check_39((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 0_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 3, 0, 1, 2, 3])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 3, 0, 1, 2, 3])));
        ({ check_40((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 8_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 2, 0, 1, 2, 3])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 3, 0, 1, 2, 3])));
        ({ check_40((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 3_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 2, 0, 1, 2, 3])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1])));
        ({ check_41((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 2_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 1, 2, 2, 0, 1, 2, 3])));
        ({ check_42((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), 2_usize) });
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 2, 3, 4])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0, 0, 4, 4])));
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Test {}
pub fn test_55() -> bool {
    ({
        for_each_0 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__  >::default()  , <Test  >::default()  , )
    });
    {
        let lhs: Value<Vec<NonTrivialMod4Comp>> = Rc::new(RefCell::new(Vec::from([
            NonTrivialMod4Comp::new({ 1 }),
            NonTrivialMod4Comp::new({ 2 }),
            NonTrivialMod4Comp::new({ 3 }),
            NonTrivialMod4Comp::new({ 4 }),
            NonTrivialMod4Comp::new({ 5 }),
            NonTrivialMod4Comp::new({ 6 }),
            NonTrivialMod4Comp::new({ 7 }),
            NonTrivialMod4Comp::new({ 8 }),
        ])));
        let rhs: Value<Vec<NonTrivialMod4Comp>> = Rc::new(RefCell::new(Vec::from([
            NonTrivialMod4Comp::new({ 1 }),
            NonTrivialMod4Comp::new({ 2 }),
            NonTrivialMod4Comp::new({ 3 }),
            NonTrivialMod4Comp::new({ 4 }),
            NonTrivialMod4Comp::new({ 1 }),
            NonTrivialMod4Comp::new({ 6 }),
            NonTrivialMod4Comp::new({ 7 }),
            NonTrivialMod4Comp::new({ 8 }),
        ])));
        ({
            let _lhs: Vec<NonTrivialMod4Comp> = std::mem::take(&mut (*lhs.borrow_mut()));
            let _rhs: Vec<NonTrivialMod4Comp> = std::mem::take(&mut (*rhs.borrow_mut()));
            check_43(_lhs, _rhs, 8_usize)
        });
    }
    {
        let lhs: Value<Vec<NonTrivialMod4Comp>> = Rc::new(RefCell::new(Vec::from([
            NonTrivialMod4Comp::new({ 1 }),
            NonTrivialMod4Comp::new({ 2 }),
            NonTrivialMod4Comp::new({ 3 }),
            NonTrivialMod4Comp::new({ 4 }),
            NonTrivialMod4Comp::new({ 7 }),
            NonTrivialMod4Comp::new({ 6 }),
            NonTrivialMod4Comp::new({ 7 }),
            NonTrivialMod4Comp::new({ 8 }),
        ])));
        let rhs: Value<Vec<NonTrivialMod4Comp>> = Rc::new(RefCell::new(Vec::from([
            NonTrivialMod4Comp::new({ 1 }),
            NonTrivialMod4Comp::new({ 2 }),
            NonTrivialMod4Comp::new({ 3 }),
            NonTrivialMod4Comp::new({ 4 }),
            NonTrivialMod4Comp::new({ 5 }),
            NonTrivialMod4Comp::new({ 6 }),
            NonTrivialMod4Comp::new({ 7 }),
            NonTrivialMod4Comp::new({ 8 }),
        ])));
        ({
            let _lhs: Vec<NonTrivialMod4Comp> = std::mem::take(&mut (*lhs.borrow_mut()));
            let _rhs: Vec<NonTrivialMod4Comp> = std::mem::take(&mut (*rhs.borrow_mut()));
            check_43(_lhs, _rhs, 4_usize)
        });
    }
    {
        let lhs: Value<Vec<TriviallyEqualityComparable>> = Rc::new(RefCell::new(Vec::from([
            TriviallyEqualityComparable::new({ 1 }),
            TriviallyEqualityComparable::new({ 2 }),
            TriviallyEqualityComparable::new({ 3 }),
            TriviallyEqualityComparable::new({ 4 }),
            TriviallyEqualityComparable::new({ 5 }),
            TriviallyEqualityComparable::new({ 6 }),
            TriviallyEqualityComparable::new({ 7 }),
            TriviallyEqualityComparable::new({ 8 }),
        ])));
        let rhs: Value<Vec<TriviallyEqualityComparable>> = Rc::new(RefCell::new(Vec::from([
            TriviallyEqualityComparable::new({ 1 }),
            TriviallyEqualityComparable::new({ 2 }),
            TriviallyEqualityComparable::new({ 3 }),
            TriviallyEqualityComparable::new({ 4 }),
            TriviallyEqualityComparable::new({ 5 }),
            TriviallyEqualityComparable::new({ 6 }),
            TriviallyEqualityComparable::new({ 7 }),
            TriviallyEqualityComparable::new({ 8 }),
        ])));
        ({
            let _lhs: Vec<TriviallyEqualityComparable> = std::mem::take(&mut (*lhs.borrow_mut()));
            let _rhs: Vec<TriviallyEqualityComparable> = std::mem::take(&mut (*rhs.borrow_mut()));
            check_44(_lhs, _rhs, 8_usize)
        });
    }
    {
        let lhs: Value<Vec<TriviallyEqualityComparable>> = Rc::new(RefCell::new(Vec::from([
            TriviallyEqualityComparable::new({ 1 }),
            TriviallyEqualityComparable::new({ 2 }),
            TriviallyEqualityComparable::new({ 3 }),
            TriviallyEqualityComparable::new({ 4 }),
            TriviallyEqualityComparable::new({ 7 }),
            TriviallyEqualityComparable::new({ 6 }),
            TriviallyEqualityComparable::new({ 7 }),
            TriviallyEqualityComparable::new({ 8 }),
        ])));
        let rhs: Value<Vec<TriviallyEqualityComparable>> = Rc::new(RefCell::new(Vec::from([
            TriviallyEqualityComparable::new({ 1 }),
            TriviallyEqualityComparable::new({ 2 }),
            TriviallyEqualityComparable::new({ 3 }),
            TriviallyEqualityComparable::new({ 4 }),
            TriviallyEqualityComparable::new({ 5 }),
            TriviallyEqualityComparable::new({ 6 }),
            TriviallyEqualityComparable::new({ 7 }),
            TriviallyEqualityComparable::new({ 8 }),
        ])));
        ({
            let _lhs: Vec<TriviallyEqualityComparable> = std::mem::take(&mut (*lhs.borrow_mut()));
            let _rhs: Vec<TriviallyEqualityComparable> = std::mem::take(&mut (*rhs.borrow_mut()));
            check_44(_lhs, _rhs, 4_usize)
        });
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_55() });
    const _: () = assert!(true, "test()");;
    {
        {
            let lhs: Value<Vec<i8>> = Rc::new(RefCell::new(
                (0..(256_usize) as usize).map(|_| 0_i8).collect::<Vec<_>>(),
            ));
            let rhs: Value<Vec<i8>> = Rc::new(RefCell::new(
                (0..(256_usize) as usize).map(|_| 0_i8).collect::<Vec<_>>(),
            ));
            let mut i: usize = 0_usize;
            'loop_: while (i != (*lhs.borrow()).len()) {
                lhs.decay().offset(i as isize).write(1_i8);
                ({ check_45((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), i) });
                lhs.decay().offset(i as isize).write(0_i8);
                rhs.decay().offset(i as isize).write(1_i8);
                ({ check_45((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), i) });
                rhs.decay().offset(i as isize).write(0_i8);
                i.prefix_inc();
            }
        }
        {
            let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(
                (0..(256_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>(),
            ));
            let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(
                (0..(256_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>(),
            ));
            let mut i: usize = 0_usize;
            'loop_: while (i != (*lhs.borrow()).len()) {
                lhs.decay().offset(i as isize).write(1);
                ({ check_46((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), i) });
                lhs.decay().offset(i as isize).write(0);
                rhs.decay().offset(i as isize).write(1);
                ({ check_46((*lhs.borrow()).clone(), (*rhs.borrow()).clone(), i) });
                rhs.decay().offset(i as isize).write(0);
                i.prefix_inc();
            }
        }
    }
    {
        let mut vec_size: usize = 1_usize;
        'loop_: while (vec_size != 256_usize) {
            {
                let lhs: Value<Vec<i8>> = Rc::new(RefCell::new(
                    (0..(vec_size) as usize).map(|_| 0_i8).collect::<Vec<_>>(),
                ));
                let rhs: Value<Vec<i8>> = Rc::new(RefCell::new(
                    (0..(vec_size) as usize).map(|_| 0_i8).collect::<Vec<_>>(),
                ));
                ({
                    let _lhs: Vec<i8> = (*lhs.borrow()).clone();
                    let _offset: usize = (*lhs.borrow()).len();
                    check_45(_lhs, (*rhs.borrow()).clone(), _offset)
                });
                lhs.decay().to_last().write(1_i8);
                ({
                    let _lhs: Vec<i8> = (*lhs.borrow()).clone();
                    let _offset: usize = ((*lhs.borrow()).len()).wrapping_sub(1_usize);
                    check_45(_lhs, (*rhs.borrow()).clone(), _offset)
                });
                lhs.decay().to_last().write(0_i8);
                rhs.decay().to_last().write(1_i8);
                ({
                    let _lhs: Vec<i8> = (*lhs.borrow()).clone();
                    let _offset: usize = ((*lhs.borrow()).len()).wrapping_sub(1_usize);
                    check_45(_lhs, (*rhs.borrow()).clone(), _offset)
                });
                rhs.decay().to_last().write(0_i8);
            }
            {
                let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(
                    (0..(vec_size) as usize).map(|_| 0_i32).collect::<Vec<_>>(),
                ));
                let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(
                    (0..(vec_size) as usize).map(|_| 0_i32).collect::<Vec<_>>(),
                ));
                ({
                    let _lhs: Vec<i32> = (*lhs.borrow()).clone();
                    let _offset: usize = (*lhs.borrow()).len();
                    check_46(_lhs, (*rhs.borrow()).clone(), _offset)
                });
                lhs.decay().to_last().write(1);
                ({
                    let _lhs: Vec<i32> = (*lhs.borrow()).clone();
                    let _offset: usize = ((*lhs.borrow()).len()).wrapping_sub(1_usize);
                    check_46(_lhs, (*rhs.borrow()).clone(), _offset)
                });
                lhs.decay().to_last().write(0);
                rhs.decay().to_last().write(1);
                ({
                    let _lhs: Vec<i32> = (*lhs.borrow()).clone();
                    let _offset: usize = ((*lhs.borrow()).len()).wrapping_sub(1_usize);
                    check_46(_lhs, (*rhs.borrow()).clone(), _offset)
                });
                rhs.decay().to_last().write(0);
            }
            vec_size.prefix_inc();
        }
    }
    return 0;
}
pub trait ModTwoCompImpl {
    fn operator_call(&self, lhs: i32, rhs: i32) -> bool;
}
impl ModTwoCompImpl for Ptr<ModTwoComp> {
    fn operator_call(&self, mut lhs: i32, mut rhs: i32) -> bool {
        return ((lhs % 2) == (rhs % 2));
    }
}
pub trait TestImpl {
    fn operator_call_1_int_ptr(&self);
    fn operator_call_1_class_contiguous_iterator_int_ptr_(&self);
    fn operator_call_1_class_random_access_iterator_int_ptr_(&self);
    fn operator_call_1_class_bidirectional_iterator_int_ptr_(&self);
    fn operator_call_1_class_forward_iterator_int_ptr_(&self);
    fn operator_call_1_class_cpp17_input_iterator_int_ptr_(&self);
}
impl TestImpl for Ptr<Test> {
    fn operator_call_1_int_ptr(&self) {
        ({ test_49() });
    }
    fn operator_call_1_class_contiguous_iterator_int_ptr_(&self) {
        ({ test_50() });
    }
    fn operator_call_1_class_random_access_iterator_int_ptr_(&self) {
        ({ test_51() });
    }
    fn operator_call_1_class_bidirectional_iterator_int_ptr_(&self) {
        ({ test_52() });
    }
    fn operator_call_1_class_forward_iterator_int_ptr_(&self) {
        ({ test_53() });
    }
    fn operator_call_1_class_cpp17_input_iterator_int_ptr_(&self) {
        ({ test_54() });
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
pub trait bidirectional_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_int_ptr_>;
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_8(&self, mut _a0: i32) -> bidirectional_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_9(&self, mut _a0: i32) -> bidirectional_iterator_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_int_ptr_>,
    ) -> Ptr<bidirectional_iterator_int_ptr_>;
}
impl bidirectional_iterator_int_ptr_Impl for Ptr<bidirectional_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_int_ptr_>,
    ) -> Ptr<bidirectional_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait contiguous_iterator_int_ptr_Impl {
    fn base(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_arrow(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_int_ptr_>;
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: i32) -> contiguous_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: i32) -> contiguous_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<contiguous_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<contiguous_iterator_int_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_int_ptr_>,
    ) -> Ptr<contiguous_iterator_int_ptr_>;
}
impl contiguous_iterator_int_ptr_Impl for Ptr<contiguous_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
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
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_>;
    fn operator_post_inc_6(&self, mut _a0: i32) -> cpp17_input_iterator_int_ptr__int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>,
    ) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_>;
}
impl cpp17_input_iterator_int_ptr__int_ptr_Impl for Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
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
pub trait forward_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_6(&self) -> Ptr<forward_iterator_int_ptr_>;
    fn operator_post_inc_7(&self, mut _a0: i32) -> forward_iterator_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(&self, _a0: Ptr<forward_iterator_int_ptr_>) -> Ptr<forward_iterator_int_ptr_>;
}
impl forward_iterator_int_ptr_Impl for Ptr<forward_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<forward_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<forward_iterator_int_ptr_>) -> Ptr<forward_iterator_int_ptr_> {
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
pub trait random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: i32) -> random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: i32) -> random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_int_ptr_>,
    ) -> Ptr<random_access_iterator_int_ptr_>;
}
impl random_access_iterator_int_ptr_Impl for Ptr<random_access_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
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
