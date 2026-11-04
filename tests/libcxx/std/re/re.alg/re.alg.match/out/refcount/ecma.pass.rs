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
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub fn operator_ne_0(
    x: Ptr<forward_iterator_const_char_ptr_>,
    y: Ptr<forward_iterator_const_char_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
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
impl std::cmp::PartialEq for forward_iterator_const_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                !(operator_ne_0(__lhs.clone(), __rhs.clone()))
            })
        }
    }
}
impl std::cmp::Eq for forward_iterator_const_char_ptr_ {}
impl CppAdvance for forward_iterator_const_char_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
}
impl CppIterator<i8> for forward_iterator_const_char_ptr_ {
    fn iter_deref(&self) -> Ptr<i8> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            forward_iterator_const_char_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            forward_iterator_const_char_ptr_Impl::operator_inc_6(__it);
        });
        self
    }
}
impl CppInputIterator<i8> for forward_iterator_const_char_ptr_ {}
impl CppMove for forward_iterator_const_char_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_ne_1(
    x: Ptr<forward_iterator_const_wchar_t_ptr_>,
    y: Ptr<forward_iterator_const_wchar_t_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct forward_iterator_const_wchar_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_wchar_t_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<forward_iterator_const_wchar_t_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for forward_iterator_const_wchar_t_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                !(operator_ne_1(__lhs.clone(), __rhs.clone()))
            })
        }
    }
}
impl std::cmp::Eq for forward_iterator_const_wchar_t_ptr_ {}
impl CppAdvance for forward_iterator_const_wchar_t_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
}
impl CppIterator<i32> for forward_iterator_const_wchar_t_ptr_ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            forward_iterator_const_wchar_t_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            forward_iterator_const_wchar_t_ptr_Impl::operator_inc_6(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for forward_iterator_const_wchar_t_ptr_ {}
impl CppMove for forward_iterator_const_wchar_t_ptr_ {
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
pub fn operator_ne_3(
    x: Ptr<bidirectional_iterator_const_char_ptr_>,
    y: Ptr<bidirectional_iterator_const_char_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
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
    pub fn new_1() -> Self {
        Self {
            it_: Ptr::<i8>::null(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn new_2(mut it: Ptr<i8>) -> Self {
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
impl std::cmp::PartialEq for bidirectional_iterator_const_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_2(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_const_char_ptr_ {}
impl CppAdvance for bidirectional_iterator_const_char_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_7();
    }
}
impl CppIterator<i8> for bidirectional_iterator_const_char_ptr_ {
    fn iter_deref(&self) -> Ptr<i8> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            bidirectional_iterator_const_char_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            bidirectional_iterator_const_char_ptr_Impl::operator_inc_6(__it);
        });
        self
    }
}
impl CppInputIterator<i8> for bidirectional_iterator_const_char_ptr_ {}
impl CppBidirectionalIterator<i8> for bidirectional_iterator_const_char_ptr_ {
    fn iter_dec(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            bidirectional_iterator_const_char_ptr_Impl::operator_dec_7(__it);
        });
        self
    }
}
impl CppMove for bidirectional_iterator_const_char_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for bidirectional_iterator_const_char_ptr_ {
    fn default() -> Self {
        { bidirectional_iterator_const_char_ptr_::new_1() }
    }
}
pub fn operator_eq_4(
    x: Ptr<bidirectional_iterator_const_wchar_t_ptr_>,
    y: Ptr<bidirectional_iterator_const_wchar_t_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_5(
    x: Ptr<bidirectional_iterator_const_wchar_t_ptr_>,
    y: Ptr<bidirectional_iterator_const_wchar_t_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(16)]
pub struct bidirectional_iterator_const_wchar_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_const_wchar_t_ptr_ {
    pub fn new_1() -> Self {
        Self {
            it_: Ptr::<i32>::null(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn new_2(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<bidirectional_iterator_const_wchar_t_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_const_wchar_t_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_4(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_const_wchar_t_ptr_ {}
impl CppAdvance for bidirectional_iterator_const_wchar_t_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_7();
    }
}
impl CppIterator<i32> for bidirectional_iterator_const_wchar_t_ptr_ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            bidirectional_iterator_const_wchar_t_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            bidirectional_iterator_const_wchar_t_ptr_Impl::operator_inc_6(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for bidirectional_iterator_const_wchar_t_ptr_ {}
impl CppBidirectionalIterator<i32> for bidirectional_iterator_const_wchar_t_ptr_ {
    fn iter_dec(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            bidirectional_iterator_const_wchar_t_ptr_Impl::operator_dec_7(__it);
        });
        self
    }
}
impl CppMove for bidirectional_iterator_const_wchar_t_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for bidirectional_iterator_const_wchar_t_ptr_ {
    fn default() -> Self {
        { bidirectional_iterator_const_wchar_t_ptr_::new_1() }
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
pub fn operator_eq_6(
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
        operator_eq_6(__lhs.clone(), __rhs.clone())
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
pub fn operator_add_7(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_8(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_7(_i, n)
    });
}
pub fn iter_swap_9(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap!((a.ptr_).clone(), (b.ptr_).clone());
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| {
            __v.prefix_inc();
        });
    }
}
pub fn iter_move_10(iter: adl_Iterator) -> Ptr<i32> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| {
            __v.prefix_inc();
        });
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_11(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> bool {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_12(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> std::cmp::Ordering {
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
                operator_cmp_12(__lhs.clone(), __rhs.clone())
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
                operator_eq_11(__lhs.clone(), __rhs.clone())
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
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"a\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"a") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !({ (*m.borrow()).empty() }) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>).offset((1) as isize) })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 1_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_0,) => {
                    Ptr::<i8>::from_string_literal(b"a").with_c_str(|__s| {
                        (*__tmp_0.upgrade().deref())
                            [..(*__tmp_0.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"ab\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"ab") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 2_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_1,) => {
                    Ptr::<i8>::from_string_literal(b"ab").with_c_str(|__s| {
                        (*__tmp_1.upgrade().deref())
                            [..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"ab\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"ba") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
        assert!(({ ({ (*m.borrow()).empty() }) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"aab\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"ab") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"aab\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"ab") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    Some(match_continuous_14.with(|rc| *rc.borrow())),
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abcd\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"bc") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abbc\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"ab*c") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>).offset((4) as isize) })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 4_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_2,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_2.upgrade().deref())
                            [..(*__tmp_2.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"ababc\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"(ab)*c") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 2_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>).offset((5) as isize) })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 5_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_3,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_3.upgrade().deref())
                            [..(*__tmp_3.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(({ (({ (*m.borrow()).length(1_u64,) }) == 2_i64) }));
        assert!(({ (({ (*m.borrow()).position(1_u64,) }) == 2_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(1_u64,) }))).as_pointer(),) {
                (__tmp_4,) => {
                    Ptr::<i8>::from_string_literal(b"ab").with_c_str(|__s| {
                        (*__tmp_4.upgrade().deref())
                            [..(*__tmp_4.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abcdefghijk\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"cd((e)fg)hi") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abc\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"^abc") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>).offset((3) as isize) })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 3_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_5,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_5.upgrade().deref())
                            [..(*__tmp_5.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abcd\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"^abc") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"aabc\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"^abc") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abc\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"abc$") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>).offset((3) as isize) })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 3_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_6,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_6.upgrade().deref())
                            [..(*__tmp_6.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"efabc\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"abc$") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"efabcg\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"abc$") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abc\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"a.c") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>).offset((3) as isize) })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 3_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_7,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_7.upgrade().deref())
                            [..(*__tmp_7.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"acc\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"a.c") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>).offset((3) as isize) })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 3_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_8,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_8.upgrade().deref())
                            [..(*__tmp_8.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"acc\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"a.c") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>).offset((3) as isize) })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 3_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_9,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_9.upgrade().deref())
                            [..(*__tmp_9.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abcdef\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"(.*).*") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 2_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>).offset((6) as isize) })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 6_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_10,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_10.upgrade().deref())
                            [..(*__tmp_10.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(({ (({ (*m.borrow()).length(1_u64,) }) == 6_i64) }));
        assert!(({ (({ (*m.borrow()).position(1_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(1_u64,) }))).as_pointer(),) {
                (__tmp_11,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_11.upgrade().deref())
                            [..(*__tmp_11.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"bc\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"(a*)*") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abbc\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"ab{3,5}c") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abbbc\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"ab{3,5}c") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_char_::length((s.as_pointer() as Ptr<i8>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_12,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_12.upgrade().deref())
                            [..(*__tmp_12.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abbbbc\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"ab{3,5}c") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_char_::length((s.as_pointer() as Ptr<i8>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_13,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_13.upgrade().deref())
                            [..(*__tmp_13.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abbbbbc\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"ab{3,5}c") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_char_::length((s.as_pointer() as Ptr<i8>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_14,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_14.upgrade().deref())
                            [..(*__tmp_14.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"adefc\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"ab{3,5}c") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abbbbbbc\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"ab{3,5}c") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"adec\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"a.{3,5}c") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"adefc\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"a.{3,5}c") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_char_::length((s.as_pointer() as Ptr<i8>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_15,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_15.upgrade().deref())
                            [..(*__tmp_15.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"adefgc\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"a.{3,5}c") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_char_::length((s.as_pointer() as Ptr<i8>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_16,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_16.upgrade().deref())
                            [..(*__tmp_16.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"adefghc\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"a.{3,5}c") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_char_::length((s.as_pointer() as Ptr<i8>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_17,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_17.upgrade().deref())
                            [..(*__tmp_17.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"adefghic\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"a.{3,5}c") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"tournament\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"tour|to|tournament") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_char_::length((s.as_pointer() as Ptr<i8>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_18,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_18.upgrade().deref())
                            [..(*__tmp_18.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> =
            Rc::new(RefCell::new(i8::array_from_literal(b"tournamenttotour\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"(tour|to|tournament)+") },
                        { Some(nosubs_15.with(|rc| *rc.borrow())) },
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_char_::length((s.as_pointer() as Ptr<i8>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_19,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_19.upgrade().deref())
                            [..(*__tmp_19.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"ttotour\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"(tour|to|t)+") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 2_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_char_::length((s.as_pointer() as Ptr<i8>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_20,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_20.upgrade().deref())
                            [..(*__tmp_20.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(({ (({ (*m.borrow()).length(1_u64,) }) == 4_i64) }));
        assert!(({ (({ (*m.borrow()).position(1_u64,) }) == 3_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(1_u64,) }))).as_pointer(),) {
                (__tmp_21,) => {
                    Ptr::<i8>::from_string_literal(b"tour").with_c_str(|__s| {
                        (*__tmp_21.upgrade().deref())
                            [..(*__tmp_21.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"-ab,ab-\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"-(.*),\x01-") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"-ab,ab-\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"-.*,.*-") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_char_::length((s.as_pointer() as Ptr<i8>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_22,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_22.upgrade().deref())
                            [..(*__tmp_22.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"a\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"^[a]$") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 1_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_23,) => {
                    Ptr::<i8>::from_string_literal(b"a").with_c_str(|__s| {
                        (*__tmp_23.upgrade().deref())
                            [..(*__tmp_23.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"a\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"^[ab]$") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 1_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_24,) => {
                    Ptr::<i8>::from_string_literal(b"a").with_c_str(|__s| {
                        (*__tmp_24.upgrade().deref())
                            [..(*__tmp_24.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"c\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"^[a-f]$") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 1_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_25,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_25.upgrade().deref())
                            [..(*__tmp_25.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"g\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"^[a-f]$") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"Iraqi\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"q[^u]") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"Iraq\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"q[^u]") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"AmB\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"A[[:lower:]]B") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_char_::length((s.as_pointer() as Ptr<i8>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_26,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_26.upgrade().deref())
                            [..(*__tmp_26.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"AMB\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"A[[:lower:]]B") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"AMB\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"A[^[:lower:]]B") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_char_::length((s.as_pointer() as Ptr<i8>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_27,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_27.upgrade().deref())
                            [..(*__tmp_27.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"AmB\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"A[^[:lower:]]B") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"A5B\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"A[^[:lower:]0-9]B") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"A?B\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"A[^[:lower:]0-9]B") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_char_::length((s.as_pointer() as Ptr<i8>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_28,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_28.upgrade().deref())
                            [..(*__tmp_28.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"m\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"[a[=m=]z]") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_char_::length((s.as_pointer() as Ptr<i8>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_29,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_29.upgrade().deref())
                            [..(*__tmp_29.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"m\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"[a[=M=]z]") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"-\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"[a[.hyphen.]z]") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_char_::length((s.as_pointer() as Ptr<i8>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_30,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_30.upgrade().deref())
                            [..(*__tmp_30.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"z\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"[a[.hyphen.]z]") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_char_::length((s.as_pointer() as Ptr<i8>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_31,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_31.upgrade().deref())
                            [..(*__tmp_31.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"m\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"[a[.hyphen.]z]") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"foobar\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"[^\\0]*") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"foo\0bar\0")));
        assert!(
            ({
                let ___s: Ptr<i8> = (s.as_pointer() as Ptr<i8>);
                let ___e: Ptr<i8> = (s.as_pointer() as Ptr<i8>).offset((7) as isize);
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"[abfor\\0]*") },
                        None,
                    )),
                );
                regex_match_16(___s, ___e, m.as_pointer(), ___re.as_pointer(), None)
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"01a45cef9\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"[ace1-9]*") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"01a45cef9\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"[ace1-9]+") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let r: Value<Box<[i8]>> =
            Rc::new(RefCell::new(i8::array_from_literal(b"^[-+]?[0-9]+[CF]$\0")));
        let mut sr: isize =
            (({ std_char_traits_char_::length((r.as_pointer() as Ptr<i8>)) }) as isize);
        let regex: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
            std_basic_regex_char__std_regex_traits_char__::new_2(
                { forward_iterator_const_char_ptr_::new_1({ (r.as_pointer() as Ptr<i8>) }) },
                {
                    forward_iterator_const_char_ptr_::new_1({
                        (r.as_pointer() as Ptr<i8>).offset((sr) as isize)
                    })
                },
                None,
            ),
        ));
        let m : Value<std_match_results_bidirectional_iterator_const_char_ptr___std_allocator_std_sub_match_bidirectional_iterator_const_char_ptr____ > = Rc::new(RefCell::new(<std_match_results_bidirectional_iterator_const_char_ptr___std_allocator_std_sub_match_bidirectional_iterator_const_char_ptr____  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"-40C\0")));
        let mut ss: isize =
            (({ std_char_traits_char_::length((s.as_pointer() as Ptr<i8>)) }) as isize);
        assert!(
            ({
                let ___s: bidirectional_iterator_const_char_ptr_ =
                    bidirectional_iterator_const_char_ptr_::new_2({ (s.as_pointer() as Ptr<i8>) });
                let ___e: bidirectional_iterator_const_char_ptr_ =
                    bidirectional_iterator_const_char_ptr_::new_2({
                        (s.as_pointer() as Ptr<i8>).offset((ss) as isize)
                    });
                regex_match_17(___s, ___e, m.as_pointer(), regex.as_pointer(), None)
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                let _x: Ptr<bidirectional_iterator_const_char_ptr_> = ({
                    ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                        as (
                            Value<bidirectional_iterator_const_char_ptr_>,
                            Value<bidirectional_iterator_const_char_ptr_>,
                        ))
                })
                .0
                .as_pointer();
                let _y: Value<bidirectional_iterator_const_char_ptr_> = Rc::new(RefCell::new(
                    bidirectional_iterator_const_char_ptr_::new_2({ (s.as_pointer() as Ptr<i8>) }),
                ));
                operator_eq_2(_x, _y.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Ptr<bidirectional_iterator_const_char_ptr_> = ({
                    ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                        as (
                            Value<bidirectional_iterator_const_char_ptr_>,
                            Value<bidirectional_iterator_const_char_ptr_>,
                        ))
                })
                .1
                .as_pointer();
                let _y: Ptr<bidirectional_iterator_const_char_ptr_> = ({
                    ((*m.borrow())[0_u64]
                        as (
                            Value<bidirectional_iterator_const_char_ptr_>,
                            Value<bidirectional_iterator_const_char_ptr_>,
                        ))
                })
                .0
                .as_pointer();
                operator_eq_2(_x, _y)
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                let _x: Ptr<bidirectional_iterator_const_char_ptr_> = ({
                    ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                        as (
                            Value<bidirectional_iterator_const_char_ptr_>,
                            Value<bidirectional_iterator_const_char_ptr_>,
                        ))
                })
                .0
                .as_pointer();
                let _y: Ptr<bidirectional_iterator_const_char_ptr_> = ({
                    ((*m.borrow())[0_u64]
                        as (
                            Value<bidirectional_iterator_const_char_ptr_>,
                            Value<bidirectional_iterator_const_char_ptr_>,
                        ))
                })
                .1
                .as_pointer();
                operator_eq_2(_x, _y)
            })
        );
        assert!(
            ({
                let _x: Ptr<bidirectional_iterator_const_char_ptr_> = ({
                    ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                        as (
                            Value<bidirectional_iterator_const_char_ptr_>,
                            Value<bidirectional_iterator_const_char_ptr_>,
                        ))
                })
                .1
                .as_pointer();
                let _y: Ptr<bidirectional_iterator_const_char_ptr_> = ({
                    ((*m.borrow())[0_u64]
                        as (
                            Value<bidirectional_iterator_const_char_ptr_>,
                            Value<bidirectional_iterator_const_char_ptr_>,
                        ))
                })
                .1
                .as_pointer();
                operator_eq_2(_x, _y)
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 4_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_32,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_32.upgrade().deref())
                            [..(*__tmp_32.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"Jeff Jeffs \0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"Jeff(?=s\\b)") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"Jeffs Jeff\0")));
        assert!(
            !({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"Jeff(?!s\\b)") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"5%k\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"\\d[\\W]k") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (s.as_pointer() as Ptr<i8>).offset(
                        ({ std_char_traits_char_::length((s.as_pointer() as Ptr<i8>)) }) as isize,
                    )
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_char_::length((s.as_pointer() as Ptr<i8>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_33,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_33.upgrade().deref())
                            [..(*__tmp_33.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"$_se\0")));
        assert!(
            ({
                let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"\\$\\_se") },
                        None,
                    )),
                );
                regex_match_13(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i8>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i8>, Value<*const i8>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i8>, Value<*const i8>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (s.as_pointer() as Ptr<i8>).offset(
                        ({ std_char_traits_char_::length((s.as_pointer() as Ptr<i8>)) }) as isize,
                    )
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_char_::length((s.as_pointer() as Ptr<i8>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_34,) => {
                    (s.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_34.upgrade().deref())
                            [..(*__tmp_34.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([97 as i32, 0 as i32])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        { Ptr::<i32>::from_string_literal(&[97 as i32, 0 as i32]) },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !({ (*m.borrow()).empty() }) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>).offset((1) as isize) })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 1_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_35,) => {
                    Ptr::<i32>::from_string_literal(&[97 as i32, 0 as i32]).with_c_str(|__s| {
                        (*__tmp_35.upgrade().deref())
                            [..(*__tmp_35.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> =
            Rc::new(RefCell::new(Box::from([97 as i32, 98 as i32, 0 as i32])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        { Ptr::<i32>::from_string_literal(&[97 as i32, 98 as i32, 0 as i32]) },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>).offset((2) as isize) })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 2_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_36,) => {
                    Ptr::<i32>::from_string_literal(&[97 as i32, 98 as i32, 0 as i32]).with_c_str(
                        |__s| {
                            (*__tmp_36.upgrade().deref())
                                [..(*__tmp_36.upgrade().deref()).len().saturating_sub(1)]
                                == *__s
                        },
                    )
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> =
            Rc::new(RefCell::new(Box::from([97 as i32, 98 as i32, 0 as i32])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        { Ptr::<i32>::from_string_literal(&[98 as i32, 97 as i32, 0 as i32]) },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
        assert!(({ ({ (*m.borrow()).empty() }) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 97 as i32, 98 as i32, 0 as i32,
        ])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        { Ptr::<i32>::from_string_literal(&[97 as i32, 98 as i32, 0 as i32]) },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 97 as i32, 98 as i32, 0 as i32,
        ])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        { Ptr::<i32>::from_string_literal(&[97 as i32, 98 as i32, 0 as i32]) },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    Some(match_continuous_14.with(|rc| *rc.borrow())),
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 98 as i32, 99 as i32, 100 as i32, 0 as i32,
        ])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        { Ptr::<i32>::from_string_literal(&[98 as i32, 99 as i32, 0 as i32]) },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 98 as i32, 98 as i32, 99 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                97 as i32, 98 as i32, 42 as i32, 99 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>).offset((4) as isize) })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 4_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_37,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_37.upgrade().deref())
                            [..(*__tmp_37.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 98 as i32, 97 as i32, 98 as i32, 99 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                40 as i32, 97 as i32, 98 as i32, 41 as i32, 42 as i32, 99 as i32,
                                0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 2_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>).offset((5) as isize) })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 5_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_38,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_38.upgrade().deref())
                            [..(*__tmp_38.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(({ (({ (*m.borrow()).length(1_u64,) }) == 2_i64) }));
        assert!(({ (({ (*m.borrow()).position(1_u64,) }) == 2_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(1_u64,) }))).as_pointer(),) {
                (__tmp_39,) => {
                    Ptr::<i32>::from_string_literal(&[97 as i32, 98 as i32, 0 as i32]).with_c_str(
                        |__s| {
                            (*__tmp_39.upgrade().deref())
                                [..(*__tmp_39.upgrade().deref()).len().saturating_sub(1)]
                                == *__s
                        },
                    )
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
            104 as i32, 105 as i32, 106 as i32, 107 as i32, 0 as i32,
        ])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                99 as i32, 100 as i32, 40 as i32, 40 as i32, 101 as i32, 41 as i32,
                                102 as i32, 103 as i32, 41 as i32, 104 as i32, 105 as i32,
                                0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 98 as i32, 99 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                94 as i32, 97 as i32, 98 as i32, 99 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>).offset((3) as isize) })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 3_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_40,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_40.upgrade().deref())
                            [..(*__tmp_40.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 98 as i32, 99 as i32, 100 as i32, 0 as i32,
        ])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                94 as i32, 97 as i32, 98 as i32, 99 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 97 as i32, 98 as i32, 99 as i32, 0 as i32,
        ])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                94 as i32, 97 as i32, 98 as i32, 99 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 98 as i32, 99 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                97 as i32, 98 as i32, 99 as i32, 36 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>).offset((3) as isize) })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 3_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_41,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_41.upgrade().deref())
                            [..(*__tmp_41.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            101 as i32, 102 as i32, 97 as i32, 98 as i32, 99 as i32, 0 as i32,
        ])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                97 as i32, 98 as i32, 99 as i32, 36 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            101 as i32, 102 as i32, 97 as i32, 98 as i32, 99 as i32, 103 as i32, 0 as i32,
        ])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                97 as i32, 98 as i32, 99 as i32, 36 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 98 as i32, 99 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                97 as i32, 46 as i32, 99 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>).offset((3) as isize) })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 3_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_42,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_42.upgrade().deref())
                            [..(*__tmp_42.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 99 as i32, 99 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                97 as i32, 46 as i32, 99 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>).offset((3) as isize) })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 3_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_43,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_43.upgrade().deref())
                            [..(*__tmp_43.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 99 as i32, 99 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                97 as i32, 46 as i32, 99 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>).offset((3) as isize) })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 3_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_44,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_44.upgrade().deref())
                            [..(*__tmp_44.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                40 as i32, 46 as i32, 42 as i32, 41 as i32, 46 as i32, 42 as i32,
                                0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 2_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>).offset((6) as isize) })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 6_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_45,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_45.upgrade().deref())
                            [..(*__tmp_45.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(({ (({ (*m.borrow()).length(1_u64,) }) == 6_i64) }));
        assert!(({ (({ (*m.borrow()).position(1_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(1_u64,) }))).as_pointer(),) {
                (__tmp_46,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_46.upgrade().deref())
                            [..(*__tmp_46.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> =
            Rc::new(RefCell::new(Box::from([98 as i32, 99 as i32, 0 as i32])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                40 as i32, 97 as i32, 42 as i32, 41 as i32, 42 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 98 as i32, 98 as i32, 99 as i32, 0 as i32,
        ])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                97 as i32, 98 as i32, 123 as i32, 51 as i32, 44 as i32, 53 as i32,
                                125 as i32, 99 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 98 as i32, 98 as i32, 98 as i32, 99 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                97 as i32, 98 as i32, 123 as i32, 51 as i32, 44 as i32, 53 as i32,
                                125 as i32, 99 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_wchar_t_::length((s.as_pointer() as Ptr<i32>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_47,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_47.upgrade().deref())
                            [..(*__tmp_47.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 98 as i32, 98 as i32, 98 as i32, 98 as i32, 99 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                97 as i32, 98 as i32, 123 as i32, 51 as i32, 44 as i32, 53 as i32,
                                125 as i32, 99 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_wchar_t_::length((s.as_pointer() as Ptr<i32>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_48,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_48.upgrade().deref())
                            [..(*__tmp_48.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 98 as i32, 98 as i32, 98 as i32, 98 as i32, 98 as i32, 99 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                97 as i32, 98 as i32, 123 as i32, 51 as i32, 44 as i32, 53 as i32,
                                125 as i32, 99 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_wchar_t_::length((s.as_pointer() as Ptr<i32>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_49,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_49.upgrade().deref())
                            [..(*__tmp_49.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 100 as i32, 101 as i32, 102 as i32, 99 as i32, 0 as i32,
        ])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                97 as i32, 98 as i32, 123 as i32, 51 as i32, 44 as i32, 53 as i32,
                                125 as i32, 99 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 98 as i32, 98 as i32, 98 as i32, 98 as i32, 98 as i32, 98 as i32, 99 as i32,
            0 as i32,
        ])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                97 as i32, 98 as i32, 123 as i32, 51 as i32, 44 as i32, 53 as i32,
                                125 as i32, 99 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 100 as i32, 101 as i32, 99 as i32, 0 as i32,
        ])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                97 as i32, 46 as i32, 123 as i32, 51 as i32, 44 as i32, 53 as i32,
                                125 as i32, 99 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 100 as i32, 101 as i32, 102 as i32, 99 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                97 as i32, 46 as i32, 123 as i32, 51 as i32, 44 as i32, 53 as i32,
                                125 as i32, 99 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_wchar_t_::length((s.as_pointer() as Ptr<i32>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_50,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_50.upgrade().deref())
                            [..(*__tmp_50.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32, 99 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                97 as i32, 46 as i32, 123 as i32, 51 as i32, 44 as i32, 53 as i32,
                                125 as i32, 99 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_wchar_t_::length((s.as_pointer() as Ptr<i32>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_51,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_51.upgrade().deref())
                            [..(*__tmp_51.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32, 104 as i32, 99 as i32,
            0 as i32,
        ])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                97 as i32, 46 as i32, 123 as i32, 51 as i32, 44 as i32, 53 as i32,
                                125 as i32, 99 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_wchar_t_::length((s.as_pointer() as Ptr<i32>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_52,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_52.upgrade().deref())
                            [..(*__tmp_52.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32, 104 as i32, 105 as i32,
            99 as i32, 0 as i32,
        ])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                97 as i32, 46 as i32, 123 as i32, 51 as i32, 44 as i32, 53 as i32,
                                125 as i32, 99 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            116 as i32, 111 as i32, 117 as i32, 114 as i32, 110 as i32, 97 as i32, 109 as i32,
            101 as i32, 110 as i32, 116 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                116 as i32, 111 as i32, 117 as i32, 114 as i32, 124 as i32,
                                116 as i32, 111 as i32, 124 as i32, 116 as i32, 111 as i32,
                                117 as i32, 114 as i32, 110 as i32, 97 as i32, 109 as i32,
                                101 as i32, 110 as i32, 116 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_wchar_t_::length((s.as_pointer() as Ptr<i32>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_53,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_53.upgrade().deref())
                            [..(*__tmp_53.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            116 as i32, 111 as i32, 117 as i32, 114 as i32, 110 as i32, 97 as i32, 109 as i32,
            101 as i32, 110 as i32, 116 as i32, 116 as i32, 111 as i32, 116 as i32, 111 as i32,
            117 as i32, 114 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                40 as i32, 116 as i32, 111 as i32, 117 as i32, 114 as i32,
                                124 as i32, 116 as i32, 111 as i32, 124 as i32, 116 as i32,
                                111 as i32, 117 as i32, 114 as i32, 110 as i32, 97 as i32,
                                109 as i32, 101 as i32, 110 as i32, 116 as i32, 41 as i32,
                                43 as i32, 0 as i32,
                            ])
                        },
                        { Some(nosubs_15.with(|rc| *rc.borrow())) },
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_wchar_t_::length((s.as_pointer() as Ptr<i32>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_54,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_54.upgrade().deref())
                            [..(*__tmp_54.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            116 as i32, 116 as i32, 111 as i32, 116 as i32, 111 as i32, 117 as i32, 114 as i32,
            0 as i32,
        ])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                40 as i32, 116 as i32, 111 as i32, 117 as i32, 114 as i32,
                                124 as i32, 116 as i32, 111 as i32, 124 as i32, 116 as i32,
                                41 as i32, 43 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 2_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_wchar_t_::length((s.as_pointer() as Ptr<i32>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_55,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_55.upgrade().deref())
                            [..(*__tmp_55.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(({ (({ (*m.borrow()).length(1_u64,) }) == 4_i64) }));
        assert!(({ (({ (*m.borrow()).position(1_u64,) }) == 3_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(1_u64,) }))).as_pointer(),) {
                (__tmp_56,) => {
                    Ptr::<i32>::from_string_literal(&[
                        116 as i32, 111 as i32, 117 as i32, 114 as i32, 0 as i32,
                    ])
                    .with_c_str(|__s| {
                        (*__tmp_56.upgrade().deref())
                            [..(*__tmp_56.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            45 as i32, 97 as i32, 98 as i32, 44 as i32, 97 as i32, 98 as i32, 45 as i32, 0 as i32,
        ])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                45 as i32, 40 as i32, 46 as i32, 42 as i32, 41 as i32, 44 as i32,
                                1 as i32, 45 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            45 as i32, 97 as i32, 98 as i32, 44 as i32, 97 as i32, 98 as i32, 45 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                45 as i32, 46 as i32, 42 as i32, 44 as i32, 46 as i32, 42 as i32,
                                45 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_wchar_t_::length((s.as_pointer() as Ptr<i32>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_57,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_57.upgrade().deref())
                            [..(*__tmp_57.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([97 as i32, 0 as i32])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                94 as i32, 91 as i32, 97 as i32, 93 as i32, 36 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 1_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_58,) => {
                    Ptr::<i32>::from_string_literal(&[97 as i32, 0 as i32]).with_c_str(|__s| {
                        (*__tmp_58.upgrade().deref())
                            [..(*__tmp_58.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([97 as i32, 0 as i32])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                94 as i32, 91 as i32, 97 as i32, 98 as i32, 93 as i32, 36 as i32,
                                0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 1_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_59,) => {
                    Ptr::<i32>::from_string_literal(&[97 as i32, 0 as i32]).with_c_str(|__s| {
                        (*__tmp_59.upgrade().deref())
                            [..(*__tmp_59.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([99 as i32, 0 as i32])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                94 as i32, 91 as i32, 97 as i32, 45 as i32, 102 as i32, 93 as i32,
                                36 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 1_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_60,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_60.upgrade().deref())
                            [..(*__tmp_60.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([103 as i32, 0 as i32])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                94 as i32, 91 as i32, 97 as i32, 45 as i32, 102 as i32, 93 as i32,
                                36 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            73 as i32, 114 as i32, 97 as i32, 113 as i32, 105 as i32, 0 as i32,
        ])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                113 as i32, 91 as i32, 94 as i32, 117 as i32, 93 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            73 as i32, 114 as i32, 97 as i32, 113 as i32, 0 as i32,
        ])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                113 as i32, 91 as i32, 94 as i32, 117 as i32, 93 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            65 as i32, 109 as i32, 66 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                65 as i32, 91 as i32, 91 as i32, 58 as i32, 108 as i32, 111 as i32,
                                119 as i32, 101 as i32, 114 as i32, 58 as i32, 93 as i32,
                                93 as i32, 66 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_wchar_t_::length((s.as_pointer() as Ptr<i32>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_61,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_61.upgrade().deref())
                            [..(*__tmp_61.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            65 as i32, 77 as i32, 66 as i32, 0 as i32,
        ])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                65 as i32, 91 as i32, 91 as i32, 58 as i32, 108 as i32, 111 as i32,
                                119 as i32, 101 as i32, 114 as i32, 58 as i32, 93 as i32,
                                93 as i32, 66 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            65 as i32, 77 as i32, 66 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                65 as i32, 91 as i32, 94 as i32, 91 as i32, 58 as i32, 108 as i32,
                                111 as i32, 119 as i32, 101 as i32, 114 as i32, 58 as i32,
                                93 as i32, 93 as i32, 66 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_wchar_t_::length((s.as_pointer() as Ptr<i32>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_62,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_62.upgrade().deref())
                            [..(*__tmp_62.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            65 as i32, 109 as i32, 66 as i32, 0 as i32,
        ])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                65 as i32, 91 as i32, 94 as i32, 91 as i32, 58 as i32, 108 as i32,
                                111 as i32, 119 as i32, 101 as i32, 114 as i32, 58 as i32,
                                93 as i32, 93 as i32, 66 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            65 as i32, 53 as i32, 66 as i32, 0 as i32,
        ])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                65 as i32, 91 as i32, 94 as i32, 91 as i32, 58 as i32, 108 as i32,
                                111 as i32, 119 as i32, 101 as i32, 114 as i32, 58 as i32,
                                93 as i32, 48 as i32, 45 as i32, 57 as i32, 93 as i32, 66 as i32,
                                0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            65 as i32, 63 as i32, 66 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                65 as i32, 91 as i32, 94 as i32, 91 as i32, 58 as i32, 108 as i32,
                                111 as i32, 119 as i32, 101 as i32, 114 as i32, 58 as i32,
                                93 as i32, 48 as i32, 45 as i32, 57 as i32, 93 as i32, 66 as i32,
                                0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_wchar_t_::length((s.as_pointer() as Ptr<i32>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_63,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_63.upgrade().deref())
                            [..(*__tmp_63.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([109 as i32, 0 as i32])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                91 as i32, 97 as i32, 91 as i32, 61 as i32, 109 as i32, 61 as i32,
                                93 as i32, 122 as i32, 93 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_wchar_t_::length((s.as_pointer() as Ptr<i32>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_64,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_64.upgrade().deref())
                            [..(*__tmp_64.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([109 as i32, 0 as i32])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                91 as i32, 97 as i32, 91 as i32, 61 as i32, 77 as i32, 61 as i32,
                                93 as i32, 122 as i32, 93 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([45 as i32, 0 as i32])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                91 as i32, 97 as i32, 91 as i32, 46 as i32, 104 as i32, 121 as i32,
                                112 as i32, 104 as i32, 101 as i32, 110 as i32, 46 as i32,
                                93 as i32, 122 as i32, 93 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_wchar_t_::length((s.as_pointer() as Ptr<i32>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_65,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_65.upgrade().deref())
                            [..(*__tmp_65.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([122 as i32, 0 as i32])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                91 as i32, 97 as i32, 91 as i32, 46 as i32, 104 as i32, 121 as i32,
                                112 as i32, 104 as i32, 101 as i32, 110 as i32, 46 as i32,
                                93 as i32, 122 as i32, 93 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_wchar_t_::length((s.as_pointer() as Ptr<i32>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_66,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_66.upgrade().deref())
                            [..(*__tmp_66.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([109 as i32, 0 as i32])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                91 as i32, 97 as i32, 91 as i32, 46 as i32, 104 as i32, 121 as i32,
                                112 as i32, 104 as i32, 101 as i32, 110 as i32, 46 as i32,
                                93 as i32, 122 as i32, 93 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            48 as i32, 49 as i32, 97 as i32, 52 as i32, 53 as i32, 99 as i32, 101 as i32,
            102 as i32, 57 as i32, 0 as i32,
        ])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                91 as i32, 97 as i32, 99 as i32, 101 as i32, 49 as i32, 45 as i32,
                                57 as i32, 93 as i32, 42 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            48 as i32, 49 as i32, 97 as i32, 52 as i32, 53 as i32, 99 as i32, 101 as i32,
            102 as i32, 57 as i32, 0 as i32,
        ])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                91 as i32, 97 as i32, 99 as i32, 101 as i32, 49 as i32, 45 as i32,
                                57 as i32, 93 as i32, 43 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let r: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            94 as i32, 91 as i32, 45 as i32, 43 as i32, 93 as i32, 63 as i32, 91 as i32, 48 as i32,
            45 as i32, 57 as i32, 93 as i32, 43 as i32, 91 as i32, 67 as i32, 70 as i32, 93 as i32,
            36 as i32, 0 as i32,
        ])));
        let mut sr: isize =
            (({ std_char_traits_wchar_t_::length((r.as_pointer() as Ptr<i32>)) }) as isize);
        let regex: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
            RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_2(
                { forward_iterator_const_wchar_t_ptr_::new_1({ (r.as_pointer() as Ptr<i32>) }) },
                {
                    forward_iterator_const_wchar_t_ptr_::new_1({
                        (r.as_pointer() as Ptr<i32>).offset((sr) as isize)
                    })
                },
                None,
            )),
        );
        let m : Value<std_match_results_bidirectional_iterator_const_wchar_t_ptr___std_allocator_std_sub_match_bidirectional_iterator_const_wchar_t_ptr____ > = Rc::new(RefCell::new(<std_match_results_bidirectional_iterator_const_wchar_t_ptr___std_allocator_std_sub_match_bidirectional_iterator_const_wchar_t_ptr____  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            45 as i32, 52 as i32, 48 as i32, 67 as i32, 0 as i32,
        ])));
        let mut ss: isize =
            (({ std_char_traits_wchar_t_::length((s.as_pointer() as Ptr<i32>)) }) as isize);
        assert!(
            ({
                let ___s: bidirectional_iterator_const_wchar_t_ptr_ =
                    bidirectional_iterator_const_wchar_t_ptr_::new_2({
                        (s.as_pointer() as Ptr<i32>)
                    });
                let ___e: bidirectional_iterator_const_wchar_t_ptr_ =
                    bidirectional_iterator_const_wchar_t_ptr_::new_2({
                        (s.as_pointer() as Ptr<i32>).offset((ss) as isize)
                    });
                regex_match_19(___s, ___e, m.as_pointer(), regex.as_pointer(), None)
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                let _x: Ptr<bidirectional_iterator_const_wchar_t_ptr_> = ({
                    ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                        as (
                            Value<bidirectional_iterator_const_wchar_t_ptr_>,
                            Value<bidirectional_iterator_const_wchar_t_ptr_>,
                        ))
                })
                .0
                .as_pointer();
                let _y: Value<bidirectional_iterator_const_wchar_t_ptr_> = Rc::new(RefCell::new(
                    bidirectional_iterator_const_wchar_t_ptr_::new_2({
                        (s.as_pointer() as Ptr<i32>)
                    }),
                ));
                operator_eq_4(_x, _y.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Ptr<bidirectional_iterator_const_wchar_t_ptr_> = ({
                    ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                        as (
                            Value<bidirectional_iterator_const_wchar_t_ptr_>,
                            Value<bidirectional_iterator_const_wchar_t_ptr_>,
                        ))
                })
                .1
                .as_pointer();
                let _y: Ptr<bidirectional_iterator_const_wchar_t_ptr_> = ({
                    ((*m.borrow())[0_u64]
                        as (
                            Value<bidirectional_iterator_const_wchar_t_ptr_>,
                            Value<bidirectional_iterator_const_wchar_t_ptr_>,
                        ))
                })
                .0
                .as_pointer();
                operator_eq_4(_x, _y)
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                let _x: Ptr<bidirectional_iterator_const_wchar_t_ptr_> = ({
                    ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                        as (
                            Value<bidirectional_iterator_const_wchar_t_ptr_>,
                            Value<bidirectional_iterator_const_wchar_t_ptr_>,
                        ))
                })
                .0
                .as_pointer();
                let _y: Ptr<bidirectional_iterator_const_wchar_t_ptr_> = ({
                    ((*m.borrow())[0_u64]
                        as (
                            Value<bidirectional_iterator_const_wchar_t_ptr_>,
                            Value<bidirectional_iterator_const_wchar_t_ptr_>,
                        ))
                })
                .1
                .as_pointer();
                operator_eq_4(_x, _y)
            })
        );
        assert!(
            ({
                let _x: Ptr<bidirectional_iterator_const_wchar_t_ptr_> = ({
                    ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                        as (
                            Value<bidirectional_iterator_const_wchar_t_ptr_>,
                            Value<bidirectional_iterator_const_wchar_t_ptr_>,
                        ))
                })
                .1
                .as_pointer();
                let _y: Ptr<bidirectional_iterator_const_wchar_t_ptr_> = ({
                    ((*m.borrow())[0_u64]
                        as (
                            Value<bidirectional_iterator_const_wchar_t_ptr_>,
                            Value<bidirectional_iterator_const_wchar_t_ptr_>,
                        ))
                })
                .1
                .as_pointer();
                operator_eq_4(_x, _y)
            })
        );
        assert!(({ (({ (*m.borrow()).length(0_u64,) }) == 4_i64) }));
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_67,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_67.upgrade().deref())
                            [..(*__tmp_67.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            74 as i32, 101 as i32, 102 as i32, 102 as i32, 32 as i32, 74 as i32, 101 as i32,
            102 as i32, 102 as i32, 115 as i32, 32 as i32, 0 as i32,
        ])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                74 as i32, 101 as i32, 102 as i32, 102 as i32, 40 as i32,
                                63 as i32, 61 as i32, 115 as i32, 92 as i32, 98 as i32, 41 as i32,
                                0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            74 as i32, 101 as i32, 102 as i32, 102 as i32, 115 as i32, 32 as i32, 74 as i32,
            101 as i32, 102 as i32, 102 as i32, 0 as i32,
        ])));
        assert!(
            !({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                74 as i32, 101 as i32, 102 as i32, 102 as i32, 40 as i32,
                                63 as i32, 33 as i32, 115 as i32, 92 as i32, 98 as i32, 41 as i32,
                                0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            53 as i32, 37 as i32, 107 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                92 as i32, 100 as i32, 91 as i32, 92 as i32, 87 as i32, 93 as i32,
                                107 as i32, 0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (s.as_pointer() as Ptr<i32>).offset(
                        ({ std_char_traits_wchar_t_::length((s.as_pointer() as Ptr<i32>)) })
                            as isize,
                    )
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_wchar_t_::length((s.as_pointer() as Ptr<i32>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_68,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_68.upgrade().deref())
                            [..(*__tmp_68.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            36 as i32, 95 as i32, 115 as i32, 101 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___re: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                92 as i32, 36 as i32, 92 as i32, 95 as i32, 115 as i32, 101 as i32,
                                0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_match_18(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___re.as_pointer(),
                    None,
                )
            })
        );
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(({ !(({ (*m.borrow()).prefix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == { (s.as_pointer() as Ptr<i32>) })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).prefix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .0
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(({ !(({ (*m.borrow()).suffix() }).with(|__s| __s.matched)) }));
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .0
                    .borrow())
                    .clone()
                } == {
                    (*({ ((*m.borrow())[0_u64] as (Value<*const i32>, Value<*const i32>)) })
                        .1
                        .borrow())
                    .clone()
                })
            })
        );
        assert!(
            ({
                ({
                    (*({
                        ((*({ (*m.borrow()).suffix() }).upgrade().deref())
                            as (Value<*const i32>, Value<*const i32>))
                    })
                    .1
                    .borrow())
                    .clone()
                } == {
                    (s.as_pointer() as Ptr<i32>).offset(
                        ({ std_char_traits_wchar_t_::length((s.as_pointer() as Ptr<i32>)) })
                            as isize,
                    )
                })
            })
        );
        assert!(
            ({
                (({ (*m.borrow()).length(0_u64) }) >= 0_i64)
                    && ((({ (*m.borrow()).length(0_u64) }) as usize)
                        == ({ std_char_traits_wchar_t_::length((s.as_pointer() as Ptr<i32>)) }))
            })
        );
        assert!(({ (({ (*m.borrow()).position(0_u64,) }) == 0_i64) }));
        assert!(
            match (Rc::new(RefCell::new(({ (*m.borrow()).str(0_u64,) }))).as_pointer(),) {
                (__tmp_69,) => {
                    (s.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_69.upgrade().deref())
                            [..(*__tmp_69.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
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
    fn operator_deref(&self) -> Ptr<i8>;
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
    fn operator_deref(&self) -> Ptr<i8> {
        return (*self).with(|__s| __s.it_.clone());
    }
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
pub trait bidirectional_iterator_const_wchar_t_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_const_wchar_t_ptr_>;
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_const_wchar_t_ptr_>;
    fn operator_post_inc_8(&self, mut _a0: i32) -> bidirectional_iterator_const_wchar_t_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_9(&self, mut _a0: i32) -> bidirectional_iterator_const_wchar_t_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_const_wchar_t_ptr_>,
    ) -> Ptr<bidirectional_iterator_const_wchar_t_ptr_>;
}
impl bidirectional_iterator_const_wchar_t_ptr_Impl
    for Ptr<bidirectional_iterator_const_wchar_t_ptr_>
{
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_const_wchar_t_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_const_wchar_t_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_dec();
        });
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_const_wchar_t_ptr_>,
    ) -> Ptr<bidirectional_iterator_const_wchar_t_ptr_> {
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
    fn operator_deref(&self) -> Ptr<i8>;
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
    fn operator_deref(&self) -> Ptr<i8> {
        return (*self).with(|__s| __s.it_.clone());
    }
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
pub trait forward_iterator_const_wchar_t_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_wchar_t_ptr_>;
    fn operator_post_inc_7(&self, mut _a0: i32) -> forward_iterator_const_wchar_t_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<forward_iterator_const_wchar_t_ptr_>,
    ) -> Ptr<forward_iterator_const_wchar_t_ptr_>;
}
impl forward_iterator_const_wchar_t_ptr_Impl for Ptr<forward_iterator_const_wchar_t_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_wchar_t_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<forward_iterator_const_wchar_t_ptr_>,
    ) -> Ptr<forward_iterator_const_wchar_t_ptr_> {
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
