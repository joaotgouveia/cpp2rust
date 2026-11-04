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
pub fn base_2(i: Ptr<forward_iterator_int_ptr_>) -> Ptr<i32> {
    return i.with(|__s| __s.it_.clone());
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
pub fn operator_eq_3(
    x: Ptr<forward_iterator_std_unique_ptr_int__do_nothing__ptr_>,
    y: Ptr<forward_iterator_std_unique_ptr_int__do_nothing__ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_4(
    x: Ptr<forward_iterator_std_unique_ptr_int__do_nothing__ptr_>,
    y: Ptr<forward_iterator_std_unique_ptr_int__do_nothing__ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
pub fn base_5(
    i: Ptr<forward_iterator_std_unique_ptr_int__do_nothing__ptr_>,
) -> Ptr<Option<Value<i32>>> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct forward_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<Option<Value<i32>>>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl forward_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    pub fn new_1(mut it: Ptr<Option<Value<i32>>>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<forward_iterator_std_unique_ptr_int__do_nothing__ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for forward_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_3(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for forward_iterator_std_unique_ptr_int__do_nothing__ptr_ {}
impl CppAdvance for forward_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
}
impl CppIterator<Option<Value<i32>>> for forward_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    fn iter_deref(&self) -> Ptr<Option<Value<i32>>> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            forward_iterator_std_unique_ptr_int__do_nothing__ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            forward_iterator_std_unique_ptr_int__do_nothing__ptr_Impl::operator_inc_6(__it);
        });
        self
    }
}
impl CppInputIterator<Option<Value<i32>>>
    for forward_iterator_std_unique_ptr_int__do_nothing__ptr_
{
}
impl CppMove for forward_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
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
pub fn base_8(i: Ptr<bidirectional_iterator_int_ptr_>) -> Ptr<i32> {
    return i.with(|__s| __s.it_.clone());
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
                operator_eq_6(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_int_ptr_ {}
impl CppAdvance for bidirectional_iterator_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
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
impl CppMove for bidirectional_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_eq_9(
    x: Ptr<bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_>,
    y: Ptr<bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_10(
    x: Ptr<bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_>,
    y: Ptr<bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
pub fn base_11(
    i: Ptr<bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_>,
) -> Ptr<Option<Value<i32>>> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<Option<Value<i32>>>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    pub fn new_1(mut it: Ptr<Option<Value<i32>>>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(
        _a0: Ptr<bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_>,
    ) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_9(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_ {}
impl CppAdvance for bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
}
impl CppIterator<Option<Value<i32>>>
    for bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_
{
    fn iter_deref(&self) -> Ptr<Option<Value<i32>>> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_Impl::operator_inc_6(__it);
        });
        self
    }
}
impl CppInputIterator<Option<Value<i32>>>
    for bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_
{
}
impl CppMove for bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_eq_12(
    x: Ptr<random_access_iterator_int_ptr_>,
    y: Ptr<random_access_iterator_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_13(
    x: Ptr<random_access_iterator_int_ptr_>,
    y: Ptr<random_access_iterator_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
pub fn base_14(i: Ptr<random_access_iterator_int_ptr_>) -> Ptr<i32> {
    return i.with(|__s| __s.it_.clone());
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
                operator_eq_12(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for random_access_iterator_int_ptr_ {}
impl CppAdvance for random_access_iterator_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_7();
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
impl CppMove for random_access_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_eq_15(
    x: Ptr<random_access_iterator_std_unique_ptr_int__do_nothing__ptr_>,
    y: Ptr<random_access_iterator_std_unique_ptr_int__do_nothing__ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_16(
    x: Ptr<random_access_iterator_std_unique_ptr_int__do_nothing__ptr_>,
    y: Ptr<random_access_iterator_std_unique_ptr_int__do_nothing__ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
pub fn base_17(
    i: Ptr<random_access_iterator_std_unique_ptr_int__do_nothing__ptr_>,
) -> Ptr<Option<Value<i32>>> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct random_access_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<Option<Value<i32>>>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    pub fn new_1(mut it: Ptr<Option<Value<i32>>>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(
        _a0: Ptr<random_access_iterator_std_unique_ptr_int__do_nothing__ptr_>,
    ) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for random_access_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_15(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for random_access_iterator_std_unique_ptr_int__do_nothing__ptr_ {}
impl CppAdvance for random_access_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_7();
    }
}
impl CppIterator<Option<Value<i32>>>
    for random_access_iterator_std_unique_ptr_int__do_nothing__ptr_
{
    fn iter_deref(&self) -> Ptr<Option<Value<i32>>> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            random_access_iterator_std_unique_ptr_int__do_nothing__ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            random_access_iterator_std_unique_ptr_int__do_nothing__ptr_Impl::operator_inc_7(__it);
        });
        self
    }
}
impl CppInputIterator<Option<Value<i32>>>
    for random_access_iterator_std_unique_ptr_int__do_nothing__ptr_
{
}
impl CppMove for random_access_iterator_std_unique_ptr_int__do_nothing__ptr_ {
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
pub fn base_18(i: Ptr<i32>) -> Ptr<i32> {
    let i: Value<Ptr<i32>> = Rc::new(RefCell::new(i));
    return (*i.borrow()).clone();
}
pub fn base_19(i: Ptr<Option<Value<i32>>>) -> Ptr<Option<Value<i32>>> {
    let i: Value<Ptr<Option<Value<i32>>>> = Rc::new(RefCell::new(i));
    return (*i.borrow()).clone();
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
pub fn operator_eq_20(
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
        operator_eq_20(__lhs.clone(), __rhs.clone())
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
pub fn operator_add_21(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_22(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_21(_i, n)
    });
}
pub fn iter_swap_23(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap!((a.ptr_).clone(), (b.ptr_).clone());
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| {
            __v.prefix_inc();
        });
    }
}
pub fn iter_move_24(iter: adl_Iterator) -> Ptr<i32> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| {
            __v.prefix_inc();
        });
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_25(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> bool {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_26(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> std::cmp::Ordering {
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
                operator_cmp_26(__lhs.clone(), __rhs.clone())
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
                operator_eq_25(__lhs.clone(), __rhs.clone())
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
pub fn test_constexpr_27() -> bool {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 3, 4])));
    let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 3, 4])));
    let mut N: usize = 4_usize;
    let mut it: Ptr<i32> = match ({ ({ begin_28(ia.as_pointer()) }) }, {
        (ia.as_pointer() as Ptr<i32>).to_end()
    }) {
        (mut __tmp_0, mut __tmp_1) => {
            if __tmp_0 == __tmp_1 {
                (__tmp_1).clone()
            } else {
                let mut __write = __tmp_0.clone();
                let mut __it = (__tmp_0).clone();
                __it.iter_inc();
                while __it != __tmp_1 {
                    if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                        __write.iter_inc();
                        __write.iter_deref().write(__it.iter_deref().read());
                    }
                    __it.iter_inc();
                }
                __write.iter_inc();
                __write
            }
        }
    };
    return ({ (it).clone() } == { (({ begin_28(ia.as_pointer()) }).offset((N) as isize)) })
        && ({
            let ___first2: Ptr<i32> = ({ begin_29(expected.as_pointer()) });
            let ___last2: Ptr<i32> = (expected.as_pointer() as Ptr<i32>).to_end();
            equal_30(
                ({ begin_28(ia.as_pointer()) }),
                (it).clone(),
                ___first2,
                ___last2,
            )
        });
}
pub fn test_31() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
        match (
            { forward_iterator_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }) },
            {
                forward_iterator_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((sa) as isize)
                })
            },
        ) {
            (mut __tmp_2, mut __tmp_3) => {
                if __tmp_2 == __tmp_3 {
                    (__tmp_3).clone()
                } else {
                    let mut __write = __tmp_2.clone();
                    let mut __it = (__tmp_2).clone();
                    __it.iter_inc();
                    while __it != __tmp_3 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        },
    ));
    assert!(
        (({ base_2(r.as_pointer(),) }) == (ia.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(({ ((*ia.borrow())[(0) as usize] == 0) }));
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1])));
    let mut sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    {
        let __a1 = match (
            { forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }) },
            {
                forward_iterator_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sb) as isize)
                })
            },
        ) {
            (mut __tmp_4, mut __tmp_5) => {
                if __tmp_4 == __tmp_5 {
                    (__tmp_5).clone()
                } else {
                    let mut __write = __tmp_4.clone();
                    let mut __it = (__tmp_4).clone();
                    __it.iter_inc();
                    while __it != __tmp_5 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_2(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sb) as isize))
    );
    assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
    let ic: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0])));
    let mut sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    {
        let __a1 = match (
            { forward_iterator_int_ptr_::new_1({ (ic.as_pointer() as Ptr<i32>) }) },
            {
                forward_iterator_int_ptr_::new_1({
                    (ic.as_pointer() as Ptr<i32>).offset((sc) as isize)
                })
            },
        ) {
            (mut __tmp_6, mut __tmp_7) => {
                if __tmp_6 == __tmp_7 {
                    (__tmp_7).clone()
                } else {
                    let mut __write = __tmp_6.clone();
                    let mut __it = (__tmp_6).clone();
                    __it.iter_inc();
                    while __it != __tmp_7 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_2(r.as_pointer(),) }) == (ic.as_pointer() as Ptr::<i32>).offset((1) as isize))
    );
    assert!(({ ((*ic.borrow())[(0) as usize] == 0) }));
    let id: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1])));
    let mut sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    {
        let __a1 = match (
            { forward_iterator_int_ptr_::new_1({ (id.as_pointer() as Ptr<i32>) }) },
            {
                forward_iterator_int_ptr_::new_1({
                    (id.as_pointer() as Ptr<i32>).offset((sd) as isize)
                })
            },
        ) {
            (mut __tmp_8, mut __tmp_9) => {
                if __tmp_8 == __tmp_9 {
                    (__tmp_9).clone()
                } else {
                    let mut __write = __tmp_8.clone();
                    let mut __it = (__tmp_8).clone();
                    __it.iter_inc();
                    while __it != __tmp_9 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_2(r.as_pointer(),) }) == (id.as_pointer() as Ptr::<i32>).offset((2) as isize))
    );
    assert!(({ ((*id.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*id.borrow())[(1) as usize] == 1) }));
    let ie: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 0])));
    let mut se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    {
        let __a1 = match (
            { forward_iterator_int_ptr_::new_1({ (ie.as_pointer() as Ptr<i32>) }) },
            {
                forward_iterator_int_ptr_::new_1({
                    (ie.as_pointer() as Ptr<i32>).offset((se) as isize)
                })
            },
        ) {
            (mut __tmp_10, mut __tmp_11) => {
                if __tmp_10 == __tmp_11 {
                    (__tmp_11).clone()
                } else {
                    let mut __write = __tmp_10.clone();
                    let mut __it = (__tmp_10).clone();
                    __it.iter_inc();
                    while __it != __tmp_11 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_2(r.as_pointer(),) }) == (ie.as_pointer() as Ptr::<i32>).offset((3) as isize))
    );
    assert!(({ ((*ie.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*ie.borrow())[(1) as usize] == 1) }));
    assert!(({ ((*ie.borrow())[(2) as usize] == 0) }));
    let ig: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 1])));
    let mut sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    {
        let __a1 = match (
            { forward_iterator_int_ptr_::new_1({ (ig.as_pointer() as Ptr<i32>) }) },
            {
                forward_iterator_int_ptr_::new_1({
                    (ig.as_pointer() as Ptr<i32>).offset((sg) as isize)
                })
            },
        ) {
            (mut __tmp_12, mut __tmp_13) => {
                if __tmp_12 == __tmp_13 {
                    (__tmp_13).clone()
                } else {
                    let mut __write = __tmp_12.clone();
                    let mut __it = (__tmp_12).clone();
                    __it.iter_inc();
                    while __it != __tmp_13 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_2(r.as_pointer(),) }) == (ig.as_pointer() as Ptr::<i32>).offset((2) as isize))
    );
    assert!(({ ((*ig.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*ig.borrow())[(1) as usize] == 1) }));
    let ih: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1])));
    let mut sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    {
        let __a1 = match (
            { forward_iterator_int_ptr_::new_1({ (ih.as_pointer() as Ptr<i32>) }) },
            {
                forward_iterator_int_ptr_::new_1({
                    (ih.as_pointer() as Ptr<i32>).offset((sh) as isize)
                })
            },
        ) {
            (mut __tmp_14, mut __tmp_15) => {
                if __tmp_14 == __tmp_15 {
                    (__tmp_15).clone()
                } else {
                    let mut __write = __tmp_14.clone();
                    let mut __it = (__tmp_14).clone();
                    __it.iter_inc();
                    while __it != __tmp_15 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_2(r.as_pointer(),) }) == (ih.as_pointer() as Ptr::<i32>).offset((2) as isize))
    );
    assert!(({ ((*ih.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*ih.borrow())[(1) as usize] == 1) }));
    let ii: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 1, 2, 2, 2])));
    let mut si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    {
        let __a1 = match (
            { forward_iterator_int_ptr_::new_1({ (ii.as_pointer() as Ptr<i32>) }) },
            {
                forward_iterator_int_ptr_::new_1({
                    (ii.as_pointer() as Ptr<i32>).offset((si) as isize)
                })
            },
        ) {
            (mut __tmp_16, mut __tmp_17) => {
                if __tmp_16 == __tmp_17 {
                    (__tmp_17).clone()
                } else {
                    let mut __write = __tmp_16.clone();
                    let mut __it = (__tmp_16).clone();
                    __it.iter_inc();
                    while __it != __tmp_17 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_2(r.as_pointer(),) }) == (ii.as_pointer() as Ptr::<i32>).offset((3) as isize))
    );
    assert!(({ ((*ii.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*ii.borrow())[(1) as usize] == 1) }));
    assert!(({ ((*ii.borrow())[(2) as usize] == 2) }));
}
pub fn test_32() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
        match (
            { bidirectional_iterator_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }) },
            {
                bidirectional_iterator_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((sa) as isize)
                })
            },
        ) {
            (mut __tmp_18, mut __tmp_19) => {
                if __tmp_18 == __tmp_19 {
                    (__tmp_19).clone()
                } else {
                    let mut __write = __tmp_18.clone();
                    let mut __it = (__tmp_18).clone();
                    __it.iter_inc();
                    while __it != __tmp_19 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        },
    ));
    assert!(
        (({ base_8(r.as_pointer(),) }) == (ia.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(({ ((*ia.borrow())[(0) as usize] == 0) }));
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1])));
    let mut sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    {
        let __a1 = match (
            { bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }) },
            {
                bidirectional_iterator_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sb) as isize)
                })
            },
        ) {
            (mut __tmp_20, mut __tmp_21) => {
                if __tmp_20 == __tmp_21 {
                    (__tmp_21).clone()
                } else {
                    let mut __write = __tmp_20.clone();
                    let mut __it = (__tmp_20).clone();
                    __it.iter_inc();
                    while __it != __tmp_21 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sb) as isize))
    );
    assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
    let ic: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0])));
    let mut sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    {
        let __a1 = match (
            { bidirectional_iterator_int_ptr_::new_1({ (ic.as_pointer() as Ptr<i32>) }) },
            {
                bidirectional_iterator_int_ptr_::new_1({
                    (ic.as_pointer() as Ptr<i32>).offset((sc) as isize)
                })
            },
        ) {
            (mut __tmp_22, mut __tmp_23) => {
                if __tmp_22 == __tmp_23 {
                    (__tmp_23).clone()
                } else {
                    let mut __write = __tmp_22.clone();
                    let mut __it = (__tmp_22).clone();
                    __it.iter_inc();
                    while __it != __tmp_23 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_8(r.as_pointer(),) }) == (ic.as_pointer() as Ptr::<i32>).offset((1) as isize))
    );
    assert!(({ ((*ic.borrow())[(0) as usize] == 0) }));
    let id: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1])));
    let mut sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    {
        let __a1 = match (
            { bidirectional_iterator_int_ptr_::new_1({ (id.as_pointer() as Ptr<i32>) }) },
            {
                bidirectional_iterator_int_ptr_::new_1({
                    (id.as_pointer() as Ptr<i32>).offset((sd) as isize)
                })
            },
        ) {
            (mut __tmp_24, mut __tmp_25) => {
                if __tmp_24 == __tmp_25 {
                    (__tmp_25).clone()
                } else {
                    let mut __write = __tmp_24.clone();
                    let mut __it = (__tmp_24).clone();
                    __it.iter_inc();
                    while __it != __tmp_25 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_8(r.as_pointer(),) }) == (id.as_pointer() as Ptr::<i32>).offset((2) as isize))
    );
    assert!(({ ((*id.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*id.borrow())[(1) as usize] == 1) }));
    let ie: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 0])));
    let mut se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    {
        let __a1 = match (
            { bidirectional_iterator_int_ptr_::new_1({ (ie.as_pointer() as Ptr<i32>) }) },
            {
                bidirectional_iterator_int_ptr_::new_1({
                    (ie.as_pointer() as Ptr<i32>).offset((se) as isize)
                })
            },
        ) {
            (mut __tmp_26, mut __tmp_27) => {
                if __tmp_26 == __tmp_27 {
                    (__tmp_27).clone()
                } else {
                    let mut __write = __tmp_26.clone();
                    let mut __it = (__tmp_26).clone();
                    __it.iter_inc();
                    while __it != __tmp_27 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_8(r.as_pointer(),) }) == (ie.as_pointer() as Ptr::<i32>).offset((3) as isize))
    );
    assert!(({ ((*ie.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*ie.borrow())[(1) as usize] == 1) }));
    assert!(({ ((*ie.borrow())[(2) as usize] == 0) }));
    let ig: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 1])));
    let mut sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    {
        let __a1 = match (
            { bidirectional_iterator_int_ptr_::new_1({ (ig.as_pointer() as Ptr<i32>) }) },
            {
                bidirectional_iterator_int_ptr_::new_1({
                    (ig.as_pointer() as Ptr<i32>).offset((sg) as isize)
                })
            },
        ) {
            (mut __tmp_28, mut __tmp_29) => {
                if __tmp_28 == __tmp_29 {
                    (__tmp_29).clone()
                } else {
                    let mut __write = __tmp_28.clone();
                    let mut __it = (__tmp_28).clone();
                    __it.iter_inc();
                    while __it != __tmp_29 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_8(r.as_pointer(),) }) == (ig.as_pointer() as Ptr::<i32>).offset((2) as isize))
    );
    assert!(({ ((*ig.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*ig.borrow())[(1) as usize] == 1) }));
    let ih: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1])));
    let mut sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    {
        let __a1 = match (
            { bidirectional_iterator_int_ptr_::new_1({ (ih.as_pointer() as Ptr<i32>) }) },
            {
                bidirectional_iterator_int_ptr_::new_1({
                    (ih.as_pointer() as Ptr<i32>).offset((sh) as isize)
                })
            },
        ) {
            (mut __tmp_30, mut __tmp_31) => {
                if __tmp_30 == __tmp_31 {
                    (__tmp_31).clone()
                } else {
                    let mut __write = __tmp_30.clone();
                    let mut __it = (__tmp_30).clone();
                    __it.iter_inc();
                    while __it != __tmp_31 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_8(r.as_pointer(),) }) == (ih.as_pointer() as Ptr::<i32>).offset((2) as isize))
    );
    assert!(({ ((*ih.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*ih.borrow())[(1) as usize] == 1) }));
    let ii: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 1, 2, 2, 2])));
    let mut si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    {
        let __a1 = match (
            { bidirectional_iterator_int_ptr_::new_1({ (ii.as_pointer() as Ptr<i32>) }) },
            {
                bidirectional_iterator_int_ptr_::new_1({
                    (ii.as_pointer() as Ptr<i32>).offset((si) as isize)
                })
            },
        ) {
            (mut __tmp_32, mut __tmp_33) => {
                if __tmp_32 == __tmp_33 {
                    (__tmp_33).clone()
                } else {
                    let mut __write = __tmp_32.clone();
                    let mut __it = (__tmp_32).clone();
                    __it.iter_inc();
                    while __it != __tmp_33 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_8(r.as_pointer(),) }) == (ii.as_pointer() as Ptr::<i32>).offset((3) as isize))
    );
    assert!(({ ((*ii.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*ii.borrow())[(1) as usize] == 1) }));
    assert!(({ ((*ii.borrow())[(2) as usize] == 2) }));
}
pub fn test_33() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
        match (
            { random_access_iterator_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }) },
            {
                random_access_iterator_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((sa) as isize)
                })
            },
        ) {
            (mut __tmp_34, mut __tmp_35) => {
                if __tmp_34 == __tmp_35 {
                    (__tmp_35).clone()
                } else {
                    let mut __write = __tmp_34.clone();
                    let mut __it = (__tmp_34).clone();
                    __it.iter_inc();
                    while __it != __tmp_35 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        },
    ));
    assert!(
        (({ base_14(r.as_pointer(),) }) == (ia.as_pointer() as Ptr::<i32>).offset((sa) as isize))
    );
    assert!(({ ((*ia.borrow())[(0) as usize] == 0) }));
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1])));
    let mut sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    {
        let __a1 = match (
            { random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }) },
            {
                random_access_iterator_int_ptr_::new_1({
                    (ib.as_pointer() as Ptr<i32>).offset((sb) as isize)
                })
            },
        ) {
            (mut __tmp_36, mut __tmp_37) => {
                if __tmp_36 == __tmp_37 {
                    (__tmp_37).clone()
                } else {
                    let mut __write = __tmp_36.clone();
                    let mut __it = (__tmp_36).clone();
                    __it.iter_inc();
                    while __it != __tmp_37 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_14(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((sb) as isize))
    );
    assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
    let ic: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0])));
    let mut sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    {
        let __a1 = match (
            { random_access_iterator_int_ptr_::new_1({ (ic.as_pointer() as Ptr<i32>) }) },
            {
                random_access_iterator_int_ptr_::new_1({
                    (ic.as_pointer() as Ptr<i32>).offset((sc) as isize)
                })
            },
        ) {
            (mut __tmp_38, mut __tmp_39) => {
                if __tmp_38 == __tmp_39 {
                    (__tmp_39).clone()
                } else {
                    let mut __write = __tmp_38.clone();
                    let mut __it = (__tmp_38).clone();
                    __it.iter_inc();
                    while __it != __tmp_39 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_14(r.as_pointer(),) }) == (ic.as_pointer() as Ptr::<i32>).offset((1) as isize))
    );
    assert!(({ ((*ic.borrow())[(0) as usize] == 0) }));
    let id: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1])));
    let mut sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    {
        let __a1 = match (
            { random_access_iterator_int_ptr_::new_1({ (id.as_pointer() as Ptr<i32>) }) },
            {
                random_access_iterator_int_ptr_::new_1({
                    (id.as_pointer() as Ptr<i32>).offset((sd) as isize)
                })
            },
        ) {
            (mut __tmp_40, mut __tmp_41) => {
                if __tmp_40 == __tmp_41 {
                    (__tmp_41).clone()
                } else {
                    let mut __write = __tmp_40.clone();
                    let mut __it = (__tmp_40).clone();
                    __it.iter_inc();
                    while __it != __tmp_41 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_14(r.as_pointer(),) }) == (id.as_pointer() as Ptr::<i32>).offset((2) as isize))
    );
    assert!(({ ((*id.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*id.borrow())[(1) as usize] == 1) }));
    let ie: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 0])));
    let mut se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    {
        let __a1 = match (
            { random_access_iterator_int_ptr_::new_1({ (ie.as_pointer() as Ptr<i32>) }) },
            {
                random_access_iterator_int_ptr_::new_1({
                    (ie.as_pointer() as Ptr<i32>).offset((se) as isize)
                })
            },
        ) {
            (mut __tmp_42, mut __tmp_43) => {
                if __tmp_42 == __tmp_43 {
                    (__tmp_43).clone()
                } else {
                    let mut __write = __tmp_42.clone();
                    let mut __it = (__tmp_42).clone();
                    __it.iter_inc();
                    while __it != __tmp_43 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_14(r.as_pointer(),) }) == (ie.as_pointer() as Ptr::<i32>).offset((3) as isize))
    );
    assert!(({ ((*ie.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*ie.borrow())[(1) as usize] == 1) }));
    assert!(({ ((*ie.borrow())[(2) as usize] == 0) }));
    let ig: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 1])));
    let mut sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    {
        let __a1 = match (
            { random_access_iterator_int_ptr_::new_1({ (ig.as_pointer() as Ptr<i32>) }) },
            {
                random_access_iterator_int_ptr_::new_1({
                    (ig.as_pointer() as Ptr<i32>).offset((sg) as isize)
                })
            },
        ) {
            (mut __tmp_44, mut __tmp_45) => {
                if __tmp_44 == __tmp_45 {
                    (__tmp_45).clone()
                } else {
                    let mut __write = __tmp_44.clone();
                    let mut __it = (__tmp_44).clone();
                    __it.iter_inc();
                    while __it != __tmp_45 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_14(r.as_pointer(),) }) == (ig.as_pointer() as Ptr::<i32>).offset((2) as isize))
    );
    assert!(({ ((*ig.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*ig.borrow())[(1) as usize] == 1) }));
    let ih: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1])));
    let mut sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    {
        let __a1 = match (
            { random_access_iterator_int_ptr_::new_1({ (ih.as_pointer() as Ptr<i32>) }) },
            {
                random_access_iterator_int_ptr_::new_1({
                    (ih.as_pointer() as Ptr<i32>).offset((sh) as isize)
                })
            },
        ) {
            (mut __tmp_46, mut __tmp_47) => {
                if __tmp_46 == __tmp_47 {
                    (__tmp_47).clone()
                } else {
                    let mut __write = __tmp_46.clone();
                    let mut __it = (__tmp_46).clone();
                    __it.iter_inc();
                    while __it != __tmp_47 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_14(r.as_pointer(),) }) == (ih.as_pointer() as Ptr::<i32>).offset((2) as isize))
    );
    assert!(({ ((*ih.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*ih.borrow())[(1) as usize] == 1) }));
    let ii: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 1, 2, 2, 2])));
    let mut si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    {
        let __a1 = match (
            { random_access_iterator_int_ptr_::new_1({ (ii.as_pointer() as Ptr<i32>) }) },
            {
                random_access_iterator_int_ptr_::new_1({
                    (ii.as_pointer() as Ptr<i32>).offset((si) as isize)
                })
            },
        ) {
            (mut __tmp_48, mut __tmp_49) => {
                if __tmp_48 == __tmp_49 {
                    (__tmp_49).clone()
                } else {
                    let mut __write = __tmp_48.clone();
                    let mut __it = (__tmp_48).clone();
                    __it.iter_inc();
                    while __it != __tmp_49 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_14(r.as_pointer(),) }) == (ii.as_pointer() as Ptr::<i32>).offset((3) as isize))
    );
    assert!(({ ((*ii.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*ii.borrow())[(1) as usize] == 1) }));
    assert!(({ ((*ii.borrow())[(2) as usize] == 2) }));
}
pub fn test_34() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut r: Ptr<i32> = match ({ (ia.as_pointer() as Ptr<i32>) }, {
        (ia.as_pointer() as Ptr<i32>).offset((sa) as isize)
    }) {
        (mut __tmp_50, mut __tmp_51) => {
            if __tmp_50 == __tmp_51 {
                (__tmp_51).clone()
            } else {
                let mut __write = __tmp_50.clone();
                let mut __it = (__tmp_50).clone();
                __it.iter_inc();
                while __it != __tmp_51 {
                    if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                        __write.iter_inc();
                        __write.iter_deref().write(__it.iter_deref().read());
                    }
                    __it.iter_inc();
                }
                __write.iter_inc();
                __write
            }
        }
    };
    assert!(
        ({ ({ base_18((r).clone(),) }) } == {
            (ia.as_pointer() as Ptr<i32>).offset((sa) as isize)
        })
    );
    assert!(({ ((*ia.borrow())[(0) as usize] == 0) }));
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1])));
    let mut sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = match ({ (ib.as_pointer() as Ptr<i32>) }, {
        (ib.as_pointer() as Ptr<i32>).offset((sb) as isize)
    }) {
        (mut __tmp_52, mut __tmp_53) => {
            if __tmp_52 == __tmp_53 {
                (__tmp_53).clone()
            } else {
                let mut __write = __tmp_52.clone();
                let mut __it = (__tmp_52).clone();
                __it.iter_inc();
                while __it != __tmp_53 {
                    if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                        __write.iter_inc();
                        __write.iter_deref().write(__it.iter_deref().read());
                    }
                    __it.iter_inc();
                }
                __write.iter_inc();
                __write
            }
        }
    };
    assert!(
        ({ ({ base_18((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<i32>).offset((sb) as isize)
        })
    );
    assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
    let ic: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0])));
    let mut sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = match ({ (ic.as_pointer() as Ptr<i32>) }, {
        (ic.as_pointer() as Ptr<i32>).offset((sc) as isize)
    }) {
        (mut __tmp_54, mut __tmp_55) => {
            if __tmp_54 == __tmp_55 {
                (__tmp_55).clone()
            } else {
                let mut __write = __tmp_54.clone();
                let mut __it = (__tmp_54).clone();
                __it.iter_inc();
                while __it != __tmp_55 {
                    if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                        __write.iter_inc();
                        __write.iter_deref().write(__it.iter_deref().read());
                    }
                    __it.iter_inc();
                }
                __write.iter_inc();
                __write
            }
        }
    };
    assert!(
        ({ ({ base_18((r).clone(),) }) } == {
            (ic.as_pointer() as Ptr<i32>).offset((1) as isize)
        })
    );
    assert!(({ ((*ic.borrow())[(0) as usize] == 0) }));
    let id: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1])));
    let mut sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = match ({ (id.as_pointer() as Ptr<i32>) }, {
        (id.as_pointer() as Ptr<i32>).offset((sd) as isize)
    }) {
        (mut __tmp_56, mut __tmp_57) => {
            if __tmp_56 == __tmp_57 {
                (__tmp_57).clone()
            } else {
                let mut __write = __tmp_56.clone();
                let mut __it = (__tmp_56).clone();
                __it.iter_inc();
                while __it != __tmp_57 {
                    if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                        __write.iter_inc();
                        __write.iter_deref().write(__it.iter_deref().read());
                    }
                    __it.iter_inc();
                }
                __write.iter_inc();
                __write
            }
        }
    };
    assert!(
        ({ ({ base_18((r).clone(),) }) } == {
            (id.as_pointer() as Ptr<i32>).offset((2) as isize)
        })
    );
    assert!(({ ((*id.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*id.borrow())[(1) as usize] == 1) }));
    let ie: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 0])));
    let mut se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = match ({ (ie.as_pointer() as Ptr<i32>) }, {
        (ie.as_pointer() as Ptr<i32>).offset((se) as isize)
    }) {
        (mut __tmp_58, mut __tmp_59) => {
            if __tmp_58 == __tmp_59 {
                (__tmp_59).clone()
            } else {
                let mut __write = __tmp_58.clone();
                let mut __it = (__tmp_58).clone();
                __it.iter_inc();
                while __it != __tmp_59 {
                    if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                        __write.iter_inc();
                        __write.iter_deref().write(__it.iter_deref().read());
                    }
                    __it.iter_inc();
                }
                __write.iter_inc();
                __write
            }
        }
    };
    assert!(
        ({ ({ base_18((r).clone(),) }) } == {
            (ie.as_pointer() as Ptr<i32>).offset((3) as isize)
        })
    );
    assert!(({ ((*ie.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*ie.borrow())[(1) as usize] == 1) }));
    assert!(({ ((*ie.borrow())[(2) as usize] == 0) }));
    let ig: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 1])));
    let mut sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = match ({ (ig.as_pointer() as Ptr<i32>) }, {
        (ig.as_pointer() as Ptr<i32>).offset((sg) as isize)
    }) {
        (mut __tmp_60, mut __tmp_61) => {
            if __tmp_60 == __tmp_61 {
                (__tmp_61).clone()
            } else {
                let mut __write = __tmp_60.clone();
                let mut __it = (__tmp_60).clone();
                __it.iter_inc();
                while __it != __tmp_61 {
                    if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                        __write.iter_inc();
                        __write.iter_deref().write(__it.iter_deref().read());
                    }
                    __it.iter_inc();
                }
                __write.iter_inc();
                __write
            }
        }
    };
    assert!(
        ({ ({ base_18((r).clone(),) }) } == {
            (ig.as_pointer() as Ptr<i32>).offset((2) as isize)
        })
    );
    assert!(({ ((*ig.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*ig.borrow())[(1) as usize] == 1) }));
    let ih: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1])));
    let mut sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = match ({ (ih.as_pointer() as Ptr<i32>) }, {
        (ih.as_pointer() as Ptr<i32>).offset((sh) as isize)
    }) {
        (mut __tmp_62, mut __tmp_63) => {
            if __tmp_62 == __tmp_63 {
                (__tmp_63).clone()
            } else {
                let mut __write = __tmp_62.clone();
                let mut __it = (__tmp_62).clone();
                __it.iter_inc();
                while __it != __tmp_63 {
                    if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                        __write.iter_inc();
                        __write.iter_deref().write(__it.iter_deref().read());
                    }
                    __it.iter_inc();
                }
                __write.iter_inc();
                __write
            }
        }
    };
    assert!(
        ({ ({ base_18((r).clone(),) }) } == {
            (ih.as_pointer() as Ptr<i32>).offset((2) as isize)
        })
    );
    assert!(({ ((*ih.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*ih.borrow())[(1) as usize] == 1) }));
    let ii: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 1, 2, 2, 2])));
    let mut si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = match ({ (ii.as_pointer() as Ptr<i32>) }, {
        (ii.as_pointer() as Ptr<i32>).offset((si) as isize)
    }) {
        (mut __tmp_64, mut __tmp_65) => {
            if __tmp_64 == __tmp_65 {
                (__tmp_65).clone()
            } else {
                let mut __write = __tmp_64.clone();
                let mut __it = (__tmp_64).clone();
                __it.iter_inc();
                while __it != __tmp_65 {
                    if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                        __write.iter_inc();
                        __write.iter_deref().write(__it.iter_deref().read());
                    }
                    __it.iter_inc();
                }
                __write.iter_inc();
                __write
            }
        }
    };
    assert!(
        ({ ({ base_18((r).clone(),) }) } == {
            (ii.as_pointer() as Ptr<i32>).offset((3) as isize)
        })
    );
    assert!(({ ((*ii.borrow())[(0) as usize] == 0) }));
    assert!(({ ((*ii.borrow())[(1) as usize] == 1) }));
    assert!(({ ((*ii.borrow())[(2) as usize] == 2) }));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct do_nothing {}
