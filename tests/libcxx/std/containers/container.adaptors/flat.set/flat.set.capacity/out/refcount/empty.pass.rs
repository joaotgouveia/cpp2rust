extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_bool__bool_ {}
impl CppMove for test_macros_detail_is_same_bool__bool_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
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
impl CppMove for support_double_move_tracker {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
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
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
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
impl CppMove for cpp17_output_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
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
impl CppMove for cpp17_input_iterator_int_ptr__int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub fn operator_add_1(
    x: three_way_random_access_iterator_int_ptr_,
    mut n: isize,
) -> three_way_random_access_iterator_int_ptr_ {
    let x: Value<three_way_random_access_iterator_int_ptr_> = Rc::new(RefCell::new(x));
    ({ three_way_random_access_iterator_int_ptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return three_way_random_access_iterator_int_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_add_2(
    mut n: isize,
    x: three_way_random_access_iterator_int_ptr_,
) -> three_way_random_access_iterator_int_ptr_ {
    let x: Value<three_way_random_access_iterator_int_ptr_> = Rc::new(RefCell::new(x));
    ({ three_way_random_access_iterator_int_ptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return three_way_random_access_iterator_int_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_3(
    x: three_way_random_access_iterator_int_ptr_,
    mut n: isize,
) -> three_way_random_access_iterator_int_ptr_ {
    let x: Value<three_way_random_access_iterator_int_ptr_> = Rc::new(RefCell::new(x));
    ({ three_way_random_access_iterator_int_ptr_Impl::operator_sub_assign(&x.as_pointer(), n) });
    return three_way_random_access_iterator_int_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_4(
    mut x: three_way_random_access_iterator_int_ptr_,
    mut y: three_way_random_access_iterator_int_ptr_,
) -> isize {
    return (((x.it_).clone() - (y.it_).clone()) as i64 as isize);
}
pub fn operator_eq_5(
    x: Ptr<three_way_random_access_iterator_int_ptr_>,
    y: Ptr<three_way_random_access_iterator_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_lt_6(
    x: Ptr<three_way_random_access_iterator_int_ptr_>,
    y: Ptr<three_way_random_access_iterator_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } < { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct three_way_random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl three_way_random_access_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
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
impl std::cmp::Ord for three_way_random_access_iterator_int_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                if operator_lt_6(__lhs.clone(), __rhs.clone()) {
                    std::cmp::Ordering::Less
                } else if operator_lt_6(__rhs.clone(), __lhs.clone()) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            })
        }
    }
}
impl std::cmp::PartialOrd for three_way_random_access_iterator_int_ptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for three_way_random_access_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_5(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for three_way_random_access_iterator_int_ptr_ {}
impl CppAdvance for three_way_random_access_iterator_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_7();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_8();
    }
}
impl CppIterator<i32> for three_way_random_access_iterator_int_ptr_ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            three_way_random_access_iterator_int_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            three_way_random_access_iterator_int_ptr_Impl::operator_inc_7(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for three_way_random_access_iterator_int_ptr_ {}
impl CppBidirectionalIterator<i32> for three_way_random_access_iterator_int_ptr_ {
    fn iter_dec(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            three_way_random_access_iterator_int_ptr_Impl::operator_dec_8(__it);
        });
        self
    }
}
impl CppRandomAccessIterator<i32> for three_way_random_access_iterator_int_ptr_ {
    fn iter_add_assign(&mut self, n: isize) -> Self {
        libcc2rs::with_record_ptr(self, |__it| {
            three_way_random_access_iterator_int_ptr_Impl::operator_add_assign(__it, n as _);
        });
        self.clone()
    }
    fn iter_distance(&self, from: &Self) -> isize {
        libcc2rs::with_record_ptrs(self, from, |__lhs, __rhs| {
            (operator_sub_4(__lhs.read(), __rhs.read())) as isize
        })
    }
}
impl CppMove for three_way_random_access_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
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
        field!(u, it_).write(Ptr::<i32>::null());
        __this
    }
}
pub fn operator_sub_7(
    mut x: three_way_random_access_iterator_const_int_ptr_,
    mut y: three_way_random_access_iterator_const_int_ptr_,
) -> isize {
    return (((x.it_).clone() - (y.it_).clone()) as i64 as isize);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CppFrom, MoveAssign, Default)]
