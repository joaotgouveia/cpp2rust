extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_1(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_2(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_3(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_4(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_5(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_6(_a0: Ptr<MoveOnly>, _a1: Ptr<MoveOnly>) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
            return (*cmp.borrow());
        }
    }
    return std::cmp::Ordering::Equal;
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
        operator_lt_2(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for MoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new(None) }
    }
}
pub fn operator_eq_7(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_8(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_9(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_10(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_11(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_12(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_13(_a0: Ptr<TrivialMoveOnly>, _a1: Ptr<TrivialMoveOnly>) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
            return (*cmp.borrow());
        }
    }
    return std::cmp::Ordering::Equal;
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
        operator_lt_9(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for TrivialMoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_7(__lhs.clone(), __rhs.clone())
    }
}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        { TrivialMoveOnly::new(None) }
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
pub fn operator_ne_14(
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
pub fn operator_ne_15(
    x: Ptr<cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_>,
    y: Ptr<cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<u32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ {
    pub fn new(mut it: Ptr<u32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(
        _a0: Ptr<cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_>,
    ) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppAdvance for cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_5();
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub fn operator_ne_16(
    x: Ptr<forward_iterator_const_unsigned_int_ptr_>,
    y: Ptr<forward_iterator_const_unsigned_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct forward_iterator_const_unsigned_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<u32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_unsigned_int_ptr_ {
    pub fn new_1(mut it: Ptr<u32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<forward_iterator_const_unsigned_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppAdvance for forward_iterator_const_unsigned_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
}
pub fn operator_ne_17(
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
pub fn operator_ne_18(
    x: Ptr<bidirectional_iterator_const_unsigned_int_ptr_>,
    y: Ptr<bidirectional_iterator_const_unsigned_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct bidirectional_iterator_const_unsigned_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<u32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_const_unsigned_int_ptr_ {
    pub fn new_1(mut it: Ptr<u32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<bidirectional_iterator_const_unsigned_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppAdvance for bidirectional_iterator_const_unsigned_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
}
pub fn operator_ne_19(
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
pub fn operator_sub_20(
    mut x: random_access_iterator_const_unsigned_int_ptr_,
    mut y: random_access_iterator_const_unsigned_int_ptr_,
) -> isize {
    return (((x.it_).clone() - (y.it_).clone()) as i64 as isize);
}
pub fn operator_ne_21(
    x: Ptr<random_access_iterator_const_unsigned_int_ptr_>,
    y: Ptr<random_access_iterator_const_unsigned_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct random_access_iterator_const_unsigned_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<u32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_const_unsigned_int_ptr_ {
    pub fn new_1(mut it: Ptr<u32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<random_access_iterator_const_unsigned_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppAdvance for random_access_iterator_const_unsigned_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_7();
    }
}
pub fn operator_sub_22(
    mut x: random_access_iterator_const_int_ptr_,
    mut y: random_access_iterator_const_int_ptr_,
) -> isize {
    return (((x.it_).clone() - (y.it_).clone()) as i64 as isize);
}
pub fn operator_ne_23(
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
pub fn operator_eq_24(
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
        operator_eq_24(__lhs.clone(), __rhs.clone())
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
pub fn operator_add_25(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_26(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_25(_i, n)
    });
}
pub fn iter_swap_27(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap!((a.ptr_).clone(), (b.ptr_).clone());
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_28(iter: adl_Iterator) -> Ptr<i32> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_29(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> bool {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_30(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> std::cmp::Ordering {
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
        operator_cmp_30(__lhs.clone(), __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for adl_Iterator {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_29(__lhs.clone(), __rhs.clone())
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
pub fn test_31(
    mut first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut first2: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_32(
    mut first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_33(
    mut first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut first2: forward_iterator_const_unsigned_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_34(
    mut first1: forward_iterator_const_unsigned_int_ptr_,
    mut last1: forward_iterator_const_unsigned_int_ptr_,
    mut first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_35(
    mut first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut first2: bidirectional_iterator_const_unsigned_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_36(
    mut first1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut last1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_37(
    mut first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut first2: random_access_iterator_const_unsigned_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_38(
    mut first1: random_access_iterator_const_unsigned_int_ptr_,
    mut last1: random_access_iterator_const_unsigned_int_ptr_,
    mut first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_39(
    mut first1: forward_iterator_const_int_ptr_,
    mut last1: forward_iterator_const_int_ptr_,
    mut first2: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_40(
    mut first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut first2: forward_iterator_const_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_41(
    mut first1: forward_iterator_const_int_ptr_,
    mut last1: forward_iterator_const_int_ptr_,
    mut first2: forward_iterator_const_unsigned_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_42(
    mut first1: forward_iterator_const_unsigned_int_ptr_,
    mut last1: forward_iterator_const_unsigned_int_ptr_,
    mut first2: forward_iterator_const_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_43(
    mut first1: forward_iterator_const_int_ptr_,
    mut last1: forward_iterator_const_int_ptr_,
    mut first2: bidirectional_iterator_const_unsigned_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_44(
    mut first1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut last1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut first2: forward_iterator_const_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_45(
    mut first1: forward_iterator_const_int_ptr_,
    mut last1: forward_iterator_const_int_ptr_,
    mut first2: random_access_iterator_const_unsigned_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_46(
    mut first1: random_access_iterator_const_unsigned_int_ptr_,
    mut last1: random_access_iterator_const_unsigned_int_ptr_,
    mut first2: forward_iterator_const_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_47(
    mut first1: bidirectional_iterator_const_int_ptr_,
    mut last1: bidirectional_iterator_const_int_ptr_,
    mut first2: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_48(
    mut first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut first2: bidirectional_iterator_const_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_49(
    mut first1: bidirectional_iterator_const_int_ptr_,
    mut last1: bidirectional_iterator_const_int_ptr_,
    mut first2: forward_iterator_const_unsigned_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_50(
    mut first1: forward_iterator_const_unsigned_int_ptr_,
    mut last1: forward_iterator_const_unsigned_int_ptr_,
    mut first2: bidirectional_iterator_const_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_51(
    mut first1: bidirectional_iterator_const_int_ptr_,
    mut last1: bidirectional_iterator_const_int_ptr_,
    mut first2: bidirectional_iterator_const_unsigned_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_52(
    mut first1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut last1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut first2: bidirectional_iterator_const_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_53(
    mut first1: bidirectional_iterator_const_int_ptr_,
    mut last1: bidirectional_iterator_const_int_ptr_,
    mut first2: random_access_iterator_const_unsigned_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_54(
    mut first1: random_access_iterator_const_unsigned_int_ptr_,
    mut last1: random_access_iterator_const_unsigned_int_ptr_,
    mut first2: bidirectional_iterator_const_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_55(
    mut first1: random_access_iterator_const_int_ptr_,
    mut last1: random_access_iterator_const_int_ptr_,
    mut first2: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_56(
    mut first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_,
    mut first2: random_access_iterator_const_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_57(
    mut first1: random_access_iterator_const_int_ptr_,
    mut last1: random_access_iterator_const_int_ptr_,
    mut first2: forward_iterator_const_unsigned_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_58(
    mut first1: forward_iterator_const_unsigned_int_ptr_,
    mut last1: forward_iterator_const_unsigned_int_ptr_,
    mut first2: random_access_iterator_const_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_59(
    mut first1: random_access_iterator_const_int_ptr_,
    mut last1: random_access_iterator_const_int_ptr_,
    mut first2: bidirectional_iterator_const_unsigned_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_60(
    mut first1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut last1: bidirectional_iterator_const_unsigned_int_ptr_,
    mut first2: random_access_iterator_const_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_61(
    mut first1: random_access_iterator_const_int_ptr_,
    mut last1: random_access_iterator_const_int_ptr_,
    mut first2: random_access_iterator_const_unsigned_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_62(
    mut first1: random_access_iterator_const_unsigned_int_ptr_,
    mut last1: random_access_iterator_const_unsigned_int_ptr_,
    mut first2: random_access_iterator_const_int_ptr_,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!((unimplemented!() == x));
}
pub fn test_63(
    mut first1: Ptr<i32>,
    mut last1: Ptr<i32>,
    mut first2: Ptr<u32>,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!(({ unimplemented!() } == { x }));
}
pub fn test_64(
    mut first1: Ptr<u32>,
    mut last1: Ptr<u32>,
    mut first2: Ptr<i32>,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!(({ unimplemented!() } == { x }));
}
pub fn test_65(
    mut first1: Ptr<i32>,
    mut last1: Ptr<i32>,
    mut first2: Ptr<u32>,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!(({ unimplemented!() } == { x }));
}
pub fn test_66(
    mut first1: Ptr<u32>,
    mut last1: Ptr<u32>,
    mut first2: Ptr<i32>,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!(({ unimplemented!() } == { x }));
}
pub fn test_67(
    mut first1: Ptr<i32>,
    mut last1: Ptr<i32>,
    mut first2: Ptr<u32>,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!(({ unimplemented!() } == { x }));
}
pub fn test_68(
    mut first1: Ptr<u32>,
    mut last1: Ptr<u32>,
    mut first2: Ptr<i32>,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!(({ unimplemented!() } == { x }));
}
pub fn test_69(
    mut first1: Ptr<i32>,
    mut last1: Ptr<i32>,
    mut first2: Ptr<u32>,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!(({ unimplemented!() } == { x }));
}
pub fn test_70(
    mut first1: Ptr<u32>,
    mut last1: Ptr<u32>,
    mut first2: Ptr<i32>,
    mut init: i32,
    mut x: i32,
) {
    const _: () = assert!(
        true,
        "std::is_same_v<T,\n            decltype(std::transform_reduce(first1, last1, first2, init))>"
    );;
    assert!(({ unimplemented!() } == { x }));
}
pub fn test_71() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
    let ua: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        2_u32, 4_u32, 6_u32, 8_u32, 10_u32, 12_u32,
    ])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        ((sa as usize)
            == (::std::mem::size_of::<[u32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<u32>() as usize)))
    );
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            });
        test_31(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            0,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            });
        test_32(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            }),
            1,
            1,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
            });
        test_31(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            2,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>).offset((1) as isize)
            });
        test_32(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            }),
            2,
            4,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
            });
        test_31(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            10,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>).offset((2) as isize)
            });
        test_32(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            }),
            3,
            13,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        test_31(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            182,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>).offset((sa) as isize)
            });
        test_32(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            }),
            4,
            186,
        )
    });
}
pub fn test_72() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
    let ua: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        2_u32, 4_u32, 6_u32, 8_u32, 10_u32, 12_u32,
    ])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        ((sa as usize)
            == (::std::mem::size_of::<[u32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<u32>() as usize)))
    );
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            });
        test_33(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) }),
            0,
            0,
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) });
        test_34(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            }),
            1,
            1,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
            });
        test_33(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) }),
            0,
            2,
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((1) as isize)
            });
        test_34(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            }),
            2,
            4,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
            });
        test_33(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) }),
            0,
            10,
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((2) as isize)
            });
        test_34(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            }),
            3,
            13,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        test_33(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) }),
            0,
            182,
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((sa) as isize)
            });
        test_34(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            }),
            4,
            186,
        )
    });
}
pub fn test_73() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
    let ua: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        2_u32, 4_u32, 6_u32, 8_u32, 10_u32, 12_u32,
    ])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        ((sa as usize)
            == (::std::mem::size_of::<[u32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<u32>() as usize)))
    );
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            });
        test_35(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            0,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        test_36(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            }),
            1,
            1,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
            });
        test_35(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            2,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((1) as isize)
            });
        test_36(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            }),
            2,
            4,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
            });
        test_35(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            10,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((2) as isize)
            });
        test_36(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            }),
            3,
            13,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        test_35(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            182,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((sa) as isize)
            });
        test_36(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            }),
            4,
            186,
        )
    });
}
pub fn test_74() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
    let ua: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        2_u32, 4_u32, 6_u32, 8_u32, 10_u32, 12_u32,
    ])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        ((sa as usize)
            == (::std::mem::size_of::<[u32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<u32>() as usize)))
    );
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            });
        test_37(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            0,
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        test_38(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            }),
            1,
            1,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
            });
        test_37(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            2,
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((1) as isize)
            });
        test_38(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            }),
            2,
            4,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
            });
        test_37(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            10,
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((2) as isize)
            });
        test_38(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            }),
            3,
            13,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            });
        let _last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        test_37(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            182,
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((sa) as isize)
            });
        test_38(
            _first1,
            _last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_pointer() as Ptr<i32>)
            }),
            4,
            186,
        )
    });
}
pub fn test_75() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
    let ua: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        2_u32, 4_u32, 6_u32, 8_u32, 10_u32, 12_u32,
    ])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        ((sa as usize)
            == (::std::mem::size_of::<[u32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<u32>() as usize)))
    );
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        test_39(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            0,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            });
        test_40(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            1,
            1,
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
        });
        test_39(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            2,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>).offset((1) as isize)
            });
        test_40(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            2,
            4,
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
        });
        test_39(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            10,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>).offset((2) as isize)
            });
        test_40(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            3,
            13,
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<i32>).offset((sa) as isize)
        });
        test_39(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            182,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>).offset((sa) as isize)
            });
        test_40(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            4,
            186,
        )
    });
}
pub fn test_76() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
    let ua: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        2_u32, 4_u32, 6_u32, 8_u32, 10_u32, 12_u32,
    ])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        ((sa as usize)
            == (::std::mem::size_of::<[u32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<u32>() as usize)))
    );
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        test_41(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) }),
            0,
            0,
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) });
        test_42(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            1,
            1,
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
        });
        test_41(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) }),
            0,
            2,
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((1) as isize)
            });
        test_42(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            2,
            4,
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
        });
        test_41(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) }),
            0,
            10,
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((2) as isize)
            });
        test_42(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            3,
            13,
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<i32>).offset((sa) as isize)
        });
        test_41(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) }),
            0,
            182,
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((sa) as isize)
            });
        test_42(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            4,
            186,
        )
    });
}
pub fn test_77() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
    let ua: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        2_u32, 4_u32, 6_u32, 8_u32, 10_u32, 12_u32,
    ])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        ((sa as usize)
            == (::std::mem::size_of::<[u32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<u32>() as usize)))
    );
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        test_43(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            0,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        test_44(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            1,
            1,
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
        });
        test_43(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            2,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((1) as isize)
            });
        test_44(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            2,
            4,
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
        });
        test_43(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            10,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((2) as isize)
            });
        test_44(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            3,
            13,
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<i32>).offset((sa) as isize)
        });
        test_43(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            182,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((sa) as isize)
            });
        test_44(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            4,
            186,
        )
    });
}
pub fn test_78() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
    let ua: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        2_u32, 4_u32, 6_u32, 8_u32, 10_u32, 12_u32,
    ])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        ((sa as usize)
            == (::std::mem::size_of::<[u32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<u32>() as usize)))
    );
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        test_45(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            0,
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        test_46(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            1,
            1,
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
        });
        test_45(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            2,
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((1) as isize)
            });
        test_46(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            2,
            4,
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
        });
        test_45(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            10,
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((2) as isize)
            });
        test_46(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            3,
            13,
        )
    });
    ({
        let _first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ia.as_pointer() as Ptr<i32>).offset((sa) as isize)
        });
        test_45(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            182,
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((sa) as isize)
            });
        test_46(
            _first1,
            _last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            4,
            186,
        )
    });
}
pub fn test_79() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
    let ua: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        2_u32, 4_u32, 6_u32, 8_u32, 10_u32, 12_u32,
    ])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        ((sa as usize)
            == (::std::mem::size_of::<[u32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<u32>() as usize)))
    );
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        test_47(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            0,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            });
        test_48(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            1,
            1,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
            });
        test_47(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            2,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>).offset((1) as isize)
            });
        test_48(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            2,
            4,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
            });
        test_47(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            10,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>).offset((2) as isize)
            });
        test_48(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            3,
            13,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        test_47(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            182,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>).offset((sa) as isize)
            });
        test_48(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            4,
            186,
        )
    });
}
pub fn test_80() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
    let ua: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        2_u32, 4_u32, 6_u32, 8_u32, 10_u32, 12_u32,
    ])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        ((sa as usize)
            == (::std::mem::size_of::<[u32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<u32>() as usize)))
    );
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        test_49(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) }),
            0,
            0,
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) });
        test_50(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            1,
            1,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
            });
        test_49(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) }),
            0,
            2,
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((1) as isize)
            });
        test_50(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            2,
            4,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
            });
        test_49(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) }),
            0,
            10,
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((2) as isize)
            });
        test_50(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            3,
            13,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        test_49(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) }),
            0,
            182,
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((sa) as isize)
            });
        test_50(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            4,
            186,
        )
    });
}
pub fn test_81() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
    let ua: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        2_u32, 4_u32, 6_u32, 8_u32, 10_u32, 12_u32,
    ])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        ((sa as usize)
            == (::std::mem::size_of::<[u32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<u32>() as usize)))
    );
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        test_51(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            0,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        test_52(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            1,
            1,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
            });
        test_51(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            2,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((1) as isize)
            });
        test_52(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            2,
            4,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
            });
        test_51(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            10,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((2) as isize)
            });
        test_52(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            3,
            13,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        test_51(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            182,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((sa) as isize)
            });
        test_52(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            4,
            186,
        )
    });
}
pub fn test_82() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
    let ua: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        2_u32, 4_u32, 6_u32, 8_u32, 10_u32, 12_u32,
    ])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        ((sa as usize)
            == (::std::mem::size_of::<[u32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<u32>() as usize)))
    );
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        test_53(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            0,
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        test_54(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            1,
            1,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
            });
        test_53(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            2,
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((1) as isize)
            });
        test_54(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            2,
            4,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
            });
        test_53(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            10,
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((2) as isize)
            });
        test_54(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            3,
            13,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        test_53(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            182,
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((sa) as isize)
            });
        test_54(
            _first1,
            _last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            4,
            186,
        )
    });
}
pub fn test_83() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
    let ua: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        2_u32, 4_u32, 6_u32, 8_u32, 10_u32, 12_u32,
    ])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        ((sa as usize)
            == (::std::mem::size_of::<[u32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<u32>() as usize)))
    );
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        test_55(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            0,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            });
        test_56(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            1,
            1,
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
            });
        test_55(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            2,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>).offset((1) as isize)
            });
        test_56(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            2,
            4,
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
            });
        test_55(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            10,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>).offset((2) as isize)
            });
        test_56(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            3,
            13,
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        test_55(
            _first1,
            _last1,
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            182,
        )
    });
    ({
        let _first1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ =
            cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_::new({
                (ua.as_pointer() as Ptr<u32>).offset((sa) as isize)
            });
        test_56(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            4,
            186,
        )
    });
}
pub fn test_84() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
    let ua: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        2_u32, 4_u32, 6_u32, 8_u32, 10_u32, 12_u32,
    ])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        ((sa as usize)
            == (::std::mem::size_of::<[u32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<u32>() as usize)))
    );
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        test_57(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) }),
            0,
            0,
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) });
        test_58(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            1,
            1,
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
            });
        test_57(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) }),
            0,
            2,
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((1) as isize)
            });
        test_58(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            2,
            4,
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
            });
        test_57(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) }),
            0,
            10,
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((2) as isize)
            });
        test_58(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            3,
            13,
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        test_57(
            _first1,
            _last1,
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) }),
            0,
            182,
        )
    });
    ({
        let _first1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({ (ua.as_pointer() as Ptr<u32>) });
        let _last1: forward_iterator_const_unsigned_int_ptr_ =
            forward_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((sa) as isize)
            });
        test_58(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            4,
            186,
        )
    });
}
pub fn test_85() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
    let ua: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        2_u32, 4_u32, 6_u32, 8_u32, 10_u32, 12_u32,
    ])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        ((sa as usize)
            == (::std::mem::size_of::<[u32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<u32>() as usize)))
    );
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        test_59(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            0,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        test_60(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            1,
            1,
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
            });
        test_59(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            2,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((1) as isize)
            });
        test_60(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            2,
            4,
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
            });
        test_59(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            10,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((2) as isize)
            });
        test_60(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            3,
            13,
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        test_59(
            _first1,
            _last1,
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            182,
        )
    });
    ({
        let _first1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: bidirectional_iterator_const_unsigned_int_ptr_ =
            bidirectional_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((sa) as isize)
            });
        test_60(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            4,
            186,
        )
    });
}
pub fn test_86() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
    let ua: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        2_u32, 4_u32, 6_u32, 8_u32, 10_u32, 12_u32,
    ])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        ((sa as usize)
            == (::std::mem::size_of::<[u32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<u32>() as usize)))
    );
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        test_61(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            0,
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        test_62(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            1,
            1,
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
            });
        test_61(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            2,
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((1) as isize)
            });
        test_62(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            2,
            4,
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
            });
        test_61(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            10,
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((2) as isize)
            });
        test_62(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            3,
            13,
        )
    });
    ({
        let _first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
        let _last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<i32>).offset((sa) as isize)
            });
        test_61(
            _first1,
            _last1,
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            }),
            0,
            182,
        )
    });
    ({
        let _first1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>)
            });
        let _last1: random_access_iterator_const_unsigned_int_ptr_ =
            random_access_iterator_const_unsigned_int_ptr_::new_1({
                (ua.as_pointer() as Ptr<u32>).offset((sa) as isize)
            });
        test_62(
            _first1,
            _last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) }),
            4,
            186,
        )
    });
}
pub fn test_87() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
    let ua: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        2_u32, 4_u32, 6_u32, 8_u32, 10_u32, 12_u32,
    ])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        ((sa as usize)
            == (::std::mem::size_of::<[u32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<u32>() as usize)))
    );
    ({
        let _first1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
        let _last1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
        test_63(_first1, _last1, (ua.as_pointer() as Ptr<u32>), 0, 0)
    });
    ({
        let _first1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>);
        let _last1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>);
        test_64(_first1, _last1, (ia.as_pointer() as Ptr<i32>), 1, 1)
    });
    ({
        let _first1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
        let _last1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
        test_63(_first1, _last1, (ua.as_pointer() as Ptr<u32>), 0, 2)
    });
    ({
        let _first1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>);
        let _last1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>).offset((1) as isize);
        test_64(_first1, _last1, (ia.as_pointer() as Ptr<i32>), 2, 4)
    });
    ({
        let _first1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
        let _last1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
        test_63(_first1, _last1, (ua.as_pointer() as Ptr<u32>), 0, 10)
    });
    ({
        let _first1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>);
        let _last1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>).offset((2) as isize);
        test_64(_first1, _last1, (ia.as_pointer() as Ptr<i32>), 3, 13)
    });
    ({
        let _first1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
        let _last1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((sa) as isize);
        test_63(_first1, _last1, (ua.as_pointer() as Ptr<u32>), 0, 182)
    });
    ({
        let _first1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>);
        let _last1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>).offset((sa) as isize);
        test_64(_first1, _last1, (ia.as_pointer() as Ptr<i32>), 4, 186)
    });
}
pub fn test_88() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
    let ua: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        2_u32, 4_u32, 6_u32, 8_u32, 10_u32, 12_u32,
    ])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        ((sa as usize)
            == (::std::mem::size_of::<[u32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<u32>() as usize)))
    );
    ({
        let _first1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
        let _last1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
        test_65(_first1, _last1, (ua.as_pointer() as Ptr<u32>), 0, 0)
    });
    ({
        let _first1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>);
        let _last1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>);
        test_66(_first1, _last1, (ia.as_pointer() as Ptr<i32>), 1, 1)
    });
    ({
        let _first1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
        let _last1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
        test_65(_first1, _last1, (ua.as_pointer() as Ptr<u32>), 0, 2)
    });
    ({
        let _first1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>);
        let _last1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>).offset((1) as isize);
        test_66(_first1, _last1, (ia.as_pointer() as Ptr<i32>), 2, 4)
    });
    ({
        let _first1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
        let _last1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
        test_65(_first1, _last1, (ua.as_pointer() as Ptr<u32>), 0, 10)
    });
    ({
        let _first1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>);
        let _last1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>).offset((2) as isize);
        test_66(_first1, _last1, (ia.as_pointer() as Ptr<i32>), 3, 13)
    });
    ({
        let _first1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
        let _last1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((sa) as isize);
        test_65(_first1, _last1, (ua.as_pointer() as Ptr<u32>), 0, 182)
    });
    ({
        let _first1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>);
        let _last1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>).offset((sa) as isize);
        test_66(_first1, _last1, (ia.as_pointer() as Ptr<i32>), 4, 186)
    });
}
pub fn test_89() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
    let ua: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        2_u32, 4_u32, 6_u32, 8_u32, 10_u32, 12_u32,
    ])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        ((sa as usize)
            == (::std::mem::size_of::<[u32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<u32>() as usize)))
    );
    ({
        let _first1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
        let _last1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
        test_67(_first1, _last1, (ua.as_pointer() as Ptr<u32>), 0, 0)
    });
    ({
        let _first1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>);
        let _last1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>);
        test_68(_first1, _last1, (ia.as_pointer() as Ptr<i32>), 1, 1)
    });
    ({
        let _first1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
        let _last1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
        test_67(_first1, _last1, (ua.as_pointer() as Ptr<u32>), 0, 2)
    });
    ({
        let _first1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>);
        let _last1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>).offset((1) as isize);
        test_68(_first1, _last1, (ia.as_pointer() as Ptr<i32>), 2, 4)
    });
    ({
        let _first1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
        let _last1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
        test_67(_first1, _last1, (ua.as_pointer() as Ptr<u32>), 0, 10)
    });
    ({
        let _first1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>);
        let _last1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>).offset((2) as isize);
        test_68(_first1, _last1, (ia.as_pointer() as Ptr<i32>), 3, 13)
    });
    ({
        let _first1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
        let _last1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((sa) as isize);
        test_67(_first1, _last1, (ua.as_pointer() as Ptr<u32>), 0, 182)
    });
    ({
        let _first1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>);
        let _last1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>).offset((sa) as isize);
        test_68(_first1, _last1, (ia.as_pointer() as Ptr<i32>), 4, 186)
    });
}
pub fn test_90() {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
    let ua: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        2_u32, 4_u32, 6_u32, 8_u32, 10_u32, 12_u32,
    ])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        ((sa as usize)
            == (::std::mem::size_of::<[u32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<u32>() as usize)))
    );
    ({
        let _first1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
        let _last1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
        test_69(_first1, _last1, (ua.as_pointer() as Ptr<u32>), 0, 0)
    });
    ({
        let _first1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>);
        let _last1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>);
        test_70(_first1, _last1, (ia.as_pointer() as Ptr<i32>), 1, 1)
    });
    ({
        let _first1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
        let _last1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
        test_69(_first1, _last1, (ua.as_pointer() as Ptr<u32>), 0, 2)
    });
    ({
        let _first1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>);
        let _last1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>).offset((1) as isize);
        test_70(_first1, _last1, (ia.as_pointer() as Ptr<i32>), 2, 4)
    });
    ({
        let _first1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
        let _last1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
        test_69(_first1, _last1, (ua.as_pointer() as Ptr<u32>), 0, 10)
    });
    ({
        let _first1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>);
        let _last1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>).offset((2) as isize);
        test_70(_first1, _last1, (ia.as_pointer() as Ptr<i32>), 3, 13)
    });
    ({
        let _first1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
        let _last1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((sa) as isize);
        test_69(_first1, _last1, (ua.as_pointer() as Ptr<u32>), 0, 182)
    });
    ({
        let _first1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>);
        let _last1: Ptr<u32> = (ua.as_pointer() as Ptr<u32>).offset((sa) as isize);
        test_70(_first1, _last1, (ia.as_pointer() as Ptr<i32>), 4, 186)
    });
}
pub fn test_return_type_91() {
    let mut p: Ptr<i8> = Ptr::<i8>::null();
    const _: () = assert!(
        true,
        "std::is_same_v<Init,\n                       decltype(std::transform_reduce(p, p, p, Init{{}}))>"
    );;
}
pub fn test_return_type_92() {
    let mut p: Ptr<i32> = Ptr::<i32>::null();
    const _: () = assert!(
        true,
        "std::is_same_v<Init,\n                       decltype(std::transform_reduce(p, p, p, Init{{}}))>"
    );;
}
pub fn test_return_type_93() {
    let mut p: Ptr<i32> = Ptr::<i32>::null();
    const _: () = assert!(
        true,
        "std::is_same_v<Init,\n                       decltype(std::transform_reduce(p, p, p, Init{{}}))>"
    );;
}
pub fn test_return_type_94() {
    let mut p: Ptr<f32> = Ptr::<f32>::null();
    const _: () = assert!(
        true,
        "std::is_same_v<Init,\n                       decltype(std::transform_reduce(p, p, p, Init{{}}))>"
    );;
}
pub fn test_return_type_95() {
    let mut p: Ptr<i16> = Ptr::<i16>::null();
    const _: () = assert!(
        true,
        "std::is_same_v<Init,\n                       decltype(std::transform_reduce(p, p, p, Init{{}}))>"
    );;
}
pub fn test_return_type_96() {
    let mut p: Ptr<f64> = Ptr::<f64>::null();
    const _: () = assert!(
        true,
        "std::is_same_v<Init,\n                       decltype(std::transform_reduce(p, p, p, Init{{}}))>"
    );;
}
pub fn test_return_type_97() {
    let mut p: Ptr<i8> = Ptr::<i8>::null();
    const _: () = assert!(
        true,
        "std::is_same_v<Init,\n                       decltype(std::transform_reduce(p, p, p, Init{{}}))>"
    );;
}
pub fn test_move_only_types_98() {
    let ia: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
        MoveOnly::new({ Some(1) }),
        MoveOnly::new({ Some(2) }),
        MoveOnly::new({ Some(3) }),
    ])));
    let mut ib: [MoveOnly; 3] = [
        MoveOnly::new({ Some(1) }),
        MoveOnly::new({ Some(2) }),
        MoveOnly::new({ Some(3) }),
    ];
    assert!(
        (14 == ({ MoveOnlyImpl::get(&Rc::new(RefCell::new(unimplemented!())).as_pointer(),) }))
    );
}
pub fn test_99() -> bool {
    ({ test_return_type_91() });
    ({ test_return_type_92() });
    ({ test_return_type_93() });
    ({ test_return_type_94() });
    ({ test_return_type_95() });
    ({ test_return_type_96() });
    ({ test_return_type_97() });
    ({ test_71() });
    ({ test_72() });
    ({ test_73() });
    ({ test_74() });
    ({ test_75() });
    ({ test_76() });
    ({ test_77() });
    ({ test_78() });
    ({ test_79() });
    ({ test_80() });
    ({ test_81() });
    ({ test_82() });
    ({ test_83() });
    ({ test_84() });
    ({ test_85() });
    ({ test_86() });
    ({ test_87() });
    ({ test_88() });
    ({ test_89() });
    ({ test_90() });
    ({ test_move_only_types_98() });
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
    ({ test_99() });
    const _: () = assert!(true, "test()");;
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
pub trait bidirectional_iterator_const_unsigned_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<u32>;
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_const_unsigned_int_ptr_>;
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_const_unsigned_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_8(&self, mut _a0: i32) -> bidirectional_iterator_const_unsigned_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_9(&self, mut _a0: i32) -> bidirectional_iterator_const_unsigned_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_const_unsigned_int_ptr_>,
    ) -> Ptr<bidirectional_iterator_const_unsigned_int_ptr_>;
}
impl bidirectional_iterator_const_unsigned_int_ptr_Impl
    for Ptr<bidirectional_iterator_const_unsigned_int_ptr_>
{
    fn operator_deref(&self) -> Ptr<u32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_const_unsigned_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_const_unsigned_int_ptr_>,
    ) -> Ptr<bidirectional_iterator_const_unsigned_int_ptr_> {
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
pub trait cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<u32>;
    fn operator_inc_5(
        &self,
    ) -> Ptr<cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_>;
    fn operator_post_inc_6(
        &self,
        mut _a0: i32,
    ) -> cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_>,
    ) -> Ptr<cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_>;
}
impl cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_Impl
    for Ptr<cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_>
{
    fn operator_deref(&self) -> Ptr<u32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_5(
        &self,
    ) -> Ptr<cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_>,
    ) -> Ptr<cpp17_input_iterator_const_unsigned_int_ptr__const_unsigned_int_ptr_> {
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
pub trait forward_iterator_const_unsigned_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<u32>;
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_unsigned_int_ptr_>;
    fn operator_post_inc_7(&self, mut _a0: i32) -> forward_iterator_const_unsigned_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<forward_iterator_const_unsigned_int_ptr_>,
    ) -> Ptr<forward_iterator_const_unsigned_int_ptr_>;
}
impl forward_iterator_const_unsigned_int_ptr_Impl
    for Ptr<forward_iterator_const_unsigned_int_ptr_>
{
    fn operator_deref(&self) -> Ptr<u32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_unsigned_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<forward_iterator_const_unsigned_int_ptr_>,
    ) -> Ptr<forward_iterator_const_unsigned_int_ptr_> {
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
    fn operator_index(&self, n: isize) -> Ptr<i32>;
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
    fn operator_add_assign(&self, n: isize) -> Ptr<random_access_iterator_const_int_ptr_>;
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
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone()).offset((n) as isize);
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<random_access_iterator_const_int_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
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
pub trait random_access_iterator_const_unsigned_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<u32>;
    fn operator_index(&self, n: isize) -> Ptr<u32>;
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_unsigned_int_ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_const_unsigned_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: i32) -> random_access_iterator_const_unsigned_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: i32) -> random_access_iterator_const_unsigned_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: isize) -> Ptr<random_access_iterator_const_unsigned_int_ptr_>;
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<random_access_iterator_const_unsigned_int_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_const_unsigned_int_ptr_>,
    ) -> Ptr<random_access_iterator_const_unsigned_int_ptr_>;
}
impl random_access_iterator_const_unsigned_int_ptr_Impl
    for Ptr<random_access_iterator_const_unsigned_int_ptr_>
{
    fn operator_deref(&self) -> Ptr<u32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_index(&self, mut n: isize) -> Ptr<u32> {
        return (*self).with(|__s| __s.it_.clone()).offset((n) as isize);
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_unsigned_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<random_access_iterator_const_unsigned_int_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_const_unsigned_int_ptr_>,
    ) -> Ptr<random_access_iterator_const_unsigned_int_ptr_> {
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
