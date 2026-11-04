extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(Ptr< i8  >  , to_pconsti8)]
#[cpp_into(Ptr< i32  >  , to_pconsti32)]
#[cpp_into(Ptr< u8  >  , to_pconstu8)]
#[cpp_into(Ptr< u16  >  , to_pconstu16)]
#[cpp_into(Ptr< u32  >  , to_pconstu32)]
#[byte_size(80)]
pub struct MultiStringType {
    #[offset(0)]
    #[byte_size(8)]
    pub w_: Ptr<i32>,
    #[offset(8)]
    pub wn_: usize,
    #[offset(16)]
    #[byte_size(8)]
    pub u8_: Ptr<u8>,
    #[offset(24)]
    pub u8n_: usize,
    #[offset(32)]
    #[byte_size(8)]
    pub u16_: Ptr<u16>,
    #[offset(40)]
    pub u16n_: usize,
    #[offset(48)]
    #[byte_size(8)]
    pub u32_: Ptr<u32>,
    #[offset(56)]
    pub u32n_: usize,
    #[offset(64)]
    #[byte_size(8)]
    pub s_: Ptr<i8>,
    #[offset(72)]
    pub sn_: usize,
}
impl MultiStringType {
    pub fn new(
        mut w: Ptr<i32>,
        mut wn: usize,
        mut u8: Ptr<u8>,
        mut u8n: usize,
        mut u16: Ptr<u16>,
        mut u16n: usize,
        mut u32: Ptr<u32>,
        mut u32n: usize,
        mut s: Ptr<i8>,
        mut sn: usize,
    ) -> Self {
        Self {
            w_: (w).clone(),
            wn_: wn,
            u8_: (u8).clone(),
            u8n_: u8n,
            u16_: (u16).clone(),
            u16n_: u16n,
            u32_: (u32).clone(),
            u32n_: u32n,
            s_: (s).clone(),
            sn_: sn,
        }
    }
}
impl CppMove for MultiStringType {
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
pub fn operator_sub_0(
    mut x: contiguous_iterator_const_char_ptr_,
    mut y: contiguous_iterator_const_char_ptr_,
) -> isize {
    return (((x.it_).clone() - (y.it_).clone()) as i64 as isize);
}
pub fn base_1(i: Ptr<contiguous_iterator_const_char_ptr_>) -> Ptr<i8> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_const_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i8>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_char_ptr_ {
    pub fn new_1(mut it: Ptr<i8>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<contiguous_iterator_const_char_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for contiguous_iterator_const_char_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_sub_2(
    mut x: contiguous_iterator_const_wchar_t_ptr_,
    mut y: contiguous_iterator_const_wchar_t_ptr_,
) -> isize {
    return (((x.it_).clone() - (y.it_).clone()) as i64 as isize);
}
pub fn base_3(i: Ptr<contiguous_iterator_const_wchar_t_ptr_>) -> Ptr<i32> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_const_wchar_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_wchar_t_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<contiguous_iterator_const_wchar_t_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for contiguous_iterator_const_wchar_t_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_sub_4(
    mut x: contiguous_iterator_const_char8_t_ptr_,
    mut y: contiguous_iterator_const_char8_t_ptr_,
) -> isize {
    return (((x.it_).clone() - (y.it_).clone()) as i64 as isize);
}
pub fn base_5(i: Ptr<contiguous_iterator_const_char8_t_ptr_>) -> Ptr<u8> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_const_char8_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<u8>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_char8_t_ptr_ {
    pub fn new_1(mut it: Ptr<u8>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<contiguous_iterator_const_char8_t_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for contiguous_iterator_const_char8_t_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_sub_6(
    mut x: contiguous_iterator_const_char16_t_ptr_,
    mut y: contiguous_iterator_const_char16_t_ptr_,
) -> isize {
    return (((x.it_).clone() - (y.it_).clone()) as i64 as isize);
}
pub fn base_7(i: Ptr<contiguous_iterator_const_char16_t_ptr_>) -> Ptr<u16> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_const_char16_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<u16>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_char16_t_ptr_ {
    pub fn new_1(mut it: Ptr<u16>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<contiguous_iterator_const_char16_t_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for contiguous_iterator_const_char16_t_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_sub_8(
    mut x: contiguous_iterator_const_char32_t_ptr_,
    mut y: contiguous_iterator_const_char32_t_ptr_,
) -> isize {
    return (((x.it_).clone() - (y.it_).clone()) as i64 as isize);
}
pub fn base_9(i: Ptr<contiguous_iterator_const_char32_t_ptr_>) -> Ptr<u32> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_const_char32_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<u32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_char32_t_ptr_ {
    pub fn new_1(mut it: Ptr<u32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<contiguous_iterator_const_char32_t_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for contiguous_iterator_const_char32_t_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
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
pub fn base_10(i: Ptr<i8>) -> Ptr<i8> {
    let i: Value<Ptr<i8>> = Rc::new(RefCell::new(i));
    return (*i.borrow()).clone();
}
pub fn base_11(i: Ptr<i32>) -> Ptr<i32> {
    let i: Value<Ptr<i32>> = Rc::new(RefCell::new(i));
    return (*i.borrow()).clone();
}
pub fn base_12(i: Ptr<u8>) -> Ptr<u8> {
    let i: Value<Ptr<u8>> = Rc::new(RefCell::new(i));
    return (*i.borrow()).clone();
}
pub fn base_13(i: Ptr<u16>) -> Ptr<u16> {
    let i: Value<Ptr<u16>> = Rc::new(RefCell::new(i));
    return (*i.borrow()).clone();
}
pub fn base_14(i: Ptr<u32>) -> Ptr<u32> {
    let i: Value<Ptr<u32>> = Rc::new(RefCell::new(i));
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
pub fn operator_sub_15(s: Ptr<sized_sentinel_const_char_ptr_>, i: Ptr<Ptr<i8>>) -> i64 {
    return (s.with(|__s| __s.base_.clone()) - ({ base_10((i.read())) })) as i64;
}
pub fn operator_sub_16(i: Ptr<Ptr<i8>>, s: Ptr<sized_sentinel_const_char_ptr_>) -> i64 {
    return (({ base_10((i.read())) }) - s.with(|__s| __s.base_.clone())) as i64;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_const_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<i8>,
}
impl sized_sentinel_const_char_ptr_ {
    pub fn new_1(it: Ptr<Ptr<i8>>) -> Self {
        Self {
            base_: ({ base_10((it.read())) }),
        }
    }
}
impl CppMove for sized_sentinel_const_char_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_sub_17(
    s: Ptr<sized_sentinel_contiguous_iterator_const_char_ptr__>,
    i: Ptr<contiguous_iterator_const_char_ptr_>,
) -> i64 {
    return (s.with(|__s| __s.base_.clone()) - ({ base_1((i).clone()) })) as i64;
}
pub fn operator_sub_18(
    i: Ptr<contiguous_iterator_const_char_ptr_>,
    s: Ptr<sized_sentinel_contiguous_iterator_const_char_ptr__>,
) -> i64 {
    return (({ base_1((i).clone()) }) - s.with(|__s| __s.base_.clone())) as i64;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_contiguous_iterator_const_char_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<i8>,
}
impl sized_sentinel_contiguous_iterator_const_char_ptr__ {
    pub fn new_1(it: Ptr<contiguous_iterator_const_char_ptr_>) -> Self {
        Self {
            base_: ({ base_1((it).clone()) }),
        }
    }
}
impl CppMove for sized_sentinel_contiguous_iterator_const_char_ptr__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_sub_19(s: Ptr<sized_sentinel_const_wchar_t_ptr_>, i: Ptr<Ptr<i32>>) -> i64 {
    return (s.with(|__s| __s.base_.clone()) - ({ base_11((i.read())) })) as i64;
}
pub fn operator_sub_20(i: Ptr<Ptr<i32>>, s: Ptr<sized_sentinel_const_wchar_t_ptr_>) -> i64 {
    return (({ base_11((i.read())) }) - s.with(|__s| __s.base_.clone())) as i64;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_const_wchar_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<i32>,
}
impl sized_sentinel_const_wchar_t_ptr_ {
    pub fn new_1(it: Ptr<Ptr<i32>>) -> Self {
        Self {
            base_: ({ base_11((it.read())) }),
        }
    }
}
impl CppMove for sized_sentinel_const_wchar_t_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_sub_21(
    s: Ptr<sized_sentinel_contiguous_iterator_const_wchar_t_ptr__>,
    i: Ptr<contiguous_iterator_const_wchar_t_ptr_>,
) -> i64 {
    return (s.with(|__s| __s.base_.clone()) - ({ base_3((i).clone()) })) as i64;
}
pub fn operator_sub_22(
    i: Ptr<contiguous_iterator_const_wchar_t_ptr_>,
    s: Ptr<sized_sentinel_contiguous_iterator_const_wchar_t_ptr__>,
) -> i64 {
    return (({ base_3((i).clone()) }) - s.with(|__s| __s.base_.clone())) as i64;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_contiguous_iterator_const_wchar_t_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<i32>,
}
impl sized_sentinel_contiguous_iterator_const_wchar_t_ptr__ {
    pub fn new_1(it: Ptr<contiguous_iterator_const_wchar_t_ptr_>) -> Self {
        Self {
            base_: ({ base_3((it).clone()) }),
        }
    }
}
impl CppMove for sized_sentinel_contiguous_iterator_const_wchar_t_ptr__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_sub_23(s: Ptr<sized_sentinel_const_char8_t_ptr_>, i: Ptr<Ptr<u8>>) -> i64 {
    return (s.with(|__s| __s.base_.clone()) - ({ base_12((i.read())) })) as i64;
}
pub fn operator_sub_24(i: Ptr<Ptr<u8>>, s: Ptr<sized_sentinel_const_char8_t_ptr_>) -> i64 {
    return (({ base_12((i.read())) }) - s.with(|__s| __s.base_.clone())) as i64;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_const_char8_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<u8>,
}
impl sized_sentinel_const_char8_t_ptr_ {
    pub fn new_1(it: Ptr<Ptr<u8>>) -> Self {
        Self {
            base_: ({ base_12((it.read())) }),
        }
    }
}
impl CppMove for sized_sentinel_const_char8_t_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_sub_25(
    s: Ptr<sized_sentinel_contiguous_iterator_const_char8_t_ptr__>,
    i: Ptr<contiguous_iterator_const_char8_t_ptr_>,
) -> i64 {
    return (s.with(|__s| __s.base_.clone()) - ({ base_5((i).clone()) })) as i64;
}
pub fn operator_sub_26(
    i: Ptr<contiguous_iterator_const_char8_t_ptr_>,
    s: Ptr<sized_sentinel_contiguous_iterator_const_char8_t_ptr__>,
) -> i64 {
    return (({ base_5((i).clone()) }) - s.with(|__s| __s.base_.clone())) as i64;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_contiguous_iterator_const_char8_t_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<u8>,
}
impl sized_sentinel_contiguous_iterator_const_char8_t_ptr__ {
    pub fn new_1(it: Ptr<contiguous_iterator_const_char8_t_ptr_>) -> Self {
        Self {
            base_: ({ base_5((it).clone()) }),
        }
    }
}
impl CppMove for sized_sentinel_contiguous_iterator_const_char8_t_ptr__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_sub_27(s: Ptr<sized_sentinel_const_char16_t_ptr_>, i: Ptr<Ptr<u16>>) -> i64 {
    return (s.with(|__s| __s.base_.clone()) - ({ base_13((i.read())) })) as i64;
}
pub fn operator_sub_28(i: Ptr<Ptr<u16>>, s: Ptr<sized_sentinel_const_char16_t_ptr_>) -> i64 {
    return (({ base_13((i.read())) }) - s.with(|__s| __s.base_.clone())) as i64;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_const_char16_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<u16>,
}
impl sized_sentinel_const_char16_t_ptr_ {
    pub fn new_1(it: Ptr<Ptr<u16>>) -> Self {
        Self {
            base_: ({ base_13((it.read())) }),
        }
    }
}
impl CppMove for sized_sentinel_const_char16_t_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_sub_29(
    s: Ptr<sized_sentinel_contiguous_iterator_const_char16_t_ptr__>,
    i: Ptr<contiguous_iterator_const_char16_t_ptr_>,
) -> i64 {
    return (s.with(|__s| __s.base_.clone()) - ({ base_7((i).clone()) })) as i64;
}
pub fn operator_sub_30(
    i: Ptr<contiguous_iterator_const_char16_t_ptr_>,
    s: Ptr<sized_sentinel_contiguous_iterator_const_char16_t_ptr__>,
) -> i64 {
    return (({ base_7((i).clone()) }) - s.with(|__s| __s.base_.clone())) as i64;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_contiguous_iterator_const_char16_t_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<u16>,
}
impl sized_sentinel_contiguous_iterator_const_char16_t_ptr__ {
    pub fn new_1(it: Ptr<contiguous_iterator_const_char16_t_ptr_>) -> Self {
        Self {
            base_: ({ base_7((it).clone()) }),
        }
    }
}
impl CppMove for sized_sentinel_contiguous_iterator_const_char16_t_ptr__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_sub_31(s: Ptr<sized_sentinel_const_char32_t_ptr_>, i: Ptr<Ptr<u32>>) -> i64 {
    return (s.with(|__s| __s.base_.clone()) - ({ base_14((i.read())) })) as i64;
}
pub fn operator_sub_32(i: Ptr<Ptr<u32>>, s: Ptr<sized_sentinel_const_char32_t_ptr_>) -> i64 {
    return (({ base_14((i.read())) }) - s.with(|__s| __s.base_.clone())) as i64;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_const_char32_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<u32>,
}
impl sized_sentinel_const_char32_t_ptr_ {
    pub fn new_1(it: Ptr<Ptr<u32>>) -> Self {
        Self {
            base_: ({ base_14((it.read())) }),
        }
    }
}
impl CppMove for sized_sentinel_const_char32_t_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_sub_33(
    s: Ptr<sized_sentinel_contiguous_iterator_const_char32_t_ptr__>,
    i: Ptr<contiguous_iterator_const_char32_t_ptr_>,
) -> i64 {
    return (s.with(|__s| __s.base_.clone()) - ({ base_9((i).clone()) })) as i64;
}
pub fn operator_sub_34(
    i: Ptr<contiguous_iterator_const_char32_t_ptr_>,
    s: Ptr<sized_sentinel_contiguous_iterator_const_char32_t_ptr__>,
) -> i64 {
    return (({ base_9((i).clone()) }) - s.with(|__s| __s.base_.clone())) as i64;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_contiguous_iterator_const_char32_t_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<u32>,
}
impl sized_sentinel_contiguous_iterator_const_char32_t_ptr__ {
    pub fn new_1(it: Ptr<contiguous_iterator_const_char32_t_ptr_>) -> Self {
        Self {
            base_: ({ base_9((it).clone()) }),
        }
    }
}
impl CppMove for sized_sentinel_contiguous_iterator_const_char32_t_ptr__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_35(
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
        operator_eq_35(__lhs.clone(), __rhs.clone())
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
pub fn operator_add_36(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_37(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_36(_i, n)
    });
}
pub fn iter_swap_38(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap!((a.ptr_).clone(), (b.ptr_).clone());
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| {
            __v.prefix_inc();
        });
    }
}
pub fn iter_move_39(iter: adl_Iterator) -> Ptr<i32> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| {
            __v.prefix_inc();
        });
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_40(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> bool {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_41(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> std::cmp::Ordering {
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
                operator_cmp_41(__lhs.clone(), __rhs.clone())
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
                operator_eq_40(__lhs.clone(), __rhs.clone())
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
pub fn test_construction_42(val: std_basic_string_view_char__std_char_traits_char__) {
    let val: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_1(
            { ({ (*val.borrow()).data() }) },
            { ({ (*val.borrow()).data() }).offset(({ (*val.borrow()).size() }) as isize) },
        ),
    ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_43(val: std_basic_string_view_char__std_char_traits_char__) {
    let val: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_2(
            { ({ (*val.borrow()).data() }) },
            { ({ (*val.borrow()).data() }).offset(({ (*val.borrow()).size() }) as isize) },
        ),
    ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_44(val: std_basic_string_view_char__std_char_traits_char__) {
    let val: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_3(
            { ({ (*val.borrow()).data() }) },
            { ({ (*val.borrow()).data() }).offset(({ (*val.borrow()).size() }) as isize) },
        ),
    ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_45(val: std_basic_string_view_char__std_char_traits_char__) {
    let val: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_4(
            { ({ (*val.borrow()).data() }) },
            {
                {
                    let __tmp_0: Value<Ptr<i8>> = Rc::new(RefCell::new(
                        ({ (*val.borrow()).data() }).offset(({ (*val.borrow()).size() }) as isize),
                    ));
                    sized_sentinel_const_char_ptr_::new_1({ __tmp_0.as_pointer() })
                }
            },
        ),
    ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_46(val: std_basic_string_view_char__std_char_traits_char__) {
    let val: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_5(
            { contiguous_iterator_const_char_ptr_::new_1({ ({ (*val.borrow()).data() }) }) },
            {
                contiguous_iterator_const_char_ptr_::new_1({
                    ({ (*val.borrow()).data() }).offset(({ (*val.borrow()).size() }) as isize)
                })
            },
        ),
    ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_47(val: std_basic_string_view_char__std_char_traits_char__) {
    let val: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_6(
            { contiguous_iterator_const_char_ptr_::new_1({ ({ (*val.borrow()).data() }) }) },
            {
                {
                    let __tmp_1: Value<contiguous_iterator_const_char_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_const_char_ptr_::new_1({
                            ({ (*val.borrow()).data() })
                                .offset(({ (*val.borrow()).size() }) as isize)
                        })));
                    sized_sentinel_contiguous_iterator_const_char_ptr__::new_1({
                        __tmp_1.as_pointer()
                    })
                }
            },
        ),
    ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_48(val: std_basic_string_view_wchar_t__std_char_traits_wchar_t__) {
    let val: Value<std_basic_string_view_wchar_t__std_char_traits_wchar_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_wchar_t__std_char_traits_wchar_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_wchar_t__std_char_traits_wchar_t__::new_1(
                { ({ (*val.borrow()).data() }) },
                { ({ (*val.borrow()).data() }).offset(({ (*val.borrow()).size() }) as isize) },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_49(val: std_basic_string_view_wchar_t__std_char_traits_wchar_t__) {
    let val: Value<std_basic_string_view_wchar_t__std_char_traits_wchar_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_wchar_t__std_char_traits_wchar_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_wchar_t__std_char_traits_wchar_t__::new_2(
                { ({ (*val.borrow()).data() }) },
                { ({ (*val.borrow()).data() }).offset(({ (*val.borrow()).size() }) as isize) },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_50(val: std_basic_string_view_wchar_t__std_char_traits_wchar_t__) {
    let val: Value<std_basic_string_view_wchar_t__std_char_traits_wchar_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_wchar_t__std_char_traits_wchar_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_wchar_t__std_char_traits_wchar_t__::new_3(
                { ({ (*val.borrow()).data() }) },
                { ({ (*val.borrow()).data() }).offset(({ (*val.borrow()).size() }) as isize) },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_51(val: std_basic_string_view_wchar_t__std_char_traits_wchar_t__) {
    let val: Value<std_basic_string_view_wchar_t__std_char_traits_wchar_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_wchar_t__std_char_traits_wchar_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_wchar_t__std_char_traits_wchar_t__::new_4(
                { ({ (*val.borrow()).data() }) },
                {
                    {
                        let __tmp_2: Value<Ptr<i32>> = Rc::new(RefCell::new(
                            ({ (*val.borrow()).data() })
                                .offset(({ (*val.borrow()).size() }) as isize),
                        ));
                        sized_sentinel_const_wchar_t_ptr_::new_1({ __tmp_2.as_pointer() })
                    }
                },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_52(val: std_basic_string_view_wchar_t__std_char_traits_wchar_t__) {
    let val: Value<std_basic_string_view_wchar_t__std_char_traits_wchar_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_wchar_t__std_char_traits_wchar_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_wchar_t__std_char_traits_wchar_t__::new_5(
                { contiguous_iterator_const_wchar_t_ptr_::new_1({ ({ (*val.borrow()).data() }) }) },
                {
                    contiguous_iterator_const_wchar_t_ptr_::new_1({
                        ({ (*val.borrow()).data() }).offset(({ (*val.borrow()).size() }) as isize)
                    })
                },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_53(val: std_basic_string_view_wchar_t__std_char_traits_wchar_t__) {
    let val: Value<std_basic_string_view_wchar_t__std_char_traits_wchar_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_wchar_t__std_char_traits_wchar_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_wchar_t__std_char_traits_wchar_t__::new_6(
                { contiguous_iterator_const_wchar_t_ptr_::new_1({ ({ (*val.borrow()).data() }) }) },
                {
                    {
                        let __tmp_3: Value<contiguous_iterator_const_wchar_t_ptr_> = Rc::new(
                            RefCell::new(contiguous_iterator_const_wchar_t_ptr_::new_1({
                                ({ (*val.borrow()).data() })
                                    .offset(({ (*val.borrow()).size() }) as isize)
                            })),
                        );
                        sized_sentinel_contiguous_iterator_const_wchar_t_ptr__::new_1({
                            __tmp_3.as_pointer()
                        })
                    }
                },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_54(val: std_basic_string_view_char8_t__std_char_traits_char8_t__) {
    let val: Value<std_basic_string_view_char8_t__std_char_traits_char8_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char8_t__std_char_traits_char8_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char8_t__std_char_traits_char8_t__::new_1(
                { ({ (*val.borrow()).data() }) },
                { ({ (*val.borrow()).data() }).offset(({ (*val.borrow()).size() }) as isize) },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_55(val: std_basic_string_view_char8_t__std_char_traits_char8_t__) {
    let val: Value<std_basic_string_view_char8_t__std_char_traits_char8_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char8_t__std_char_traits_char8_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char8_t__std_char_traits_char8_t__::new_2(
                { ({ (*val.borrow()).data() }) },
                { ({ (*val.borrow()).data() }).offset(({ (*val.borrow()).size() }) as isize) },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_56(val: std_basic_string_view_char8_t__std_char_traits_char8_t__) {
    let val: Value<std_basic_string_view_char8_t__std_char_traits_char8_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char8_t__std_char_traits_char8_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char8_t__std_char_traits_char8_t__::new_3(
                { ({ (*val.borrow()).data() }) },
                { ({ (*val.borrow()).data() }).offset(({ (*val.borrow()).size() }) as isize) },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_57(val: std_basic_string_view_char8_t__std_char_traits_char8_t__) {
    let val: Value<std_basic_string_view_char8_t__std_char_traits_char8_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char8_t__std_char_traits_char8_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char8_t__std_char_traits_char8_t__::new_4(
                { ({ (*val.borrow()).data() }) },
                {
                    {
                        let __tmp_4: Value<Ptr<u8>> = Rc::new(RefCell::new(
                            ({ (*val.borrow()).data() })
                                .offset(({ (*val.borrow()).size() }) as isize),
                        ));
                        sized_sentinel_const_char8_t_ptr_::new_1({ __tmp_4.as_pointer() })
                    }
                },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_58(val: std_basic_string_view_char8_t__std_char_traits_char8_t__) {
    let val: Value<std_basic_string_view_char8_t__std_char_traits_char8_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char8_t__std_char_traits_char8_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char8_t__std_char_traits_char8_t__::new_5(
                { contiguous_iterator_const_char8_t_ptr_::new_1({ ({ (*val.borrow()).data() }) }) },
                {
                    contiguous_iterator_const_char8_t_ptr_::new_1({
                        ({ (*val.borrow()).data() }).offset(({ (*val.borrow()).size() }) as isize)
                    })
                },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_59(val: std_basic_string_view_char8_t__std_char_traits_char8_t__) {
    let val: Value<std_basic_string_view_char8_t__std_char_traits_char8_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char8_t__std_char_traits_char8_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char8_t__std_char_traits_char8_t__::new_6(
                { contiguous_iterator_const_char8_t_ptr_::new_1({ ({ (*val.borrow()).data() }) }) },
                {
                    {
                        let __tmp_5: Value<contiguous_iterator_const_char8_t_ptr_> = Rc::new(
                            RefCell::new(contiguous_iterator_const_char8_t_ptr_::new_1({
                                ({ (*val.borrow()).data() })
                                    .offset(({ (*val.borrow()).size() }) as isize)
                            })),
                        );
                        sized_sentinel_contiguous_iterator_const_char8_t_ptr__::new_1({
                            __tmp_5.as_pointer()
                        })
                    }
                },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_60(val: std_basic_string_view_char16_t__std_char_traits_char16_t__) {
    let val: Value<std_basic_string_view_char16_t__std_char_traits_char16_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char16_t__std_char_traits_char16_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char16_t__std_char_traits_char16_t__::new_1(
                { ({ (*val.borrow()).data() }) },
                { ({ (*val.borrow()).data() }).offset(({ (*val.borrow()).size() }) as isize) },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_61(val: std_basic_string_view_char16_t__std_char_traits_char16_t__) {
    let val: Value<std_basic_string_view_char16_t__std_char_traits_char16_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char16_t__std_char_traits_char16_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char16_t__std_char_traits_char16_t__::new_2(
                { ({ (*val.borrow()).data() }) },
                { ({ (*val.borrow()).data() }).offset(({ (*val.borrow()).size() }) as isize) },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_62(val: std_basic_string_view_char16_t__std_char_traits_char16_t__) {
    let val: Value<std_basic_string_view_char16_t__std_char_traits_char16_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char16_t__std_char_traits_char16_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char16_t__std_char_traits_char16_t__::new_3(
                { ({ (*val.borrow()).data() }) },
                { ({ (*val.borrow()).data() }).offset(({ (*val.borrow()).size() }) as isize) },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_63(val: std_basic_string_view_char16_t__std_char_traits_char16_t__) {
    let val: Value<std_basic_string_view_char16_t__std_char_traits_char16_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char16_t__std_char_traits_char16_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char16_t__std_char_traits_char16_t__::new_4(
                { ({ (*val.borrow()).data() }) },
                {
                    {
                        let __tmp_6: Value<Ptr<u16>> = Rc::new(RefCell::new(
                            ({ (*val.borrow()).data() })
                                .offset(({ (*val.borrow()).size() }) as isize),
                        ));
                        sized_sentinel_const_char16_t_ptr_::new_1({ __tmp_6.as_pointer() })
                    }
                },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_64(val: std_basic_string_view_char16_t__std_char_traits_char16_t__) {
    let val: Value<std_basic_string_view_char16_t__std_char_traits_char16_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char16_t__std_char_traits_char16_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char16_t__std_char_traits_char16_t__::new_5(
                {
                    contiguous_iterator_const_char16_t_ptr_::new_1({ ({ (*val.borrow()).data() }) })
                },
                {
                    contiguous_iterator_const_char16_t_ptr_::new_1({
                        ({ (*val.borrow()).data() }).offset(({ (*val.borrow()).size() }) as isize)
                    })
                },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_65(val: std_basic_string_view_char16_t__std_char_traits_char16_t__) {
    let val: Value<std_basic_string_view_char16_t__std_char_traits_char16_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char16_t__std_char_traits_char16_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char16_t__std_char_traits_char16_t__::new_6(
                {
                    contiguous_iterator_const_char16_t_ptr_::new_1({ ({ (*val.borrow()).data() }) })
                },
                {
                    {
                        let __tmp_7: Value<contiguous_iterator_const_char16_t_ptr_> = Rc::new(
                            RefCell::new(contiguous_iterator_const_char16_t_ptr_::new_1({
                                ({ (*val.borrow()).data() })
                                    .offset(({ (*val.borrow()).size() }) as isize)
                            })),
                        );
                        sized_sentinel_contiguous_iterator_const_char16_t_ptr__::new_1({
                            __tmp_7.as_pointer()
                        })
                    }
                },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_66(val: std_basic_string_view_char32_t__std_char_traits_char32_t__) {
    let val: Value<std_basic_string_view_char32_t__std_char_traits_char32_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char32_t__std_char_traits_char32_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char32_t__std_char_traits_char32_t__::new_1(
                { ({ (*val.borrow()).data() }) },
                { ({ (*val.borrow()).data() }).offset(({ (*val.borrow()).size() }) as isize) },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_67(val: std_basic_string_view_char32_t__std_char_traits_char32_t__) {
    let val: Value<std_basic_string_view_char32_t__std_char_traits_char32_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char32_t__std_char_traits_char32_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char32_t__std_char_traits_char32_t__::new_2(
                { ({ (*val.borrow()).data() }) },
                { ({ (*val.borrow()).data() }).offset(({ (*val.borrow()).size() }) as isize) },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_68(val: std_basic_string_view_char32_t__std_char_traits_char32_t__) {
    let val: Value<std_basic_string_view_char32_t__std_char_traits_char32_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char32_t__std_char_traits_char32_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char32_t__std_char_traits_char32_t__::new_3(
                { ({ (*val.borrow()).data() }) },
                { ({ (*val.borrow()).data() }).offset(({ (*val.borrow()).size() }) as isize) },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_69(val: std_basic_string_view_char32_t__std_char_traits_char32_t__) {
    let val: Value<std_basic_string_view_char32_t__std_char_traits_char32_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char32_t__std_char_traits_char32_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char32_t__std_char_traits_char32_t__::new_4(
                { ({ (*val.borrow()).data() }) },
                {
                    {
                        let __tmp_8: Value<Ptr<u32>> = Rc::new(RefCell::new(
                            ({ (*val.borrow()).data() })
                                .offset(({ (*val.borrow()).size() }) as isize),
                        ));
                        sized_sentinel_const_char32_t_ptr_::new_1({ __tmp_8.as_pointer() })
                    }
                },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_70(val: std_basic_string_view_char32_t__std_char_traits_char32_t__) {
    let val: Value<std_basic_string_view_char32_t__std_char_traits_char32_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char32_t__std_char_traits_char32_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char32_t__std_char_traits_char32_t__::new_5(
                {
                    contiguous_iterator_const_char32_t_ptr_::new_1({ ({ (*val.borrow()).data() }) })
                },
                {
                    contiguous_iterator_const_char32_t_ptr_::new_1({
                        ({ (*val.borrow()).data() }).offset(({ (*val.borrow()).size() }) as isize)
                    })
                },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_construction_71(val: std_basic_string_view_char32_t__std_char_traits_char32_t__) {
    let val: Value<std_basic_string_view_char32_t__std_char_traits_char32_t__> =
        Rc::new(RefCell::new(val));
    let sv: Value<std_basic_string_view_char32_t__std_char_traits_char32_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char32_t__std_char_traits_char32_t__::new_6(
                {
                    contiguous_iterator_const_char32_t_ptr_::new_1({ ({ (*val.borrow()).data() }) })
                },
                {
                    {
                        let __tmp_9: Value<contiguous_iterator_const_char32_t_ptr_> = Rc::new(
                            RefCell::new(contiguous_iterator_const_char32_t_ptr_::new_1({
                                ({ (*val.borrow()).data() })
                                    .offset(({ (*val.borrow()).size() }) as isize)
                            })),
                        );
                        sized_sentinel_contiguous_iterator_const_char32_t_ptr__::new_1({
                            __tmp_9.as_pointer()
                        })
                    }
                },
            ),
        ));
    assert!(({ (({ (*sv.borrow()).data() }) == ({ (*val.borrow()).data() })) }));
    assert!(({ (({ (*sv.borrow()).size() }) == ({ (*val.borrow()).size() })) }));
}
pub fn test_with_char_72() {
    let val: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_7(
            {
                ({
                    MultiStringTypeImpl::as_ptr_2(
                        &Rc::new(RefCell::new(MultiStringType::new(
                            {
                                Ptr::<i32>::from_string_literal(&[
                                    116 as i32, 101 as i32, 115 as i32, 116 as i32, 0 as i32,
                                ])
                            },
                            {
                                ((::std::mem::size_of::<[i32; 5]>() as usize)
                                    .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                    as usize)
                                    .wrapping_sub(1_usize)
                            },
                            {
                                Ptr::<u8>::from_string_literal(&[
                                    116 as u8, 101 as u8, 115 as u8, 116 as u8, 0 as u8,
                                ])
                            },
                            {
                                ((::std::mem::size_of::<[u8; 5]>() as usize)
                                    .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                    as usize)
                                    .wrapping_sub(1_usize)
                            },
                            {
                                Ptr::<u16>::from_string_literal(&[
                                    116 as u16, 101 as u16, 115 as u16, 116 as u16, 0 as u16,
                                ])
                            },
                            {
                                ((::std::mem::size_of::<[u16; 5]>() as usize)
                                    .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                    as usize)
                                    .wrapping_sub(1_usize)
                            },
                            {
                                Ptr::<u32>::from_string_literal(&[
                                    116 as u32, 101 as u32, 115 as u32, 116 as u32, 0 as u32,
                                ])
                            },
                            {
                                ((::std::mem::size_of::<[u32; 5]>() as usize)
                                    .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                    as usize)
                                    .wrapping_sub(1_usize)
                            },
                            { Ptr::<i8>::from_string_literal(b"test") },
                            { (::std::mem::size_of::<[i8; 5]>() as usize).wrapping_sub(1_usize) },
                        )))
                        .as_pointer(),
                        Ptr::<i8>::null(),
                    )
                })
            },
            {
                (({
                    MultiStringTypeImpl::length_7(
                        &Rc::new(RefCell::new(MultiStringType::new(
                            {
                                Ptr::<i32>::from_string_literal(&[
                                    116 as i32, 101 as i32, 115 as i32, 116 as i32, 0 as i32,
                                ])
                            },
                            {
                                ((::std::mem::size_of::<[i32; 5]>() as usize)
                                    .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                    as usize)
                                    .wrapping_sub(1_usize)
                            },
                            {
                                Ptr::<u8>::from_string_literal(&[
                                    116 as u8, 101 as u8, 115 as u8, 116 as u8, 0 as u8,
                                ])
                            },
                            {
                                ((::std::mem::size_of::<[u8; 5]>() as usize)
                                    .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                    as usize)
                                    .wrapping_sub(1_usize)
                            },
                            {
                                Ptr::<u16>::from_string_literal(&[
                                    116 as u16, 101 as u16, 115 as u16, 116 as u16, 0 as u16,
                                ])
                            },
                            {
                                ((::std::mem::size_of::<[u16; 5]>() as usize)
                                    .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                    as usize)
                                    .wrapping_sub(1_usize)
                            },
                            {
                                Ptr::<u32>::from_string_literal(&[
                                    116 as u32, 101 as u32, 115 as u32, 116 as u32, 0 as u32,
                                ])
                            },
                            {
                                ((::std::mem::size_of::<[u32; 5]>() as usize)
                                    .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                    as usize)
                                    .wrapping_sub(1_usize)
                            },
                            { Ptr::<i8>::from_string_literal(b"test") },
                            { (::std::mem::size_of::<[i8; 5]>() as usize).wrapping_sub(1_usize) },
                        )))
                        .as_pointer(),
                        Ptr::<i8>::null(),
                    )
                }) as u64)
            },
        ),
    ));
    ({ test_construction_42((*val.borrow()).clone()) });
    ({ test_construction_43((*val.borrow()).clone()) });
    ({ test_construction_44((*val.borrow()).clone()) });
    ({ test_construction_45((*val.borrow()).clone()) });
    ({ test_construction_46((*val.borrow()).clone()) });
    ({ test_construction_47((*val.borrow()).clone()) });
}
pub fn test_with_char_73() {
    let val: Value<std_basic_string_view_wchar_t__std_char_traits_wchar_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_wchar_t__std_char_traits_wchar_t__::new_7(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_3(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<i32>::from_string_literal(&[
                                        116 as i32, 101 as i32, 115 as i32, 116 as i32, 0 as i32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[i32; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u8>::from_string_literal(&[
                                        116 as u8, 101 as u8, 115 as u8, 116 as u8, 0 as u8,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u8; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u16>::from_string_literal(&[
                                        116 as u16, 101 as u16, 115 as u16, 116 as u16, 0 as u16,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u16; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u32>::from_string_literal(&[
                                        116 as u32, 101 as u32, 115 as u32, 116 as u32, 0 as u32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u32; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b"test") },
                                {
                                    (::std::mem::size_of::<[i8; 5]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<i32>::null(),
                        )
                    })
                },
                {
                    (({
                        MultiStringTypeImpl::length_8(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<i32>::from_string_literal(&[
                                        116 as i32, 101 as i32, 115 as i32, 116 as i32, 0 as i32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[i32; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u8>::from_string_literal(&[
                                        116 as u8, 101 as u8, 115 as u8, 116 as u8, 0 as u8,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u8; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u16>::from_string_literal(&[
                                        116 as u16, 101 as u16, 115 as u16, 116 as u16, 0 as u16,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u16; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u32>::from_string_literal(&[
                                        116 as u32, 101 as u32, 115 as u32, 116 as u32, 0 as u32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u32; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b"test") },
                                {
                                    (::std::mem::size_of::<[i8; 5]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<i32>::null(),
                        )
                    }) as u64)
                },
            ),
        ));
    ({ test_construction_48((*val.borrow()).clone()) });
    ({ test_construction_49((*val.borrow()).clone()) });
    ({ test_construction_50((*val.borrow()).clone()) });
    ({ test_construction_51((*val.borrow()).clone()) });
    ({ test_construction_52((*val.borrow()).clone()) });
    ({ test_construction_53((*val.borrow()).clone()) });
}
pub fn test_with_char_74() {
    let val: Value<std_basic_string_view_char8_t__std_char_traits_char8_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char8_t__std_char_traits_char8_t__::new_7(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_4(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<i32>::from_string_literal(&[
                                        116 as i32, 101 as i32, 115 as i32, 116 as i32, 0 as i32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[i32; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u8>::from_string_literal(&[
                                        116 as u8, 101 as u8, 115 as u8, 116 as u8, 0 as u8,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u8; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u16>::from_string_literal(&[
                                        116 as u16, 101 as u16, 115 as u16, 116 as u16, 0 as u16,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u16; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u32>::from_string_literal(&[
                                        116 as u32, 101 as u32, 115 as u32, 116 as u32, 0 as u32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u32; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b"test") },
                                {
                                    (::std::mem::size_of::<[i8; 5]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<u8>::null(),
                        )
                    })
                },
                {
                    (({
                        MultiStringTypeImpl::length_9(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<i32>::from_string_literal(&[
                                        116 as i32, 101 as i32, 115 as i32, 116 as i32, 0 as i32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[i32; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u8>::from_string_literal(&[
                                        116 as u8, 101 as u8, 115 as u8, 116 as u8, 0 as u8,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u8; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u16>::from_string_literal(&[
                                        116 as u16, 101 as u16, 115 as u16, 116 as u16, 0 as u16,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u16; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u32>::from_string_literal(&[
                                        116 as u32, 101 as u32, 115 as u32, 116 as u32, 0 as u32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u32; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b"test") },
                                {
                                    (::std::mem::size_of::<[i8; 5]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<u8>::null(),
                        )
                    }) as u64)
                },
            ),
        ));
    ({ test_construction_54((*val.borrow()).clone()) });
    ({ test_construction_55((*val.borrow()).clone()) });
    ({ test_construction_56((*val.borrow()).clone()) });
    ({ test_construction_57((*val.borrow()).clone()) });
    ({ test_construction_58((*val.borrow()).clone()) });
    ({ test_construction_59((*val.borrow()).clone()) });
}
pub fn test_with_char_75() {
    let val: Value<std_basic_string_view_char16_t__std_char_traits_char16_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char16_t__std_char_traits_char16_t__::new_7(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_5(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<i32>::from_string_literal(&[
                                        116 as i32, 101 as i32, 115 as i32, 116 as i32, 0 as i32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[i32; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u8>::from_string_literal(&[
                                        116 as u8, 101 as u8, 115 as u8, 116 as u8, 0 as u8,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u8; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u16>::from_string_literal(&[
                                        116 as u16, 101 as u16, 115 as u16, 116 as u16, 0 as u16,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u16; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u32>::from_string_literal(&[
                                        116 as u32, 101 as u32, 115 as u32, 116 as u32, 0 as u32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u32; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b"test") },
                                {
                                    (::std::mem::size_of::<[i8; 5]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<u16>::null(),
                        )
                    })
                },
                {
                    (({
                        MultiStringTypeImpl::length_10(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<i32>::from_string_literal(&[
                                        116 as i32, 101 as i32, 115 as i32, 116 as i32, 0 as i32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[i32; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u8>::from_string_literal(&[
                                        116 as u8, 101 as u8, 115 as u8, 116 as u8, 0 as u8,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u8; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u16>::from_string_literal(&[
                                        116 as u16, 101 as u16, 115 as u16, 116 as u16, 0 as u16,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u16; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u32>::from_string_literal(&[
                                        116 as u32, 101 as u32, 115 as u32, 116 as u32, 0 as u32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u32; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b"test") },
                                {
                                    (::std::mem::size_of::<[i8; 5]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<u16>::null(),
                        )
                    }) as u64)
                },
            ),
        ));
    ({ test_construction_60((*val.borrow()).clone()) });
    ({ test_construction_61((*val.borrow()).clone()) });
    ({ test_construction_62((*val.borrow()).clone()) });
    ({ test_construction_63((*val.borrow()).clone()) });
    ({ test_construction_64((*val.borrow()).clone()) });
    ({ test_construction_65((*val.borrow()).clone()) });
}
pub fn test_with_char_76() {
    let val: Value<std_basic_string_view_char32_t__std_char_traits_char32_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char32_t__std_char_traits_char32_t__::new_7(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_6(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<i32>::from_string_literal(&[
                                        116 as i32, 101 as i32, 115 as i32, 116 as i32, 0 as i32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[i32; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u8>::from_string_literal(&[
                                        116 as u8, 101 as u8, 115 as u8, 116 as u8, 0 as u8,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u8; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u16>::from_string_literal(&[
                                        116 as u16, 101 as u16, 115 as u16, 116 as u16, 0 as u16,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u16; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u32>::from_string_literal(&[
                                        116 as u32, 101 as u32, 115 as u32, 116 as u32, 0 as u32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u32; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b"test") },
                                {
                                    (::std::mem::size_of::<[i8; 5]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<u32>::null(),
                        )
                    })
                },
                {
                    (({
                        MultiStringTypeImpl::length_11(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<i32>::from_string_literal(&[
                                        116 as i32, 101 as i32, 115 as i32, 116 as i32, 0 as i32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[i32; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u8>::from_string_literal(&[
                                        116 as u8, 101 as u8, 115 as u8, 116 as u8, 0 as u8,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u8; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u16>::from_string_literal(&[
                                        116 as u16, 101 as u16, 115 as u16, 116 as u16, 0 as u16,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u16; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u32>::from_string_literal(&[
                                        116 as u32, 101 as u32, 115 as u32, 116 as u32, 0 as u32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u32; 5]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b"test") },
                                {
                                    (::std::mem::size_of::<[i8; 5]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<u32>::null(),
                        )
                    }) as u64)
                },
            ),
        ));
    ({ test_construction_66((*val.borrow()).clone()) });
    ({ test_construction_67((*val.borrow()).clone()) });
    ({ test_construction_68((*val.borrow()).clone()) });
    ({ test_construction_69((*val.borrow()).clone()) });
    ({ test_construction_70((*val.borrow()).clone()) });
    ({ test_construction_71((*val.borrow()).clone()) });
}
pub fn test_77() -> bool {
    ({ test_with_char_72() });
    ({ test_with_char_73() });
    ({ test_with_char_74() });
    ({ test_with_char_75() });
    ({ test_with_char_76() });
    return true;
}
const _: () = assert!(
    true,
    "std::is_constructible_v<std::string_view, const char*, char*>"
);
const _: () = assert!(
    true,
    "std::is_constructible_v<std::string_view, char*, const char*>"
);
const _: () = assert!(
    true,
    "!std::is_constructible_v<std::string_view, char*, void*>"
);
const _: () = assert!(
    true,
    "!std::is_constructible_v<std::string_view, signed char*, signed char*>"
);
const _: () = assert!(
    true,
    "!std::is_constructible_v<std::string_view,\n                                       random_access_iterator<char*>,\n                                       random_access_iterator<char*>>"
);
const _: () = assert!(
    true,
    "std::is_constructible_v<std::string_view, contiguous_iterator<char*>, contiguous_iterator<char*>>"
);
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
    ({ test_77() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait MultiStringTypeImpl {
    fn as_ptr_2(&self, _a0: Ptr<i8>) -> Ptr<i8>;
    fn as_ptr_3(&self, _a0: Ptr<i32>) -> Ptr<i32>;
    fn as_ptr_4(&self, _a0: Ptr<u8>) -> Ptr<u8>;
    fn as_ptr_5(&self, _a0: Ptr<u16>) -> Ptr<u16>;
    fn as_ptr_6(&self, _a0: Ptr<u32>) -> Ptr<u32>;
    fn length_7(&self, _a0: Ptr<i8>) -> usize;
    fn length_8(&self, _a0: Ptr<i32>) -> usize;
    fn length_9(&self, _a0: Ptr<u8>) -> usize;
    fn length_10(&self, _a0: Ptr<u16>) -> usize;
    fn length_11(&self, _a0: Ptr<u32>) -> usize;
    fn to_pconsti8(&self) -> Ptr<i8>;
    fn to_pconsti32(&self) -> Ptr<i32>;
    fn to_pconstu8(&self) -> Ptr<u8>;
    fn to_pconstu16(&self) -> Ptr<u16>;
    fn to_pconstu32(&self) -> Ptr<u32>;
}
impl MultiStringTypeImpl for Ptr<MultiStringType> {
    fn as_ptr_2(&self, mut _a0: Ptr<i8>) -> Ptr<i8> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn as_ptr_3(&self, mut _a0: Ptr<i32>) -> Ptr<i32> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn as_ptr_4(&self, mut _a0: Ptr<u8>) -> Ptr<u8> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn as_ptr_5(&self, mut _a0: Ptr<u16>) -> Ptr<u16> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn as_ptr_6(&self, mut _a0: Ptr<u32>) -> Ptr<u32> {
        return (*self).with(|__s| __s.u32_.clone());
    }
    fn length_7(&self, mut _a0: Ptr<i8>) -> usize {
        return (*self).with(|__s| __s.sn_);
    }
    fn length_8(&self, mut _a0: Ptr<i32>) -> usize {
        return (*self).with(|__s| __s.wn_);
    }
    fn length_9(&self, mut _a0: Ptr<u8>) -> usize {
        return (*self).with(|__s| __s.u8n_);
    }
    fn length_10(&self, mut _a0: Ptr<u16>) -> usize {
        return (*self).with(|__s| __s.u16n_);
    }
    fn length_11(&self, mut _a0: Ptr<u32>) -> usize {
        return (*self).with(|__s| __s.u32n_);
    }
    fn to_pconsti8(&self) -> Ptr<i8> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn to_pconsti32(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn to_pconstu8(&self) -> Ptr<u8> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn to_pconstu16(&self) -> Ptr<u16> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn to_pconstu32(&self) -> Ptr<u32> {
        return (*self).with(|__s| __s.u32_.clone());
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
pub trait contiguous_iterator_char_ptr_Impl {
    fn base(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<i8>;
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
    fn operator_arrow(&self) -> Ptr<i8> {
        return (*self).with(|__s| __s.it_.clone());
    }
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
pub trait contiguous_iterator_const_char16_t_ptr_Impl {
    fn base(&self) -> Ptr<u16> {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<u16> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<u16>;
    fn operator_index(&self, mut n: isize) -> Ptr<u16> {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_const_char16_t_ptr_> {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_const_char16_t_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: i32) -> contiguous_iterator_const_char16_t_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: i32) -> contiguous_iterator_const_char16_t_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<contiguous_iterator_const_char16_t_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<contiguous_iterator_const_char16_t_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_const_char16_t_ptr_>,
    ) -> Ptr<contiguous_iterator_const_char16_t_ptr_>;
}
impl contiguous_iterator_const_char16_t_ptr_Impl for Ptr<contiguous_iterator_const_char16_t_ptr_> {
    fn operator_arrow(&self) -> Ptr<u16> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_const_char16_t_ptr_>,
    ) -> Ptr<contiguous_iterator_const_char16_t_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait contiguous_iterator_const_char32_t_ptr_Impl {
    fn base(&self) -> Ptr<u32> {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<u32> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<u32>;
    fn operator_index(&self, mut n: isize) -> Ptr<u32> {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_const_char32_t_ptr_> {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_const_char32_t_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: i32) -> contiguous_iterator_const_char32_t_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: i32) -> contiguous_iterator_const_char32_t_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<contiguous_iterator_const_char32_t_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<contiguous_iterator_const_char32_t_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_const_char32_t_ptr_>,
    ) -> Ptr<contiguous_iterator_const_char32_t_ptr_>;
}
impl contiguous_iterator_const_char32_t_ptr_Impl for Ptr<contiguous_iterator_const_char32_t_ptr_> {
    fn operator_arrow(&self) -> Ptr<u32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_const_char32_t_ptr_>,
    ) -> Ptr<contiguous_iterator_const_char32_t_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait contiguous_iterator_const_char8_t_ptr_Impl {
    fn base(&self) -> Ptr<u8> {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<u8> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<u8>;
    fn operator_index(&self, mut n: isize) -> Ptr<u8> {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_const_char8_t_ptr_> {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_const_char8_t_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: i32) -> contiguous_iterator_const_char8_t_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: i32) -> contiguous_iterator_const_char8_t_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<contiguous_iterator_const_char8_t_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<contiguous_iterator_const_char8_t_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_const_char8_t_ptr_>,
    ) -> Ptr<contiguous_iterator_const_char8_t_ptr_>;
}
impl contiguous_iterator_const_char8_t_ptr_Impl for Ptr<contiguous_iterator_const_char8_t_ptr_> {
    fn operator_arrow(&self) -> Ptr<u8> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_const_char8_t_ptr_>,
    ) -> Ptr<contiguous_iterator_const_char8_t_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait contiguous_iterator_const_char_ptr_Impl {
    fn base(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<i8>;
    fn operator_index(&self, mut n: isize) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_const_char_ptr_> {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_const_char_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: i32) -> contiguous_iterator_const_char_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: i32) -> contiguous_iterator_const_char_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<contiguous_iterator_const_char_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<contiguous_iterator_const_char_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_const_char_ptr_>,
    ) -> Ptr<contiguous_iterator_const_char_ptr_>;
}
impl contiguous_iterator_const_char_ptr_Impl for Ptr<contiguous_iterator_const_char_ptr_> {
    fn operator_arrow(&self) -> Ptr<i8> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_const_char_ptr_>,
    ) -> Ptr<contiguous_iterator_const_char_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait contiguous_iterator_const_wchar_t_ptr_Impl {
    fn base(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<i32>;
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_const_wchar_t_ptr_> {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_const_wchar_t_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: i32) -> contiguous_iterator_const_wchar_t_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: i32) -> contiguous_iterator_const_wchar_t_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<contiguous_iterator_const_wchar_t_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<contiguous_iterator_const_wchar_t_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_const_wchar_t_ptr_>,
    ) -> Ptr<contiguous_iterator_const_wchar_t_ptr_>;
}
impl contiguous_iterator_const_wchar_t_ptr_Impl for Ptr<contiguous_iterator_const_wchar_t_ptr_> {
    fn operator_arrow(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_const_wchar_t_ptr_>,
    ) -> Ptr<contiguous_iterator_const_wchar_t_ptr_> {
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
pub trait sized_sentinel_const_char16_t_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<u16>>) -> bool {
        unimplemented!()
    }
}
impl sized_sentinel_const_char16_t_ptr_Impl for Ptr<sized_sentinel_const_char16_t_ptr_> {}
pub trait sized_sentinel_const_char32_t_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<u32>>) -> bool {
        unimplemented!()
    }
}
impl sized_sentinel_const_char32_t_ptr_Impl for Ptr<sized_sentinel_const_char32_t_ptr_> {}
pub trait sized_sentinel_const_char8_t_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<u8>>) -> bool {
        unimplemented!()
    }
}
impl sized_sentinel_const_char8_t_ptr_Impl for Ptr<sized_sentinel_const_char8_t_ptr_> {}
pub trait sized_sentinel_const_char_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<i8>>) -> bool {
        unimplemented!()
    }
}
impl sized_sentinel_const_char_ptr_Impl for Ptr<sized_sentinel_const_char_ptr_> {}
pub trait sized_sentinel_const_wchar_t_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<i32>>) -> bool {
        unimplemented!()
    }
}
impl sized_sentinel_const_wchar_t_ptr_Impl for Ptr<sized_sentinel_const_wchar_t_ptr_> {}
pub trait sized_sentinel_contiguous_iterator_const_char16_t_ptr__Impl {
    fn operator_eq(&self, other: Ptr<contiguous_iterator_const_char16_t_ptr_>) -> bool {
        unimplemented!()
    }
}
impl sized_sentinel_contiguous_iterator_const_char16_t_ptr__Impl
    for Ptr<sized_sentinel_contiguous_iterator_const_char16_t_ptr__>
{
}
pub trait sized_sentinel_contiguous_iterator_const_char32_t_ptr__Impl {
    fn operator_eq(&self, other: Ptr<contiguous_iterator_const_char32_t_ptr_>) -> bool {
        unimplemented!()
    }
}
impl sized_sentinel_contiguous_iterator_const_char32_t_ptr__Impl
    for Ptr<sized_sentinel_contiguous_iterator_const_char32_t_ptr__>
{
}
pub trait sized_sentinel_contiguous_iterator_const_char8_t_ptr__Impl {
    fn operator_eq(&self, other: Ptr<contiguous_iterator_const_char8_t_ptr_>) -> bool {
        unimplemented!()
    }
}
impl sized_sentinel_contiguous_iterator_const_char8_t_ptr__Impl
    for Ptr<sized_sentinel_contiguous_iterator_const_char8_t_ptr__>
{
}
pub trait sized_sentinel_contiguous_iterator_const_char_ptr__Impl {
    fn operator_eq(&self, other: Ptr<contiguous_iterator_const_char_ptr_>) -> bool {
        unimplemented!()
    }
}
impl sized_sentinel_contiguous_iterator_const_char_ptr__Impl
    for Ptr<sized_sentinel_contiguous_iterator_const_char_ptr__>
{
}
pub trait sized_sentinel_contiguous_iterator_const_wchar_t_ptr__Impl {
    fn operator_eq(&self, other: Ptr<contiguous_iterator_const_wchar_t_ptr_>) -> bool {
        unimplemented!()
    }
}
impl sized_sentinel_contiguous_iterator_const_wchar_t_ptr__Impl
    for Ptr<sized_sentinel_contiguous_iterator_const_wchar_t_ptr__>
{
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
