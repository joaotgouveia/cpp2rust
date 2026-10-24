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
pub struct types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr____types_type_list_cpp17_input_iterator_int_ptr__int_ptr___
{}
pub fn for_each_0(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__,
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
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub fn swallow_1(mut _a0: (), mut _a1: (), mut _a2: (), mut _a3: (), mut _a4: (), mut _a5: ()) {}
pub fn base_2(i: Ptr<cpp17_output_iterator_int_ptr_>) -> Ptr<()> {
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
    pub fn new(mut it: Ptr<()>) -> Self {
        Self {
            it_: (unimplemented!().read()),
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
pub fn base_3(i: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>) -> Ptr<()> {
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
pub fn base_5(i: Ptr<bidirectional_iterator_int_ptr_>) -> Ptr<()> {
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
pub fn base_6(i: Ptr<random_access_iterator_int_ptr_>) -> Ptr<()> {
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
pub fn base_7(i: Ptr<contiguous_iterator_int_ptr_>) -> Ptr<()> {
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
pub fn base_8(i: Ptr<()>) -> Ptr<()> {
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
pub fn base_9(i: Ptr<cpp20_output_iterator_int_ptr_>) -> Ptr<()> {
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
impl sentinel_wrapper_int_ptr_ {
    pub fn new_1(it: Ptr<Ptr<()>>) -> Self {
        Self {
            base_: ({ base_8((it.read())) }),
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
            base_: ({ base_7((it).clone()) }),
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
            base_: ({ base_6((it).clone()) }),
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
            base_: ({ base_5((it).clone()) }),
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
pub struct sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ {
    pub fn new_1(it: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>) -> Self {
        Self {
            base_: ({ base_3((it).clone()) }),
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
            base_: ({ base_2((it).clone()) }),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_cpp20_output_iterator_int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_cpp20_output_iterator_int_ptr__ {
    pub fn new_1(it: Ptr<cpp20_output_iterator_int_ptr_>) -> Self {
        Self {
            base_: ({ base_9((it).clone()) }),
        }
    }
}
pub fn operator_sub_10(s: Ptr<sized_sentinel_int_ptr_>, i: Ptr<Ptr<()>>) -> () {
    return (s.with(|__s| __s.base_.clone()) - ({ base_8((i.read())) })) as ();
}
pub fn operator_sub_11(i: Ptr<Ptr<()>>, s: Ptr<sized_sentinel_int_ptr_>) -> () {
    return (({ base_8((i.read())) }) - s.with(|__s| __s.base_.clone())) as ();
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
            base_: ({ base_8((it.read())) }),
        }
    }
}
pub fn operator_eq_12(
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
            operator_eq_12(
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
pub fn operator_add_13(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_14(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_13(_i, n)
    });
}
pub fn iter_swap_15(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_16(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return unimplemented!();
}
pub fn operator_eq_17(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_18(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
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
            operator_cmp_18(
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
            operator_eq_17(
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
pub struct UncheckedRange_BidirectionalIteratorNotDerivedFrom__sentinel_wrapper_BidirectionalIteratorNotDerivedFrom__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct UncheckedRange_BidirectionalIteratorNotDecrementable__sentinel_wrapper_BidirectionalIteratorNotDecrementable__
{}
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
const _: () = assert!(true, "HasIotaIter<int*, int*, int>");
const _: () = assert!(
    !(false),
    "!HasIotaIter<InputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(!(false), "!HasIotaIter<int*, SentinelForNotSemiregular>");
const _: () = assert!(
    !(false),
    "!HasIotaIter<int*, SentinelForNotWeaklyEqualityComparableWith>"
);
const _: () = assert!(
    !(false),
    "!HasIotaIter<int*, int*, WeaklyIncrementableNotMovable>"
);
const _: () = assert!(
    !(false),
    "!HasIotaIter<OutputIteratorNotIndirectlyWritable, int*, int>"
);
const _: () = assert!(true, "HasIotaRange<UncheckedRange<int*>, int>");
const _: () = assert!(
    !(false),
    "!HasIotaRange<UncheckedRange<int*>, WeaklyIncrementableNotMovable>"
);
const _: () = assert!(
    !(false),
    "!HasIotaRange<UncheckedRange<int*>, OutputIteratorNotIndirectlyWritable>"
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(4)]
pub struct DangerousCopyAssign {
    #[offset(0)]
    pub val: (),
}
impl DangerousCopyAssign {
    pub fn new(mut v: ()) -> Self {
        Self { val: v }
    }
    pub fn copy_from(other: Ptr<DangerousCopyAssign>) -> Self {
        let __this: Value<DangerousCopyAssign> = Rc::new(RefCell::new(Self {
            val: unimplemented!(),
        }));
        let this: Ptr<DangerousCopyAssign> = __this.as_pointer();
        field!(this, val).write({ other.with(|__s| __s.val) });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for DangerousCopyAssign {
    fn eq(&self, other: &Self) -> bool {
        {
            DangerousCopyAssignImpl::operator_eq(
                &Rc::new(RefCell::new(DangerousCopyAssign {
                    val: self.val.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(DangerousCopyAssign {
                    val: other.val.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for DangerousCopyAssign {}
impl Clone for DangerousCopyAssign {
    fn clone(&self) -> Self {
        let __src: Value<DangerousCopyAssign> = Rc::new(RefCell::new(DangerousCopyAssign {
            val: self.val.clone(),
        }));
        DangerousCopyAssign::copy_from(__src.as_pointer())
    }
}
pub fn test_result_19(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: Ptr<()> = unimplemented!();
        let in_end: Value<sentinel_wrapper_int_ptr_> = Rc::new(RefCell::new({
            let __tmp_0: Value<Ptr<()>> = Rc::new(RefCell::new(
                unimplemented!().offset((unimplemented!()) as isize),
            ));
            sentinel_wrapper_int_ptr_::new_1({ __tmp_0.as_pointer() })
        }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = (unimplemented!().read());
                let ___value: () = starting_value;
                (*iota_20.with(Value::clone).borrow_mut())(
                    ___first,
                    (*unimplemented!().upgrade().deref()).clone(),
                    ___value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: Ptr<()> = unimplemented!();
        let in_end: Value<sentinel_wrapper_int_ptr_> = Rc::new(RefCell::new({
            let __tmp_1: Value<Ptr<()>> = Rc::new(RefCell::new(
                unimplemented!().offset((unimplemented!()) as isize),
            ));
            sentinel_wrapper_int_ptr_::new_1({ __tmp_1.as_pointer() })
        }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_21(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: Ptr<()> = unimplemented!();
        let in_end: Value<sentinel_wrapper_int_ptr_> = Rc::new(RefCell::new({
            let __tmp_2: Value<Ptr<()>> = Rc::new(RefCell::new(
                unimplemented!().offset((unimplemented!()) as isize),
            ));
            sentinel_wrapper_int_ptr_::new_1({ __tmp_2.as_pointer() })
        }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = (unimplemented!().read());
                let ___value: () = starting_value;
                (*iota_20.with(Value::clone).borrow_mut())(
                    ___first,
                    (*unimplemented!().upgrade().deref()).clone(),
                    ___value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: Ptr<()> = unimplemented!();
        let in_end: Value<sentinel_wrapper_int_ptr_> = Rc::new(RefCell::new({
            let __tmp_3: Value<Ptr<()>> = Rc::new(RefCell::new(
                unimplemented!().offset((unimplemented!()) as isize),
            ));
            sentinel_wrapper_int_ptr_::new_1({ __tmp_3.as_pointer() })
        }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_22(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: Ptr<()> = unimplemented!();
        let in_end: Value<sentinel_wrapper_int_ptr_> = Rc::new(RefCell::new({
            let __tmp_4: Value<Ptr<()>> = Rc::new(RefCell::new(
                unimplemented!().offset((unimplemented!()) as isize),
            ));
            sentinel_wrapper_int_ptr_::new_1({ __tmp_4.as_pointer() })
        }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = (unimplemented!().read());
                let ___value: () = starting_value;
                (*iota_20.with(Value::clone).borrow_mut())(
                    ___first,
                    (*unimplemented!().upgrade().deref()).clone(),
                    ___value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: Ptr<()> = unimplemented!();
        let in_end: Value<sentinel_wrapper_int_ptr_> = Rc::new(RefCell::new({
            let __tmp_5: Value<Ptr<()>> = Rc::new(RefCell::new(
                unimplemented!().offset((unimplemented!()) as isize),
            ));
            sentinel_wrapper_int_ptr_::new_1({ __tmp_5.as_pointer() })
        }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_23(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_contiguous_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_6: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_6.as_pointer() })
            }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*iota_20.with(Value::clone).borrow_mut())(
                    contiguous_iterator_int_ptr_::move_from({ unimplemented!() }),
                    (*unimplemented!().upgrade().deref()).clone(),
                    starting_value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_contiguous_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_7: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_7.as_pointer() })
            }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_24(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_contiguous_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_8: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_8.as_pointer() })
            }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*iota_20.with(Value::clone).borrow_mut())(
                    contiguous_iterator_int_ptr_::move_from({ unimplemented!() }),
                    (*unimplemented!().upgrade().deref()).clone(),
                    starting_value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_contiguous_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_9: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_9.as_pointer() })
            }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_25(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_contiguous_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_10: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_10.as_pointer() })
            }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*iota_20.with(Value::clone).borrow_mut())(
                    contiguous_iterator_int_ptr_::move_from({ unimplemented!() }),
                    (*unimplemented!().upgrade().deref()).clone(),
                    starting_value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_contiguous_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_11: Value<contiguous_iterator_int_ptr_> =
                    Rc::new(RefCell::new(contiguous_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ __tmp_11.as_pointer() })
            }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_26(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_random_access_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_12: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ __tmp_12.as_pointer() })
            }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*iota_20.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::move_from({ unimplemented!() }),
                    (*unimplemented!().upgrade().deref()).clone(),
                    starting_value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_random_access_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_13: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ __tmp_13.as_pointer() })
            }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_27(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_random_access_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_14: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ __tmp_14.as_pointer() })
            }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*iota_20.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::move_from({ unimplemented!() }),
                    (*unimplemented!().upgrade().deref()).clone(),
                    starting_value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_random_access_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_15: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ __tmp_15.as_pointer() })
            }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_28(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_random_access_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_16: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ __tmp_16.as_pointer() })
            }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*iota_20.with(Value::clone).borrow_mut())(
                    random_access_iterator_int_ptr_::move_from({ unimplemented!() }),
                    (*unimplemented!().upgrade().deref()).clone(),
                    starting_value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_random_access_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_17: Value<random_access_iterator_int_ptr_> =
                    Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ __tmp_17.as_pointer() })
            }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_29(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_bidirectional_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_18: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ __tmp_18.as_pointer() })
            }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*iota_20.with(Value::clone).borrow_mut())(
                    bidirectional_iterator_int_ptr_::move_from({ unimplemented!() }),
                    (*unimplemented!().upgrade().deref()).clone(),
                    starting_value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_bidirectional_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_19: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ __tmp_19.as_pointer() })
            }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_30(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_bidirectional_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_20: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ __tmp_20.as_pointer() })
            }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*iota_20.with(Value::clone).borrow_mut())(
                    bidirectional_iterator_int_ptr_::move_from({ unimplemented!() }),
                    (*unimplemented!().upgrade().deref()).clone(),
                    starting_value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_bidirectional_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_21: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ __tmp_21.as_pointer() })
            }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_31(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_bidirectional_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_22: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ __tmp_22.as_pointer() })
            }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*iota_20.with(Value::clone).borrow_mut())(
                    bidirectional_iterator_int_ptr_::move_from({ unimplemented!() }),
                    (*unimplemented!().upgrade().deref()).clone(),
                    starting_value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_bidirectional_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_23: Value<bidirectional_iterator_int_ptr_> =
                    Rc::new(RefCell::new(bidirectional_iterator_int_ptr_::new_1({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ __tmp_23.as_pointer() })
            }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_32(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_forward_iterator_int_ptr__> = Rc::new(RefCell::new({
            let __tmp_24: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                })));
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_24.as_pointer() })
        }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*iota_20.with(Value::clone).borrow_mut())(
                    forward_iterator_int_ptr_::move_from({ unimplemented!() }),
                    (*unimplemented!().upgrade().deref()).clone(),
                    starting_value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_forward_iterator_int_ptr__> = Rc::new(RefCell::new({
            let __tmp_25: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                })));
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_25.as_pointer() })
        }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_33(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_forward_iterator_int_ptr__> = Rc::new(RefCell::new({
            let __tmp_26: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                })));
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_26.as_pointer() })
        }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*iota_20.with(Value::clone).borrow_mut())(
                    forward_iterator_int_ptr_::move_from({ unimplemented!() }),
                    (*unimplemented!().upgrade().deref()).clone(),
                    starting_value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_forward_iterator_int_ptr__> = Rc::new(RefCell::new({
            let __tmp_27: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                })));
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_27.as_pointer() })
        }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_34(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_forward_iterator_int_ptr__> = Rc::new(RefCell::new({
            let __tmp_28: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                })));
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_28.as_pointer() })
        }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*iota_20.with(Value::clone).borrow_mut())(
                    forward_iterator_int_ptr_::move_from({ unimplemented!() }),
                    (*unimplemented!().upgrade().deref()).clone(),
                    starting_value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_forward_iterator_int_ptr__> = Rc::new(RefCell::new({
            let __tmp_29: Value<forward_iterator_int_ptr_> =
                Rc::new(RefCell::new(forward_iterator_int_ptr_::new_1({
                    unimplemented!().offset((unimplemented!()) as isize)
                })));
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ __tmp_29.as_pointer() })
        }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_35(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_30: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                    Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                    __tmp_30.as_pointer()
                })
            }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*iota_20.with(Value::clone).borrow_mut())(
                    cpp17_input_iterator_int_ptr__int_ptr_::move_from({ unimplemented!() }),
                    (*unimplemented!().upgrade().deref()).clone(),
                    starting_value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_31: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                    Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                    __tmp_31.as_pointer()
                })
            }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_36(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_32: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                    Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                    __tmp_32.as_pointer()
                })
            }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*iota_20.with(Value::clone).borrow_mut())(
                    cpp17_input_iterator_int_ptr__int_ptr_::move_from({ unimplemented!() }),
                    (*unimplemented!().upgrade().deref()).clone(),
                    starting_value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_33: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                    Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                    __tmp_33.as_pointer()
                })
            }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_37(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_34: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                    Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                    __tmp_34.as_pointer()
                })
            }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*iota_20.with(Value::clone).borrow_mut())(
                    cpp17_input_iterator_int_ptr__int_ptr_::move_from({ unimplemented!() }),
                    (*unimplemented!().upgrade().deref()).clone(),
                    starting_value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_35: Value<cpp17_input_iterator_int_ptr__int_ptr_> =
                    Rc::new(RefCell::new(cpp17_input_iterator_int_ptr__int_ptr_::new({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({
                    __tmp_35.as_pointer()
                })
            }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_38(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_cpp17_output_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_36: Value<cpp17_output_iterator_int_ptr_> =
                    Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ __tmp_36.as_pointer() })
            }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*iota_20.with(Value::clone).borrow_mut())(
                    cpp17_output_iterator_int_ptr_::move_from({ unimplemented!() }),
                    (*unimplemented!().upgrade().deref()).clone(),
                    starting_value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_cpp17_output_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_37: Value<cpp17_output_iterator_int_ptr_> =
                    Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ __tmp_37.as_pointer() })
            }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_39(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_cpp17_output_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_38: Value<cpp17_output_iterator_int_ptr_> =
                    Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ __tmp_38.as_pointer() })
            }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*iota_20.with(Value::clone).borrow_mut())(
                    cpp17_output_iterator_int_ptr_::move_from({ unimplemented!() }),
                    (*unimplemented!().upgrade().deref()).clone(),
                    starting_value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_cpp17_output_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_39: Value<cpp17_output_iterator_int_ptr_> =
                    Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ __tmp_39.as_pointer() })
            }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_40(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_cpp17_output_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_40: Value<cpp17_output_iterator_int_ptr_> =
                    Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ __tmp_40.as_pointer() })
            }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*iota_20.with(Value::clone).borrow_mut())(
                    cpp17_output_iterator_int_ptr_::move_from({ unimplemented!() }),
                    (*unimplemented!().upgrade().deref()).clone(),
                    starting_value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_cpp17_output_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_41: Value<cpp17_output_iterator_int_ptr_> =
                    Rc::new(RefCell::new(cpp17_output_iterator_int_ptr_::new({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_cpp17_output_iterator_int_ptr__::new_1({ __tmp_41.as_pointer() })
            }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_41(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: cpp20_output_iterator_int_ptr_ =
            cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_cpp20_output_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_42: Value<cpp20_output_iterator_int_ptr_> =
                    Rc::new(RefCell::new(cpp20_output_iterator_int_ptr_::new({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_cpp20_output_iterator_int_ptr__::new_1({ __tmp_42.as_pointer() })
            }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*iota_20.with(Value::clone).borrow_mut())(
                    cpp20_output_iterator_int_ptr_::move_from({ unimplemented!() }),
                    (*unimplemented!().upgrade().deref()).clone(),
                    starting_value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: cpp20_output_iterator_int_ptr_ =
            cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_cpp20_output_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_43: Value<cpp20_output_iterator_int_ptr_> =
                    Rc::new(RefCell::new(cpp20_output_iterator_int_ptr_::new({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_cpp20_output_iterator_int_ptr__::new_1({ __tmp_43.as_pointer() })
            }));
        let range : Value<() > = Rc::new(RefCell::new(std_ranges_subrange_cpp20_output_iterator_int_ptr___sentinel_wrapper_cpp20_output_iterator_int_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { cpp20_output_iterator_int_ptr_ :: move_from ( { unimplemented!()   } , )   } , { ((*unimplemented!() .upgrade().deref()) ).clone()  } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_42(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: cpp20_output_iterator_int_ptr_ =
            cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_cpp20_output_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_44: Value<cpp20_output_iterator_int_ptr_> =
                    Rc::new(RefCell::new(cpp20_output_iterator_int_ptr_::new({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_cpp20_output_iterator_int_ptr__::new_1({ __tmp_44.as_pointer() })
            }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*iota_20.with(Value::clone).borrow_mut())(
                    cpp20_output_iterator_int_ptr_::move_from({ unimplemented!() }),
                    (*unimplemented!().upgrade().deref()).clone(),
                    starting_value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: cpp20_output_iterator_int_ptr_ =
            cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_cpp20_output_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_45: Value<cpp20_output_iterator_int_ptr_> =
                    Rc::new(RefCell::new(cpp20_output_iterator_int_ptr_::new({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_cpp20_output_iterator_int_ptr__::new_1({ __tmp_45.as_pointer() })
            }));
        let range : Value<() > = Rc::new(RefCell::new(std_ranges_subrange_cpp20_output_iterator_int_ptr___sentinel_wrapper_cpp20_output_iterator_int_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { cpp20_output_iterator_int_ptr_ :: move_from ( { unimplemented!()   } , )   } , { ((*unimplemented!() .upgrade().deref()) ).clone()  } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_43(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: cpp20_output_iterator_int_ptr_ =
            cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_cpp20_output_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_46: Value<cpp20_output_iterator_int_ptr_> =
                    Rc::new(RefCell::new(cpp20_output_iterator_int_ptr_::new({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_cpp20_output_iterator_int_ptr__::new_1({ __tmp_46.as_pointer() })
            }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*iota_20.with(Value::clone).borrow_mut())(
                    cpp20_output_iterator_int_ptr_::move_from({ unimplemented!() }),
                    (*unimplemented!().upgrade().deref()).clone(),
                    starting_value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: cpp20_output_iterator_int_ptr_ =
            cpp20_output_iterator_int_ptr_::new({ unimplemented!() });
        let in_end: Value<sentinel_wrapper_cpp20_output_iterator_int_ptr__> =
            Rc::new(RefCell::new({
                let __tmp_47: Value<cpp20_output_iterator_int_ptr_> =
                    Rc::new(RefCell::new(cpp20_output_iterator_int_ptr_::new({
                        unimplemented!().offset((unimplemented!()) as isize)
                    })));
                sentinel_wrapper_cpp20_output_iterator_int_ptr__::new_1({ __tmp_47.as_pointer() })
            }));
        let range : Value<() > = Rc::new(RefCell::new(std_ranges_subrange_cpp20_output_iterator_int_ptr___sentinel_wrapper_cpp20_output_iterator_int_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { cpp20_output_iterator_int_ptr_ :: move_from ( { unimplemented!()   } , )   } , { ((*unimplemented!() .upgrade().deref()) ).clone()  } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_44(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: Ptr<()> = unimplemented!();
        let in_end: Value<sized_sentinel_int_ptr_> = Rc::new(RefCell::new({
            let __tmp_48: Value<Ptr<()>> = Rc::new(RefCell::new(
                unimplemented!().offset((unimplemented!()) as isize),
            ));
            sized_sentinel_int_ptr_::new_1({ __tmp_48.as_pointer() })
        }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = (unimplemented!().read());
                let ___value: () = starting_value;
                (*iota_20.with(Value::clone).borrow_mut())(
                    ___first,
                    (*unimplemented!().upgrade().deref()).clone(),
                    ___value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: Ptr<()> = unimplemented!();
        let in_end: Value<sized_sentinel_int_ptr_> = Rc::new(RefCell::new({
            let __tmp_49: Value<Ptr<()>> = Rc::new(RefCell::new(
                unimplemented!().offset((unimplemented!()) as isize),
            ));
            sized_sentinel_int_ptr_::new_1({ __tmp_49.as_pointer() })
        }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_45(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: Ptr<()> = unimplemented!();
        let in_end: Value<sized_sentinel_int_ptr_> = Rc::new(RefCell::new({
            let __tmp_50: Value<Ptr<()>> = Rc::new(RefCell::new(
                unimplemented!().offset((unimplemented!()) as isize),
            ));
            sized_sentinel_int_ptr_::new_1({ __tmp_50.as_pointer() })
        }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = (unimplemented!().read());
                let ___value: () = starting_value;
                (*iota_20.with(Value::clone).borrow_mut())(
                    ___first,
                    (*unimplemented!().upgrade().deref()).clone(),
                    ___value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: Ptr<()> = unimplemented!();
        let in_end: Value<sized_sentinel_int_ptr_> = Rc::new(RefCell::new({
            let __tmp_51: Value<Ptr<()>> = Rc::new(RefCell::new(
                unimplemented!().offset((unimplemented!()) as isize),
            ));
            sized_sentinel_int_ptr_::new_1({ __tmp_51.as_pointer() })
        }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_result_46(input: (), mut starting_value: (), expected: ()) {
    let input: Value<()> = Rc::new(RefCell::new(input));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    {
        let mut in_begin: Ptr<()> = unimplemented!();
        let in_end: Value<sized_sentinel_int_ptr_> = Rc::new(RefCell::new({
            let __tmp_52: Value<Ptr<()>> = Rc::new(RefCell::new(
                unimplemented!().offset((unimplemented!()) as isize),
            ));
            sized_sentinel_int_ptr_::new_1({ __tmp_52.as_pointer() })
        }));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<()> = (unimplemented!().read());
                let ___value: () = starting_value;
                (*iota_20.with(Value::clone).borrow_mut())(
                    ___first,
                    (*unimplemented!().upgrade().deref()).clone(),
                    ___value,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_begin: Ptr<()> = unimplemented!();
        let in_end: Value<sized_sentinel_int_ptr_> = Rc::new(RefCell::new({
            let __tmp_53: Value<Ptr<()>> = Rc::new(RefCell::new(
                unimplemented!().offset((unimplemented!()) as isize),
            ));
            sized_sentinel_int_ptr_::new_1({ __tmp_53.as_pointer() })
        }));
        let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(
            ({ (*iota_20.with(Value::clone).borrow_mut())(range.as_pointer(), starting_value) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_results_47() {
    ({ test_result_19(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_result_21(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_result_22(unimplemented!(), 0_(), unimplemented!()) });
}
pub fn test_results_48() {
    ({ test_result_23(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_result_24(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_result_25(unimplemented!(), 0_(), unimplemented!()) });
}
pub fn test_results_49() {
    ({ test_result_26(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_result_27(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_result_28(unimplemented!(), 0_(), unimplemented!()) });
}
pub fn test_results_50() {
    ({ test_result_29(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_result_30(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_result_31(unimplemented!(), 0_(), unimplemented!()) });
}
pub fn test_results_51() {
    ({ test_result_32(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_result_33(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_result_34(unimplemented!(), 0_(), unimplemented!()) });
}
pub fn test_results_52() {
    ({ test_result_35(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_result_36(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_result_37(unimplemented!(), 0_(), unimplemented!()) });
}
pub fn test_results_53() {
    ({ test_result_38(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_result_39(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_result_40(unimplemented!(), 0_(), unimplemented!()) });
}
pub fn test_results_54() {
    ({ test_result_41(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_result_42(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_result_43(unimplemented!(), 0_(), unimplemented!()) });
}
pub fn test_results_55() {
    ({ test_result_44(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_result_45(unimplemented!(), 0_(), unimplemented!()) });
    ({ test_result_46(unimplemented!(), 0_(), unimplemented!()) });
}
pub fn test_user_defined_type_56() {
    let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({
        (*iota_20.with(Value::clone).borrow_mut())(
            a.as_pointer(),
            UserDefinedType_57::new({ 0_() }),
        )
    });
    let proj_val: Value<FnPtr<fn(Ptr<UserDefinedType_57>) -> ()>> =
        Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<UserDefinedType_57>) -> ()>::new(|el: Ptr<UserDefinedType_57>| -> () {
                {
                    return el.with(|__s| __s.val);
                }
            }),
        ));
    unimplemented!();
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(4)]
pub struct UserDefinedType_57 {
    #[offset(0)]
    pub val: (),
}
impl UserDefinedType_57 {
    pub fn new(mut v: ()) -> Self {
        Self { val: v }
    }
    pub fn copy_from(other: Ptr<UserDefinedType_57>) -> Self {
        let __this: Value<UserDefinedType_57> = Rc::new(RefCell::new(Self {
            val: unimplemented!(),
        }));
        let this: Ptr<UserDefinedType_57> = __this.as_pointer();
        field!(this, val).write({ other.with(|__s| __s.val) });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for UserDefinedType_57 {
    fn clone(&self) -> Self {
        let __src: Value<UserDefinedType_57> = Rc::new(RefCell::new(UserDefinedType_57 {
            val: self.val.clone(),
        }));
        UserDefinedType_57::copy_from(__src.as_pointer())
    }
}
pub fn test_dangerous_copy_assign_58() {
    let aa: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({
        (*iota_20.with(Value::clone).borrow_mut())(
            aa.as_pointer(),
            DangerousCopyAssign::new({ 0_() }),
        )
    });
    let proj_val: Value<FnPtr<fn(Ptr<DangerousCopyAssign>) -> ()>> = Rc::new(RefCell::new(
        FnPtr::<fn(Ptr<DangerousCopyAssign>) -> ()>::new(|el: Ptr<DangerousCopyAssign>| -> () {
            {
                return el.with(|__s| __s.val);
            }
        }),
    ));
    unimplemented!();
}
pub fn test_results_59() -> () {
    ({
        for_each_0 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__  >::default()  , lambda! ( Generic , { } , | | { ( { test_results_47 ( ) } )  ;
 } , | | { ( { test_results_48 ( ) } )  ;
 } , | | { ( { test_results_49 ( ) } )  ;
 } , | | { ( { test_results_50 ( ) } )  ;
 } , | | { ( { test_results_51 ( ) } )  ;
 } , | | { ( { test_results_52 ( ) } )  ;
 } )  , )
    });
    ({ test_results_53() });
    ({ test_results_54() });
    ({ test_results_55() });
    ({ test_user_defined_type_56() });
    ({ test_dangerous_copy_assign_58() });
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
    ({ test_results_59() });
    const _: () = assert!(true, "test_results()");;
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
pub trait DangerousCopyAssignImpl {
    fn copy_assign(&self, a: Ptr<DangerousCopyAssign>) -> Ptr<DangerousCopyAssign>;
    fn operator_eq(&self, rhs: Ptr<DangerousCopyAssign>) -> ();
    fn operator_inc_6(&self) -> Ptr<DangerousCopyAssign>;
    fn operator_post_inc_7(&self, _a0: ()) -> DangerousCopyAssign;
}
impl DangerousCopyAssignImpl for Ptr<DangerousCopyAssign> {
    fn copy_assign(&self, a: Ptr<DangerousCopyAssign>) -> Ptr<DangerousCopyAssign> {
        field!((*self), val).write({ a.with(|__s| __s.val) });
        return (*self).clone();
    }
    fn operator_eq(&self, rhs: Ptr<DangerousCopyAssign>) -> () {
        return ({ (*self).with(|__s| __s.val) } == { rhs.with(|__s| __s.val) });
    }
    fn operator_inc_6(&self) -> Ptr<DangerousCopyAssign> {
        field!((*self), val).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_post_inc_7(&self, mut _a0: ()) -> DangerousCopyAssign {
        let tmp: Value<DangerousCopyAssign> =
            Rc::new(RefCell::new(DangerousCopyAssign::copy_from({
                (*self).clone()
            })));
        field!((*self), val).with_mut(|__v| __v.prefix_inc());
        return DangerousCopyAssign::copy_from({ tmp.as_pointer() });
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
pub trait UserDefinedType_57Impl {
    fn copy_assign(&self, a: Ptr<UserDefinedType_57>) -> Ptr<UserDefinedType_57>;
    fn operator_inc_4(&self) -> Ptr<UserDefinedType_57>;
    fn operator_post_inc_5(&self, _a0: ()) -> UserDefinedType_57;
}
impl UserDefinedType_57Impl for Ptr<UserDefinedType_57> {
    fn copy_assign(&self, a: Ptr<UserDefinedType_57>) -> Ptr<UserDefinedType_57> {
        field!((*self), val).write({ a.with(|__s| __s.val) });
        return (*self).clone();
    }
    fn operator_inc_4(&self) -> Ptr<UserDefinedType_57> {
        field!((*self), val).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_post_inc_5(&self, mut _a0: ()) -> UserDefinedType_57 {
        let tmp: Value<UserDefinedType_57> =
            Rc::new(RefCell::new(UserDefinedType_57::copy_from({
                (*self).clone()
            })));
        field!((*self), val).with_mut(|__v| __v.prefix_inc());
        return UserDefinedType_57::copy_from({ tmp.as_pointer() });
    }
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
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
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
}
impl contiguous_iterator_int_ptr_Impl for Ptr<contiguous_iterator_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait cpp17_input_iterator_int_ptr__int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_>;
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp17_input_iterator_int_ptr__int_ptr_ {
        unimplemented!()
    }
}
impl cpp17_input_iterator_int_ptr__int_ptr_Impl for Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait cpp17_output_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_5(&self) -> Ptr<cpp17_output_iterator_int_ptr_>;
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp17_output_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl cpp17_output_iterator_int_ptr_Impl for Ptr<cpp17_output_iterator_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
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
}
impl forward_iterator_int_ptr_Impl for Ptr<forward_iterator_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
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
}
impl random_access_iterator_int_ptr_Impl for Ptr<random_access_iterator_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
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
pub trait sentinel_wrapper_bidirectional_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<bidirectional_iterator_int_ptr_>) -> ();
}
impl sentinel_wrapper_bidirectional_iterator_int_ptr__Impl
    for Ptr<sentinel_wrapper_bidirectional_iterator_int_ptr__>
{
    fn operator_eq(&self, other: Ptr<bidirectional_iterator_int_ptr_>) -> () {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_5((other).clone()) }) });
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
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_7((other).clone()) }) });
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
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_3((other).clone()) }) });
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
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_2((other).clone()) }) });
    }
}
pub trait sentinel_wrapper_cpp20_output_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<cpp20_output_iterator_int_ptr_>) -> ();
}
impl sentinel_wrapper_cpp20_output_iterator_int_ptr__Impl
    for Ptr<sentinel_wrapper_cpp20_output_iterator_int_ptr__>
{
    fn operator_eq(&self, other: Ptr<cpp20_output_iterator_int_ptr_>) -> () {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_9((other).clone()) }) });
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
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_8((other.read())) }) });
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
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_6((other).clone()) }) });
    }
}
pub trait sized_sentinel_int_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> ();
}
impl sized_sentinel_int_ptr_Impl for Ptr<sized_sentinel_int_ptr_> {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> () {
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_8((other.read())) }) });
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