impl Callable1<AnyPtr, ()> for do_nothing {
    fn call(&self, a1: AnyPtr) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            do_nothingImpl::operator_call(__it, a1)
        })
    }
}
impl CppMove for do_nothing {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test1_35() {
    let one: Value<i32> = Rc::new(RefCell::new(1));
    let two: Value<i32> = Rc::new(RefCell::new(2));
    let ia: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..1).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    let mut sa: u32 = (((8usize as usize).wrapping_div((8usize as usize))) as u32);
    let r: Value<forward_iterator_std_unique_ptr_int__do_nothing__ptr_> = Rc::new(RefCell::new(
        match (
            {
                forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ia.as_pointer() as Ptr<Option<Value<i32>>>)
                })
            },
            {
                forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ia.as_pointer() as Ptr<Option<Value<i32>>>).offset((sa) as isize)
                })
            },
        ) {
            (mut __tmp_66, mut __tmp_67) => {
                if __tmp_66 == __tmp_67 {
                    (__tmp_67).clone()
                } else {
                    let mut __write = __tmp_66.clone();
                    let mut __it = (__tmp_66).clone();
                    __it.iter_inc();
                    while __it != __tmp_67 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        },
    ));
    assert!(
        (({ base_5(r.as_pointer(),) })
            == (ia.as_pointer() as Ptr<Option<Value<i32>>>).offset((sa) as isize))
    );
    assert!(({ (*ia.borrow())[(0) as usize].is_none() }));
    let ib: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..2).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ib.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(1)
            .write(_p.to_owned_opt())
    };
    let mut sb: u32 = (((16usize as usize).wrapping_div((8usize as usize))) as u32);
    {
        let __a1 = match (
            {
                forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ib.as_pointer() as Ptr<Option<Value<i32>>>)
                })
            },
            {
                forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ib.as_pointer() as Ptr<Option<Value<i32>>>).offset((sb) as isize)
                })
            },
        ) {
            (mut __tmp_68, mut __tmp_69) => {
                if __tmp_68 == __tmp_69 {
                    (__tmp_69).clone()
                } else {
                    let mut __write = __tmp_68.clone();
                    let mut __it = (__tmp_68).clone();
                    __it.iter_inc();
                    while __it != __tmp_69 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_5(r.as_pointer(),) })
            == (ib.as_pointer() as Ptr<Option<Value<i32>>>).offset((sb) as isize))
    );
    assert!(({ (*ib.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*ib.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    let ic: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..2).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    let mut sc: u32 = (((16usize as usize).wrapping_div((8usize as usize))) as u32);
    {
        let __a1 = match (
            {
                forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ic.as_pointer() as Ptr<Option<Value<i32>>>)
                })
            },
            {
                forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ic.as_pointer() as Ptr<Option<Value<i32>>>).offset((sc) as isize)
                })
            },
        ) {
            (mut __tmp_70, mut __tmp_71) => {
                if __tmp_70 == __tmp_71 {
                    (__tmp_71).clone()
                } else {
                    let mut __write = __tmp_70.clone();
                    let mut __it = (__tmp_70).clone();
                    __it.iter_inc();
                    while __it != __tmp_71 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_5(r.as_pointer(),) })
            == (ic.as_pointer() as Ptr<Option<Value<i32>>>).offset((1) as isize))
    );
    assert!(({ (*ic.borrow())[(0) as usize].is_none() }));
    let id: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..3).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (id.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(2)
            .write(_p.to_owned_opt())
    };
    let mut sd: u32 = (((24usize as usize).wrapping_div((8usize as usize))) as u32);
    {
        let __a1 = match (
            {
                forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (id.as_pointer() as Ptr<Option<Value<i32>>>)
                })
            },
            {
                forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (id.as_pointer() as Ptr<Option<Value<i32>>>).offset((sd) as isize)
                })
            },
        ) {
            (mut __tmp_72, mut __tmp_73) => {
                if __tmp_72 == __tmp_73 {
                    (__tmp_73).clone()
                } else {
                    let mut __write = __tmp_72.clone();
                    let mut __it = (__tmp_72).clone();
                    __it.iter_inc();
                    while __it != __tmp_73 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_5(r.as_pointer(),) })
            == (id.as_pointer() as Ptr<Option<Value<i32>>>).offset((2) as isize))
    );
    assert!(({ (*id.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*id.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    let ie: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..4).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ie.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(2)
            .write(_p.to_owned_opt())
    };
    let mut se: u32 = (((32usize as usize).wrapping_div((8usize as usize))) as u32);
    {
        let __a1 = match (
            {
                forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ie.as_pointer() as Ptr<Option<Value<i32>>>)
                })
            },
            {
                forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ie.as_pointer() as Ptr<Option<Value<i32>>>).offset((se) as isize)
                })
            },
        ) {
            (mut __tmp_74, mut __tmp_75) => {
                if __tmp_74 == __tmp_75 {
                    (__tmp_75).clone()
                } else {
                    let mut __write = __tmp_74.clone();
                    let mut __it = (__tmp_74).clone();
                    __it.iter_inc();
                    while __it != __tmp_75 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_5(r.as_pointer(),) })
            == (ie.as_pointer() as Ptr<Option<Value<i32>>>).offset((3) as isize))
    );
    assert!(({ (*ie.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*ie.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    assert!(({ (*ie.borrow())[(2) as usize].is_none() }));
    let ig: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..4).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ig.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(2)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ig.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(3)
            .write(_p.to_owned_opt())
    };
    let mut sg: u32 = (((32usize as usize).wrapping_div((8usize as usize))) as u32);
    {
        let __a1 = match (
            {
                forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ig.as_pointer() as Ptr<Option<Value<i32>>>)
                })
            },
            {
                forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ig.as_pointer() as Ptr<Option<Value<i32>>>).offset((sg) as isize)
                })
            },
        ) {
            (mut __tmp_76, mut __tmp_77) => {
                if __tmp_76 == __tmp_77 {
                    (__tmp_77).clone()
                } else {
                    let mut __write = __tmp_76.clone();
                    let mut __it = (__tmp_76).clone();
                    __it.iter_inc();
                    while __it != __tmp_77 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_5(r.as_pointer(),) })
            == (ig.as_pointer() as Ptr<Option<Value<i32>>>).offset((2) as isize))
    );
    assert!(({ (*ig.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*ig.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    let ih: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..3).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ih.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(1)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ih.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(2)
            .write(_p.to_owned_opt())
    };
    let mut sh: u32 = (((24usize as usize).wrapping_div((8usize as usize))) as u32);
    {
        let __a1 = match (
            {
                forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ih.as_pointer() as Ptr<Option<Value<i32>>>)
                })
            },
            {
                forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ih.as_pointer() as Ptr<Option<Value<i32>>>).offset((sh) as isize)
                })
            },
        ) {
            (mut __tmp_78, mut __tmp_79) => {
                if __tmp_78 == __tmp_79 {
                    (__tmp_79).clone()
                } else {
                    let mut __write = __tmp_78.clone();
                    let mut __it = (__tmp_78).clone();
                    __it.iter_inc();
                    while __it != __tmp_79 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_5(r.as_pointer(),) })
            == (ih.as_pointer() as Ptr<Option<Value<i32>>>).offset((2) as isize))
    );
    assert!(({ (*ih.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*ih.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    let ii: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..7).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(1)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(2)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(3)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (two.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(4)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (two.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(5)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (two.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(6)
            .write(_p.to_owned_opt())
    };
    let mut si: u32 = (((56usize as usize).wrapping_div((8usize as usize))) as u32);
    {
        let __a1 = match (
            {
                forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ii.as_pointer() as Ptr<Option<Value<i32>>>)
                })
            },
            {
                forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ii.as_pointer() as Ptr<Option<Value<i32>>>).offset((si) as isize)
                })
            },
        ) {
            (mut __tmp_80, mut __tmp_81) => {
                if __tmp_80 == __tmp_81 {
                    (__tmp_81).clone()
                } else {
                    let mut __write = __tmp_80.clone();
                    let mut __it = (__tmp_80).clone();
                    __it.iter_inc();
                    while __it != __tmp_81 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_5(r.as_pointer(),) })
            == (ii.as_pointer() as Ptr<Option<Value<i32>>>).offset((3) as isize))
    );
    assert!(({ (*ii.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*ii.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    assert!(({ (((*ii.borrow())[(2) as usize].as_pointer().read()) == 2) }));
}
pub fn test1_36() {
    let one: Value<i32> = Rc::new(RefCell::new(1));
    let two: Value<i32> = Rc::new(RefCell::new(2));
    let ia: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..1).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    let mut sa: u32 = (((8usize as usize).wrapping_div((8usize as usize))) as u32);
    let r: Value<bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_> =
        Rc::new(RefCell::new(
            match (
                {
                    bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                        (ia.as_pointer() as Ptr<Option<Value<i32>>>)
                    })
                },
                {
                    bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                        (ia.as_pointer() as Ptr<Option<Value<i32>>>).offset((sa) as isize)
                    })
                },
            ) {
                (mut __tmp_82, mut __tmp_83) => {
                    if __tmp_82 == __tmp_83 {
                        (__tmp_83).clone()
                    } else {
                        let mut __write = __tmp_82.clone();
                        let mut __it = (__tmp_82).clone();
                        __it.iter_inc();
                        while __it != __tmp_83 {
                            if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                                __write.iter_inc();
                                __write.iter_deref().write(__it.iter_deref().read());
                            }
                            __it.iter_inc();
                        }
                        __write.iter_inc();
                        __write
                    }
                }
            },
        ));
    assert!(
        (({ base_11(r.as_pointer(),) })
            == (ia.as_pointer() as Ptr<Option<Value<i32>>>).offset((sa) as isize))
    );
    assert!(({ (*ia.borrow())[(0) as usize].is_none() }));
    let ib: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..2).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ib.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(1)
            .write(_p.to_owned_opt())
    };
    let mut sb: u32 = (((16usize as usize).wrapping_div((8usize as usize))) as u32);
    {
        let __a1 = match (
            {
                bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ib.as_pointer() as Ptr<Option<Value<i32>>>)
                })
            },
            {
                bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ib.as_pointer() as Ptr<Option<Value<i32>>>).offset((sb) as isize)
                })
            },
        ) {
            (mut __tmp_84, mut __tmp_85) => {
                if __tmp_84 == __tmp_85 {
                    (__tmp_85).clone()
                } else {
                    let mut __write = __tmp_84.clone();
                    let mut __it = (__tmp_84).clone();
                    __it.iter_inc();
                    while __it != __tmp_85 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_11(r.as_pointer(),) })
            == (ib.as_pointer() as Ptr<Option<Value<i32>>>).offset((sb) as isize))
    );
    assert!(({ (*ib.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*ib.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    let ic: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..2).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    let mut sc: u32 = (((16usize as usize).wrapping_div((8usize as usize))) as u32);
    {
        let __a1 = match (
            {
                bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ic.as_pointer() as Ptr<Option<Value<i32>>>)
                })
            },
            {
                bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ic.as_pointer() as Ptr<Option<Value<i32>>>).offset((sc) as isize)
                })
            },
        ) {
            (mut __tmp_86, mut __tmp_87) => {
                if __tmp_86 == __tmp_87 {
                    (__tmp_87).clone()
                } else {
                    let mut __write = __tmp_86.clone();
                    let mut __it = (__tmp_86).clone();
                    __it.iter_inc();
                    while __it != __tmp_87 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_11(r.as_pointer(),) })
            == (ic.as_pointer() as Ptr<Option<Value<i32>>>).offset((1) as isize))
    );
    assert!(({ (*ic.borrow())[(0) as usize].is_none() }));
    let id: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..3).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (id.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(2)
            .write(_p.to_owned_opt())
    };
    let mut sd: u32 = (((24usize as usize).wrapping_div((8usize as usize))) as u32);
    {
        let __a1 = match (
            {
                bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (id.as_pointer() as Ptr<Option<Value<i32>>>)
                })
            },
            {
                bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (id.as_pointer() as Ptr<Option<Value<i32>>>).offset((sd) as isize)
                })
            },
        ) {
            (mut __tmp_88, mut __tmp_89) => {
                if __tmp_88 == __tmp_89 {
                    (__tmp_89).clone()
                } else {
                    let mut __write = __tmp_88.clone();
                    let mut __it = (__tmp_88).clone();
                    __it.iter_inc();
                    while __it != __tmp_89 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_11(r.as_pointer(),) })
            == (id.as_pointer() as Ptr<Option<Value<i32>>>).offset((2) as isize))
    );
    assert!(({ (*id.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*id.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    let ie: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..4).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ie.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(2)
            .write(_p.to_owned_opt())
    };
    let mut se: u32 = (((32usize as usize).wrapping_div((8usize as usize))) as u32);
    {
        let __a1 = match (
            {
                bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ie.as_pointer() as Ptr<Option<Value<i32>>>)
                })
            },
            {
                bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ie.as_pointer() as Ptr<Option<Value<i32>>>).offset((se) as isize)
                })
            },
        ) {
            (mut __tmp_90, mut __tmp_91) => {
                if __tmp_90 == __tmp_91 {
                    (__tmp_91).clone()
                } else {
                    let mut __write = __tmp_90.clone();
                    let mut __it = (__tmp_90).clone();
                    __it.iter_inc();
                    while __it != __tmp_91 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_11(r.as_pointer(),) })
            == (ie.as_pointer() as Ptr<Option<Value<i32>>>).offset((3) as isize))
    );
    assert!(({ (*ie.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*ie.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    assert!(({ (*ie.borrow())[(2) as usize].is_none() }));
    let ig: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..4).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ig.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(2)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ig.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(3)
            .write(_p.to_owned_opt())
    };
    let mut sg: u32 = (((32usize as usize).wrapping_div((8usize as usize))) as u32);
    {
        let __a1 = match (
            {
                bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ig.as_pointer() as Ptr<Option<Value<i32>>>)
                })
            },
            {
                bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ig.as_pointer() as Ptr<Option<Value<i32>>>).offset((sg) as isize)
                })
            },
        ) {
            (mut __tmp_92, mut __tmp_93) => {
                if __tmp_92 == __tmp_93 {
                    (__tmp_93).clone()
                } else {
                    let mut __write = __tmp_92.clone();
                    let mut __it = (__tmp_92).clone();
                    __it.iter_inc();
                    while __it != __tmp_93 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_11(r.as_pointer(),) })
            == (ig.as_pointer() as Ptr<Option<Value<i32>>>).offset((2) as isize))
    );
    assert!(({ (*ig.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*ig.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    let ih: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..3).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ih.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(1)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ih.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(2)
            .write(_p.to_owned_opt())
    };
    let mut sh: u32 = (((24usize as usize).wrapping_div((8usize as usize))) as u32);
    {
        let __a1 = match (
            {
                bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ih.as_pointer() as Ptr<Option<Value<i32>>>)
                })
            },
            {
                bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ih.as_pointer() as Ptr<Option<Value<i32>>>).offset((sh) as isize)
                })
            },
        ) {
            (mut __tmp_94, mut __tmp_95) => {
                if __tmp_94 == __tmp_95 {
                    (__tmp_95).clone()
                } else {
                    let mut __write = __tmp_94.clone();
                    let mut __it = (__tmp_94).clone();
                    __it.iter_inc();
                    while __it != __tmp_95 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_11(r.as_pointer(),) })
            == (ih.as_pointer() as Ptr<Option<Value<i32>>>).offset((2) as isize))
    );
    assert!(({ (*ih.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*ih.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    let ii: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..7).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(1)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(2)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(3)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (two.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(4)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (two.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(5)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (two.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(6)
            .write(_p.to_owned_opt())
    };
    let mut si: u32 = (((56usize as usize).wrapping_div((8usize as usize))) as u32);
    {
        let __a1 = match (
            {
                bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ii.as_pointer() as Ptr<Option<Value<i32>>>)
                })
            },
            {
                bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ii.as_pointer() as Ptr<Option<Value<i32>>>).offset((si) as isize)
                })
            },
        ) {
            (mut __tmp_96, mut __tmp_97) => {
                if __tmp_96 == __tmp_97 {
                    (__tmp_97).clone()
                } else {
                    let mut __write = __tmp_96.clone();
                    let mut __it = (__tmp_96).clone();
                    __it.iter_inc();
                    while __it != __tmp_97 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_11(r.as_pointer(),) })
            == (ii.as_pointer() as Ptr<Option<Value<i32>>>).offset((3) as isize))
    );
    assert!(({ (*ii.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*ii.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    assert!(({ (((*ii.borrow())[(2) as usize].as_pointer().read()) == 2) }));
}
pub fn test1_37() {
    let one: Value<i32> = Rc::new(RefCell::new(1));
    let two: Value<i32> = Rc::new(RefCell::new(2));
    let ia: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..1).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    let mut sa: u32 = (((8usize as usize).wrapping_div((8usize as usize))) as u32);
    let r: Value<random_access_iterator_std_unique_ptr_int__do_nothing__ptr_> =
        Rc::new(RefCell::new(
            match (
                {
                    random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                        (ia.as_pointer() as Ptr<Option<Value<i32>>>)
                    })
                },
                {
                    random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                        (ia.as_pointer() as Ptr<Option<Value<i32>>>).offset((sa) as isize)
                    })
                },
            ) {
                (mut __tmp_98, mut __tmp_99) => {
                    if __tmp_98 == __tmp_99 {
                        (__tmp_99).clone()
                    } else {
                        let mut __write = __tmp_98.clone();
                        let mut __it = (__tmp_98).clone();
                        __it.iter_inc();
                        while __it != __tmp_99 {
                            if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                                __write.iter_inc();
                                __write.iter_deref().write(__it.iter_deref().read());
                            }
                            __it.iter_inc();
                        }
                        __write.iter_inc();
                        __write
                    }
                }
            },
        ));
    assert!(
        (({ base_17(r.as_pointer(),) })
            == (ia.as_pointer() as Ptr<Option<Value<i32>>>).offset((sa) as isize))
    );
    assert!(({ (*ia.borrow())[(0) as usize].is_none() }));
    let ib: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..2).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ib.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(1)
            .write(_p.to_owned_opt())
    };
    let mut sb: u32 = (((16usize as usize).wrapping_div((8usize as usize))) as u32);
    {
        let __a1 = match (
            {
                random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ib.as_pointer() as Ptr<Option<Value<i32>>>)
                })
            },
            {
                random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ib.as_pointer() as Ptr<Option<Value<i32>>>).offset((sb) as isize)
                })
            },
        ) {
            (mut __tmp_100, mut __tmp_101) => {
                if __tmp_100 == __tmp_101 {
                    (__tmp_101).clone()
                } else {
                    let mut __write = __tmp_100.clone();
                    let mut __it = (__tmp_100).clone();
                    __it.iter_inc();
                    while __it != __tmp_101 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_17(r.as_pointer(),) })
            == (ib.as_pointer() as Ptr<Option<Value<i32>>>).offset((sb) as isize))
    );
    assert!(({ (*ib.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*ib.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    let ic: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..2).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    let mut sc: u32 = (((16usize as usize).wrapping_div((8usize as usize))) as u32);
    {
        let __a1 = match (
            {
                random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ic.as_pointer() as Ptr<Option<Value<i32>>>)
                })
            },
            {
                random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ic.as_pointer() as Ptr<Option<Value<i32>>>).offset((sc) as isize)
                })
            },
        ) {
            (mut __tmp_102, mut __tmp_103) => {
                if __tmp_102 == __tmp_103 {
                    (__tmp_103).clone()
                } else {
                    let mut __write = __tmp_102.clone();
                    let mut __it = (__tmp_102).clone();
                    __it.iter_inc();
                    while __it != __tmp_103 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_17(r.as_pointer(),) })
            == (ic.as_pointer() as Ptr<Option<Value<i32>>>).offset((1) as isize))
    );
    assert!(({ (*ic.borrow())[(0) as usize].is_none() }));
    let id: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..3).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (id.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(2)
            .write(_p.to_owned_opt())
    };
    let mut sd: u32 = (((24usize as usize).wrapping_div((8usize as usize))) as u32);
    {
        let __a1 = match (
            {
                random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (id.as_pointer() as Ptr<Option<Value<i32>>>)
                })
            },
            {
                random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (id.as_pointer() as Ptr<Option<Value<i32>>>).offset((sd) as isize)
                })
            },
        ) {
            (mut __tmp_104, mut __tmp_105) => {
                if __tmp_104 == __tmp_105 {
                    (__tmp_105).clone()
                } else {
                    let mut __write = __tmp_104.clone();
                    let mut __it = (__tmp_104).clone();
                    __it.iter_inc();
                    while __it != __tmp_105 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_17(r.as_pointer(),) })
            == (id.as_pointer() as Ptr<Option<Value<i32>>>).offset((2) as isize))
    );
    assert!(({ (*id.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*id.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    let ie: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..4).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ie.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(2)
            .write(_p.to_owned_opt())
    };
    let mut se: u32 = (((32usize as usize).wrapping_div((8usize as usize))) as u32);
    {
        let __a1 = match (
            {
                random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ie.as_pointer() as Ptr<Option<Value<i32>>>)
                })
            },
            {
                random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ie.as_pointer() as Ptr<Option<Value<i32>>>).offset((se) as isize)
                })
            },
        ) {
            (mut __tmp_106, mut __tmp_107) => {
                if __tmp_106 == __tmp_107 {
                    (__tmp_107).clone()
                } else {
                    let mut __write = __tmp_106.clone();
                    let mut __it = (__tmp_106).clone();
                    __it.iter_inc();
                    while __it != __tmp_107 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_17(r.as_pointer(),) })
            == (ie.as_pointer() as Ptr<Option<Value<i32>>>).offset((3) as isize))
    );
    assert!(({ (*ie.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*ie.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    assert!(({ (*ie.borrow())[(2) as usize].is_none() }));
    let ig: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..4).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ig.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(2)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ig.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(3)
            .write(_p.to_owned_opt())
    };
    let mut sg: u32 = (((32usize as usize).wrapping_div((8usize as usize))) as u32);
    {
        let __a1 = match (
            {
                random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ig.as_pointer() as Ptr<Option<Value<i32>>>)
                })
            },
            {
                random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ig.as_pointer() as Ptr<Option<Value<i32>>>).offset((sg) as isize)
                })
            },
        ) {
            (mut __tmp_108, mut __tmp_109) => {
                if __tmp_108 == __tmp_109 {
                    (__tmp_109).clone()
                } else {
                    let mut __write = __tmp_108.clone();
                    let mut __it = (__tmp_108).clone();
                    __it.iter_inc();
                    while __it != __tmp_109 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_17(r.as_pointer(),) })
            == (ig.as_pointer() as Ptr<Option<Value<i32>>>).offset((2) as isize))
    );
    assert!(({ (*ig.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*ig.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    let ih: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..3).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ih.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(1)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ih.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(2)
            .write(_p.to_owned_opt())
    };
    let mut sh: u32 = (((24usize as usize).wrapping_div((8usize as usize))) as u32);
    {
        let __a1 = match (
            {
                random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ih.as_pointer() as Ptr<Option<Value<i32>>>)
                })
            },
            {
                random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ih.as_pointer() as Ptr<Option<Value<i32>>>).offset((sh) as isize)
                })
            },
        ) {
            (mut __tmp_110, mut __tmp_111) => {
                if __tmp_110 == __tmp_111 {
                    (__tmp_111).clone()
                } else {
                    let mut __write = __tmp_110.clone();
                    let mut __it = (__tmp_110).clone();
                    __it.iter_inc();
                    while __it != __tmp_111 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_17(r.as_pointer(),) })
            == (ih.as_pointer() as Ptr<Option<Value<i32>>>).offset((2) as isize))
    );
    assert!(({ (*ih.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*ih.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    let ii: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..7).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(1)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(2)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(3)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (two.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(4)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (two.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(5)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (two.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(6)
            .write(_p.to_owned_opt())
    };
    let mut si: u32 = (((56usize as usize).wrapping_div((8usize as usize))) as u32);
    {
        let __a1 = match (
            {
                random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ii.as_pointer() as Ptr<Option<Value<i32>>>)
                })
            },
            {
                random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
                    (ii.as_pointer() as Ptr<Option<Value<i32>>>).offset((si) as isize)
                })
            },
        ) {
            (mut __tmp_112, mut __tmp_113) => {
                if __tmp_112 == __tmp_113 {
                    (__tmp_113).clone()
                } else {
                    let mut __write = __tmp_112.clone();
                    let mut __it = (__tmp_112).clone();
                    __it.iter_inc();
                    while __it != __tmp_113 {
                        if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                            __write.iter_inc();
                            __write.iter_deref().write(__it.iter_deref().read());
                        }
                        __it.iter_inc();
                    }
                    __write.iter_inc();
                    __write
                }
            }
        };
        r.as_pointer().with_mut(|__it| *__it = __a1);
        r.as_pointer()
    };
    assert!(
        (({ base_17(r.as_pointer(),) })
            == (ii.as_pointer() as Ptr<Option<Value<i32>>>).offset((3) as isize))
    );
    assert!(({ (*ii.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*ii.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    assert!(({ (((*ii.borrow())[(2) as usize].as_pointer().read()) == 2) }));
}
pub fn test1_38() {
    let one: Value<i32> = Rc::new(RefCell::new(1));
    let two: Value<i32> = Rc::new(RefCell::new(2));
    let ia: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..1).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    let mut sa: u32 = (((8usize as usize).wrapping_div((8usize as usize))) as u32);
    let mut r: Ptr<Option<Value<i32>>> = match ({ (ia.as_pointer() as Ptr<Option<Value<i32>>>) }, {
        (ia.as_pointer() as Ptr<Option<Value<i32>>>).offset((sa) as isize)
    }) {
        (mut __tmp_114, mut __tmp_115) => {
            if __tmp_114 == __tmp_115 {
                (__tmp_115).clone()
            } else {
                let mut __write = __tmp_114.clone();
                let mut __it = (__tmp_114).clone();
                __it.iter_inc();
                while __it != __tmp_115 {
                    if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                        __write.iter_inc();
                        __write.iter_deref().write(__it.iter_deref().read());
                    }
                    __it.iter_inc();
                }
                __write.iter_inc();
                __write
            }
        }
    };
    assert!(
        ({ ({ base_19((r).clone(),) }) } == {
            (ia.as_pointer() as Ptr<Option<Value<i32>>>).offset((sa) as isize)
        })
    );
    assert!(({ (*ia.borrow())[(0) as usize].is_none() }));
    let ib: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..2).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ib.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(1)
            .write(_p.to_owned_opt())
    };
    let mut sb: u32 = (((16usize as usize).wrapping_div((8usize as usize))) as u32);
    r = match ({ (ib.as_pointer() as Ptr<Option<Value<i32>>>) }, {
        (ib.as_pointer() as Ptr<Option<Value<i32>>>).offset((sb) as isize)
    }) {
        (mut __tmp_116, mut __tmp_117) => {
            if __tmp_116 == __tmp_117 {
                (__tmp_117).clone()
            } else {
                let mut __write = __tmp_116.clone();
                let mut __it = (__tmp_116).clone();
                __it.iter_inc();
                while __it != __tmp_117 {
                    if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                        __write.iter_inc();
                        __write.iter_deref().write(__it.iter_deref().read());
                    }
                    __it.iter_inc();
                }
                __write.iter_inc();
                __write
            }
        }
    };
    assert!(
        ({ ({ base_19((r).clone(),) }) } == {
            (ib.as_pointer() as Ptr<Option<Value<i32>>>).offset((sb) as isize)
        })
    );
    assert!(({ (*ib.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*ib.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    let ic: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..2).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    let mut sc: u32 = (((16usize as usize).wrapping_div((8usize as usize))) as u32);
    r = match ({ (ic.as_pointer() as Ptr<Option<Value<i32>>>) }, {
        (ic.as_pointer() as Ptr<Option<Value<i32>>>).offset((sc) as isize)
    }) {
        (mut __tmp_118, mut __tmp_119) => {
            if __tmp_118 == __tmp_119 {
                (__tmp_119).clone()
            } else {
                let mut __write = __tmp_118.clone();
                let mut __it = (__tmp_118).clone();
                __it.iter_inc();
                while __it != __tmp_119 {
                    if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                        __write.iter_inc();
                        __write.iter_deref().write(__it.iter_deref().read());
                    }
                    __it.iter_inc();
                }
                __write.iter_inc();
                __write
            }
        }
    };
    assert!(
        ({ ({ base_19((r).clone(),) }) } == {
            (ic.as_pointer() as Ptr<Option<Value<i32>>>).offset((1) as isize)
        })
    );
    assert!(({ (*ic.borrow())[(0) as usize].is_none() }));
    let id: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..3).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (id.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(2)
            .write(_p.to_owned_opt())
    };
    let mut sd: u32 = (((24usize as usize).wrapping_div((8usize as usize))) as u32);
    r = match ({ (id.as_pointer() as Ptr<Option<Value<i32>>>) }, {
        (id.as_pointer() as Ptr<Option<Value<i32>>>).offset((sd) as isize)
    }) {
        (mut __tmp_120, mut __tmp_121) => {
            if __tmp_120 == __tmp_121 {
                (__tmp_121).clone()
            } else {
                let mut __write = __tmp_120.clone();
                let mut __it = (__tmp_120).clone();
                __it.iter_inc();
                while __it != __tmp_121 {
                    if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                        __write.iter_inc();
                        __write.iter_deref().write(__it.iter_deref().read());
                    }
                    __it.iter_inc();
                }
                __write.iter_inc();
                __write
            }
        }
    };
    assert!(
        ({ ({ base_19((r).clone(),) }) } == {
            (id.as_pointer() as Ptr<Option<Value<i32>>>).offset((2) as isize)
        })
    );
    assert!(({ (*id.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*id.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    let ie: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..4).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ie.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(2)
            .write(_p.to_owned_opt())
    };
    let mut se: u32 = (((32usize as usize).wrapping_div((8usize as usize))) as u32);
    r = match ({ (ie.as_pointer() as Ptr<Option<Value<i32>>>) }, {
        (ie.as_pointer() as Ptr<Option<Value<i32>>>).offset((se) as isize)
    }) {
        (mut __tmp_122, mut __tmp_123) => {
            if __tmp_122 == __tmp_123 {
                (__tmp_123).clone()
            } else {
                let mut __write = __tmp_122.clone();
                let mut __it = (__tmp_122).clone();
                __it.iter_inc();
                while __it != __tmp_123 {
                    if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                        __write.iter_inc();
                        __write.iter_deref().write(__it.iter_deref().read());
                    }
                    __it.iter_inc();
                }
                __write.iter_inc();
                __write
            }
        }
    };
    assert!(
        ({ ({ base_19((r).clone(),) }) } == {
            (ie.as_pointer() as Ptr<Option<Value<i32>>>).offset((3) as isize)
        })
    );
    assert!(({ (*ie.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*ie.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    assert!(({ (*ie.borrow())[(2) as usize].is_none() }));
    let ig: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..4).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ig.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(2)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ig.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(3)
            .write(_p.to_owned_opt())
    };
    let mut sg: u32 = (((32usize as usize).wrapping_div((8usize as usize))) as u32);
    r = match ({ (ig.as_pointer() as Ptr<Option<Value<i32>>>) }, {
        (ig.as_pointer() as Ptr<Option<Value<i32>>>).offset((sg) as isize)
    }) {
        (mut __tmp_124, mut __tmp_125) => {
            if __tmp_124 == __tmp_125 {
                (__tmp_125).clone()
            } else {
                let mut __write = __tmp_124.clone();
                let mut __it = (__tmp_124).clone();
                __it.iter_inc();
                while __it != __tmp_125 {
                    if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                        __write.iter_inc();
                        __write.iter_deref().write(__it.iter_deref().read());
                    }
                    __it.iter_inc();
                }
                __write.iter_inc();
                __write
            }
        }
    };
    assert!(
        ({ ({ base_19((r).clone(),) }) } == {
            (ig.as_pointer() as Ptr<Option<Value<i32>>>).offset((2) as isize)
        })
    );
    assert!(({ (*ig.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*ig.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    let ih: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..3).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ih.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(1)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ih.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(2)
            .write(_p.to_owned_opt())
    };
    let mut sh: u32 = (((24usize as usize).wrapping_div((8usize as usize))) as u32);
    r = match ({ (ih.as_pointer() as Ptr<Option<Value<i32>>>) }, {
        (ih.as_pointer() as Ptr<Option<Value<i32>>>).offset((sh) as isize)
    }) {
        (mut __tmp_126, mut __tmp_127) => {
            if __tmp_126 == __tmp_127 {
                (__tmp_127).clone()
            } else {
                let mut __write = __tmp_126.clone();
                let mut __it = (__tmp_126).clone();
                __it.iter_inc();
                while __it != __tmp_127 {
                    if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                        __write.iter_inc();
                        __write.iter_deref().write(__it.iter_deref().read());
                    }
                    __it.iter_inc();
                }
                __write.iter_inc();
                __write
            }
        }
    };
    assert!(
        ({ ({ base_19((r).clone(),) }) } == {
            (ih.as_pointer() as Ptr<Option<Value<i32>>>).offset((2) as isize)
        })
    );
    assert!(({ (*ih.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*ih.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    let ii: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..7).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(1)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(2)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (one.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(3)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (two.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(4)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (two.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(5)
            .write(_p.to_owned_opt())
    };
    {
        let _p: Ptr<_> = (two.as_pointer());
        (ii.as_pointer() as Ptr<Option<Value<i32>>>)
            .offset(6)
            .write(_p.to_owned_opt())
    };
    let mut si: u32 = (((56usize as usize).wrapping_div((8usize as usize))) as u32);
    r = match ({ (ii.as_pointer() as Ptr<Option<Value<i32>>>) }, {
        (ii.as_pointer() as Ptr<Option<Value<i32>>>).offset((si) as isize)
    }) {
        (mut __tmp_128, mut __tmp_129) => {
            if __tmp_128 == __tmp_129 {
                (__tmp_129).clone()
            } else {
                let mut __write = __tmp_128.clone();
                let mut __it = (__tmp_128).clone();
                __it.iter_inc();
                while __it != __tmp_129 {
                    if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                        __write.iter_inc();
                        __write.iter_deref().write(__it.iter_deref().read());
                    }
                    __it.iter_inc();
                }
                __write.iter_inc();
                __write
            }
        }
    };
    assert!(
        ({ ({ base_19((r).clone(),) }) } == {
            (ii.as_pointer() as Ptr<Option<Value<i32>>>).offset((3) as isize)
        })
    );
    assert!(({ (*ii.borrow())[(0) as usize].is_none() }));
    assert!(({ (((*ii.borrow())[(1) as usize].as_pointer().read()) == 1) }));
    assert!(({ (((*ii.borrow())[(2) as usize].as_pointer().read()) == 2) }));
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
    ({ test_31() });
    ({ test_32() });
    ({ test_33() });
    ({ test_34() });
    ({ test1_35() });
    ({ test1_36() });
    ({ test1_37() });
    ({ test1_38() });
    const _: () = assert!(true, "test_constexpr()");;
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
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
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
pub trait bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_Impl {
    fn operator_deref(&self) -> Ptr<Option<Value<i32>>>;
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_>;
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_8(
        &self,
        mut _a0: i32,
    ) -> bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_9(
        &self,
        mut _a0: i32,
    ) -> bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_>,
    ) -> Ptr<bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_>;
}
impl bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_Impl
    for Ptr<bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_>
{
    fn operator_deref(&self) -> Ptr<Option<Value<i32>>> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_>,
    ) -> Ptr<bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_> {
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
pub trait do_nothingImpl {
    fn operator_call(&self, _a0: AnyPtr);
}
impl do_nothingImpl for Ptr<do_nothing> {
    fn operator_call(&self, mut _a0: AnyPtr) {}
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
pub trait forward_iterator_std_unique_ptr_int__do_nothing__ptr_Impl {
    fn operator_deref(&self) -> Ptr<Option<Value<i32>>>;
    fn operator_inc_6(&self) -> Ptr<forward_iterator_std_unique_ptr_int__do_nothing__ptr_>;
    fn operator_post_inc_7(
        &self,
        mut _a0: i32,
    ) -> forward_iterator_std_unique_ptr_int__do_nothing__ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<forward_iterator_std_unique_ptr_int__do_nothing__ptr_>,
    ) -> Ptr<forward_iterator_std_unique_ptr_int__do_nothing__ptr_>;
}
impl forward_iterator_std_unique_ptr_int__do_nothing__ptr_Impl
    for Ptr<forward_iterator_std_unique_ptr_int__do_nothing__ptr_>
{
    fn operator_deref(&self) -> Ptr<Option<Value<i32>>> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<forward_iterator_std_unique_ptr_int__do_nothing__ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<forward_iterator_std_unique_ptr_int__do_nothing__ptr_>,
    ) -> Ptr<forward_iterator_std_unique_ptr_int__do_nothing__ptr_> {
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
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
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
pub trait random_access_iterator_std_unique_ptr_int__do_nothing__ptr_Impl {
    fn operator_deref(&self) -> Ptr<Option<Value<i32>>>;
    fn operator_index(&self, mut n: isize) -> Ptr<Option<Value<i32>>> {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_std_unique_ptr_int__do_nothing__ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_std_unique_ptr_int__do_nothing__ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> random_access_iterator_std_unique_ptr_int__do_nothing__ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(
        &self,
        mut _a0: i32,
    ) -> random_access_iterator_std_unique_ptr_int__do_nothing__ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<random_access_iterator_std_unique_ptr_int__do_nothing__ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<random_access_iterator_std_unique_ptr_int__do_nothing__ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_std_unique_ptr_int__do_nothing__ptr_>,
    ) -> Ptr<random_access_iterator_std_unique_ptr_int__do_nothing__ptr_>;
}
impl random_access_iterator_std_unique_ptr_int__do_nothing__ptr_Impl
    for Ptr<random_access_iterator_std_unique_ptr_int__do_nothing__ptr_>
{
    fn operator_deref(&self) -> Ptr<Option<Value<i32>>> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_std_unique_ptr_int__do_nothing__ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_std_unique_ptr_int__do_nothing__ptr_>,
    ) -> Ptr<random_access_iterator_std_unique_ptr_int__do_nothing__ptr_> {
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
