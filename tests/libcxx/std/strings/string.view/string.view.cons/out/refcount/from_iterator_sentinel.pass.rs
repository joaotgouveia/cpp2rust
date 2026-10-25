extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(80)]
pub struct MultiStringType {
    #[offset(0)]
    #[byte_size(8)]
    pub w_: Ptr<()>,
    #[offset(8)]
    pub wn_: (),
    #[offset(16)]
    #[byte_size(8)]
    pub u8_: Ptr<()>,
    #[offset(24)]
    pub u8n_: (),
    #[offset(32)]
    #[byte_size(8)]
    pub u16_: Ptr<()>,
    #[offset(40)]
    pub u16n_: (),
    #[offset(48)]
    #[byte_size(8)]
    pub u32_: Ptr<()>,
    #[offset(56)]
    pub u32n_: (),
    #[offset(64)]
    #[byte_size(8)]
    pub s_: Ptr<()>,
    #[offset(72)]
    pub sn_: (),
}
impl MultiStringType {
    pub fn new(
        mut w: Ptr<()>,
        mut wn: (),
        mut u8: Ptr<()>,
        mut u8n: (),
        mut u16: Ptr<()>,
        mut u16n: (),
        mut u32: Ptr<()>,
        mut u32n: (),
        mut s: Ptr<()>,
        mut sn: (),
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp17_output_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct random_access_iterator_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_char_ptr_ {}
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
pub fn operator_sub_0(
    mut x: contiguous_iterator_const_char_ptr_,
    mut y: contiguous_iterator_const_char_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn base_1(i: Ptr<contiguous_iterator_const_char_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_const_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_char_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
}
pub fn operator_sub_2(
    mut x: contiguous_iterator_const_wchar_t_ptr_,
    mut y: contiguous_iterator_const_wchar_t_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn base_3(i: Ptr<contiguous_iterator_const_wchar_t_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_const_wchar_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_wchar_t_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
}
pub fn operator_sub_4(
    mut x: contiguous_iterator_const_char8_t_ptr_,
    mut y: contiguous_iterator_const_char8_t_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn base_5(i: Ptr<contiguous_iterator_const_char8_t_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_const_char8_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_char8_t_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
}
pub fn operator_sub_6(
    mut x: contiguous_iterator_const_char16_t_ptr_,
    mut y: contiguous_iterator_const_char16_t_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn base_7(i: Ptr<contiguous_iterator_const_char16_t_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_const_char16_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_char16_t_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
}
pub fn operator_sub_8(
    mut x: contiguous_iterator_const_char32_t_ptr_,
    mut y: contiguous_iterator_const_char32_t_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn base_9(i: Ptr<contiguous_iterator_const_char32_t_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_const_char32_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_char32_t_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_char_ptr_ {}
pub fn base_10(i: Ptr<()>) -> Ptr<()> {
    let i: Value<Ptr<()>> = Rc::new(RefCell::new(i));
    return (*i.borrow()).clone();
}
pub fn base_11(i: Ptr<()>) -> Ptr<()> {
    let i: Value<Ptr<()>> = Rc::new(RefCell::new(i));
    return (*i.borrow()).clone();
}
pub fn base_12(i: Ptr<()>) -> Ptr<()> {
    let i: Value<Ptr<()>> = Rc::new(RefCell::new(i));
    return (*i.borrow()).clone();
}
pub fn base_13(i: Ptr<()>) -> Ptr<()> {
    let i: Value<Ptr<()>> = Rc::new(RefCell::new(i));
    return (*i.borrow()).clone();
}
pub fn base_14(i: Ptr<()>) -> Ptr<()> {
    let i: Value<Ptr<()>> = Rc::new(RefCell::new(i));
    return (*i.borrow()).clone();
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
pub fn operator_sub_15(s: Ptr<sized_sentinel_const_char_ptr_>, i: Ptr<Ptr<()>>) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_10((i.read())) })) as ();
}
pub fn operator_sub_16(i: Ptr<Ptr<()>>, s: Ptr<sized_sentinel_const_char_ptr_>) -> () {
    return (({ base_10((i.read())) }) - s.with(|__s| __s.base_.clone())) as ();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_const_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sized_sentinel_const_char_ptr_ {
    pub fn new_1(it: Ptr<Ptr<()>>) -> Self {
        Self {
            base_: ({ base_10((it.read())) }),
        }
    }
}
pub fn operator_sub_17(
    s: Ptr<sized_sentinel_contiguous_iterator_const_char_ptr__>,
    i: Ptr<contiguous_iterator_const_char_ptr_>,
) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_1((i).clone()) })) as ();
}
pub fn operator_sub_18(
    i: Ptr<contiguous_iterator_const_char_ptr_>,
    s: Ptr<sized_sentinel_contiguous_iterator_const_char_ptr__>,
) -> () {
    return (({ base_1((i).clone()) }) - s.with(|__s| __s.base_.clone())) as ();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_contiguous_iterator_const_char_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sized_sentinel_contiguous_iterator_const_char_ptr__ {
    pub fn new_1(it: Ptr<contiguous_iterator_const_char_ptr_>) -> Self {
        Self {
            base_: ({ base_1((it).clone()) }),
        }
    }
}
pub fn operator_sub_19(s: Ptr<sized_sentinel_const_wchar_t_ptr_>, i: Ptr<Ptr<()>>) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_11((i.read())) })) as ();
}
pub fn operator_sub_20(i: Ptr<Ptr<()>>, s: Ptr<sized_sentinel_const_wchar_t_ptr_>) -> () {
    return (({ base_11((i.read())) }) - s.with(|__s| __s.base_.clone())) as ();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_const_wchar_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sized_sentinel_const_wchar_t_ptr_ {
    pub fn new_1(it: Ptr<Ptr<()>>) -> Self {
        Self {
            base_: ({ base_11((it.read())) }),
        }
    }
}
pub fn operator_sub_21(
    s: Ptr<sized_sentinel_contiguous_iterator_const_wchar_t_ptr__>,
    i: Ptr<contiguous_iterator_const_wchar_t_ptr_>,
) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_3((i).clone()) })) as ();
}
pub fn operator_sub_22(
    i: Ptr<contiguous_iterator_const_wchar_t_ptr_>,
    s: Ptr<sized_sentinel_contiguous_iterator_const_wchar_t_ptr__>,
) -> () {
    return (({ base_3((i).clone()) }) - s.with(|__s| __s.base_.clone())) as ();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_contiguous_iterator_const_wchar_t_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sized_sentinel_contiguous_iterator_const_wchar_t_ptr__ {
    pub fn new_1(it: Ptr<contiguous_iterator_const_wchar_t_ptr_>) -> Self {
        Self {
            base_: ({ base_3((it).clone()) }),
        }
    }
}
pub fn operator_sub_23(s: Ptr<sized_sentinel_const_char8_t_ptr_>, i: Ptr<Ptr<()>>) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_12((i.read())) })) as ();
}
pub fn operator_sub_24(i: Ptr<Ptr<()>>, s: Ptr<sized_sentinel_const_char8_t_ptr_>) -> () {
    return (({ base_12((i.read())) }) - s.with(|__s| __s.base_.clone())) as ();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_const_char8_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sized_sentinel_const_char8_t_ptr_ {
    pub fn new_1(it: Ptr<Ptr<()>>) -> Self {
        Self {
            base_: ({ base_12((it.read())) }),
        }
    }
}
pub fn operator_sub_25(
    s: Ptr<sized_sentinel_contiguous_iterator_const_char8_t_ptr__>,
    i: Ptr<contiguous_iterator_const_char8_t_ptr_>,
) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_5((i).clone()) })) as ();
}
pub fn operator_sub_26(
    i: Ptr<contiguous_iterator_const_char8_t_ptr_>,
    s: Ptr<sized_sentinel_contiguous_iterator_const_char8_t_ptr__>,
) -> () {
    return (({ base_5((i).clone()) }) - s.with(|__s| __s.base_.clone())) as ();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_contiguous_iterator_const_char8_t_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sized_sentinel_contiguous_iterator_const_char8_t_ptr__ {
    pub fn new_1(it: Ptr<contiguous_iterator_const_char8_t_ptr_>) -> Self {
        Self {
            base_: ({ base_5((it).clone()) }),
        }
    }
}
pub fn operator_sub_27(s: Ptr<sized_sentinel_const_char16_t_ptr_>, i: Ptr<Ptr<()>>) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_13((i.read())) })) as ();
}
pub fn operator_sub_28(i: Ptr<Ptr<()>>, s: Ptr<sized_sentinel_const_char16_t_ptr_>) -> () {
    return (({ base_13((i.read())) }) - s.with(|__s| __s.base_.clone())) as ();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_const_char16_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sized_sentinel_const_char16_t_ptr_ {
    pub fn new_1(it: Ptr<Ptr<()>>) -> Self {
        Self {
            base_: ({ base_13((it.read())) }),
        }
    }
}
pub fn operator_sub_29(
    s: Ptr<sized_sentinel_contiguous_iterator_const_char16_t_ptr__>,
    i: Ptr<contiguous_iterator_const_char16_t_ptr_>,
) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_7((i).clone()) })) as ();
}
pub fn operator_sub_30(
    i: Ptr<contiguous_iterator_const_char16_t_ptr_>,
    s: Ptr<sized_sentinel_contiguous_iterator_const_char16_t_ptr__>,
) -> () {
    return (({ base_7((i).clone()) }) - s.with(|__s| __s.base_.clone())) as ();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_contiguous_iterator_const_char16_t_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sized_sentinel_contiguous_iterator_const_char16_t_ptr__ {
    pub fn new_1(it: Ptr<contiguous_iterator_const_char16_t_ptr_>) -> Self {
        Self {
            base_: ({ base_7((it).clone()) }),
        }
    }
}
pub fn operator_sub_31(s: Ptr<sized_sentinel_const_char32_t_ptr_>, i: Ptr<Ptr<()>>) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_14((i.read())) })) as ();
}
pub fn operator_sub_32(i: Ptr<Ptr<()>>, s: Ptr<sized_sentinel_const_char32_t_ptr_>) -> () {
    return (({ base_14((i.read())) }) - s.with(|__s| __s.base_.clone())) as ();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_const_char32_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sized_sentinel_const_char32_t_ptr_ {
    pub fn new_1(it: Ptr<Ptr<()>>) -> Self {
        Self {
            base_: ({ base_14((it.read())) }),
        }
    }
}
pub fn operator_sub_33(
    s: Ptr<sized_sentinel_contiguous_iterator_const_char32_t_ptr__>,
    i: Ptr<contiguous_iterator_const_char32_t_ptr_>,
) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_9((i).clone()) })) as ();
}
pub fn operator_sub_34(
    i: Ptr<contiguous_iterator_const_char32_t_ptr_>,
    s: Ptr<sized_sentinel_contiguous_iterator_const_char32_t_ptr__>,
) -> () {
    return (({ base_9((i).clone()) }) - s.with(|__s| __s.base_.clone())) as ();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sized_sentinel_contiguous_iterator_const_char32_t_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sized_sentinel_contiguous_iterator_const_char32_t_ptr__ {
    pub fn new_1(it: Ptr<contiguous_iterator_const_char32_t_ptr_>) -> Self {
        Self {
            base_: ({ base_9((it).clone()) }),
        }
    }
}
pub fn operator_eq_35(
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
            operator_eq_35(
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
pub fn operator_add_36(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_37(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_36(_i, n)
    });
}
pub fn iter_swap_38(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_39(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_40(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_41(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
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
            operator_cmp_41(
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
            operator_eq_40(
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
pub fn test_construction_42(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_43(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_44(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_45(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_46(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_47(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_48(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_49(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_50(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_51(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_52(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_53(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_54(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_55(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_56(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_57(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_58(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_59(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_60(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_61(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_62(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_63(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_64(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_65(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_66(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_67(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_68(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_69(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_70(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_construction_71(val: ()) {
    let val: Value<()> = Rc::new(RefCell::new(val));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_with_char_72() {
    let val: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test_construction_42(unimplemented!()) });
    ({ test_construction_43(unimplemented!()) });
    ({ test_construction_44(unimplemented!()) });
    ({ test_construction_45(unimplemented!()) });
    ({ test_construction_46(unimplemented!()) });
    ({ test_construction_47(unimplemented!()) });
}
pub fn test_with_char_73() {
    let val: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test_construction_48(unimplemented!()) });
    ({ test_construction_49(unimplemented!()) });
    ({ test_construction_50(unimplemented!()) });
    ({ test_construction_51(unimplemented!()) });
    ({ test_construction_52(unimplemented!()) });
    ({ test_construction_53(unimplemented!()) });
}
pub fn test_with_char_74() {
    let val: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test_construction_54(unimplemented!()) });
    ({ test_construction_55(unimplemented!()) });
    ({ test_construction_56(unimplemented!()) });
    ({ test_construction_57(unimplemented!()) });
    ({ test_construction_58(unimplemented!()) });
    ({ test_construction_59(unimplemented!()) });
}
pub fn test_with_char_75() {
    let val: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test_construction_60(unimplemented!()) });
    ({ test_construction_61(unimplemented!()) });
    ({ test_construction_62(unimplemented!()) });
    ({ test_construction_63(unimplemented!()) });
    ({ test_construction_64(unimplemented!()) });
    ({ test_construction_65(unimplemented!()) });
}
pub fn test_with_char_76() {
    let val: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test_construction_66(unimplemented!()) });
    ({ test_construction_67(unimplemented!()) });
    ({ test_construction_68(unimplemented!()) });
    ({ test_construction_69(unimplemented!()) });
    ({ test_construction_70(unimplemented!()) });
    ({ test_construction_71(unimplemented!()) });
}
pub fn test_77() -> () {
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_77() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait MultiStringTypeImpl {
    fn as_ptr_2(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_3(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_4(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_5(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_6(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn length_7(&self, _a0: Ptr<()>) -> ();
    fn length_8(&self, _a0: Ptr<()>) -> ();
    fn length_9(&self, _a0: Ptr<()>) -> ();
    fn length_10(&self, _a0: Ptr<()>) -> ();
    fn length_11(&self, _a0: Ptr<()>) -> ();
    fn to_pconst_12(&self) -> Ptr<()>;
    fn to_pconst_13(&self) -> Ptr<()>;
    fn to_pconst_14(&self) -> Ptr<()>;
    fn to_pconst_15(&self) -> Ptr<()>;
    fn to_pconst_16(&self) -> Ptr<()>;
}
impl MultiStringTypeImpl for Ptr<MultiStringType> {
    fn as_ptr_2(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn as_ptr_3(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn as_ptr_4(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn as_ptr_5(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn as_ptr_6(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u32_.clone());
    }
    fn length_7(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.sn_);
    }
    fn length_8(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.wn_);
    }
    fn length_9(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u8n_);
    }
    fn length_10(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u16n_);
    }
    fn length_11(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u32n_);
    }
    fn to_pconst_12(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn to_pconst_13(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn to_pconst_14(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn to_pconst_15(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn to_pconst_16(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u32_.clone());
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
pub trait contiguous_iterator_char_ptr_Impl {
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
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_char_ptr_> {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_char_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: ()) -> contiguous_iterator_char_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: ()) -> contiguous_iterator_char_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_char_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_char_ptr_> {
        unimplemented!()
    }
}
impl contiguous_iterator_char_ptr_Impl for Ptr<contiguous_iterator_char_ptr_> {}
pub trait contiguous_iterator_const_char16_t_ptr_Impl {
    fn base(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_arrow(&self) -> ();
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_const_char16_t_ptr_> {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_const_char16_t_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: ()) -> contiguous_iterator_const_char16_t_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: ()) -> contiguous_iterator_const_char16_t_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_const_char16_t_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_const_char16_t_ptr_> {
        unimplemented!()
    }
}
impl contiguous_iterator_const_char16_t_ptr_Impl for Ptr<contiguous_iterator_const_char16_t_ptr_> {
    fn operator_arrow(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait contiguous_iterator_const_char32_t_ptr_Impl {
    fn base(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_arrow(&self) -> ();
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_const_char32_t_ptr_> {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_const_char32_t_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: ()) -> contiguous_iterator_const_char32_t_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: ()) -> contiguous_iterator_const_char32_t_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_const_char32_t_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_const_char32_t_ptr_> {
        unimplemented!()
    }
}
impl contiguous_iterator_const_char32_t_ptr_Impl for Ptr<contiguous_iterator_const_char32_t_ptr_> {
    fn operator_arrow(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait contiguous_iterator_const_char8_t_ptr_Impl {
    fn base(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_arrow(&self) -> ();
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_const_char8_t_ptr_> {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_const_char8_t_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: ()) -> contiguous_iterator_const_char8_t_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: ()) -> contiguous_iterator_const_char8_t_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_const_char8_t_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_const_char8_t_ptr_> {
        unimplemented!()
    }
}
impl contiguous_iterator_const_char8_t_ptr_Impl for Ptr<contiguous_iterator_const_char8_t_ptr_> {
    fn operator_arrow(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait contiguous_iterator_const_char_ptr_Impl {
    fn base(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_arrow(&self) -> ();
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_const_char_ptr_> {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_const_char_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: ()) -> contiguous_iterator_const_char_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: ()) -> contiguous_iterator_const_char_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_const_char_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_const_char_ptr_> {
        unimplemented!()
    }
}
impl contiguous_iterator_const_char_ptr_Impl for Ptr<contiguous_iterator_const_char_ptr_> {
    fn operator_arrow(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait contiguous_iterator_const_wchar_t_ptr_Impl {
    fn base(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_arrow(&self) -> ();
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_const_wchar_t_ptr_> {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_const_wchar_t_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: ()) -> contiguous_iterator_const_wchar_t_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: ()) -> contiguous_iterator_const_wchar_t_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_const_wchar_t_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_const_wchar_t_ptr_> {
        unimplemented!()
    }
}
impl contiguous_iterator_const_wchar_t_ptr_Impl for Ptr<contiguous_iterator_const_wchar_t_ptr_> {
    fn operator_arrow(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait cpp17_input_iterator_int_ptr__int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp17_input_iterator_int_ptr__int_ptr_ {
        unimplemented!()
    }
}
impl cpp17_input_iterator_int_ptr__int_ptr_Impl for Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {}
pub trait cpp17_output_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp17_output_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl cpp17_output_iterator_int_ptr_Impl for Ptr<cpp17_output_iterator_int_ptr_> {}
pub trait cpp20_input_iterator_int_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_input_iterator_int_ptr_>,
    ) -> Ptr<cpp20_input_iterator_int_ptr_>;
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_inc_5(&self) -> Ptr<cpp20_input_iterator_int_ptr_> {
        unimplemented!()
    }
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
pub trait random_access_iterator_char_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_char_ptr_> {
        unimplemented!()
    }
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_char_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: ()) -> random_access_iterator_char_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> random_access_iterator_char_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<random_access_iterator_char_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<random_access_iterator_char_ptr_> {
        unimplemented!()
    }
}
impl random_access_iterator_char_ptr_Impl for Ptr<random_access_iterator_char_ptr_> {}
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
pub trait sized_sentinel_const_char16_t_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> () {
        unimplemented!()
    }
}
impl sized_sentinel_const_char16_t_ptr_Impl for Ptr<sized_sentinel_const_char16_t_ptr_> {}
pub trait sized_sentinel_const_char32_t_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> () {
        unimplemented!()
    }
}
impl sized_sentinel_const_char32_t_ptr_Impl for Ptr<sized_sentinel_const_char32_t_ptr_> {}
pub trait sized_sentinel_const_char8_t_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> () {
        unimplemented!()
    }
}
impl sized_sentinel_const_char8_t_ptr_Impl for Ptr<sized_sentinel_const_char8_t_ptr_> {}
pub trait sized_sentinel_const_char_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> () {
        unimplemented!()
    }
}
impl sized_sentinel_const_char_ptr_Impl for Ptr<sized_sentinel_const_char_ptr_> {}
pub trait sized_sentinel_const_wchar_t_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> () {
        unimplemented!()
    }
}
impl sized_sentinel_const_wchar_t_ptr_Impl for Ptr<sized_sentinel_const_wchar_t_ptr_> {}
pub trait sized_sentinel_contiguous_iterator_const_char16_t_ptr__Impl {
    fn operator_eq(&self, other: Ptr<contiguous_iterator_const_char16_t_ptr_>) -> () {
        unimplemented!()
    }
}
impl sized_sentinel_contiguous_iterator_const_char16_t_ptr__Impl
    for Ptr<sized_sentinel_contiguous_iterator_const_char16_t_ptr__>
{
}
pub trait sized_sentinel_contiguous_iterator_const_char32_t_ptr__Impl {
    fn operator_eq(&self, other: Ptr<contiguous_iterator_const_char32_t_ptr_>) -> () {
        unimplemented!()
    }
}
impl sized_sentinel_contiguous_iterator_const_char32_t_ptr__Impl
    for Ptr<sized_sentinel_contiguous_iterator_const_char32_t_ptr__>
{
}
pub trait sized_sentinel_contiguous_iterator_const_char8_t_ptr__Impl {
    fn operator_eq(&self, other: Ptr<contiguous_iterator_const_char8_t_ptr_>) -> () {
        unimplemented!()
    }
}
impl sized_sentinel_contiguous_iterator_const_char8_t_ptr__Impl
    for Ptr<sized_sentinel_contiguous_iterator_const_char8_t_ptr__>
{
}
pub trait sized_sentinel_contiguous_iterator_const_char_ptr__Impl {
    fn operator_eq(&self, other: Ptr<contiguous_iterator_const_char_ptr_>) -> () {
        unimplemented!()
    }
}
impl sized_sentinel_contiguous_iterator_const_char_ptr__Impl
    for Ptr<sized_sentinel_contiguous_iterator_const_char_ptr__>
{
}
pub trait sized_sentinel_contiguous_iterator_const_wchar_t_ptr__Impl {
    fn operator_eq(&self, other: Ptr<contiguous_iterator_const_wchar_t_ptr_>) -> () {
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
        unimplemented!();
        field!(other, moved_from_).write(true);
        field!((*self), moved_from_).write(false);
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
