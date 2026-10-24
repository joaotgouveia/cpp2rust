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
pub fn operator_eq_0(
    x: Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>,
    y: Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    pub fn new(mut it: Ptr<()>) -> Self {
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
impl std::cmp::PartialEq for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_0(
                Rc::new(RefCell::new(
                    cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
                        it_: self.it_.clone(),
                        tracker_: support_double_move_tracker {
                            moved_from_: self.tracker_.moved_from_.clone(),
                        },
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
                        it_: other.it_.clone(),
                        tracker_: support_double_move_tracker {
                            moved_from_: other.tracker_.moved_from_.clone(),
                        },
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub fn operator_eq_1(
    x: Ptr<forward_iterator_const_int_ptr_>,
    y: Ptr<forward_iterator_const_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
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
impl std::cmp::PartialEq for forward_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_1(
                Rc::new(RefCell::new(forward_iterator_const_int_ptr_ {
                    it_: self.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: self.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
                Rc::new(RefCell::new(forward_iterator_const_int_ptr_ {
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
impl std::cmp::Eq for forward_iterator_const_int_ptr_ {}
pub fn operator_eq_2(
    x: Ptr<bidirectional_iterator_const_int_ptr_>,
    y: Ptr<bidirectional_iterator_const_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
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
impl std::cmp::PartialEq for bidirectional_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_2(
                Rc::new(RefCell::new(bidirectional_iterator_const_int_ptr_ {
                    it_: self.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: self.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
                Rc::new(RefCell::new(bidirectional_iterator_const_int_ptr_ {
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
impl std::cmp::Eq for bidirectional_iterator_const_int_ptr_ {}
pub fn operator_sub_3(
    mut x: random_access_iterator_const_int_ptr_,
    mut y: random_access_iterator_const_int_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn operator_eq_4(
    x: Ptr<random_access_iterator_const_int_ptr_>,
    y: Ptr<random_access_iterator_const_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
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
impl std::cmp::PartialEq for random_access_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_4(
                Rc::new(RefCell::new(random_access_iterator_const_int_ptr_ {
                    it_: self.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: self.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
                Rc::new(RefCell::new(random_access_iterator_const_int_ptr_ {
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
impl std::cmp::Eq for random_access_iterator_const_int_ptr_ {}
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
pub fn operator_sub_5(
    mut x: contiguous_iterator_const_int_ptr_,
    mut y: contiguous_iterator_const_int_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn operator_eq_6(
    x: Ptr<contiguous_iterator_const_int_ptr_>,
    y: Ptr<contiguous_iterator_const_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
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
impl std::cmp::PartialEq for contiguous_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_6(
                Rc::new(RefCell::new(contiguous_iterator_const_int_ptr_ {
                    it_: self.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: self.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
                Rc::new(RefCell::new(contiguous_iterator_const_int_ptr_ {
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
impl std::cmp::Eq for contiguous_iterator_const_int_ptr_ {}
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
pub fn operator_eq_7(
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
            operator_eq_7(
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
pub fn operator_add_8(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_9(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_8(_i, n)
    });
}
pub fn iter_swap_10(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_11(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return (unimplemented!()).reinterpret_cast::<()>();
}
pub fn operator_eq_12(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_13(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
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
            operator_cmp_13(
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
            operator_eq_12(
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
thread_local!(
    pub static compare_last_digit_strong_14: Value<FnPtr<fn((), ()) -> ()>> =
        Rc::new(RefCell::new(FnPtr::<fn((), ()) -> ()>::new(
            |mut a: (), mut b: ()| -> () {
                {
                    return std::cmp::Ord::cmp(&(a % 10_()), &(b % 10_()));
                }
            },
        )));
);
thread_local!(
    pub static compare_last_digit_weak_15: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        FnPtr::<fn((), ()) -> ()>::new(|mut a: (), mut b: ()| -> () {
            {
                return unimplemented!();
            }
        }),
    ));
);
thread_local!(
    pub static compare_last_digit_partial_16: Value<FnPtr<fn((), ()) -> ()>> =
        Rc::new(RefCell::new(FnPtr::<fn((), ()) -> ()>::new(
            |mut a: (), mut b: ()| -> () {
                {
                    if (a == unimplemented!()) || (b == unimplemented!()) {
                        return unimplemented!();
                    }
                    return unimplemented!();
                }
            },
        )));
);
thread_local!(
    pub static compare_int_result_17: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        FnPtr::<fn((), ()) -> ()>::new(|mut a: (), mut b: ()| -> () {
            {
                return ((a % 10_()) - (b % 10_()));
            }
        }),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct StructWithoutCallOperator {}
const _: () = assert!(
    true,
    "has_lexicographical_compare<decltype(compare_last_digit_strong)>"
);
const _: () = assert!(
    true,
    "has_lexicographical_compare<decltype(compare_last_digit_weak)>"
);
const _: () = assert!(
    true,
    "has_lexicographical_compare<decltype(compare_last_digit_partial)>"
);
const _: () = assert!(
    !(false),
    "!has_lexicographical_compare<StructWithoutCallOperator>"
);
const _: () = assert!(
    true,
    "has_lexicographical_compare<decltype(compare_int_result)>"
);
pub fn test_lexicographical_compare_18(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_19(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_20(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_21(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_22(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_23(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_24(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_25(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_26(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_27(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_28(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_29(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_30(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_31(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_32(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_33(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_34(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_35(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_36(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_37(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_38(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_39(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_40(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_41(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_42(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_43(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_44(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_45(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_46(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_47(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_48(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_49(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_50(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_51(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_52(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_53(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_54(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_55(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_56(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_57(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_58(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_59(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_60(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_61(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_62(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_63(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_64(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_65(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_66(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_67(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_68(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_69(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_70(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_71(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_72(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_73(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_74(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_75(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_76(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_77(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_78(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_79(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_80(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_81(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_82(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_83(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_84(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_85(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_86(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_87(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_88(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_89(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_90(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_91(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_92(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_93(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_94(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_95(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_96(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_97(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_98(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_99(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_100(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_101(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_102(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_103(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_104(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_105(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_106(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_107(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_108(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_109(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_110(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_111(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_112(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_113(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_114(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_115(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_116(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_117(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_118(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_119(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_120(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_121(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_122(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_123(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_124(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_125(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_126(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_127(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_128(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_129(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_130(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_131(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_132(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_133(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_134(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_135(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_136(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_137(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_138(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_139(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_140(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_141(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_142(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_143(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_144(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_145(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_146(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_147(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_148(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_149(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_150(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_151(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_152(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_153(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_154(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_155(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_156(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_157(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_158(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_159(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_160(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_161(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_162(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_163(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_164(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_165(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_166(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_167(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_168(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_169(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_170(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_171(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_172(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_173(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_174(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_175(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_176(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_177(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_178(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_179(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_180(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_181(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_182(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_183(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_184(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_185(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_186(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_187(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_188(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_189(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_190(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_191(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_192(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_193(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_194(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_195(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_196(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_197(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_198(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_199(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_200(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_201(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_202(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_203(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_204(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_205(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_206(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_207(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_208(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_209(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_210(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_211(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_212(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_213(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_214(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_215(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_216(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_217(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_218(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_219(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_220(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_221(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_222(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_223(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_224(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_225(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_226(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_227(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_228(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_229(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_230(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_231(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_232(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_233(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_234(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_235(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_236(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_237(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_238(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_239(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_240(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_241(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_242(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_243(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_244(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_245(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_246(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_247(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_248(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_249(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_250(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_251(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_252(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_253(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_254(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_255(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_256(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_257(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_258(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_259(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_260(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_261(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_262(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_263(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_264(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_265(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_266(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_267(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_268(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_269(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_270(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_271(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_272(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_273(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_274(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_275(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_276(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_277(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_278(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_279(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_280(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_281(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_282(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_283(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_284(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_285(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_286(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_287(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_288(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_289(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_290(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_291(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_292(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_293(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_294(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_295(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_296(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_297(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_298(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_299(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_300(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_301(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_302(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_303(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_304(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_305(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_306(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_307(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_308(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_309(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_310(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_311(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_312(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_313(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_314(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_315(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_316(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_317(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_318(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_lexicographical_compare_319(a: (), b: (), comp: FnPtr<fn((), ()) -> ()>, expected: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let comp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(comp));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_given_iterator_types_320() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_18(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_19(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_20(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_21(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_21(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_21(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_21(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_22(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_23(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_21(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_321() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_24(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_25(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_26(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_27(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_27(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_27(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_27(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_28(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_29(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_27(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_322() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_30(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_31(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_32(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_33(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_33(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_33(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_33(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_34(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_35(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_33(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_323() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_36(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_37(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_38(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_39(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_39(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_39(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_39(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_40(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_41(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_39(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_324() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_42(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_43(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_44(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_45(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_45(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_45(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_45(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_46(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_47(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_45(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_325() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_48(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_49(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_50(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_51(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_51(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_51(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_51(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_52(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_53(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_51(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_326() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_54(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_55(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_56(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_57(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_57(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_57(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_57(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_58(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_59(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_57(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_327() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_60(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_61(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_62(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_63(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_63(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_63(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_63(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_64(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_65(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_63(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_328() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_66(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_67(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_68(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_69(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_69(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_69(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_69(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_70(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_71(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_69(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_329() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_72(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_73(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_74(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_75(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_75(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_75(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_75(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_76(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_77(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_75(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_330() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_78(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_79(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_80(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_81(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_81(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_81(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_81(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_82(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_83(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_81(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_331() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_84(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_85(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_86(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_87(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_87(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_87(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_87(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_88(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_89(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_87(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_332() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_90(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_91(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_92(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_93(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_93(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_93(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_93(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_94(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_95(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_93(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_333() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_96(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_97(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_98(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_99(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_99(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_99(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_99(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_100(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_101(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_99(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_334() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_102(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_103(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_104(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_105(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_105(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_105(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_105(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_106(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_107(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_105(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_335() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_108(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_109(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_110(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_111(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_111(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_111(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_111(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_112(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_113(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_111(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_336() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_114(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_115(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_116(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_117(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_117(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_117(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_117(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_118(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_119(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_117(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_337() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_120(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_121(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_122(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_123(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_123(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_123(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_123(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_124(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_125(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_123(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_338() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_126(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_127(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_128(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_129(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_129(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_129(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_129(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_130(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_131(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_129(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_339() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_132(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_133(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_134(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_135(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_135(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_135(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_135(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_136(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_137(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_135(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_340() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_138(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_139(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_140(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_141(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_141(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_141(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_141(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_142(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_143(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_141(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_341() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_144(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_145(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_146(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_147(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_147(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_147(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_147(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_148(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_149(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_147(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_342() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_150(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_151(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_152(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_153(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_153(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_153(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_153(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_154(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_155(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_153(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_343() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_156(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_157(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_158(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_159(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_159(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_159(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_159(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_160(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_161(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_159(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_344() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_162(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_163(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_164(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_165(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_165(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_165(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_165(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_166(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_167(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_165(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_345() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_168(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_169(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_170(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_171(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_171(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_171(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_171(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_172(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_173(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_171(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_346() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_174(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_175(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_176(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_177(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_177(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_177(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_177(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_178(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_179(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_177(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_347() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_180(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_181(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_182(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_183(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_183(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_183(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_183(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_184(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_185(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_183(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_348() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_186(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_187(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_188(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_189(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_189(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_189(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_189(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_190(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_191(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_189(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_349() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_192(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_193(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_194(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_195(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_195(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_195(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_195(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_196(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_197(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_195(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_350() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_198(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_199(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_200(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_201(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_201(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_201(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_201(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_202(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_203(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_201(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_351() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_204(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_205(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_206(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_207(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_207(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_207(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_207(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_208(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_209(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_207(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_352() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_210(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_211(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_212(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_213(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_213(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_213(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_213(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_214(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_215(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_213(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_353() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_216(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_217(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_218(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_219(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_219(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_219(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_219(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_220(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_221(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_219(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_354() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_222(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_223(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_224(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_225(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_225(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_225(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_225(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_226(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_227(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_225(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_355() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_228(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_229(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_230(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_231(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_231(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_231(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_231(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_232(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_233(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_231(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_356() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_234(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_235(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_236(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_237(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_237(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_237(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_237(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_238(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_239(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_237(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_357() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_240(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_241(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_242(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_243(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_243(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_243(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_243(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_244(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_245(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_243(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_358() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_246(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_247(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_248(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_249(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_249(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_249(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_249(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_250(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_251(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_249(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_359() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_252(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_253(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_254(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_255(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_255(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_255(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_255(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_256(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_257(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_255(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_360() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_258(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_259(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_260(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_261(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_261(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_261(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_261(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_262(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_263(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_261(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_361() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_264(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_265(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_266(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_267(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_267(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_267(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_267(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_268(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_269(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_267(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_362() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_270(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_271(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_272(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_273(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_273(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_273(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_273(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_274(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_275(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_273(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_363() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_276(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_277(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_278(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_279(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_279(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_279(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_279(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_280(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_281(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_279(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_364() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_282(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_283(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_284(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_285(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_285(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_285(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_285(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_286(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_287(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_285(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_365() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_288(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_289(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_290(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_291(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_291(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_291(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_291(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_292(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_293(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_291(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_366() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_294(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_295(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_296(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_297(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_297(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_297(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_297(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_298(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_299(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_297(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_367() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_300(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_301(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_302(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_303(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_303(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_303(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_303(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_304(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_305(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_303(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_given_iterator_types_368() {
    let cmp: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
    ));
    ({
        test_lexicographical_compare_306(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_307(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_308(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_309(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_309(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_309(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_309(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_310(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_311(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_309(
            unimplemented!(),
            unimplemented!(),
            (*cmp.borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_iterator_types1_369() {
    ({ test_given_iterator_types_320() });
    ({ test_given_iterator_types_321() });
    ({ test_given_iterator_types_322() });
    ({ test_given_iterator_types_323() });
    ({ test_given_iterator_types_324() });
    ({ test_given_iterator_types_325() });
    ({ test_given_iterator_types_326() });
}
pub fn test_iterator_types1_370() {
    ({ test_given_iterator_types_327() });
    ({ test_given_iterator_types_328() });
    ({ test_given_iterator_types_329() });
    ({ test_given_iterator_types_330() });
    ({ test_given_iterator_types_331() });
    ({ test_given_iterator_types_332() });
    ({ test_given_iterator_types_333() });
}
pub fn test_iterator_types1_371() {
    ({ test_given_iterator_types_334() });
    ({ test_given_iterator_types_335() });
    ({ test_given_iterator_types_336() });
    ({ test_given_iterator_types_337() });
    ({ test_given_iterator_types_338() });
    ({ test_given_iterator_types_339() });
    ({ test_given_iterator_types_340() });
}
pub fn test_iterator_types1_372() {
    ({ test_given_iterator_types_341() });
    ({ test_given_iterator_types_342() });
    ({ test_given_iterator_types_343() });
    ({ test_given_iterator_types_344() });
    ({ test_given_iterator_types_345() });
    ({ test_given_iterator_types_346() });
    ({ test_given_iterator_types_347() });
}
pub fn test_iterator_types1_373() {
    ({ test_given_iterator_types_348() });
    ({ test_given_iterator_types_349() });
    ({ test_given_iterator_types_350() });
    ({ test_given_iterator_types_351() });
    ({ test_given_iterator_types_352() });
    ({ test_given_iterator_types_353() });
    ({ test_given_iterator_types_354() });
}
pub fn test_iterator_types1_374() {
    ({ test_given_iterator_types_355() });
    ({ test_given_iterator_types_356() });
    ({ test_given_iterator_types_357() });
    ({ test_given_iterator_types_358() });
    ({ test_given_iterator_types_359() });
    ({ test_given_iterator_types_360() });
    ({ test_given_iterator_types_361() });
}
pub fn test_iterator_types1_375() {
    ({ test_given_iterator_types_362() });
    ({ test_given_iterator_types_363() });
    ({ test_given_iterator_types_364() });
    ({ test_given_iterator_types_365() });
    ({ test_given_iterator_types_366() });
    ({ test_given_iterator_types_367() });
    ({ test_given_iterator_types_368() });
}
pub fn test_iterator_types_376() {
    ({ test_iterator_types1_369() });
    ({ test_iterator_types1_370() });
    ({ test_iterator_types1_371() });
    ({ test_iterator_types1_372() });
    ({ test_iterator_types1_373() });
    ({ test_iterator_types1_374() });
    ({ test_iterator_types1_375() });
}
pub fn test_comparison_categories_377() {
    ({
        test_lexicographical_compare_312(
            unimplemented!(),
            unimplemented!(),
            (*compare_last_digit_weak_15.with(Value::clone).borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_313(
            unimplemented!(),
            unimplemented!(),
            (*compare_last_digit_partial_16.with(Value::clone).borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_314(
            unimplemented!(),
            unimplemented!(),
            (*compare_last_digit_strong_14.with(Value::clone).borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_315(
            unimplemented!(),
            unimplemented!(),
            (*compare_last_digit_weak_15.with(Value::clone).borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_316(
            unimplemented!(),
            unimplemented!(),
            (*compare_last_digit_partial_16.with(Value::clone).borrow()).copy_from(),
            unimplemented!(),
        )
    });
    ({
        test_lexicographical_compare_317(
            unimplemented!(),
            unimplemented!(),
            (*compare_last_digit_partial_16.with(Value::clone).borrow()).copy_from(),
            unimplemented!(),
        )
    });
}
pub fn test_comparator_invocation_count_378() {
    let compare_invocation_count: Value<()> = Rc::new(RefCell::new(0_()));
    let compare_last_digit_counting: Value<FnPtr<fn((), ()) -> ()>> =
        Rc::new(RefCell::new(lambda!(
            {
                let compare_invocation_count: Ptr<()> = compare_invocation_count.as_pointer();
            },
            |mut a: (), mut b: ()| -> () {
                compare_invocation_count.with_mut(|__v| __v.prefix_inc());
                return std::cmp::Ord::cmp(&(a % 10_()), &(b % 10_()));
            }
        )));
    (*compare_invocation_count.borrow_mut()) = 0_();
    ({
        test_lexicographical_compare_318(
            unimplemented!(),
            unimplemented!(),
            (*compare_last_digit_counting.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    unimplemented!();
    ({
        test_lexicographical_compare_319(
            unimplemented!(),
            unimplemented!(),
            (*compare_last_digit_counting.borrow()).copy_from(),
            unimplemented!(),
        )
    });
    unimplemented!();
}
pub fn test_proxy_iterators_379() {
    let mut vec_: () = unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_380() -> () {
    ({ test_iterator_types_376() });
    ({ test_comparison_categories_377() });
    ({ test_comparator_invocation_count_378() });
    ({ test_proxy_iterators_379() });
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
    ({ test_380() });
    const _: () = assert!(true, "test()");;
    return 0_();
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
}
impl bidirectional_iterator_const_int_ptr_Impl for Ptr<bidirectional_iterator_const_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
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
}
impl contiguous_iterator_const_int_ptr_Impl for Ptr<contiguous_iterator_const_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait cpp17_input_iterator_const_int_ptr__const_int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>;
    fn operator_post_inc_6(
        &self,
        mut _a0: (),
    ) -> cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
        unimplemented!()
    }
}
impl cpp17_input_iterator_const_int_ptr__const_int_ptr_Impl
    for Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_>
{
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_const_int_ptr__const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
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
pub trait forward_iterator_const_int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_int_ptr_>;
    fn operator_post_inc_7(&self, mut _a0: ()) -> forward_iterator_const_int_ptr_ {
        unimplemented!()
    }
}
impl forward_iterator_const_int_ptr_Impl for Ptr<forward_iterator_const_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_int_ptr_> {
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
}
impl random_access_iterator_const_int_ptr_Impl for Ptr<random_access_iterator_const_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_int_ptr_> {
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
    let _ = compare_last_digit_strong_14.with(|_| ());
    let _ = compare_last_digit_weak_15.with(|_| ());
    let _ = compare_last_digit_partial_16.with(|_| ());
    let _ = compare_int_result_17.with(|_| ());
}