#[cpp_from(three_way_random_access_iterator_int_ptr_, new_1, ptr)]
#[cpp_from_move(three_way_random_access_iterator_int_ptr_, new_2, ptr)]
#[byte_size(16)]
pub struct three_way_random_access_iterator_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl three_way_random_access_iterator_const_int_ptr_ {
    pub fn new_3(mut it: Ptr<i32>) -> Self {
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
impl CppMove for three_way_random_access_iterator_const_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
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
impl CppMove for cpp20_random_access_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
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
impl CppMove for cpp20_input_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
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
impl CppMove for cpp20_output_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
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
impl CppMove for IteratorOpCounts {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
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
impl CppMove for forward_sized_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
        a.iter_swaps_.with_mut(|__v| {
            __v.prefix_inc();
        });
    }
}
pub fn iter_move_12(iter: adl_Iterator) -> Ptr<i32> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| {
            __v.prefix_inc();
        });
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
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_cmp_14(__lhs.clone(), __rhs.clone())
            })
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
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_13(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for adl_Iterator {}
impl CppAdvance for adl_Iterator {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_14();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_16();
    }
}
impl CppIterator<i32> for adl_Iterator {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            adl_IteratorImpl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            adl_IteratorImpl::operator_inc_14(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for adl_Iterator {}
impl CppBidirectionalIterator<i32> for adl_Iterator {
    fn iter_dec(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            adl_IteratorImpl::operator_dec_16(__it);
        });
        self
    }
}
impl CppRandomAccessIterator<i32> for adl_Iterator {
    fn iter_add_assign(&mut self, n: isize) -> Self {
        libcc2rs::with_record_ptr(self, |__it| {
            adl_IteratorImpl::operator_add_assign(__it, n as _);
        });
        self.clone()
    }
    fn iter_distance(&self, from: &Self) -> isize {
        libcc2rs::with_record_ptrs(self, from, |__lhs, __rhs| {
            (adl_IteratorImpl::operator_sub_11(__lhs, __rhs.read())) as isize
        })
    }
}
impl CppMove for adl_Iterator {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
impl CppMove for rvalue_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
impl CppMove for util_iterator_error {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__
{
    #[offset(0)]
    #[byte_size(24)]
    data_: Vec<i32>,
}
impl MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__ { pub fn move_from ( _a0 : Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  > , ) -> Self { Self { data_ : field!(_a0, data_).with_mut(std::mem::take)  , }  } }
impl CppMove for MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__ { fn cpp_move(src: Ptr<Self>) -> Self { Self::move_from(src) } }
const _: () = assert!(!(false), "!HasAllocatorType<MinSequenceContainer<int>>");
const _: () = assert!(!(false), "!HasEmplace<MinSequenceContainer<int>>");
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
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
pub fn operator_lt_20(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } < { (y.ptr_).clone() });
}
pub fn operator_eq_21(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_22(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_21(_x, y)
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
impl std::cmp::Ord for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                if operator_lt_20(__lhs.read(), __rhs.read()) {
                    std::cmp::Ordering::Less
                } else if operator_lt_20(__rhs.read(), __lhs.read()) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            })
        }
    }
}
impl std::cmp::PartialOrd for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_21(__lhs.read(), __rhs.read())
            })
        }
    }
}
impl std::cmp::Eq for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
impl CppAdvance for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_8();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_10();
    }
}
impl CppIterator<i32> for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            min_pointer_int__std_integral_constant_unsigned_long__0__Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            min_pointer_int__std_integral_constant_unsigned_long__0__Impl::operator_inc_8(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
impl CppBidirectionalIterator<i32> for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn iter_dec(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            min_pointer_int__std_integral_constant_unsigned_long__0__Impl::operator_dec_10(__it);
        });
        self
    }
}
impl CppRandomAccessIterator<i32> for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn iter_add_assign(&mut self, n: isize) -> Self {
        libcc2rs::with_record_ptr(self, |__it| {
            min_pointer_int__std_integral_constant_unsigned_long__0__Impl::operator_add_assign(
                __it, n as _,
            );
        });
        self.clone()
    }
    fn iter_distance(&self, from: &Self) -> isize {
        libcc2rs::with_record_ptrs(self, from, |__lhs, __rhs| {
            (operator_sub_19(__lhs.read(), __rhs.read())) as isize
        })
    }
}
impl CppMove for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
impl CppMove for min_pointer_void__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_sub_23(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_eq_24(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(
    min_pointer_int__std_integral_constant_unsigned_long__0__,
    new_1,
    value
)]
#[byte_size(8)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<i32>,
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
        operator_eq_24(__lhs.read(), __rhs.read())
    }
}
impl CppMove for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: AnyPtr,
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
impl CppMove for min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppMove for min_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_one_25() {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(unimplemented!());
    assert!(unimplemented!());
    (*m.borrow_mut()) = unimplemented!();
    assert!(!(unimplemented!()));
    unimplemented!();
    assert!(unimplemented!());
}
pub fn test_one_26() {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(unimplemented!());
    assert!(unimplemented!());
    (*m.borrow_mut()) = unimplemented!();
    assert!(!(unimplemented!()));
    unimplemented!();
    assert!(unimplemented!());
}
pub fn test_one_27() {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(unimplemented!());
    assert!(unimplemented!());
    (*m.borrow_mut()) = unimplemented!();
    assert!(!(unimplemented!()));
    unimplemented!();
    assert!(unimplemented!());
}
pub fn test_one_28() {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(unimplemented!());
    assert!(unimplemented!());
    (*m.borrow_mut()) = unimplemented!();
    assert!(!(unimplemented!()));
    unimplemented!();
    assert!(unimplemented!());
}
pub fn test_29() -> bool {
    ({ test_one_25() });
    {
        ({ test_one_26() });
    }
    ({ test_one_27() });
    ({ test_one_28() });
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
    ({ test_29() });
    return 0;
}
pub trait MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl
{
    fn operator_assign_6 ( &self , il : Vec<i32  >  , ) -> Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  >{
        unimplemented!()
    }
    fn assign_8(&self, il: Vec<i32>) {
        unimplemented!()
    }
    fn assign_9(&self, mut n: u32, mut t: i32) {
        unimplemented!()
    }
    fn begin_11(&self) -> three_way_random_access_iterator_int_ptr_;
    fn begin_12(&self) -> three_way_random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn cbegin(&self) -> three_way_random_access_iterator_const_int_ptr_;
    fn end_14(&self) -> three_way_random_access_iterator_int_ptr_;
    fn end_15(&self) -> three_way_random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn cend(&self) -> three_way_random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn size(&self) -> u32;
    fn empty(&self) -> bool;
    fn clear(&self);
    fn insert_21(
        &self,
        mut p: three_way_random_access_iterator_const_int_ptr_,
        mut value: i32,
    ) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn insert_22(
        &self,
        mut p: three_way_random_access_iterator_const_int_ptr_,
        mut n: u32,
        mut value: i32,
    ) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn insert_23(
        &self,
        mut p: three_way_random_access_iterator_const_int_ptr_,
        il: Vec<i32>,
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
    fn to_vector_iterator(
        &self,
        cit: three_way_random_access_iterator_const_int_ptr_,
    ) -> RandomAccessIterator<i32>;
    fn from_vector_iterator(
        &self,
        it: RandomAccessIterator<i32>,
    ) -> three_way_random_access_iterator_int_ptr_;
    fn move_assign ( &self , _a0 : Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  > , ) -> Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  > ;
    fn insert_20_const_int_ptr(
        &self,
        p: three_way_random_access_iterator_const_int_ptr_,
        first: Ptr<i32>,
        last: Ptr<i32>,
    ) -> three_way_random_access_iterator_int_ptr_;
}
impl MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl for Ptr<MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__> {
fn begin_11 ( &self , ) -> three_way_random_access_iterator_int_ptr_  { return three_way_random_access_iterator_int_ptr_ :: new_1 ( { field_ref!((* self )  , data_).decay()    } , )    ;
 } fn cbegin ( &self , ) -> three_way_random_access_iterator_const_int_ptr_  { return three_way_random_access_iterator_const_int_ptr_ :: new_3 ( { field_ref!((* self )  , data_).decay()    } , )    ;
 } fn end_14 ( &self , ) -> three_way_random_access_iterator_int_ptr_  { return ( { let _x: three_way_random_access_iterator_int_ptr_   = ( { MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl :: begin_11 ( self , ) } )   ; let _n: isize   = ( ( { MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl :: size ( self , ) } )  as isize )  ; operator_add_1 ( _x , _n , ) } )    ;
 } fn size ( &self , ) -> u32  { return ( (*(* self ) .upgrade().deref()) . data_  .len()  as u32 )   ;
 } fn empty ( &self , ) -> bool  { return (*(* self ) .upgrade().deref()) . data_  .is_empty()    ;
 } fn clear ( &self , ) { field!((* self ) , data_).with_mut(|__v: &mut Vec<i32  >| __v.clear())  ;
 } fn erase_25 ( &self , mut first : three_way_random_access_iterator_const_int_ptr_  , mut last : three_way_random_access_iterator_const_int_ptr_  , ) -> three_way_random_access_iterator_int_ptr_  { return ( { let _it: RandomAccessIterator<i32  >   = unimplemented!()   ; MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl :: from_vector_iterator ( self , _it , ) } )    ;
 } fn to_vector_iterator ( &self , mut cit : three_way_random_access_iterator_const_int_ptr_  , ) -> RandomAccessIterator<i32  >  { return match ({ ( { let _x: three_way_random_access_iterator_const_int_ptr_   = (cit ).clone()  ; operator_sub_7 ( _x , ( { MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl :: cbegin ( self , ) } )   , ) } )   },) { (mut __tmp_0,) => { field_ref!((* self )  , data_).decay() .begin()  .iter_add(__tmp_0 ) } }    ;
 } fn from_vector_iterator ( &self , it : RandomAccessIterator<i32  >  , ) -> three_way_random_access_iterator_int_ptr_  { let it : Value<RandomAccessIterator<i32  > >  = Rc::new(RefCell::new(it)) ;
 return ( { let _n: isize   = ((*it.borrow()) .iter_distance(&field_ref!((* self )  , data_).decay() .begin()  .into()) as i64 as isize)   ; let _x: three_way_random_access_iterator_int_ptr_   = ( { MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl :: begin_11 ( self , ) } )   ; operator_add_2 ( _n , _x , ) } )    ;
 } fn move_assign ( &self , _a0 : Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  > , ) -> Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  > { field_ref!((* self )  , data_).as_pointer() .write(field!(_a0, data_).with_mut(std::mem::take))  ;
 return ((* self )  ).clone()  ;
 } fn insert_20_const_int_ptr ( &self , mut p : three_way_random_access_iterator_const_int_ptr_  , mut first : Ptr< i32  >  , mut last : Ptr< i32  >  , ) -> three_way_random_access_iterator_int_ptr_  { return ( { let _it: RandomAccessIterator<i32  >   = match ({ ( { MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl :: to_vector_iterator ( self , (p ).clone()  , ) } )   },) { (mut __tmp_1,) => { {let __start = __tmp_1 .index();
    let mut __tmp: Vec<i32  > = Vec::new();
    let mut __it = (first ).clone();
    while __it != last  {
        __tmp.push((__it.iter_deref().read()));
        __it.iter_inc();
    }
    field_ref!((* self )  , data_).as_pointer() .with_mut(|__v: &mut Vec<i32  >| {
        __v.splice(__start..__start, __tmp);
    });
    (__tmp_1 ).clone()} } }   ; MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl :: from_vector_iterator ( self , _it , ) } )    ;
 } }
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
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_post_inc_15(&self, mut _a0: i32) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return prev;
    }
    fn operator_dec_16(&self) -> Ptr<adl_Iterator> {
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_dec();
        });
        return (*self).clone();
    }
    fn operator_post_dec_17(&self, mut _a0: i32) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_dec();
        });
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
pub trait min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool;
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_arrow(&self) -> Ptr<i32> {
        unimplemented!()
    }
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
    fn to_bool(&self) -> bool {
        return !(((*self).with(|__s| __s.ptr_.clone())).is_null());
    }
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_dec();
        });
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
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
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
            if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
                return ({ (*cmp.borrow()) });
            }
        }
        return ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) });
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
pub trait three_way_random_access_iterator_const_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<three_way_random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_dec_8(&self) -> Ptr<three_way_random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: i32) -> three_way_random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(
        &self,
        mut _a0: i32,
    ) -> three_way_random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<three_way_random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
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
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<three_way_random_access_iterator_int_ptr_>;
    fn operator_dec_8(&self) -> Ptr<three_way_random_access_iterator_int_ptr_>;
    fn operator_post_inc_9(&self, mut _a0: i32) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: i32) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: isize) -> Ptr<three_way_random_access_iterator_int_ptr_>;
    fn operator_sub_assign(&self, n: isize) -> Ptr<three_way_random_access_iterator_int_ptr_>;
    fn move_assign(
        &self,
        _a0: Ptr<three_way_random_access_iterator_int_ptr_>,
    ) -> Ptr<three_way_random_access_iterator_int_ptr_>;
}
impl three_way_random_access_iterator_int_ptr_Impl
    for Ptr<three_way_random_access_iterator_int_ptr_>
{
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<three_way_random_access_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_dec_8(&self) -> Ptr<three_way_random_access_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_dec();
        });
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<three_way_random_access_iterator_int_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<three_way_random_access_iterator_int_ptr_> {
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
    let _ = outstanding_bytes_15.with(|_| ());
    let _ = alloc_count_16.with(|_| ());
    let _ = dealloc_count_17.with(|_| ());
    let _ = disable_default_constructor_18.with(|_| ());
}
