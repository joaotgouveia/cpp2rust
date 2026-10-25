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
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub fn operator_add_0(
    x: random_access_iterator_const_int_ptr_,
    mut n: (),
) -> random_access_iterator_const_int_ptr_ {
    let x: Value<random_access_iterator_const_int_ptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_const_int_ptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return random_access_iterator_const_int_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_1(
    x: random_access_iterator_const_int_ptr_,
    mut n: (),
) -> random_access_iterator_const_int_ptr_ {
    let x: Value<random_access_iterator_const_int_ptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_const_int_ptr_Impl::operator_sub_assign(&x.as_pointer(), n) });
    return random_access_iterator_const_int_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_2(
    mut x: random_access_iterator_const_int_ptr_,
    mut y: random_access_iterator_const_int_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn operator_eq_3(
    x: Ptr<random_access_iterator_const_int_ptr_>,
    y: Ptr<random_access_iterator_const_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_4(
    x: Ptr<random_access_iterator_const_int_ptr_>,
    y: Ptr<random_access_iterator_const_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_le_5(
    x: Ptr<random_access_iterator_const_int_ptr_>,
    y: Ptr<random_access_iterator_const_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } <= { y.with(|__s| __s.it_.clone()) });
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
            operator_eq_3(
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
pub fn operator_add_6(
    x: random_access_iterator_const_char_ptr_,
    mut n: (),
) -> random_access_iterator_const_char_ptr_ {
    let x: Value<random_access_iterator_const_char_ptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_const_char_ptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return random_access_iterator_const_char_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_7(
    x: random_access_iterator_const_char_ptr_,
    mut n: (),
) -> random_access_iterator_const_char_ptr_ {
    let x: Value<random_access_iterator_const_char_ptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_const_char_ptr_Impl::operator_sub_assign(&x.as_pointer(), n) });
    return random_access_iterator_const_char_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_8(
    mut x: random_access_iterator_const_char_ptr_,
    mut y: random_access_iterator_const_char_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn operator_eq_9(
    x: Ptr<random_access_iterator_const_char_ptr_>,
    y: Ptr<random_access_iterator_const_char_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_10(
    x: Ptr<random_access_iterator_const_char_ptr_>,
    y: Ptr<random_access_iterator_const_char_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_le_11(
    x: Ptr<random_access_iterator_const_char_ptr_>,
    y: Ptr<random_access_iterator_const_char_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } <= { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(16)]
pub struct random_access_iterator_const_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_const_char_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<random_access_iterator_const_char_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for random_access_iterator_const_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_9(
                Rc::new(RefCell::new(random_access_iterator_const_char_ptr_ {
                    it_: self.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: self.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
                Rc::new(RefCell::new(random_access_iterator_const_char_ptr_ {
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
impl std::cmp::Eq for random_access_iterator_const_char_ptr_ {}
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
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
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
pub struct count_equal {}
thread_local!(
    pub static count_19: Value<()> = Rc::new(RefCell::new(0_()));
);
pub fn do_search_20(
    mut b1: random_access_iterator_const_int_ptr_,
    mut e1: random_access_iterator_const_int_ptr_,
    mut b2: random_access_iterator_const_int_ptr_,
    mut e2: random_access_iterator_const_int_ptr_,
    result: random_access_iterator_const_int_ptr_,
    mut max_count: (),
) {
    let result: Value<random_access_iterator_const_int_ptr_> = Rc::new(RefCell::new(result));
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    count_19.with(|rc| *rc.borrow_mut() = 0_());
    unimplemented!();
    unimplemented!();
}
pub fn do_search_21(
    mut b1: random_access_iterator_const_char_ptr_,
    mut e1: random_access_iterator_const_char_ptr_,
    mut b2: random_access_iterator_const_char_ptr_,
    mut e2: random_access_iterator_const_char_ptr_,
    result: random_access_iterator_const_char_ptr_,
    mut max_count: (),
) {
    let result: Value<random_access_iterator_const_char_ptr_> = Rc::new(RefCell::new(result));
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    count_19.with(|rc| *rc.borrow_mut() = 0_());
    unimplemented!();
    unimplemented!();
}
pub fn test_22() {
    let ia: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 1_(), 2_(), 3_(), 4_(), 5_()])));
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    ({
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        do_search_20(_b1, _e1, _b2, _e2, _result, 0_())
    });
    ({
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _max_count: () = sa;
        do_search_20(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    ({
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        let _max_count: () = sa;
        do_search_20(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    ({
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        do_search_20(_b1, _e1, _b2, _e2, _result, 0_())
    });
    ({
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((3_()) as isize)
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        let _max_count: () = sa;
        do_search_20(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    ({
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((3_()) as isize)
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        let _max_count: () = sa;
        do_search_20(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    ({
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((3_()) as isize)
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        do_search_20(_b1, _e1, _b2, _e2, _result, 0_())
    });
    ({
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>)
                    .offset((sa) as isize)
                    .offset(-((1_()) as isize))
            });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>)
                    .offset((sa) as isize)
                    .offset(-((1_()) as isize))
            });
        let _max_count: () = sa;
        do_search_20(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    ({
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>)
                    .offset((sa) as isize)
                    .offset(-((3_()) as isize))
            });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>)
                    .offset((sa) as isize)
                    .offset(-((3_()) as isize))
            });
        let _max_count: () = (3_()).wrapping_mul(sa);
        do_search_20(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    ({
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _max_count: () = (sa).wrapping_mul(sa);
        do_search_20(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    ({
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>)
                    .offset((sa) as isize)
                    .offset(-((1_()) as isize))
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>)
                    .offset((sa) as isize)
                    .offset(-((1_()) as isize))
            });
        let _max_count: () = (((sa).wrapping_sub(1_())) as ()).wrapping_mul(sa);
        do_search_20(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    ({
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        let _max_count: () = sa;
        do_search_20(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    let ib: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        2_(),
        0_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ])));
    let mut sb: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let ic: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_()])));
    ({
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_pointer() as Ptr<()>).offset((sb) as isize)
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ic.as_pointer() as Ptr<()>) });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ic.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        let _max_count: () = sb;
        do_search_20(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    let id: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_()])));
    ({
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_pointer() as Ptr<()>).offset((sb) as isize)
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (id.as_pointer() as Ptr<()>) });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (id.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        let _max_count: () = (sb).wrapping_mul(2_());
        do_search_20(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    let ie: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    ({
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_pointer() as Ptr<()>).offset((sb) as isize)
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ie.as_pointer() as Ptr<()>) });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ie.as_pointer() as Ptr<()>).offset((3_()) as isize)
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_pointer() as Ptr<()>).offset((4_()) as isize)
            });
        let _max_count: () = (sb).wrapping_mul(3_());
        do_search_20(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    let ig: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_()])));
    ({
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_pointer() as Ptr<()>).offset((sb) as isize)
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ig.as_pointer() as Ptr<()>) });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ig.as_pointer() as Ptr<()>).offset((4_()) as isize)
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_pointer() as Ptr<()>).offset((8_()) as isize)
            });
        let _max_count: () = (sb).wrapping_mul(4_());
        do_search_20(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    let ih: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ])));
    let mut sh: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let ii: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 1_(), 2_()])));
    ({
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ih.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ih.as_pointer() as Ptr<()>).offset((sh) as isize)
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ii.as_pointer() as Ptr<()>) });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ii.as_pointer() as Ptr<()>).offset((3_()) as isize)
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ih.as_pointer() as Ptr<()>).offset((3_()) as isize)
            });
        let _max_count: () = (sh).wrapping_mul(3_());
        do_search_20(_b1, _e1, _b2, _e2, _result, _max_count)
    });
}
pub fn test2_23() {
    let ia: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 1_(), 2_(), 3_(), 4_(), 5_()])));
    let mut sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    ({
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        do_search_21(_b1, _e1, _b2, _e2, _result, 0_())
    });
    ({
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _max_count: () = sa;
        do_search_21(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    ({
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        let _max_count: () = sa;
        do_search_21(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    ({
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        do_search_21(_b1, _e1, _b2, _e2, _result, 0_())
    });
    ({
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((3_()) as isize)
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        let _max_count: () = sa;
        do_search_21(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    ({
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((3_()) as isize)
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        let _max_count: () = sa;
        do_search_21(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    ({
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((3_()) as isize)
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        do_search_21(_b1, _e1, _b2, _e2, _result, 0_())
    });
    ({
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>)
                    .offset((sa) as isize)
                    .offset(-((1_()) as isize))
            });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>)
                    .offset((sa) as isize)
                    .offset(-((1_()) as isize))
            });
        let _max_count: () = sa;
        do_search_21(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    ({
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>)
                    .offset((sa) as isize)
                    .offset(-((3_()) as isize))
            });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>)
                    .offset((sa) as isize)
                    .offset(-((3_()) as isize))
            });
        let _max_count: () = (3_()).wrapping_mul(sa);
        do_search_21(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    ({
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _max_count: () = (sa).wrapping_mul(sa);
        do_search_21(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    ({
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>)
                    .offset((sa) as isize)
                    .offset(-((1_()) as isize))
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>)
                    .offset((sa) as isize)
                    .offset(-((1_()) as isize))
            });
        let _max_count: () = (((sa).wrapping_sub(1_())) as ()).wrapping_mul(sa);
        do_search_21(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    ({
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_pointer() as Ptr<()>) });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((sa) as isize)
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        let _max_count: () = sa;
        do_search_21(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    let ib: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        2_(),
        0_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ])));
    let mut sb: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let ic: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_()])));
    ({
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ib.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_pointer() as Ptr<()>).offset((sb) as isize)
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ic.as_pointer() as Ptr<()>) });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ic.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        let _max_count: () = sb;
        do_search_21(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    let id: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_()])));
    ({
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ib.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_pointer() as Ptr<()>).offset((sb) as isize)
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (id.as_pointer() as Ptr<()>) });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (id.as_pointer() as Ptr<()>).offset((2_()) as isize)
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_pointer() as Ptr<()>).offset((1_()) as isize)
            });
        let _max_count: () = (sb).wrapping_mul(2_());
        do_search_21(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    let ie: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    ({
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ib.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_pointer() as Ptr<()>).offset((sb) as isize)
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ie.as_pointer() as Ptr<()>) });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ie.as_pointer() as Ptr<()>).offset((3_()) as isize)
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_pointer() as Ptr<()>).offset((4_()) as isize)
            });
        let _max_count: () = (sb).wrapping_mul(3_());
        do_search_21(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    let ig: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_()])));
    ({
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ib.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_pointer() as Ptr<()>).offset((sb) as isize)
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ig.as_pointer() as Ptr<()>) });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ig.as_pointer() as Ptr<()>).offset((4_()) as isize)
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_pointer() as Ptr<()>).offset((8_()) as isize)
            });
        let _max_count: () = (sb).wrapping_mul(4_());
        do_search_21(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    let ih: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ])));
    let mut sh: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let ii: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 1_(), 2_()])));
    ({
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ih.as_pointer() as Ptr<()>) });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ih.as_pointer() as Ptr<()>).offset((sh) as isize)
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ii.as_pointer() as Ptr<()>) });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ii.as_pointer() as Ptr<()>).offset((3_()) as isize)
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ih.as_pointer() as Ptr<()>).offset((3_()) as isize)
            });
        let _max_count: () = (sh).wrapping_mul(3_());
        do_search_21(_b1, _e1, _b2, _e2, _result, _max_count)
    });
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
    ({ test_22() });
    ({ test2_23() });
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
pub trait count_equalImpl {
    fn operator_call_1_int(&self, x: Ptr<()>, y: Ptr<()>) -> ();
    fn operator_call_1_char(&self, x: Ptr<()>, y: Ptr<()>) -> ();
}
impl count_equalImpl for Ptr<count_equal> {
    fn operator_call_1_int(&self, x: Ptr<()>, y: Ptr<()>) -> () {
        (*count_19.with(Value::clone).borrow_mut()).prefix_inc();
        return ({ (x.read()) } == { (y.read()) });
    }
    fn operator_call_1_char(&self, x: Ptr<()>, y: Ptr<()>) -> () {
        (*count_19.with(Value::clone).borrow_mut()).prefix_inc();
        return ({ (x.read()) } == { (y.read()) });
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
pub trait random_access_iterator_const_char_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_index(&self, n: ()) -> ();
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_char_ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_const_char_ptr_>;
    fn operator_post_inc_9(&self, mut _a0: ()) -> random_access_iterator_const_char_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> random_access_iterator_const_char_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: ()) -> Ptr<random_access_iterator_const_char_ptr_>;
    fn operator_sub_assign(&self, n: ()) -> Ptr<random_access_iterator_const_char_ptr_>;
}
impl random_access_iterator_const_char_ptr_Impl for Ptr<random_access_iterator_const_char_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_index(&self, mut n: ()) -> () {
        return (*self).with(|__s| __s.it_.clone()).offset((n) as isize);
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_char_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_const_char_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<random_access_iterator_const_char_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<random_access_iterator_const_char_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
}
pub trait random_access_iterator_const_int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_index(&self, n: ()) -> ();
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_int_ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_const_int_ptr_>;
    fn operator_post_inc_9(&self, mut _a0: ()) -> random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: ()) -> Ptr<random_access_iterator_const_int_ptr_>;
    fn operator_sub_assign(&self, n: ()) -> Ptr<random_access_iterator_const_int_ptr_>;
}
impl random_access_iterator_const_int_ptr_Impl for Ptr<random_access_iterator_const_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_index(&self, mut n: ()) -> () {
        return (*self).with(|__s| __s.it_.clone()).offset((n) as isize);
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<random_access_iterator_const_int_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<random_access_iterator_const_int_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() - n)
        };
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
    let _ = count_19.with(|_| ());
}
