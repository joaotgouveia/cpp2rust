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
pub fn base_1(i: Ptr<cpp17_input_iterator_char_ptr__char_ptr_>) -> Ptr<i8> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_char_ptr__char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i8>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_char_ptr__char_ptr_ {
    pub fn new(mut it: Ptr<i8>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<cpp17_input_iterator_char_ptr__char_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for cpp17_input_iterator_char_ptr__char_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub fn base_2(i: Ptr<forward_iterator_char_ptr_>) -> Ptr<i8> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct forward_iterator_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i8>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl forward_iterator_char_ptr_ {
    pub fn new_1(mut it: Ptr<i8>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<forward_iterator_char_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for forward_iterator_char_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn base_3(i: Ptr<bidirectional_iterator_char_ptr_>) -> Ptr<i8> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct bidirectional_iterator_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i8>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_char_ptr_ {
    pub fn new_1(mut it: Ptr<i8>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<bidirectional_iterator_char_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for bidirectional_iterator_char_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn base_4(i: Ptr<random_access_iterator_char_ptr_>) -> Ptr<i8> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct random_access_iterator_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i8>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_char_ptr_ {
    pub fn new_1(mut it: Ptr<i8>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<random_access_iterator_char_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for random_access_iterator_char_ptr_ {
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
pub fn base_5(i: Ptr<contiguous_iterator_char_ptr_>) -> Ptr<i8> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i8>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_char_ptr_ {
    pub fn new_1(mut it: Ptr<i8>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<contiguous_iterator_char_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for contiguous_iterator_char_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_cmp_6(
    x: Ptr<three_way_contiguous_iterator_char_ptr_>,
    y: Ptr<three_way_contiguous_iterator_char_ptr_>,
) -> std::cmp::Ordering {
    return std::cmp::Ord::cmp(
        &(x.with(|__s| __s.it_.clone())),
        &(y.with(|__s| __s.it_.clone())),
    );
}
pub fn operator_eq_7(
    x: Ptr<three_way_contiguous_iterator_char_ptr_>,
    y: Ptr<three_way_contiguous_iterator_char_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct three_way_contiguous_iterator_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i8>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl three_way_contiguous_iterator_char_ptr_ {
    pub fn new_1(mut it: Ptr<i8>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<three_way_contiguous_iterator_char_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppLess for three_way_contiguous_iterator_char_ptr_ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_6(__lhs.clone(), __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for three_way_contiguous_iterator_char_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_7(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for three_way_contiguous_iterator_char_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn base_8(
    i: three_way_contiguous_iterator_char_ptr_,
) -> three_way_contiguous_iterator_char_ptr_ {
    let i: Value<three_way_contiguous_iterator_char_ptr_> = Rc::new(RefCell::new(i));
    return three_way_contiguous_iterator_char_ptr_::move_from({ i.as_pointer() });
}
pub fn base_9(i: Ptr<i8>) -> Ptr<i8> {
    let i: Value<Ptr<i8>> = Rc::new(RefCell::new(i));
    return (*i.borrow()).clone();
}
pub fn base_10(i: Ptr<i8>) -> Ptr<i8> {
    let i: Value<Ptr<i8>> = Rc::new(RefCell::new(i));
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
pub fn base_11(i: Ptr<cpp20_input_iterator_char_ptr_>) -> Ptr<i8> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_input_iterator_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i8>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_char_ptr_ {
    pub fn new(mut it: Ptr<i8>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<cpp20_input_iterator_char_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for cpp20_input_iterator_char_ptr_ {
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<i8>,
}
impl sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr__ {
    pub fn new_1(it: Ptr<cpp17_input_iterator_char_ptr__char_ptr_>) -> Self {
        Self {
            base_: ({ base_1((it).clone()) }),
        }
    }
}
impl CppMove for sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_cpp20_input_iterator_char_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<i8>,
}
impl sentinel_wrapper_cpp20_input_iterator_char_ptr__ {
    pub fn new_1(it: Ptr<cpp20_input_iterator_char_ptr_>) -> Self {
        Self {
            base_: ({ base_11((it).clone()) }),
        }
    }
}
impl CppMove for sentinel_wrapper_cpp20_input_iterator_char_ptr__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_forward_iterator_char_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<i8>,
}
impl sentinel_wrapper_forward_iterator_char_ptr__ {
    pub fn new_1(it: Ptr<forward_iterator_char_ptr_>) -> Self {
        Self {
            base_: ({ base_2((it).clone()) }),
        }
    }
}
impl CppMove for sentinel_wrapper_forward_iterator_char_ptr__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_bidirectional_iterator_char_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<i8>,
}
impl sentinel_wrapper_bidirectional_iterator_char_ptr__ {
    pub fn new_1(it: Ptr<bidirectional_iterator_char_ptr_>) -> Self {
        Self {
            base_: ({ base_3((it).clone()) }),
        }
    }
}
impl CppMove for sentinel_wrapper_bidirectional_iterator_char_ptr__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_random_access_iterator_char_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<i8>,
}
impl sentinel_wrapper_random_access_iterator_char_ptr__ {
    pub fn new_1(it: Ptr<random_access_iterator_char_ptr_>) -> Self {
        Self {
            base_: ({ base_4((it).clone()) }),
        }
    }
}
impl CppMove for sentinel_wrapper_random_access_iterator_char_ptr__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_contiguous_iterator_char_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<i8>,
}
impl sentinel_wrapper_contiguous_iterator_char_ptr__ {
    pub fn new_1(it: Ptr<contiguous_iterator_char_ptr_>) -> Self {
        Self {
            base_: ({ base_5((it).clone()) }),
        }
    }
}
impl CppMove for sentinel_wrapper_contiguous_iterator_char_ptr__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct sentinel_wrapper_three_way_contiguous_iterator_char_ptr__ {
    #[offset(0)]
    #[byte_size(16)]
    base_: three_way_contiguous_iterator_char_ptr_,
}
impl sentinel_wrapper_three_way_contiguous_iterator_char_ptr__ {
    pub fn new_1(it: Ptr<three_way_contiguous_iterator_char_ptr_>) -> Self {
        Self {
            base_: ({ base_8((*it.upgrade().deref()).clone()) }),
        }
    }
    pub fn move_from(_a0: Ptr<sentinel_wrapper_three_way_contiguous_iterator_char_ptr__>) -> Self {
        Self {
            base_: three_way_contiguous_iterator_char_ptr_::move_from({ field_ptr!(_a0, base_) }),
        }
    }
}
impl CppMove for sentinel_wrapper_three_way_contiguous_iterator_char_ptr__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<i8>,
}
impl sentinel_wrapper_char_ptr_ {
    pub fn new_1(it: Ptr<Ptr<i8>>) -> Self {
        Self {
            base_: ({ base_9((it.read())) }),
        }
    }
}
impl CppMove for sentinel_wrapper_char_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_const_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<i8>,
}
impl sentinel_wrapper_const_char_ptr_ {
    pub fn new_1(it: Ptr<Ptr<i8>>) -> Self {
        Self {
            base_: ({ base_10((it.read())) }),
        }
    }
}
impl CppMove for sentinel_wrapper_const_char_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
        a.iter_swaps_.with_mut(|__v| {
            __v.prefix_inc();
        });
    }
}
pub fn iter_move_16(iter: adl_Iterator) -> Ptr<i32> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| {
            __v.prefix_inc();
        });
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
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_cmp_18(__lhs.clone(), __rhs.clone())
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
                operator_eq_17(__lhs.clone(), __rhs.clone())
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
const _: () = assert!(
    !(false),
    "!HasEquals<std::move_iterator<int*>, std::move_sentinel<char*>>"
);
const _: () = assert!(
    !(false),
    "!HasNotEquals<std::move_iterator<int*>, std::move_sentinel<char*>>"
);
const _: () = assert!(
    !(false),
    "!HasLess<std::move_iterator<int*>, std::move_sentinel<char*>>"
);
const _: () = assert!(
    true,
    "HasEquals<std::move_iterator<int*>, std::move_sentinel<const int*>>"
);
const _: () = assert!(
    true,
    "HasNotEquals<std::move_iterator<int*>, std::move_sentinel<const int*>>"
);
const _: () = assert!(
    !(false),
    "!HasLess<std::move_iterator<int*>, std::move_sentinel<const int*>>"
);
const _: () = assert!(
    true,
    "HasEquals<std::move_iterator<const int*>, std::move_sentinel<int*>>"
);
const _: () = assert!(
    true,
    "HasNotEquals<std::move_iterator<const int*>, std::move_sentinel<int*>>"
);
const _: () = assert!(
    !(false),
    "!HasLess<std::move_iterator<const int*>, std::move_sentinel<int*>>"
);
pub fn test_one_19() {
    let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abc\0")));
    let it: Value<std_move_iterator_cpp17_input_iterator_char_ptr__char_ptr__> =
        Rc::new(RefCell::new(
            std_move_iterator_cpp17_input_iterator_char_ptr__char_ptr__::new_1({
                cpp17_input_iterator_char_ptr__char_ptr_::new({ (s.as_pointer() as Ptr<i8>) })
            }),
        ));
    let sent1: Value<
        std_move_sentinel_sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr___,
    > = Rc::new(RefCell::new(
        std_move_sentinel_sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr___::new_1({
            {
                let __tmp_0: Value<cpp17_input_iterator_char_ptr__char_ptr_> =
                    Rc::new(RefCell::new(cpp17_input_iterator_char_ptr__char_ptr_::new(
                        { (s.as_pointer() as Ptr<i8>) },
                    )));
                sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr__::new_1({
                    __tmp_0.as_pointer()
                })
            }
        }),
    ));
    let sent2: Value<
        std_move_sentinel_sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr___,
    > = Rc::new(RefCell::new(
        std_move_sentinel_sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr___::new_1({
            {
                let __tmp_1: Value<cpp17_input_iterator_char_ptr__char_ptr_> =
                    Rc::new(RefCell::new(cpp17_input_iterator_char_ptr__char_ptr_::new(
                        { (s.as_pointer() as Ptr<i8>).offset((1) as isize) },
                    )));
                sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr__::new_1({
                    __tmp_1.as_pointer()
                })
            }
        }),
    ));
    const _: () = assert!(true, "");;
    assert!(({ ((*it.borrow()) == (*sent1.borrow())) }));
    assert!(({ !((*it.borrow()) != (*sent1.borrow())) }));
    assert!(({ !((*it.borrow()) == (*sent2.borrow())) }));
    assert!(((*it.borrow()) != (*sent2.borrow())));
    assert!(((*sent1.borrow()) == (*it.borrow())));
    assert!(({ !((*sent1.borrow()) != (*it.borrow())) }));
    assert!(({ !((*sent2.borrow()) == (*it.borrow())) }));
    assert!(((*sent2.borrow()) != (*it.borrow())));
    const _: () = assert!(!(false), "!HasEquals<decltype(sent1), decltype(sent1)>");;
    const _: () = assert!(!(false), "!HasLess<decltype(sent1), decltype(sent1)>");;
}
pub fn test_one_20() {
    let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abc\0")));
    let it: Value<std_move_iterator_cpp20_input_iterator_char_ptr__> = Rc::new(RefCell::new(
        std_move_iterator_cpp20_input_iterator_char_ptr__::new_1({
            cpp20_input_iterator_char_ptr_::new({ (s.as_pointer() as Ptr<i8>) })
        }),
    ));
    let sent1: Value<std_move_sentinel_sentinel_wrapper_cpp20_input_iterator_char_ptr___> =
        Rc::new(RefCell::new(
            std_move_sentinel_sentinel_wrapper_cpp20_input_iterator_char_ptr___::new_1({
                {
                    let __tmp_2: Value<cpp20_input_iterator_char_ptr_> =
                        Rc::new(RefCell::new(cpp20_input_iterator_char_ptr_::new({
                            (s.as_pointer() as Ptr<i8>)
                        })));
                    sentinel_wrapper_cpp20_input_iterator_char_ptr__::new_1({
                        __tmp_2.as_pointer()
                    })
                }
            }),
        ));
    let sent2: Value<std_move_sentinel_sentinel_wrapper_cpp20_input_iterator_char_ptr___> =
        Rc::new(RefCell::new(
            std_move_sentinel_sentinel_wrapper_cpp20_input_iterator_char_ptr___::new_1({
                {
                    let __tmp_3: Value<cpp20_input_iterator_char_ptr_> =
                        Rc::new(RefCell::new(cpp20_input_iterator_char_ptr_::new({
                            (s.as_pointer() as Ptr<i8>).offset((1) as isize)
                        })));
                    sentinel_wrapper_cpp20_input_iterator_char_ptr__::new_1({
                        __tmp_3.as_pointer()
                    })
                }
            }),
        ));
    const _: () = assert!(true, "");;
    assert!(({ ((*it.borrow()) == (*sent1.borrow())) }));
    assert!(({ !((*it.borrow()) != (*sent1.borrow())) }));
    assert!(({ !((*it.borrow()) == (*sent2.borrow())) }));
    assert!(((*it.borrow()) != (*sent2.borrow())));
    assert!(((*sent1.borrow()) == (*it.borrow())));
    assert!(({ !((*sent1.borrow()) != (*it.borrow())) }));
    assert!(({ !((*sent2.borrow()) == (*it.borrow())) }));
    assert!(((*sent2.borrow()) != (*it.borrow())));
    const _: () = assert!(!(false), "!HasEquals<decltype(sent1), decltype(sent1)>");;
    const _: () = assert!(!(false), "!HasLess<decltype(sent1), decltype(sent1)>");;
}
pub fn test_one_21() {
    let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abc\0")));
    let it: Value<std_move_iterator_forward_iterator_char_ptr__> = Rc::new(RefCell::new(
        std_move_iterator_forward_iterator_char_ptr__::new_1({
            forward_iterator_char_ptr_::new_1({ (s.as_pointer() as Ptr<i8>) })
        }),
    ));
    let sent1: Value<std_move_sentinel_sentinel_wrapper_forward_iterator_char_ptr___> =
        Rc::new(RefCell::new(
            std_move_sentinel_sentinel_wrapper_forward_iterator_char_ptr___::new_1({
                {
                    let __tmp_4: Value<forward_iterator_char_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_char_ptr_::new_1({
                            (s.as_pointer() as Ptr<i8>)
                        })));
                    sentinel_wrapper_forward_iterator_char_ptr__::new_1({ __tmp_4.as_pointer() })
                }
            }),
        ));
    let sent2: Value<std_move_sentinel_sentinel_wrapper_forward_iterator_char_ptr___> =
        Rc::new(RefCell::new(
            std_move_sentinel_sentinel_wrapper_forward_iterator_char_ptr___::new_1({
                {
                    let __tmp_5: Value<forward_iterator_char_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_char_ptr_::new_1({
                            (s.as_pointer() as Ptr<i8>).offset((1) as isize)
                        })));
                    sentinel_wrapper_forward_iterator_char_ptr__::new_1({ __tmp_5.as_pointer() })
                }
            }),
        ));
    const _: () = assert!(true, "");;
    assert!(({ ((*it.borrow()) == (*sent1.borrow())) }));
    assert!(({ !((*it.borrow()) != (*sent1.borrow())) }));
    assert!(({ !((*it.borrow()) == (*sent2.borrow())) }));
    assert!(((*it.borrow()) != (*sent2.borrow())));
    assert!(((*sent1.borrow()) == (*it.borrow())));
    assert!(({ !((*sent1.borrow()) != (*it.borrow())) }));
    assert!(({ !((*sent2.borrow()) == (*it.borrow())) }));
    assert!(((*sent2.borrow()) != (*it.borrow())));
    const _: () = assert!(!(false), "!HasEquals<decltype(sent1), decltype(sent1)>");;
    const _: () = assert!(!(false), "!HasLess<decltype(sent1), decltype(sent1)>");;
}
pub fn test_one_22() {
    let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abc\0")));
    let it: Value<BidirectionalIterator<i8>> = Rc::new(RefCell::new(
        std_move_iterator_bidirectional_iterator_char_ptr__::new_1({
            bidirectional_iterator_char_ptr_::new_1({ (s.as_pointer() as Ptr<i8>) })
        }),
    ));
    let sent1: Value<std_move_sentinel_sentinel_wrapper_bidirectional_iterator_char_ptr___> =
        Rc::new(RefCell::new(
            std_move_sentinel_sentinel_wrapper_bidirectional_iterator_char_ptr___::new_1({
                {
                    let __tmp_6: Value<bidirectional_iterator_char_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_char_ptr_::new_1({
                            (s.as_pointer() as Ptr<i8>)
                        })));
                    sentinel_wrapper_bidirectional_iterator_char_ptr__::new_1({
                        __tmp_6.as_pointer()
                    })
                }
            }),
        ));
    let sent2: Value<std_move_sentinel_sentinel_wrapper_bidirectional_iterator_char_ptr___> =
        Rc::new(RefCell::new(
            std_move_sentinel_sentinel_wrapper_bidirectional_iterator_char_ptr___::new_1({
                {
                    let __tmp_7: Value<bidirectional_iterator_char_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_char_ptr_::new_1({
                            (s.as_pointer() as Ptr<i8>).offset((1) as isize)
                        })));
                    sentinel_wrapper_bidirectional_iterator_char_ptr__::new_1({
                        __tmp_7.as_pointer()
                    })
                }
            }),
        ));
    const _: () = assert!(true, "");;
    assert!(({ ((*it.borrow()) == (*sent1.borrow())) }));
    assert!(({ !((*it.borrow()) != (*sent1.borrow())) }));
    assert!(({ !((*it.borrow()) == (*sent2.borrow())) }));
    assert!(((*it.borrow()) != (*sent2.borrow())));
    assert!(((*sent1.borrow()) == (*it.borrow())));
    assert!(({ !((*sent1.borrow()) != (*it.borrow())) }));
    assert!(({ !((*sent2.borrow()) == (*it.borrow())) }));
    assert!(((*sent2.borrow()) != (*it.borrow())));
    const _: () = assert!(!(false), "!HasEquals<decltype(sent1), decltype(sent1)>");;
    const _: () = assert!(!(false), "!HasLess<decltype(sent1), decltype(sent1)>");;
}
pub fn test_one_23() {
    let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abc\0")));
    let it: Value<std_move_iterator_random_access_iterator_char_ptr__> = Rc::new(RefCell::new(
        std_move_iterator_random_access_iterator_char_ptr__::new_1({
            random_access_iterator_char_ptr_::new_1({ (s.as_pointer() as Ptr<i8>) })
        }),
    ));
    let sent1: Value<std_move_sentinel_sentinel_wrapper_random_access_iterator_char_ptr___> =
        Rc::new(RefCell::new(
            std_move_sentinel_sentinel_wrapper_random_access_iterator_char_ptr___::new_1({
                {
                    let __tmp_8: Value<random_access_iterator_char_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_char_ptr_::new_1({
                            (s.as_pointer() as Ptr<i8>)
                        })));
                    sentinel_wrapper_random_access_iterator_char_ptr__::new_1({
                        __tmp_8.as_pointer()
                    })
                }
            }),
        ));
    let sent2: Value<std_move_sentinel_sentinel_wrapper_random_access_iterator_char_ptr___> =
        Rc::new(RefCell::new(
            std_move_sentinel_sentinel_wrapper_random_access_iterator_char_ptr___::new_1({
                {
                    let __tmp_9: Value<random_access_iterator_char_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_char_ptr_::new_1({
                            (s.as_pointer() as Ptr<i8>).offset((1) as isize)
                        })));
                    sentinel_wrapper_random_access_iterator_char_ptr__::new_1({
                        __tmp_9.as_pointer()
                    })
                }
            }),
        ));
    const _: () = assert!(true, "");;
    assert!(({ ((*it.borrow()) == (*sent1.borrow())) }));
    assert!(({ !((*it.borrow()) != (*sent1.borrow())) }));
    assert!(({ !((*it.borrow()) == (*sent2.borrow())) }));
    assert!(((*it.borrow()) != (*sent2.borrow())));
    assert!(((*sent1.borrow()) == (*it.borrow())));
    assert!(({ !((*sent1.borrow()) != (*it.borrow())) }));
    assert!(({ !((*sent2.borrow()) == (*it.borrow())) }));
    assert!(((*sent2.borrow()) != (*it.borrow())));
    const _: () = assert!(!(false), "!HasEquals<decltype(sent1), decltype(sent1)>");;
    const _: () = assert!(!(false), "!HasLess<decltype(sent1), decltype(sent1)>");;
}
pub fn test_one_24() {
    let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abc\0")));
    let it: Value<std_move_iterator_contiguous_iterator_char_ptr__> = Rc::new(RefCell::new(
        std_move_iterator_contiguous_iterator_char_ptr__::new_1({
            contiguous_iterator_char_ptr_::new_1({ (s.as_pointer() as Ptr<i8>) })
        }),
    ));
    let sent1: Value<std_move_sentinel_sentinel_wrapper_contiguous_iterator_char_ptr___> =
        Rc::new(RefCell::new(
            std_move_sentinel_sentinel_wrapper_contiguous_iterator_char_ptr___::new_1({
                {
                    let __tmp_10: Value<contiguous_iterator_char_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_char_ptr_::new_1({
                            (s.as_pointer() as Ptr<i8>)
                        })));
                    sentinel_wrapper_contiguous_iterator_char_ptr__::new_1({
                        __tmp_10.as_pointer()
                    })
                }
            }),
        ));
    let sent2: Value<std_move_sentinel_sentinel_wrapper_contiguous_iterator_char_ptr___> =
        Rc::new(RefCell::new(
            std_move_sentinel_sentinel_wrapper_contiguous_iterator_char_ptr___::new_1({
                {
                    let __tmp_11: Value<contiguous_iterator_char_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_char_ptr_::new_1({
                            (s.as_pointer() as Ptr<i8>).offset((1) as isize)
                        })));
                    sentinel_wrapper_contiguous_iterator_char_ptr__::new_1({
                        __tmp_11.as_pointer()
                    })
                }
            }),
        ));
    const _: () = assert!(true, "");;
    assert!(({ ((*it.borrow()) == (*sent1.borrow())) }));
    assert!(({ !((*it.borrow()) != (*sent1.borrow())) }));
    assert!(({ !((*it.borrow()) == (*sent2.borrow())) }));
    assert!(((*it.borrow()) != (*sent2.borrow())));
    assert!(((*sent1.borrow()) == (*it.borrow())));
    assert!(({ !((*sent1.borrow()) != (*it.borrow())) }));
    assert!(({ !((*sent2.borrow()) == (*it.borrow())) }));
    assert!(((*sent2.borrow()) != (*it.borrow())));
    const _: () = assert!(!(false), "!HasEquals<decltype(sent1), decltype(sent1)>");;
    const _: () = assert!(!(false), "!HasLess<decltype(sent1), decltype(sent1)>");;
}
pub fn test_one_25() {
    let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abc\0")));
    let it: Value<std_move_iterator_three_way_contiguous_iterator_char_ptr__> =
        Rc::new(RefCell::new(
            std_move_iterator_three_way_contiguous_iterator_char_ptr__::new_1({
                three_way_contiguous_iterator_char_ptr_::new_1({ (s.as_pointer() as Ptr<i8>) })
            }),
        ));
    let sent1: Value<std_move_sentinel_sentinel_wrapper_three_way_contiguous_iterator_char_ptr___> =
        Rc::new(RefCell::new(
            std_move_sentinel_sentinel_wrapper_three_way_contiguous_iterator_char_ptr___::new_1({
                {
                    let __tmp_12: Value<three_way_contiguous_iterator_char_ptr_> = Rc::new(
                        RefCell::new(three_way_contiguous_iterator_char_ptr_::new_1({
                            (s.as_pointer() as Ptr<i8>)
                        })),
                    );
                    sentinel_wrapper_three_way_contiguous_iterator_char_ptr__::new_1({
                        __tmp_12.as_pointer()
                    })
                }
            }),
        ));
    let sent2: Value<std_move_sentinel_sentinel_wrapper_three_way_contiguous_iterator_char_ptr___> =
        Rc::new(RefCell::new(
            std_move_sentinel_sentinel_wrapper_three_way_contiguous_iterator_char_ptr___::new_1({
                {
                    let __tmp_13: Value<three_way_contiguous_iterator_char_ptr_> = Rc::new(
                        RefCell::new(three_way_contiguous_iterator_char_ptr_::new_1({
                            (s.as_pointer() as Ptr<i8>).offset((1) as isize)
                        })),
                    );
                    sentinel_wrapper_three_way_contiguous_iterator_char_ptr__::new_1({
                        __tmp_13.as_pointer()
                    })
                }
            }),
        ));
    const _: () = assert!(true, "");;
    assert!(({ ((*it.borrow()) == (*sent1.borrow())) }));
    assert!(({ !((*it.borrow()) != (*sent1.borrow())) }));
    assert!(({ !((*it.borrow()) == (*sent2.borrow())) }));
    assert!(((*it.borrow()) != (*sent2.borrow())));
    assert!(((*sent1.borrow()) == (*it.borrow())));
    assert!(({ !((*sent1.borrow()) != (*it.borrow())) }));
    assert!(({ !((*sent2.borrow()) == (*it.borrow())) }));
    assert!(((*sent2.borrow()) != (*it.borrow())));
    const _: () = assert!(!(false), "!HasEquals<decltype(sent1), decltype(sent1)>");;
    const _: () = assert!(!(false), "!HasLess<decltype(sent1), decltype(sent1)>");;
}
pub fn test_one_26() {
    let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abc\0")));
    let it: Value<std_move_iterator_char_ptr_> =
        Rc::new(RefCell::new(std_move_iterator_char_ptr_::new_1({
            (s.as_pointer() as Ptr<i8>)
        })));
    let sent1: Value<std_move_sentinel_sentinel_wrapper_char_ptr__> = Rc::new(RefCell::new(
        std_move_sentinel_sentinel_wrapper_char_ptr__::new_1({
            {
                let __tmp_14: Value<Ptr<i8>> = Rc::new(RefCell::new((s.as_pointer() as Ptr<i8>)));
                sentinel_wrapper_char_ptr_::new_1({ __tmp_14.as_pointer() })
            }
        }),
    ));
    let sent2: Value<std_move_sentinel_sentinel_wrapper_char_ptr__> = Rc::new(RefCell::new(
        std_move_sentinel_sentinel_wrapper_char_ptr__::new_1({
            {
                let __tmp_15: Value<Ptr<i8>> = Rc::new(RefCell::new(
                    (s.as_pointer() as Ptr<i8>).offset((1) as isize),
                ));
                sentinel_wrapper_char_ptr_::new_1({ __tmp_15.as_pointer() })
            }
        }),
    ));
    const _: () = assert!(true, "");;
    assert!(({ ((*it.borrow()) == (*sent1.borrow())) }));
    assert!(({ !((*it.borrow()) != (*sent1.borrow())) }));
    assert!(({ !((*it.borrow()) == (*sent2.borrow())) }));
    assert!(((*it.borrow()) != (*sent2.borrow())));
    assert!(((*sent1.borrow()) == (*it.borrow())));
    assert!(({ !((*sent1.borrow()) != (*it.borrow())) }));
    assert!(({ !((*sent2.borrow()) == (*it.borrow())) }));
    assert!(((*sent2.borrow()) != (*it.borrow())));
    const _: () = assert!(!(false), "!HasEquals<decltype(sent1), decltype(sent1)>");;
    const _: () = assert!(!(false), "!HasLess<decltype(sent1), decltype(sent1)>");;
}
pub fn test_one_27() {
    let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abc\0")));
    let it: Value<std_move_iterator_const_char_ptr_> =
        Rc::new(RefCell::new(std_move_iterator_const_char_ptr_::new_1({
            (s.as_pointer() as Ptr<i8>)
        })));
    let sent1: Value<std_move_sentinel_sentinel_wrapper_const_char_ptr__> = Rc::new(RefCell::new(
        std_move_sentinel_sentinel_wrapper_const_char_ptr__::new_1({
            {
                let __tmp_16: Value<Ptr<i8>> = Rc::new(RefCell::new((s.as_pointer() as Ptr<i8>)));
                sentinel_wrapper_const_char_ptr_::new_1({ __tmp_16.as_pointer() })
            }
        }),
    ));
    let sent2: Value<std_move_sentinel_sentinel_wrapper_const_char_ptr__> = Rc::new(RefCell::new(
        std_move_sentinel_sentinel_wrapper_const_char_ptr__::new_1({
            {
                let __tmp_17: Value<Ptr<i8>> = Rc::new(RefCell::new(
                    (s.as_pointer() as Ptr<i8>).offset((1) as isize),
                ));
                sentinel_wrapper_const_char_ptr_::new_1({ __tmp_17.as_pointer() })
            }
        }),
    ));
    const _: () = assert!(true, "");;
    assert!(({ ((*it.borrow()) == (*sent1.borrow())) }));
    assert!(({ !((*it.borrow()) != (*sent1.borrow())) }));
    assert!(({ !((*it.borrow()) == (*sent2.borrow())) }));
    assert!(((*it.borrow()) != (*sent2.borrow())));
    assert!(((*sent1.borrow()) == (*it.borrow())));
    assert!(({ !((*sent1.borrow()) != (*it.borrow())) }));
    assert!(({ !((*sent2.borrow()) == (*it.borrow())) }));
    assert!(((*sent2.borrow()) != (*it.borrow())));
    const _: () = assert!(!(false), "!HasEquals<decltype(sent1), decltype(sent1)>");;
    const _: () = assert!(!(false), "!HasLess<decltype(sent1), decltype(sent1)>");;
}
pub fn test_28() -> bool {
    ({ test_one_19() });
    ({ test_one_20() });
    ({ test_one_21() });
    ({ test_one_22() });
    ({ test_one_23() });
    ({ test_one_24() });
    ({ test_one_25() });
    ({ test_one_26() });
    ({ test_one_27() });
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
    ({ test_28() });
    const _: () = assert!(true, "test()");;
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
pub trait bidirectional_iterator_char_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_char_ptr_> {
        unimplemented!()
    }
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_char_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_8(&self, mut _a0: i32) -> bidirectional_iterator_char_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_9(&self, mut _a0: i32) -> bidirectional_iterator_char_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_char_ptr_>,
    ) -> Ptr<bidirectional_iterator_char_ptr_>;
}
impl bidirectional_iterator_char_ptr_Impl for Ptr<bidirectional_iterator_char_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_char_ptr_>,
    ) -> Ptr<bidirectional_iterator_char_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait contiguous_iterator_char_ptr_Impl {
    fn base(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_char_ptr_> {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_char_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: i32) -> contiguous_iterator_char_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: i32) -> contiguous_iterator_char_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<contiguous_iterator_char_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<contiguous_iterator_char_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_char_ptr_>,
    ) -> Ptr<contiguous_iterator_char_ptr_>;
}
impl contiguous_iterator_char_ptr_Impl for Ptr<contiguous_iterator_char_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_char_ptr_>,
    ) -> Ptr<contiguous_iterator_char_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait cpp17_input_iterator_char_ptr__char_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_char_ptr__char_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: i32) -> cpp17_input_iterator_char_ptr__char_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_input_iterator_char_ptr__char_ptr_>,
    ) -> Ptr<cpp17_input_iterator_char_ptr__char_ptr_>;
}
impl cpp17_input_iterator_char_ptr__char_ptr_Impl
    for Ptr<cpp17_input_iterator_char_ptr__char_ptr_>
{
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_input_iterator_char_ptr__char_ptr_>,
    ) -> Ptr<cpp17_input_iterator_char_ptr__char_ptr_> {
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
pub trait cpp20_input_iterator_char_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_input_iterator_char_ptr_>,
    ) -> Ptr<cpp20_input_iterator_char_ptr_>;
    fn operator_deref(&self) -> Ptr<i8>;
    fn operator_inc_5(&self) -> Ptr<cpp20_input_iterator_char_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: i32) {
        unimplemented!()
    }
}
impl cpp20_input_iterator_char_ptr_Impl for Ptr<cpp20_input_iterator_char_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_input_iterator_char_ptr_>,
    ) -> Ptr<cpp20_input_iterator_char_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
    fn operator_deref(&self) -> Ptr<i8> {
        return (*self).with(|__s| __s.it_.clone());
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
pub trait forward_iterator_char_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<forward_iterator_char_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_7(&self, mut _a0: i32) -> forward_iterator_char_ptr_ {
        unimplemented!()
    }
    fn move_assign(&self, _a0: Ptr<forward_iterator_char_ptr_>) -> Ptr<forward_iterator_char_ptr_>;
}
impl forward_iterator_char_ptr_Impl for Ptr<forward_iterator_char_ptr_> {
    fn move_assign(&self, _a0: Ptr<forward_iterator_char_ptr_>) -> Ptr<forward_iterator_char_ptr_> {
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
pub trait random_access_iterator_char_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_char_ptr_> {
        unimplemented!()
    }
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_char_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: i32) -> random_access_iterator_char_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: i32) -> random_access_iterator_char_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<random_access_iterator_char_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<random_access_iterator_char_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_char_ptr_>,
    ) -> Ptr<random_access_iterator_char_ptr_>;
}
impl random_access_iterator_char_ptr_Impl for Ptr<random_access_iterator_char_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_char_ptr_>,
    ) -> Ptr<random_access_iterator_char_ptr_> {
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
pub trait sentinel_wrapper_bidirectional_iterator_char_ptr__Impl {
    fn operator_eq(&self, other: Ptr<bidirectional_iterator_char_ptr_>) -> bool;
}
impl sentinel_wrapper_bidirectional_iterator_char_ptr__Impl
    for Ptr<sentinel_wrapper_bidirectional_iterator_char_ptr__>
{
    fn operator_eq(&self, other: Ptr<bidirectional_iterator_char_ptr_>) -> bool {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_3((other).clone()) }) });
    }
}
pub trait sentinel_wrapper_char_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<i8>>) -> bool;
}
impl sentinel_wrapper_char_ptr_Impl for Ptr<sentinel_wrapper_char_ptr_> {
    fn operator_eq(&self, other: Ptr<Ptr<i8>>) -> bool {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_9((other.read())) }) });
    }
}
pub trait sentinel_wrapper_const_char_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<i8>>) -> bool;
}
impl sentinel_wrapper_const_char_ptr_Impl for Ptr<sentinel_wrapper_const_char_ptr_> {
    fn operator_eq(&self, other: Ptr<Ptr<i8>>) -> bool {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_10((other.read())) }) });
    }
}
pub trait sentinel_wrapper_contiguous_iterator_char_ptr__Impl {
    fn operator_eq(&self, other: Ptr<contiguous_iterator_char_ptr_>) -> bool;
}
impl sentinel_wrapper_contiguous_iterator_char_ptr__Impl
    for Ptr<sentinel_wrapper_contiguous_iterator_char_ptr__>
{
    fn operator_eq(&self, other: Ptr<contiguous_iterator_char_ptr_>) -> bool {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_5((other).clone()) }) });
    }
}
pub trait sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr__Impl {
    fn operator_eq(&self, other: Ptr<cpp17_input_iterator_char_ptr__char_ptr_>) -> bool;
}
impl sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr__Impl
    for Ptr<sentinel_wrapper_cpp17_input_iterator_char_ptr__char_ptr__>
{
    fn operator_eq(&self, other: Ptr<cpp17_input_iterator_char_ptr__char_ptr_>) -> bool {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_1((other).clone()) }) });
    }
}
pub trait sentinel_wrapper_cpp20_input_iterator_char_ptr__Impl {
    fn operator_eq(&self, other: Ptr<cpp20_input_iterator_char_ptr_>) -> bool;
}
impl sentinel_wrapper_cpp20_input_iterator_char_ptr__Impl
    for Ptr<sentinel_wrapper_cpp20_input_iterator_char_ptr__>
{
    fn operator_eq(&self, other: Ptr<cpp20_input_iterator_char_ptr_>) -> bool {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_11((other).clone()) }) });
    }
}
pub trait sentinel_wrapper_forward_iterator_char_ptr__Impl {
    fn operator_eq(&self, other: Ptr<forward_iterator_char_ptr_>) -> bool;
}
impl sentinel_wrapper_forward_iterator_char_ptr__Impl
    for Ptr<sentinel_wrapper_forward_iterator_char_ptr__>
{
    fn operator_eq(&self, other: Ptr<forward_iterator_char_ptr_>) -> bool {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_2((other).clone()) }) });
    }
}
pub trait sentinel_wrapper_random_access_iterator_char_ptr__Impl {
    fn operator_eq(&self, other: Ptr<random_access_iterator_char_ptr_>) -> bool;
}
impl sentinel_wrapper_random_access_iterator_char_ptr__Impl
    for Ptr<sentinel_wrapper_random_access_iterator_char_ptr__>
{
    fn operator_eq(&self, other: Ptr<random_access_iterator_char_ptr_>) -> bool {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_4((other).clone()) }) });
    }
}
pub trait sentinel_wrapper_three_way_contiguous_iterator_char_ptr__Impl {
    fn operator_eq(&self, other: Ptr<three_way_contiguous_iterator_char_ptr_>) -> bool;
    fn move_assign(
        &self,
        _a0: Ptr<sentinel_wrapper_three_way_contiguous_iterator_char_ptr__>,
    ) -> Ptr<sentinel_wrapper_three_way_contiguous_iterator_char_ptr__>;
}
impl sentinel_wrapper_three_way_contiguous_iterator_char_ptr__Impl
    for Ptr<sentinel_wrapper_three_way_contiguous_iterator_char_ptr__>
{
    fn operator_eq(&self, other: Ptr<three_way_contiguous_iterator_char_ptr_>) -> bool {
        {}
        return ({
            let _x: Ptr<three_way_contiguous_iterator_char_ptr_> = field_ptr!((*self), base_);
            let _y: Value<three_way_contiguous_iterator_char_ptr_> = Rc::new(RefCell::new(
                ({ base_8((*other.upgrade().deref()).clone()) }),
            ));
            operator_eq_7(_x, _y.as_pointer())
        });
    }
    fn move_assign(
        &self,
        _a0: Ptr<sentinel_wrapper_three_way_contiguous_iterator_char_ptr__>,
    ) -> Ptr<sentinel_wrapper_three_way_contiguous_iterator_char_ptr__> {
        {
            let __a1 =
                three_way_contiguous_iterator_char_ptr_::move_from({ field_ptr!(_a0, base_) });
            field_ptr!((*self), base_).with_mut(|__it| *__it = __a1);
            field_ptr!((*self), base_)
        };
        return (*self).clone();
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
pub trait three_way_contiguous_iterator_char_ptr_Impl {
    fn base(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<three_way_contiguous_iterator_char_ptr_> {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<three_way_contiguous_iterator_char_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: i32) -> three_way_contiguous_iterator_char_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: i32) -> three_way_contiguous_iterator_char_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<three_way_contiguous_iterator_char_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<three_way_contiguous_iterator_char_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<three_way_contiguous_iterator_char_ptr_>,
    ) -> Ptr<three_way_contiguous_iterator_char_ptr_>;
}
impl three_way_contiguous_iterator_char_ptr_Impl for Ptr<three_way_contiguous_iterator_char_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<three_way_contiguous_iterator_char_ptr_>,
    ) -> Ptr<three_way_contiguous_iterator_char_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
