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
    x: random_access_iterator_int_ptr_,
    mut n: (),
) -> random_access_iterator_int_ptr_ {
    let x: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_int_ptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return random_access_iterator_int_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_1(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn operator_eq_2(
    x: Ptr<random_access_iterator_int_ptr_>,
    y: Ptr<random_access_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
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
impl std::cmp::PartialEq for random_access_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_2(
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
pub fn operator_eq_3(
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
            operator_eq_3(
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
pub fn operator_add_4(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_5(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_4(_i, n)
    });
}
pub fn iter_swap_6(a: adl_Iterator, b: adl_Iterator) {
    let a: Value<adl_Iterator> = Rc::new(RefCell::new(a));
    let b: Value<adl_Iterator> = Rc::new(RefCell::new(b));
    ({
        let ___x: Ptr<Ptr<()>> = field_ptr!(a.as_pointer(), ptr_);
        let ___y: Ptr<Ptr<()>> = field_ptr!(b.as_pointer(), ptr_);
        swap_7(___x, ___y)
    });
    if !({ (*a.borrow()).iter_swaps_.clone() }).is_null() {
        { (*a.borrow()).iter_swaps_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_8(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) }).reinterpret_cast::<()>();
}
pub fn operator_eq_9(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_10(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
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
            operator_cmp_10(
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
            operator_eq_9(
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
pub fn test_constexpr_11() -> () {
    let ia: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 0_(), 0_(), 0_()])));
    let ib: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 1_(), 1_(), 0_()])));
    return (unimplemented!() == (ia.as_pointer() as Ptr<()>).offset((1_()) as isize))
        && (unimplemented!() == (ib.as_pointer() as Ptr<()>).offset((3_()) as isize));
}
pub fn test_12() {
    let i1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 0_()])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let i2: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 1_()])));
    let i3: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 0_()])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let i4: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 0_(), 0_()])));
    let i5: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 0_(), 1_()])));
    let i6: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 1_(), 0_()])));
    let i7: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 1_(), 1_()])));
    let i8: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 0_(), 0_()])));
    let i9: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 0_(), 1_()])));
    let i10: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 1_(), 0_()])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let i11: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 0_(), 0_(), 0_()])));
    let i12: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 0_(), 0_(), 1_()])));
    let i13: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 0_(), 1_(), 0_()])));
    let i14: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 0_(), 1_(), 1_()])));
    let i15: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 1_(), 0_(), 0_()])));
    let i16: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 1_(), 0_(), 1_()])));
    let i17: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 1_(), 1_(), 0_()])));
    let i18: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 1_(), 1_(), 1_()])));
    let i19: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 0_(), 0_(), 0_()])));
    let i20: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 0_(), 0_(), 1_()])));
    let i21: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 0_(), 1_(), 0_()])));
    let i22: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 0_(), 1_(), 1_()])));
    let i23: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 1_(), 0_(), 0_()])));
    let i24: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 1_(), 0_(), 1_()])));
    let i25: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 1_(), 1_(), 0_()])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let i26: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 0_(), 0_(), 0_(), 0_()])));
    let i27: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 0_(), 0_(), 0_(), 1_()])));
    let i28: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 0_(), 0_(), 1_(), 0_()])));
    let i29: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 0_(), 0_(), 1_(), 1_()])));
    let i30: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 0_(), 1_(), 0_(), 0_()])));
    let i31: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 0_(), 1_(), 0_(), 1_()])));
    let i32: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 0_(), 1_(), 1_(), 0_()])));
    let i33: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 0_(), 1_(), 1_(), 1_()])));
    let i34: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 1_(), 0_(), 0_(), 0_()])));
    let i35: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 1_(), 0_(), 0_(), 1_()])));
    let i36: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 1_(), 0_(), 1_(), 0_()])));
    let i37: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 1_(), 0_(), 1_(), 1_()])));
    let i38: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 1_(), 1_(), 0_(), 0_()])));
    let i39: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 1_(), 1_(), 0_(), 1_()])));
    let i40: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 1_(), 1_(), 1_(), 0_()])));
    let i41: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 1_(), 1_(), 1_(), 1_()])));
    let i42: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 0_(), 0_(), 0_(), 0_()])));
    let i43: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 0_(), 0_(), 0_(), 1_()])));
    let i44: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 0_(), 0_(), 1_(), 0_()])));
    let i45: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 0_(), 0_(), 1_(), 1_()])));
    let i46: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 0_(), 1_(), 0_(), 0_()])));
    let i47: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 0_(), 1_(), 0_(), 1_()])));
    let i48: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 0_(), 1_(), 1_(), 0_()])));
    let i49: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 0_(), 1_(), 1_(), 1_()])));
    let i50: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 1_(), 0_(), 0_(), 0_()])));
    let i51: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 1_(), 0_(), 0_(), 1_()])));
    let i52: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 1_(), 0_(), 1_(), 0_()])));
    let i53: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 1_(), 0_(), 1_(), 1_()])));
    let i54: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 1_(), 1_(), 0_(), 0_()])));
    let i55: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 1_(), 1_(), 0_(), 1_()])));
    let i56: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 1_(), 1_(), 1_(), 0_()])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let i57: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 0_(), 0_(), 0_(), 0_(), 0_()])));
    let i58: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 0_(), 0_(), 0_(), 0_(), 1_()])));
    let i59: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 0_(), 0_(), 0_(), 1_(), 0_()])));
    let i60: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 0_(), 0_(), 0_(), 1_(), 1_()])));
    let i61: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 0_(), 0_(), 1_(), 0_(), 0_()])));
    let i62: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 0_(), 0_(), 1_(), 0_(), 1_()])));
    let i63: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 0_(), 0_(), 1_(), 1_(), 0_()])));
    let i64: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 0_(), 0_(), 1_(), 1_(), 1_()])));
    let i65: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 0_(), 1_(), 0_(), 0_(), 0_()])));
    let i66: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 0_(), 1_(), 0_(), 0_(), 1_()])));
    let i67: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 0_(), 1_(), 0_(), 1_(), 0_()])));
    let i68: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 0_(), 1_(), 0_(), 1_(), 1_()])));
    let i69: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 0_(), 1_(), 1_(), 0_(), 0_()])));
    let i70: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 0_(), 1_(), 1_(), 0_(), 1_()])));
    let i71: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 0_(), 1_(), 1_(), 1_(), 0_()])));
    let i72: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 0_(), 1_(), 1_(), 1_(), 1_()])));
    let i73: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 1_(), 0_(), 0_(), 0_(), 0_()])));
    let i74: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 1_(), 0_(), 0_(), 0_(), 1_()])));
    let i75: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 1_(), 0_(), 0_(), 1_(), 0_()])));
    let i76: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 1_(), 0_(), 0_(), 1_(), 1_()])));
    let i77: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 1_(), 0_(), 1_(), 0_(), 0_()])));
    let i78: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 1_(), 0_(), 1_(), 0_(), 1_()])));
    let i79: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 1_(), 0_(), 1_(), 1_(), 0_()])));
    let i80: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 1_(), 0_(), 1_(), 1_(), 1_()])));
    let i81: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 1_(), 1_(), 0_(), 0_(), 0_()])));
    let i82: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 1_(), 1_(), 0_(), 0_(), 1_()])));
    let i83: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 1_(), 1_(), 0_(), 1_(), 0_()])));
    let i84: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 1_(), 1_(), 0_(), 1_(), 1_()])));
    let i85: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 1_(), 1_(), 1_(), 0_(), 0_()])));
    let i86: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 1_(), 1_(), 1_(), 0_(), 1_()])));
    let i87: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 1_(), 1_(), 1_(), 1_(), 0_()])));
    let i88: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([0_(), 1_(), 1_(), 1_(), 1_(), 1_()])));
    let i89: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 0_(), 0_(), 0_(), 0_(), 0_()])));
    let i90: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 0_(), 0_(), 0_(), 0_(), 1_()])));
    let i91: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 0_(), 0_(), 0_(), 1_(), 0_()])));
    let i92: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 0_(), 0_(), 0_(), 1_(), 1_()])));
    let i93: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 0_(), 0_(), 1_(), 0_(), 0_()])));
    let i94: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 0_(), 0_(), 1_(), 0_(), 1_()])));
    let i95: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 0_(), 0_(), 1_(), 1_(), 0_()])));
    let i96: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 0_(), 0_(), 1_(), 1_(), 1_()])));
    let i97: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 0_(), 1_(), 0_(), 0_(), 0_()])));
    let i98: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 0_(), 1_(), 0_(), 0_(), 1_()])));
    let i99: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 0_(), 1_(), 0_(), 1_(), 0_()])));
    let i100: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 0_(), 1_(), 0_(), 1_(), 1_()])));
    let i101: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 0_(), 1_(), 1_(), 0_(), 0_()])));
    let i102: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 0_(), 1_(), 1_(), 0_(), 1_()])));
    let i103: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 0_(), 1_(), 1_(), 1_(), 0_()])));
    let i104: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 0_(), 1_(), 1_(), 1_(), 1_()])));
    let i105: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 1_(), 0_(), 0_(), 0_(), 0_()])));
    let i106: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 1_(), 0_(), 0_(), 0_(), 1_()])));
    let i107: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 1_(), 0_(), 0_(), 1_(), 0_()])));
    let i108: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 1_(), 0_(), 0_(), 1_(), 1_()])));
    let i109: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 1_(), 0_(), 1_(), 0_(), 0_()])));
    let i110: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 1_(), 0_(), 1_(), 0_(), 1_()])));
    let i111: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 1_(), 0_(), 1_(), 1_(), 0_()])));
    let i112: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 1_(), 0_(), 1_(), 1_(), 1_()])));
    let i113: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 1_(), 1_(), 0_(), 0_(), 0_()])));
    let i114: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 1_(), 1_(), 0_(), 0_(), 1_()])));
    let i115: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 1_(), 1_(), 0_(), 1_(), 0_()])));
    let i116: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 1_(), 1_(), 0_(), 1_(), 1_()])));
    let i117: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 1_(), 1_(), 1_(), 0_(), 0_()])));
    let i118: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 1_(), 1_(), 1_(), 0_(), 1_()])));
    let i119: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 1_(), 1_(), 1_(), 1_(), 0_()])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let i120: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
    ])));
    let i121: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
    ])));
    let i122: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
        0_(),
    ])));
    let i123: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
        1_(),
    ])));
    let i124: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
        0_(),
        0_(),
    ])));
    let i125: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
        0_(),
        1_(),
    ])));
    let i126: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
        1_(),
        0_(),
    ])));
    let i127: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
        1_(),
        1_(),
    ])));
    let i128: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        0_(),
        1_(),
        0_(),
        0_(),
        0_(),
    ])));
    let i129: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        0_(),
        1_(),
        0_(),
        0_(),
        1_(),
    ])));
    let i130: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        0_(),
        1_(),
        0_(),
        1_(),
        0_(),
    ])));
    let i131: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        0_(),
        1_(),
        0_(),
        1_(),
        1_(),
    ])));
    let i132: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        0_(),
        1_(),
        1_(),
        0_(),
        0_(),
    ])));
    let i133: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        0_(),
        1_(),
        1_(),
        0_(),
        1_(),
    ])));
    let i134: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        0_(),
        1_(),
        1_(),
        1_(),
        0_(),
    ])));
    let i135: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
    ])));
    let i136: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        1_(),
        0_(),
        0_(),
        0_(),
        0_(),
    ])));
    let i137: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        1_(),
        0_(),
        0_(),
        0_(),
        1_(),
    ])));
    let i138: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        1_(),
        0_(),
        0_(),
        1_(),
        0_(),
    ])));
    let i139: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        1_(),
        0_(),
        0_(),
        1_(),
        1_(),
    ])));
    let i140: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        1_(),
        0_(),
        1_(),
        0_(),
        0_(),
    ])));
    let i141: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        1_(),
        0_(),
        1_(),
        0_(),
        1_(),
    ])));
    let i142: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        1_(),
        0_(),
        1_(),
        1_(),
        0_(),
    ])));
    let i143: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        1_(),
        0_(),
        1_(),
        1_(),
        1_(),
    ])));
    let i144: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        1_(),
        1_(),
        0_(),
        0_(),
        0_(),
    ])));
    let i145: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        1_(),
        1_(),
        0_(),
        0_(),
        1_(),
    ])));
    let i146: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        1_(),
        1_(),
        0_(),
        1_(),
        0_(),
    ])));
    let i147: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        1_(),
        1_(),
        0_(),
        1_(),
        1_(),
    ])));
    let i148: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        1_(),
        1_(),
        1_(),
        0_(),
        0_(),
    ])));
    let i149: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        1_(),
        1_(),
        1_(),
        0_(),
        1_(),
    ])));
    let i150: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        0_(),
    ])));
    let i151: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        1_(),
    ])));
    let i152: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
    ])));
    let i153: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
    ])));
    let i154: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        0_(),
        0_(),
        0_(),
        1_(),
        0_(),
    ])));
    let i155: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        0_(),
        0_(),
        0_(),
        1_(),
        1_(),
    ])));
    let i156: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        0_(),
        0_(),
        1_(),
        0_(),
        0_(),
    ])));
    let i157: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        0_(),
        0_(),
        1_(),
        0_(),
        1_(),
    ])));
    let i158: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        0_(),
        0_(),
        1_(),
        1_(),
        0_(),
    ])));
    let i159: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        0_(),
        0_(),
        1_(),
        1_(),
        1_(),
    ])));
    let i160: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        0_(),
        1_(),
        0_(),
        0_(),
        0_(),
    ])));
    let i161: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        0_(),
        1_(),
        0_(),
        0_(),
        1_(),
    ])));
    let i162: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        0_(),
        1_(),
        0_(),
        1_(),
        0_(),
    ])));
    let i163: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        0_(),
        1_(),
        0_(),
        1_(),
        1_(),
    ])));
    let i164: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        0_(),
        1_(),
        1_(),
        0_(),
        0_(),
    ])));
    let i165: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        0_(),
        1_(),
        1_(),
        0_(),
        1_(),
    ])));
    let i166: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        0_(),
        1_(),
        1_(),
        1_(),
        0_(),
    ])));
    let i167: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
    ])));
    let i168: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        1_(),
        0_(),
        0_(),
        0_(),
        0_(),
    ])));
    let i169: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        1_(),
        0_(),
        0_(),
        0_(),
        1_(),
    ])));
    let i170: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        1_(),
        0_(),
        0_(),
        1_(),
        0_(),
    ])));
    let i171: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        1_(),
        0_(),
        0_(),
        1_(),
        1_(),
    ])));
    let i172: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        1_(),
        0_(),
        1_(),
        0_(),
        0_(),
    ])));
    let i173: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        1_(),
        0_(),
        1_(),
        0_(),
        1_(),
    ])));
    let i174: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        1_(),
        0_(),
        1_(),
        1_(),
        0_(),
    ])));
    let i175: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        1_(),
        0_(),
        1_(),
        1_(),
        1_(),
    ])));
    let i176: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        1_(),
        1_(),
        0_(),
        0_(),
        0_(),
    ])));
    let i177: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        1_(),
        1_(),
        0_(),
        0_(),
        1_(),
    ])));
    let i178: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        1_(),
        1_(),
        0_(),
        1_(),
        0_(),
    ])));
    let i179: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        1_(),
        1_(),
        0_(),
        1_(),
        1_(),
    ])));
    let i180: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        0_(),
        0_(),
    ])));
    let i181: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        0_(),
        1_(),
    ])));
    let i182: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        1_(),
        0_(),
    ])));
    let i183: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        1_(),
        1_(),
    ])));
    let i184: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
    ])));
    let i185: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
    ])));
    let i186: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
        0_(),
    ])));
    let i187: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
        1_(),
    ])));
    let i188: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        0_(),
        0_(),
        1_(),
        0_(),
        0_(),
    ])));
    let i189: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        0_(),
        0_(),
        1_(),
        0_(),
        1_(),
    ])));
    let i190: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        0_(),
        0_(),
        1_(),
        1_(),
        0_(),
    ])));
    let i191: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        0_(),
        0_(),
        1_(),
        1_(),
        1_(),
    ])));
    let i192: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        0_(),
        1_(),
        0_(),
        0_(),
        0_(),
    ])));
    let i193: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        0_(),
        1_(),
        0_(),
        0_(),
        1_(),
    ])));
    let i194: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        0_(),
        1_(),
        0_(),
        1_(),
        0_(),
    ])));
    let i195: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        0_(),
        1_(),
        0_(),
        1_(),
        1_(),
    ])));
    let i196: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        0_(),
        1_(),
        1_(),
        0_(),
        0_(),
    ])));
    let i197: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        0_(),
        1_(),
        1_(),
        0_(),
        1_(),
    ])));
    let i198: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        0_(),
        1_(),
        1_(),
        1_(),
        0_(),
    ])));
    let i199: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
    ])));
    let i200: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        1_(),
        0_(),
        0_(),
        0_(),
        0_(),
    ])));
    let i201: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        1_(),
        0_(),
        0_(),
        0_(),
        1_(),
    ])));
    let i202: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        1_(),
        0_(),
        0_(),
        1_(),
        0_(),
    ])));
    let i203: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        1_(),
        0_(),
        0_(),
        1_(),
        1_(),
    ])));
    let i204: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        1_(),
        0_(),
        1_(),
        0_(),
        0_(),
    ])));
    let i205: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        1_(),
        0_(),
        1_(),
        0_(),
        1_(),
    ])));
    let i206: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        1_(),
        0_(),
        1_(),
        1_(),
        0_(),
    ])));
    let i207: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        1_(),
        0_(),
        1_(),
        1_(),
        1_(),
    ])));
    let i208: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        1_(),
        1_(),
        0_(),
        0_(),
        0_(),
    ])));
    let i209: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        1_(),
        1_(),
        0_(),
        0_(),
        1_(),
    ])));
    let i210: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        1_(),
        1_(),
        0_(),
        1_(),
        0_(),
    ])));
    let i211: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        1_(),
        1_(),
        0_(),
        1_(),
        1_(),
    ])));
    let i212: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        1_(),
        1_(),
        1_(),
        0_(),
        0_(),
    ])));
    let i213: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        1_(),
        1_(),
        1_(),
        0_(),
        1_(),
    ])));
    let i214: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        0_(),
    ])));
    let i215: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        1_(),
    ])));
    let i216: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
    ])));
    let i217: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
    ])));
    let i218: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        0_(),
        0_(),
        0_(),
        1_(),
        0_(),
    ])));
    let i219: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        0_(),
        0_(),
        0_(),
        1_(),
        1_(),
    ])));
    let i220: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        0_(),
        0_(),
        1_(),
        0_(),
        0_(),
    ])));
    let i221: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        0_(),
        0_(),
        1_(),
        0_(),
        1_(),
    ])));
    let i222: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        0_(),
        0_(),
        1_(),
        1_(),
        0_(),
    ])));
    let i223: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        0_(),
        0_(),
        1_(),
        1_(),
        1_(),
    ])));
    let i224: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        0_(),
        1_(),
        0_(),
        0_(),
        0_(),
    ])));
    let i225: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        0_(),
        1_(),
        0_(),
        0_(),
        1_(),
    ])));
    let i226: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        0_(),
        1_(),
        0_(),
        1_(),
        0_(),
    ])));
    let i227: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        0_(),
        1_(),
        0_(),
        1_(),
        1_(),
    ])));
    let i228: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        0_(),
        1_(),
        1_(),
        0_(),
        0_(),
    ])));
    let i229: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        0_(),
        1_(),
        1_(),
        0_(),
        1_(),
    ])));
    let i230: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        0_(),
        1_(),
        1_(),
        1_(),
        0_(),
    ])));
    let i231: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
    ])));
    let i232: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        1_(),
        0_(),
        0_(),
        0_(),
        0_(),
    ])));
    let i233: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        1_(),
        0_(),
        0_(),
        0_(),
        1_(),
    ])));
    let i234: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        1_(),
        0_(),
        0_(),
        1_(),
        0_(),
    ])));
    let i235: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        1_(),
        0_(),
        0_(),
        1_(),
        1_(),
    ])));
    let i236: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        1_(),
        0_(),
        1_(),
        0_(),
        0_(),
    ])));
    let i237: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        1_(),
        0_(),
        1_(),
        0_(),
        1_(),
    ])));
    let i238: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        1_(),
        0_(),
        1_(),
        1_(),
        0_(),
    ])));
    let i239: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        1_(),
        0_(),
        1_(),
        1_(),
        1_(),
    ])));
    let i240: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        1_(),
        1_(),
        0_(),
        0_(),
        0_(),
    ])));
    let i241: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        1_(),
        1_(),
        0_(),
        0_(),
        1_(),
    ])));
    let i242: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        1_(),
        1_(),
        0_(),
        1_(),
        0_(),
    ])));
    let i243: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        1_(),
        1_(),
        0_(),
        1_(),
        1_(),
    ])));
    let i244: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        1_(),
        1_(),
        1_(),
        0_(),
        0_(),
    ])));
    let i245: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        1_(),
        1_(),
        1_(),
        0_(),
        1_(),
    ])));
    let i246: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        1_(),
        1_(),
        1_(),
        1_(),
        1_(),
        0_(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    ({ test_12() });
    const _: () = assert!(true, "test_constexpr()");;
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
pub fn __cpp2rust_init_globals() {}
