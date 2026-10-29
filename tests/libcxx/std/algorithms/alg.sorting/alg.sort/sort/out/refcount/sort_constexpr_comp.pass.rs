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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
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
    it_: Ptr<()>,
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
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub fn operator_add_0(
    x: random_access_iterator_int_ptr_,
    mut n: (),
) -> random_access_iterator_int_ptr_ {
    let x: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_int_ptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return random_access_iterator_int_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_1(
    x: random_access_iterator_int_ptr_,
    mut n: (),
) -> random_access_iterator_int_ptr_ {
    let x: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_int_ptr_Impl::operator_sub_assign(&x.as_pointer(), n) });
    return random_access_iterator_int_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_2(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn operator_eq_3(
    x: Ptr<random_access_iterator_int_ptr_>,
    y: Ptr<random_access_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_4(
    x: Ptr<random_access_iterator_int_ptr_>,
    y: Ptr<random_access_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_lt_5(
    x: Ptr<random_access_iterator_int_ptr_>,
    y: Ptr<random_access_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } < { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
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
impl CppLess for random_access_iterator_int_ptr_ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_5(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for random_access_iterator_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_3(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_add_6(
    x: random_access_iterator_MoveOnly_ptr_,
    mut n: (),
) -> random_access_iterator_MoveOnly_ptr_ {
    let x: Value<random_access_iterator_MoveOnly_ptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_MoveOnly_ptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return random_access_iterator_MoveOnly_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_7(
    x: random_access_iterator_MoveOnly_ptr_,
    mut n: (),
) -> random_access_iterator_MoveOnly_ptr_ {
    let x: Value<random_access_iterator_MoveOnly_ptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_MoveOnly_ptr_Impl::operator_sub_assign(&x.as_pointer(), n) });
    return random_access_iterator_MoveOnly_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_8(
    mut x: random_access_iterator_MoveOnly_ptr_,
    mut y: random_access_iterator_MoveOnly_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn operator_eq_9(
    x: Ptr<random_access_iterator_MoveOnly_ptr_>,
    y: Ptr<random_access_iterator_MoveOnly_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_10(
    x: Ptr<random_access_iterator_MoveOnly_ptr_>,
    y: Ptr<random_access_iterator_MoveOnly_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_lt_11(
    x: Ptr<random_access_iterator_MoveOnly_ptr_>,
    y: Ptr<random_access_iterator_MoveOnly_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } < { y.with(|__s| __s.it_.clone()) });
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
impl CppLess for random_access_iterator_MoveOnly_ptr_ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_11(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for random_access_iterator_MoveOnly_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_9(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_add_12(
    x: random_access_iterator_char_ptrptr_,
    mut n: (),
) -> random_access_iterator_char_ptrptr_ {
    let x: Value<random_access_iterator_char_ptrptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_char_ptrptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return random_access_iterator_char_ptrptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_13(
    x: random_access_iterator_char_ptrptr_,
    mut n: (),
) -> random_access_iterator_char_ptrptr_ {
    let x: Value<random_access_iterator_char_ptrptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_char_ptrptr_Impl::operator_sub_assign(&x.as_pointer(), n) });
    return random_access_iterator_char_ptrptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_14(
    mut x: random_access_iterator_char_ptrptr_,
    mut y: random_access_iterator_char_ptrptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn operator_eq_15(
    x: Ptr<random_access_iterator_char_ptrptr_>,
    y: Ptr<random_access_iterator_char_ptrptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_16(
    x: Ptr<random_access_iterator_char_ptrptr_>,
    y: Ptr<random_access_iterator_char_ptrptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_lt_17(
    x: Ptr<random_access_iterator_char_ptrptr_>,
    y: Ptr<random_access_iterator_char_ptrptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } < { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct random_access_iterator_char_ptrptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<Ptr<()>>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_char_ptrptr_ {
    pub fn new_1(mut it: Ptr<Ptr<()>>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<random_access_iterator_char_ptrptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppLess for random_access_iterator_char_ptrptr_ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_17(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for random_access_iterator_char_ptrptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_15(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_add_18(
    x: random_access_iterator_const_char_ptrptr_,
    mut n: (),
) -> random_access_iterator_const_char_ptrptr_ {
    let x: Value<random_access_iterator_const_char_ptrptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_const_char_ptrptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return random_access_iterator_const_char_ptrptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_19(
    x: random_access_iterator_const_char_ptrptr_,
    mut n: (),
) -> random_access_iterator_const_char_ptrptr_ {
    let x: Value<random_access_iterator_const_char_ptrptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_const_char_ptrptr_Impl::operator_sub_assign(&x.as_pointer(), n) });
    return random_access_iterator_const_char_ptrptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_20(
    mut x: random_access_iterator_const_char_ptrptr_,
    mut y: random_access_iterator_const_char_ptrptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn operator_eq_21(
    x: Ptr<random_access_iterator_const_char_ptrptr_>,
    y: Ptr<random_access_iterator_const_char_ptrptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_22(
    x: Ptr<random_access_iterator_const_char_ptrptr_>,
    y: Ptr<random_access_iterator_const_char_ptrptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_lt_23(
    x: Ptr<random_access_iterator_const_char_ptrptr_>,
    y: Ptr<random_access_iterator_const_char_ptrptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } < { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct random_access_iterator_const_char_ptrptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<Ptr<()>>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_const_char_ptrptr_ {
    pub fn new_1(mut it: Ptr<Ptr<()>>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<random_access_iterator_const_char_ptrptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppLess for random_access_iterator_const_char_ptrptr_ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_23(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for random_access_iterator_const_char_ptrptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_21(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_add_24(
    x: random_access_iterator_int_ptrptr_,
    mut n: (),
) -> random_access_iterator_int_ptrptr_ {
    let x: Value<random_access_iterator_int_ptrptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_int_ptrptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return random_access_iterator_int_ptrptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_25(
    x: random_access_iterator_int_ptrptr_,
    mut n: (),
) -> random_access_iterator_int_ptrptr_ {
    let x: Value<random_access_iterator_int_ptrptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_int_ptrptr_Impl::operator_sub_assign(&x.as_pointer(), n) });
    return random_access_iterator_int_ptrptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_26(
    mut x: random_access_iterator_int_ptrptr_,
    mut y: random_access_iterator_int_ptrptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn operator_eq_27(
    x: Ptr<random_access_iterator_int_ptrptr_>,
    y: Ptr<random_access_iterator_int_ptrptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_28(
    x: Ptr<random_access_iterator_int_ptrptr_>,
    y: Ptr<random_access_iterator_int_ptrptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_lt_29(
    x: Ptr<random_access_iterator_int_ptrptr_>,
    y: Ptr<random_access_iterator_int_ptrptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } < { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct random_access_iterator_int_ptrptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<Ptr<()>>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptrptr_ {
    pub fn new_1(mut it: Ptr<Ptr<()>>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<random_access_iterator_int_ptrptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppLess for random_access_iterator_int_ptrptr_ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_29(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for random_access_iterator_int_ptrptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_27(__lhs.clone(), __rhs.clone())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
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
pub fn operator_add_30(x: contiguous_iterator_int_ptr_, mut n: ()) -> contiguous_iterator_int_ptr_ {
    let x: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(x));
    ({ contiguous_iterator_int_ptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return contiguous_iterator_int_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_31(x: contiguous_iterator_int_ptr_, mut n: ()) -> contiguous_iterator_int_ptr_ {
    let x: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(x));
    ({ contiguous_iterator_int_ptr_Impl::operator_sub_assign(&x.as_pointer(), n) });
    return contiguous_iterator_int_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_32(
    mut x: contiguous_iterator_int_ptr_,
    mut y: contiguous_iterator_int_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn operator_eq_33(
    x: Ptr<contiguous_iterator_int_ptr_>,
    y: Ptr<contiguous_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_34(
    x: Ptr<contiguous_iterator_int_ptr_>,
    y: Ptr<contiguous_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_lt_35(
    x: Ptr<contiguous_iterator_int_ptr_>,
    y: Ptr<contiguous_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } < { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
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
impl CppLess for contiguous_iterator_int_ptr_ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_35(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for contiguous_iterator_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_33(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_add_36(
    x: contiguous_iterator_MoveOnly_ptr_,
    mut n: (),
) -> contiguous_iterator_MoveOnly_ptr_ {
    let x: Value<contiguous_iterator_MoveOnly_ptr_> = Rc::new(RefCell::new(x));
    ({ contiguous_iterator_MoveOnly_ptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return contiguous_iterator_MoveOnly_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_37(
    x: contiguous_iterator_MoveOnly_ptr_,
    mut n: (),
) -> contiguous_iterator_MoveOnly_ptr_ {
    let x: Value<contiguous_iterator_MoveOnly_ptr_> = Rc::new(RefCell::new(x));
    ({ contiguous_iterator_MoveOnly_ptr_Impl::operator_sub_assign(&x.as_pointer(), n) });
    return contiguous_iterator_MoveOnly_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_38(
    mut x: contiguous_iterator_MoveOnly_ptr_,
    mut y: contiguous_iterator_MoveOnly_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn operator_eq_39(
    x: Ptr<contiguous_iterator_MoveOnly_ptr_>,
    y: Ptr<contiguous_iterator_MoveOnly_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_40(
    x: Ptr<contiguous_iterator_MoveOnly_ptr_>,
    y: Ptr<contiguous_iterator_MoveOnly_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_lt_41(
    x: Ptr<contiguous_iterator_MoveOnly_ptr_>,
    y: Ptr<contiguous_iterator_MoveOnly_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } < { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_MoveOnly_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<MoveOnly>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_MoveOnly_ptr_ {
    pub fn new_1(mut it: Ptr<MoveOnly>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<contiguous_iterator_MoveOnly_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppLess for contiguous_iterator_MoveOnly_ptr_ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_41(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for contiguous_iterator_MoveOnly_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_39(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_add_42(
    x: contiguous_iterator_char_ptrptr_,
    mut n: (),
) -> contiguous_iterator_char_ptrptr_ {
    let x: Value<contiguous_iterator_char_ptrptr_> = Rc::new(RefCell::new(x));
    ({ contiguous_iterator_char_ptrptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return contiguous_iterator_char_ptrptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_43(
    x: contiguous_iterator_char_ptrptr_,
    mut n: (),
) -> contiguous_iterator_char_ptrptr_ {
    let x: Value<contiguous_iterator_char_ptrptr_> = Rc::new(RefCell::new(x));
    ({ contiguous_iterator_char_ptrptr_Impl::operator_sub_assign(&x.as_pointer(), n) });
    return contiguous_iterator_char_ptrptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_44(
    mut x: contiguous_iterator_char_ptrptr_,
    mut y: contiguous_iterator_char_ptrptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn operator_eq_45(
    x: Ptr<contiguous_iterator_char_ptrptr_>,
    y: Ptr<contiguous_iterator_char_ptrptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_46(
    x: Ptr<contiguous_iterator_char_ptrptr_>,
    y: Ptr<contiguous_iterator_char_ptrptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_lt_47(
    x: Ptr<contiguous_iterator_char_ptrptr_>,
    y: Ptr<contiguous_iterator_char_ptrptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } < { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_char_ptrptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<Ptr<()>>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_char_ptrptr_ {
    pub fn new_1(mut it: Ptr<Ptr<()>>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<contiguous_iterator_char_ptrptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppLess for contiguous_iterator_char_ptrptr_ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_47(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for contiguous_iterator_char_ptrptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_45(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_add_48(
    x: contiguous_iterator_const_char_ptrptr_,
    mut n: (),
) -> contiguous_iterator_const_char_ptrptr_ {
    let x: Value<contiguous_iterator_const_char_ptrptr_> = Rc::new(RefCell::new(x));
    ({ contiguous_iterator_const_char_ptrptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return contiguous_iterator_const_char_ptrptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_49(
    x: contiguous_iterator_const_char_ptrptr_,
    mut n: (),
) -> contiguous_iterator_const_char_ptrptr_ {
    let x: Value<contiguous_iterator_const_char_ptrptr_> = Rc::new(RefCell::new(x));
    ({ contiguous_iterator_const_char_ptrptr_Impl::operator_sub_assign(&x.as_pointer(), n) });
    return contiguous_iterator_const_char_ptrptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_50(
    mut x: contiguous_iterator_const_char_ptrptr_,
    mut y: contiguous_iterator_const_char_ptrptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn operator_eq_51(
    x: Ptr<contiguous_iterator_const_char_ptrptr_>,
    y: Ptr<contiguous_iterator_const_char_ptrptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_52(
    x: Ptr<contiguous_iterator_const_char_ptrptr_>,
    y: Ptr<contiguous_iterator_const_char_ptrptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_lt_53(
    x: Ptr<contiguous_iterator_const_char_ptrptr_>,
    y: Ptr<contiguous_iterator_const_char_ptrptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } < { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_const_char_ptrptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<Ptr<()>>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_char_ptrptr_ {
    pub fn new_1(mut it: Ptr<Ptr<()>>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<contiguous_iterator_const_char_ptrptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppLess for contiguous_iterator_const_char_ptrptr_ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_53(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for contiguous_iterator_const_char_ptrptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_51(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_add_54(
    x: contiguous_iterator_int_ptrptr_,
    mut n: (),
) -> contiguous_iterator_int_ptrptr_ {
    let x: Value<contiguous_iterator_int_ptrptr_> = Rc::new(RefCell::new(x));
    ({ contiguous_iterator_int_ptrptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return contiguous_iterator_int_ptrptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_55(
    x: contiguous_iterator_int_ptrptr_,
    mut n: (),
) -> contiguous_iterator_int_ptrptr_ {
    let x: Value<contiguous_iterator_int_ptrptr_> = Rc::new(RefCell::new(x));
    ({ contiguous_iterator_int_ptrptr_Impl::operator_sub_assign(&x.as_pointer(), n) });
    return contiguous_iterator_int_ptrptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_56(
    mut x: contiguous_iterator_int_ptrptr_,
    mut y: contiguous_iterator_int_ptrptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn operator_eq_57(
    x: Ptr<contiguous_iterator_int_ptrptr_>,
    y: Ptr<contiguous_iterator_int_ptrptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_58(
    x: Ptr<contiguous_iterator_int_ptrptr_>,
    y: Ptr<contiguous_iterator_int_ptrptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_lt_59(
    x: Ptr<contiguous_iterator_int_ptrptr_>,
    y: Ptr<contiguous_iterator_int_ptrptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } < { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_int_ptrptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<Ptr<()>>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_int_ptrptr_ {
    pub fn new_1(mut it: Ptr<Ptr<()>>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<contiguous_iterator_int_ptrptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppLess for contiguous_iterator_int_ptrptr_ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_59(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for contiguous_iterator_int_ptrptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_57(__lhs.clone(), __rhs.clone())
    }
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
pub fn operator_eq_60(
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
impl CppEq for forward_sized_iterator_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_60(__lhs.clone(), __rhs.clone())
    }
}
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
pub fn operator_add_61(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_62(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_61(_i, n)
    });
}
pub fn iter_swap_63(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_64(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_65(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_66(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
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
impl CppLess for adl_Iterator {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_66(__lhs.clone(), __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for adl_Iterator {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_65(__lhs.clone(), __rhs.clone())
    }
}
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub fn new(mut what: Ptr<()>) -> Self {
        Self {}
    }
}
pub fn operator_eq_67(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_68(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_69(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_70(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_71(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_72(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_73(_a0: Ptr<MoveOnly>, _a1: Ptr<MoveOnly>) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    data_: (),
}
impl MoveOnly {
    pub fn new(data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| 1_());
        Self { data_: data }
    }
    pub fn move_from(x: Ptr<MoveOnly>) -> Self {
        let __this: MoveOnly = Self {
            data_: x.with(|__s| __s.data_),
        };
        field!(x, data_).write(0_());
        __this
    }
}
impl CppLess for MoveOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_69(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for MoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_67(__lhs.clone(), __rhs.clone())
    }
}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new(None) }
    }
}
pub fn operator_eq_74(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_75(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_76(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_77(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_78(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_79(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_80(_a0: Ptr<TrivialMoveOnly>, _a1: Ptr<TrivialMoveOnly>) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(4)]
pub struct TrivialMoveOnly {
    #[offset(0)]
    data_: (),
}
impl TrivialMoveOnly {
    pub fn new(data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| 1_());
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
        operator_lt_76(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for TrivialMoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_74(__lhs.clone(), __rhs.clone())
    }
}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        { TrivialMoveOnly::new(None) }
    }
}
thread_local!(
    pub static LargeN_81: Value<()> = Rc::new(RefCell::new(128_()));
);
pub fn test_82() -> () {
    let orig: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while (i < 7_()) {
        x = { ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_()) };
        (*orig.borrow_mut())[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let work: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_83() -> () {
    let orig: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while (i < 7_()) {
        x = { ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_()) };
        (*orig.borrow_mut())[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let work: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_84() -> () {
    let orig: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while (i < 128_()) {
        x = { ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_()) };
        (*orig.borrow_mut())[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let work: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_85() -> () {
    let orig: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while (i < 128_()) {
        x = { ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_()) };
        (*orig.borrow_mut())[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let work: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_86() -> () {
    let orig: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while (i < 7_()) {
        x = { ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_()) };
        (*orig.borrow_mut())[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let work: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_87() -> () {
    let orig: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while (i < 7_()) {
        x = { ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_()) };
        (*orig.borrow_mut())[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let work: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_88() -> () {
    let orig: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while (i < 128_()) {
        x = { ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_()) };
        (*orig.borrow_mut())[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let work: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_89() -> () {
    let orig: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while (i < 128_()) {
        x = { ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_()) };
        (*orig.borrow_mut())[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let work: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_90() -> () {
    let orig: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while (i < 7_()) {
        x = { ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_()) };
        (*orig.borrow_mut())[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let work: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_91() -> () {
    let orig: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while (i < 128_()) {
        x = { ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_()) };
        (*orig.borrow_mut())[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let work: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_92() -> () {
    let orig: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while (i < 7_()) {
        x = { ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_()) };
        (*orig.borrow_mut())[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let work: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_93() -> () {
    let orig: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while (i < 128_()) {
        x = { ((x).wrapping_mul(1664525_())).wrapping_add(1013904223_()) };
        (*orig.borrow_mut())[(i) as usize] = (x).wrapping_rem(1000_());
        i.prefix_inc();
    }
    let work: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
        MoveOnly::new(None),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_pointers_94() -> () {
    let data: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let orig: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
    ])));
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while (i < 17_()) {
        (*orig.borrow_mut())[(i) as usize] =
            ((data.as_pointer() as Ptr<()>).offset((x).wrapping_rem(258_())));
        i.prefix_inc();
    }
    let work: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_pointers_95() -> () {
    let data: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let orig: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
    ])));
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while (i < 17_()) {
        (*orig.borrow_mut())[(i) as usize] =
            ((data.as_pointer() as Ptr<()>).offset((x).wrapping_rem(258_())));
        i.prefix_inc();
    }
    let work: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_pointers_96() -> () {
    let data: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let orig: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
    ])));
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while (i < 17_()) {
        (*orig.borrow_mut())[(i) as usize] =
            ((data.as_pointer() as Ptr<()>).offset((x).wrapping_rem(258_())));
        i.prefix_inc();
    }
    let work: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_pointers_97() -> () {
    let data: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let orig: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
    ])));
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while (i < 17_()) {
        (*orig.borrow_mut())[(i) as usize] =
            ((data.as_pointer() as Ptr<()>).offset((x).wrapping_rem(258_())));
        i.prefix_inc();
    }
    let work: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_pointers_98() -> () {
    let data: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let orig: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
    ])));
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while (i < 17_()) {
        (*orig.borrow_mut())[(i) as usize] =
            ((data.as_pointer() as Ptr<()>).offset((x).wrapping_rem(258_())));
        i.prefix_inc();
    }
    let work: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_pointers_99() -> () {
    let data: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let orig: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
    ])));
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while (i < 17_()) {
        (*orig.borrow_mut())[(i) as usize] =
            ((data.as_pointer() as Ptr<()>).offset((x).wrapping_rem(258_())));
        i.prefix_inc();
    }
    let work: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_pointers_100() -> () {
    let data: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let orig: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
    ])));
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while (i < 17_()) {
        (*orig.borrow_mut())[(i) as usize] =
            ((data.as_pointer() as Ptr<()>).offset((x).wrapping_rem(258_())));
        i.prefix_inc();
    }
    let work: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_pointers_101() -> () {
    let data: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let orig: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
    ])));
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while (i < 17_()) {
        (*orig.borrow_mut())[(i) as usize] =
            ((data.as_pointer() as Ptr<()>).offset((x).wrapping_rem(258_())));
        i.prefix_inc();
    }
    let work: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_pointers_102() -> () {
    let data: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let orig: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
    ])));
    let mut x: () = 1_();
    let mut i: () = 0_();
    'loop_: while (i < 17_()) {
        (*orig.borrow_mut())[(i) as usize] =
            ((data.as_pointer() as Ptr<()>).offset((x).wrapping_rem(258_())));
        i.prefix_inc();
    }
    let work: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
        Ptr::<()>::null(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    ({ test_82() });
    ({ test_83() });
    ({ test_84() });
    ({ test_85() });
    ({ test_86() });
    ({ test_87() });
    ({ test_88() });
    ({ test_89() });
    ({ test_pointers_94() });
    ({ test_pointers_95() });
    ({ test_pointers_96() });
    ({ test_pointers_97() });
    ({ test_pointers_98() });
    ({ test_pointers_99() });
    ({ test_90() });
    ({ test_91() });
    ({ test_92() });
    ({ test_93() });
    ({ test_pointers_100() });
    ({ test_pointers_101() });
    ({ test_pointers_102() });
    const _: () = assert!(true, "test<7, int, int*>()");;
    const _: () = assert!(true, "test<7, int, random_access_iterator<int*>>()");;
    const _: () = assert!(true, "test<7, int, contiguous_iterator<int*>>()");;
    const _: () = assert!(true, "test<LargeN, int, int*>()");;
    const _: () = assert!(true, "test<LargeN, int, random_access_iterator<int*>>()");;
    const _: () = assert!(true, "test<LargeN, int, contiguous_iterator<int*>>()");;
    const _: () = assert!(true, "test<7, MoveOnly, MoveOnly*>()");;
    const _: () = assert!(
        true,
        "test<7, MoveOnly, random_access_iterator<MoveOnly*>>()"
    );;
    const _: () = assert!(true, "test<7, MoveOnly, contiguous_iterator<MoveOnly*>>()");;
    const _: () = assert!(true, "test<LargeN, MoveOnly, MoveOnly*>()");;
    const _: () = assert!(
        true,
        "test<LargeN, MoveOnly, random_access_iterator<MoveOnly*>>()"
    );;
    const _: () = assert!(
        true,
        "test<LargeN, MoveOnly, contiguous_iterator<MoveOnly*>>()"
    );;
    const _: () = assert!(true, "test_pointers<17, char, char**>()");;
    const _: () = assert!(
        true,
        "test_pointers<17, char, random_access_iterator<char**>>()"
    );;
    const _: () = assert!(
        true,
        "test_pointers<17, char, contiguous_iterator<char**>>()"
    );;
    const _: () = assert!(true, "test_pointers<17, const char, const char**>()");;
    const _: () = assert!(
        true,
        "test_pointers<17, const char, random_access_iterator<const char**>>()"
    );;
    const _: () = assert!(
        true,
        "test_pointers<17, const char, contiguous_iterator<const char**>>()"
    );;
    const _: () = assert!(true, "test_pointers<17, int, int**>()");;
    const _: () = assert!(
        true,
        "test_pointers<17, int, random_access_iterator<int**>>()"
    );;
    const _: () = assert!(true, "test_pointers<17, int, contiguous_iterator<int**>>()");;
    return 0_();
}
pub trait MoveOnlyImpl {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
    fn get(&self) -> ();
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly;
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        field!((*self), data_).write({ x.with(|__s| __s.data_) });
        field!(x, data_).write(0_());
        return (*self).clone();
    }
    fn get(&self) -> () {
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
    fn get(&self) -> ();
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
}
impl TrivialMoveOnlyImpl for Ptr<TrivialMoveOnly> {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly> {
        field!((*self), data_).write({ { (*_a0.upgrade().deref()).data_ } });
        return (*self).clone();
    }
    fn get(&self) -> () {
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
pub trait contiguous_iterator_MoveOnly_ptr_Impl {
    fn base(&self) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn operator_deref(&self) -> ();
    fn operator_arrow(&self) -> () {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_MoveOnly_ptr_>;
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_MoveOnly_ptr_>;
    fn operator_post_inc_11(&self, mut _a0: ()) -> contiguous_iterator_MoveOnly_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: ()) -> contiguous_iterator_MoveOnly_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: ()) -> Ptr<contiguous_iterator_MoveOnly_ptr_>;
    fn operator_sub_assign(&self, n: ()) -> Ptr<contiguous_iterator_MoveOnly_ptr_>;
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_MoveOnly_ptr_>,
    ) -> Ptr<contiguous_iterator_MoveOnly_ptr_>;
}
impl contiguous_iterator_MoveOnly_ptr_Impl for Ptr<contiguous_iterator_MoveOnly_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_MoveOnly_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_MoveOnly_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_MoveOnly_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_MoveOnly_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_MoveOnly_ptr_>,
    ) -> Ptr<contiguous_iterator_MoveOnly_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait contiguous_iterator_char_ptrptr_Impl {
    fn base(&self) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn operator_deref(&self) -> ();
    fn operator_arrow(&self) -> ();
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_char_ptrptr_>;
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_char_ptrptr_>;
    fn operator_post_inc_11(&self, mut _a0: ()) -> contiguous_iterator_char_ptrptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: ()) -> contiguous_iterator_char_ptrptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: ()) -> Ptr<contiguous_iterator_char_ptrptr_>;
    fn operator_sub_assign(&self, n: ()) -> Ptr<contiguous_iterator_char_ptrptr_>;
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_char_ptrptr_>,
    ) -> Ptr<contiguous_iterator_char_ptrptr_>;
}
impl contiguous_iterator_char_ptrptr_Impl for Ptr<contiguous_iterator_char_ptrptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_arrow(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_char_ptrptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_char_ptrptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_char_ptrptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_char_ptrptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_char_ptrptr_>,
    ) -> Ptr<contiguous_iterator_char_ptrptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait contiguous_iterator_const_char_ptrptr_Impl {
    fn base(&self) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn operator_deref(&self) -> ();
    fn operator_arrow(&self) -> ();
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_const_char_ptrptr_>;
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_const_char_ptrptr_>;
    fn operator_post_inc_11(&self, mut _a0: ()) -> contiguous_iterator_const_char_ptrptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: ()) -> contiguous_iterator_const_char_ptrptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: ()) -> Ptr<contiguous_iterator_const_char_ptrptr_>;
    fn operator_sub_assign(&self, n: ()) -> Ptr<contiguous_iterator_const_char_ptrptr_>;
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_const_char_ptrptr_>,
    ) -> Ptr<contiguous_iterator_const_char_ptrptr_>;
}
impl contiguous_iterator_const_char_ptrptr_Impl for Ptr<contiguous_iterator_const_char_ptrptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_arrow(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_const_char_ptrptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_const_char_ptrptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_const_char_ptrptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_const_char_ptrptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_const_char_ptrptr_>,
    ) -> Ptr<contiguous_iterator_const_char_ptrptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait contiguous_iterator_int_ptr_Impl {
    fn base(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_deref(&self) -> ();
    fn operator_arrow(&self) -> ();
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_int_ptr_>;
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_int_ptr_>;
    fn operator_post_inc_11(&self, mut _a0: ()) -> contiguous_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: ()) -> contiguous_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: ()) -> Ptr<contiguous_iterator_int_ptr_>;
    fn operator_sub_assign(&self, n: ()) -> Ptr<contiguous_iterator_int_ptr_>;
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_int_ptr_>,
    ) -> Ptr<contiguous_iterator_int_ptr_>;
}
impl contiguous_iterator_int_ptr_Impl for Ptr<contiguous_iterator_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_arrow(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_int_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_int_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_int_ptr_>,
    ) -> Ptr<contiguous_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait contiguous_iterator_int_ptrptr_Impl {
    fn base(&self) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn operator_deref(&self) -> ();
    fn operator_arrow(&self) -> ();
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_int_ptrptr_>;
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_int_ptrptr_>;
    fn operator_post_inc_11(&self, mut _a0: ()) -> contiguous_iterator_int_ptrptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: ()) -> contiguous_iterator_int_ptrptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: ()) -> Ptr<contiguous_iterator_int_ptrptr_>;
    fn operator_sub_assign(&self, n: ()) -> Ptr<contiguous_iterator_int_ptrptr_>;
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_int_ptrptr_>,
    ) -> Ptr<contiguous_iterator_int_ptrptr_>;
}
impl contiguous_iterator_int_ptrptr_Impl for Ptr<contiguous_iterator_int_ptrptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_arrow(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_int_ptrptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_int_ptrptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_int_ptrptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_int_ptrptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_int_ptrptr_>,
    ) -> Ptr<contiguous_iterator_int_ptrptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
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
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp17_output_iterator_int_ptr_ {
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
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_random_access_iterator_int_ptr_>,
    ) -> Ptr<cpp20_random_access_iterator_int_ptr_>;
}
impl cpp20_random_access_iterator_int_ptr_Impl for Ptr<cpp20_random_access_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
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
pub trait random_access_iterator_MoveOnly_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_MoveOnly_ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_MoveOnly_ptr_>;
    fn operator_post_inc_9(&self, mut _a0: ()) -> random_access_iterator_MoveOnly_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> random_access_iterator_MoveOnly_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: ()) -> Ptr<random_access_iterator_MoveOnly_ptr_>;
    fn operator_sub_assign(&self, n: ()) -> Ptr<random_access_iterator_MoveOnly_ptr_>;
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_MoveOnly_ptr_>,
    ) -> Ptr<random_access_iterator_MoveOnly_ptr_>;
}
impl random_access_iterator_MoveOnly_ptr_Impl for Ptr<random_access_iterator_MoveOnly_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_MoveOnly_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_MoveOnly_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<random_access_iterator_MoveOnly_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<random_access_iterator_MoveOnly_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() - n)
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
pub trait random_access_iterator_char_ptrptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_char_ptrptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_char_ptrptr_>;
    fn operator_post_inc_9(&self, mut _a0: ()) -> random_access_iterator_char_ptrptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> random_access_iterator_char_ptrptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: ()) -> Ptr<random_access_iterator_char_ptrptr_>;
    fn operator_sub_assign(&self, n: ()) -> Ptr<random_access_iterator_char_ptrptr_>;
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_char_ptrptr_>,
    ) -> Ptr<random_access_iterator_char_ptrptr_>;
}
impl random_access_iterator_char_ptrptr_Impl for Ptr<random_access_iterator_char_ptrptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_char_ptrptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_char_ptrptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<random_access_iterator_char_ptrptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<random_access_iterator_char_ptrptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_char_ptrptr_>,
    ) -> Ptr<random_access_iterator_char_ptrptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait random_access_iterator_const_char_ptrptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_char_ptrptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_const_char_ptrptr_>;
    fn operator_post_inc_9(&self, mut _a0: ()) -> random_access_iterator_const_char_ptrptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> random_access_iterator_const_char_ptrptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: ()) -> Ptr<random_access_iterator_const_char_ptrptr_>;
    fn operator_sub_assign(&self, n: ()) -> Ptr<random_access_iterator_const_char_ptrptr_>;
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_const_char_ptrptr_>,
    ) -> Ptr<random_access_iterator_const_char_ptrptr_>;
}
impl random_access_iterator_const_char_ptrptr_Impl
    for Ptr<random_access_iterator_const_char_ptrptr_>
{
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_char_ptrptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_const_char_ptrptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<random_access_iterator_const_char_ptrptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<random_access_iterator_const_char_ptrptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_const_char_ptrptr_>,
    ) -> Ptr<random_access_iterator_const_char_ptrptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_int_ptr_>;
    fn operator_post_inc_9(&self, mut _a0: ()) -> random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: ()) -> Ptr<random_access_iterator_int_ptr_>;
    fn operator_sub_assign(&self, n: ()) -> Ptr<random_access_iterator_int_ptr_>;
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_int_ptr_>,
    ) -> Ptr<random_access_iterator_int_ptr_>;
}
impl random_access_iterator_int_ptr_Impl for Ptr<random_access_iterator_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<random_access_iterator_int_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<random_access_iterator_int_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() - n)
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
pub trait random_access_iterator_int_ptrptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptrptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_int_ptrptr_>;
    fn operator_post_inc_9(&self, mut _a0: ()) -> random_access_iterator_int_ptrptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> random_access_iterator_int_ptrptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: ()) -> Ptr<random_access_iterator_int_ptrptr_>;
    fn operator_sub_assign(&self, n: ()) -> Ptr<random_access_iterator_int_ptrptr_>;
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_int_ptrptr_>,
    ) -> Ptr<random_access_iterator_int_ptrptr_>;
}
impl random_access_iterator_int_ptrptr_Impl for Ptr<random_access_iterator_int_ptrptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptrptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_int_ptrptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<random_access_iterator_int_ptrptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<random_access_iterator_int_ptrptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_int_ptrptr_>,
    ) -> Ptr<random_access_iterator_int_ptrptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
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
pub fn __cpp2rust_init_globals() {
    let _ = LargeN_81.with(|_| ());
}
