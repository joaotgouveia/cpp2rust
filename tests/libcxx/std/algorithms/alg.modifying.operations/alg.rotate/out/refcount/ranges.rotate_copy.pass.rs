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
pub struct types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_output_iterator_int_ptr__
{}
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
#[byte_size(1)]
pub struct types_concatenate_types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr____types_type_list_bidirectional_iterator_const_int_ptr___
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr____types_type_list_forward_iterator_const_int_ptr___
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr____types_type_list_bidirectional_iterator_int_ptr___
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr____types_type_list_forward_iterator_int_ptr___
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr____types_type_list_cpp20_output_iterator_int_ptr___
{}
pub fn for_each_0(
    mut _a0 : types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr__,
    f: FnPtr<Generic>,
) {
    let f: Value<FnPtr<Generic>> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ (*f.borrow()).spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            ({ (*f.borrow()).spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            ({ (*f.borrow()).spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            ({ (*f.borrow()).spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            ({ (*f.borrow()).spec::<fn()>(4).call() });
            0_()
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4)
    });
}
pub fn for_each_2(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_output_iterator_int_ptr__,
    f: FnPtr<Generic>,
) {
    let f: Value<FnPtr<Generic>> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ (*f.borrow()).spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            ({ (*f.borrow()).spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            ({ (*f.borrow()).spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            ({ (*f.borrow()).spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            ({ (*f.borrow()).spec::<fn()>(4).call() });
            0_()
        };
        let _arg5: () = {
            ({ (*f.borrow()).spec::<fn()>(5).call() });
            0_()
        };
        swallow_3(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub fn swallow_3(mut _a0: (), mut _a1: (), mut _a2: (), mut _a3: (), mut _a4: (), mut _a5: ()) {}
pub fn swallow_1(mut _a0: (), mut _a1: (), mut _a2: (), mut _a3: (), mut _a4: ()) {}
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
pub fn base_4(i: Ptr<forward_iterator_int_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
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
    x: Ptr<forward_iterator_const_int_ptr_>,
    y: Ptr<forward_iterator_const_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
pub fn base_6(i: Ptr<forward_iterator_const_int_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct forward_iterator_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_int_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
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
pub fn base_7(i: Ptr<bidirectional_iterator_int_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
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
pub fn operator_ne_8(
    x: Ptr<bidirectional_iterator_const_int_ptr_>,
    y: Ptr<bidirectional_iterator_const_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
pub fn base_9(i: Ptr<bidirectional_iterator_const_int_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct bidirectional_iterator_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_const_int_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
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
pub fn operator_sub_10(
    mut x: random_access_iterator_const_int_ptr_,
    mut y: random_access_iterator_const_int_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn base_11(i: Ptr<random_access_iterator_const_int_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct random_access_iterator_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_const_int_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
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
pub fn base_12(i: Ptr<random_access_iterator_int_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
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
pub fn operator_sub_13(
    mut x: contiguous_iterator_const_int_ptr_,
    mut y: contiguous_iterator_const_int_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn base_14(i: Ptr<contiguous_iterator_const_int_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_int_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<contiguous_iterator_const_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
pub fn base_15(i: Ptr<contiguous_iterator_int_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
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
pub fn base_16(i: Ptr<()>) -> Ptr<()> {
    let i: Value<Ptr<()>> = Rc::new(RefCell::new(i));
    return (*i.borrow()).clone();
}
pub fn base_17(i: Ptr<()>) -> Ptr<()> {
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
pub fn base_18(i: Ptr<cpp20_output_iterator_int_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
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
    pub fn new(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
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
#[byte_size(1)]
pub struct sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int__ {
    #[offset(0)]
    #[byte_size(1)]
    base_: InputIteratorNotDerivedFromGeneric_int_,
}
impl sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct sentinel_wrapper_InputIteratorNotIndirectlyReadable_ {
    #[offset(0)]
    #[byte_size(1)]
    base_: InputIteratorNotIndirectlyReadable,
}
impl sentinel_wrapper_InputIteratorNotIndirectlyReadable_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct sentinel_wrapper_InputIteratorNotInputOrOutputIterator_ {
    #[offset(0)]
    #[byte_size(1)]
    base_: InputIteratorNotInputOrOutputIterator,
}
impl sentinel_wrapper_InputIteratorNotInputOrOutputIterator_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct sentinel_wrapper_ForwardIteratorNotDerivedFrom_ {
    #[offset(0)]
    #[byte_size(1)]
    base_: ForwardIteratorNotDerivedFrom,
}
impl sentinel_wrapper_ForwardIteratorNotDerivedFrom_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct sentinel_wrapper_ForwardIteratorNotIncrementable_ {
    #[offset(0)]
    #[byte_size(1)]
    base_: ForwardIteratorNotIncrementable,
}
impl sentinel_wrapper_ForwardIteratorNotIncrementable_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct sentinel_wrapper_BidirectionalIteratorNotDerivedFrom_ {
    #[offset(0)]
    #[byte_size(1)]
    base_: BidirectionalIteratorNotDerivedFrom,
}
impl sentinel_wrapper_BidirectionalIteratorNotDerivedFrom_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct sentinel_wrapper_BidirectionalIteratorNotDecrementable_ {
    #[offset(0)]
    #[byte_size(1)]
    base_: BidirectionalIteratorNotDecrementable,
}
impl sentinel_wrapper_BidirectionalIteratorNotDecrementable_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_int_ptr_ {}
pub fn operator_eq_19(
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
        operator_eq_19(__lhs.clone(), __rhs.clone())
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
pub fn operator_add_20(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_21(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_20(_i, n)
    });
}
pub fn iter_swap_22(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_23(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_24(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_25(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
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
        operator_cmp_25(__lhs.clone(), __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for adl_Iterator {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_24(__lhs.clone(), __rhs.clone())
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_int_ptr__int_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_InputIteratorNotDerivedFromGeneric_int___sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int___
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_InputIteratorNotIndirectlyReadable__sentinel_wrapper_InputIteratorNotIndirectlyReadable__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_InputIteratorNotInputOrOutputIterator__sentinel_wrapper_InputIteratorNotInputOrOutputIterator__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_ForwardIteratorNotDerivedFrom__sentinel_wrapper_ForwardIteratorNotDerivedFrom__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_ForwardIteratorNotIncrementable__sentinel_wrapper_ForwardIteratorNotIncrementable__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_BidirectionalIteratorNotDerivedFrom__sentinel_wrapper_BidirectionalIteratorNotDerivedFrom__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_BidirectionalIteratorNotDecrementable__sentinel_wrapper_BidirectionalIteratorNotDecrementable__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_int_ptr__sentinel_wrapper_int_ptr__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_int_ptr__SentinelForNotSemiregular_ {}
const _: () = assert!(
    true,
    "std::ranges::contiguous_range<UncheckedRange<int*, int*>>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct InputIteratorNotDerivedFromGeneric_int_ {}
const _: () = assert!(
    true,
    "std::input_or_output_iterator<InputIteratorNotDerivedFrom>"
);
const _: () = assert!(
    true,
    "std::indirectly_readable<InputIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::input_iterator<InputIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::input_range<InputRangeNotDerivedFrom>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct InputIteratorNotIndirectlyReadable {}
const _: () = assert!(
    true,
    "std::input_or_output_iterator<InputIteratorNotIndirectlyReadable>"
);
const _: () = assert!(
    !(false),
    "!std::indirectly_readable<InputIteratorNotIndirectlyReadable>"
);
const _: () = assert!(
    !(false),
    "!std::input_iterator<InputIteratorNotIndirectlyReadable>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::input_range<InputRangeNotIndirectlyReadable>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct InputIteratorNotInputOrOutputIterator {}
const _: () = assert!(
    !(false),
    "!std::input_or_output_iterator<InputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(
    true,
    "std::indirectly_readable<InputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(
    !(false),
    "!std::input_iterator<InputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::input_range<InputRangeNotInputOrOutputIterator>"
);
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct IndirectUnaryPredicateNotCopyConstructible {}
const _: () = assert!(
    true,
    "std::predicate<IndirectUnaryPredicateNotCopyConstructible, int&>"
);
const _: () = assert!(
    !(false),
    "!std::indirect_unary_predicate<IndirectUnaryPredicateNotCopyConstructible, int*>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct IndirectUnaryPredicateNotPredicate {}
const _: () = assert!(
    !(false),
    "!std::predicate<IndirectUnaryPredicateNotPredicate, int&>"
);
const _: () = assert!(
    !(false),
    "!std::indirect_unary_predicate<IndirectUnaryPredicateNotPredicate, int*>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct SentinelForNotSemiregular {}
const _: () = assert!(
    true,
    "std::input_or_output_iterator<SentinelForNotSemiregular>"
);
const _: () = assert!(!(false), "!std::semiregular<SentinelForNotSemiregular>");
const _: () = assert!(
    !(false),
    "!std::sentinel_for<SentinelForNotSemiregular, cpp20_input_iterator<int*>>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct SentinelForNotWeaklyEqualityComparableWith {}
const _: () = assert!(
    true,
    "std::input_or_output_iterator<SentinelForNotWeaklyEqualityComparableWith>"
);
const _: () = assert!(
    true,
    "std::semiregular<SentinelForNotWeaklyEqualityComparableWith>"
);
const _: () = assert!(
    !(false),
    "!std::sentinel_for<SentinelForNotWeaklyEqualityComparableWith, cpp20_input_iterator<int*>>"
);
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct WeaklyIncrementableNotMovable {}
const _: () = assert!(!(false), "!std::movable<WeaklyIncrementableNotMovable>");
const _: () = assert!(
    !(false),
    "!std::weakly_incrementable<WeaklyIncrementableNotMovable>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ForwardIteratorNotDerivedFrom {}
impl CppEq for ForwardIteratorNotDerivedFrom {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        ForwardIteratorNotDerivedFromImpl::operator_eq(__lhs, __rhs.clone())
    }
}
const _: () = assert!(true, "std::input_iterator<ForwardIteratorNotDerivedFrom>");
const _: () = assert!(true, "std::incrementable<ForwardIteratorNotDerivedFrom>");
const _: () = assert!(
    true,
    "std::sentinel_for<ForwardIteratorNotDerivedFrom, ForwardIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::forward_iterator<ForwardIteratorNotDerivedFrom>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ForwardIteratorNotIncrementable {}
impl CppEq for ForwardIteratorNotIncrementable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        ForwardIteratorNotIncrementableImpl::operator_eq(__lhs, __rhs.clone())
    }
}
const _: () = assert!(true, "std::input_iterator<ForwardIteratorNotIncrementable>");
const _: () = assert!(
    !(false),
    "!std::incrementable<ForwardIteratorNotIncrementable>"
);
const _: () = assert!(
    true,
    "std::sentinel_for<ForwardIteratorNotIncrementable, ForwardIteratorNotIncrementable>"
);
const _: () = assert!(
    !(false),
    "!std::forward_iterator<ForwardIteratorNotIncrementable>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct BidirectionalIteratorNotDerivedFrom {}
impl CppEq for BidirectionalIteratorNotDerivedFrom {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        BidirectionalIteratorNotDerivedFromImpl::operator_eq(__lhs, __rhs.clone())
    }
}
const _: () = assert!(
    true,
    "std::forward_iterator<BidirectionalIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::bidirectional_iterator<BidirectionalIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::bidirectional_range<BidirectionalRangeNotDerivedFrom>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct BidirectionalIteratorNotDecrementable {}
impl CppEq for BidirectionalIteratorNotDecrementable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        BidirectionalIteratorNotDecrementableImpl::operator_eq(__lhs, __rhs.clone())
    }
}
const _: () = assert!(
    true,
    "std::forward_iterator<BidirectionalIteratorNotDecrementable>"
);
const _: () = assert!(
    !(false),
    "!std::bidirectional_iterator<BidirectionalIteratorNotDecrementable>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::bidirectional_range<BidirectionalRangeNotDecrementable>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct PermutableNotForwardIterator {}
const _: () = assert!(true, "std::input_iterator<PermutableNotForwardIterator>");
const _: () = assert!(
    !(false),
    "!std::forward_iterator<PermutableNotForwardIterator>"
);
const _: () = assert!(!(false), "!std::permutable<PermutableNotForwardIterator>");
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct PermutableNotSwappable_NotSwappable {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct PermutableNotSwappable {}
impl CppEq for PermutableNotSwappable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        PermutableNotSwappableImpl::operator_eq(__lhs, __rhs.clone())
    }
}
const _: () = assert!(true, "std::input_iterator<PermutableNotSwappable>");
const _: () = assert!(true, "std::forward_iterator<PermutableNotSwappable>");
const _: () = assert!(!(false), "!std::permutable<PermutableNotSwappable>");
const _: () = assert!(
    !(false),
    "!std::indirectly_swappable<PermutableNotSwappable>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct OutputIteratorNotInputOrOutputIterator {}
const _: () = assert!(
    !(false),
    "!std::input_or_output_iterator<OutputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(
    true,
    "std::indirectly_writable<OutputIteratorNotInputOrOutputIterator, int>"
);
const _: () = assert!(
    !(false),
    "!std::output_iterator<OutputIteratorNotInputOrOutputIterator, int>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::output_range<OutputRangeNotInputOrOutputIterator, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct OutputIteratorNotIndirectlyWritable {}
const _: () = assert!(
    true,
    "std::input_or_output_iterator<OutputIteratorNotIndirectlyWritable>"
);
const _: () = assert!(
    !(false),
    "!std::indirectly_writable<OutputIteratorNotIndirectlyWritable, int>"
);
const _: () = assert!(
    !(false),
    "!std::output_iterator<OutputIteratorNotIndirectlyWritable, int>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::output_range<OutputIteratorNotIndirectlyWritable, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct IndirectBinaryPredicateNotIndirectlyReadable {}
const _: () = assert!(
    !(false),
    "!std::indirect_binary_predicate<std::ranges::equal_to, IndirectBinaryPredicateNotIndirectlyReadable, int*>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RandomAccessIteratorNotDerivedFrom {}
impl CppLess for RandomAccessIteratorNotDerivedFrom {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        RandomAccessIteratorNotDerivedFromImpl::operator_cmp(__lhs, __rhs.clone())
            == std::cmp::Ordering::Less
    }
}
impl CppEq for RandomAccessIteratorNotDerivedFrom {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        RandomAccessIteratorNotDerivedFromImpl::operator_cmp(__lhs, __rhs.clone())
            == std::cmp::Ordering::Equal
    }
}
const _: () = assert!(
    true,
    "std::bidirectional_iterator<RandomAccessIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::random_access_iterator<RandomAccessIteratorNotDerivedFrom>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RandomAccessIteratorBadIndex {}
impl CppLess for RandomAccessIteratorBadIndex {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        RandomAccessIteratorBadIndexImpl::operator_cmp(__lhs, __rhs.clone())
            == std::cmp::Ordering::Less
    }
}
impl CppEq for RandomAccessIteratorBadIndex {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        RandomAccessIteratorBadIndexImpl::operator_cmp(__lhs, __rhs.clone())
            == std::cmp::Ordering::Equal
    }
}
const _: () = assert!(
    true,
    "std::bidirectional_iterator<RandomAccessIteratorBadIndex>"
);
const _: () = assert!(
    !(false),
    "!std::random_access_iterator<RandomAccessIteratorBadIndex>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RandomAccessIteratorBadDifferenceType {}
impl CppLess for RandomAccessIteratorBadDifferenceType {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        RandomAccessIteratorBadDifferenceTypeImpl::operator_cmp(__lhs, __rhs.clone())
            == std::cmp::Ordering::Less
    }
}
impl CppEq for RandomAccessIteratorBadDifferenceType {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        RandomAccessIteratorBadDifferenceTypeImpl::operator_cmp(__lhs, __rhs.clone())
            == std::cmp::Ordering::Equal
    }
}
const _: () = assert!(true, "std::regular<RandomAccessIteratorBadDifferenceType>");
const _: () = assert!(
    !(false),
    "!std::weakly_incrementable<RandomAccessIteratorBadDifferenceType>"
);
const _: () = assert!(
    !(false),
    "!std::random_access_iterator<RandomAccessIteratorBadDifferenceType>"
);
const _: () = assert!(true, "HasRotateCopyIt<int*>");
const _: () = assert!(!(false), "!HasRotateCopyIt<ForwardIteratorNotDerivedFrom>");
const _: () = assert!(
    !(false),
    "!HasRotateCopyIt<ForwardIteratorNotIncrementable>"
);
const _: () = assert!(
    !(false),
    "!HasRotateCopyIt<int*, SentinelForNotSemiregular>"
);
const _: () = assert!(
    !(false),
    "!HasRotateCopyIt<int*, SentinelForNotWeaklyEqualityComparableWith>"
);
const _: () = assert!(
    !(false),
    "!HasRotateCopyIt<int*, OutputIteratorNotIndirectlyWritable>"
);
const _: () = assert!(
    !(false),
    "!HasRotateCopyIt<int*, OutputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(true, "HasRotateCopyR<UncheckedRange<int*>>");
const _: () = assert!(!(false), "!HasRotateCopyR<ForwardRangeNotDerivedFrom>");
const _: () = assert!(!(false), "!HasRotateCopyR<ForwardRangeNotIncrementable>");
const _: () = assert!(
    !(false),
    "!HasRotateCopyR<UncheckedRange<int*, SentinelForNotSemiregular>>"
);
const _: () = assert!(
    !(false),
    "!HasRotateCopyR<UncheckedRange<int*>, OutputIteratorNotIndirectlyWritable>"
);
const _: () = assert!(
    !(false),
    "!HasRotateCopyR<UncheckedRange<int*>, OutputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(
    true,
    "std::is_same_v<std::ranges::rotate_copy_result<int, int>, std::ranges::in_out_result<int, int>>"
);
pub fn test_26(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_28(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_29(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_30(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_31(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_32(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_33(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_34(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_35(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_36(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_37(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_38(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_39(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_40(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_41(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_42(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_43(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_44(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_45(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_46(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_47(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_48(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_49(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_50(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_51(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_52(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_53(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_54(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_55(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_56(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = unimplemented!();
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___last: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: Ptr<()> = unimplemented!().offset((middle) as isize);
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_57(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_58(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_59(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_60(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_61(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_62(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_63(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_64(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_65(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_66(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_67(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_68(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_69(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_70(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_71(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_72(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_73(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_74(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_75(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_76(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_77(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_78(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_79(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_80(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_81(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_82(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_83(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_84(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_85(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_86(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_87(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_88(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_89(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_90(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_91(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_92(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_93(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_94(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_95(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_96(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_97(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_98(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_99(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_100(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_101(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_102(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_103(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_104(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_105(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_106(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_107(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_108(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_109(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_110(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_111(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_112(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_113(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_114(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_115(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_116(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_117(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_118(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_119(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_120(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_121(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_122(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_123(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_124(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_125(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_126(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_127(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_128(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_129(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_130(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_131(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_132(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_133(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_134(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_135(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_136(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_137(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_138(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_139(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_140(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_141(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_142(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_143(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_144(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_145(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_146(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_147(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_148(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_149(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_150(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_151(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: Ptr<()> = unimplemented!();
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_152(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_153(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_154(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_155(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_156(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_157(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_158(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_159(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_160(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_161(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_162(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_163(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_164(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_165(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_166(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_167(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_168(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_169(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_170(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_171(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_172(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_173(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_174(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_175(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_176(value: (), mut middle: (), expected: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({ unimplemented!() });
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___last: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first, ___middle, ___last, ___result,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let ___middle: forward_iterator_const_int_ptr_ =
                    forward_iterator_const_int_ptr_::new_1({
                        unimplemented!().offset((middle) as isize)
                    });
                let ___result: cpp20_output_iterator_int_ptr_ =
                    cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    ___middle,
                    ___result,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_iterators_177() {
    ({ test_26(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_28(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_29(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_30(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_31(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_31(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_31(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_31(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_178() {
    ({ test_32(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_33(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_34(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_35(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_36(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_36(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_36(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_36(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_179() {
    ({ test_37(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_38(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_39(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_40(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_41(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_41(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_41(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_41(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_180() {
    ({ test_42(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_43(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_44(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_45(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_46(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_46(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_46(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_46(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_181() {
    ({ test_47(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_48(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_49(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_50(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_51(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_51(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_51(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_51(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_182() {
    ({ test_52(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_53(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_54(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_55(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_56(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_56(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_56(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_56(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_183() {
    ({ test_57(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_58(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_59(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_60(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_61(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_61(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_61(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_61(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_184() {
    ({ test_62(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_63(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_64(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_65(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_66(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_66(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_66(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_66(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_185() {
    ({ test_67(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_68(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_69(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_70(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_71(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_71(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_71(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_71(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_186() {
    ({ test_72(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_73(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_74(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_75(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_76(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_76(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_76(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_76(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_187() {
    ({ test_77(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_78(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_79(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_80(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_81(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_81(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_81(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_81(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_188() {
    ({ test_82(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_83(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_84(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_85(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_86(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_86(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_86(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_86(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_189() {
    ({ test_87(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_88(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_89(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_90(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_91(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_91(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_91(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_91(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_190() {
    ({ test_92(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_93(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_94(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_95(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_96(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_96(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_96(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_96(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_191() {
    ({ test_97(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_98(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_99(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_100(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_101(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_101(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_101(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_101(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_192() {
    ({ test_102(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_103(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_104(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_105(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_106(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_106(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_106(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_106(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_193() {
    ({ test_107(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_108(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_109(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_110(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_111(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_111(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_111(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_111(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_194() {
    ({ test_112(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_113(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_114(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_115(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_116(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_116(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_116(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_116(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_195() {
    ({ test_117(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_118(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_119(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_120(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_121(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_121(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_121(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_121(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_196() {
    ({ test_122(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_123(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_124(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_125(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_126(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_126(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_126(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_126(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_197() {
    ({ test_127(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_128(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_129(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_130(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_131(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_131(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_131(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_131(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_198() {
    ({ test_132(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_133(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_134(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_135(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_136(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_136(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_136(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_136(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_199() {
    ({ test_137(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_138(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_139(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_140(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_141(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_141(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_141(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_141(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_200() {
    ({ test_142(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_143(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_144(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_145(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_146(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_146(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_146(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_146(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_201() {
    ({ test_147(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_148(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_149(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_150(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_151(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_151(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_151(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_151(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_202() {
    ({ test_152(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_153(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_154(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_155(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_156(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_156(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_156(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_156(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_203() {
    ({ test_157(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_158(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_159(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_160(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_161(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_161(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_161(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_161(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_204() {
    ({ test_162(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_163(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_164(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_165(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_166(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_166(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_166(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_166(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_205() {
    ({ test_167(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_168(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_169(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_170(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_171(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_171(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_171(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_171(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_iterators_206() {
    ({ test_172(unimplemented!(), 2_(), unimplemented!()) });
    ({ test_173(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_174(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_175(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_176(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_176(unimplemented!(), 1_(), unimplemented!()) });
    ({ test_176(unimplemented!(), 6_(), unimplemented!()) });
    ({ test_176(unimplemented!(), 7_(), unimplemented!()) });
}
pub fn test_207() -> () {
    ({
        for_each_0 ( <types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr__  >::default()  , lambda! ( Generic , { } , | | { ( { for_each_2 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_output_iterator_int_ptr__  >::default()  , lambda! ( Generic , { } , | | { ( { test_iterators_177 ( ) } )  ;
 } , | | { ( { test_iterators_178 ( ) } )  ;
 } , | | { ( { test_iterators_179 ( ) } )  ;
 } , | | { ( { test_iterators_180 ( ) } )  ;
 } , | | { ( { test_iterators_181 ( ) } )  ;
 } , | | { ( { test_iterators_182 ( ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_208 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_output_iterator_int_ptr__  >::default()  , lambda! ( Generic , { } , | | { ( { test_iterators_183 ( ) } )  ;
 } , | | { ( { test_iterators_184 ( ) } )  ;
 } , | | { ( { test_iterators_185 ( ) } )  ;
 } , | | { ( { test_iterators_186 ( ) } )  ;
 } , | | { ( { test_iterators_187 ( ) } )  ;
 } , | | { ( { test_iterators_188 ( ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_209 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_output_iterator_int_ptr__  >::default()  , lambda! ( Generic , { } , | | { ( { test_iterators_189 ( ) } )  ;
 } , | | { ( { test_iterators_190 ( ) } )  ;
 } , | | { ( { test_iterators_191 ( ) } )  ;
 } , | | { ( { test_iterators_192 ( ) } )  ;
 } , | | { ( { test_iterators_193 ( ) } )  ;
 } , | | { ( { test_iterators_194 ( ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_210 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_output_iterator_int_ptr__  >::default()  , lambda! ( Generic , { } , | | { ( { test_iterators_195 ( ) } )  ;
 } , | | { ( { test_iterators_196 ( ) } )  ;
 } , | | { ( { test_iterators_197 ( ) } )  ;
 } , | | { ( { test_iterators_198 ( ) } )  ;
 } , | | { ( { test_iterators_199 ( ) } )  ;
 } , | | { ( { test_iterators_200 ( ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_211 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_output_iterator_int_ptr__  >::default()  , lambda! ( Generic , { } , | | { ( { test_iterators_201 ( ) } )  ;
 } , | | { ( { test_iterators_202 ( ) } )  ;
 } , | | { ( { test_iterators_203 ( ) } )  ;
 } , | | { ( { test_iterators_204 ( ) } )  ;
 } , | | { ( { test_iterators_205 ( ) } )  ;
 } , | | { ( { test_iterators_206 ( ) } )  ;
 } )  , ) } )  ;
 } )  , )
    });
    {
        {
            let c: Value<()> = Rc::new(RefCell::new(0_()));
            let a: Value<Box<[AssignmentCounter_212]>> = Rc::new(RefCell::new(Box::new([
                AssignmentCounter_212::new({ (c.as_pointer()) }),
                AssignmentCounter_212::new({ (c.as_pointer()) }),
                AssignmentCounter_212::new({ (c.as_pointer()) }),
                AssignmentCounter_212::new({ (c.as_pointer()) }),
            ])));
            let b: Value<Box<[AssignmentCounter_212]>> = Rc::new(RefCell::new(Box::new([
                AssignmentCounter_212::new({ (c.as_pointer()) }),
                AssignmentCounter_212::new({ (c.as_pointer()) }),
                AssignmentCounter_212::new({ (c.as_pointer()) }),
                AssignmentCounter_212::new({ (c.as_pointer()) }),
            ])));
            ({
                let ___first: Ptr<AssignmentCounter_212> =
                    (a.as_pointer() as Ptr<AssignmentCounter_212>);
                let ___middle: Ptr<AssignmentCounter_212> =
                    (a.as_pointer() as Ptr<AssignmentCounter_212>).offset((2_()) as isize);
                let ___last: Ptr<AssignmentCounter_212> =
                    (a.as_pointer() as Ptr<AssignmentCounter_212>).offset((4_()) as isize);
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___first,
                    ___middle,
                    ___last,
                    (b.as_pointer() as Ptr<AssignmentCounter_212>),
                )
            });
            unimplemented!();
        }
        {
            let c: Value<()> = Rc::new(RefCell::new(0_()));
            let a: Value<Box<[AssignmentCounter_212]>> = Rc::new(RefCell::new(Box::new([
                AssignmentCounter_212::new({ (c.as_pointer()) }),
                AssignmentCounter_212::new({ (c.as_pointer()) }),
                AssignmentCounter_212::new({ (c.as_pointer()) }),
                AssignmentCounter_212::new({ (c.as_pointer()) }),
            ])));
            let b: Value<Box<[AssignmentCounter_212]>> = Rc::new(RefCell::new(Box::new([
                AssignmentCounter_212::new({ (c.as_pointer()) }),
                AssignmentCounter_212::new({ (c.as_pointer()) }),
                AssignmentCounter_212::new({ (c.as_pointer()) }),
                AssignmentCounter_212::new({ (c.as_pointer()) }),
            ])));
            ({
                let ___r: Ptr<AssignmentCounter_212> = a.as_pointer();
                let ___middle: Ptr<AssignmentCounter_212> =
                    (a.as_pointer() as Ptr<AssignmentCounter_212>).offset((2_()) as isize);
                (*rotate_copy_27.with(Value::clone).borrow_mut())(
                    ___r,
                    ___middle,
                    (b.as_pointer() as Ptr<AssignmentCounter_212>),
                )
            });
            unimplemented!();
        }
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(8)]
pub struct AssignmentCounter_212 {
    #[offset(0)]
    #[byte_size(8)]
    pub counter: Ptr<()>,
}
impl AssignmentCounter_212 {
    pub fn new(mut counter_: Ptr<()>) -> Self {
        Self {
            counter: (counter_).clone(),
        }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_207() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait AssignmentCounter_212Impl {
    fn copy_assign(&self, _a0: Ptr<AssignmentCounter_212>) -> Ptr<AssignmentCounter_212>;
}
impl AssignmentCounter_212Impl for Ptr<AssignmentCounter_212> {
    fn copy_assign(&self, _a0: Ptr<AssignmentCounter_212>) -> Ptr<AssignmentCounter_212> {
        (*self)
            .with(|__s| __s.counter.clone())
            .with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait BidirectionalIteratorNotDecrementableImpl {
    fn operator_inc_1(&self) -> Ptr<BidirectionalIteratorNotDecrementable> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) -> BidirectionalIteratorNotDecrementable {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_eq(&self, _a0: Ptr<BidirectionalIteratorNotDecrementable>) -> ();
}
impl BidirectionalIteratorNotDecrementableImpl for Ptr<BidirectionalIteratorNotDecrementable> {
    fn operator_eq(&self, _a0: Ptr<BidirectionalIteratorNotDecrementable>) -> () {
        return true;
    }
}
pub trait BidirectionalIteratorNotDerivedFromImpl {
    fn operator_inc_1(&self) -> Ptr<BidirectionalIteratorNotDerivedFrom> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) -> BidirectionalIteratorNotDerivedFrom {
        unimplemented!()
    }
    fn operator_dec_3(&self) -> Ptr<BidirectionalIteratorNotDerivedFrom> {
        unimplemented!()
    }
    fn operator_post_dec_4(&self, mut _a0: ()) -> BidirectionalIteratorNotDerivedFrom {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_eq(&self, _a0: Ptr<BidirectionalIteratorNotDerivedFrom>) -> ();
}
impl BidirectionalIteratorNotDerivedFromImpl for Ptr<BidirectionalIteratorNotDerivedFrom> {
    fn operator_eq(&self, _a0: Ptr<BidirectionalIteratorNotDerivedFrom>) -> () {
        return true;
    }
}
pub trait ForwardIteratorNotDerivedFromImpl {
    fn operator_inc_1(&self) -> Ptr<ForwardIteratorNotDerivedFrom> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) -> ForwardIteratorNotDerivedFrom {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_eq(&self, _a0: Ptr<ForwardIteratorNotDerivedFrom>) -> ();
}
impl ForwardIteratorNotDerivedFromImpl for Ptr<ForwardIteratorNotDerivedFrom> {
    fn operator_eq(&self, _a0: Ptr<ForwardIteratorNotDerivedFrom>) -> () {
        return true;
    }
}
pub trait ForwardIteratorNotIncrementableImpl {
    fn operator_inc_1(&self) -> Ptr<ForwardIteratorNotIncrementable> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_eq(&self, _a0: Ptr<ForwardIteratorNotIncrementable>) -> ();
}
impl ForwardIteratorNotIncrementableImpl for Ptr<ForwardIteratorNotIncrementable> {
    fn operator_eq(&self, _a0: Ptr<ForwardIteratorNotIncrementable>) -> () {
        return true;
    }
}
pub trait IndirectBinaryPredicateNotIndirectlyReadableImpl {
    fn operator_inc_1(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl IndirectBinaryPredicateNotIndirectlyReadableImpl
    for Ptr<IndirectBinaryPredicateNotIndirectlyReadable>
{
}
pub trait IndirectUnaryPredicateNotCopyConstructibleImpl {
    fn operator_call(&self, mut _a0: ()) -> () {
        unimplemented!()
    }
}
impl IndirectUnaryPredicateNotCopyConstructibleImpl
    for Ptr<IndirectUnaryPredicateNotCopyConstructible>
{
}
pub trait IndirectUnaryPredicateNotPredicateImpl {
    fn operator_call(&self, _a0: Ptr<()>) -> () {
        unimplemented!()
    }
}
impl IndirectUnaryPredicateNotPredicateImpl for Ptr<IndirectUnaryPredicateNotPredicate> {}
pub trait InputIteratorNotDerivedFromGeneric_int_Impl {
    fn operator_inc_1(&self) -> Ptr<InputIteratorNotDerivedFromGeneric_int_> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl InputIteratorNotDerivedFromGeneric_int_Impl for Ptr<InputIteratorNotDerivedFromGeneric_int_> {}
pub trait InputIteratorNotIndirectlyReadableImpl {
    fn operator_inc_1(&self) -> Ptr<InputIteratorNotIndirectlyReadable> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl InputIteratorNotIndirectlyReadableImpl for Ptr<InputIteratorNotIndirectlyReadable> {}
pub trait InputIteratorNotInputOrOutputIteratorImpl {
    fn operator_inc_1(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl InputIteratorNotInputOrOutputIteratorImpl for Ptr<InputIteratorNotInputOrOutputIterator> {}
pub trait OutputIteratorNotIndirectlyWritableImpl {
    fn operator_inc_1(&self) -> Ptr<OutputIteratorNotIndirectlyWritable> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl OutputIteratorNotIndirectlyWritableImpl for Ptr<OutputIteratorNotIndirectlyWritable> {}
pub trait OutputIteratorNotInputOrOutputIteratorImpl {
    fn operator_inc_1(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl OutputIteratorNotInputOrOutputIteratorImpl for Ptr<OutputIteratorNotInputOrOutputIterator> {}
pub trait PermutableNotForwardIteratorImpl {
    fn operator_inc_1(&self) -> Ptr<PermutableNotForwardIterator> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl PermutableNotForwardIteratorImpl for Ptr<PermutableNotForwardIterator> {}
pub trait PermutableNotSwappableImpl {
    fn operator_inc_1(&self) -> Ptr<PermutableNotSwappable> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) -> PermutableNotSwappable {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<PermutableNotSwappable_NotSwappable> {
        unimplemented!()
    }
    fn operator_eq(&self, _a0: Ptr<PermutableNotSwappable>) -> ();
}
impl PermutableNotSwappableImpl for Ptr<PermutableNotSwappable> {
    fn operator_eq(&self, _a0: Ptr<PermutableNotSwappable>) -> () {
        return true;
    }
}
pub trait RandomAccessIteratorBadDifferenceTypeImpl {
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_index(&self, mut _a0: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_3(&self) -> Ptr<RandomAccessIteratorBadDifferenceType> {
        unimplemented!()
    }
    fn operator_dec_4(&self) -> Ptr<RandomAccessIteratorBadDifferenceType> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: ()) -> RandomAccessIteratorBadDifferenceType {
        unimplemented!()
    }
    fn operator_post_dec_6(&self, mut _a0: ()) -> RandomAccessIteratorBadDifferenceType {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut _a0: ()) -> Ptr<RandomAccessIteratorBadDifferenceType> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut _a0: ()) -> Ptr<RandomAccessIteratorBadDifferenceType> {
        unimplemented!()
    }
    fn operator_cmp(&self, _a0: Ptr<RandomAccessIteratorBadDifferenceType>) -> ();
    fn operator_eq(&self, _a0: Ptr<RandomAccessIteratorBadDifferenceType>) -> ();
}
impl RandomAccessIteratorBadDifferenceTypeImpl for Ptr<RandomAccessIteratorBadDifferenceType> {
    fn operator_cmp(&self, _a0: Ptr<RandomAccessIteratorBadDifferenceType>) -> () {
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<RandomAccessIteratorBadDifferenceType>) -> () {
        return true;
    }
}
pub trait RandomAccessIteratorBadIndexImpl {
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_index(&self, mut _a0: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_3(&self) -> Ptr<RandomAccessIteratorBadIndex> {
        unimplemented!()
    }
    fn operator_dec_4(&self) -> Ptr<RandomAccessIteratorBadIndex> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: ()) -> RandomAccessIteratorBadIndex {
        unimplemented!()
    }
    fn operator_post_dec_6(&self, mut _a0: ()) -> RandomAccessIteratorBadIndex {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut _a0: ()) -> Ptr<RandomAccessIteratorBadIndex> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut _a0: ()) -> Ptr<RandomAccessIteratorBadIndex> {
        unimplemented!()
    }
    fn operator_cmp(&self, _a0: Ptr<RandomAccessIteratorBadIndex>) -> ();
    fn operator_eq(&self, _a0: Ptr<RandomAccessIteratorBadIndex>) -> ();
}
impl RandomAccessIteratorBadIndexImpl for Ptr<RandomAccessIteratorBadIndex> {
    fn operator_cmp(&self, _a0: Ptr<RandomAccessIteratorBadIndex>) -> () {
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<RandomAccessIteratorBadIndex>) -> () {
        return true;
    }
}
pub trait RandomAccessIteratorNotDerivedFromImpl {
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_index(&self, mut _a0: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_3(&self) -> Ptr<RandomAccessIteratorNotDerivedFrom> {
        unimplemented!()
    }
    fn operator_dec_4(&self) -> Ptr<RandomAccessIteratorNotDerivedFrom> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: ()) -> RandomAccessIteratorNotDerivedFrom {
        unimplemented!()
    }
    fn operator_post_dec_6(&self, mut _a0: ()) -> RandomAccessIteratorNotDerivedFrom {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut _a0: ()) -> Ptr<RandomAccessIteratorNotDerivedFrom> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut _a0: ()) -> Ptr<RandomAccessIteratorNotDerivedFrom> {
        unimplemented!()
    }
    fn operator_cmp(&self, _a0: Ptr<RandomAccessIteratorNotDerivedFrom>) -> ();
    fn operator_eq(&self, _a0: Ptr<RandomAccessIteratorNotDerivedFrom>) -> ();
}
impl RandomAccessIteratorNotDerivedFromImpl for Ptr<RandomAccessIteratorNotDerivedFrom> {
    fn operator_cmp(&self, _a0: Ptr<RandomAccessIteratorNotDerivedFrom>) -> () {
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<RandomAccessIteratorNotDerivedFrom>) -> () {
        return true;
    }
}
pub trait SentinelForNotSemiregularImpl {
    fn operator_inc_2(&self) -> Ptr<SentinelForNotSemiregular> {
        unimplemented!()
    }
    fn operator_post_inc_3(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl SentinelForNotSemiregularImpl for Ptr<SentinelForNotSemiregular> {}
pub trait SentinelForNotWeaklyEqualityComparableWithImpl {
    fn operator_inc_1(&self) -> Ptr<SentinelForNotWeaklyEqualityComparableWith> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl SentinelForNotWeaklyEqualityComparableWithImpl
    for Ptr<SentinelForNotWeaklyEqualityComparableWith>
{
}
pub trait UncheckedRange_BidirectionalIteratorNotDecrementable__sentinel_wrapper_BidirectionalIteratorNotDecrementable__Impl
{
    fn begin(&self) -> BidirectionalIteratorNotDecrementable {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_BidirectionalIteratorNotDecrementable_ {
        unimplemented!()
    }
}
impl UncheckedRange_BidirectionalIteratorNotDecrementable__sentinel_wrapper_BidirectionalIteratorNotDecrementable__Impl for Ptr<UncheckedRange_BidirectionalIteratorNotDecrementable__sentinel_wrapper_BidirectionalIteratorNotDecrementable__> {
}
pub trait UncheckedRange_BidirectionalIteratorNotDerivedFrom__sentinel_wrapper_BidirectionalIteratorNotDerivedFrom__Impl
{
    fn begin(&self) -> BidirectionalIteratorNotDerivedFrom {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_BidirectionalIteratorNotDerivedFrom_ {
        unimplemented!()
    }
}
impl UncheckedRange_BidirectionalIteratorNotDerivedFrom__sentinel_wrapper_BidirectionalIteratorNotDerivedFrom__Impl for Ptr<UncheckedRange_BidirectionalIteratorNotDerivedFrom__sentinel_wrapper_BidirectionalIteratorNotDerivedFrom__> {
}
pub trait UncheckedRange_ForwardIteratorNotDerivedFrom__sentinel_wrapper_ForwardIteratorNotDerivedFrom__Impl
{
    fn begin(&self) -> ForwardIteratorNotDerivedFrom {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_ForwardIteratorNotDerivedFrom_ {
        unimplemented!()
    }
}
impl UncheckedRange_ForwardIteratorNotDerivedFrom__sentinel_wrapper_ForwardIteratorNotDerivedFrom__Impl for Ptr<UncheckedRange_ForwardIteratorNotDerivedFrom__sentinel_wrapper_ForwardIteratorNotDerivedFrom__> {
}
pub trait UncheckedRange_ForwardIteratorNotIncrementable__sentinel_wrapper_ForwardIteratorNotIncrementable__Impl
{
    fn begin(&self) -> ForwardIteratorNotIncrementable {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_ForwardIteratorNotIncrementable_ {
        unimplemented!()
    }
}
impl UncheckedRange_ForwardIteratorNotIncrementable__sentinel_wrapper_ForwardIteratorNotIncrementable__Impl for Ptr<UncheckedRange_ForwardIteratorNotIncrementable__sentinel_wrapper_ForwardIteratorNotIncrementable__> {
}
pub trait UncheckedRange_InputIteratorNotDerivedFromGeneric_int___sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int___Impl
{
    fn begin(&self) -> InputIteratorNotDerivedFromGeneric_int_ {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int__ {
        unimplemented!()
    }
}
impl UncheckedRange_InputIteratorNotDerivedFromGeneric_int___sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int___Impl for Ptr<UncheckedRange_InputIteratorNotDerivedFromGeneric_int___sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int___> {
}
pub trait UncheckedRange_InputIteratorNotIndirectlyReadable__sentinel_wrapper_InputIteratorNotIndirectlyReadable__Impl
{
    fn begin(&self) -> InputIteratorNotIndirectlyReadable {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_InputIteratorNotIndirectlyReadable_ {
        unimplemented!()
    }
}
impl UncheckedRange_InputIteratorNotIndirectlyReadable__sentinel_wrapper_InputIteratorNotIndirectlyReadable__Impl for Ptr<UncheckedRange_InputIteratorNotIndirectlyReadable__sentinel_wrapper_InputIteratorNotIndirectlyReadable__> {
}
pub trait UncheckedRange_InputIteratorNotInputOrOutputIterator__sentinel_wrapper_InputIteratorNotInputOrOutputIterator__Impl
{
    fn begin(&self) -> InputIteratorNotInputOrOutputIterator {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_InputIteratorNotInputOrOutputIterator_ {
        unimplemented!()
    }
}
impl UncheckedRange_InputIteratorNotInputOrOutputIterator__sentinel_wrapper_InputIteratorNotInputOrOutputIterator__Impl for Ptr<UncheckedRange_InputIteratorNotInputOrOutputIterator__sentinel_wrapper_InputIteratorNotInputOrOutputIterator__> {
}
pub trait UncheckedRange_int_ptr__SentinelForNotSemiregular_Impl {
    fn begin(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end(&self) -> SentinelForNotSemiregular {
        unimplemented!()
    }
}
impl UncheckedRange_int_ptr__SentinelForNotSemiregular_Impl
    for Ptr<UncheckedRange_int_ptr__SentinelForNotSemiregular_>
{
}
pub trait UncheckedRange_int_ptr__int_ptr_Impl {
    fn begin(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl UncheckedRange_int_ptr__int_ptr_Impl for Ptr<UncheckedRange_int_ptr__int_ptr_> {}
pub trait UncheckedRange_int_ptr__sentinel_wrapper_int_ptr__Impl {
    fn begin(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_int_ptr_ {
        unimplemented!()
    }
}
impl UncheckedRange_int_ptr__sentinel_wrapper_int_ptr__Impl
    for Ptr<UncheckedRange_int_ptr__sentinel_wrapper_int_ptr__>
{
}
pub trait WeaklyIncrementableNotMovableImpl {
    fn operator_inc_1(&self) -> Ptr<WeaklyIncrementableNotMovable> {
        unimplemented!()
    }
    fn operator_post_inc_2(&self, mut _a0: ()) {
        unimplemented!()
    }
}
impl WeaklyIncrementableNotMovableImpl for Ptr<WeaklyIncrementableNotMovable> {}
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
pub trait bidirectional_iterator_const_int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_const_int_ptr_>;
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_8(&self, mut _a0: ()) -> bidirectional_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_9(&self, mut _a0: ()) -> bidirectional_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_const_int_ptr_>,
    ) -> Ptr<bidirectional_iterator_const_int_ptr_>;
}
impl bidirectional_iterator_const_int_ptr_Impl for Ptr<bidirectional_iterator_const_int_ptr_> {
    fn operator_deref(&self) -> () {
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
pub trait bidirectional_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> ();
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
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_int_ptr_>,
    ) -> Ptr<bidirectional_iterator_int_ptr_>;
}
impl bidirectional_iterator_int_ptr_Impl for Ptr<bidirectional_iterator_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
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
pub trait contiguous_iterator_const_int_ptr_Impl {
    fn base(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_deref(&self) -> ();
    fn operator_arrow(&self) -> () {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_const_int_ptr_>;
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: ()) -> contiguous_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: ()) -> contiguous_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_const_int_ptr_>,
    ) -> Ptr<contiguous_iterator_const_int_ptr_>;
}
impl contiguous_iterator_const_int_ptr_Impl for Ptr<contiguous_iterator_const_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_const_int_ptr_>,
    ) -> Ptr<contiguous_iterator_const_int_ptr_> {
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
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_int_ptr_>,
    ) -> Ptr<contiguous_iterator_int_ptr_>;
}
impl contiguous_iterator_int_ptr_Impl for Ptr<contiguous_iterator_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
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
    fn operator_inc_5(&self) -> Ptr<cpp20_output_iterator_int_ptr_>;
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
    fn operator_inc_5(&self) -> Ptr<cpp20_output_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
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
pub trait forward_iterator_const_int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_int_ptr_>;
    fn operator_post_inc_7(&self, mut _a0: ()) -> forward_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<forward_iterator_const_int_ptr_>,
    ) -> Ptr<forward_iterator_const_int_ptr_>;
}
impl forward_iterator_const_int_ptr_Impl for Ptr<forward_iterator_const_int_ptr_> {
    fn operator_deref(&self) -> () {
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
pub trait forward_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_6(&self) -> Ptr<forward_iterator_int_ptr_>;
    fn operator_post_inc_7(&self, mut _a0: ()) -> forward_iterator_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(&self, _a0: Ptr<forward_iterator_int_ptr_>) -> Ptr<forward_iterator_int_ptr_>;
}
impl forward_iterator_int_ptr_Impl for Ptr<forward_iterator_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<forward_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
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
pub trait random_access_iterator_const_int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_int_ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: ()) -> random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_const_int_ptr_>,
    ) -> Ptr<random_access_iterator_const_int_ptr_>;
}
impl random_access_iterator_const_int_ptr_Impl for Ptr<random_access_iterator_const_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
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
pub trait random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> ();
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
pub trait sentinel_wrapper_BidirectionalIteratorNotDecrementable_Impl {
    fn operator_eq(&self, other: Ptr<BidirectionalIteratorNotDecrementable>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_BidirectionalIteratorNotDecrementable_Impl
    for Ptr<sentinel_wrapper_BidirectionalIteratorNotDecrementable_>
{
}
pub trait sentinel_wrapper_BidirectionalIteratorNotDerivedFrom_Impl {
    fn operator_eq(&self, other: Ptr<BidirectionalIteratorNotDerivedFrom>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_BidirectionalIteratorNotDerivedFrom_Impl
    for Ptr<sentinel_wrapper_BidirectionalIteratorNotDerivedFrom_>
{
}
pub trait sentinel_wrapper_ForwardIteratorNotDerivedFrom_Impl {
    fn operator_eq(&self, other: Ptr<ForwardIteratorNotDerivedFrom>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_ForwardIteratorNotDerivedFrom_Impl
    for Ptr<sentinel_wrapper_ForwardIteratorNotDerivedFrom_>
{
}
pub trait sentinel_wrapper_ForwardIteratorNotIncrementable_Impl {
    fn operator_eq(&self, other: Ptr<ForwardIteratorNotIncrementable>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_ForwardIteratorNotIncrementable_Impl
    for Ptr<sentinel_wrapper_ForwardIteratorNotIncrementable_>
{
}
pub trait sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int__Impl {
    fn operator_eq(&self, other: Ptr<InputIteratorNotDerivedFromGeneric_int_>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int__Impl
    for Ptr<sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int__>
{
}
pub trait sentinel_wrapper_InputIteratorNotIndirectlyReadable_Impl {
    fn operator_eq(&self, other: Ptr<InputIteratorNotIndirectlyReadable>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_InputIteratorNotIndirectlyReadable_Impl
    for Ptr<sentinel_wrapper_InputIteratorNotIndirectlyReadable_>
{
}
pub trait sentinel_wrapper_InputIteratorNotInputOrOutputIterator_Impl {
    fn operator_eq(&self, other: Ptr<InputIteratorNotInputOrOutputIterator>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_InputIteratorNotInputOrOutputIterator_Impl
    for Ptr<sentinel_wrapper_InputIteratorNotInputOrOutputIterator_>
{
}
pub trait sentinel_wrapper_int_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_int_ptr_Impl for Ptr<sentinel_wrapper_int_ptr_> {}
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
