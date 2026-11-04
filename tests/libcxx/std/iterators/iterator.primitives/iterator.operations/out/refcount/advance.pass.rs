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
pub fn operator_eq_0(
    x: Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_>,
    y: Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i8>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
    pub fn new(mut it: Ptr<i8>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppEq for cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
impl CppAdvance for cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_5();
    }
}
impl CppMove for cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub fn operator_eq_1(
    x: Ptr<forward_iterator_const_char_ptr_>,
    y: Ptr<forward_iterator_const_char_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct forward_iterator_const_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i8>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_char_ptr_ {
    pub fn new_1(mut it: Ptr<i8>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<forward_iterator_const_char_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppEq for forward_iterator_const_char_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
impl CppAdvance for forward_iterator_const_char_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
}
impl CppMove for forward_iterator_const_char_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_eq_2(
    x: Ptr<bidirectional_iterator_const_char_ptr_>,
    y: Ptr<bidirectional_iterator_const_char_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct bidirectional_iterator_const_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i8>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_const_char_ptr_ {
    pub fn new_1(mut it: Ptr<i8>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<bidirectional_iterator_const_char_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppEq for bidirectional_iterator_const_char_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_2(__lhs.clone(), __rhs.clone())
    }
}
impl CppAdvance for bidirectional_iterator_const_char_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_7();
    }
}
impl CppMove for bidirectional_iterator_const_char_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_eq_3(
    x: Ptr<random_access_iterator_const_char_ptr_>,
    y: Ptr<random_access_iterator_const_char_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct random_access_iterator_const_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i8>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_const_char_ptr_ {
    pub fn new_1(mut it: Ptr<i8>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<random_access_iterator_const_char_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppEq for random_access_iterator_const_char_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_3(__lhs.clone(), __rhs.clone())
    }
}
impl CppAdvance for random_access_iterator_const_char_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_7();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_8();
    }
}
impl CppMove for random_access_iterator_const_char_ptr_ {
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
        a.iter_swaps_.with_mut(|__v| {
            __v.prefix_inc();
        });
    }
}
pub fn iter_move_8(iter: adl_Iterator) -> Ptr<i32> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| {
            __v.prefix_inc();
        });
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
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_cmp_10(__lhs.clone(), __rhs.clone())
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
                operator_eq_9(__lhs.clone(), __rhs.clone())
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
pub fn check_advance_11(
    it: cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    mut n: i64,
    result: cpp17_input_iterator_const_char_ptr__const_char_ptr_,
) {
    let it: Value<cpp17_input_iterator_const_char_ptr__const_char_ptr_> = Rc::new(RefCell::new(it));
    let result: Value<cpp17_input_iterator_const_char_ptr__const_char_ptr_> =
        Rc::new(RefCell::new(result));
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::advance(it, n)), void>::value"
    );;
    ({ advance_12(it.as_pointer(), n) });
    assert!(
        ({
            let _x: Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_> = it.as_pointer();
            operator_eq_0(_x, result.as_pointer())
        })
    );
}
pub fn check_advance_13(
    it: forward_iterator_const_char_ptr_,
    mut n: i64,
    result: forward_iterator_const_char_ptr_,
) {
    let it: Value<forward_iterator_const_char_ptr_> = Rc::new(RefCell::new(it));
    let result: Value<forward_iterator_const_char_ptr_> = Rc::new(RefCell::new(result));
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::advance(it, n)), void>::value"
    );;
    ({ advance_14(it.as_pointer(), n) });
    assert!(
        ({
            let _x: Ptr<forward_iterator_const_char_ptr_> = it.as_pointer();
            operator_eq_1(_x, result.as_pointer())
        })
    );
}
pub fn check_advance_15(
    it: bidirectional_iterator_const_char_ptr_,
    mut n: i64,
    result: bidirectional_iterator_const_char_ptr_,
) {
    let it: Value<bidirectional_iterator_const_char_ptr_> = Rc::new(RefCell::new(it));
    let result: Value<bidirectional_iterator_const_char_ptr_> = Rc::new(RefCell::new(result));
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::advance(it, n)), void>::value"
    );;
    ({ advance_16(it.as_pointer(), n) });
    assert!(
        ({
            let _x: Ptr<bidirectional_iterator_const_char_ptr_> = it.as_pointer();
            operator_eq_2(_x, result.as_pointer())
        })
    );
}
pub fn check_advance_17(
    it: random_access_iterator_const_char_ptr_,
    mut n: i64,
    result: random_access_iterator_const_char_ptr_,
) {
    let it: Value<random_access_iterator_const_char_ptr_> = Rc::new(RefCell::new(it));
    let result: Value<random_access_iterator_const_char_ptr_> = Rc::new(RefCell::new(result));
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::advance(it, n)), void>::value"
    );;
    ({ advance_18(it.as_pointer(), n) });
    assert!(
        ({
            let _x: Ptr<random_access_iterator_const_char_ptr_> = it.as_pointer();
            operator_eq_3(_x, result.as_pointer())
        })
    );
}
pub fn check_advance_19(it: Ptr<i8>, mut n: i64, mut result: Ptr<i8>) {
    let it: Value<Ptr<i8>> = Rc::new(RefCell::new(it));
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::advance(it, n)), void>::value"
    );;
    ({ advance_20(it.as_pointer(), n) });
    assert!(({ ({ (*it.borrow()).clone() } == { (result).clone() }) }));
}
pub fn check_advance_21(
    it: cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    mut n: i32,
    result: cpp17_input_iterator_const_char_ptr__const_char_ptr_,
) {
    let it: Value<cpp17_input_iterator_const_char_ptr__const_char_ptr_> = Rc::new(RefCell::new(it));
    let result: Value<cpp17_input_iterator_const_char_ptr__const_char_ptr_> =
        Rc::new(RefCell::new(result));
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::advance(it, n)), void>::value"
    );;
    ({ advance_22(it.as_pointer(), n) });
    assert!(
        ({
            let _x: Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_> = it.as_pointer();
            operator_eq_0(_x, result.as_pointer())
        })
    );
}
pub fn check_advance_23(
    it: forward_iterator_const_char_ptr_,
    mut n: i32,
    result: forward_iterator_const_char_ptr_,
) {
    let it: Value<forward_iterator_const_char_ptr_> = Rc::new(RefCell::new(it));
    let result: Value<forward_iterator_const_char_ptr_> = Rc::new(RefCell::new(result));
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::advance(it, n)), void>::value"
    );;
    ({ advance_24(it.as_pointer(), n) });
    assert!(
        ({
            let _x: Ptr<forward_iterator_const_char_ptr_> = it.as_pointer();
            operator_eq_1(_x, result.as_pointer())
        })
    );
}
pub fn check_advance_25(
    it: bidirectional_iterator_const_char_ptr_,
    mut n: i32,
    result: bidirectional_iterator_const_char_ptr_,
) {
    let it: Value<bidirectional_iterator_const_char_ptr_> = Rc::new(RefCell::new(it));
    let result: Value<bidirectional_iterator_const_char_ptr_> = Rc::new(RefCell::new(result));
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::advance(it, n)), void>::value"
    );;
    ({ advance_26(it.as_pointer(), n) });
    assert!(
        ({
            let _x: Ptr<bidirectional_iterator_const_char_ptr_> = it.as_pointer();
            operator_eq_2(_x, result.as_pointer())
        })
    );
}
pub fn check_advance_27(
    it: random_access_iterator_const_char_ptr_,
    mut n: i32,
    result: random_access_iterator_const_char_ptr_,
) {
    let it: Value<random_access_iterator_const_char_ptr_> = Rc::new(RefCell::new(it));
    let result: Value<random_access_iterator_const_char_ptr_> = Rc::new(RefCell::new(result));
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::advance(it, n)), void>::value"
    );;
    ({ advance_28(it.as_pointer(), n) });
    assert!(
        ({
            let _x: Ptr<random_access_iterator_const_char_ptr_> = it.as_pointer();
            operator_eq_3(_x, result.as_pointer())
        })
    );
}
pub fn check_advance_29(
    it: cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    mut n: u64,
    result: cpp17_input_iterator_const_char_ptr__const_char_ptr_,
) {
    let it: Value<cpp17_input_iterator_const_char_ptr__const_char_ptr_> = Rc::new(RefCell::new(it));
    let result: Value<cpp17_input_iterator_const_char_ptr__const_char_ptr_> =
        Rc::new(RefCell::new(result));
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::advance(it, n)), void>::value"
    );;
    ({ advance_30(it.as_pointer(), n) });
    assert!(
        ({
            let _x: Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_> = it.as_pointer();
            operator_eq_0(_x, result.as_pointer())
        })
    );
}
pub fn check_advance_31(
    it: forward_iterator_const_char_ptr_,
    mut n: u64,
    result: forward_iterator_const_char_ptr_,
) {
    let it: Value<forward_iterator_const_char_ptr_> = Rc::new(RefCell::new(it));
    let result: Value<forward_iterator_const_char_ptr_> = Rc::new(RefCell::new(result));
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::advance(it, n)), void>::value"
    );;
    ({ advance_32(it.as_pointer(), n) });
    assert!(
        ({
            let _x: Ptr<forward_iterator_const_char_ptr_> = it.as_pointer();
            operator_eq_1(_x, result.as_pointer())
        })
    );
}
pub fn check_advance_33(
    it: bidirectional_iterator_const_char_ptr_,
    mut n: u64,
    result: bidirectional_iterator_const_char_ptr_,
) {
    let it: Value<bidirectional_iterator_const_char_ptr_> = Rc::new(RefCell::new(it));
    let result: Value<bidirectional_iterator_const_char_ptr_> = Rc::new(RefCell::new(result));
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::advance(it, n)), void>::value"
    );;
    ({ advance_34(it.as_pointer(), n) });
    assert!(
        ({
            let _x: Ptr<bidirectional_iterator_const_char_ptr_> = it.as_pointer();
            operator_eq_2(_x, result.as_pointer())
        })
    );
}
pub fn check_advance_35(
    it: random_access_iterator_const_char_ptr_,
    mut n: u64,
    result: random_access_iterator_const_char_ptr_,
) {
    let it: Value<random_access_iterator_const_char_ptr_> = Rc::new(RefCell::new(it));
    let result: Value<random_access_iterator_const_char_ptr_> = Rc::new(RefCell::new(result));
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::advance(it, n)), void>::value"
    );;
    ({ advance_36(it.as_pointer(), n) });
    assert!(
        ({
            let _x: Ptr<random_access_iterator_const_char_ptr_> = it.as_pointer();
            operator_eq_3(_x, result.as_pointer())
        })
    );
}
pub fn tests_37() -> bool {
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"1234567890");
    {
        ({
            let _it: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _result: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((10) as isize)
                });
            check_advance_11(_it, 10_i64, _result)
        });
        ({
            let _it: forward_iterator_const_char_ptr_ =
                forward_iterator_const_char_ptr_::new_1({ (s).clone() });
            let _result: forward_iterator_const_char_ptr_ =
                forward_iterator_const_char_ptr_::new_1({ s.offset((10) as isize) });
            check_advance_13(_it, 10_i64, _result)
        });
        ({
            let _it: bidirectional_iterator_const_char_ptr_ =
                bidirectional_iterator_const_char_ptr_::new_1({ s.offset((5) as isize) });
            let _result: bidirectional_iterator_const_char_ptr_ =
                bidirectional_iterator_const_char_ptr_::new_1({ s.offset((10) as isize) });
            check_advance_15(_it, 5_i64, _result)
        });
        ({
            let _it: bidirectional_iterator_const_char_ptr_ =
                bidirectional_iterator_const_char_ptr_::new_1({ s.offset((5) as isize) });
            let _result: bidirectional_iterator_const_char_ptr_ =
                bidirectional_iterator_const_char_ptr_::new_1({ (s).clone() });
            check_advance_15(_it, (-5_i32 as i64), _result)
        });
        ({
            let _it: random_access_iterator_const_char_ptr_ =
                random_access_iterator_const_char_ptr_::new_1({ s.offset((5) as isize) });
            let _result: random_access_iterator_const_char_ptr_ =
                random_access_iterator_const_char_ptr_::new_1({ s.offset((10) as isize) });
            check_advance_17(_it, 5_i64, _result)
        });
        ({
            let _it: random_access_iterator_const_char_ptr_ =
                random_access_iterator_const_char_ptr_::new_1({ s.offset((5) as isize) });
            let _result: random_access_iterator_const_char_ptr_ =
                random_access_iterator_const_char_ptr_::new_1({ (s).clone() });
            check_advance_17(_it, (-5_i32 as i64), _result)
        });
        ({
            let _it: Ptr<i8> = s.offset((5) as isize);
            let _result: Ptr<i8> = s.offset((10) as isize);
            check_advance_19(_it, 5_i64, _result)
        });
        ({
            let _it: Ptr<i8> = s.offset((5) as isize);
            let _result: Ptr<i8> = (s).clone();
            check_advance_19(_it, (-5_i32 as i64), _result)
        });
    }
    {
        ({
            let _it: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _result: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((10) as isize)
                });
            check_advance_21(_it, 10, _result)
        });
        ({
            let _it: forward_iterator_const_char_ptr_ =
                forward_iterator_const_char_ptr_::new_1({ (s).clone() });
            let _result: forward_iterator_const_char_ptr_ =
                forward_iterator_const_char_ptr_::new_1({ s.offset((10) as isize) });
            check_advance_23(_it, 10, _result)
        });
        ({
            let _it: bidirectional_iterator_const_char_ptr_ =
                bidirectional_iterator_const_char_ptr_::new_1({ (s).clone() });
            let _result: bidirectional_iterator_const_char_ptr_ =
                bidirectional_iterator_const_char_ptr_::new_1({ s.offset((10) as isize) });
            check_advance_25(_it, 10, _result)
        });
        ({
            let _it: random_access_iterator_const_char_ptr_ =
                random_access_iterator_const_char_ptr_::new_1({ (s).clone() });
            let _result: random_access_iterator_const_char_ptr_ =
                random_access_iterator_const_char_ptr_::new_1({ s.offset((10) as isize) });
            check_advance_27(_it, 10, _result)
        });
    }
    {
        ({
            let _it: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _result: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((10) as isize)
                });
            check_advance_29(_it, 10_u64, _result)
        });
        ({
            let _it: forward_iterator_const_char_ptr_ =
                forward_iterator_const_char_ptr_::new_1({ (s).clone() });
            let _result: forward_iterator_const_char_ptr_ =
                forward_iterator_const_char_ptr_::new_1({ s.offset((10) as isize) });
            check_advance_31(_it, 10_u64, _result)
        });
        ({
            let _it: bidirectional_iterator_const_char_ptr_ =
                bidirectional_iterator_const_char_ptr_::new_1({ (s).clone() });
            let _result: bidirectional_iterator_const_char_ptr_ =
                bidirectional_iterator_const_char_ptr_::new_1({ s.offset((10) as isize) });
            check_advance_33(_it, 10_u64, _result)
        });
        ({
            let _it: random_access_iterator_const_char_ptr_ =
                random_access_iterator_const_char_ptr_::new_1({ (s).clone() });
            let _result: random_access_iterator_const_char_ptr_ =
                random_access_iterator_const_char_ptr_::new_1({ s.offset((10) as isize) });
            check_advance_35(_it, 10_u64, _result)
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
    ({ tests_37() });
    const _: () = assert!(true, "tests()");;
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
pub trait bidirectional_iterator_const_char_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_const_char_ptr_>;
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_const_char_ptr_>;
    fn operator_post_inc_8(&self, mut _a0: i32) -> bidirectional_iterator_const_char_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_9(&self, mut _a0: i32) -> bidirectional_iterator_const_char_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_const_char_ptr_>,
    ) -> Ptr<bidirectional_iterator_const_char_ptr_>;
}
impl bidirectional_iterator_const_char_ptr_Impl for Ptr<bidirectional_iterator_const_char_ptr_> {
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_const_char_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_const_char_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_dec();
        });
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_const_char_ptr_>,
    ) -> Ptr<bidirectional_iterator_const_char_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait cpp17_input_iterator_const_char_ptr__const_char_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_>;
    fn operator_post_inc_6(
        &self,
        mut _a0: i32,
    ) -> cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_>,
    ) -> Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_>;
}
impl cpp17_input_iterator_const_char_ptr__const_char_ptr_Impl
    for Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_>
{
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_>,
    ) -> Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_> {
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
pub trait forward_iterator_const_char_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_char_ptr_>;
    fn operator_post_inc_7(&self, mut _a0: i32) -> forward_iterator_const_char_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<forward_iterator_const_char_ptr_>,
    ) -> Ptr<forward_iterator_const_char_ptr_>;
}
impl forward_iterator_const_char_ptr_Impl for Ptr<forward_iterator_const_char_ptr_> {
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_char_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<forward_iterator_const_char_ptr_>,
    ) -> Ptr<forward_iterator_const_char_ptr_> {
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
pub trait random_access_iterator_const_char_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_char_ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_const_char_ptr_>;
    fn operator_post_inc_9(&self, mut _a0: i32) -> random_access_iterator_const_char_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: i32) -> random_access_iterator_const_char_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: isize) -> Ptr<random_access_iterator_const_char_ptr_>;
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<random_access_iterator_const_char_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_const_char_ptr_>,
    ) -> Ptr<random_access_iterator_const_char_ptr_>;
}
impl random_access_iterator_const_char_ptr_Impl for Ptr<random_access_iterator_const_char_ptr_> {
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_char_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_const_char_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_dec();
        });
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<random_access_iterator_const_char_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_const_char_ptr_>,
    ) -> Ptr<random_access_iterator_const_char_ptr_> {
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
pub fn __cpp2rust_init_globals() {}
