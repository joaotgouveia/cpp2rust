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
pub fn operator_eq_0(x: Ptr<forward_iterator_int_ptr_>, y: Ptr<forward_iterator_int_ptr_>) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_1(x: Ptr<forward_iterator_int_ptr_>, y: Ptr<forward_iterator_int_ptr_>) -> bool {
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
impl std::cmp::PartialEq for forward_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_0(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for forward_iterator_int_ptr_ {}
impl CppAdvance for forward_iterator_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
}
impl CppIterator<i32> for forward_iterator_int_ptr_ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            forward_iterator_int_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            forward_iterator_int_ptr_Impl::operator_inc_6(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for forward_iterator_int_ptr_ {}
impl CppMove for forward_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_eq_2(
    x: Ptr<forward_iterator_MoveOnly_ptr_>,
    y: Ptr<forward_iterator_MoveOnly_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_3(
    x: Ptr<forward_iterator_MoveOnly_ptr_>,
    y: Ptr<forward_iterator_MoveOnly_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct forward_iterator_MoveOnly_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<MoveOnly>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl forward_iterator_MoveOnly_ptr_ {
    pub fn new_1(mut it: Ptr<MoveOnly>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<forward_iterator_MoveOnly_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for forward_iterator_MoveOnly_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_2(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for forward_iterator_MoveOnly_ptr_ {}
impl CppAdvance for forward_iterator_MoveOnly_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
}
impl CppIterator<MoveOnly> for forward_iterator_MoveOnly_ptr_ {
    fn iter_deref(&self) -> Ptr<MoveOnly> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            forward_iterator_MoveOnly_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            forward_iterator_MoveOnly_ptr_Impl::operator_inc_6(__it);
        });
        self
    }
}
impl CppInputIterator<MoveOnly> for forward_iterator_MoveOnly_ptr_ {}
impl CppMove for forward_iterator_MoveOnly_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_eq_4(
    x: Ptr<bidirectional_iterator_int_ptr_>,
    y: Ptr<bidirectional_iterator_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_5(
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
impl std::cmp::PartialEq for bidirectional_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_4(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_int_ptr_ {}
impl CppAdvance for bidirectional_iterator_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_7();
    }
}
impl CppIterator<i32> for bidirectional_iterator_int_ptr_ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            bidirectional_iterator_int_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            bidirectional_iterator_int_ptr_Impl::operator_inc_6(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for bidirectional_iterator_int_ptr_ {}
impl CppBidirectionalIterator<i32> for bidirectional_iterator_int_ptr_ {
    fn iter_dec(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            bidirectional_iterator_int_ptr_Impl::operator_dec_7(__it);
        });
        self
    }
}
impl CppMove for bidirectional_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_eq_6(
    x: Ptr<bidirectional_iterator_MoveOnly_ptr_>,
    y: Ptr<bidirectional_iterator_MoveOnly_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_7(
    x: Ptr<bidirectional_iterator_MoveOnly_ptr_>,
    y: Ptr<bidirectional_iterator_MoveOnly_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct bidirectional_iterator_MoveOnly_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<MoveOnly>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_MoveOnly_ptr_ {
    pub fn new_1(mut it: Ptr<MoveOnly>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<bidirectional_iterator_MoveOnly_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_MoveOnly_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_6(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_MoveOnly_ptr_ {}
impl CppAdvance for bidirectional_iterator_MoveOnly_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_7();
    }
}
impl CppIterator<MoveOnly> for bidirectional_iterator_MoveOnly_ptr_ {
    fn iter_deref(&self) -> Ptr<MoveOnly> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            bidirectional_iterator_MoveOnly_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            bidirectional_iterator_MoveOnly_ptr_Impl::operator_inc_6(__it);
        });
        self
    }
}
impl CppInputIterator<MoveOnly> for bidirectional_iterator_MoveOnly_ptr_ {}
impl CppBidirectionalIterator<MoveOnly> for bidirectional_iterator_MoveOnly_ptr_ {
    fn iter_dec(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            bidirectional_iterator_MoveOnly_ptr_Impl::operator_dec_7(__it);
        });
        self
    }
}
impl CppMove for bidirectional_iterator_MoveOnly_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_sub_8(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> isize {
    return (((x.it_).clone() - (y.it_).clone()) as i64 as isize);
}
pub fn operator_eq_9(
    x: Ptr<random_access_iterator_int_ptr_>,
    y: Ptr<random_access_iterator_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_10(
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
impl std::cmp::PartialEq for random_access_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_9(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for random_access_iterator_int_ptr_ {}
impl CppAdvance for random_access_iterator_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_7();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_8();
    }
}
impl CppIterator<i32> for random_access_iterator_int_ptr_ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            random_access_iterator_int_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            random_access_iterator_int_ptr_Impl::operator_inc_7(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for random_access_iterator_int_ptr_ {}
impl CppBidirectionalIterator<i32> for random_access_iterator_int_ptr_ {
    fn iter_dec(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            random_access_iterator_int_ptr_Impl::operator_dec_8(__it);
        });
        self
    }
}
impl CppMove for random_access_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_sub_11(
    mut x: random_access_iterator_MoveOnly_ptr_,
    mut y: random_access_iterator_MoveOnly_ptr_,
) -> isize {
    return (((x.it_).clone() - (y.it_).clone()) as i64 as isize);
}
pub fn operator_eq_12(
    x: Ptr<random_access_iterator_MoveOnly_ptr_>,
    y: Ptr<random_access_iterator_MoveOnly_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_13(
    x: Ptr<random_access_iterator_MoveOnly_ptr_>,
    y: Ptr<random_access_iterator_MoveOnly_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct random_access_iterator_MoveOnly_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<MoveOnly>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_MoveOnly_ptr_ {
    pub fn new_1(mut it: Ptr<MoveOnly>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<random_access_iterator_MoveOnly_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for random_access_iterator_MoveOnly_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_12(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for random_access_iterator_MoveOnly_ptr_ {}
impl CppAdvance for random_access_iterator_MoveOnly_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_7();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_8();
    }
}
impl CppIterator<MoveOnly> for random_access_iterator_MoveOnly_ptr_ {
    fn iter_deref(&self) -> Ptr<MoveOnly> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            random_access_iterator_MoveOnly_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            random_access_iterator_MoveOnly_ptr_Impl::operator_inc_7(__it);
        });
        self
    }
}
impl CppInputIterator<MoveOnly> for random_access_iterator_MoveOnly_ptr_ {}
impl CppBidirectionalIterator<MoveOnly> for random_access_iterator_MoveOnly_ptr_ {
    fn iter_dec(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            random_access_iterator_MoveOnly_ptr_Impl::operator_dec_8(__it);
        });
        self
    }
}
impl CppMove for random_access_iterator_MoveOnly_ptr_ {
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
pub fn operator_eq_14(
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
        operator_eq_14(__lhs.clone(), __rhs.clone())
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
pub fn operator_add_15(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_16(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_15(_i, n)
    });
}
pub fn iter_swap_17(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap!((a.ptr_).clone(), (b.ptr_).clone());
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| {
            __v.prefix_inc();
        });
    }
}
pub fn iter_move_18(iter: adl_Iterator) -> Ptr<i32> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| {
            __v.prefix_inc();
        });
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_19(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> bool {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_20(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> std::cmp::Ordering {
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
                operator_cmp_20(__lhs.clone(), __rhs.clone())
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
                operator_eq_19(__lhs.clone(), __rhs.clone())
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
pub fn operator_eq_21(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_22(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_23(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_24(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_25(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_26(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_27(_a0: Ptr<MoveOnly>, _a1: Ptr<MoveOnly>) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
            return ({ (*cmp.borrow()) });
        }
    }
    return ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) });
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, MoveAssign)]
#[cpp_from(i32, new, some_value)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    data_: i32,
}
impl MoveOnly {
    pub fn new(data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| 1);
        Self { data_: data }
    }
    pub fn move_from(x: Ptr<MoveOnly>) -> Self {
        let __this: MoveOnly = Self {
            data_: x.with(|__s| __s.data_),
        };
        field!(x, data_).write(0);
        __this
    }
}
impl CppLess for MoveOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_23(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for MoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_21(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for MoveOnly {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new(None) }
    }
}
pub fn operator_eq_28(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_29(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_30(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_31(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_32(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_33(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_34(_a0: Ptr<TrivialMoveOnly>, _a1: Ptr<TrivialMoveOnly>) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
            return ({ (*cmp.borrow()) });
        }
    }
    return ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) });
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, MoveAssign)]
#[cpp_from(i32, new, some_value)]
#[byte_size(4)]
pub struct TrivialMoveOnly {
    #[offset(0)]
    data_: i32,
}
impl TrivialMoveOnly {
    pub fn new(data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| 1);
        Self { data_: data }
    }
    pub fn move_from(_a0: Ptr<TrivialMoveOnly>) -> Self {
        Self {
            data_: { (*_a0.upgrade().deref()).data_ },
        }
    }
}
impl CppLess for TrivialMoveOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_30(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for TrivialMoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_28(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for TrivialMoveOnly {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        { TrivialMoveOnly::new(None) }
    }
}
pub fn test_35() -> bool {
    let orig: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9,
    ])));
    let work: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9,
    ])));
    let mut n: i32 = 0;
    'loop_: while (n <= 15) {
        let mut k: i32 = 0;
        'loop_: while (k <= (n + 2)) {
            match ({ (orig.as_pointer() as Ptr<i32>).offset((n) as isize) },) {
                (mut __tmp_0,) => {
                    let mut __first = (orig.as_pointer() as Ptr<i32>);
                    let mut __out = (work.as_pointer() as Ptr<i32>);
                    while __first != __tmp_0 {
                        __out.iter_deref().write((__first.iter_deref().read()));
                        __first.iter_inc();
                        __out.iter_inc();
                    }
                    __out
                }
            };
            let it: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
                ({
                    let ___first: forward_iterator_int_ptr_ =
                        forward_iterator_int_ptr_::new_1({ (work.as_pointer() as Ptr<i32>) });
                    let ___last: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                        (work.as_pointer() as Ptr<i32>).offset((n) as isize)
                    });
                    shift_right_36(___first, ___last, (k as isize))
                }),
            ));
            if (k < n) {
                assert!(
                    ({
                        let _x: Ptr<forward_iterator_int_ptr_> = it.as_pointer();
                        let _y: Value<forward_iterator_int_ptr_> =
                            Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                                (work.as_pointer() as Ptr<i32>).offset((k) as isize)
                            })));
                        operator_eq_0(_x, _y.as_pointer())
                    })
                );
                assert!(
                    ({
                        let ___first1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>);
                        let ___last1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>)
                            .offset((n) as isize)
                            .offset(-((k) as isize));
                        let ___first2: Ptr<i32> =
                            (work.as_pointer() as Ptr<i32>).offset((k) as isize);
                        let ___last2: Ptr<i32> =
                            (work.as_pointer() as Ptr<i32>).offset((n) as isize);
                        equal_37(___first1, ___last1, ___first2, ___last2)
                    })
                );
            } else {
                assert!(
                    ({
                        let _x: Ptr<forward_iterator_int_ptr_> = it.as_pointer();
                        let _y: Value<forward_iterator_int_ptr_> =
                            Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                                (work.as_pointer() as Ptr<i32>).offset((n) as isize)
                            })));
                        operator_eq_0(_x, _y.as_pointer())
                    })
                );
                assert!(
                    ({
                        let ___first1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>);
                        let ___last1: Ptr<i32> =
                            (orig.as_pointer() as Ptr<i32>).offset((n) as isize);
                        let ___first2: Ptr<i32> = (work.as_pointer() as Ptr<i32>);
                        let ___last2: Ptr<i32> =
                            (work.as_pointer() as Ptr<i32>).offset((n) as isize);
                        equal_37(___first1, ___last1, ___first2, ___last2)
                    })
                );
            }
            k.prefix_inc();
        }
        n.prefix_inc();
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let mut b: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ({ begin_38(input.as_pointer()) }) });
        let mut e: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ (input.as_pointer() as Ptr<i32>).to_end() });
        let mut it: forward_iterator_int_ptr_ =
            ({ shift_right_36((b).clone(), (e).clone(), 0_isize) });
        assert!(
            ({
                let ___first1: Ptr<i32> = ({ begin_39(expected.as_pointer()) });
                let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                equal_40(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1])));
        let mut b: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ({ begin_38(input.as_pointer()) }) });
        let mut e: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ (input.as_pointer() as Ptr<i32>).to_end() });
        let mut it: forward_iterator_int_ptr_ =
            ({ shift_right_36((b).clone(), (e).clone(), 1_isize) });
        assert!(
            ({
                let ___first1: Ptr<i32> = ({ begin_41(expected.as_pointer()) });
                let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                equal_40(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6, 7, 8])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
        let mut b: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ({ begin_42(input.as_pointer()) }) });
        let mut e: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ (input.as_pointer() as Ptr<i32>).to_end() });
        let mut it: forward_iterator_int_ptr_ =
            ({ shift_right_36((b).clone(), (e).clone(), 2_isize) });
        assert!(
            ({
                let ___first1: Ptr<i32> = ({ begin_43(expected.as_pointer()) });
                let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                equal_40(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6, 7, 8])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
        let mut b: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ({ begin_42(input.as_pointer()) }) });
        let mut e: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ (input.as_pointer() as Ptr<i32>).to_end() });
        let mut it: forward_iterator_int_ptr_ =
            ({ shift_right_36((b).clone(), (e).clone(), 6_isize) });
        assert!(
            ({
                let ___first1: Ptr<i32> = ({ begin_41(expected.as_pointer()) });
                let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                equal_40(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let mut b: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ({ begin_38(input.as_pointer()) }) });
        let e: Value<forward_iterator_int_ptr_> =
            Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                (input.as_pointer() as Ptr<i32>).to_end()
            })));
        let it: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                shift_right_36(
                    (b).clone(),
                    (*e.borrow()).clone(),
                    (({ size_44(input.as_pointer()) }) as isize),
                )
            }),
        ));
        assert!(
            ({
                ({
                    let ___first1: Ptr<i32> = ({ begin_39(expected.as_pointer()) });
                    let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                    equal_40(___first1, ___last1, (b).clone(), (*e.borrow()).clone())
                })
            })
        );
        assert!(
            ({
                let _x: Ptr<forward_iterator_int_ptr_> = it.as_pointer();
                operator_eq_0(_x, e.as_pointer())
            })
        );
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let mut b: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ({ begin_38(input.as_pointer()) }) });
        let e: Value<forward_iterator_int_ptr_> =
            Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                (input.as_pointer() as Ptr<i32>).to_end()
            })));
        let it: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                shift_right_36(
                    (b).clone(),
                    (*e.borrow()).clone(),
                    (((({ size_44(input.as_pointer()) }) as usize).wrapping_add(1_usize)) as isize),
                )
            }),
        ));
        assert!(
            ({
                ({
                    let ___first1: Ptr<i32> = ({ begin_39(expected.as_pointer()) });
                    let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                    equal_40(___first1, ___last1, (b).clone(), (*e.borrow()).clone())
                })
            })
        );
        assert!(
            ({
                let _x: Ptr<forward_iterator_int_ptr_> = it.as_pointer();
                operator_eq_0(_x, e.as_pointer())
            })
        );
    }
    return true;
}
pub fn test_45() -> bool {
    let orig: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9,
    ])));
    let work: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9,
    ])));
    let mut n: i32 = 0;
    'loop_: while (n <= 15) {
        let mut k: i32 = 0;
        'loop_: while (k <= (n + 2)) {
            match ({ (orig.as_pointer() as Ptr<i32>).offset((n) as isize) },) {
                (mut __tmp_1,) => {
                    let mut __first = (orig.as_pointer() as Ptr<i32>);
                    let mut __out = (work.as_pointer() as Ptr<i32>);
                    while __first != __tmp_1 {
                        __out.iter_deref().write((__first.iter_deref().read()));
                        __first.iter_inc();
                        __out.iter_inc();
                    }
                    __out
                }
            };
            let it: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
                ({
                    let ___first: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({ (work.as_pointer() as Ptr<i32>) });
                    let ___last: bidirectional_iterator_int_ptr_ =
                        bidirectional_iterator_int_ptr_::new_1({
                            (work.as_pointer() as Ptr<i32>).offset((n) as isize)
                        });
                    shift_right_46(___first, ___last, (k as isize))
                }),
            ));
            if (k < n) {
                assert!(
                    ({
                        let _x: Ptr<bidirectional_iterator_int_ptr_> = it.as_pointer();
                        let _y: Value<bidirectional_iterator_int_ptr_> =
                            Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                                (work.as_pointer() as Ptr<i32>).offset((k) as isize)
                            })));
                        operator_eq_4(_x, _y.as_pointer())
                    })
                );
                assert!(
                    ({
                        let ___first1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>);
                        let ___last1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>)
                            .offset((n) as isize)
                            .offset(-((k) as isize));
                        let ___first2: Ptr<i32> =
                            (work.as_pointer() as Ptr<i32>).offset((k) as isize);
                        let ___last2: Ptr<i32> =
                            (work.as_pointer() as Ptr<i32>).offset((n) as isize);
                        equal_37(___first1, ___last1, ___first2, ___last2)
                    })
                );
            } else {
                assert!(
                    ({
                        let _x: Ptr<bidirectional_iterator_int_ptr_> = it.as_pointer();
                        let _y: Value<bidirectional_iterator_int_ptr_> =
                            Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                                (work.as_pointer() as Ptr<i32>).offset((n) as isize)
                            })));
                        operator_eq_4(_x, _y.as_pointer())
                    })
                );
                assert!(
                    ({
                        let ___first1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>);
                        let ___last1: Ptr<i32> =
                            (orig.as_pointer() as Ptr<i32>).offset((n) as isize);
                        let ___first2: Ptr<i32> = (work.as_pointer() as Ptr<i32>);
                        let ___last2: Ptr<i32> =
                            (work.as_pointer() as Ptr<i32>).offset((n) as isize);
                        equal_37(___first1, ___last1, ___first2, ___last2)
                    })
                );
            }
            k.prefix_inc();
        }
        n.prefix_inc();
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let mut b: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ({ begin_38(input.as_pointer()) }) });
        let mut e: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ (input.as_pointer() as Ptr<i32>).to_end() });
        let mut it: bidirectional_iterator_int_ptr_ =
            ({ shift_right_46((b).clone(), (e).clone(), 0_isize) });
        assert!(
            ({
                let ___first1: Ptr<i32> = ({ begin_39(expected.as_pointer()) });
                let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                equal_47(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1])));
        let mut b: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ({ begin_38(input.as_pointer()) }) });
        let mut e: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ (input.as_pointer() as Ptr<i32>).to_end() });
        let mut it: bidirectional_iterator_int_ptr_ =
            ({ shift_right_46((b).clone(), (e).clone(), 1_isize) });
        assert!(
            ({
                let ___first1: Ptr<i32> = ({ begin_41(expected.as_pointer()) });
                let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                equal_47(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6, 7, 8])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
        let mut b: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ({ begin_42(input.as_pointer()) }) });
        let mut e: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ (input.as_pointer() as Ptr<i32>).to_end() });
        let mut it: bidirectional_iterator_int_ptr_ =
            ({ shift_right_46((b).clone(), (e).clone(), 2_isize) });
        assert!(
            ({
                let ___first1: Ptr<i32> = ({ begin_43(expected.as_pointer()) });
                let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                equal_47(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6, 7, 8])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
        let mut b: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ({ begin_42(input.as_pointer()) }) });
        let mut e: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ (input.as_pointer() as Ptr<i32>).to_end() });
        let mut it: bidirectional_iterator_int_ptr_ =
            ({ shift_right_46((b).clone(), (e).clone(), 6_isize) });
        assert!(
            ({
                let ___first1: Ptr<i32> = ({ begin_41(expected.as_pointer()) });
                let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                equal_47(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let mut b: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ({ begin_38(input.as_pointer()) }) });
        let e: Value<bidirectional_iterator_int_ptr_> =
            Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                (input.as_pointer() as Ptr<i32>).to_end()
            })));
        let it: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                shift_right_46(
                    (b).clone(),
                    (*e.borrow()).clone(),
                    (({ size_44(input.as_pointer()) }) as isize),
                )
            }),
        ));
        assert!(
            ({
                ({
                    let ___first1: Ptr<i32> = ({ begin_39(expected.as_pointer()) });
                    let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                    equal_47(___first1, ___last1, (b).clone(), (*e.borrow()).clone())
                })
            })
        );
        assert!(
            ({
                let _x: Ptr<bidirectional_iterator_int_ptr_> = it.as_pointer();
                operator_eq_4(_x, e.as_pointer())
            })
        );
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let mut b: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ({ begin_38(input.as_pointer()) }) });
        let e: Value<bidirectional_iterator_int_ptr_> =
            Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                (input.as_pointer() as Ptr<i32>).to_end()
            })));
        let it: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                shift_right_46(
                    (b).clone(),
                    (*e.borrow()).clone(),
                    (((({ size_44(input.as_pointer()) }) as usize).wrapping_add(1_usize)) as isize),
                )
            }),
        ));
        assert!(
            ({
                ({
                    let ___first1: Ptr<i32> = ({ begin_39(expected.as_pointer()) });
                    let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                    equal_47(___first1, ___last1, (b).clone(), (*e.borrow()).clone())
                })
            })
        );
        assert!(
            ({
                let _x: Ptr<bidirectional_iterator_int_ptr_> = it.as_pointer();
                operator_eq_4(_x, e.as_pointer())
            })
        );
    }
    return true;
}
pub fn test_48() -> bool {
    let orig: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9,
    ])));
    let work: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9,
    ])));
    let mut n: i32 = 0;
    'loop_: while (n <= 15) {
        let mut k: i32 = 0;
        'loop_: while (k <= (n + 2)) {
            match ({ (orig.as_pointer() as Ptr<i32>).offset((n) as isize) },) {
                (mut __tmp_2,) => {
                    let mut __first = (orig.as_pointer() as Ptr<i32>);
                    let mut __out = (work.as_pointer() as Ptr<i32>);
                    while __first != __tmp_2 {
                        __out.iter_deref().write((__first.iter_deref().read()));
                        __first.iter_inc();
                        __out.iter_inc();
                    }
                    __out
                }
            };
            let it: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
                ({
                    let ___first: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ (work.as_pointer() as Ptr<i32>) });
                    let ___last: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({
                            (work.as_pointer() as Ptr<i32>).offset((n) as isize)
                        });
                    shift_right_49(___first, ___last, (k as isize))
                }),
            ));
            if (k < n) {
                assert!(
                    ({
                        let _x: Ptr<random_access_iterator_int_ptr_> = it.as_pointer();
                        let _y: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (work.as_pointer() as Ptr<i32>).offset((k) as isize)
                            })));
                        operator_eq_9(_x, _y.as_pointer())
                    })
                );
                assert!(
                    ({
                        let ___first1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>);
                        let ___last1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>)
                            .offset((n) as isize)
                            .offset(-((k) as isize));
                        let ___first2: Ptr<i32> =
                            (work.as_pointer() as Ptr<i32>).offset((k) as isize);
                        let ___last2: Ptr<i32> =
                            (work.as_pointer() as Ptr<i32>).offset((n) as isize);
                        equal_37(___first1, ___last1, ___first2, ___last2)
                    })
                );
            } else {
                assert!(
                    ({
                        let _x: Ptr<random_access_iterator_int_ptr_> = it.as_pointer();
                        let _y: Value<random_access_iterator_int_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                                (work.as_pointer() as Ptr<i32>).offset((n) as isize)
                            })));
                        operator_eq_9(_x, _y.as_pointer())
                    })
                );
                assert!(
                    ({
                        let ___first1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>);
                        let ___last1: Ptr<i32> =
                            (orig.as_pointer() as Ptr<i32>).offset((n) as isize);
                        let ___first2: Ptr<i32> = (work.as_pointer() as Ptr<i32>);
                        let ___last2: Ptr<i32> =
                            (work.as_pointer() as Ptr<i32>).offset((n) as isize);
                        equal_37(___first1, ___last1, ___first2, ___last2)
                    })
                );
            }
            k.prefix_inc();
        }
        n.prefix_inc();
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let mut b: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ({ begin_38(input.as_pointer()) }) });
        let mut e: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ (input.as_pointer() as Ptr<i32>).to_end() });
        let mut it: random_access_iterator_int_ptr_ =
            ({ shift_right_49((b).clone(), (e).clone(), 0_isize) });
        assert!(
            ({
                let ___first1: Ptr<i32> = ({ begin_39(expected.as_pointer()) });
                let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                equal_50(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1])));
        let mut b: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ({ begin_38(input.as_pointer()) }) });
        let mut e: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ (input.as_pointer() as Ptr<i32>).to_end() });
        let mut it: random_access_iterator_int_ptr_ =
            ({ shift_right_49((b).clone(), (e).clone(), 1_isize) });
        assert!(
            ({
                let ___first1: Ptr<i32> = ({ begin_41(expected.as_pointer()) });
                let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                equal_50(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6, 7, 8])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
        let mut b: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ({ begin_42(input.as_pointer()) }) });
        let mut e: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ (input.as_pointer() as Ptr<i32>).to_end() });
        let mut it: random_access_iterator_int_ptr_ =
            ({ shift_right_49((b).clone(), (e).clone(), 2_isize) });
        assert!(
            ({
                let ___first1: Ptr<i32> = ({ begin_43(expected.as_pointer()) });
                let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                equal_50(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6, 7, 8])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
        let mut b: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ({ begin_42(input.as_pointer()) }) });
        let mut e: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ (input.as_pointer() as Ptr<i32>).to_end() });
        let mut it: random_access_iterator_int_ptr_ =
            ({ shift_right_49((b).clone(), (e).clone(), 6_isize) });
        assert!(
            ({
                let ___first1: Ptr<i32> = ({ begin_41(expected.as_pointer()) });
                let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                equal_50(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let mut b: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ({ begin_38(input.as_pointer()) }) });
        let e: Value<random_access_iterator_int_ptr_> =
            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                (input.as_pointer() as Ptr<i32>).to_end()
            })));
        let it: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                shift_right_49(
                    (b).clone(),
                    (*e.borrow()).clone(),
                    (({ size_44(input.as_pointer()) }) as isize),
                )
            }),
        ));
        assert!(
            ({
                ({
                    let ___first1: Ptr<i32> = ({ begin_39(expected.as_pointer()) });
                    let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                    equal_50(___first1, ___last1, (b).clone(), (*e.borrow()).clone())
                })
            })
        );
        assert!(
            ({
                let _x: Ptr<random_access_iterator_int_ptr_> = it.as_pointer();
                operator_eq_9(_x, e.as_pointer())
            })
        );
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let mut b: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ({ begin_38(input.as_pointer()) }) });
        let e: Value<random_access_iterator_int_ptr_> =
            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                (input.as_pointer() as Ptr<i32>).to_end()
            })));
        let it: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                shift_right_49(
                    (b).clone(),
                    (*e.borrow()).clone(),
                    (((({ size_44(input.as_pointer()) }) as usize).wrapping_add(1_usize)) as isize),
                )
            }),
        ));
        assert!(
            ({
                ({
                    let ___first1: Ptr<i32> = ({ begin_39(expected.as_pointer()) });
                    let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                    equal_50(___first1, ___last1, (b).clone(), (*e.borrow()).clone())
                })
            })
        );
        assert!(
            ({
                let _x: Ptr<random_access_iterator_int_ptr_> = it.as_pointer();
                operator_eq_9(_x, e.as_pointer())
            })
        );
    }
    return true;
}
pub fn test_51() -> bool {
    let orig: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9,
    ])));
    let work: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9,
    ])));
    let mut n: i32 = 0;
    'loop_: while (n <= 15) {
        let mut k: i32 = 0;
        'loop_: while (k <= (n + 2)) {
            match ({ (orig.as_pointer() as Ptr<i32>).offset((n) as isize) },) {
                (mut __tmp_3,) => {
                    let mut __first = (orig.as_pointer() as Ptr<i32>);
                    let mut __out = (work.as_pointer() as Ptr<i32>);
                    while __first != __tmp_3 {
                        __out.iter_deref().write((__first.iter_deref().read()));
                        __first.iter_inc();
                        __out.iter_inc();
                    }
                    __out
                }
            };
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = (work.as_pointer() as Ptr<i32>);
                let ___last: Ptr<i32> = (work.as_pointer() as Ptr<i32>).offset((n) as isize);
                shift_right_52(___first, ___last, (k as isize))
            });
            if (k < n) {
                assert!(
                    ({ (it).clone() } == {
                        (work.as_pointer() as Ptr<i32>).offset((k) as isize)
                    })
                );
                assert!(
                    ({
                        let ___first1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>);
                        let ___last1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>)
                            .offset((n) as isize)
                            .offset(-((k) as isize));
                        let ___first2: Ptr<i32> =
                            (work.as_pointer() as Ptr<i32>).offset((k) as isize);
                        let ___last2: Ptr<i32> =
                            (work.as_pointer() as Ptr<i32>).offset((n) as isize);
                        equal_37(___first1, ___last1, ___first2, ___last2)
                    })
                );
            } else {
                assert!(
                    ({ (it).clone() } == {
                        (work.as_pointer() as Ptr<i32>).offset((n) as isize)
                    })
                );
                assert!(
                    ({
                        let ___first1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>);
                        let ___last1: Ptr<i32> =
                            (orig.as_pointer() as Ptr<i32>).offset((n) as isize);
                        let ___first2: Ptr<i32> = (work.as_pointer() as Ptr<i32>);
                        let ___last2: Ptr<i32> =
                            (work.as_pointer() as Ptr<i32>).offset((n) as isize);
                        equal_37(___first1, ___last1, ___first2, ___last2)
                    })
                );
            }
            k.prefix_inc();
        }
        n.prefix_inc();
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let mut b: Ptr<i32> = ({ begin_38(input.as_pointer()) });
        let mut e: Ptr<i32> = (input.as_pointer() as Ptr<i32>).to_end();
        let mut it: Ptr<i32> = ({ shift_right_52((b).clone(), (e).clone(), 0_isize) });
        assert!(
            ({
                let ___first1: Ptr<i32> = ({ begin_39(expected.as_pointer()) });
                let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                equal_53(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1])));
        let mut b: Ptr<i32> = ({ begin_38(input.as_pointer()) });
        let mut e: Ptr<i32> = (input.as_pointer() as Ptr<i32>).to_end();
        let mut it: Ptr<i32> = ({ shift_right_52((b).clone(), (e).clone(), 1_isize) });
        assert!(
            ({
                let ___first1: Ptr<i32> = ({ begin_41(expected.as_pointer()) });
                let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                equal_53(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6, 7, 8])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
        let mut b: Ptr<i32> = ({ begin_42(input.as_pointer()) });
        let mut e: Ptr<i32> = (input.as_pointer() as Ptr<i32>).to_end();
        let mut it: Ptr<i32> = ({ shift_right_52((b).clone(), (e).clone(), 2_isize) });
        assert!(
            ({
                let ___first1: Ptr<i32> = ({ begin_43(expected.as_pointer()) });
                let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                equal_53(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6, 7, 8])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
        let mut b: Ptr<i32> = ({ begin_42(input.as_pointer()) });
        let mut e: Ptr<i32> = (input.as_pointer() as Ptr<i32>).to_end();
        let mut it: Ptr<i32> = ({ shift_right_52((b).clone(), (e).clone(), 6_isize) });
        assert!(
            ({
                let ___first1: Ptr<i32> = ({ begin_41(expected.as_pointer()) });
                let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                equal_53(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let mut b: Ptr<i32> = ({ begin_38(input.as_pointer()) });
        let mut e: Ptr<i32> = (input.as_pointer() as Ptr<i32>).to_end();
        let mut it: Ptr<i32> = ({
            shift_right_52(
                (b).clone(),
                (e).clone(),
                (({ size_44(input.as_pointer()) }) as isize),
            )
        });
        assert!(
            ({
                let ___first1: Ptr<i32> = ({ begin_39(expected.as_pointer()) });
                let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                equal_53(___first1, ___last1, (b).clone(), (e).clone())
            })
        );
        assert!(({ (it).clone() } == { (e).clone() }));
    }
    {
        let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let mut b: Ptr<i32> = ({ begin_38(input.as_pointer()) });
        let mut e: Ptr<i32> = (input.as_pointer() as Ptr<i32>).to_end();
        let mut it: Ptr<i32> = ({
            shift_right_52(
                (b).clone(),
                (e).clone(),
                (((({ size_44(input.as_pointer()) }) as usize).wrapping_add(1_usize)) as isize),
            )
        });
        assert!(
            ({
                let ___first1: Ptr<i32> = ({ begin_39(expected.as_pointer()) });
                let ___last1: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
                equal_53(___first1, ___last1, (b).clone(), (e).clone())
            })
        );
        assert!(({ (it).clone() } == { (e).clone() }));
    }
    return true;
}
pub fn test_54() -> bool {
    let orig: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9,
    ])));
    let work: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
        MoveOnly::new({ Some(3) }),
        MoveOnly::new({ Some(1) }),
        MoveOnly::new({ Some(4) }),
        MoveOnly::new({ Some(1) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(9) }),
        MoveOnly::new({ Some(2) }),
        MoveOnly::new({ Some(6) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(3) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(8) }),
        MoveOnly::new({ Some(9) }),
        MoveOnly::new({ Some(7) }),
        MoveOnly::new({ Some(9) }),
    ])));
    let mut n: i32 = 0;
    'loop_: while (n <= 15) {
        let mut k: i32 = 0;
        'loop_: while (k <= (n + 2)) {
            match ({ (orig.as_pointer() as Ptr<i32>).offset((n) as isize) },) {
                (mut __tmp_4,) => {
                    let mut __first = (orig.as_pointer() as Ptr<i32>);
                    let mut __out = (work.as_pointer() as Ptr<MoveOnly>);
                    while __first != __tmp_4 {
                        __out
                            .iter_deref()
                            .write(<MoveOnly as CppFrom<i32>>::cpp_from(__first.iter_deref()));
                        __first.iter_inc();
                        __out.iter_inc();
                    }
                    __out
                }
            };
            let it: Value<forward_iterator_MoveOnly_ptr_> = Rc::new(RefCell::new(
                ({
                    let ___first: forward_iterator_MoveOnly_ptr_ =
                        forward_iterator_MoveOnly_ptr_::new_1({
                            (work.as_pointer() as Ptr<MoveOnly>)
                        });
                    let ___last: forward_iterator_MoveOnly_ptr_ =
                        forward_iterator_MoveOnly_ptr_::new_1({
                            (work.as_pointer() as Ptr<MoveOnly>).offset((n) as isize)
                        });
                    shift_right_55(___first, ___last, (k as isize))
                }),
            ));
            if (k < n) {
                assert!(
                    ({
                        let _x: Ptr<forward_iterator_MoveOnly_ptr_> = it.as_pointer();
                        let _y: Value<forward_iterator_MoveOnly_ptr_> =
                            Rc::new(RefCell::new(forward_iterator_MoveOnly_ptr_::new_1({
                                (work.as_pointer() as Ptr<MoveOnly>).offset((k) as isize)
                            })));
                        operator_eq_2(_x, _y.as_pointer())
                    })
                );
                assert!(
                    ({
                        let ___first1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>);
                        let ___last1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>)
                            .offset((n) as isize)
                            .offset(-((k) as isize));
                        let ___first2: Ptr<MoveOnly> =
                            (work.as_pointer() as Ptr<MoveOnly>).offset((k) as isize);
                        let ___last2: Ptr<MoveOnly> =
                            (work.as_pointer() as Ptr<MoveOnly>).offset((n) as isize);
                        equal_56(___first1, ___last1, ___first2, ___last2)
                    })
                );
            } else {
                assert!(
                    ({
                        let _x: Ptr<forward_iterator_MoveOnly_ptr_> = it.as_pointer();
                        let _y: Value<forward_iterator_MoveOnly_ptr_> =
                            Rc::new(RefCell::new(forward_iterator_MoveOnly_ptr_::new_1({
                                (work.as_pointer() as Ptr<MoveOnly>).offset((n) as isize)
                            })));
                        operator_eq_2(_x, _y.as_pointer())
                    })
                );
                assert!(
                    ({
                        let ___first1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>);
                        let ___last1: Ptr<i32> =
                            (orig.as_pointer() as Ptr<i32>).offset((n) as isize);
                        let ___first2: Ptr<MoveOnly> = (work.as_pointer() as Ptr<MoveOnly>);
                        let ___last2: Ptr<MoveOnly> =
                            (work.as_pointer() as Ptr<MoveOnly>).offset((n) as isize);
                        equal_56(___first1, ___last1, ___first2, ___last2)
                    })
                );
            }
            k.prefix_inc();
        }
        n.prefix_inc();
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let mut b: forward_iterator_MoveOnly_ptr_ =
            forward_iterator_MoveOnly_ptr_::new_1({ ({ begin_57(input.as_pointer()) }) });
        let mut e: forward_iterator_MoveOnly_ptr_ = forward_iterator_MoveOnly_ptr_::new_1({
            (input.as_pointer() as Ptr<MoveOnly>).to_end()
        });
        let mut it: forward_iterator_MoveOnly_ptr_ =
            ({ shift_right_55((b).clone(), (e).clone(), 0_isize) });
        assert!(
            ({
                let ___first1: Ptr<MoveOnly> = ({ begin_58(expected.as_pointer()) });
                let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                equal_59(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
        ])));
        let mut b: forward_iterator_MoveOnly_ptr_ =
            forward_iterator_MoveOnly_ptr_::new_1({ ({ begin_57(input.as_pointer()) }) });
        let mut e: forward_iterator_MoveOnly_ptr_ = forward_iterator_MoveOnly_ptr_::new_1({
            (input.as_pointer() as Ptr<MoveOnly>).to_end()
        });
        let mut it: forward_iterator_MoveOnly_ptr_ =
            ({ shift_right_55((b).clone(), (e).clone(), 1_isize) });
        assert!(
            ({
                let ___first1: Ptr<MoveOnly> = ({ begin_60(expected.as_pointer()) });
                let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                equal_59(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
        ])));
        let mut b: forward_iterator_MoveOnly_ptr_ =
            forward_iterator_MoveOnly_ptr_::new_1({ ({ begin_61(input.as_pointer()) }) });
        let mut e: forward_iterator_MoveOnly_ptr_ = forward_iterator_MoveOnly_ptr_::new_1({
            (input.as_pointer() as Ptr<MoveOnly>).to_end()
        });
        let mut it: forward_iterator_MoveOnly_ptr_ =
            ({ shift_right_55((b).clone(), (e).clone(), 2_isize) });
        assert!(
            ({
                let ___first1: Ptr<MoveOnly> = ({ begin_62(expected.as_pointer()) });
                let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                equal_59(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let mut b: forward_iterator_MoveOnly_ptr_ =
            forward_iterator_MoveOnly_ptr_::new_1({ ({ begin_61(input.as_pointer()) }) });
        let mut e: forward_iterator_MoveOnly_ptr_ = forward_iterator_MoveOnly_ptr_::new_1({
            (input.as_pointer() as Ptr<MoveOnly>).to_end()
        });
        let mut it: forward_iterator_MoveOnly_ptr_ =
            ({ shift_right_55((b).clone(), (e).clone(), 6_isize) });
        assert!(
            ({
                let ___first1: Ptr<MoveOnly> = ({ begin_60(expected.as_pointer()) });
                let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                equal_59(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let mut b: forward_iterator_MoveOnly_ptr_ =
            forward_iterator_MoveOnly_ptr_::new_1({ ({ begin_57(input.as_pointer()) }) });
        let e: Value<forward_iterator_MoveOnly_ptr_> =
            Rc::new(RefCell::new(forward_iterator_MoveOnly_ptr_::new_1({
                (input.as_pointer() as Ptr<MoveOnly>).to_end()
            })));
        let it: Value<forward_iterator_MoveOnly_ptr_> = Rc::new(RefCell::new(
            ({
                shift_right_55(
                    (b).clone(),
                    (*e.borrow()).clone(),
                    (({ size_63(input.as_pointer()) }) as isize),
                )
            }),
        ));
        assert!(
            ({
                ({
                    let ___first1: Ptr<MoveOnly> = ({ begin_58(expected.as_pointer()) });
                    let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                    equal_59(___first1, ___last1, (b).clone(), (*e.borrow()).clone())
                })
            })
        );
        assert!(
            ({
                let _x: Ptr<forward_iterator_MoveOnly_ptr_> = it.as_pointer();
                operator_eq_2(_x, e.as_pointer())
            })
        );
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let mut b: forward_iterator_MoveOnly_ptr_ =
            forward_iterator_MoveOnly_ptr_::new_1({ ({ begin_57(input.as_pointer()) }) });
        let e: Value<forward_iterator_MoveOnly_ptr_> =
            Rc::new(RefCell::new(forward_iterator_MoveOnly_ptr_::new_1({
                (input.as_pointer() as Ptr<MoveOnly>).to_end()
            })));
        let it: Value<forward_iterator_MoveOnly_ptr_> = Rc::new(RefCell::new(
            ({
                shift_right_55(
                    (b).clone(),
                    (*e.borrow()).clone(),
                    (((({ size_63(input.as_pointer()) }) as usize).wrapping_add(1_usize)) as isize),
                )
            }),
        ));
        assert!(
            ({
                ({
                    let ___first1: Ptr<MoveOnly> = ({ begin_58(expected.as_pointer()) });
                    let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                    equal_59(___first1, ___last1, (b).clone(), (*e.borrow()).clone())
                })
            })
        );
        assert!(
            ({
                let _x: Ptr<forward_iterator_MoveOnly_ptr_> = it.as_pointer();
                operator_eq_2(_x, e.as_pointer())
            })
        );
    }
    return true;
}
pub fn test_64() -> bool {
    let orig: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9,
    ])));
    let work: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
        MoveOnly::new({ Some(3) }),
        MoveOnly::new({ Some(1) }),
        MoveOnly::new({ Some(4) }),
        MoveOnly::new({ Some(1) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(9) }),
        MoveOnly::new({ Some(2) }),
        MoveOnly::new({ Some(6) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(3) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(8) }),
        MoveOnly::new({ Some(9) }),
        MoveOnly::new({ Some(7) }),
        MoveOnly::new({ Some(9) }),
    ])));
    let mut n: i32 = 0;
    'loop_: while (n <= 15) {
        let mut k: i32 = 0;
        'loop_: while (k <= (n + 2)) {
            match ({ (orig.as_pointer() as Ptr<i32>).offset((n) as isize) },) {
                (mut __tmp_5,) => {
                    let mut __first = (orig.as_pointer() as Ptr<i32>);
                    let mut __out = (work.as_pointer() as Ptr<MoveOnly>);
                    while __first != __tmp_5 {
                        __out
                            .iter_deref()
                            .write(<MoveOnly as CppFrom<i32>>::cpp_from(__first.iter_deref()));
                        __first.iter_inc();
                        __out.iter_inc();
                    }
                    __out
                }
            };
            let it: Value<bidirectional_iterator_MoveOnly_ptr_> = Rc::new(RefCell::new(
                ({
                    let ___first: bidirectional_iterator_MoveOnly_ptr_ =
                        bidirectional_iterator_MoveOnly_ptr_::new_1({
                            (work.as_pointer() as Ptr<MoveOnly>)
                        });
                    let ___last: bidirectional_iterator_MoveOnly_ptr_ =
                        bidirectional_iterator_MoveOnly_ptr_::new_1({
                            (work.as_pointer() as Ptr<MoveOnly>).offset((n) as isize)
                        });
                    shift_right_65(___first, ___last, (k as isize))
                }),
            ));
            if (k < n) {
                assert!(
                    ({
                        let _x: Ptr<bidirectional_iterator_MoveOnly_ptr_> = it.as_pointer();
                        let _y: Value<bidirectional_iterator_MoveOnly_ptr_> =
                            Rc::new(RefCell::new(bidirectional_iterator_MoveOnly_ptr_::new_1({
                                (work.as_pointer() as Ptr<MoveOnly>).offset((k) as isize)
                            })));
                        operator_eq_6(_x, _y.as_pointer())
                    })
                );
                assert!(
                    ({
                        let ___first1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>);
                        let ___last1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>)
                            .offset((n) as isize)
                            .offset(-((k) as isize));
                        let ___first2: Ptr<MoveOnly> =
                            (work.as_pointer() as Ptr<MoveOnly>).offset((k) as isize);
                        let ___last2: Ptr<MoveOnly> =
                            (work.as_pointer() as Ptr<MoveOnly>).offset((n) as isize);
                        equal_56(___first1, ___last1, ___first2, ___last2)
                    })
                );
            } else {
                assert!(
                    ({
                        let _x: Ptr<bidirectional_iterator_MoveOnly_ptr_> = it.as_pointer();
                        let _y: Value<bidirectional_iterator_MoveOnly_ptr_> =
                            Rc::new(RefCell::new(bidirectional_iterator_MoveOnly_ptr_::new_1({
                                (work.as_pointer() as Ptr<MoveOnly>).offset((n) as isize)
                            })));
                        operator_eq_6(_x, _y.as_pointer())
                    })
                );
                assert!(
                    ({
                        let ___first1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>);
                        let ___last1: Ptr<i32> =
                            (orig.as_pointer() as Ptr<i32>).offset((n) as isize);
                        let ___first2: Ptr<MoveOnly> = (work.as_pointer() as Ptr<MoveOnly>);
                        let ___last2: Ptr<MoveOnly> =
                            (work.as_pointer() as Ptr<MoveOnly>).offset((n) as isize);
                        equal_56(___first1, ___last1, ___first2, ___last2)
                    })
                );
            }
            k.prefix_inc();
        }
        n.prefix_inc();
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let mut b: bidirectional_iterator_MoveOnly_ptr_ =
            bidirectional_iterator_MoveOnly_ptr_::new_1({ ({ begin_57(input.as_pointer()) }) });
        let mut e: bidirectional_iterator_MoveOnly_ptr_ =
            bidirectional_iterator_MoveOnly_ptr_::new_1({
                (input.as_pointer() as Ptr<MoveOnly>).to_end()
            });
        let mut it: bidirectional_iterator_MoveOnly_ptr_ =
            ({ shift_right_65((b).clone(), (e).clone(), 0_isize) });
        assert!(
            ({
                let ___first1: Ptr<MoveOnly> = ({ begin_58(expected.as_pointer()) });
                let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                equal_66(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
        ])));
        let mut b: bidirectional_iterator_MoveOnly_ptr_ =
            bidirectional_iterator_MoveOnly_ptr_::new_1({ ({ begin_57(input.as_pointer()) }) });
        let mut e: bidirectional_iterator_MoveOnly_ptr_ =
            bidirectional_iterator_MoveOnly_ptr_::new_1({
                (input.as_pointer() as Ptr<MoveOnly>).to_end()
            });
        let mut it: bidirectional_iterator_MoveOnly_ptr_ =
            ({ shift_right_65((b).clone(), (e).clone(), 1_isize) });
        assert!(
            ({
                let ___first1: Ptr<MoveOnly> = ({ begin_60(expected.as_pointer()) });
                let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                equal_66(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
        ])));
        let mut b: bidirectional_iterator_MoveOnly_ptr_ =
            bidirectional_iterator_MoveOnly_ptr_::new_1({ ({ begin_61(input.as_pointer()) }) });
        let mut e: bidirectional_iterator_MoveOnly_ptr_ =
            bidirectional_iterator_MoveOnly_ptr_::new_1({
                (input.as_pointer() as Ptr<MoveOnly>).to_end()
            });
        let mut it: bidirectional_iterator_MoveOnly_ptr_ =
            ({ shift_right_65((b).clone(), (e).clone(), 2_isize) });
        assert!(
            ({
                let ___first1: Ptr<MoveOnly> = ({ begin_62(expected.as_pointer()) });
                let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                equal_66(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let mut b: bidirectional_iterator_MoveOnly_ptr_ =
            bidirectional_iterator_MoveOnly_ptr_::new_1({ ({ begin_61(input.as_pointer()) }) });
        let mut e: bidirectional_iterator_MoveOnly_ptr_ =
            bidirectional_iterator_MoveOnly_ptr_::new_1({
                (input.as_pointer() as Ptr<MoveOnly>).to_end()
            });
        let mut it: bidirectional_iterator_MoveOnly_ptr_ =
            ({ shift_right_65((b).clone(), (e).clone(), 6_isize) });
        assert!(
            ({
                let ___first1: Ptr<MoveOnly> = ({ begin_60(expected.as_pointer()) });
                let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                equal_66(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let mut b: bidirectional_iterator_MoveOnly_ptr_ =
            bidirectional_iterator_MoveOnly_ptr_::new_1({ ({ begin_57(input.as_pointer()) }) });
        let e: Value<bidirectional_iterator_MoveOnly_ptr_> =
            Rc::new(RefCell::new(bidirectional_iterator_MoveOnly_ptr_::new_1({
                (input.as_pointer() as Ptr<MoveOnly>).to_end()
            })));
        let it: Value<bidirectional_iterator_MoveOnly_ptr_> = Rc::new(RefCell::new(
            ({
                shift_right_65(
                    (b).clone(),
                    (*e.borrow()).clone(),
                    (({ size_63(input.as_pointer()) }) as isize),
                )
            }),
        ));
        assert!(
            ({
                ({
                    let ___first1: Ptr<MoveOnly> = ({ begin_58(expected.as_pointer()) });
                    let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                    equal_66(___first1, ___last1, (b).clone(), (*e.borrow()).clone())
                })
            })
        );
        assert!(
            ({
                let _x: Ptr<bidirectional_iterator_MoveOnly_ptr_> = it.as_pointer();
                operator_eq_6(_x, e.as_pointer())
            })
        );
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let mut b: bidirectional_iterator_MoveOnly_ptr_ =
            bidirectional_iterator_MoveOnly_ptr_::new_1({ ({ begin_57(input.as_pointer()) }) });
        let e: Value<bidirectional_iterator_MoveOnly_ptr_> =
            Rc::new(RefCell::new(bidirectional_iterator_MoveOnly_ptr_::new_1({
                (input.as_pointer() as Ptr<MoveOnly>).to_end()
            })));
        let it: Value<bidirectional_iterator_MoveOnly_ptr_> = Rc::new(RefCell::new(
            ({
                shift_right_65(
                    (b).clone(),
                    (*e.borrow()).clone(),
                    (((({ size_63(input.as_pointer()) }) as usize).wrapping_add(1_usize)) as isize),
                )
            }),
        ));
        assert!(
            ({
                ({
                    let ___first1: Ptr<MoveOnly> = ({ begin_58(expected.as_pointer()) });
                    let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                    equal_66(___first1, ___last1, (b).clone(), (*e.borrow()).clone())
                })
            })
        );
        assert!(
            ({
                let _x: Ptr<bidirectional_iterator_MoveOnly_ptr_> = it.as_pointer();
                operator_eq_6(_x, e.as_pointer())
            })
        );
    }
    return true;
}
pub fn test_67() -> bool {
    let orig: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9,
    ])));
    let work: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
        MoveOnly::new({ Some(3) }),
        MoveOnly::new({ Some(1) }),
        MoveOnly::new({ Some(4) }),
        MoveOnly::new({ Some(1) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(9) }),
        MoveOnly::new({ Some(2) }),
        MoveOnly::new({ Some(6) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(3) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(8) }),
        MoveOnly::new({ Some(9) }),
        MoveOnly::new({ Some(7) }),
        MoveOnly::new({ Some(9) }),
    ])));
    let mut n: i32 = 0;
    'loop_: while (n <= 15) {
        let mut k: i32 = 0;
        'loop_: while (k <= (n + 2)) {
            match ({ (orig.as_pointer() as Ptr<i32>).offset((n) as isize) },) {
                (mut __tmp_6,) => {
                    let mut __first = (orig.as_pointer() as Ptr<i32>);
                    let mut __out = (work.as_pointer() as Ptr<MoveOnly>);
                    while __first != __tmp_6 {
                        __out
                            .iter_deref()
                            .write(<MoveOnly as CppFrom<i32>>::cpp_from(__first.iter_deref()));
                        __first.iter_inc();
                        __out.iter_inc();
                    }
                    __out
                }
            };
            let it: Value<random_access_iterator_MoveOnly_ptr_> = Rc::new(RefCell::new(
                ({
                    let ___first: random_access_iterator_MoveOnly_ptr_ =
                        random_access_iterator_MoveOnly_ptr_::new_1({
                            (work.as_pointer() as Ptr<MoveOnly>)
                        });
                    let ___last: random_access_iterator_MoveOnly_ptr_ =
                        random_access_iterator_MoveOnly_ptr_::new_1({
                            (work.as_pointer() as Ptr<MoveOnly>).offset((n) as isize)
                        });
                    shift_right_68(___first, ___last, (k as isize))
                }),
            ));
            if (k < n) {
                assert!(
                    ({
                        let _x: Ptr<random_access_iterator_MoveOnly_ptr_> = it.as_pointer();
                        let _y: Value<random_access_iterator_MoveOnly_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_MoveOnly_ptr_::new_1({
                                (work.as_pointer() as Ptr<MoveOnly>).offset((k) as isize)
                            })));
                        operator_eq_12(_x, _y.as_pointer())
                    })
                );
                assert!(
                    ({
                        let ___first1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>);
                        let ___last1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>)
                            .offset((n) as isize)
                            .offset(-((k) as isize));
                        let ___first2: Ptr<MoveOnly> =
                            (work.as_pointer() as Ptr<MoveOnly>).offset((k) as isize);
                        let ___last2: Ptr<MoveOnly> =
                            (work.as_pointer() as Ptr<MoveOnly>).offset((n) as isize);
                        equal_56(___first1, ___last1, ___first2, ___last2)
                    })
                );
            } else {
                assert!(
                    ({
                        let _x: Ptr<random_access_iterator_MoveOnly_ptr_> = it.as_pointer();
                        let _y: Value<random_access_iterator_MoveOnly_ptr_> =
                            Rc::new(RefCell::new(random_access_iterator_MoveOnly_ptr_::new_1({
                                (work.as_pointer() as Ptr<MoveOnly>).offset((n) as isize)
                            })));
                        operator_eq_12(_x, _y.as_pointer())
                    })
                );
                assert!(
                    ({
                        let ___first1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>);
                        let ___last1: Ptr<i32> =
                            (orig.as_pointer() as Ptr<i32>).offset((n) as isize);
                        let ___first2: Ptr<MoveOnly> = (work.as_pointer() as Ptr<MoveOnly>);
                        let ___last2: Ptr<MoveOnly> =
                            (work.as_pointer() as Ptr<MoveOnly>).offset((n) as isize);
                        equal_56(___first1, ___last1, ___first2, ___last2)
                    })
                );
            }
            k.prefix_inc();
        }
        n.prefix_inc();
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let mut b: random_access_iterator_MoveOnly_ptr_ =
            random_access_iterator_MoveOnly_ptr_::new_1({ ({ begin_57(input.as_pointer()) }) });
        let mut e: random_access_iterator_MoveOnly_ptr_ =
            random_access_iterator_MoveOnly_ptr_::new_1({
                (input.as_pointer() as Ptr<MoveOnly>).to_end()
            });
        let mut it: random_access_iterator_MoveOnly_ptr_ =
            ({ shift_right_68((b).clone(), (e).clone(), 0_isize) });
        assert!(
            ({
                let ___first1: Ptr<MoveOnly> = ({ begin_58(expected.as_pointer()) });
                let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                equal_69(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
        ])));
        let mut b: random_access_iterator_MoveOnly_ptr_ =
            random_access_iterator_MoveOnly_ptr_::new_1({ ({ begin_57(input.as_pointer()) }) });
        let mut e: random_access_iterator_MoveOnly_ptr_ =
            random_access_iterator_MoveOnly_ptr_::new_1({
                (input.as_pointer() as Ptr<MoveOnly>).to_end()
            });
        let mut it: random_access_iterator_MoveOnly_ptr_ =
            ({ shift_right_68((b).clone(), (e).clone(), 1_isize) });
        assert!(
            ({
                let ___first1: Ptr<MoveOnly> = ({ begin_60(expected.as_pointer()) });
                let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                equal_69(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
        ])));
        let mut b: random_access_iterator_MoveOnly_ptr_ =
            random_access_iterator_MoveOnly_ptr_::new_1({ ({ begin_61(input.as_pointer()) }) });
        let mut e: random_access_iterator_MoveOnly_ptr_ =
            random_access_iterator_MoveOnly_ptr_::new_1({
                (input.as_pointer() as Ptr<MoveOnly>).to_end()
            });
        let mut it: random_access_iterator_MoveOnly_ptr_ =
            ({ shift_right_68((b).clone(), (e).clone(), 2_isize) });
        assert!(
            ({
                let ___first1: Ptr<MoveOnly> = ({ begin_62(expected.as_pointer()) });
                let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                equal_69(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let mut b: random_access_iterator_MoveOnly_ptr_ =
            random_access_iterator_MoveOnly_ptr_::new_1({ ({ begin_61(input.as_pointer()) }) });
        let mut e: random_access_iterator_MoveOnly_ptr_ =
            random_access_iterator_MoveOnly_ptr_::new_1({
                (input.as_pointer() as Ptr<MoveOnly>).to_end()
            });
        let mut it: random_access_iterator_MoveOnly_ptr_ =
            ({ shift_right_68((b).clone(), (e).clone(), 6_isize) });
        assert!(
            ({
                let ___first1: Ptr<MoveOnly> = ({ begin_60(expected.as_pointer()) });
                let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                equal_69(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let mut b: random_access_iterator_MoveOnly_ptr_ =
            random_access_iterator_MoveOnly_ptr_::new_1({ ({ begin_57(input.as_pointer()) }) });
        let e: Value<random_access_iterator_MoveOnly_ptr_> =
            Rc::new(RefCell::new(random_access_iterator_MoveOnly_ptr_::new_1({
                (input.as_pointer() as Ptr<MoveOnly>).to_end()
            })));
        let it: Value<random_access_iterator_MoveOnly_ptr_> = Rc::new(RefCell::new(
            ({
                shift_right_68(
                    (b).clone(),
                    (*e.borrow()).clone(),
                    (({ size_63(input.as_pointer()) }) as isize),
                )
            }),
        ));
        assert!(
            ({
                ({
                    let ___first1: Ptr<MoveOnly> = ({ begin_58(expected.as_pointer()) });
                    let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                    equal_69(___first1, ___last1, (b).clone(), (*e.borrow()).clone())
                })
            })
        );
        assert!(
            ({
                let _x: Ptr<random_access_iterator_MoveOnly_ptr_> = it.as_pointer();
                operator_eq_12(_x, e.as_pointer())
            })
        );
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let mut b: random_access_iterator_MoveOnly_ptr_ =
            random_access_iterator_MoveOnly_ptr_::new_1({ ({ begin_57(input.as_pointer()) }) });
        let e: Value<random_access_iterator_MoveOnly_ptr_> =
            Rc::new(RefCell::new(random_access_iterator_MoveOnly_ptr_::new_1({
                (input.as_pointer() as Ptr<MoveOnly>).to_end()
            })));
        let it: Value<random_access_iterator_MoveOnly_ptr_> = Rc::new(RefCell::new(
            ({
                shift_right_68(
                    (b).clone(),
                    (*e.borrow()).clone(),
                    (((({ size_63(input.as_pointer()) }) as usize).wrapping_add(1_usize)) as isize),
                )
            }),
        ));
        assert!(
            ({
                ({
                    let ___first1: Ptr<MoveOnly> = ({ begin_58(expected.as_pointer()) });
                    let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                    equal_69(___first1, ___last1, (b).clone(), (*e.borrow()).clone())
                })
            })
        );
        assert!(
            ({
                let _x: Ptr<random_access_iterator_MoveOnly_ptr_> = it.as_pointer();
                operator_eq_12(_x, e.as_pointer())
            })
        );
    }
    return true;
}
pub fn test_70() -> bool {
    let orig: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9,
    ])));
    let work: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
        MoveOnly::new({ Some(3) }),
        MoveOnly::new({ Some(1) }),
        MoveOnly::new({ Some(4) }),
        MoveOnly::new({ Some(1) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(9) }),
        MoveOnly::new({ Some(2) }),
        MoveOnly::new({ Some(6) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(3) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(8) }),
        MoveOnly::new({ Some(9) }),
        MoveOnly::new({ Some(7) }),
        MoveOnly::new({ Some(9) }),
    ])));
    let mut n: i32 = 0;
    'loop_: while (n <= 15) {
        let mut k: i32 = 0;
        'loop_: while (k <= (n + 2)) {
            match ({ (orig.as_pointer() as Ptr<i32>).offset((n) as isize) },) {
                (mut __tmp_7,) => {
                    let mut __first = (orig.as_pointer() as Ptr<i32>);
                    let mut __out = (work.as_pointer() as Ptr<MoveOnly>);
                    while __first != __tmp_7 {
                        __out
                            .iter_deref()
                            .write(<MoveOnly as CppFrom<i32>>::cpp_from(__first.iter_deref()));
                        __first.iter_inc();
                        __out.iter_inc();
                    }
                    __out
                }
            };
            let mut it: Ptr<MoveOnly> = ({
                let ___first: Ptr<MoveOnly> = (work.as_pointer() as Ptr<MoveOnly>);
                let ___last: Ptr<MoveOnly> =
                    (work.as_pointer() as Ptr<MoveOnly>).offset((n) as isize);
                shift_right_71(___first, ___last, (k as isize))
            });
            if (k < n) {
                assert!(
                    ({ (it).clone() } == {
                        (work.as_pointer() as Ptr<MoveOnly>).offset((k) as isize)
                    })
                );
                assert!(
                    ({
                        let ___first1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>);
                        let ___last1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>)
                            .offset((n) as isize)
                            .offset(-((k) as isize));
                        let ___first2: Ptr<MoveOnly> =
                            (work.as_pointer() as Ptr<MoveOnly>).offset((k) as isize);
                        let ___last2: Ptr<MoveOnly> =
                            (work.as_pointer() as Ptr<MoveOnly>).offset((n) as isize);
                        equal_56(___first1, ___last1, ___first2, ___last2)
                    })
                );
            } else {
                assert!(
                    ({ (it).clone() } == {
                        (work.as_pointer() as Ptr<MoveOnly>).offset((n) as isize)
                    })
                );
                assert!(
                    ({
                        let ___first1: Ptr<i32> = (orig.as_pointer() as Ptr<i32>);
                        let ___last1: Ptr<i32> =
                            (orig.as_pointer() as Ptr<i32>).offset((n) as isize);
                        let ___first2: Ptr<MoveOnly> = (work.as_pointer() as Ptr<MoveOnly>);
                        let ___last2: Ptr<MoveOnly> =
                            (work.as_pointer() as Ptr<MoveOnly>).offset((n) as isize);
                        equal_56(___first1, ___last1, ___first2, ___last2)
                    })
                );
            }
            k.prefix_inc();
        }
        n.prefix_inc();
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let mut b: Ptr<MoveOnly> = ({ begin_57(input.as_pointer()) });
        let mut e: Ptr<MoveOnly> = (input.as_pointer() as Ptr<MoveOnly>).to_end();
        let mut it: Ptr<MoveOnly> = ({ shift_right_71((b).clone(), (e).clone(), 0_isize) });
        assert!(
            ({
                let ___first1: Ptr<MoveOnly> = ({ begin_58(expected.as_pointer()) });
                let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                equal_72(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
        ])));
        let mut b: Ptr<MoveOnly> = ({ begin_57(input.as_pointer()) });
        let mut e: Ptr<MoveOnly> = (input.as_pointer() as Ptr<MoveOnly>).to_end();
        let mut it: Ptr<MoveOnly> = ({ shift_right_71((b).clone(), (e).clone(), 1_isize) });
        assert!(
            ({
                let ___first1: Ptr<MoveOnly> = ({ begin_60(expected.as_pointer()) });
                let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                equal_72(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
        ])));
        let mut b: Ptr<MoveOnly> = ({ begin_61(input.as_pointer()) });
        let mut e: Ptr<MoveOnly> = (input.as_pointer() as Ptr<MoveOnly>).to_end();
        let mut it: Ptr<MoveOnly> = ({ shift_right_71((b).clone(), (e).clone(), 2_isize) });
        assert!(
            ({
                let ___first1: Ptr<MoveOnly> = ({ begin_62(expected.as_pointer()) });
                let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                equal_72(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let mut b: Ptr<MoveOnly> = ({ begin_61(input.as_pointer()) });
        let mut e: Ptr<MoveOnly> = (input.as_pointer() as Ptr<MoveOnly>).to_end();
        let mut it: Ptr<MoveOnly> = ({ shift_right_71((b).clone(), (e).clone(), 6_isize) });
        assert!(
            ({
                let ___first1: Ptr<MoveOnly> = ({ begin_60(expected.as_pointer()) });
                let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                equal_72(___first1, ___last1, (it).clone(), (e).clone())
            })
        );
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let mut b: Ptr<MoveOnly> = ({ begin_57(input.as_pointer()) });
        let mut e: Ptr<MoveOnly> = (input.as_pointer() as Ptr<MoveOnly>).to_end();
        let mut it: Ptr<MoveOnly> = ({
            shift_right_71(
                (b).clone(),
                (e).clone(),
                (({ size_63(input.as_pointer()) }) as isize),
            )
        });
        assert!(
            ({
                let ___first1: Ptr<MoveOnly> = ({ begin_58(expected.as_pointer()) });
                let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                equal_72(___first1, ___last1, (b).clone(), (e).clone())
            })
        );
        assert!(({ (it).clone() } == { (e).clone() }));
    }
    {
        let input: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let expected: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ])));
        let mut b: Ptr<MoveOnly> = ({ begin_57(input.as_pointer()) });
        let mut e: Ptr<MoveOnly> = (input.as_pointer() as Ptr<MoveOnly>).to_end();
        let mut it: Ptr<MoveOnly> = ({
            shift_right_71(
                (b).clone(),
                (e).clone(),
                (((({ size_63(input.as_pointer()) }) as usize).wrapping_add(1_usize)) as isize),
            )
        });
        assert!(
            ({
                let ___first1: Ptr<MoveOnly> = ({ begin_58(expected.as_pointer()) });
                let ___last1: Ptr<MoveOnly> = (expected.as_pointer() as Ptr<MoveOnly>).to_end();
                equal_72(___first1, ___last1, (b).clone(), (e).clone())
            })
        );
        assert!(({ (it).clone() } == { (e).clone() }));
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
    ({ test_35() });
    ({ test_45() });
    ({ test_48() });
    ({ test_51() });
    ({ test_54() });
    ({ test_64() });
    ({ test_67() });
    ({ test_70() });
    const _: () = assert!(true, "test<int, forward_iterator<int*>>()");;
    const _: () = assert!(true, "test<int, bidirectional_iterator<int*>>()");;
    const _: () = assert!(true, "test<int, random_access_iterator<int*>>()");;
    const _: () = assert!(true, "test<int, int*>()");;
    const _: () = assert!(true, "test<MoveOnly, forward_iterator<MoveOnly*>>()");;
    const _: () = assert!(true, "test<MoveOnly, bidirectional_iterator<MoveOnly*>>()");;
    const _: () = assert!(true, "test<MoveOnly, random_access_iterator<MoveOnly*>>()");;
    const _: () = assert!(true, "test<MoveOnly, MoveOnly*>()");;
    return 0;
}
pub trait MoveOnlyImpl {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
    fn get(&self) -> i32;
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly;
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        field!((*self), data_).write({ x.with(|__s| __s.data_) });
        field!(x, data_).write(0);
        return (*self).clone();
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.data_);
    }
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly {
        return MoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } + { x.with(|__s| __s.data_) }))
        });
    }
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly {
        return MoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } * { x.with(|__s| __s.data_) }))
        });
    }
}
pub trait TrivialMoveOnlyImpl {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly>;
    fn get(&self) -> i32;
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
}
impl TrivialMoveOnlyImpl for Ptr<TrivialMoveOnly> {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly> {
        field!((*self), data_).write({ { (*_a0.upgrade().deref()).data_ } });
        return (*self).clone();
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.data_);
    }
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } + { x.with(|__s| __s.data_) }))
        });
    }
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } * { x.with(|__s| __s.data_) }))
        });
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
pub trait bidirectional_iterator_MoveOnly_ptr_Impl {
    fn operator_deref(&self) -> Ptr<MoveOnly>;
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_MoveOnly_ptr_>;
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_MoveOnly_ptr_>;
    fn operator_post_inc_8(&self, mut _a0: i32) -> bidirectional_iterator_MoveOnly_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_9(&self, mut _a0: i32) -> bidirectional_iterator_MoveOnly_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_MoveOnly_ptr_>,
    ) -> Ptr<bidirectional_iterator_MoveOnly_ptr_>;
}
impl bidirectional_iterator_MoveOnly_ptr_Impl for Ptr<bidirectional_iterator_MoveOnly_ptr_> {
    fn operator_deref(&self) -> Ptr<MoveOnly> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_MoveOnly_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_MoveOnly_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_dec();
        });
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_MoveOnly_ptr_>,
    ) -> Ptr<bidirectional_iterator_MoveOnly_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait bidirectional_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_int_ptr_>;
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_int_ptr_>;
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
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_dec();
        });
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
pub trait forward_iterator_MoveOnly_ptr_Impl {
    fn operator_deref(&self) -> Ptr<MoveOnly>;
    fn operator_inc_6(&self) -> Ptr<forward_iterator_MoveOnly_ptr_>;
    fn operator_post_inc_7(&self, mut _a0: i32) -> forward_iterator_MoveOnly_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<forward_iterator_MoveOnly_ptr_>,
    ) -> Ptr<forward_iterator_MoveOnly_ptr_>;
}
impl forward_iterator_MoveOnly_ptr_Impl for Ptr<forward_iterator_MoveOnly_ptr_> {
    fn operator_deref(&self) -> Ptr<MoveOnly> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<forward_iterator_MoveOnly_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<forward_iterator_MoveOnly_ptr_>,
    ) -> Ptr<forward_iterator_MoveOnly_ptr_> {
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
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
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
pub trait random_access_iterator_MoveOnly_ptr_Impl {
    fn operator_deref(&self) -> Ptr<MoveOnly>;
    fn operator_index(&self, mut n: isize) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_MoveOnly_ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_MoveOnly_ptr_>;
    fn operator_post_inc_9(&self, mut _a0: i32) -> random_access_iterator_MoveOnly_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: i32) -> random_access_iterator_MoveOnly_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: isize) -> Ptr<random_access_iterator_MoveOnly_ptr_>;
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<random_access_iterator_MoveOnly_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_MoveOnly_ptr_>,
    ) -> Ptr<random_access_iterator_MoveOnly_ptr_>;
}
impl random_access_iterator_MoveOnly_ptr_Impl for Ptr<random_access_iterator_MoveOnly_ptr_> {
    fn operator_deref(&self) -> Ptr<MoveOnly> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_MoveOnly_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_MoveOnly_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_dec();
        });
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<random_access_iterator_MoveOnly_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_MoveOnly_ptr_>,
    ) -> Ptr<random_access_iterator_MoveOnly_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_int_ptr_>;
    fn operator_post_inc_9(&self, mut _a0: i32) -> random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: i32) -> random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: isize) -> Ptr<random_access_iterator_int_ptr_>;
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
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_dec();
        });
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<random_access_iterator_int_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
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
