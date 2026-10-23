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
pub fn base_3(i: Ptr<forward_iterator_int_ptr_>) -> Ptr<()> {
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
pub fn base_4(i: Ptr<bidirectional_iterator_int_ptr_>) -> Ptr<()> {
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
pub fn base_5(i: Ptr<random_access_iterator_int_ptr_>) -> Ptr<()> {
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
pub fn base_6(i: Ptr<contiguous_iterator_int_ptr_>) -> Ptr<()> {
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
pub fn base_7(i: Ptr<()>) -> Ptr<()> {
    let i: Value<Ptr<()>> = Rc::new(RefCell::new(i));
    return (*i.borrow()).clone();
}
pub fn base_8(i: Ptr<cpp20_input_iterator_int_ptr_>) -> Ptr<()> {
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
pub struct sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
    pub fn new_1(it: Ptr<cpp20_input_iterator_int_ptr_>) -> Self {
        Self {
            base_: ({ base_8((it).clone()) }),
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
            base_: ({ base_3((it).clone()) }),
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
            base_: ({ base_4((it).clone()) }),
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
            base_: ({ base_5((it).clone()) }),
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
            base_: ({ base_6((it).clone()) }),
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
            base_: ({ base_7((it.read())) }),
        }
    }
}
pub fn operator_sub_9(
    s: Ptr<sized_sentinel_cpp20_input_iterator_int_ptr__>,
    i: Ptr<cpp20_input_iterator_int_ptr_>,
) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_8((i).clone()) })) as ();
}
pub fn operator_sub_10(
    i: Ptr<cpp20_input_iterator_int_ptr_>,
    s: Ptr<sized_sentinel_cpp20_input_iterator_int_ptr__>,
) -> () {
    return (({ base_8((i).clone()) }) - s.with(|__s| __s.base_.clone())) as ();
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
            base_: ({ base_8((it).clone()) }),
        }
    }
}
pub fn operator_sub_11(
    s: Ptr<sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__>,
    i: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>,
) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_2((i).clone()) })) as ();
}
pub fn operator_sub_12(
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
pub fn operator_sub_13(
    s: Ptr<sized_sentinel_cpp17_output_iterator_int_ptr__>,
    i: Ptr<cpp17_output_iterator_int_ptr_>,
) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_1((i).clone()) })) as ();
}
pub fn operator_sub_14(
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
pub fn operator_sub_15(
    s: Ptr<sized_sentinel_forward_iterator_int_ptr__>,
    i: Ptr<forward_iterator_int_ptr_>,
) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_3((i).clone()) })) as ();
}
pub fn operator_sub_16(
    i: Ptr<forward_iterator_int_ptr_>,
    s: Ptr<sized_sentinel_forward_iterator_int_ptr__>,
) -> () {
    return (({ base_3((i).clone()) }) - s.with(|__s| __s.base_.clone())) as ();
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
            base_: ({ base_3((it).clone()) }),
        }
    }
}
pub fn operator_sub_17(
    s: Ptr<sized_sentinel_bidirectional_iterator_int_ptr__>,
    i: Ptr<bidirectional_iterator_int_ptr_>,
) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_4((i).clone()) })) as ();
}
pub fn operator_sub_18(
    i: Ptr<bidirectional_iterator_int_ptr_>,
    s: Ptr<sized_sentinel_bidirectional_iterator_int_ptr__>,
) -> () {
    return (({ base_4((i).clone()) }) - s.with(|__s| __s.base_.clone())) as ();
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
            base_: ({ base_4((it).clone()) }),
        }
    }
}
pub fn operator_sub_19(
    s: Ptr<sized_sentinel_random_access_iterator_int_ptr__>,
    i: Ptr<random_access_iterator_int_ptr_>,
) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_5((i).clone()) })) as ();
}
pub fn operator_sub_20(
    i: Ptr<random_access_iterator_int_ptr_>,
    s: Ptr<sized_sentinel_random_access_iterator_int_ptr__>,
) -> () {
    return (({ base_5((i).clone()) }) - s.with(|__s| __s.base_.clone())) as ();
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
            base_: ({ base_5((it).clone()) }),
        }
    }
}
pub fn operator_sub_21(
    s: Ptr<sized_sentinel_contiguous_iterator_int_ptr__>,
    i: Ptr<contiguous_iterator_int_ptr_>,
) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_6((i).clone()) })) as ();
}
pub fn operator_sub_22(
    i: Ptr<contiguous_iterator_int_ptr_>,
    s: Ptr<sized_sentinel_contiguous_iterator_int_ptr__>,
) -> () {
    return (({ base_6((i).clone()) }) - s.with(|__s| __s.base_.clone())) as ();
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
            base_: ({ base_6((it).clone()) }),
        }
    }
}
pub fn operator_sub_23(s: Ptr<sized_sentinel_int_ptr_>, i: Ptr<Ptr<()>>) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_7((i.read())) })) as ();
}
pub fn operator_sub_24(i: Ptr<Ptr<()>>, s: Ptr<sized_sentinel_int_ptr_>) -> () {
    return (({ base_7((i.read())) }) - s.with(|__s| __s.base_.clone())) as ();
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
            base_: ({ base_7((it.read())) }),
        }
    }
}
pub fn operator_eq_25(
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
            operator_eq_25(
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
pub fn operator_add_26(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_27(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_26(_i, n)
    });
}
pub fn iter_swap_28(a: adl_Iterator, b: adl_Iterator) {
    let a: Value<adl_Iterator> = Rc::new(RefCell::new(a));
    let b: Value<adl_Iterator> = Rc::new(RefCell::new(b));
    ({
        let ___x: Ptr<Ptr<()>> = field_ptr!(a.as_pointer(), ptr_);
        let ___y: Ptr<Ptr<()>> = field_ptr!(b.as_pointer(), ptr_);
        swap_29(___x, ___y)
    });
    if !({ (*a.borrow()).iter_swaps_.clone() }).is_null() {
        { (*a.borrow()).iter_swaps_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_30(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) }).reinterpret_cast::<()>();
}
pub fn operator_eq_31(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_32(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
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
            operator_cmp_32(
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
            operator_eq_31(
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
pub fn test_ordinary_33() {
    let r: Value<R_34> = Rc::new(RefCell::new(<R_34>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub fn test_ordinary_35() {
    let r: Value<R_36> = Rc::new(RefCell::new(<R_36>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub fn test_ordinary_37() {
    let r: Value<R_38> = Rc::new(RefCell::new(<R_38>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub fn test_ordinary_39() {
    let r: Value<R_40> = Rc::new(RefCell::new(<R_40>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub fn test_ordinary_41() {
    let r: Value<R_42> = Rc::new(RefCell::new(<R_42>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub fn test_ordinary_43() {
    let r: Value<R_44> = Rc::new(RefCell::new(<R_44>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub fn test_ordinary_45() {
    let r: Value<R_46> = Rc::new(RefCell::new(<R_46>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub fn test_ordinary_47() {
    let r: Value<R_48> = Rc::new(RefCell::new(<R_48>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub fn test_ordinary_49() {
    let r: Value<R_50> = Rc::new(RefCell::new(<R_50>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub fn test_ordinary_51() {
    let r: Value<R_52> = Rc::new(RefCell::new(<R_52>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub fn test_ordinary_53() {
    let r: Value<R_54> = Rc::new(RefCell::new(<R_54>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub fn test_ordinary_55() {
    let r: Value<R_56> = Rc::new(RefCell::new(<R_56>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub fn test_ordinary_57() {
    let r: Value<R_58> = Rc::new(RefCell::new(<R_58>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub fn test_ordinary_59() {
    let r: Value<R_60> = Rc::new(RefCell::new(<R_60>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub fn test_ordinary_61() {
    let r: Value<R_62> = Rc::new(RefCell::new(<R_62>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub fn test_ordinary_63() {
    let r: Value<R_64> = Rc::new(RefCell::new(<R_64>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
pub fn test_ordinary_65() {
    let r: Value<R_66> = Rc::new(RefCell::new(<R_66>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct R_34 {
    #[offset(0)]
    #[byte_size(12)]
    pub a: Box<[()]>,
}
impl Default for R_34 {
    fn default() -> Self {
        {
            R_34 {
                a: Box::new([1_(), 2_(), 3_()]),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct R_36 {
    #[offset(0)]
    #[byte_size(12)]
    pub a: Box<[()]>,
}
impl Default for R_36 {
    fn default() -> Self {
        {
            R_36 {
                a: Box::new([1_(), 2_(), 3_()]),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct R_38 {
    #[offset(0)]
    #[byte_size(12)]
    pub a: Box<[()]>,
}
impl Default for R_38 {
    fn default() -> Self {
        {
            R_38 {
                a: Box::new([1_(), 2_(), 3_()]),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct R_40 {
    #[offset(0)]
    #[byte_size(12)]
    pub a: Box<[()]>,
}
impl Default for R_40 {
    fn default() -> Self {
        {
            R_40 {
                a: Box::new([1_(), 2_(), 3_()]),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct R_42 {
    #[offset(0)]
    #[byte_size(12)]
    pub a: Box<[()]>,
}
impl Default for R_42 {
    fn default() -> Self {
        {
            R_42 {
                a: Box::new([1_(), 2_(), 3_()]),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct R_44 {
    #[offset(0)]
    #[byte_size(12)]
    pub a: Box<[()]>,
}
impl Default for R_44 {
    fn default() -> Self {
        {
            R_44 {
                a: Box::new([1_(), 2_(), 3_()]),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct R_46 {
    #[offset(0)]
    #[byte_size(12)]
    pub a: Box<[()]>,
}
impl Default for R_46 {
    fn default() -> Self {
        {
            R_46 {
                a: Box::new([1_(), 2_(), 3_()]),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct R_48 {
    #[offset(0)]
    #[byte_size(12)]
    pub a: Box<[()]>,
}
impl Default for R_48 {
    fn default() -> Self {
        {
            R_48 {
                a: Box::new([1_(), 2_(), 3_()]),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct R_50 {
    #[offset(0)]
    #[byte_size(12)]
    pub a: Box<[()]>,
}
impl Default for R_50 {
    fn default() -> Self {
        {
            R_50 {
                a: Box::new([1_(), 2_(), 3_()]),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct R_52 {
    #[offset(0)]
    #[byte_size(12)]
    pub a: Box<[()]>,
}
impl Default for R_52 {
    fn default() -> Self {
        {
            R_52 {
                a: Box::new([1_(), 2_(), 3_()]),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct R_54 {
    #[offset(0)]
    #[byte_size(12)]
    pub a: Box<[()]>,
}
impl Default for R_54 {
    fn default() -> Self {
        {
            R_54 {
                a: Box::new([1_(), 2_(), 3_()]),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct R_56 {
    #[offset(0)]
    #[byte_size(12)]
    pub a: Box<[()]>,
}
impl Default for R_56 {
    fn default() -> Self {
        {
            R_56 {
                a: Box::new([1_(), 2_(), 3_()]),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct R_58 {
    #[offset(0)]
    #[byte_size(12)]
    pub a: Box<[()]>,
}
impl Default for R_58 {
    fn default() -> Self {
        {
            R_58 {
                a: Box::new([1_(), 2_(), 3_()]),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct R_60 {
    #[offset(0)]
    #[byte_size(12)]
    pub a: Box<[()]>,
}
impl Default for R_60 {
    fn default() -> Self {
        {
            R_60 {
                a: Box::new([1_(), 2_(), 3_()]),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct R_62 {
    #[offset(0)]
    #[byte_size(12)]
    pub a: Box<[()]>,
}
impl Default for R_62 {
    fn default() -> Self {
        {
            R_62 {
                a: Box::new([1_(), 2_(), 3_()]),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct R_64 {
    #[offset(0)]
    #[byte_size(12)]
    pub a: Box<[()]>,
}
impl Default for R_64 {
    fn default() -> Self {
        {
            R_64 {
                a: Box::new([1_(), 2_(), 3_()]),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct R_66 {
    #[offset(0)]
    #[byte_size(12)]
    pub a: Box<[()]>,
}
impl Default for R_66 {
    fn default() -> Self {
        {
            R_66 {
                a: Box::new([1_(), 2_(), 3_()]),
            }
        }
    }
}
pub fn test_67() -> () {
    {
        let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
        {
            let r : Value<() > = Rc::new(RefCell::new(std_ranges_subrange_cpp20_input_iterator_int_ptr___sentinel_wrapper_cpp20_input_iterator_int_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { cpp20_input_iterator_int_ptr_ :: new ( { ( a .as_pointer()  as Ptr::<()> )  } , )   } , { { let __tmp_16 : Value<cpp20_input_iterator_int_ptr_ > = Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_ :: new ( { ( a .as_pointer()  as Ptr::<()> ) . offset ( ( 3_() ) as isize )  } , )  )); sentinel_wrapper_cpp20_input_iterator_int_ptr__ :: new_1 ( { __tmp_16.as_pointer() } , )  }  } , )  )) ;
            unimplemented!();
        }
        {
            let r : Value<() > = Rc::new(RefCell::new(std_ranges_subrange_cpp20_input_iterator_int_ptr___sentinel_wrapper_cpp20_input_iterator_int_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { cpp20_input_iterator_int_ptr_ :: new ( { ( a .as_pointer()  as Ptr::<()> )  } , )   } , { { let __tmp_17 : Value<cpp20_input_iterator_int_ptr_ > = Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_ :: new ( { ( a .as_pointer()  as Ptr::<()> ) . offset ( ( 3_() ) as isize )  } , )  )); sentinel_wrapper_cpp20_input_iterator_int_ptr__ :: new_1 ( { __tmp_17.as_pointer() } , )  }  } , )  )) ;
            unimplemented!();
        }
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), const R&>"
        );;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), const R&&>"
        );;
    }
    {
        let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
        {
            let r : Value<() > = Rc::new(RefCell::new(std_ranges_subrange_cpp20_input_iterator_int_ptr___sentinel_wrapper_cpp20_input_iterator_int_ptr____std_ranges_subrange_kind_sized_ :: new_1 ( { cpp20_input_iterator_int_ptr_ :: new ( { ( a .as_pointer()  as Ptr::<()> )  } , )   } , { { let __tmp_18 : Value<cpp20_input_iterator_int_ptr_ > = Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_ :: new ( { ( a .as_pointer()  as Ptr::<()> ) . offset ( ( 3_() ) as isize )  } , )  )); sentinel_wrapper_cpp20_input_iterator_int_ptr__ :: new_1 ( { __tmp_18.as_pointer() } , )  }  } , { 3_()  } , )  )) ;
            unimplemented!();
        }
        {
            let r : Value<() > = Rc::new(RefCell::new(std_ranges_subrange_cpp20_input_iterator_int_ptr___sentinel_wrapper_cpp20_input_iterator_int_ptr____std_ranges_subrange_kind_sized_ :: new_1 ( { cpp20_input_iterator_int_ptr_ :: new ( { ( a .as_pointer()  as Ptr::<()> )  } , )   } , { { let __tmp_19 : Value<cpp20_input_iterator_int_ptr_ > = Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_ :: new ( { ( a .as_pointer()  as Ptr::<()> ) . offset ( ( 3_() ) as isize )  } , )  )); sentinel_wrapper_cpp20_input_iterator_int_ptr__ :: new_1 ( { __tmp_19.as_pointer() } , )  }  } , { 3_()  } , )  )) ;
            unimplemented!();
        }
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), const R&>"
        );;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::ranges::distance), const R&&>"
        );;
    }
    {
        ({ test_ordinary_33() });
    }
    ({ test_ordinary_35() });
    ({ test_ordinary_37() });
    ({ test_ordinary_39() });
    ({ test_ordinary_41() });
    ({ test_ordinary_43() });
    ({ test_ordinary_45() });
    ({ test_ordinary_47() });
    ({ test_ordinary_49() });
    ({ test_ordinary_51() });
    ({ test_ordinary_33() });
    ({ test_ordinary_53() });
    ({ test_ordinary_55() });
    ({ test_ordinary_57() });
    ({ test_ordinary_59() });
    ({ test_ordinary_61() });
    ({ test_ordinary_63() });
    ({ test_ordinary_65() });
    const _: () = assert!(
        true,
        "!std::is_invocable_v<decltype(std::ranges::distance), int>"
    );;
    const _: () = assert!(
        true,
        "!std::is_invocable_v<decltype(std::ranges::distance), int*>"
    );;
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
    ({ test_67() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait R_34Impl {
    fn begin(&self) -> cpp20_input_iterator_int_ptr_;
    fn end(&self) -> sized_sentinel_cpp20_input_iterator_int_ptr__;
}
impl R_34Impl for Ptr<R_34> {
    fn begin(&self) -> cpp20_input_iterator_int_ptr_ {
        return cpp20_input_iterator_int_ptr_::new({ (array_field_ptr!((*self), a) as Ptr<()>) });
    }
    fn end(&self) -> sized_sentinel_cpp20_input_iterator_int_ptr__ {
        return {
            let __tmp_0: Value<cpp20_input_iterator_int_ptr_> =
                Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                    (array_field_ptr!((*self), a) as Ptr<()>).offset((3_()) as isize)
                })));
            sized_sentinel_cpp20_input_iterator_int_ptr__::new_1({ __tmp_0.as_pointer() })
        };
    }
}
pub trait R_36Impl {
    fn begin(&self) -> cpp17_input_iterator_int_ptr__int_ptr_;
    fn end(&self) -> sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__;
}
impl R_36Impl for Ptr<R_36> {
    fn begin(&self) -> cpp17_input_iterator_int_ptr__int_ptr_ {
        return cpp17_input_iterator_int_ptr__int_ptr_::new({
            (array_field_ptr!((*self), a) as Ptr<()>)
        });
    }
    fn end(&self) -> sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ {
        return {
            let __tmp_1: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                    (array_field_ptr!((*self), a) as Ptr<()>).offset((3_()) as isize)
                })));
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                __tmp_1.as_pointer()
            })
        };
    }
}
pub trait R_38Impl {
    fn begin(&self) -> cpp20_input_iterator_int_ptr_;
    fn end(&self) -> sentinel_wrapper_cpp20_input_iterator_int_ptr__;
}
impl R_38Impl for Ptr<R_38> {
    fn begin(&self) -> cpp20_input_iterator_int_ptr_ {
        return cpp20_input_iterator_int_ptr_::new({ (array_field_ptr!((*self), a) as Ptr<()>) });
    }
    fn end(&self) -> sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
        return {
            let __tmp_2: Value<cpp20_input_iterator_int_ptr_> =
                Rc::new(RefCell::new(cpp20_input_iterator_int_ptr_::new({
                    (array_field_ptr!((*self), a) as Ptr<()>).offset((3_()) as isize)
                })));
            sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ __tmp_2.as_pointer() })
        };
    }
}
pub trait R_40Impl {
    fn begin(&self) -> cpp17_output_iterator_int_ptr_;
    fn end(&self) -> sentinel_wrapper_cpp17_output_iterator_int_ptr__;
}
impl R_40Impl for Ptr<R_40> {
    fn begin(&self) -> cpp17_output_iterator_int_ptr_ {
        return cpp17_output_iterator_int_ptr_::new({ (array_field_ptr!((*self), a) as Ptr<()>) });
    }
    fn end(&self) -> sentinel_wrapper_cpp17_output_iterator_int_ptr__ {
        return {
            let __tmp_3: Value<cpp17_output_iterator_int_ptr_> =
                Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                    (array_field_ptr!((*self), a) as Ptr<()>).offset((3_()) as isize)
                })));
            sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ __tmp_3.as_pointer() })
        };
    }
}
pub trait R_42Impl {
    fn begin(&self) -> forward_iterator_int_ptr_;
    fn end(&self) -> sentinel_wrapper_forward_iterator_int_ptr__;
}
impl R_42Impl for Ptr<R_42> {
    fn begin(&self) -> forward_iterator_int_ptr_ {
        return forward_iterator_int_ptr_::new_1({ (array_field_ptr!((*self), a) as Ptr<()>) });
    }
    fn end(&self) -> sentinel_wrapper_forward_iterator_int_ptr__ {
        return {
            let __tmp_4: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    (array_field_ptr!((*self), a) as Ptr<()>).offset((3_()) as isize)
                })));
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_4.as_pointer() })
        };
    }
}
pub trait R_44Impl {
    fn begin(&self) -> bidirectional_iterator_int_ptr_;
    fn end(&self) -> sentinel_wrapper_bidirectional_iterator_int_ptr__;
}
impl R_44Impl for Ptr<R_44> {
    fn begin(&self) -> bidirectional_iterator_int_ptr_ {
        return bidirectional_iterator_int_ptr_::new_1({
            (array_field_ptr!((*self), a) as Ptr<()>)
        });
    }
    fn end(&self) -> sentinel_wrapper_bidirectional_iterator_int_ptr__ {
        return {
            let __tmp_5: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    (array_field_ptr!((*self), a) as Ptr<()>).offset((3_()) as isize)
                })));
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ __tmp_5.as_pointer() })
        };
    }
}
pub trait R_46Impl {
    fn begin(&self) -> random_access_iterator_int_ptr_;
    fn end(&self) -> sentinel_wrapper_random_access_iterator_int_ptr__;
}
impl R_46Impl for Ptr<R_46> {
    fn begin(&self) -> random_access_iterator_int_ptr_ {
        return random_access_iterator_int_ptr_::new_1({
            (array_field_ptr!((*self), a) as Ptr<()>)
        });
    }
    fn end(&self) -> sentinel_wrapper_random_access_iterator_int_ptr__ {
        return {
            let __tmp_6: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    (array_field_ptr!((*self), a) as Ptr<()>).offset((3_()) as isize)
                })));
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ __tmp_6.as_pointer() })
        };
    }
}
pub trait R_48Impl {
    fn begin(&self) -> contiguous_iterator_int_ptr_;
    fn end(&self) -> sentinel_wrapper_contiguous_iterator_int_ptr__;
}
impl R_48Impl for Ptr<R_48> {
    fn begin(&self) -> contiguous_iterator_int_ptr_ {
        return contiguous_iterator_int_ptr_::new_1({ (array_field_ptr!((*self), a) as Ptr<()>) });
    }
    fn end(&self) -> sentinel_wrapper_contiguous_iterator_int_ptr__ {
        return {
            let __tmp_7: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    (array_field_ptr!((*self), a) as Ptr<()>).offset((3_()) as isize)
                })));
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_7.as_pointer() })
        };
    }
}
pub trait R_50Impl {
    fn begin(&self) -> Ptr<()>;
    fn end(&self) -> sentinel_wrapper_int_ptr_;
}
impl R_50Impl for Ptr<R_50> {
    fn begin(&self) -> Ptr<()> {
        return (array_field_ptr!((*self), a) as Ptr<()>);
    }
    fn end(&self) -> sentinel_wrapper_int_ptr_ {
        return {
            let __tmp_8: Value<Ptr<()>> = Rc::new(RefCell::new(
                (array_field_ptr!((*self), a) as Ptr<()>).offset((3_()) as isize),
            ));
            sentinel_wrapper_int_ptr_::new_1({ __tmp_8.as_pointer() })
        };
    }
}
pub trait R_52Impl {
    fn begin(&self) -> cpp17_input_iterator_int_ptr__int_ptr_;
    fn end(&self) -> sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__;
}
impl R_52Impl for Ptr<R_52> {
    fn begin(&self) -> cpp17_input_iterator_int_ptr__int_ptr_ {
        return cpp17_input_iterator_int_ptr__int_ptr_::new({
            (array_field_ptr!((*self), a) as Ptr<()>)
        });
    }
    fn end(&self) -> sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__ {
        return {
            let __tmp_9: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                    (array_field_ptr!((*self), a) as Ptr<()>).offset((3_()) as isize)
                })));
            sized_sentinel_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ __tmp_9.as_pointer() })
        };
    }
}
pub trait R_54Impl {
    fn begin(&self) -> cpp17_output_iterator_int_ptr_;
    fn end(&self) -> sized_sentinel_cpp17_output_iterator_int_ptr__;
}
impl R_54Impl for Ptr<R_54> {
    fn begin(&self) -> cpp17_output_iterator_int_ptr_ {
        return cpp17_output_iterator_int_ptr_::new({ (array_field_ptr!((*self), a) as Ptr<()>) });
    }
    fn end(&self) -> sized_sentinel_cpp17_output_iterator_int_ptr__ {
        return {
            let __tmp_10: Value<cpp17_output_iterator_int_ptr_> =
                Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                    (array_field_ptr!((*self), a) as Ptr<()>).offset((3_()) as isize)
                })));
            sized_sentinel_cpp17_output_iterator_int_ptr__::new_1({ __tmp_10.as_pointer() })
        };
    }
}
pub trait R_56Impl {
    fn begin(&self) -> forward_iterator_int_ptr_;
    fn end(&self) -> sized_sentinel_forward_iterator_int_ptr__;
}
impl R_56Impl for Ptr<R_56> {
    fn begin(&self) -> forward_iterator_int_ptr_ {
        return forward_iterator_int_ptr_::new_1({ (array_field_ptr!((*self), a) as Ptr<()>) });
    }
    fn end(&self) -> sized_sentinel_forward_iterator_int_ptr__ {
        return {
            let __tmp_11: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    (array_field_ptr!((*self), a) as Ptr<()>).offset((3_()) as isize)
                })));
            sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_11.as_pointer() })
        };
    }
}
pub trait R_58Impl {
    fn begin(&self) -> bidirectional_iterator_int_ptr_;
    fn end(&self) -> sized_sentinel_bidirectional_iterator_int_ptr__;
}
impl R_58Impl for Ptr<R_58> {
    fn begin(&self) -> bidirectional_iterator_int_ptr_ {
        return bidirectional_iterator_int_ptr_::new_1({
            (array_field_ptr!((*self), a) as Ptr<()>)
        });
    }
    fn end(&self) -> sized_sentinel_bidirectional_iterator_int_ptr__ {
        return {
            let __tmp_12: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    (array_field_ptr!((*self), a) as Ptr<()>).offset((3_()) as isize)
                })));
            sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ __tmp_12.as_pointer() })
        };
    }
}
pub trait R_60Impl {
    fn begin(&self) -> random_access_iterator_int_ptr_;
    fn end(&self) -> sized_sentinel_random_access_iterator_int_ptr__;
}
impl R_60Impl for Ptr<R_60> {
    fn begin(&self) -> random_access_iterator_int_ptr_ {
        return random_access_iterator_int_ptr_::new_1({
            (array_field_ptr!((*self), a) as Ptr<()>)
        });
    }
    fn end(&self) -> sized_sentinel_random_access_iterator_int_ptr__ {
        return {
            let __tmp_13: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    (array_field_ptr!((*self), a) as Ptr<()>).offset((3_()) as isize)
                })));
            sized_sentinel_random_access_iterator_int_ptr__::new_1({ __tmp_13.as_pointer() })
        };
    }
}
pub trait R_62Impl {
    fn begin(&self) -> contiguous_iterator_int_ptr_;
    fn end(&self) -> sized_sentinel_contiguous_iterator_int_ptr__;
}
impl R_62Impl for Ptr<R_62> {
    fn begin(&self) -> contiguous_iterator_int_ptr_ {
        return contiguous_iterator_int_ptr_::new_1({ (array_field_ptr!((*self), a) as Ptr<()>) });
    }
    fn end(&self) -> sized_sentinel_contiguous_iterator_int_ptr__ {
        return {
            let __tmp_14: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    (array_field_ptr!((*self), a) as Ptr<()>).offset((3_()) as isize)
                })));
            sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_14.as_pointer() })
        };
    }
}
pub trait R_64Impl {
    fn begin(&self) -> Ptr<()>;
    fn end(&self) -> sized_sentinel_int_ptr_;
}
impl R_64Impl for Ptr<R_64> {
    fn begin(&self) -> Ptr<()> {
        return (array_field_ptr!((*self), a) as Ptr<()>);
    }
    fn end(&self) -> sized_sentinel_int_ptr_ {
        return {
            let __tmp_15: Value<Ptr<()>> = Rc::new(RefCell::new(
                (array_field_ptr!((*self), a) as Ptr<()>).offset((3_()) as isize),
            ));
            sized_sentinel_int_ptr_::new_1({ __tmp_15.as_pointer() })
        };
    }
}
pub trait R_66Impl {
    fn begin(&self) -> Ptr<()>;
    fn end(&self) -> Ptr<()>;
}
impl R_66Impl for Ptr<R_66> {
    fn begin(&self) -> Ptr<()> {
        return (array_field_ptr!((*self), a) as Ptr<()>);
    }
    fn end(&self) -> Ptr<()> {
        return (array_field_ptr!((*self), a) as Ptr<()>).offset((3_()) as isize);
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
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_4((other).clone()) }) });
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
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_6((other).clone()) }) });
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
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_8((other).clone()) }) });
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
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_3((other).clone()) }) });
    }
}
pub trait sentinel_wrapper_int_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> ();
}
impl sentinel_wrapper_int_ptr_Impl for Ptr<sentinel_wrapper_int_ptr_> {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> () {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_7((other.read())) }) });
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
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_5((other).clone()) }) });
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
