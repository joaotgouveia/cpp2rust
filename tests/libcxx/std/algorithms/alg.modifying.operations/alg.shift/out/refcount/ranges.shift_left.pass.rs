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
pub struct types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__
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
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr____types_type_list_bidirectional_iterator_int_ptr___
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr____types_type_list_forward_iterator_int_ptr___
{}
pub fn for_each_0(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__,
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
pub fn swallow_1(mut _a0: (), mut _a1: (), mut _a2: (), mut _a3: (), mut _a4: ()) {}
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
pub fn operator_eq_2(x: Ptr<forward_iterator_int_ptr_>, y: Ptr<forward_iterator_int_ptr_>) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_3(x: Ptr<forward_iterator_int_ptr_>, y: Ptr<forward_iterator_int_ptr_>) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
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
impl std::cmp::PartialEq for forward_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_2(
                Rc::new(RefCell::new(forward_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: self.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
                Rc::new(RefCell::new(forward_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: other.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_int_ptr_ {}
pub fn operator_eq_5(
    x: Ptr<bidirectional_iterator_int_ptr_>,
    y: Ptr<bidirectional_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_6(
    x: Ptr<bidirectional_iterator_int_ptr_>,
    y: Ptr<bidirectional_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
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
impl std::cmp::PartialEq for bidirectional_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_5(
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: self.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: other.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_int_ptr_ {}
pub fn operator_add_8(
    x: random_access_iterator_int_ptr_,
    mut n: (),
) -> random_access_iterator_int_ptr_ {
    let x: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_int_ptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return random_access_iterator_int_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_9(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn operator_eq_10(
    x: Ptr<random_access_iterator_int_ptr_>,
    y: Ptr<random_access_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_11(
    x: Ptr<random_access_iterator_int_ptr_>,
    y: Ptr<random_access_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
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
impl std::cmp::PartialEq for random_access_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_10(
                Rc::new(RefCell::new(random_access_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: self.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
                Rc::new(RefCell::new(random_access_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: other.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_int_ptr_ {}
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
pub fn operator_add_13(x: contiguous_iterator_int_ptr_, mut n: ()) -> contiguous_iterator_int_ptr_ {
    let x: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(x));
    ({ contiguous_iterator_int_ptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return contiguous_iterator_int_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_14(
    mut x: contiguous_iterator_int_ptr_,
    mut y: contiguous_iterator_int_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn operator_eq_15(
    x: Ptr<contiguous_iterator_int_ptr_>,
    y: Ptr<contiguous_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn base_16(i: Ptr<contiguous_iterator_int_ptr_>) -> Ptr<()> {
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
impl std::cmp::PartialEq for contiguous_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_15(
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: self.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: other.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_int_ptr_ {}
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
#[byte_size(1)]
pub struct sentinel_wrapper_PermutableNotForwardIterator_ {
    #[offset(0)]
    #[byte_size(1)]
    base_: PermutableNotForwardIterator,
}
impl sentinel_wrapper_PermutableNotForwardIterator_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct sentinel_wrapper_PermutableNotSwappable_ {
    #[offset(0)]
    #[byte_size(1)]
    base_: PermutableNotSwappable,
}
impl sentinel_wrapper_PermutableNotSwappable_ {}
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
            base_: ({ base_17((it.read())) }),
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
            base_: ({ base_16((it).clone()) }),
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
            base_: ({ base_12((it).clone()) }),
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
            base_: ({ base_7((it).clone()) }),
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
pub fn operator_sub_18(s: Ptr<sized_sentinel_int_ptr_>, i: Ptr<Ptr<()>>) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_17((i.read())) })) as ();
}
pub fn operator_sub_19(i: Ptr<Ptr<()>>, s: Ptr<sized_sentinel_int_ptr_>) -> () {
    return (({ base_17((i.read())) }) - s.with(|__s| __s.base_.clone())) as ();
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
            base_: ({ base_17((it.read())) }),
        }
    }
}
pub fn operator_sub_20(
    s: Ptr<sized_sentinel_contiguous_iterator_int_ptr__>,
    i: Ptr<contiguous_iterator_int_ptr_>,
) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_16((i).clone()) })) as ();
}
pub fn operator_sub_21(
    i: Ptr<contiguous_iterator_int_ptr_>,
    s: Ptr<sized_sentinel_contiguous_iterator_int_ptr__>,
) -> () {
    return (({ base_16((i).clone()) }) - s.with(|__s| __s.base_.clone())) as ();
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
            base_: ({ base_16((it).clone()) }),
        }
    }
}
pub fn operator_sub_22(
    s: Ptr<sized_sentinel_random_access_iterator_int_ptr__>,
    i: Ptr<random_access_iterator_int_ptr_>,
) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_12((i).clone()) })) as ();
}
pub fn operator_sub_23(
    i: Ptr<random_access_iterator_int_ptr_>,
    s: Ptr<sized_sentinel_random_access_iterator_int_ptr__>,
) -> () {
    return (({ base_12((i).clone()) }) - s.with(|__s| __s.base_.clone())) as ();
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
            base_: ({ base_12((it).clone()) }),
        }
    }
}
pub fn operator_sub_24(
    s: Ptr<sized_sentinel_bidirectional_iterator_int_ptr__>,
    i: Ptr<bidirectional_iterator_int_ptr_>,
) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_7((i).clone()) })) as ();
}
pub fn operator_sub_25(
    i: Ptr<bidirectional_iterator_int_ptr_>,
    s: Ptr<sized_sentinel_bidirectional_iterator_int_ptr__>,
) -> () {
    return (({ base_7((i).clone()) }) - s.with(|__s| __s.base_.clone())) as ();
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
            base_: ({ base_7((it).clone()) }),
        }
    }
}
pub fn operator_sub_26(
    s: Ptr<sized_sentinel_forward_iterator_int_ptr__>,
    i: Ptr<forward_iterator_int_ptr_>,
) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_4((i).clone()) })) as ();
}
pub fn operator_sub_27(
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
pub fn operator_eq_28(
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
            operator_eq_28(
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
pub fn operator_add_29(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_30(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_29(_i, n)
    });
}
pub fn iter_swap_31(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_32(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return unimplemented!();
}
pub fn operator_eq_33(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_34(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
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
            operator_cmp_34(
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
            operator_eq_33(
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
pub struct UncheckedRange_BidirectionalIteratorNotDerivedFrom__sentinel_wrapper_BidirectionalIteratorNotDerivedFrom__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_BidirectionalIteratorNotDecrementable__sentinel_wrapper_BidirectionalIteratorNotDecrementable__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_PermutableNotForwardIterator__sentinel_wrapper_PermutableNotForwardIterator__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_PermutableNotSwappable__sentinel_wrapper_PermutableNotSwappable__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_int_ptr__sentinel_wrapper_int_ptr__ {}
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
impl std::cmp::PartialEq for ForwardIteratorNotDerivedFrom {
    fn eq(&self, other: &Self) -> bool {
        {
            ForwardIteratorNotDerivedFromImpl::operator_eq(
                &Rc::new(RefCell::new(ForwardIteratorNotDerivedFrom {})).as_pointer(),
                Rc::new(RefCell::new(ForwardIteratorNotDerivedFrom {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for ForwardIteratorNotDerivedFrom {}
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
impl std::cmp::PartialEq for ForwardIteratorNotIncrementable {
    fn eq(&self, other: &Self) -> bool {
        {
            ForwardIteratorNotIncrementableImpl::operator_eq(
                &Rc::new(RefCell::new(ForwardIteratorNotIncrementable {})).as_pointer(),
                Rc::new(RefCell::new(ForwardIteratorNotIncrementable {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for ForwardIteratorNotIncrementable {}
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
impl std::cmp::PartialEq for BidirectionalIteratorNotDerivedFrom {
    fn eq(&self, other: &Self) -> bool {
        {
            BidirectionalIteratorNotDerivedFromImpl::operator_eq(
                &Rc::new(RefCell::new(BidirectionalIteratorNotDerivedFrom {})).as_pointer(),
                Rc::new(RefCell::new(BidirectionalIteratorNotDerivedFrom {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for BidirectionalIteratorNotDerivedFrom {}
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
impl std::cmp::PartialEq for BidirectionalIteratorNotDecrementable {
    fn eq(&self, other: &Self) -> bool {
        {
            BidirectionalIteratorNotDecrementableImpl::operator_eq(
                &Rc::new(RefCell::new(BidirectionalIteratorNotDecrementable {})).as_pointer(),
                Rc::new(RefCell::new(BidirectionalIteratorNotDecrementable {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for BidirectionalIteratorNotDecrementable {}
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
impl std::cmp::PartialEq for PermutableNotSwappable {
    fn eq(&self, other: &Self) -> bool {
        {
            PermutableNotSwappableImpl::operator_eq(
                &Rc::new(RefCell::new(PermutableNotSwappable {})).as_pointer(),
                Rc::new(RefCell::new(PermutableNotSwappable {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for PermutableNotSwappable {}
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
impl std::cmp::Ord for RandomAccessIteratorNotDerivedFrom {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            RandomAccessIteratorNotDerivedFromImpl::operator_cmp(
                &Rc::new(RefCell::new(RandomAccessIteratorNotDerivedFrom {})).as_pointer(),
                Rc::new(RefCell::new(RandomAccessIteratorNotDerivedFrom {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for RandomAccessIteratorNotDerivedFrom {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for RandomAccessIteratorNotDerivedFrom {
    fn eq(&self, other: &Self) -> bool {
        {
            RandomAccessIteratorNotDerivedFromImpl::operator_cmp(
                &Rc::new(RefCell::new(RandomAccessIteratorNotDerivedFrom {})).as_pointer(),
                Rc::new(RefCell::new(RandomAccessIteratorNotDerivedFrom {})).as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for RandomAccessIteratorNotDerivedFrom {}
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
impl std::cmp::Ord for RandomAccessIteratorBadIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            RandomAccessIteratorBadIndexImpl::operator_cmp(
                &Rc::new(RefCell::new(RandomAccessIteratorBadIndex {})).as_pointer(),
                Rc::new(RefCell::new(RandomAccessIteratorBadIndex {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for RandomAccessIteratorBadIndex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for RandomAccessIteratorBadIndex {
    fn eq(&self, other: &Self) -> bool {
        {
            RandomAccessIteratorBadIndexImpl::operator_cmp(
                &Rc::new(RefCell::new(RandomAccessIteratorBadIndex {})).as_pointer(),
                Rc::new(RefCell::new(RandomAccessIteratorBadIndex {})).as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for RandomAccessIteratorBadIndex {}
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
impl std::cmp::Ord for RandomAccessIteratorBadDifferenceType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            RandomAccessIteratorBadDifferenceTypeImpl::operator_cmp(
                &Rc::new(RefCell::new(RandomAccessIteratorBadDifferenceType {})).as_pointer(),
                Rc::new(RefCell::new(RandomAccessIteratorBadDifferenceType {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for RandomAccessIteratorBadDifferenceType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for RandomAccessIteratorBadDifferenceType {
    fn eq(&self, other: &Self) -> bool {
        {
            RandomAccessIteratorBadDifferenceTypeImpl::operator_cmp(
                &Rc::new(RefCell::new(RandomAccessIteratorBadDifferenceType {})).as_pointer(),
                Rc::new(RefCell::new(RandomAccessIteratorBadDifferenceType {})).as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for RandomAccessIteratorBadDifferenceType {}
const _: () = assert!(true, "std::regular<RandomAccessIteratorBadDifferenceType>");
const _: () = assert!(
    !(false),
    "!std::weakly_incrementable<RandomAccessIteratorBadDifferenceType>"
);
const _: () = assert!(
    !(false),
    "!std::random_access_iterator<RandomAccessIteratorBadDifferenceType>"
);
pub fn operator_eq_35(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_36(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_37(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_38(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_39(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_40(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_41(_a0: Ptr<MoveOnly>, _a1: Ptr<MoveOnly>) -> () {
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
impl std::cmp::Ord for MoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_41(
                Rc::new(RefCell::new(MoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(MoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_35(
                Rc::new(RefCell::new(MoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(MoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new(None) }
    }
}
pub fn operator_eq_42(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_43(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_44(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_45(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_46(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_47(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_48(_a0: Ptr<TrivialMoveOnly>, _a1: Ptr<TrivialMoveOnly>) -> () {
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
impl std::cmp::Ord for TrivialMoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_48(
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for TrivialMoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for TrivialMoveOnly {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_42(
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for TrivialMoveOnly {}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        { TrivialMoveOnly::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct InvalidDifferenceT {}
const _: () = assert!(true, "HasShiftLeftIt<int*>");
const _: () = assert!(true, "HasShiftLeftIt<int*, sentinel_wrapper<int*>>");
const _: () = assert!(true, "HasShiftLeftIt<int*, sized_sentinel<int*>>");
const _: () = assert!(!(false), "!HasShiftLeftIt<int*, int*, InvalidDifferenceT>");
const _: () = assert!(!(false), "!HasShiftLeftIt<int*, int, int>");
const _: () = assert!(!(false), "!HasShiftLeftIt<ForwardIteratorNotDerivedFrom>");
const _: () = assert!(!(false), "!HasShiftLeftIt<PermutableNotForwardIterator>");
const _: () = assert!(!(false), "!HasShiftLeftIt<PermutableNotSwappable>");
const _: () = assert!(true, "HasShiftLeftR<UncheckedRange<int*>>");
const _: () = assert!(
    !(false),
    "!HasShiftLeftR<UncheckedRange<int*>, InvalidDifferenceT>"
);
const _: () = assert!(!(false), "!HasShiftLeftR<ForwardRangeNotDerivedFrom>");
const _: () = assert!(
    !(false),
    "!HasShiftLeftR<PermutableRangeNotForwardIterator>"
);
const _: () = assert!(!(false), "!HasShiftLeftR<PermutableRangeNotSwappable>");
pub fn operator_add_49(x: DiffTypeIter, mut n: ()) -> DiffTypeIter {
    let x: Value<DiffTypeIter> = Rc::new(RefCell::new(x));
    ({ DiffTypeIterImpl::operator_add_assign(&x.as_pointer(), n) });
    return (*x.borrow()).clone();
}
pub fn operator_add_50(mut n: (), x: DiffTypeIter) -> DiffTypeIter {
    let x: Value<DiffTypeIter> = Rc::new(RefCell::new(x));
    ({ DiffTypeIterImpl::operator_add_assign(&x.as_pointer(), n) });
    return (*x.borrow()).clone();
}
pub fn operator_sub_51(x: DiffTypeIter, mut n: ()) -> DiffTypeIter {
    let x: Value<DiffTypeIter> = Rc::new(RefCell::new(x));
    ({ DiffTypeIterImpl::operator_sub_assign(&x.as_pointer(), n) });
    return (*x.borrow()).clone();
}
pub fn operator_sub_52(mut x: DiffTypeIter, mut y: DiffTypeIter) -> () {
    return ((x.it_).clone() - (y.it_).clone()) as ();
}
pub fn operator_eq_53(x: Ptr<DiffTypeIter>, y: Ptr<DiffTypeIter>) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_54(x: Ptr<DiffTypeIter>, y: Ptr<DiffTypeIter>) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_lt_55(x: Ptr<DiffTypeIter>, y: Ptr<DiffTypeIter>) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } < { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_le_56(x: Ptr<DiffTypeIter>, y: Ptr<DiffTypeIter>) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } <= { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_gt_57(x: Ptr<DiffTypeIter>, y: Ptr<DiffTypeIter>) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } > { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ge_58(x: Ptr<DiffTypeIter>, y: Ptr<DiffTypeIter>) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } >= { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct DiffTypeIter {
    #[offset(0)]
    #[byte_size(8)]
    pub it_: Ptr<()>,
}
impl DiffTypeIter {
    pub fn new_1() -> Self {
        Self {
            it_: Ptr::<()>::null(),
        }
    }
    pub fn new_2(mut it: Ptr<()>) -> Self {
        Self { it_: (it).clone() }
    }
}
impl std::cmp::Ord for DiffTypeIter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_55(
                Rc::new(RefCell::new(DiffTypeIter {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(DiffTypeIter {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_55(
                Rc::new(RefCell::new(DiffTypeIter {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(DiffTypeIter {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for DiffTypeIter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for DiffTypeIter {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_53(
                Rc::new(RefCell::new(DiffTypeIter {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(DiffTypeIter {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for DiffTypeIter {}
impl Default for DiffTypeIter {
    fn default() -> Self {
        { DiffTypeIter::new_1() }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign)]
#[byte_size(8)]
pub struct TrackCopyMove {
    #[offset(0)]
    pub copy_count: (),
    #[offset(4)]
    pub move_count: (),
}
impl TrackCopyMove {
    pub fn copy_from(other: Ptr<TrackCopyMove>) -> Self {
        let __this: Value<TrackCopyMove> = Rc::new(RefCell::new(Self {
            copy_count: other.with(|__s| __s.copy_count),
            move_count: other.with(|__s| __s.move_count),
        }));
        let this: Ptr<TrackCopyMove> = __this.as_pointer();
        field!(this, copy_count).with_mut(|__v| __v.prefix_inc());
        field!(other, copy_count).with_mut(|__v| __v.prefix_inc());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(other: Ptr<TrackCopyMove>) -> Self {
        let __this: Value<TrackCopyMove> = Rc::new(RefCell::new(Self {
            copy_count: other.with(|__s| __s.copy_count),
            move_count: other.with(|__s| __s.move_count),
        }));
        let this: Ptr<TrackCopyMove> = __this.as_pointer();
        field!(this, move_count).with_mut(|__v| __v.prefix_inc());
        field!(other, move_count).with_mut(|__v| __v.prefix_inc());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for TrackCopyMove {
    fn clone(&self) -> Self {
        let __src: Value<TrackCopyMove> = Rc::new(RefCell::new(TrackCopyMove {
            copy_count: self.copy_count.clone(),
            move_count: self.move_count.clone(),
        }));
        TrackCopyMove::copy_from(__src.as_pointer())
    }
}
impl Default for TrackCopyMove {
    fn default() -> Self {
        {
            TrackCopyMove {
                copy_count: 0_(),
                move_count: 0_(),
            }
        }
    }
}
pub fn test_iter_sent_59() {
    {
        let original: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut begin: Ptr<()> = unimplemented!();
                let mut end: Ptr<()> = unimplemented!().offset((n) as isize);
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(___first, ___last, (begin).clone())
                });
                let result: Value<()> = Rc::new(RefCell::new(
                    ({
                        (*shift_left_61.with(Value::clone).borrow_mut())(
                            (begin).clone(),
                            (end).clone(),
                            k,
                        )
                    }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut begin: Ptr<()> = unimplemented!();
                let mut end: Ptr<()> = unimplemented!().offset((n) as isize);
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(___first, ___last, (begin).clone())
                });
                let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(
                    ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), k) }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let mut begin: Ptr<()> = unimplemented!();
            let mut end: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (begin).clone(),
                        (end).clone(),
                        0_(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let mut begin: Ptr<()> = unimplemented!();
            let mut end: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), 0_()) }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let mut begin: Ptr<()> = unimplemented!();
            let mut end: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (begin).clone(),
                        (end).clone(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let mut begin: Ptr<()> = unimplemented!();
            let mut end: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let mut begin: Ptr<()> = unimplemented!();
            let mut end: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (begin).clone(),
                        (end).clone(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let mut begin: Ptr<()> = unimplemented!();
            let mut end: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let mut in_: () = unimplemented!();
            let mut begin: Ptr<()> = unimplemented!();
            let mut end: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (begin).clone(),
                        (end).clone(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut begin: Ptr<()> = unimplemented!();
            let mut end: Ptr<()> = unimplemented!().offset((unimplemented!()) as isize);
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_iter_sent_62() {
    {
        let original: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut begin: Ptr<()> = unimplemented!();
                let mut end: sentinel_wrapper_int_ptr_ = {
                    let __tmp_0: Value<Ptr<()>> =
                        Rc::new(RefCell::new(unimplemented!().offset((n) as isize)));
                    sentinel_wrapper_int_ptr_::new_1({ __tmp_0.as_pointer() })
                };
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(___first, ___last, (begin).clone())
                });
                let result: Value<()> = Rc::new(RefCell::new(
                    ({
                        (*shift_left_61.with(Value::clone).borrow_mut())(
                            (begin).clone(),
                            (end).clone(),
                            k,
                        )
                    }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut begin: Ptr<()> = unimplemented!();
                let mut end: sentinel_wrapper_int_ptr_ = {
                    let __tmp_1: Value<Ptr<()>> =
                        Rc::new(RefCell::new(unimplemented!().offset((n) as isize)));
                    sentinel_wrapper_int_ptr_::new_1({ __tmp_1.as_pointer() })
                };
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(___first, ___last, (begin).clone())
                });
                let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(
                    ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), k) }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let mut begin: Ptr<()> = unimplemented!();
            let end: Value<sentinel_wrapper_int_ptr_> = Rc::new(RefCell::new({
                let __tmp_2: Value<Ptr<()>> = Rc::new(RefCell::new(
                    unimplemented!().offset((unimplemented!()) as isize),
                ));
                sentinel_wrapper_int_ptr_::new_1({ __tmp_2.as_pointer() })
            }));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (begin).clone(),
                        (*end.borrow()).clone(),
                        0_(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let mut begin: Ptr<()> = unimplemented!();
            let end: Value<sentinel_wrapper_int_ptr_> = Rc::new(RefCell::new({
                let __tmp_3: Value<Ptr<()>> = Rc::new(RefCell::new(
                    unimplemented!().offset((unimplemented!()) as isize),
                ));
                sentinel_wrapper_int_ptr_::new_1({ __tmp_3.as_pointer() })
            }));
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), 0_()) }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let mut begin: Ptr<()> = unimplemented!();
            let mut end: sentinel_wrapper_int_ptr_ = {
                let __tmp_4: Value<Ptr<()>> = Rc::new(RefCell::new(
                    unimplemented!().offset((unimplemented!()) as isize),
                ));
                sentinel_wrapper_int_ptr_::new_1({ __tmp_4.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (begin).clone(),
                        (end).clone(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let mut begin: Ptr<()> = unimplemented!();
            let mut end: sentinel_wrapper_int_ptr_ = {
                let __tmp_5: Value<Ptr<()>> = Rc::new(RefCell::new(
                    unimplemented!().offset((unimplemented!()) as isize),
                ));
                sentinel_wrapper_int_ptr_::new_1({ __tmp_5.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let mut begin: Ptr<()> = unimplemented!();
            let mut end: sentinel_wrapper_int_ptr_ = {
                let __tmp_6: Value<Ptr<()>> = Rc::new(RefCell::new(
                    unimplemented!().offset((unimplemented!()) as isize),
                ));
                sentinel_wrapper_int_ptr_::new_1({ __tmp_6.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (begin).clone(),
                        (end).clone(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let mut begin: Ptr<()> = unimplemented!();
            let mut end: sentinel_wrapper_int_ptr_ = {
                let __tmp_7: Value<Ptr<()>> = Rc::new(RefCell::new(
                    unimplemented!().offset((unimplemented!()) as isize),
                ));
                sentinel_wrapper_int_ptr_::new_1({ __tmp_7.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let mut in_: () = unimplemented!();
            let mut begin: Ptr<()> = unimplemented!();
            let mut end: sentinel_wrapper_int_ptr_ = {
                let __tmp_8: Value<Ptr<()>> = Rc::new(RefCell::new(
                    unimplemented!().offset((unimplemented!()) as isize),
                ));
                sentinel_wrapper_int_ptr_::new_1({ __tmp_8.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (begin).clone(),
                        (end).clone(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut begin: Ptr<()> = unimplemented!();
            let mut end: sentinel_wrapper_int_ptr_ = {
                let __tmp_9: Value<Ptr<()>> = Rc::new(RefCell::new(
                    unimplemented!().offset((unimplemented!()) as isize),
                ));
                sentinel_wrapper_int_ptr_::new_1({ __tmp_9.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_iter_sent_63() {
    {
        let original: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut begin: Ptr<()> = unimplemented!();
                let mut end: sized_sentinel_int_ptr_ = {
                    let __tmp_10: Value<Ptr<()>> =
                        Rc::new(RefCell::new(unimplemented!().offset((n) as isize)));
                    sized_sentinel_int_ptr_::new_1({ __tmp_10.as_pointer() })
                };
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(___first, ___last, (begin).clone())
                });
                let result: Value<()> = Rc::new(RefCell::new(
                    ({
                        (*shift_left_61.with(Value::clone).borrow_mut())(
                            (begin).clone(),
                            (end).clone(),
                            k,
                        )
                    }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut begin: Ptr<()> = unimplemented!();
                let mut end: sized_sentinel_int_ptr_ = {
                    let __tmp_11: Value<Ptr<()>> =
                        Rc::new(RefCell::new(unimplemented!().offset((n) as isize)));
                    sized_sentinel_int_ptr_::new_1({ __tmp_11.as_pointer() })
                };
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(___first, ___last, (begin).clone())
                });
                let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(
                    ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), k) }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let mut begin: Ptr<()> = unimplemented!();
            let end: Value<sized_sentinel_int_ptr_> = Rc::new(RefCell::new({
                let __tmp_12: Value<Ptr<()>> = Rc::new(RefCell::new(
                    unimplemented!().offset((unimplemented!()) as isize),
                ));
                sized_sentinel_int_ptr_::new_1({ __tmp_12.as_pointer() })
            }));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (begin).clone(),
                        (*end.borrow()).clone(),
                        0_(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let mut begin: Ptr<()> = unimplemented!();
            let end: Value<sized_sentinel_int_ptr_> = Rc::new(RefCell::new({
                let __tmp_13: Value<Ptr<()>> = Rc::new(RefCell::new(
                    unimplemented!().offset((unimplemented!()) as isize),
                ));
                sized_sentinel_int_ptr_::new_1({ __tmp_13.as_pointer() })
            }));
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), 0_()) }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let mut begin: Ptr<()> = unimplemented!();
            let mut end: sized_sentinel_int_ptr_ = {
                let __tmp_14: Value<Ptr<()>> = Rc::new(RefCell::new(
                    unimplemented!().offset((unimplemented!()) as isize),
                ));
                sized_sentinel_int_ptr_::new_1({ __tmp_14.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (begin).clone(),
                        (end).clone(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let mut begin: Ptr<()> = unimplemented!();
            let mut end: sized_sentinel_int_ptr_ = {
                let __tmp_15: Value<Ptr<()>> = Rc::new(RefCell::new(
                    unimplemented!().offset((unimplemented!()) as isize),
                ));
                sized_sentinel_int_ptr_::new_1({ __tmp_15.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let mut begin: Ptr<()> = unimplemented!();
            let mut end: sized_sentinel_int_ptr_ = {
                let __tmp_16: Value<Ptr<()>> = Rc::new(RefCell::new(
                    unimplemented!().offset((unimplemented!()) as isize),
                ));
                sized_sentinel_int_ptr_::new_1({ __tmp_16.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (begin).clone(),
                        (end).clone(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let mut begin: Ptr<()> = unimplemented!();
            let mut end: sized_sentinel_int_ptr_ = {
                let __tmp_17: Value<Ptr<()>> = Rc::new(RefCell::new(
                    unimplemented!().offset((unimplemented!()) as isize),
                ));
                sized_sentinel_int_ptr_::new_1({ __tmp_17.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let mut in_: () = unimplemented!();
            let mut begin: Ptr<()> = unimplemented!();
            let mut end: sized_sentinel_int_ptr_ = {
                let __tmp_18: Value<Ptr<()>> = Rc::new(RefCell::new(
                    unimplemented!().offset((unimplemented!()) as isize),
                ));
                sized_sentinel_int_ptr_::new_1({ __tmp_18.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (begin).clone(),
                        (end).clone(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let mut begin: Ptr<()> = unimplemented!();
            let mut end: sized_sentinel_int_ptr_ = {
                let __tmp_19: Value<Ptr<()>> = Rc::new(RefCell::new(
                    unimplemented!().offset((unimplemented!()) as isize),
                ));
                sized_sentinel_int_ptr_::new_1({ __tmp_19.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_iter_sent_64() {
    {
        let original: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((n) as isize) });
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let result: Value<()> = Rc::new(RefCell::new(
                    ({
                        (*shift_left_61.with(Value::clone).borrow_mut())(
                            (*begin.borrow()).clone(),
                            (end).clone(),
                            k,
                        )
                    }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: contiguous_iterator_int_ptr_ =
                    contiguous_iterator_int_ptr_::new_1({ unimplemented!().offset((n) as isize) });
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(
                    ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), k) }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                })));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (*end.borrow()).clone(),
                        0_(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                })));
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), 0_()) }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let mut in_: () = unimplemented!();
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: contiguous_iterator_int_ptr_ = contiguous_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_iter_sent_65() {
    {
        let original: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let __tmp_20: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        })));
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_20.as_pointer() })
                };
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let result: Value<()> = Rc::new(RefCell::new(
                    ({
                        (*shift_left_61.with(Value::clone).borrow_mut())(
                            (*begin.borrow()).clone(),
                            (end).clone(),
                            k,
                        )
                    }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                    let __tmp_21: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        })));
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_21.as_pointer() })
                };
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(
                    ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), k) }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<sentinel_wrapper_contiguous_iterator_int_ptr__> =
                Rc::new(RefCell::new({
                    let __tmp_22: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            unimplemented!().offset((unimplemented!()) as isize)
                        })));
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_22.as_pointer() })
                }));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (*end.borrow()).clone(),
                        0_(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<sentinel_wrapper_contiguous_iterator_int_ptr__> =
                Rc::new(RefCell::new({
                    let __tmp_23: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            unimplemented!().offset((unimplemented!()) as isize)
                        })));
                    sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_23.as_pointer() })
                }));
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), 0_()) }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                let __tmp_24: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_24.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                let __tmp_25: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_25.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                let __tmp_26: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_26.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                let __tmp_27: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_27.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let mut in_: () = unimplemented!();
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                let __tmp_28: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_28.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_contiguous_iterator_int_ptr__ = {
                let __tmp_29: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_29.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_iter_sent_66() {
    {
        let original: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let __tmp_30: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        })));
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_30.as_pointer() })
                };
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let result: Value<()> = Rc::new(RefCell::new(
                    ({
                        (*shift_left_61.with(Value::clone).borrow_mut())(
                            (*begin.borrow()).clone(),
                            (end).clone(),
                            k,
                        )
                    }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: sized_sentinel_contiguous_iterator_int_ptr__ = {
                    let __tmp_31: Value<contiguous_iterator_int_ptr_> =
                        Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        })));
                    sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_31.as_pointer() })
                };
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(
                    ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), k) }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<sized_sentinel_contiguous_iterator_int_ptr__> = Rc::new(RefCell::new({
                let __tmp_32: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_32.as_pointer() })
            }));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (*end.borrow()).clone(),
                        0_(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<sized_sentinel_contiguous_iterator_int_ptr__> = Rc::new(RefCell::new({
                let __tmp_33: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_33.as_pointer() })
            }));
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), 0_()) }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_contiguous_iterator_int_ptr__ = {
                let __tmp_34: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_34.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_contiguous_iterator_int_ptr__ = {
                let __tmp_35: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_35.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_contiguous_iterator_int_ptr__ = {
                let __tmp_36: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_36.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_contiguous_iterator_int_ptr__ = {
                let __tmp_37: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_37.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let mut in_: () = unimplemented!();
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_contiguous_iterator_int_ptr__ = {
                let __tmp_38: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_38.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let begin: Value<contiguous_iterator_int_ptr_> =
                Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_contiguous_iterator_int_ptr__ = {
                let __tmp_39: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_contiguous_iterator_int_ptr__::new_1({ __tmp_39.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_iter_sent_67() {
    {
        let original: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((n) as isize)
                    });
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let result: Value<()> = Rc::new(RefCell::new(
                    ({
                        (*shift_left_61.with(Value::clone).borrow_mut())(
                            (*begin.borrow()).clone(),
                            (end).clone(),
                            k,
                        )
                    }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: random_access_iterator_int_ptr_ =
                    random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((n) as isize)
                    });
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(
                    ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), k) }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                })));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (*end.borrow()).clone(),
                        0_(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                })));
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), 0_()) }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let mut in_: () = unimplemented!();
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: random_access_iterator_int_ptr_ =
                random_access_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_iter_sent_68() {
    {
        let original: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let __tmp_40: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        })));
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                        __tmp_40.as_pointer()
                    })
                };
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let result: Value<()> = Rc::new(RefCell::new(
                    ({
                        (*shift_left_61.with(Value::clone).borrow_mut())(
                            (*begin.borrow()).clone(),
                            (end).clone(),
                            k,
                        )
                    }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                    let __tmp_41: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        })));
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                        __tmp_41.as_pointer()
                    })
                };
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(
                    ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), k) }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<sentinel_wrapper_random_access_iterator_int_ptr__> =
                Rc::new(RefCell::new({
                    let __tmp_42: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            unimplemented!().offset((unimplemented!()) as isize)
                        })));
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                        __tmp_42.as_pointer()
                    })
                }));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (*end.borrow()).clone(),
                        0_(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<sentinel_wrapper_random_access_iterator_int_ptr__> =
                Rc::new(RefCell::new({
                    let __tmp_43: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            unimplemented!().offset((unimplemented!()) as isize)
                        })));
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({
                        __tmp_43.as_pointer()
                    })
                }));
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), 0_()) }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                let __tmp_44: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ __tmp_44.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                let __tmp_45: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ __tmp_45.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                let __tmp_46: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ __tmp_46.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                let __tmp_47: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ __tmp_47.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let mut in_: () = unimplemented!();
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                let __tmp_48: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ __tmp_48.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_random_access_iterator_int_ptr__ = {
                let __tmp_49: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ __tmp_49.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_iter_sent_69() {
    {
        let original: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let __tmp_50: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        })));
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({
                        __tmp_50.as_pointer()
                    })
                };
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let result: Value<()> = Rc::new(RefCell::new(
                    ({
                        (*shift_left_61.with(Value::clone).borrow_mut())(
                            (*begin.borrow()).clone(),
                            (end).clone(),
                            k,
                        )
                    }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: sized_sentinel_random_access_iterator_int_ptr__ = {
                    let __tmp_51: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        })));
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({
                        __tmp_51.as_pointer()
                    })
                };
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(
                    ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), k) }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<sized_sentinel_random_access_iterator_int_ptr__> =
                Rc::new(RefCell::new({
                    let __tmp_52: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            unimplemented!().offset((unimplemented!()) as isize)
                        })));
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({
                        __tmp_52.as_pointer()
                    })
                }));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (*end.borrow()).clone(),
                        0_(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<sized_sentinel_random_access_iterator_int_ptr__> =
                Rc::new(RefCell::new({
                    let __tmp_53: Value<random_access_iterator_int_ptr_> =
                        Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                            unimplemented!().offset((unimplemented!()) as isize)
                        })));
                    sized_sentinel_random_access_iterator_int_ptr__::new_1({
                        __tmp_53.as_pointer()
                    })
                }));
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), 0_()) }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_random_access_iterator_int_ptr__ = {
                let __tmp_54: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_random_access_iterator_int_ptr__::new_1({ __tmp_54.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_random_access_iterator_int_ptr__ = {
                let __tmp_55: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_random_access_iterator_int_ptr__::new_1({ __tmp_55.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_random_access_iterator_int_ptr__ = {
                let __tmp_56: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_random_access_iterator_int_ptr__::new_1({ __tmp_56.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_random_access_iterator_int_ptr__ = {
                let __tmp_57: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_random_access_iterator_int_ptr__::new_1({ __tmp_57.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let mut in_: () = unimplemented!();
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_random_access_iterator_int_ptr__ = {
                let __tmp_58: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_random_access_iterator_int_ptr__::new_1({ __tmp_58.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let begin: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_random_access_iterator_int_ptr__ = {
                let __tmp_59: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_random_access_iterator_int_ptr__::new_1({ __tmp_59.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_iter_sent_70() {
    {
        let original: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((n) as isize)
                    });
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let result: Value<()> = Rc::new(RefCell::new(
                    ({
                        (*shift_left_61.with(Value::clone).borrow_mut())(
                            (*begin.borrow()).clone(),
                            (end).clone(),
                            k,
                        )
                    }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: bidirectional_iterator_int_ptr_ =
                    bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((n) as isize)
                    });
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(
                    ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), k) }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                })));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (*end.borrow()).clone(),
                        0_(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                })));
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), 0_()) }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let mut in_: () = unimplemented!();
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: bidirectional_iterator_int_ptr_ =
                bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_iter_sent_71() {
    {
        let original: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let __tmp_60: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        })));
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_60.as_pointer()
                    })
                };
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let result: Value<()> = Rc::new(RefCell::new(
                    ({
                        (*shift_left_61.with(Value::clone).borrow_mut())(
                            (*begin.borrow()).clone(),
                            (end).clone(),
                            k,
                        )
                    }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                    let __tmp_61: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        })));
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_61.as_pointer()
                    })
                };
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(
                    ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), k) }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<sentinel_wrapper_bidirectional_iterator_int_ptr__> =
                Rc::new(RefCell::new({
                    let __tmp_62: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            unimplemented!().offset((unimplemented!()) as isize)
                        })));
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_62.as_pointer()
                    })
                }));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (*end.borrow()).clone(),
                        0_(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<sentinel_wrapper_bidirectional_iterator_int_ptr__> =
                Rc::new(RefCell::new({
                    let __tmp_63: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            unimplemented!().offset((unimplemented!()) as isize)
                        })));
                    sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_63.as_pointer()
                    })
                }));
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), 0_()) }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                let __tmp_64: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ __tmp_64.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                let __tmp_65: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ __tmp_65.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                let __tmp_66: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ __tmp_66.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                let __tmp_67: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ __tmp_67.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let mut in_: () = unimplemented!();
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                let __tmp_68: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ __tmp_68.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_bidirectional_iterator_int_ptr__ = {
                let __tmp_69: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ __tmp_69.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_iter_sent_72() {
    {
        let original: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let __tmp_70: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        })));
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_70.as_pointer()
                    })
                };
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let result: Value<()> = Rc::new(RefCell::new(
                    ({
                        (*shift_left_61.with(Value::clone).borrow_mut())(
                            (*begin.borrow()).clone(),
                            (end).clone(),
                            k,
                        )
                    }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                    let __tmp_71: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        })));
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_71.as_pointer()
                    })
                };
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(
                    ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), k) }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<sized_sentinel_bidirectional_iterator_int_ptr__> =
                Rc::new(RefCell::new({
                    let __tmp_72: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            unimplemented!().offset((unimplemented!()) as isize)
                        })));
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_72.as_pointer()
                    })
                }));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (*end.borrow()).clone(),
                        0_(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<sized_sentinel_bidirectional_iterator_int_ptr__> =
                Rc::new(RefCell::new({
                    let __tmp_73: Value<bidirectional_iterator_int_ptr_> =
                        Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                            unimplemented!().offset((unimplemented!()) as isize)
                        })));
                    sized_sentinel_bidirectional_iterator_int_ptr__::new_1({
                        __tmp_73.as_pointer()
                    })
                }));
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), 0_()) }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                let __tmp_74: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ __tmp_74.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                let __tmp_75: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ __tmp_75.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                let __tmp_76: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ __tmp_76.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                let __tmp_77: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ __tmp_77.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let mut in_: () = unimplemented!();
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                let __tmp_78: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ __tmp_78.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let begin: Value<bidirectional_iterator_int_ptr_> =
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_bidirectional_iterator_int_ptr__ = {
                let __tmp_79: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_bidirectional_iterator_int_ptr__::new_1({ __tmp_79.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_iter_sent_73() {
    {
        let original: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<forward_iterator_int_ptr_> =
                    Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!().offset((n) as isize) });
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let result: Value<()> = Rc::new(RefCell::new(
                    ({
                        (*shift_left_61.with(Value::clone).borrow_mut())(
                            (*begin.borrow()).clone(),
                            (end).clone(),
                            k,
                        )
                    }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<forward_iterator_int_ptr_> =
                    Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!().offset((n) as isize) });
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(
                    ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), k) }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                })));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (*end.borrow()).clone(),
                        0_(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                })));
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), 0_()) }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let mut in_: () = unimplemented!();
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_iter_sent_74() {
    {
        let original: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<forward_iterator_int_ptr_> =
                    Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let __tmp_80: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        })));
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_80.as_pointer() })
                };
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let result: Value<()> = Rc::new(RefCell::new(
                    ({
                        (*shift_left_61.with(Value::clone).borrow_mut())(
                            (*begin.borrow()).clone(),
                            (end).clone(),
                            k,
                        )
                    }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<forward_iterator_int_ptr_> =
                    Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
                    let __tmp_81: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        })));
                    sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_81.as_pointer() })
                };
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(
                    ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), k) }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<sentinel_wrapper_forward_iterator_int_ptr__> = Rc::new(RefCell::new({
                let __tmp_82: Value<forward_iterator_int_ptr_> =
                    Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_82.as_pointer() })
            }));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (*end.borrow()).clone(),
                        0_(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<sentinel_wrapper_forward_iterator_int_ptr__> = Rc::new(RefCell::new({
                let __tmp_83: Value<forward_iterator_int_ptr_> =
                    Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_83.as_pointer() })
            }));
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), 0_()) }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
                let __tmp_84: Value<forward_iterator_int_ptr_> =
                    Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_84.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
                let __tmp_85: Value<forward_iterator_int_ptr_> =
                    Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_85.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
                let __tmp_86: Value<forward_iterator_int_ptr_> =
                    Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_86.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
                let __tmp_87: Value<forward_iterator_int_ptr_> =
                    Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_87.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let mut in_: () = unimplemented!();
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
                let __tmp_88: Value<forward_iterator_int_ptr_> =
                    Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_88.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sentinel_wrapper_forward_iterator_int_ptr__ = {
                let __tmp_89: Value<forward_iterator_int_ptr_> =
                    Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_89.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_iter_sent_75() {
    {
        let original: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<forward_iterator_int_ptr_> =
                    Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: sized_sentinel_forward_iterator_int_ptr__ = {
                    let __tmp_90: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        })));
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_90.as_pointer() })
                };
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let result: Value<()> = Rc::new(RefCell::new(
                    ({
                        (*shift_left_61.with(Value::clone).borrow_mut())(
                            (*begin.borrow()).clone(),
                            (end).clone(),
                            k,
                        )
                    }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<forward_iterator_int_ptr_> =
                    Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                        unimplemented!()
                    })));
                let mut end: sized_sentinel_forward_iterator_int_ptr__ = {
                    let __tmp_91: Value<forward_iterator_int_ptr_> =
                        Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                            unimplemented!().offset((n) as isize)
                        })));
                    sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_91.as_pointer() })
                };
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(
                    ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), k) }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<sized_sentinel_forward_iterator_int_ptr__> = Rc::new(RefCell::new({
                let __tmp_92: Value<forward_iterator_int_ptr_> =
                    Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_92.as_pointer() })
            }));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (*end.borrow()).clone(),
                        0_(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let end: Value<sized_sentinel_forward_iterator_int_ptr__> = Rc::new(RefCell::new({
                let __tmp_93: Value<forward_iterator_int_ptr_> =
                    Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_93.as_pointer() })
            }));
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), 0_()) }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_forward_iterator_int_ptr__ = {
                let __tmp_94: Value<forward_iterator_int_ptr_> =
                    Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_94.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_forward_iterator_int_ptr__ = {
                let __tmp_95: Value<forward_iterator_int_ptr_> =
                    Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_95.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_forward_iterator_int_ptr__ = {
                let __tmp_96: Value<forward_iterator_int_ptr_> =
                    Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_96.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_forward_iterator_int_ptr__ = {
                let __tmp_97: Value<forward_iterator_int_ptr_> =
                    Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_97.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let mut in_: () = unimplemented!();
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_forward_iterator_int_ptr__ = {
                let __tmp_98: Value<forward_iterator_int_ptr_> =
                    Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_98.as_pointer() })
            };
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let begin: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!()
                })));
            let mut end: sized_sentinel_forward_iterator_int_ptr__ = {
                let __tmp_99: Value<forward_iterator_int_ptr_> =
                    Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sized_sentinel_forward_iterator_int_ptr__::new_1({ __tmp_99.as_pointer() })
            };
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_iter_sent_76() {
    {
        let original: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<DiffTypeIter> =
                    Rc::new(RefCell::new(DiffTypeIter::new_2({ unimplemented!() })));
                let mut end: DiffTypeIter =
                    DiffTypeIter::new_2({ unimplemented!().offset((n) as isize) });
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let result: Value<()> = Rc::new(RefCell::new(
                    ({
                        (*shift_left_61.with(Value::clone).borrow_mut())(
                            (*begin.borrow()).clone(),
                            (end).clone(),
                            k,
                        )
                    }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
        let mut n: () = 0_();
        'loop_: while (n <= unimplemented!()) {
            let mut k: () = 0_();
            'loop_: while (k <= (n).wrapping_add(2_())) {
                let scratch: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let begin: Value<DiffTypeIter> =
                    Rc::new(RefCell::new(DiffTypeIter::new_2({ unimplemented!() })));
                let mut end: DiffTypeIter =
                    DiffTypeIter::new_2({ unimplemented!().offset((n) as isize) });
                ({
                    let ___first: Ptr<()> = unimplemented!();
                    let ___last: Ptr<()> = unimplemented!().offset((n) as isize);
                    (*copy_60.with(Value::clone).borrow_mut())(
                        ___first,
                        ___last,
                        (*begin.borrow()).clone(),
                    )
                });
                let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(
                    ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), k) }),
                ));
                unimplemented!();
                if (k < n) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    unimplemented!();
                    unimplemented!();
                }
                k.prefix_inc();
            }
            n.prefix_inc();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<DiffTypeIter> =
                Rc::new(RefCell::new(DiffTypeIter::new_2({ unimplemented!() })));
            let end: Value<DiffTypeIter> = Rc::new(RefCell::new(DiffTypeIter::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            })));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (*end.borrow()).clone(),
                        0_(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<DiffTypeIter> =
                Rc::new(RefCell::new(DiffTypeIter::new_2({ unimplemented!() })));
            let end: Value<DiffTypeIter> = Rc::new(RefCell::new(DiffTypeIter::new_2({
                unimplemented!().offset((unimplemented!()) as isize)
            })));
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({ (*shift_left_61.with(Value::clone).borrow_mut())(range.as_pointer(), 0_()) }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<DiffTypeIter> =
                Rc::new(RefCell::new(DiffTypeIter::new_2({ unimplemented!() })));
            let mut end: DiffTypeIter =
                DiffTypeIter::new_2({ unimplemented!().offset((unimplemented!()) as isize) });
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<DiffTypeIter> =
                Rc::new(RefCell::new(DiffTypeIter::new_2({ unimplemented!() })));
            let mut end: DiffTypeIter =
                DiffTypeIter::new_2({ unimplemented!().offset((unimplemented!()) as isize) });
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        unimplemented!(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<DiffTypeIter> =
                Rc::new(RefCell::new(DiffTypeIter::new_2({ unimplemented!() })));
            let mut end: DiffTypeIter =
                DiffTypeIter::new_2({ unimplemented!().offset((unimplemented!()) as isize) });
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let in_: Value<()> = Rc::new(RefCell::new((*input.borrow()).clone()));
            let begin: Value<DiffTypeIter> =
                Rc::new(RefCell::new(DiffTypeIter::new_2({ unimplemented!() })));
            let mut end: DiffTypeIter =
                DiffTypeIter::new_2({ unimplemented!().offset((unimplemented!()) as isize) });
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!() as ()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let input: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let mut in_: () = unimplemented!();
            let begin: Value<DiffTypeIter> =
                Rc::new(RefCell::new(DiffTypeIter::new_2({ unimplemented!() })));
            let mut end: DiffTypeIter =
                DiffTypeIter::new_2({ unimplemented!().offset((unimplemented!()) as isize) });
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        (*begin.borrow()).clone(),
                        (end).clone(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut in_: () = unimplemented!();
            let begin: Value<DiffTypeIter> =
                Rc::new(RefCell::new(DiffTypeIter::new_2({ unimplemented!() })));
            let mut end: DiffTypeIter =
                DiffTypeIter::new_2({ unimplemented!().offset((unimplemented!()) as isize) });
            let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*shift_left_61.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        (unimplemented!()).wrapping_add(1_()),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_77() -> () {
    ({
        for_each_0 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__  >::default()  , lambda! ( Generic , { } , | | { ( { test_iter_sent_59 ( ) } )  ;
 ( { test_iter_sent_62 ( ) } )  ;
 ( { test_iter_sent_63 ( ) } )  ;
 } , | | { ( { test_iter_sent_64 ( ) } )  ;
 ( { test_iter_sent_65 ( ) } )  ;
 ( { test_iter_sent_66 ( ) } )  ;
 } , | | { ( { test_iter_sent_67 ( ) } )  ;
 ( { test_iter_sent_68 ( ) } )  ;
 ( { test_iter_sent_69 ( ) } )  ;
 } , | | { ( { test_iter_sent_70 ( ) } )  ;
 ( { test_iter_sent_71 ( ) } )  ;
 ( { test_iter_sent_72 ( ) } )  ;
 } , | | { ( { test_iter_sent_73 ( ) } )  ;
 ( { test_iter_sent_74 ( ) } )  ;
 ( { test_iter_sent_75 ( ) } )  ;
 } )  , )
    });
    ({ test_iter_sent_76() });
    {
        let mut length: () = 100_();
        let mut n: () = (length / 2_());
        let make_vec: Value<FnPtr<fn() -> ()>> =
            Rc::new(RefCell::new(FnPtr::<fn() -> ()>::new(|| -> () {
                {
                    let mut vec_: () = unimplemented!();
                    unimplemented!();
                    let mut i: () = 0_();
                    'loop_: while (i < 100_()) {
                        unimplemented!();
                        i.prefix_inc();
                    }
                    return unimplemented!();
                }
            })));
        {
            let mut input: () = ({ (*make_vec.borrow()).call() }).clone();
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    let ___first: () = unimplemented!();
                    let ___last: () = unimplemented!();
                    (*shift_left_61.with(Value::clone).borrow_mut())(___first, ___last, n)
                }),
            ));
            unimplemented!();
            unimplemented!();
            let mut total_copies: () = 0_();
            let mut total_moves: () = 0_();
            let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            'loop_: while !(unimplemented!()) {
                let item: Ptr<TrackCopyMove> = unimplemented!();
                total_copies += item.with(|__s| __s.copy_count);
                total_moves += item.with(|__s| __s.move_count);
                unimplemented!();
            }
            unimplemented!();
            unimplemented!();
        }
        {
            let input: Value<()> = Rc::new(RefCell::new(({ (*make_vec.borrow()).call() }).clone()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({ (*shift_left_61.with(Value::clone).borrow_mut())(input.as_pointer(), n) }),
            ));
            unimplemented!();
            unimplemented!();
            let mut total_copies: () = 0_();
            let mut total_moves: () = 0_();
            let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            'loop_: while !(unimplemented!()) {
                let item: Ptr<TrackCopyMove> = unimplemented!();
                total_copies += item.with(|__s| __s.copy_count);
                total_moves += item.with(|__s| __s.move_count);
                unimplemented!();
            }
            unimplemented!();
            unimplemented!();
        }
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
    ({ test_77() });
    const _: () = assert!(true, "test()");;
    return 0_();
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
pub trait DiffTypeIterImpl {
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_index(&self, n: ()) -> Ptr<()>;
    fn operator_inc_5(&self) -> Ptr<DiffTypeIter>;
    fn operator_dec_6(&self) -> Ptr<DiffTypeIter>;
    fn operator_post_inc_7(&self, _a0: ()) -> DiffTypeIter;
    fn operator_post_dec_8(&self, _a0: ()) -> DiffTypeIter;
    fn operator_add_assign(&self, n: ()) -> Ptr<DiffTypeIter>;
    fn operator_sub_assign(&self, n: ()) -> Ptr<DiffTypeIter>;
}
impl DiffTypeIterImpl for Ptr<DiffTypeIter> {
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone()).offset((n) as isize);
    }
    fn operator_inc_5(&self) -> Ptr<DiffTypeIter> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_6(&self) -> Ptr<DiffTypeIter> {
        field!((*self), it_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_post_inc_7(&self, mut _a0: ()) -> DiffTypeIter {
        return DiffTypeIter::new_2({ field!((*self), it_).with_mut(|__v| __v.postfix_inc()) });
    }
    fn operator_post_dec_8(&self, mut _a0: ()) -> DiffTypeIter {
        return DiffTypeIter::new_2({ field!((*self), it_).with_mut(|__v| __v.postfix_dec()) });
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<DiffTypeIter> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<DiffTypeIter> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
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
pub trait TrackCopyMoveImpl {
    fn copy_assign(&self, other: Ptr<TrackCopyMove>) -> Ptr<TrackCopyMove>;
    fn move_assign(&self, other: Ptr<TrackCopyMove>) -> Ptr<TrackCopyMove>;
}
impl TrackCopyMoveImpl for Ptr<TrackCopyMove> {
    fn copy_assign(&self, other: Ptr<TrackCopyMove>) -> Ptr<TrackCopyMove> {
        field!((*self), copy_count).with_mut(|__v| __v.prefix_inc());
        field!(other, copy_count).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn move_assign(&self, other: Ptr<TrackCopyMove>) -> Ptr<TrackCopyMove> {
        field!((*self), move_count).with_mut(|__v| __v.prefix_inc());
        field!(other, move_count).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
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
pub trait UncheckedRange_PermutableNotForwardIterator__sentinel_wrapper_PermutableNotForwardIterator__Impl
{
    fn begin(&self) -> PermutableNotForwardIterator {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_PermutableNotForwardIterator_ {
        unimplemented!()
    }
}
impl UncheckedRange_PermutableNotForwardIterator__sentinel_wrapper_PermutableNotForwardIterator__Impl for Ptr<UncheckedRange_PermutableNotForwardIterator__sentinel_wrapper_PermutableNotForwardIterator__> {
}
pub trait UncheckedRange_PermutableNotSwappable__sentinel_wrapper_PermutableNotSwappable__Impl {
    fn begin(&self) -> PermutableNotSwappable {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_PermutableNotSwappable_ {
        unimplemented!()
    }
}
impl UncheckedRange_PermutableNotSwappable__sentinel_wrapper_PermutableNotSwappable__Impl
    for Ptr<UncheckedRange_PermutableNotSwappable__sentinel_wrapper_PermutableNotSwappable__>
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
pub trait bidirectional_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_int_ptr_>;
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_int_ptr_>;
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
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_dec());
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
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: ()) -> contiguous_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: ()) -> contiguous_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: ()) -> Ptr<contiguous_iterator_int_ptr_>;
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
    fn operator_arrow(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<contiguous_iterator_int_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
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
    fn operator_add_assign(&self, n: ()) -> Ptr<random_access_iterator_int_ptr_>;
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
    fn operator_add_assign(&self, mut n: ()) -> Ptr<random_access_iterator_int_ptr_> {
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
pub trait sentinel_wrapper_PermutableNotForwardIterator_Impl {
    fn operator_eq(&self, other: Ptr<PermutableNotForwardIterator>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_PermutableNotForwardIterator_Impl
    for Ptr<sentinel_wrapper_PermutableNotForwardIterator_>
{
}
pub trait sentinel_wrapper_PermutableNotSwappable_Impl {
    fn operator_eq(&self, other: Ptr<PermutableNotSwappable>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_PermutableNotSwappable_Impl
    for Ptr<sentinel_wrapper_PermutableNotSwappable_>
{
}
pub trait sentinel_wrapper_bidirectional_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<bidirectional_iterator_int_ptr_>) -> ();
}
impl sentinel_wrapper_bidirectional_iterator_int_ptr__Impl
    for Ptr<sentinel_wrapper_bidirectional_iterator_int_ptr__>
{
    fn operator_eq(&self, other: Ptr<bidirectional_iterator_int_ptr_>) -> () {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_7((other).clone()) }) });
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
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_16((other).clone()) }) });
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
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_17((other.read())) }) });
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
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_12((other).clone()) }) });
    }
}
pub trait sized_sentinel_bidirectional_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<bidirectional_iterator_int_ptr_>) -> ();
}
impl sized_sentinel_bidirectional_iterator_int_ptr__Impl
    for Ptr<sized_sentinel_bidirectional_iterator_int_ptr__>
{
    fn operator_eq(&self, other: Ptr<bidirectional_iterator_int_ptr_>) -> () {
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_7((other).clone()) }) });
    }
}
pub trait sized_sentinel_contiguous_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<contiguous_iterator_int_ptr_>) -> ();
}
impl sized_sentinel_contiguous_iterator_int_ptr__Impl
    for Ptr<sized_sentinel_contiguous_iterator_int_ptr__>
{
    fn operator_eq(&self, other: Ptr<contiguous_iterator_int_ptr_>) -> () {
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_16((other).clone()) }) });
    }
}
pub trait sized_sentinel_forward_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<forward_iterator_int_ptr_>) -> ();
}
impl sized_sentinel_forward_iterator_int_ptr__Impl
    for Ptr<sized_sentinel_forward_iterator_int_ptr__>
{
    fn operator_eq(&self, other: Ptr<forward_iterator_int_ptr_>) -> () {
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_4((other).clone()) }) });
    }
}
pub trait sized_sentinel_int_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> ();
}
impl sized_sentinel_int_ptr_Impl for Ptr<sized_sentinel_int_ptr_> {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> () {
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_17((other.read())) }) });
    }
}
pub trait sized_sentinel_random_access_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<random_access_iterator_int_ptr_>) -> ();
}
impl sized_sentinel_random_access_iterator_int_ptr__Impl
    for Ptr<sized_sentinel_random_access_iterator_int_ptr__>
{
    fn operator_eq(&self, other: Ptr<random_access_iterator_int_ptr_>) -> () {
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_12((other).clone()) }) });
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
pub fn __cpp2rust_init_globals() {}
