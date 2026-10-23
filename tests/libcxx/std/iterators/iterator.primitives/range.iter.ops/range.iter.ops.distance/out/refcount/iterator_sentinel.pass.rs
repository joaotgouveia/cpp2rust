extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_long__long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(1)]
pub struct support_double_move_tracker {
    #[offset(0)]
    moved_from_: (),
}
impl support_double_move_tracker {
    pub fn new() -> Self {
        Self { moved_from_: false }
    }
    pub fn move_from(other: Ptr<support_double_move_tracker>) -> Self {
        let __this: support_double_move_tracker = Self { moved_from_: false };
        unimplemented!();
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
pub fn base_1(i: Ptr<cpp17_output_iterator_int_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(16)]
pub struct cpp17_output_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {
    pub fn new(it: Ptr<()>) -> Self {
        let it: Value<Ptr<()>> = Rc::new(RefCell::new(it));
        Self {
            it_: (*it.borrow()).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
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
pub fn base_2(i: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {
    pub fn new(mut it: Ptr<()>) -> Self {
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
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub fn operator_ne_3(x: Ptr<forward_iterator_int_ptr_>, y: Ptr<forward_iterator_int_ptr_>) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
pub fn base_4(i: Ptr<forward_iterator_int_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(16)]
pub struct forward_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl forward_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
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
pub fn operator_ne_5(
    x: Ptr<bidirectional_iterator_int_ptr_>,
    y: Ptr<bidirectional_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
pub fn base_6(i: Ptr<bidirectional_iterator_int_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(16)]
pub struct bidirectional_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
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
pub fn operator_sub_7(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn base_8(i: Ptr<random_access_iterator_int_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(16)]
pub struct random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_random_access_iterator_int_ptr_ {}
const _: () = assert!(
    true,
    "std::random_access_iterator<cpp20_random_access_iterator<int*>>"
);
pub fn operator_sub_9(
    mut x: contiguous_iterator_int_ptr_,
    mut y: contiguous_iterator_int_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn base_10(i: Ptr<contiguous_iterator_int_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
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
pub fn base_11(i: Ptr<()>) -> Ptr<()> {
    let i: Value<Ptr<()>> = Rc::new(RefCell::new(i));
    return (*i.borrow()).clone();
}
pub fn base_12(i: Ptr<()>) -> Ptr<()> {
    let i: Value<Ptr<()>> = Rc::new(RefCell::new(i));
    return (*i.borrow()).clone();
}
pub fn base_13(i: Ptr<cpp20_input_iterator_int_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_input_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_int_ptr_ {
    pub fn new(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
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
    it_: Ptr<()>,
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
    pub increments: (),
    #[offset(8)]
    pub decrements: (),
    #[offset(16)]
    pub zero_moves: (),
    #[offset(24)]
    pub equal_cmps: (),
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        {
            IteratorOpCounts {
                increments: 0_(),
                decrements: 0_(),
                zero_moves: 0_(),
                equal_cmps: 0_(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ {
    pub fn new_1(it: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>) -> Self {
        Self {
            base_: ({ base_2((it).clone()) }),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_cpp17_output_iterator_int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_cpp17_output_iterator_int_ptr__ {
    pub fn new_1(it: Ptr<cpp17_output_iterator_int_ptr_>) -> Self {
        Self {
            base_: ({ base_1((it).clone()) }),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_forward_iterator_int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_forward_iterator_int_ptr__ {
    pub fn new_1(it: Ptr<forward_iterator_int_ptr_>) -> Self {
        Self {
            base_: ({ base_4((it).clone()) }),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_bidirectional_iterator_int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_bidirectional_iterator_int_ptr__ {
    pub fn new_1(it: Ptr<bidirectional_iterator_int_ptr_>) -> Self {
        Self {
            base_: ({ base_6((it).clone()) }),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_random_access_iterator_int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_random_access_iterator_int_ptr__ {
    pub fn new_1(it: Ptr<random_access_iterator_int_ptr_>) -> Self {
        Self {
            base_: ({ base_8((it).clone()) }),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_contiguous_iterator_int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_contiguous_iterator_int_ptr__ {
    pub fn new_1(it: Ptr<contiguous_iterator_int_ptr_>) -> Self {
        Self {
            base_: ({ base_10((it).clone()) }),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_int_ptr_ {
    pub fn new_1(it: Ptr<Ptr<()>>) -> Self {
        Self {
            base_: ({ base_11((it.read())) }),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_const_int_ptr_ {
    pub fn new_1(it: Ptr<Ptr<()>>) -> Self {
        Self {
            base_: ({ base_12((it.read())) }),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
    pub fn new_1(it: Ptr<cpp20_input_iterator_int_ptr_>) -> Self {
        Self {
            base_: ({ base_13((it).clone()) }),
        }
    }
}
pub fn operator_sub_14(
    s: Ptr<sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__>,
    i: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>,
) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_2((i).clone()) })) as ();
}
pub fn operator_sub_15(
    i: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>,
    s: Ptr<sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__>,
) -> () {
    return (({ base_2((i).clone()) }) - s.with(|__s| __s.base_.clone())) as ();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ {
    pub fn new_1(it: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>) -> Self {
        Self {
            base_: ({ base_2((it).clone()) }),
        }
    }
}
pub fn operator_sub_16(
    s: Ptr<sized_sentinel_cpp20_input_iterator_int_ptr__>,
    i: Ptr<cpp20_input_iterator_int_ptr_>,
) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_13((i).clone()) })) as ();
}
pub fn operator_sub_17(
    i: Ptr<cpp20_input_iterator_int_ptr_>,
    s: Ptr<sized_sentinel_cpp20_input_iterator_int_ptr__>,
) -> () {
    return (({ base_13((i).clone()) }) - s.with(|__s| __s.base_.clone())) as ();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_cpp20_input_iterator_int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sized_sentinel_cpp20_input_iterator_int_ptr__ {
    pub fn new_1(it: Ptr<cpp20_input_iterator_int_ptr_>) -> Self {
        Self {
            base_: ({ base_13((it).clone()) }),
        }
    }
}
pub fn operator_sub_18(
    s: Ptr<sized_sentinel_cpp17_output_iterator_int_ptr__>,
    i: Ptr<cpp17_output_iterator_int_ptr_>,
) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_1((i).clone()) })) as ();
}
pub fn operator_sub_19(
    i: Ptr<cpp17_output_iterator_int_ptr_>,
    s: Ptr<sized_sentinel_cpp17_output_iterator_int_ptr__>,
) -> () {
    return (({ base_1((i).clone()) }) - s.with(|__s| __s.base_.clone())) as ();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_cpp17_output_iterator_int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sized_sentinel_cpp17_output_iterator_int_ptr__ {
    pub fn new_1(it: Ptr<cpp17_output_iterator_int_ptr_>) -> Self {
        Self {
            base_: ({ base_1((it).clone()) }),
        }
    }
}
pub fn operator_sub_20(
    s: Ptr<sized_sentinel_forward_iterator_int_ptr__>,
    i: Ptr<forward_iterator_int_ptr_>,
) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_4((i).clone()) })) as ();
}
pub fn operator_sub_21(
    i: Ptr<forward_iterator_int_ptr_>,
    s: Ptr<sized_sentinel_forward_iterator_int_ptr__>,
) -> () {
    return (({ base_4((i).clone()) }) - s.with(|__s| __s.base_.clone())) as ();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_forward_iterator_int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sized_sentinel_forward_iterator_int_ptr__ {
    pub fn new_1(it: Ptr<forward_iterator_int_ptr_>) -> Self {
        Self {
            base_: ({ base_4((it).clone()) }),
        }
    }
}
pub fn operator_sub_22(
    s: Ptr<sized_sentinel_bidirectional_iterator_int_ptr__>,
    i: Ptr<bidirectional_iterator_int_ptr_>,
) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_6((i).clone()) })) as ();
}
pub fn operator_sub_23(
    i: Ptr<bidirectional_iterator_int_ptr_>,
    s: Ptr<sized_sentinel_bidirectional_iterator_int_ptr__>,
) -> () {
    return (({ base_6((i).clone()) }) - s.with(|__s| __s.base_.clone())) as ();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_bidirectional_iterator_int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sized_sentinel_bidirectional_iterator_int_ptr__ {
    pub fn new_1(it: Ptr<bidirectional_iterator_int_ptr_>) -> Self {
        Self {
            base_: ({ base_6((it).clone()) }),
        }
    }
}
pub fn operator_sub_24(
    s: Ptr<sized_sentinel_random_access_iterator_int_ptr__>,
    i: Ptr<random_access_iterator_int_ptr_>,
) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_8((i).clone()) })) as ();
}
pub fn operator_sub_25(
    i: Ptr<random_access_iterator_int_ptr_>,
    s: Ptr<sized_sentinel_random_access_iterator_int_ptr__>,
) -> () {
    return (({ base_8((i).clone()) }) - s.with(|__s| __s.base_.clone())) as ();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_random_access_iterator_int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sized_sentinel_random_access_iterator_int_ptr__ {
    pub fn new_1(it: Ptr<random_access_iterator_int_ptr_>) -> Self {
        Self {
            base_: ({ base_8((it).clone()) }),
        }
    }
}
pub fn operator_sub_26(
    s: Ptr<sized_sentinel_contiguous_iterator_int_ptr__>,
    i: Ptr<contiguous_iterator_int_ptr_>,
) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_10((i).clone()) })) as ();
}
pub fn operator_sub_27(
    i: Ptr<contiguous_iterator_int_ptr_>,
    s: Ptr<sized_sentinel_contiguous_iterator_int_ptr__>,
) -> () {
    return (({ base_10((i).clone()) }) - s.with(|__s| __s.base_.clone())) as ();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_contiguous_iterator_int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sized_sentinel_contiguous_iterator_int_ptr__ {
    pub fn new_1(it: Ptr<contiguous_iterator_int_ptr_>) -> Self {
        Self {
            base_: ({ base_10((it).clone()) }),
        }
    }
}
pub fn operator_sub_28(s: Ptr<sized_sentinel_int_ptr_>, i: Ptr<Ptr<()>>) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_11((i.read())) })) as ();
}
pub fn operator_sub_29(i: Ptr<Ptr<()>>, s: Ptr<sized_sentinel_int_ptr_>) -> () {
    return (({ base_11((i.read())) }) - s.with(|__s| __s.base_.clone())) as ();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sized_sentinel_int_ptr_ {
    pub fn new_1(it: Ptr<Ptr<()>>) -> Self {
        Self {
            base_: ({ base_11((it.read())) }),
        }
    }
}
pub fn operator_sub_30(s: Ptr<sized_sentinel_const_int_ptr_>, i: Ptr<Ptr<()>>) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_12((i.read())) })) as ();
}
pub fn operator_sub_31(i: Ptr<Ptr<()>>, s: Ptr<sized_sentinel_const_int_ptr_>) -> () {
    return (({ base_12((i.read())) }) - s.with(|__s| __s.base_.clone())) as ();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sized_sentinel_const_int_ptr_ {
    pub fn new_1(it: Ptr<Ptr<()>>) -> Self {
        Self {
            base_: ({ base_12((it.read())) }),
        }
    }
}
pub fn operator_eq_32(
    _a0: Ptr<forward_sized_iterator_int_ptr_>,
    _a1: Ptr<forward_sized_iterator_int_ptr_>,
) -> () {
    return ({ _a0.with(|__s| __s.it_.clone()) } == { _a1.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct forward_sized_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    pub it_: Ptr<()>,
}
impl forward_sized_iterator_int_ptr_ {}
impl std::cmp::PartialEq for forward_sized_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_32(
                Rc::new(RefCell::new(forward_sized_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(forward_sized_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for forward_sized_iterator_int_ptr_ {}
impl Default for forward_sized_iterator_int_ptr_ {
    fn default() -> Self {
        {
            forward_sized_iterator_int_ptr_ {
                it_: Ptr::<()>::null(),
            }
        }
    }
}
const _: () = assert!(true, "std::forward_iterator<forward_sized_iterator<>>");
const _: () = assert!(
    true,
    "std::sized_sentinel_for<forward_sized_iterator<>, forward_sized_iterator<>>"
);
pub fn operator_add_33(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_34(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_33(_i, n)
    });
}
pub fn iter_swap_35(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_36(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) }).reinterpret_cast::<()>();
}
pub fn operator_eq_37(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_38(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
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
    ptr_: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    iter_moves_: Ptr<()>,
    #[offset(16)]
    #[byte_size(8)]
    iter_swaps_: Ptr<()>,
}
impl adl_Iterator {
    fn new_1(mut p: Ptr<()>, mut iter_moves: Ptr<()>, mut iter_swaps: Ptr<()>) -> Self {
        Self {
            ptr_: (p).clone(),
            iter_moves_: (iter_moves).clone(),
            iter_swaps_: (iter_swaps).clone(),
        }
    }
    pub fn TrackMoves(mut p: Ptr<()>, iter_moves: Ptr<()>) -> adl_Iterator {
        return adl_Iterator::new_1({ (p).clone() }, { (iter_moves).clone() }, {
            Ptr::<()>::null()
        });
    }
    pub fn TrackSwaps_4(iter_swaps: Ptr<()>) -> adl_Iterator {
        return adl_Iterator::new_1({ Ptr::<()>::null() }, { Ptr::<()>::null() }, {
            (iter_swaps).clone()
        });
    }
    pub fn TrackSwaps_5(mut p: Ptr<()>, iter_swaps: Ptr<()>) -> adl_Iterator {
        return adl_Iterator::new_1({ (p).clone() }, { Ptr::<()>::null() }, {
            (iter_swaps).clone()
        });
    }
}
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_38(
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: self.ptr_.clone(),
                    iter_moves_: self.iter_moves_.clone(),
                    iter_swaps_: self.iter_swaps_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: other.ptr_.clone(),
                    iter_moves_: other.iter_moves_.clone(),
                    iter_swaps_: other.iter_swaps_.clone(),
                }))
                .as_pointer(),
            )
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
            operator_eq_37(
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: self.ptr_.clone(),
                    iter_moves_: self.iter_moves_.clone(),
                    iter_swaps_: self.iter_swaps_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: other.ptr_.clone(),
                    iter_moves_: other.iter_moves_.clone(),
                    iter_swaps_: other.iter_swaps_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for adl_Iterator {}
impl Default for adl_Iterator {
    fn default() -> Self {
        {
            adl_Iterator {
                ptr_: Ptr::<()>::null(),
                iter_moves_: Ptr::<()>::null(),
                iter_swaps_: Ptr::<()>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct rvalue_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<Ptr<()>>,
}
impl rvalue_iterator_int_ptr_ {}
impl std::cmp::Ord for rvalue_iterator_int_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            rvalue_iterator_int_ptr_Impl::operator_cmp(
                &Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for rvalue_iterator_int_ptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for rvalue_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            rvalue_iterator_int_ptr_Impl::operator_cmp(
                &Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for rvalue_iterator_int_ptr_ {}
const _: () = assert!(true, "std::random_access_iterator<rvalue_iterator<int*>>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub fn new(mut what: Ptr<()>) -> Self {
        Self {}
    }
}
pub fn test_unsized_39() {
    const _: () = assert!(
        true,
        "std::sentinel_for<Sent, It> && !std::sized_sentinel_for<Sent, It>"
    );;
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ (a.as_pointer() as Ptr<()>) });
        let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
            let __tmp_0: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                    (a.as_pointer() as Ptr<()>)
                })));
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                __tmp_0.as_pointer()
            })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_1: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_1.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_2: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_2.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_3: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_3.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_4: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_4.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_5: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_5.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_6: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_6.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_7: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_7.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_8: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_8.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_9: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_9.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_10: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_10.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_11: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_11.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_12: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_12.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_13: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_13.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_14: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_14.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_15: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_15.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_16: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_16.as_pointer()
                    })
                };
                unimplemented!();
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
        ({ (*check.borrow()).spec::<fn()>(2).call() });
        ({ (*check.borrow()).spec::<fn()>(3).call() });
        ({ (*check.borrow()).spec::<fn()>(4).call() });
        ({ (*check.borrow()).spec::<fn()>(5).call() });
        ({ (*check.borrow()).spec::<fn()>(6).call() });
        ({ (*check.borrow()).spec::<fn()>(7).call() });
        ({ (*check.borrow()).spec::<fn()>(8).call() });
        ({ (*check.borrow()).spec::<fn()>(9).call() });
        ({ (*check.borrow()).spec::<fn()>(10).call() });
        ({ (*check.borrow()).spec::<fn()>(11).call() });
        ({ (*check.borrow()).spec::<fn()>(12).call() });
        ({ (*check.borrow()).spec::<fn()>(13).call() });
        ({ (*check.borrow()).spec::<fn()>(14).call() });
        ({ (*check.borrow()).spec::<fn()>(15).call() });
    }
}
pub fn test_unsized_40() {
    const _: () = assert!(
        true,
        "std::sentinel_for<Sent, It> && !std::sized_sentinel_for<Sent, It>"
    );;
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let mut first: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ (a.as_pointer() as Ptr<()>) });
        let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
            let __tmp_17: Value<cpp17_output_iterator_int_ptr_> =
                Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                    (a.as_pointer() as Ptr<()>)
                })));
            sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ __tmp_17.as_pointer() })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_18: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_18.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_19: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_19.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_20: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_20.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_21: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_21.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_22: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_22.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_23: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_23.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_24: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_24.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_25: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_25.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_26: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_26.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_27: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_27.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_28: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_28.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_29: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_29.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_30: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_30.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_31: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_31.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_32: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_32.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).clone() });
                let mut last: sentinel_wrapper_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_33: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_33.as_pointer()
                    })
                };
                unimplemented!();
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
        ({ (*check.borrow()).spec::<fn()>(2).call() });
        ({ (*check.borrow()).spec::<fn()>(3).call() });
        ({ (*check.borrow()).spec::<fn()>(4).call() });
        ({ (*check.borrow()).spec::<fn()>(5).call() });
        ({ (*check.borrow()).spec::<fn()>(6).call() });
        ({ (*check.borrow()).spec::<fn()>(7).call() });
        ({ (*check.borrow()).spec::<fn()>(8).call() });
        ({ (*check.borrow()).spec::<fn()>(9).call() });
        ({ (*check.borrow()).spec::<fn()>(10).call() });
        ({ (*check.borrow()).spec::<fn()>(11).call() });
        ({ (*check.borrow()).spec::<fn()>(12).call() });
        ({ (*check.borrow()).spec::<fn()>(13).call() });
        ({ (*check.borrow()).spec::<fn()>(14).call() });
        ({ (*check.borrow()).spec::<fn()>(15).call() });
    }
}
pub fn test_unsized_41() {
    const _: () = assert!(
        true,
        "std::sentinel_for<Sent, It> && !std::sized_sentinel_for<Sent, It>"
    );;
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let mut first: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ (a.as_pointer() as Ptr<()>) });
        let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
            let __tmp_34: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    (a.as_pointer() as Ptr<()>)
                })));
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_34.as_pointer() })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let __tmp_35: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_35.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let __tmp_36: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_36.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let __tmp_37: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_37.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let __tmp_38: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_38.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let __tmp_39: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_39.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let __tmp_40: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_40.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let __tmp_41: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_41.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let __tmp_42: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_42.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let __tmp_43: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_43.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let __tmp_44: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_44.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let __tmp_45: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_45.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let __tmp_46: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_46.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let __tmp_47: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_47.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let __tmp_48: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_48.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let __tmp_49: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_49.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let __tmp_50: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_50.as_pointer() })
                };
                unimplemented!();
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
        ({ (*check.borrow()).spec::<fn()>(2).call() });
        ({ (*check.borrow()).spec::<fn()>(3).call() });
        ({ (*check.borrow()).spec::<fn()>(4).call() });
        ({ (*check.borrow()).spec::<fn()>(5).call() });
        ({ (*check.borrow()).spec::<fn()>(6).call() });
        ({ (*check.borrow()).spec::<fn()>(7).call() });
        ({ (*check.borrow()).spec::<fn()>(8).call() });
        ({ (*check.borrow()).spec::<fn()>(9).call() });
        ({ (*check.borrow()).spec::<fn()>(10).call() });
        ({ (*check.borrow()).spec::<fn()>(11).call() });
        ({ (*check.borrow()).spec::<fn()>(12).call() });
        ({ (*check.borrow()).spec::<fn()>(13).call() });
        ({ (*check.borrow()).spec::<fn()>(14).call() });
        ({ (*check.borrow()).spec::<fn()>(15).call() });
    }
}
pub fn test_unsized_42() {
    const _: () = assert!(
        true,
        "std::sentinel_for<Sent, It> && !std::sized_sentinel_for<Sent, It>"
    );;
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let mut first: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ (a.as_pointer() as Ptr<()>) });
        let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
            let __tmp_51: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    (a.as_pointer() as Ptr<()>)
                })));
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ __tmp_51.as_pointer() })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let __tmp_52: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_52.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let __tmp_53: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_53.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let __tmp_54: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_54.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let __tmp_55: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_55.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let __tmp_56: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_56.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let __tmp_57: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_57.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let __tmp_58: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_58.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let __tmp_59: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_59.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let __tmp_60: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_60.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let __tmp_61: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_61.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let __tmp_62: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_62.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let __tmp_63: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_63.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let __tmp_64: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_64.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let __tmp_65: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_65.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let __tmp_66: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_66.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let __tmp_67: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_67.as_pointer()
                    })
                };
                unimplemented!();
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
        ({ (*check.borrow()).spec::<fn()>(2).call() });
        ({ (*check.borrow()).spec::<fn()>(3).call() });
        ({ (*check.borrow()).spec::<fn()>(4).call() });
        ({ (*check.borrow()).spec::<fn()>(5).call() });
        ({ (*check.borrow()).spec::<fn()>(6).call() });
        ({ (*check.borrow()).spec::<fn()>(7).call() });
        ({ (*check.borrow()).spec::<fn()>(8).call() });
        ({ (*check.borrow()).spec::<fn()>(9).call() });
        ({ (*check.borrow()).spec::<fn()>(10).call() });
        ({ (*check.borrow()).spec::<fn()>(11).call() });
        ({ (*check.borrow()).spec::<fn()>(12).call() });
        ({ (*check.borrow()).spec::<fn()>(13).call() });
        ({ (*check.borrow()).spec::<fn()>(14).call() });
        ({ (*check.borrow()).spec::<fn()>(15).call() });
    }
}
pub fn test_unsized_43() {
    const _: () = assert!(
        true,
        "std::sentinel_for<Sent, It> && !std::sized_sentinel_for<Sent, It>"
    );;
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let mut first: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ (a.as_pointer() as Ptr<()>) });
        let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
            let __tmp_68: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    (a.as_pointer() as Ptr<()>)
                })));
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ __tmp_68.as_pointer() })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let __tmp_69: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                        __tmp_69.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let __tmp_70: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                        __tmp_70.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let __tmp_71: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                        __tmp_71.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let __tmp_72: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                        __tmp_72.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let __tmp_73: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                        __tmp_73.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let __tmp_74: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                        __tmp_74.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let __tmp_75: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                        __tmp_75.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let __tmp_76: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                        __tmp_76.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let __tmp_77: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                        __tmp_77.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let __tmp_78: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                        __tmp_78.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let __tmp_79: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                        __tmp_79.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let __tmp_80: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                        __tmp_80.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let __tmp_81: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                        __tmp_81.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let __tmp_82: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                        __tmp_82.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let __tmp_83: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                        __tmp_83.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let __tmp_84: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                        __tmp_84.as_pointer()
                    })
                };
                unimplemented!();
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
        ({ (*check.borrow()).spec::<fn()>(2).call() });
        ({ (*check.borrow()).spec::<fn()>(3).call() });
        ({ (*check.borrow()).spec::<fn()>(4).call() });
        ({ (*check.borrow()).spec::<fn()>(5).call() });
        ({ (*check.borrow()).spec::<fn()>(6).call() });
        ({ (*check.borrow()).spec::<fn()>(7).call() });
        ({ (*check.borrow()).spec::<fn()>(8).call() });
        ({ (*check.borrow()).spec::<fn()>(9).call() });
        ({ (*check.borrow()).spec::<fn()>(10).call() });
        ({ (*check.borrow()).spec::<fn()>(11).call() });
        ({ (*check.borrow()).spec::<fn()>(12).call() });
        ({ (*check.borrow()).spec::<fn()>(13).call() });
        ({ (*check.borrow()).spec::<fn()>(14).call() });
        ({ (*check.borrow()).spec::<fn()>(15).call() });
    }
}
pub fn test_unsized_44() {
    const _: () = assert!(
        true,
        "std::sentinel_for<Sent, It> && !std::sized_sentinel_for<Sent, It>"
    );;
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let mut first: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ (a.as_pointer() as Ptr<()>) });
        let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
            let __tmp_85: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    (a.as_pointer() as Ptr<()>)
                })));
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_85.as_pointer() })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let __tmp_86: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_86.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let __tmp_87: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_87.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let __tmp_88: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_88.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let __tmp_89: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_89.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let __tmp_90: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_90.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let __tmp_91: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_91.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let __tmp_92: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_92.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let __tmp_93: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_93.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let __tmp_94: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_94.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let __tmp_95: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_95.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let __tmp_96: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_96.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let __tmp_97: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_97.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let __tmp_98: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_98.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let __tmp_99: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_99.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let __tmp_100: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({
                        __tmp_100.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let __tmp_101: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).offset((3_()) as isize)
                        })));
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({
                        __tmp_101.as_pointer()
                    })
                };
                unimplemented!();
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
        ({ (*check.borrow()).spec::<fn()>(2).call() });
        ({ (*check.borrow()).spec::<fn()>(3).call() });
        ({ (*check.borrow()).spec::<fn()>(4).call() });
        ({ (*check.borrow()).spec::<fn()>(5).call() });
        ({ (*check.borrow()).spec::<fn()>(6).call() });
        ({ (*check.borrow()).spec::<fn()>(7).call() });
        ({ (*check.borrow()).spec::<fn()>(8).call() });
        ({ (*check.borrow()).spec::<fn()>(9).call() });
        ({ (*check.borrow()).spec::<fn()>(10).call() });
        ({ (*check.borrow()).spec::<fn()>(11).call() });
        ({ (*check.borrow()).spec::<fn()>(12).call() });
        ({ (*check.borrow()).spec::<fn()>(13).call() });
        ({ (*check.borrow()).spec::<fn()>(14).call() });
        ({ (*check.borrow()).spec::<fn()>(15).call() });
    }
}
pub fn test_unsized_45() {
    const _: () = assert!(
        true,
        "std::sentinel_for<Sent, It> && !std::sized_sentinel_for<Sent, It>"
    );;
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let mut first: Ptr<()> = (a.as_pointer() as Ptr<()>);
        let mut last: sentinel_wrapper_int_ptr_ = {
            let __tmp_102: Value<Ptr<()>> = Rc::new(RefCell::new((a.as_pointer() as Ptr<()>)));
            sentinel_wrapper_int_ptr_::new_1({ __tmp_102.as_pointer() })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let __tmp_103: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_int_ptr_::new_1({ __tmp_103.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let __tmp_104: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_int_ptr_::new_1({ __tmp_104.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let __tmp_105: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_int_ptr_::new_1({ __tmp_105.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let __tmp_106: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_int_ptr_::new_1({ __tmp_106.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let __tmp_107: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_int_ptr_::new_1({ __tmp_107.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let __tmp_108: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_int_ptr_::new_1({ __tmp_108.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let __tmp_109: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_int_ptr_::new_1({ __tmp_109.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let __tmp_110: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_int_ptr_::new_1({ __tmp_110.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let __tmp_111: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_int_ptr_::new_1({ __tmp_111.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let __tmp_112: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_int_ptr_::new_1({ __tmp_112.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let __tmp_113: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_int_ptr_::new_1({ __tmp_113.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let __tmp_114: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_int_ptr_::new_1({ __tmp_114.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let __tmp_115: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_int_ptr_::new_1({ __tmp_115.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let __tmp_116: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_int_ptr_::new_1({ __tmp_116.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let __tmp_117: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_int_ptr_::new_1({ __tmp_117.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_int_ptr_ = {
                    let __tmp_118: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_int_ptr_::new_1({ __tmp_118.as_pointer() })
                };
                unimplemented!();
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
        ({ (*check.borrow()).spec::<fn()>(2).call() });
        ({ (*check.borrow()).spec::<fn()>(3).call() });
        ({ (*check.borrow()).spec::<fn()>(4).call() });
        ({ (*check.borrow()).spec::<fn()>(5).call() });
        ({ (*check.borrow()).spec::<fn()>(6).call() });
        ({ (*check.borrow()).spec::<fn()>(7).call() });
        ({ (*check.borrow()).spec::<fn()>(8).call() });
        ({ (*check.borrow()).spec::<fn()>(9).call() });
        ({ (*check.borrow()).spec::<fn()>(10).call() });
        ({ (*check.borrow()).spec::<fn()>(11).call() });
        ({ (*check.borrow()).spec::<fn()>(12).call() });
        ({ (*check.borrow()).spec::<fn()>(13).call() });
        ({ (*check.borrow()).spec::<fn()>(14).call() });
        ({ (*check.borrow()).spec::<fn()>(15).call() });
    }
}
pub fn test_unsized_46() {
    const _: () = assert!(
        true,
        "std::sentinel_for<Sent, It> && !std::sized_sentinel_for<Sent, It>"
    );;
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let mut first: Ptr<()> = (a.as_pointer() as Ptr<()>);
        let mut last: sentinel_wrapper_const_int_ptr_ = {
            let __tmp_119: Value<Ptr<()>> = Rc::new(RefCell::new((a.as_pointer() as Ptr<()>)));
            sentinel_wrapper_const_int_ptr_::new_1({ __tmp_119.as_pointer() })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let __tmp_120: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_const_int_ptr_::new_1({ __tmp_120.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let __tmp_121: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_const_int_ptr_::new_1({ __tmp_121.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let __tmp_122: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_const_int_ptr_::new_1({ __tmp_122.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let __tmp_123: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_const_int_ptr_::new_1({ __tmp_123.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let __tmp_124: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_const_int_ptr_::new_1({ __tmp_124.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let __tmp_125: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_const_int_ptr_::new_1({ __tmp_125.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let __tmp_126: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_const_int_ptr_::new_1({ __tmp_126.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let __tmp_127: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_const_int_ptr_::new_1({ __tmp_127.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let __tmp_128: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_const_int_ptr_::new_1({ __tmp_128.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let __tmp_129: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_const_int_ptr_::new_1({ __tmp_129.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let __tmp_130: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_const_int_ptr_::new_1({ __tmp_130.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let __tmp_131: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_const_int_ptr_::new_1({ __tmp_131.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let __tmp_132: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_const_int_ptr_::new_1({ __tmp_132.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let __tmp_133: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_const_int_ptr_::new_1({ __tmp_133.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let __tmp_134: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_const_int_ptr_::new_1({ __tmp_134.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).clone();
                let mut last: sentinel_wrapper_const_int_ptr_ = {
                    let __tmp_135: Value<Ptr<()>> =
                        Rc::new(RefCell::new((a).offset((3_()) as isize)));
                    sentinel_wrapper_const_int_ptr_::new_1({ __tmp_135.as_pointer() })
                };
                unimplemented!();
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
        ({ (*check.borrow()).spec::<fn()>(2).call() });
        ({ (*check.borrow()).spec::<fn()>(3).call() });
        ({ (*check.borrow()).spec::<fn()>(4).call() });
        ({ (*check.borrow()).spec::<fn()>(5).call() });
        ({ (*check.borrow()).spec::<fn()>(6).call() });
        ({ (*check.borrow()).spec::<fn()>(7).call() });
        ({ (*check.borrow()).spec::<fn()>(8).call() });
        ({ (*check.borrow()).spec::<fn()>(9).call() });
        ({ (*check.borrow()).spec::<fn()>(10).call() });
        ({ (*check.borrow()).spec::<fn()>(11).call() });
        ({ (*check.borrow()).spec::<fn()>(12).call() });
        ({ (*check.borrow()).spec::<fn()>(13).call() });
        ({ (*check.borrow()).spec::<fn()>(14).call() });
        ({ (*check.borrow()).spec::<fn()>(15).call() });
    }
}
pub fn test_unsized_47() {
    const _: () = assert!(
        true,
        "std::sentinel_for<Sent, It> && !std::sized_sentinel_for<Sent, It>"
    );;
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let mut first: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ (a.as_pointer() as Ptr<()>) });
        let mut last: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ (a.as_pointer() as Ptr<()>) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
        ({ (*check.borrow()).spec::<fn()>(2).call() });
        ({ (*check.borrow()).spec::<fn()>(3).call() });
        ({ (*check.borrow()).spec::<fn()>(4).call() });
        ({ (*check.borrow()).spec::<fn()>(5).call() });
        ({ (*check.borrow()).spec::<fn()>(6).call() });
        ({ (*check.borrow()).spec::<fn()>(7).call() });
        ({ (*check.borrow()).spec::<fn()>(8).call() });
        ({ (*check.borrow()).spec::<fn()>(9).call() });
        ({ (*check.borrow()).spec::<fn()>(10).call() });
        ({ (*check.borrow()).spec::<fn()>(11).call() });
        ({ (*check.borrow()).spec::<fn()>(12).call() });
        ({ (*check.borrow()).spec::<fn()>(13).call() });
        ({ (*check.borrow()).spec::<fn()>(14).call() });
        ({ (*check.borrow()).spec::<fn()>(15).call() });
    }
}
pub fn test_unsized_48() {
    const _: () = assert!(
        true,
        "std::sentinel_for<Sent, It> && !std::sized_sentinel_for<Sent, It>"
    );;
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let mut first: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ (a.as_pointer() as Ptr<()>) });
        let mut last: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ (a.as_pointer() as Ptr<()>) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).clone() });
                let mut last: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                unimplemented!();
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
        ({ (*check.borrow()).spec::<fn()>(2).call() });
        ({ (*check.borrow()).spec::<fn()>(3).call() });
        ({ (*check.borrow()).spec::<fn()>(4).call() });
        ({ (*check.borrow()).spec::<fn()>(5).call() });
        ({ (*check.borrow()).spec::<fn()>(6).call() });
        ({ (*check.borrow()).spec::<fn()>(7).call() });
        ({ (*check.borrow()).spec::<fn()>(8).call() });
        ({ (*check.borrow()).spec::<fn()>(9).call() });
        ({ (*check.borrow()).spec::<fn()>(10).call() });
        ({ (*check.borrow()).spec::<fn()>(11).call() });
        ({ (*check.borrow()).spec::<fn()>(12).call() });
        ({ (*check.borrow()).spec::<fn()>(13).call() });
        ({ (*check.borrow()).spec::<fn()>(14).call() });
        ({ (*check.borrow()).spec::<fn()>(15).call() });
    }
}
pub fn test_sized_49() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_136: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_136.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_137: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_137.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_138: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_138.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_139: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_139.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_140: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_140.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_141: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_141.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_142: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_142.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_143: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_143.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_144: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_144.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_145: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_145.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_146: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_146.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_147: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_147.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_148: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_148.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_149: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_149.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_150: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_150.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
                    let __tmp_151: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                        Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                        __tmp_151.as_pointer()
                    })
                };
                unimplemented!();
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
        ({ (*check.borrow()).spec::<fn()>(2).call() });
        ({ (*check.borrow()).spec::<fn()>(3).call() });
        ({ (*check.borrow()).spec::<fn()>(4).call() });
        ({ (*check.borrow()).spec::<fn()>(5).call() });
        ({ (*check.borrow()).spec::<fn()>(6).call() });
        ({ (*check.borrow()).spec::<fn()>(7).call() });
        ({ (*check.borrow()).spec::<fn()>(8).call() });
        ({ (*check.borrow()).spec::<fn()>(9).call() });
        ({ (*check.borrow()).spec::<fn()>(10).call() });
        ({ (*check.borrow()).spec::<fn()>(11).call() });
        ({ (*check.borrow()).spec::<fn()>(12).call() });
        ({ (*check.borrow()).spec::<fn()>(13).call() });
        ({ (*check.borrow()).spec::<fn()>(14).call() });
        ({ (*check.borrow()).spec::<fn()>(15).call() });
    }
    {
        let first: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
            Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                (a.as_pointer() as Ptr<()>)
            })));
        let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
            let __tmp_152: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                    (a.as_pointer() as Ptr<()>)
                })));
            sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                __tmp_152.as_pointer()
            })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let first: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
            Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                (a.as_pointer() as Ptr<()>)
            })));
        let mut last: sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ = {
            let __tmp_153: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                    (a.as_pointer() as Ptr<()>).offset((3_()) as isize)
                })));
            sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                __tmp_153.as_pointer()
            })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_sized_50() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let __tmp_154: Value<cpp20_input_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ __tmp_154.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let __tmp_155: Value<cpp20_input_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ __tmp_155.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let __tmp_156: Value<cpp20_input_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ __tmp_156.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let __tmp_157: Value<cpp20_input_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ __tmp_157.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let __tmp_158: Value<cpp20_input_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ __tmp_158.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let __tmp_159: Value<cpp20_input_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ __tmp_159.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let __tmp_160: Value<cpp20_input_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ __tmp_160.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let __tmp_161: Value<cpp20_input_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ __tmp_161.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let __tmp_162: Value<cpp20_input_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ __tmp_162.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let __tmp_163: Value<cpp20_input_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ __tmp_163.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let __tmp_164: Value<cpp20_input_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ __tmp_164.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let __tmp_165: Value<cpp20_input_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ __tmp_165.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let __tmp_166: Value<cpp20_input_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ __tmp_166.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let __tmp_167: Value<cpp20_input_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ __tmp_167.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let __tmp_168: Value<cpp20_input_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ __tmp_168.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp20_input_iterator_int_ptr_ =
                    cpp20_input_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
                    let __tmp_169: Value<cpp20_input_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ __tmp_169.as_pointer() })
                };
                unimplemented!();
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
        ({ (*check.borrow()).spec::<fn()>(2).call() });
        ({ (*check.borrow()).spec::<fn()>(3).call() });
        ({ (*check.borrow()).spec::<fn()>(4).call() });
        ({ (*check.borrow()).spec::<fn()>(5).call() });
        ({ (*check.borrow()).spec::<fn()>(6).call() });
        ({ (*check.borrow()).spec::<fn()>(7).call() });
        ({ (*check.borrow()).spec::<fn()>(8).call() });
        ({ (*check.borrow()).spec::<fn()>(9).call() });
        ({ (*check.borrow()).spec::<fn()>(10).call() });
        ({ (*check.borrow()).spec::<fn()>(11).call() });
        ({ (*check.borrow()).spec::<fn()>(12).call() });
        ({ (*check.borrow()).spec::<fn()>(13).call() });
        ({ (*check.borrow()).spec::<fn()>(14).call() });
        ({ (*check.borrow()).spec::<fn()>(15).call() });
    }
    {
        let first: Value<cpp20_input_iterator_int_ptr_> =
            Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                (a.as_pointer() as Ptr<()>)
            })));
        let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
            let __tmp_170: Value<cpp20_input_iterator_int_ptr_> =
                Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                    (a.as_pointer() as Ptr<()>)
                })));
            sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ __tmp_170.as_pointer() })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let first: Value<cpp20_input_iterator_int_ptr_> =
            Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                (a.as_pointer() as Ptr<()>)
            })));
        let mut last: sized_sentinel_cpp20_input_iterator_int_ptr__ = {
            let __tmp_171: Value<cpp20_input_iterator_int_ptr_> =
                Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                    (a.as_pointer() as Ptr<()>).offset((3_()) as isize)
                })));
            sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ __tmp_171.as_pointer() })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_sized_51() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_172: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_172.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_173: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_173.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_174: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_174.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_175: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_175.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_176: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_176.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_177: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_177.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_178: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_178.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_179: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_179.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_180: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_180.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_181: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_181.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_182: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_182.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_183: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_183.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_184: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_184.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_185: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_185.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_186: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_186.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: cpp17_output_iterator_int_ptr_ =
                    cpp17_output_iterator_int_ptr_::new({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
                    let __tmp_187: Value<cpp17_output_iterator_int_ptr_> =
                        Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                            (a).clone()
                        })));
                    sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({
                        __tmp_187.as_pointer()
                    })
                };
                unimplemented!();
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
        ({ (*check.borrow()).spec::<fn()>(2).call() });
        ({ (*check.borrow()).spec::<fn()>(3).call() });
        ({ (*check.borrow()).spec::<fn()>(4).call() });
        ({ (*check.borrow()).spec::<fn()>(5).call() });
        ({ (*check.borrow()).spec::<fn()>(6).call() });
        ({ (*check.borrow()).spec::<fn()>(7).call() });
        ({ (*check.borrow()).spec::<fn()>(8).call() });
        ({ (*check.borrow()).spec::<fn()>(9).call() });
        ({ (*check.borrow()).spec::<fn()>(10).call() });
        ({ (*check.borrow()).spec::<fn()>(11).call() });
        ({ (*check.borrow()).spec::<fn()>(12).call() });
        ({ (*check.borrow()).spec::<fn()>(13).call() });
        ({ (*check.borrow()).spec::<fn()>(14).call() });
        ({ (*check.borrow()).spec::<fn()>(15).call() });
    }
    {
        let first: Value<cpp17_output_iterator_int_ptr_> =
            Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                (a.as_pointer() as Ptr<()>)
            })));
        let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
            let __tmp_188: Value<cpp17_output_iterator_int_ptr_> =
                Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                    (a.as_pointer() as Ptr<()>)
                })));
            sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({ __tmp_188.as_pointer() })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let first: Value<cpp17_output_iterator_int_ptr_> =
            Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                (a.as_pointer() as Ptr<()>)
            })));
        let mut last: sized_sentinel_cpp17_output_iterator_int_ptr__ = {
            let __tmp_189: Value<cpp17_output_iterator_int_ptr_> =
                Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                    (a.as_pointer() as Ptr<()>).offset((3_()) as isize)
                })));
            sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({ __tmp_189.as_pointer() })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_sized_52() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let __tmp_190: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_190.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let __tmp_191: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_191.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let __tmp_192: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_192.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let __tmp_193: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_193.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let __tmp_194: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_194.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let __tmp_195: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_195.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let __tmp_196: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_196.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let __tmp_197: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_197.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let __tmp_198: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_198.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let __tmp_199: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_199.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let __tmp_200: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_200.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let __tmp_201: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_201.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let __tmp_202: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_202.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let __tmp_203: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_203.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let __tmp_204: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_204.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
                    let __tmp_205: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_205.as_pointer() })
                };
                unimplemented!();
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
        ({ (*check.borrow()).spec::<fn()>(2).call() });
        ({ (*check.borrow()).spec::<fn()>(3).call() });
        ({ (*check.borrow()).spec::<fn()>(4).call() });
        ({ (*check.borrow()).spec::<fn()>(5).call() });
        ({ (*check.borrow()).spec::<fn()>(6).call() });
        ({ (*check.borrow()).spec::<fn()>(7).call() });
        ({ (*check.borrow()).spec::<fn()>(8).call() });
        ({ (*check.borrow()).spec::<fn()>(9).call() });
        ({ (*check.borrow()).spec::<fn()>(10).call() });
        ({ (*check.borrow()).spec::<fn()>(11).call() });
        ({ (*check.borrow()).spec::<fn()>(12).call() });
        ({ (*check.borrow()).spec::<fn()>(13).call() });
        ({ (*check.borrow()).spec::<fn()>(14).call() });
        ({ (*check.borrow()).spec::<fn()>(15).call() });
    }
    {
        let first: Value<forward_iterator_int_ptr_> =
            Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                (a.as_pointer() as Ptr<()>)
            })));
        let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
            let __tmp_206: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    (a.as_pointer() as Ptr<()>)
                })));
            sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_206.as_pointer() })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let first: Value<forward_iterator_int_ptr_> =
            Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                (a.as_pointer() as Ptr<()>)
            })));
        let mut last: sized_sentinel_forward_iterator_int_ptr__ = {
            let __tmp_207: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    (a.as_pointer() as Ptr<()>).offset((3_()) as isize)
                })));
            sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_207.as_pointer() })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_sized_53() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let __tmp_208: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_208.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let __tmp_209: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_209.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let __tmp_210: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_210.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let __tmp_211: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_211.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let __tmp_212: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_212.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let __tmp_213: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_213.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let __tmp_214: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_214.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let __tmp_215: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_215.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let __tmp_216: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_216.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let __tmp_217: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_217.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let __tmp_218: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_218.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let __tmp_219: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_219.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let __tmp_220: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_220.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let __tmp_221: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_221.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let __tmp_222: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_222.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let __tmp_223: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_223.as_pointer()
                    })
                };
                unimplemented!();
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
        ({ (*check.borrow()).spec::<fn()>(2).call() });
        ({ (*check.borrow()).spec::<fn()>(3).call() });
        ({ (*check.borrow()).spec::<fn()>(4).call() });
        ({ (*check.borrow()).spec::<fn()>(5).call() });
        ({ (*check.borrow()).spec::<fn()>(6).call() });
        ({ (*check.borrow()).spec::<fn()>(7).call() });
        ({ (*check.borrow()).spec::<fn()>(8).call() });
        ({ (*check.borrow()).spec::<fn()>(9).call() });
        ({ (*check.borrow()).spec::<fn()>(10).call() });
        ({ (*check.borrow()).spec::<fn()>(11).call() });
        ({ (*check.borrow()).spec::<fn()>(12).call() });
        ({ (*check.borrow()).spec::<fn()>(13).call() });
        ({ (*check.borrow()).spec::<fn()>(14).call() });
        ({ (*check.borrow()).spec::<fn()>(15).call() });
    }
    {
        let first: Value<bidirectional_iterator_int_ptr_> =
            Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                (a.as_pointer() as Ptr<()>)
            })));
        let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
            let __tmp_224: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    (a.as_pointer() as Ptr<()>)
                })));
            sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ __tmp_224.as_pointer() })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let first: Value<bidirectional_iterator_int_ptr_> =
            Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                (a.as_pointer() as Ptr<()>)
            })));
        let mut last: sized_sentinel_bidirectional_iterator_int_ptr__ = {
            let __tmp_225: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    (a.as_pointer() as Ptr<()>).offset((3_()) as isize)
                })));
            sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ __tmp_225.as_pointer() })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_sized_54() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let __tmp_226: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({
                        __tmp_226.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let __tmp_227: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({
                        __tmp_227.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let __tmp_228: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({
                        __tmp_228.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let __tmp_229: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({
                        __tmp_229.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let __tmp_230: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({
                        __tmp_230.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let __tmp_231: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({
                        __tmp_231.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let __tmp_232: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({
                        __tmp_232.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let __tmp_233: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({
                        __tmp_233.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let __tmp_234: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({
                        __tmp_234.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let __tmp_235: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({
                        __tmp_235.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let __tmp_236: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({
                        __tmp_236.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let __tmp_237: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({
                        __tmp_237.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let __tmp_238: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({
                        __tmp_238.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let __tmp_239: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({
                        __tmp_239.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let __tmp_240: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({
                        __tmp_240.as_pointer()
                    })
                };
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let __tmp_241: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({
                        __tmp_241.as_pointer()
                    })
                };
                unimplemented!();
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
        ({ (*check.borrow()).spec::<fn()>(2).call() });
        ({ (*check.borrow()).spec::<fn()>(3).call() });
        ({ (*check.borrow()).spec::<fn()>(4).call() });
        ({ (*check.borrow()).spec::<fn()>(5).call() });
        ({ (*check.borrow()).spec::<fn()>(6).call() });
        ({ (*check.borrow()).spec::<fn()>(7).call() });
        ({ (*check.borrow()).spec::<fn()>(8).call() });
        ({ (*check.borrow()).spec::<fn()>(9).call() });
        ({ (*check.borrow()).spec::<fn()>(10).call() });
        ({ (*check.borrow()).spec::<fn()>(11).call() });
        ({ (*check.borrow()).spec::<fn()>(12).call() });
        ({ (*check.borrow()).spec::<fn()>(13).call() });
        ({ (*check.borrow()).spec::<fn()>(14).call() });
        ({ (*check.borrow()).spec::<fn()>(15).call() });
    }
    {
        let first: Value<random_access_iterator_int_ptr_> =
            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                (a.as_pointer() as Ptr<()>)
            })));
        let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
            let __tmp_242: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    (a.as_pointer() as Ptr<()>)
                })));
            sized_sentinel_random_access_iterator_int_ptr__::new_1({ __tmp_242.as_pointer() })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let first: Value<random_access_iterator_int_ptr_> =
            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                (a.as_pointer() as Ptr<()>)
            })));
        let mut last: sized_sentinel_random_access_iterator_int_ptr__ = {
            let __tmp_243: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    (a.as_pointer() as Ptr<()>).offset((3_()) as isize)
                })));
            sized_sentinel_random_access_iterator_int_ptr__::new_1({ __tmp_243.as_pointer() })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_sized_55() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let __tmp_244: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_244.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let __tmp_245: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_245.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let __tmp_246: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_246.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let __tmp_247: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_247.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let __tmp_248: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_248.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let __tmp_249: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_249.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let __tmp_250: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_250.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let __tmp_251: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_251.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let __tmp_252: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_252.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let __tmp_253: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_253.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let __tmp_254: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_254.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let __tmp_255: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_255.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let __tmp_256: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_256.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let __tmp_257: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_257.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let __tmp_258: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_258.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let __tmp_259: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            (a).clone()
                        })));
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_259.as_pointer() })
                };
                unimplemented!();
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
        ({ (*check.borrow()).spec::<fn()>(2).call() });
        ({ (*check.borrow()).spec::<fn()>(3).call() });
        ({ (*check.borrow()).spec::<fn()>(4).call() });
        ({ (*check.borrow()).spec::<fn()>(5).call() });
        ({ (*check.borrow()).spec::<fn()>(6).call() });
        ({ (*check.borrow()).spec::<fn()>(7).call() });
        ({ (*check.borrow()).spec::<fn()>(8).call() });
        ({ (*check.borrow()).spec::<fn()>(9).call() });
        ({ (*check.borrow()).spec::<fn()>(10).call() });
        ({ (*check.borrow()).spec::<fn()>(11).call() });
        ({ (*check.borrow()).spec::<fn()>(12).call() });
        ({ (*check.borrow()).spec::<fn()>(13).call() });
        ({ (*check.borrow()).spec::<fn()>(14).call() });
        ({ (*check.borrow()).spec::<fn()>(15).call() });
    }
    {
        let first: Value<contiguous_iterator_int_ptr_> =
            Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                (a.as_pointer() as Ptr<()>)
            })));
        let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
            let __tmp_260: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    (a.as_pointer() as Ptr<()>)
                })));
            sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_260.as_pointer() })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let first: Value<contiguous_iterator_int_ptr_> =
            Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                (a.as_pointer() as Ptr<()>)
            })));
        let mut last: sized_sentinel_contiguous_iterator_int_ptr__ = {
            let __tmp_261: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    (a.as_pointer() as Ptr<()>).offset((3_()) as isize)
                })));
            sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_261.as_pointer() })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_sized_56() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ = {
                    let __tmp_262: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_int_ptr_::new_1({ __tmp_262.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ = {
                    let __tmp_263: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_int_ptr_::new_1({ __tmp_263.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ = {
                    let __tmp_264: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_int_ptr_::new_1({ __tmp_264.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ = {
                    let __tmp_265: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_int_ptr_::new_1({ __tmp_265.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ = {
                    let __tmp_266: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_int_ptr_::new_1({ __tmp_266.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ = {
                    let __tmp_267: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_int_ptr_::new_1({ __tmp_267.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ = {
                    let __tmp_268: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_int_ptr_::new_1({ __tmp_268.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ = {
                    let __tmp_269: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_int_ptr_::new_1({ __tmp_269.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ = {
                    let __tmp_270: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_int_ptr_::new_1({ __tmp_270.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ = {
                    let __tmp_271: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_int_ptr_::new_1({ __tmp_271.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ = {
                    let __tmp_272: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_int_ptr_::new_1({ __tmp_272.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ = {
                    let __tmp_273: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_int_ptr_::new_1({ __tmp_273.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ = {
                    let __tmp_274: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_int_ptr_::new_1({ __tmp_274.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ = {
                    let __tmp_275: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_int_ptr_::new_1({ __tmp_275.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ = {
                    let __tmp_276: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_int_ptr_::new_1({ __tmp_276.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_int_ptr_ = {
                    let __tmp_277: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_int_ptr_::new_1({ __tmp_277.as_pointer() })
                };
                unimplemented!();
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
        ({ (*check.borrow()).spec::<fn()>(2).call() });
        ({ (*check.borrow()).spec::<fn()>(3).call() });
        ({ (*check.borrow()).spec::<fn()>(4).call() });
        ({ (*check.borrow()).spec::<fn()>(5).call() });
        ({ (*check.borrow()).spec::<fn()>(6).call() });
        ({ (*check.borrow()).spec::<fn()>(7).call() });
        ({ (*check.borrow()).spec::<fn()>(8).call() });
        ({ (*check.borrow()).spec::<fn()>(9).call() });
        ({ (*check.borrow()).spec::<fn()>(10).call() });
        ({ (*check.borrow()).spec::<fn()>(11).call() });
        ({ (*check.borrow()).spec::<fn()>(12).call() });
        ({ (*check.borrow()).spec::<fn()>(13).call() });
        ({ (*check.borrow()).spec::<fn()>(14).call() });
        ({ (*check.borrow()).spec::<fn()>(15).call() });
    }
    {
        let first: Value<Ptr<()>> = Rc::new(RefCell::new((a.as_pointer() as Ptr<()>)));
        let mut last: sized_sentinel_int_ptr_ = {
            let __tmp_278: Value<Ptr<()>> = Rc::new(RefCell::new((a.as_pointer() as Ptr<()>)));
            sized_sentinel_int_ptr_::new_1({ __tmp_278.as_pointer() })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let first: Value<Ptr<()>> = Rc::new(RefCell::new((a.as_pointer() as Ptr<()>)));
        let mut last: sized_sentinel_int_ptr_ = {
            let __tmp_279: Value<Ptr<()>> = Rc::new(RefCell::new(
                (a.as_pointer() as Ptr<()>).offset((3_()) as isize),
            ));
            sized_sentinel_int_ptr_::new_1({ __tmp_279.as_pointer() })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_sized_57() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ = {
                    let __tmp_280: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_const_int_ptr_::new_1({ __tmp_280.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ = {
                    let __tmp_281: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_const_int_ptr_::new_1({ __tmp_281.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ = {
                    let __tmp_282: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_const_int_ptr_::new_1({ __tmp_282.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ = {
                    let __tmp_283: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_const_int_ptr_::new_1({ __tmp_283.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ = {
                    let __tmp_284: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_const_int_ptr_::new_1({ __tmp_284.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ = {
                    let __tmp_285: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_const_int_ptr_::new_1({ __tmp_285.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ = {
                    let __tmp_286: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_const_int_ptr_::new_1({ __tmp_286.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ = {
                    let __tmp_287: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_const_int_ptr_::new_1({ __tmp_287.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ = {
                    let __tmp_288: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_const_int_ptr_::new_1({ __tmp_288.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ = {
                    let __tmp_289: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_const_int_ptr_::new_1({ __tmp_289.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ = {
                    let __tmp_290: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_const_int_ptr_::new_1({ __tmp_290.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ = {
                    let __tmp_291: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_const_int_ptr_::new_1({ __tmp_291.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ = {
                    let __tmp_292: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_const_int_ptr_::new_1({ __tmp_292.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ = {
                    let __tmp_293: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_const_int_ptr_::new_1({ __tmp_293.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ = {
                    let __tmp_294: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_const_int_ptr_::new_1({ __tmp_294.as_pointer() })
                };
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: sized_sentinel_const_int_ptr_ = {
                    let __tmp_295: Value<Ptr<()>> = Rc::new(RefCell::new((a).clone()));
                    sized_sentinel_const_int_ptr_::new_1({ __tmp_295.as_pointer() })
                };
                unimplemented!();
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
        ({ (*check.borrow()).spec::<fn()>(2).call() });
        ({ (*check.borrow()).spec::<fn()>(3).call() });
        ({ (*check.borrow()).spec::<fn()>(4).call() });
        ({ (*check.borrow()).spec::<fn()>(5).call() });
        ({ (*check.borrow()).spec::<fn()>(6).call() });
        ({ (*check.borrow()).spec::<fn()>(7).call() });
        ({ (*check.borrow()).spec::<fn()>(8).call() });
        ({ (*check.borrow()).spec::<fn()>(9).call() });
        ({ (*check.borrow()).spec::<fn()>(10).call() });
        ({ (*check.borrow()).spec::<fn()>(11).call() });
        ({ (*check.borrow()).spec::<fn()>(12).call() });
        ({ (*check.borrow()).spec::<fn()>(13).call() });
        ({ (*check.borrow()).spec::<fn()>(14).call() });
        ({ (*check.borrow()).spec::<fn()>(15).call() });
    }
    {
        let first: Value<Ptr<()>> = Rc::new(RefCell::new((a.as_pointer() as Ptr<()>)));
        let mut last: sized_sentinel_const_int_ptr_ = {
            let __tmp_296: Value<Ptr<()>> = Rc::new(RefCell::new((a.as_pointer() as Ptr<()>)));
            sized_sentinel_const_int_ptr_::new_1({ __tmp_296.as_pointer() })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let first: Value<Ptr<()>> = Rc::new(RefCell::new((a.as_pointer() as Ptr<()>)));
        let mut last: sized_sentinel_const_int_ptr_ = {
            let __tmp_297: Value<Ptr<()>> = Rc::new(RefCell::new(
                (a.as_pointer() as Ptr<()>).offset((3_()) as isize),
            ));
            sized_sentinel_const_int_ptr_::new_1({ __tmp_297.as_pointer() })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_sized_58() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
        ({ (*check.borrow()).spec::<fn()>(2).call() });
        ({ (*check.borrow()).spec::<fn()>(3).call() });
        ({ (*check.borrow()).spec::<fn()>(4).call() });
        ({ (*check.borrow()).spec::<fn()>(5).call() });
        ({ (*check.borrow()).spec::<fn()>(6).call() });
        ({ (*check.borrow()).spec::<fn()>(7).call() });
        ({ (*check.borrow()).spec::<fn()>(8).call() });
        ({ (*check.borrow()).spec::<fn()>(9).call() });
        ({ (*check.borrow()).spec::<fn()>(10).call() });
        ({ (*check.borrow()).spec::<fn()>(11).call() });
        ({ (*check.borrow()).spec::<fn()>(12).call() });
        ({ (*check.borrow()).spec::<fn()>(13).call() });
        ({ (*check.borrow()).spec::<fn()>(14).call() });
        ({ (*check.borrow()).spec::<fn()>(15).call() });
    }
    {
        let first: Value<Ptr<()>> = Rc::new(RefCell::new((a.as_pointer() as Ptr<()>)));
        let mut last: Ptr<()> = (a.as_pointer() as Ptr<()>);
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let first: Value<Ptr<()>> = Rc::new(RefCell::new((a.as_pointer() as Ptr<()>)));
        let mut last: Ptr<()> = (a.as_pointer() as Ptr<()>).offset((3_()) as isize);
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_sized_59() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            },
            || {
                let mut first: Ptr<()> = (a).offset((3_()) as isize);
                let mut last: Ptr<()> = (a).clone();
                unimplemented!();
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
        ({ (*check.borrow()).spec::<fn()>(2).call() });
        ({ (*check.borrow()).spec::<fn()>(3).call() });
        ({ (*check.borrow()).spec::<fn()>(4).call() });
        ({ (*check.borrow()).spec::<fn()>(5).call() });
        ({ (*check.borrow()).spec::<fn()>(6).call() });
        ({ (*check.borrow()).spec::<fn()>(7).call() });
        ({ (*check.borrow()).spec::<fn()>(8).call() });
        ({ (*check.borrow()).spec::<fn()>(9).call() });
        ({ (*check.borrow()).spec::<fn()>(10).call() });
        ({ (*check.borrow()).spec::<fn()>(11).call() });
        ({ (*check.borrow()).spec::<fn()>(12).call() });
        ({ (*check.borrow()).spec::<fn()>(13).call() });
        ({ (*check.borrow()).spec::<fn()>(14).call() });
        ({ (*check.borrow()).spec::<fn()>(15).call() });
    }
    {
        let first: Value<Ptr<()>> = Rc::new(RefCell::new((a.as_pointer() as Ptr<()>)));
        let mut last: Ptr<()> = (a.as_pointer() as Ptr<()>);
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let first: Value<Ptr<()>> = Rc::new(RefCell::new((a.as_pointer() as Ptr<()>)));
        let mut last: Ptr<()> = (a.as_pointer() as Ptr<()>).offset((3_()) as isize);
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_sized_60() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
        ({ (*check.borrow()).spec::<fn()>(2).call() });
        ({ (*check.borrow()).spec::<fn()>(3).call() });
        ({ (*check.borrow()).spec::<fn()>(4).call() });
        ({ (*check.borrow()).spec::<fn()>(5).call() });
        ({ (*check.borrow()).spec::<fn()>(6).call() });
        ({ (*check.borrow()).spec::<fn()>(7).call() });
        ({ (*check.borrow()).spec::<fn()>(8).call() });
        ({ (*check.borrow()).spec::<fn()>(9).call() });
        ({ (*check.borrow()).spec::<fn()>(10).call() });
        ({ (*check.borrow()).spec::<fn()>(11).call() });
        ({ (*check.borrow()).spec::<fn()>(12).call() });
        ({ (*check.borrow()).spec::<fn()>(13).call() });
        ({ (*check.borrow()).spec::<fn()>(14).call() });
        ({ (*check.borrow()).spec::<fn()>(15).call() });
    }
    {
        let first: Value<random_access_iterator_int_ptr_> =
            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                (a.as_pointer() as Ptr<()>)
            })));
        let mut last: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ (a.as_pointer() as Ptr<()>) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let first: Value<random_access_iterator_int_ptr_> =
            Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                (a.as_pointer() as Ptr<()>)
            })));
        let mut last: random_access_iterator_int_ptr_ = random_access_iterator_int_ptr_::new_1({
            (a.as_pointer() as Ptr<()>).offset((3_()) as isize)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_sized_61() {
    const _: () = assert!(true, "std::sized_sentinel_for<Sent, It>");;
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            },
            || {
                let mut first: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).offset((3_()) as isize) });
                let mut last: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ (a).clone() });
                unimplemented!();
            }
        )));
        ({ (*check.borrow()).spec::<fn()>(0).call() });
        ({ (*check.borrow()).spec::<fn()>(1).call() });
        ({ (*check.borrow()).spec::<fn()>(2).call() });
        ({ (*check.borrow()).spec::<fn()>(3).call() });
        ({ (*check.borrow()).spec::<fn()>(4).call() });
        ({ (*check.borrow()).spec::<fn()>(5).call() });
        ({ (*check.borrow()).spec::<fn()>(6).call() });
        ({ (*check.borrow()).spec::<fn()>(7).call() });
        ({ (*check.borrow()).spec::<fn()>(8).call() });
        ({ (*check.borrow()).spec::<fn()>(9).call() });
        ({ (*check.borrow()).spec::<fn()>(10).call() });
        ({ (*check.borrow()).spec::<fn()>(11).call() });
        ({ (*check.borrow()).spec::<fn()>(12).call() });
        ({ (*check.borrow()).spec::<fn()>(13).call() });
        ({ (*check.borrow()).spec::<fn()>(14).call() });
        ({ (*check.borrow()).spec::<fn()>(15).call() });
    }
    {
        let first: Value<contiguous_iterator_int_ptr_> =
            Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                (a.as_pointer() as Ptr<()>)
            })));
        let mut last: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ (a.as_pointer() as Ptr<()>) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let first: Value<contiguous_iterator_int_ptr_> =
            Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                (a.as_pointer() as Ptr<()>)
            })));
        let mut last: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
            (a.as_pointer() as Ptr<()>).offset((3_()) as isize)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct StrideCounter {
    #[offset(0)]
    #[byte_size(8)]
    pub it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub inc_: Ptr<()>,
}
impl StrideCounter {
    pub fn new_1(mut it: Ptr<()>, mut inc: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            inc_: (inc).clone(),
        }
    }
}
const _: () = assert!(true, "std::forward_iterator<StrideCounter>");
const _: () = assert!(
    !(false),
    "!std::sized_sentinel_for<StrideCounter, StrideCounter>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct SizedStrideCounter {
    #[offset(0)]
    #[byte_size(8)]
    pub it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub minus_: Ptr<()>,
}
impl SizedStrideCounter {
    pub fn new_1(mut it: Ptr<()>, mut minus: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            minus_: (minus).clone(),
        }
    }
}
const _: () = assert!(true, "std::forward_iterator<SizedStrideCounter>");
const _: () = assert!(
    true,
    "std::sized_sentinel_for<SizedStrideCounter, SizedStrideCounter>"
);
pub fn test_stride_counting_62() {
    {
        let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
        let inc: Value<()> = Rc::new(RefCell::new(0_()));
        let mut first: StrideCounter =
            StrideCounter::new_1({ (a.as_pointer() as Ptr<()>) }, { (inc.as_pointer()) });
        let mut last: StrideCounter =
            StrideCounter::new_1({ (a.as_pointer() as Ptr<()>).offset((3_()) as isize) }, {
                Ptr::<()>::null()
            });
        let mut result: () =
            ({ (*distance_63.with(Value::clone).borrow_mut())((first).clone(), (last).clone()) });
        unimplemented!();
        unimplemented!();
    }
    {
        let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
        let minus: Value<()> = Rc::new(RefCell::new(0_()));
        let first: Value<SizedStrideCounter> = Rc::new(RefCell::new(SizedStrideCounter::new_1(
            { (a.as_pointer() as Ptr<()>) },
            { (minus.as_pointer()) },
        )));
        let mut last: SizedStrideCounter =
            SizedStrideCounter::new_1({ (a.as_pointer() as Ptr<()>).offset((3_()) as isize) }, {
                Ptr::<()>::null()
            });
        let mut result: () = ({
            (*distance_63.with(Value::clone).borrow_mut())(first.as_pointer(), (last).clone())
        });
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_deque_64() {
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let view: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut n: () = 0_();
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        n = { (n).wrapping_add(i) };
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
}
pub fn test_65() -> () {
    {
        let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    ({ test_unsized_39() });
    ({ test_unsized_40() });
    ({ test_unsized_41() });
    ({ test_unsized_42() });
    ({ test_unsized_43() });
    ({ test_unsized_44() });
    ({ test_unsized_45() });
    ({ test_unsized_46() });
    ({ test_unsized_47() });
    ({ test_unsized_48() });
    ({ test_sized_49() });
    ({ test_sized_50() });
    ({ test_sized_51() });
    ({ test_sized_52() });
    ({ test_sized_53() });
    ({ test_sized_54() });
    ({ test_sized_55() });
    ({ test_sized_56() });
    ({ test_sized_57() });
    ({ test_sized_58() });
    ({ test_sized_59() });
    ({ test_sized_60() });
    ({ test_sized_61() });
    {
        const _: () = assert!(true, "!std::is_copy_constructible_v<It>");;
        const _: () = assert!(!(false), "!std::sentinel_for<It, It>");;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), It&, It&>"
        );;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), It&, It&&>"
        );;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), It&&, It&>"
        );;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), It&&, It&&>"
        );;
    }
    {
        const _: () = assert!(
            true,
            "std::sentinel_for<Sent, It> && !std::sized_sentinel_for<Sent, It>"
        );;
        let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
        let mut last: sentinel_wrapper_cpp20_input_iterator_int_ptr__ = {
            let __tmp_298: Value<cpp20_input_iterator_int_ptr_> =
                Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                    (a.as_pointer() as Ptr<()>).offset((3_()) as isize)
                })));
            sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ __tmp_298.as_pointer() })
        };
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), It&, Sent&>"
        );;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), It&, Sent&&>"
        );;
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(!(false), "!std::sentinel_for<It, It>");;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), It&, It&>"
        );;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), It&, It&&>"
        );;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), It&&, It&>"
        );;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), It&&, It&&>"
        );;
    }
    const _: () = assert!(
        true,
        "!std::is_invocable_v<decltype(std::ranges::distance), int, int>"
    );;
    const _: () = assert!(
        true,
        "!std::is_invocable_v<decltype(std::ranges::distance), int*, int>"
    );;
    const _: () = assert!(
        true,
        "!std::is_invocable_v<decltype(std::ranges::distance), int, int*>"
    );;
    const _: () = assert!(
        true,
        "!std::is_invocable_v<decltype(std::ranges::distance), int*, char*>"
    );;
    {
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let view: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut n: () = 0_();
        let mut i: () = 0_();
        'loop_: while (i < 10_()) {
            n = { (n).wrapping_add(i) };
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
    {
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let view: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        ({ test_deque_64() });
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_65() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait SizedStrideCounterImpl {
    fn operator_inc_3(&self) -> Ptr<SizedStrideCounter> {
        unimplemented!()
    }
    fn operator_post_inc_4(&self, mut _a0: ()) -> SizedStrideCounter {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_eq(&self, mut _a0: SizedStrideCounter) -> () {
        unimplemented!()
    }
    fn operator_sub(&self, rhs: SizedStrideCounter) -> ();
}
impl SizedStrideCounterImpl for Ptr<SizedStrideCounter> {
    fn operator_sub(&self, mut rhs: SizedStrideCounter) -> () {
        {
            (*self)
                .with(|__s| __s.minus_.clone())
                .with_mut(|__v| *__v = *__v + 1_())
        };
        return ((*self).with(|__s| __s.it_.clone()) - (rhs.it_).clone()) as ();
    }
}
pub trait StrideCounterImpl {
    fn operator_inc_3(&self) -> Ptr<StrideCounter>;
    fn operator_post_inc_4(&self, mut _a0: ()) -> StrideCounter {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_eq(&self, mut _a0: StrideCounter) -> () {
        unimplemented!()
    }
}
impl StrideCounterImpl for Ptr<StrideCounter> {
    fn operator_inc_3(&self) -> Ptr<StrideCounter> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        {
            (*self)
                .with(|__s| __s.inc_.clone())
                .with_mut(|__v| *__v = *__v + 1_())
        };
        return (*self).clone();
    }
}
pub trait adl_IteratorImpl {
    fn iter_moves(&self) -> ();
    fn iter_swaps(&self) -> ();
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_index(&self, n: ()) -> Ptr<()>;
    fn operator_sub_10(&self, n: ()) -> adl_Iterator;
    fn operator_sub_11(&self, rhs: adl_Iterator) -> ();
    fn operator_add_assign(&self, n: ()) -> Ptr<adl_Iterator>;
    fn operator_sub_assign(&self, n: ()) -> Ptr<adl_Iterator>;
    fn operator_inc_14(&self) -> Ptr<adl_Iterator>;
    fn operator_post_inc_15(&self, _a0: ()) -> adl_Iterator;
    fn operator_dec_16(&self) -> Ptr<adl_Iterator>;
    fn operator_post_dec_17(&self, _a0: ()) -> adl_Iterator;
}
impl adl_IteratorImpl for Ptr<adl_Iterator> {
    fn iter_moves(&self) -> () {
        unimplemented!();
        return ((*self).with(|__s| __s.iter_moves_.clone()).read());
    }
    fn iter_swaps(&self) -> () {
        unimplemented!();
        return ((*self).with(|__s| __s.iter_swaps_.clone()).read());
    }
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
    fn operator_sub_10(&self, mut n: ()) -> adl_Iterator {
        return adl_Iterator::new_1(
            { (*self).with(|__s| __s.ptr_.clone()).offset(-((n) as isize)) },
            { (*self).with(|__s| __s.iter_moves_.clone()) },
            { (*self).with(|__s| __s.iter_swaps_.clone()) },
        );
    }
    fn operator_sub_11(&self, mut rhs: adl_Iterator) -> () {
        return (((*self).with(|__s| __s.ptr_.clone()) - (rhs.ptr_).clone()) as () as ());
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<adl_Iterator> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<adl_Iterator> {
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
    fn operator_post_inc_15(&self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return prev;
    }
    fn operator_dec_16(&self) -> Ptr<adl_Iterator> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_post_dec_17(&self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return prev;
    }
}
pub trait bidirectional_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_int_ptr_>;
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_8(&self, mut _a0: ()) -> bidirectional_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_9(&self, mut _a0: ()) -> bidirectional_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl bidirectional_iterator_int_ptr_Impl for Ptr<bidirectional_iterator_int_ptr_> {
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait contiguous_iterator_int_ptr_Impl {
    fn base(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_arrow(&self) -> () {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_int_ptr_>;
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: ()) -> contiguous_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: ()) -> contiguous_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_int_ptr_> {
        unimplemented!()
    }
}
impl contiguous_iterator_int_ptr_Impl for Ptr<contiguous_iterator_int_ptr_> {
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait cpp17_input_iterator_int_ptr__int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_>;
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp17_input_iterator_int_ptr__int_ptr_ {
        unimplemented!()
    }
}
impl cpp17_input_iterator_int_ptr__int_ptr_Impl for Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait cpp17_output_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_output_iterator_int_ptr_>;
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp17_output_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl cpp17_output_iterator_int_ptr_Impl for Ptr<cpp17_output_iterator_int_ptr_> {
    fn operator_inc_5(&self) -> Ptr<cpp17_output_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait cpp20_input_iterator_int_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_input_iterator_int_ptr_>,
    ) -> Ptr<cpp20_input_iterator_int_ptr_>;
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_inc_5(&self) -> Ptr<cpp20_input_iterator_int_ptr_>;
    fn operator_post_inc_6(&self, mut _a0: ()) {
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
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_5(&self) -> Ptr<cpp20_input_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait cpp20_output_iterator_int_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_output_iterator_int_ptr_>,
    ) -> Ptr<cpp20_output_iterator_int_ptr_>;
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_inc_5(&self) -> Ptr<cpp20_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp20_output_iterator_int_ptr_ {
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
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait cpp20_random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_index(&self, n: ()) -> Ptr<()>;
    fn operator_inc_7(&self) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_dec_8(&self) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: ()) -> cpp20_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> cpp20_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
}
impl cpp20_random_access_iterator_int_ptr_Impl for Ptr<cpp20_random_access_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone()).offset((n) as isize);
    }
}
pub trait forward_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<forward_iterator_int_ptr_>;
    fn operator_post_inc_7(&self, mut _a0: ()) -> forward_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl forward_iterator_int_ptr_Impl for Ptr<forward_iterator_int_ptr_> {
    fn operator_inc_6(&self) -> Ptr<forward_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait forward_sized_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_4(&self) -> Ptr<forward_sized_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: ()) -> forward_sized_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl forward_sized_iterator_int_ptr_Impl for Ptr<forward_sized_iterator_int_ptr_> {}
pub trait random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: ()) -> random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<random_access_iterator_int_ptr_> {
        unimplemented!()
    }
}
impl random_access_iterator_int_ptr_Impl for Ptr<random_access_iterator_int_ptr_> {
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait rvalue_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn operator_inc_4(&self) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_dec_6(&self) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_dec_7(&self, mut _a0: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add(&self, mut n: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_sub_9(&self, mut n: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_sub_10(&self, other: Ptr<rvalue_iterator_int_ptr_>) -> () {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn operator_cmp(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> ();
    fn operator_eq(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> ();
}
impl rvalue_iterator_int_ptr_Impl for Ptr<rvalue_iterator_int_ptr_> {
    fn operator_cmp(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.it_.clone())),
                &(_a0.with(|__s| __s.it_.clone())),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> () {
        return ({ (*self).with(|__s| __s.it_.clone()) } == { _a0.with(|__s| __s.it_.clone()) });
    }
}
pub trait sentinel_wrapper_bidirectional_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<bidirectional_iterator_int_ptr_>) -> ();
}
impl sentinel_wrapper_bidirectional_iterator_int_ptr__Impl
    for Ptr<sentinel_wrapper_bidirectional_iterator_int_ptr__>
{
    fn operator_eq(&self, other: Ptr<bidirectional_iterator_int_ptr_>) -> () {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_6((other).clone()) }) });
    }
}
pub trait sentinel_wrapper_const_int_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> ();
}
impl sentinel_wrapper_const_int_ptr_Impl for Ptr<sentinel_wrapper_const_int_ptr_> {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> () {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_12((other.read())) }) });
    }
}
pub trait sentinel_wrapper_contiguous_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<contiguous_iterator_int_ptr_>) -> ();
}
impl sentinel_wrapper_contiguous_iterator_int_ptr__Impl
    for Ptr<sentinel_wrapper_contiguous_iterator_int_ptr__>
{
    fn operator_eq(&self, other: Ptr<contiguous_iterator_int_ptr_>) -> () {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_10((other).clone()) }) });
    }
}
pub trait sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>) -> ();
}
impl sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__Impl
    for Ptr<sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__>
{
    fn operator_eq(&self, other: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>) -> () {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_2((other).clone()) }) });
    }
}
pub trait sentinel_wrapper_cpp17_output_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<cpp17_output_iterator_int_ptr_>) -> ();
}
impl sentinel_wrapper_cpp17_output_iterator_int_ptr__Impl
    for Ptr<sentinel_wrapper_cpp17_output_iterator_int_ptr__>
{
    fn operator_eq(&self, other: Ptr<cpp17_output_iterator_int_ptr_>) -> () {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_1((other).clone()) }) });
    }
}
pub trait sentinel_wrapper_cpp20_input_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<cpp20_input_iterator_int_ptr_>) -> ();
}
impl sentinel_wrapper_cpp20_input_iterator_int_ptr__Impl
    for Ptr<sentinel_wrapper_cpp20_input_iterator_int_ptr__>
{
    fn operator_eq(&self, other: Ptr<cpp20_input_iterator_int_ptr_>) -> () {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_13((other).clone()) }) });
    }
}
pub trait sentinel_wrapper_forward_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<forward_iterator_int_ptr_>) -> ();
}
impl sentinel_wrapper_forward_iterator_int_ptr__Impl
    for Ptr<sentinel_wrapper_forward_iterator_int_ptr__>
{
    fn operator_eq(&self, other: Ptr<forward_iterator_int_ptr_>) -> () {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_4((other).clone()) }) });
    }
}
pub trait sentinel_wrapper_int_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> ();
}
impl sentinel_wrapper_int_ptr_Impl for Ptr<sentinel_wrapper_int_ptr_> {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> () {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_11((other.read())) }) });
    }
}
pub trait sentinel_wrapper_random_access_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<random_access_iterator_int_ptr_>) -> ();
}
impl sentinel_wrapper_random_access_iterator_int_ptr__Impl
    for Ptr<sentinel_wrapper_random_access_iterator_int_ptr__>
{
    fn operator_eq(&self, other: Ptr<random_access_iterator_int_ptr_>) -> () {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_8((other).clone()) }) });
    }
}
pub trait sized_sentinel_bidirectional_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<bidirectional_iterator_int_ptr_>) -> () {
        unimplemented!()
    }
}
impl sized_sentinel_bidirectional_iterator_int_ptr__Impl
    for Ptr<sized_sentinel_bidirectional_iterator_int_ptr__>
{
}
pub trait sized_sentinel_const_int_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> () {
        unimplemented!()
    }
}
impl sized_sentinel_const_int_ptr_Impl for Ptr<sized_sentinel_const_int_ptr_> {}
pub trait sized_sentinel_contiguous_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<contiguous_iterator_int_ptr_>) -> () {
        unimplemented!()
    }
}
impl sized_sentinel_contiguous_iterator_int_ptr__Impl
    for Ptr<sized_sentinel_contiguous_iterator_int_ptr__>
{
}
pub trait sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>) -> () {
        unimplemented!()
    }
}
impl sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__Impl
    for Ptr<sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__>
{
}
pub trait sized_sentinel_cpp17_output_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<cpp17_output_iterator_int_ptr_>) -> () {
        unimplemented!()
    }
}
impl sized_sentinel_cpp17_output_iterator_int_ptr__Impl
    for Ptr<sized_sentinel_cpp17_output_iterator_int_ptr__>
{
}
pub trait sized_sentinel_cpp20_input_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<cpp20_input_iterator_int_ptr_>) -> () {
        unimplemented!()
    }
}
impl sized_sentinel_cpp20_input_iterator_int_ptr__Impl
    for Ptr<sized_sentinel_cpp20_input_iterator_int_ptr__>
{
}
pub trait sized_sentinel_forward_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<forward_iterator_int_ptr_>) -> () {
        unimplemented!()
    }
}
impl sized_sentinel_forward_iterator_int_ptr__Impl
    for Ptr<sized_sentinel_forward_iterator_int_ptr__>
{
}
pub trait sized_sentinel_int_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> () {
        unimplemented!()
    }
}
impl sized_sentinel_int_ptr_Impl for Ptr<sized_sentinel_int_ptr_> {}
pub trait sized_sentinel_random_access_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<random_access_iterator_int_ptr_>) -> () {
        unimplemented!()
    }
}
impl sized_sentinel_random_access_iterator_int_ptr__Impl
    for Ptr<sized_sentinel_random_access_iterator_int_ptr__>
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
        unimplemented!();
        field!(other, moved_from_).write(true);
        field!((*self), moved_from_).write(false);
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
