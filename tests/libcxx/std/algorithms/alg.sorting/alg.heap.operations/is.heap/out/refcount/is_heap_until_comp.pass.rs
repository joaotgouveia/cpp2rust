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
    moved_from_: bool,
}
impl support_double_move_tracker {
    pub fn new() -> Self {
        Self { moved_from_: false }
    }
    pub fn move_from(other: Ptr<support_double_move_tracker>) -> Self {
        let __this: support_double_move_tracker = Self { moved_from_: false };
        assert!(!(other.with(|__s| __s.moved_from_)));
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
    it_: Ptr<i32>,
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
    it_: Ptr<i32>,
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
    mut n: isize,
) -> random_access_iterator_int_ptr_ {
    let x: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_int_ptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return random_access_iterator_int_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_1(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> isize {
    return (((x.it_).clone() - (y.it_).clone()) as i64 as isize);
}
pub fn operator_eq_2(
    x: Ptr<random_access_iterator_int_ptr_>,
    y: Ptr<random_access_iterator_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
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
impl CppEq for random_access_iterator_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_2(__lhs.clone(), __rhs.clone())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
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
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_input_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
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
    it_: Ptr<i32>,
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
    pub increments: usize,
    #[offset(8)]
    pub decrements: usize,
    #[offset(16)]
    pub zero_moves: usize,
    #[offset(24)]
    pub equal_cmps: usize,
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        {
            IteratorOpCounts {
                increments: 0_usize,
                decrements: 0_usize,
                zero_moves: 0_usize,
                equal_cmps: 0_usize,
            }
        }
    }
}
pub fn operator_eq_3(
    _a0: Ptr<forward_sized_iterator_int_ptr_>,
    _a1: Ptr<forward_sized_iterator_int_ptr_>,
) -> bool {
    return ({ _a0.with(|__s| __s.it_.clone()) } == { _a1.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct forward_sized_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    pub it_: Ptr<i32>,
}
impl forward_sized_iterator_int_ptr_ {}
impl CppEq for forward_sized_iterator_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_3(__lhs.clone(), __rhs.clone())
    }
}
impl Default for forward_sized_iterator_int_ptr_ {
    fn default() -> Self {
        {
            forward_sized_iterator_int_ptr_ {
                it_: Ptr::<i32>::null(),
            }
        }
    }
}
const _: () = assert!(true, "std::forward_iterator<forward_sized_iterator<>>");
const _: () = assert!(
    true,
    "std::sized_sentinel_for<forward_sized_iterator<>, forward_sized_iterator<>>"
);
pub fn operator_add_4(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_5(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_4(_i, n)
    });
}
pub fn iter_swap_6(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap!((a.ptr_).clone(), (b.ptr_).clone());
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_7(iter: adl_Iterator) -> Ptr<i32> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_8(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> bool {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_9(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> std::cmp::Ordering {
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
    ptr_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    iter_moves_: Ptr<i32>,
    #[offset(16)]
    #[byte_size(8)]
    iter_swaps_: Ptr<i32>,
}
impl adl_Iterator {
    fn new_1(mut p: Ptr<i32>, mut iter_moves: Ptr<i32>, mut iter_swaps: Ptr<i32>) -> Self {
        Self {
            ptr_: (p).clone(),
            iter_moves_: (iter_moves).clone(),
            iter_swaps_: (iter_swaps).clone(),
        }
    }
    pub fn TrackMoves(mut p: Ptr<i32>, iter_moves: Ptr<i32>) -> adl_Iterator {
        return adl_Iterator::new_1({ (p).clone() }, { (iter_moves).clone() }, {
            Ptr::<i32>::null()
        });
    }
    pub fn TrackSwaps_4(iter_swaps: Ptr<i32>) -> adl_Iterator {
        return adl_Iterator::new_1({ Ptr::<i32>::null() }, { Ptr::<i32>::null() }, {
            (iter_swaps).clone()
        });
    }
    pub fn TrackSwaps_5(mut p: Ptr<i32>, iter_swaps: Ptr<i32>) -> adl_Iterator {
        return adl_Iterator::new_1({ (p).clone() }, { Ptr::<i32>::null() }, {
            (iter_swaps).clone()
        });
    }
}
impl CppLess for adl_Iterator {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_9(__lhs.clone(), __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for adl_Iterator {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_8(__lhs.clone(), __rhs.clone())
    }
}
impl CppAdvance for adl_Iterator {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_14();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_16();
    }
}
impl Default for adl_Iterator {
    fn default() -> Self {
        {
            adl_Iterator {
                ptr_: Ptr::<i32>::null(),
                iter_moves_: Ptr::<i32>::null(),
                iter_swaps_: Ptr::<i32>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct rvalue_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<Ptr<i32>>,
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Ptr< i8  >  , new, value)]
#[byte_size(16)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub fn new(mut what: Ptr<i8>) -> Self {
        Self {}
    }
}
pub fn test_constexpr_10() -> bool {
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 0])));
    let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 0])));
    return (unimplemented!() == (ia.as_pointer() as Ptr<i32>).offset((1) as isize))
        && (unimplemented!() == (ib.as_pointer() as Ptr<i32>).offset((3) as isize));
}
pub fn test_11() {
    let i1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0])));
    assert!((unimplemented!() == (i1.as_pointer() as Ptr::<i32>)));
    assert!((unimplemented!() == (i1.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!(
        ({
            let _x: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    (i1.as_pointer() as Ptr<i32>)
                })));
            operator_eq_2(_x.as_pointer(), _y.as_pointer())
        })
    );
    assert!(
        ({
            let _x: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<random_access_iterator_int_ptr_> =
                Rc::new(RefCell::new(random_access_iterator_int_ptr_::new_1({
                    (i1.as_pointer() as Ptr<i32>).offset((1) as isize)
                })));
            operator_eq_2(_x.as_pointer(), _y.as_pointer())
        })
    );
    let i2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1])));
    let i3: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0])));
    assert!((unimplemented!() == (i1.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i2.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i3.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    let i4: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0])));
    let i5: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1])));
    let i6: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0])));
    let i7: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1])));
    let i8: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0])));
    let i9: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1])));
    let i10: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0])));
    assert!((unimplemented!() == (i4.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i5.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i6.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i7.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i8.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i9.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i10.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    let i11: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 0])));
    let i12: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 1])));
    let i13: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 0])));
    let i14: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 1])));
    let i15: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 0])));
    let i16: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 1])));
    let i17: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 0])));
    let i18: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 1])));
    let i19: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 0])));
    let i20: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 1])));
    let i21: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 0])));
    let i22: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 1])));
    let i23: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 0])));
    let i24: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 1])));
    let i25: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 0])));
    assert!((unimplemented!() == (i11.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i12.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i13.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i14.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i15.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i16.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i17.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i18.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i19.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i20.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i21.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i22.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i23.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i24.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i25.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    let i26: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 0, 0])));
    let i27: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 0, 1])));
    let i28: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 1, 0])));
    let i29: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 1, 1])));
    let i30: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 0, 0])));
    let i31: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 0, 1])));
    let i32: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 1, 0])));
    let i33: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 1, 1])));
    let i34: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 0, 0])));
    let i35: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 0, 1])));
    let i36: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 1, 0])));
    let i37: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 1, 1])));
    let i38: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 0, 0])));
    let i39: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 0, 1])));
    let i40: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 1, 0])));
    let i41: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 1, 1])));
    let i42: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 0, 0])));
    let i43: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 0, 1])));
    let i44: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 1, 0])));
    let i45: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 1, 1])));
    let i46: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 0, 0])));
    let i47: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 0, 1])));
    let i48: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 1, 0])));
    let i49: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 1, 1])));
    let i50: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 0, 0])));
    let i51: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 0, 1])));
    let i52: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 1, 0])));
    let i53: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 1, 1])));
    let i54: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 0, 0])));
    let i55: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 0, 1])));
    let i56: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 1, 0])));
    assert!((unimplemented!() == (i26.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i27.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i28.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i29.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i30.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i31.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i32.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i33.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i34.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i35.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i36.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i37.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i38.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i39.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i40.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i41.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i42.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i43.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i44.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i45.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i46.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i47.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i48.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i49.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i50.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i51.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i52.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i53.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i54.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i55.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i56.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    let i57: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 0, 0, 0])));
    let i58: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 0, 0, 1])));
    let i59: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 0, 1, 0])));
    let i60: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 0, 1, 1])));
    let i61: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 1, 0, 0])));
    let i62: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 1, 0, 1])));
    let i63: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 1, 1, 0])));
    let i64: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 1, 1, 1])));
    let i65: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 0, 0, 0])));
    let i66: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 0, 0, 1])));
    let i67: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 0, 1, 0])));
    let i68: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 0, 1, 1])));
    let i69: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 1, 0, 0])));
    let i70: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 1, 0, 1])));
    let i71: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 1, 1, 0])));
    let i72: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 1, 1, 1])));
    let i73: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 0, 0, 0])));
    let i74: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 0, 0, 1])));
    let i75: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 0, 1, 0])));
    let i76: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 0, 1, 1])));
    let i77: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 1, 0, 0])));
    let i78: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 1, 0, 1])));
    let i79: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 1, 1, 0])));
    let i80: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 1, 1, 1])));
    let i81: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 0, 0, 0])));
    let i82: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 0, 0, 1])));
    let i83: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 0, 1, 0])));
    let i84: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 0, 1, 1])));
    let i85: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 1, 0, 0])));
    let i86: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 1, 0, 1])));
    let i87: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 1, 1, 0])));
    let i88: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 1, 1, 1])));
    let i89: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 0, 0, 0])));
    let i90: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 0, 0, 1])));
    let i91: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 0, 1, 0])));
    let i92: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 0, 1, 1])));
    let i93: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 1, 0, 0])));
    let i94: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 1, 0, 1])));
    let i95: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 1, 1, 0])));
    let i96: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 1, 1, 1])));
    let i97: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 0, 0, 0])));
    let i98: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 0, 0, 1])));
    let i99: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 0, 1, 0])));
    let i100: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 0, 1, 1])));
    let i101: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 1, 0, 0])));
    let i102: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 1, 0, 1])));
    let i103: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 1, 1, 0])));
    let i104: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 1, 1, 1])));
    let i105: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 0, 0, 0])));
    let i106: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 0, 0, 1])));
    let i107: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 0, 1, 0])));
    let i108: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 0, 1, 1])));
    let i109: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 1, 0, 0])));
    let i110: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 1, 0, 1])));
    let i111: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 1, 1, 0])));
    let i112: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 1, 1, 1])));
    let i113: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 0, 0, 0])));
    let i114: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 0, 0, 1])));
    let i115: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 0, 1, 0])));
    let i116: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 0, 1, 1])));
    let i117: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 1, 0, 0])));
    let i118: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 1, 0, 1])));
    let i119: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 1, 1, 0])));
    assert!((unimplemented!() == (i57.as_pointer() as Ptr::<i32>).offset((6) as isize)));
    assert!((unimplemented!() == (i58.as_pointer() as Ptr::<i32>).offset((6) as isize)));
    assert!((unimplemented!() == (i59.as_pointer() as Ptr::<i32>).offset((6) as isize)));
    assert!((unimplemented!() == (i60.as_pointer() as Ptr::<i32>).offset((6) as isize)));
    assert!((unimplemented!() == (i61.as_pointer() as Ptr::<i32>).offset((6) as isize)));
    assert!((unimplemented!() == (i62.as_pointer() as Ptr::<i32>).offset((6) as isize)));
    assert!((unimplemented!() == (i63.as_pointer() as Ptr::<i32>).offset((6) as isize)));
    assert!((unimplemented!() == (i64.as_pointer() as Ptr::<i32>).offset((6) as isize)));
    assert!((unimplemented!() == (i65.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i66.as_pointer() as Ptr::<i32>).offset((6) as isize)));
    assert!((unimplemented!() == (i67.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i68.as_pointer() as Ptr::<i32>).offset((6) as isize)));
    assert!((unimplemented!() == (i69.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i70.as_pointer() as Ptr::<i32>).offset((6) as isize)));
    assert!((unimplemented!() == (i71.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i72.as_pointer() as Ptr::<i32>).offset((6) as isize)));
    assert!((unimplemented!() == (i73.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i74.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i75.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i76.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i77.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i78.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i79.as_pointer() as Ptr::<i32>).offset((6) as isize)));
    assert!((unimplemented!() == (i80.as_pointer() as Ptr::<i32>).offset((6) as isize)));
    assert!((unimplemented!() == (i81.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i82.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i83.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i84.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i85.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i86.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i87.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i88.as_pointer() as Ptr::<i32>).offset((6) as isize)));
    assert!((unimplemented!() == (i89.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i90.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i91.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i92.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i93.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i94.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i95.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i96.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i97.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i98.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i99.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i100.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i101.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i102.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i103.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i104.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i105.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i106.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i107.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i108.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i109.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i110.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i111.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i112.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i113.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i114.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i115.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i116.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i117.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i118.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i119.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    let i120: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 0, 0, 0, 0])));
    let i121: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 0, 0, 0, 1])));
    let i122: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 0, 0, 1, 0])));
    let i123: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 0, 0, 1, 1])));
    let i124: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 0, 1, 0, 0])));
    let i125: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 0, 1, 0, 1])));
    let i126: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 0, 1, 1, 0])));
    let i127: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 0, 1, 1, 1])));
    let i128: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 1, 0, 0, 0])));
    let i129: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 1, 0, 0, 1])));
    let i130: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 1, 0, 1, 0])));
    let i131: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 1, 0, 1, 1])));
    let i132: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 1, 1, 0, 0])));
    let i133: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 1, 1, 0, 1])));
    let i134: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 1, 1, 1, 0])));
    let i135: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 1, 1, 1, 1])));
    let i136: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 0, 0, 0, 0])));
    let i137: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 0, 0, 0, 1])));
    let i138: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 0, 0, 1, 0])));
    let i139: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 0, 0, 1, 1])));
    let i140: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 0, 1, 0, 0])));
    let i141: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 0, 1, 0, 1])));
    let i142: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 0, 1, 1, 0])));
    let i143: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 0, 1, 1, 1])));
    let i144: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 1, 0, 0, 0])));
    let i145: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 1, 0, 0, 1])));
    let i146: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 1, 0, 1, 0])));
    let i147: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 1, 0, 1, 1])));
    let i148: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 1, 1, 0, 0])));
    let i149: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 1, 1, 0, 1])));
    let i150: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 1, 1, 1, 0])));
    let i151: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 1, 1, 1, 1, 1])));
    let i152: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 0, 0, 0, 0])));
    let i153: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 0, 0, 0, 1])));
    let i154: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 0, 0, 1, 0])));
    let i155: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 0, 0, 1, 1])));
    let i156: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 0, 1, 0, 0])));
    let i157: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 0, 1, 0, 1])));
    let i158: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 0, 1, 1, 0])));
    let i159: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 0, 1, 1, 1])));
    let i160: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 1, 0, 0, 0])));
    let i161: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 1, 0, 0, 1])));
    let i162: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 1, 0, 1, 0])));
    let i163: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 1, 0, 1, 1])));
    let i164: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 1, 1, 0, 0])));
    let i165: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 1, 1, 0, 1])));
    let i166: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 1, 1, 1, 0])));
    let i167: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 0, 1, 1, 1, 1])));
    let i168: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 0, 0, 0, 0])));
    let i169: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 0, 0, 0, 1])));
    let i170: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 0, 0, 1, 0])));
    let i171: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 0, 0, 1, 1])));
    let i172: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 0, 1, 0, 0])));
    let i173: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 0, 1, 0, 1])));
    let i174: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 0, 1, 1, 0])));
    let i175: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 0, 1, 1, 1])));
    let i176: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 1, 0, 0, 0])));
    let i177: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 1, 0, 0, 1])));
    let i178: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 1, 0, 1, 0])));
    let i179: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 1, 0, 1, 1])));
    let i180: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 1, 1, 0, 0])));
    let i181: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 1, 1, 0, 1])));
    let i182: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 1, 1, 1, 0])));
    let i183: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 1, 1, 1, 1, 1])));
    let i184: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 0, 0, 0, 0])));
    let i185: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 0, 0, 0, 1])));
    let i186: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 0, 0, 1, 0])));
    let i187: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 0, 0, 1, 1])));
    let i188: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 0, 1, 0, 0])));
    let i189: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 0, 1, 0, 1])));
    let i190: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 0, 1, 1, 0])));
    let i191: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 0, 1, 1, 1])));
    let i192: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 1, 0, 0, 0])));
    let i193: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 1, 0, 0, 1])));
    let i194: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 1, 0, 1, 0])));
    let i195: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 1, 0, 1, 1])));
    let i196: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 1, 1, 0, 0])));
    let i197: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 1, 1, 0, 1])));
    let i198: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 1, 1, 1, 0])));
    let i199: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 0, 1, 1, 1, 1])));
    let i200: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 0, 0, 0, 0])));
    let i201: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 0, 0, 0, 1])));
    let i202: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 0, 0, 1, 0])));
    let i203: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 0, 0, 1, 1])));
    let i204: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 0, 1, 0, 0])));
    let i205: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 0, 1, 0, 1])));
    let i206: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 0, 1, 1, 0])));
    let i207: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 0, 1, 1, 1])));
    let i208: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 1, 0, 0, 0])));
    let i209: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 1, 0, 0, 1])));
    let i210: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 1, 0, 1, 0])));
    let i211: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 1, 0, 1, 1])));
    let i212: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 1, 1, 0, 0])));
    let i213: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 1, 1, 0, 1])));
    let i214: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 1, 1, 1, 0])));
    let i215: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 0, 1, 1, 1, 1, 1])));
    let i216: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 0, 0, 0, 0])));
    let i217: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 0, 0, 0, 1])));
    let i218: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 0, 0, 1, 0])));
    let i219: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 0, 0, 1, 1])));
    let i220: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 0, 1, 0, 0])));
    let i221: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 0, 1, 0, 1])));
    let i222: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 0, 1, 1, 0])));
    let i223: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 0, 1, 1, 1])));
    let i224: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 1, 0, 0, 0])));
    let i225: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 1, 0, 0, 1])));
    let i226: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 1, 0, 1, 0])));
    let i227: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 1, 0, 1, 1])));
    let i228: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 1, 1, 0, 0])));
    let i229: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 1, 1, 0, 1])));
    let i230: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 1, 1, 1, 0])));
    let i231: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 0, 1, 1, 1, 1])));
    let i232: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 0, 0, 0, 0])));
    let i233: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 0, 0, 0, 1])));
    let i234: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 0, 0, 1, 0])));
    let i235: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 0, 0, 1, 1])));
    let i236: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 0, 1, 0, 0])));
    let i237: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 0, 1, 0, 1])));
    let i238: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 0, 1, 1, 0])));
    let i239: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 0, 1, 1, 1])));
    let i240: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 1, 0, 0, 0])));
    let i241: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 1, 0, 0, 1])));
    let i242: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 1, 0, 1, 0])));
    let i243: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 1, 0, 1, 1])));
    let i244: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 1, 1, 0, 0])));
    let i245: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 1, 1, 0, 1])));
    let i246: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 1, 1, 1, 0])));
    assert!((unimplemented!() == (i120.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i121.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i122.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i123.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i124.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i125.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i126.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i127.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i128.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i129.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i130.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i131.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i132.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i133.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i134.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i135.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i136.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i137.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i138.as_pointer() as Ptr::<i32>).offset((6) as isize)));
    assert!((unimplemented!() == (i139.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i140.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i141.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i142.as_pointer() as Ptr::<i32>).offset((6) as isize)));
    assert!((unimplemented!() == (i143.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i144.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i145.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i146.as_pointer() as Ptr::<i32>).offset((6) as isize)));
    assert!((unimplemented!() == (i147.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i148.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i149.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i150.as_pointer() as Ptr::<i32>).offset((6) as isize)));
    assert!((unimplemented!() == (i151.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i152.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i153.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i154.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i155.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i156.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i157.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i158.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i159.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i160.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i161.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i162.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i163.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i164.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i165.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i166.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i167.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i168.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i169.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i170.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i171.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i172.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i173.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i174.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i175.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i176.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i177.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i178.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i179.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i180.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i181.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i182.as_pointer() as Ptr::<i32>).offset((6) as isize)));
    assert!((unimplemented!() == (i183.as_pointer() as Ptr::<i32>).offset((7) as isize)));
    assert!((unimplemented!() == (i184.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i185.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i186.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i187.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i188.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i189.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i190.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i191.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i192.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i193.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i194.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i195.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i196.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i197.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i198.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i199.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i200.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i201.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i202.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i203.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i204.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i205.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i206.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i207.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i208.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i209.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i210.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i211.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i212.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i213.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i214.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i215.as_pointer() as Ptr::<i32>).offset((1) as isize)));
    assert!((unimplemented!() == (i216.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i217.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i218.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i219.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i220.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i221.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i222.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i223.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i224.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i225.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i226.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i227.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i228.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i229.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i230.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i231.as_pointer() as Ptr::<i32>).offset((2) as isize)));
    assert!((unimplemented!() == (i232.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i233.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i234.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i235.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i236.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i237.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i238.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i239.as_pointer() as Ptr::<i32>).offset((3) as isize)));
    assert!((unimplemented!() == (i240.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i241.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i242.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i243.as_pointer() as Ptr::<i32>).offset((4) as isize)));
    assert!((unimplemented!() == (i244.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i245.as_pointer() as Ptr::<i32>).offset((5) as isize)));
    assert!((unimplemented!() == (i246.as_pointer() as Ptr::<i32>).offset((6) as isize)));
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_11() });
    const _: () = assert!(true, "test_constexpr()");;
    return 0;
}
pub trait adl_IteratorImpl {
    fn iter_moves(&self) -> i32;
    fn iter_swaps(&self) -> i32;
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_index(&self, n: isize) -> Ptr<i32>;
    fn operator_sub_10(&self, n: isize) -> adl_Iterator;
    fn operator_sub_11(&self, rhs: adl_Iterator) -> isize;
    fn operator_add_assign(&self, n: isize) -> Ptr<adl_Iterator>;
    fn operator_sub_assign(&self, n: isize) -> Ptr<adl_Iterator>;
    fn operator_inc_14(&self) -> Ptr<adl_Iterator>;
    fn operator_post_inc_15(&self, _a0: i32) -> adl_Iterator;
    fn operator_dec_16(&self) -> Ptr<adl_Iterator>;
    fn operator_post_dec_17(&self, _a0: i32) -> adl_Iterator;
}
impl adl_IteratorImpl for Ptr<adl_Iterator> {
    fn iter_moves(&self) -> i32 {
        assert!(!((*self).with(|__s| __s.iter_moves_.clone())).is_null());
        return ((*self).with(|__s| __s.iter_moves_.clone()).read());
    }
    fn iter_swaps(&self) -> i32 {
        assert!(!((*self).with(|__s| __s.iter_swaps_.clone())).is_null());
        return ((*self).with(|__s| __s.iter_swaps_.clone()).read());
    }
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
    fn operator_sub_10(&self, mut n: isize) -> adl_Iterator {
        return adl_Iterator::new_1(
            { (*self).with(|__s| __s.ptr_.clone()).offset(-((n) as isize)) },
            { (*self).with(|__s| __s.iter_moves_.clone()) },
            { (*self).with(|__s| __s.iter_swaps_.clone()) },
        );
    }
    fn operator_sub_11(&self, mut rhs: adl_Iterator) -> isize {
        return (((*self).with(|__s| __s.ptr_.clone()) - (rhs.ptr_).clone()) as i64 as isize);
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<adl_Iterator> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<adl_Iterator> {
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
    fn operator_post_inc_15(&self, mut _a0: i32) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return prev;
    }
    fn operator_dec_16(&self) -> Ptr<adl_Iterator> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_post_dec_17(&self, mut _a0: i32) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return prev;
    }
}
pub trait cpp17_input_iterator_int_ptr__int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: i32) -> cpp17_input_iterator_int_ptr__int_ptr_ {
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
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: i32) -> cpp17_output_iterator_int_ptr_ {
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
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_5(&self) -> Ptr<cpp20_input_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: i32) {
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
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait cpp20_output_iterator_int_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_output_iterator_int_ptr_>,
    ) -> Ptr<cpp20_output_iterator_int_ptr_>;
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_5(&self) -> Ptr<cpp20_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: i32) -> cpp20_output_iterator_int_ptr_ {
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
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait cpp20_random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_index(&self, n: isize) -> Ptr<i32>;
    fn operator_inc_7(&self) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_dec_8(&self) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: i32) -> cpp20_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: i32) -> cpp20_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_random_access_iterator_int_ptr_>,
    ) -> Ptr<cpp20_random_access_iterator_int_ptr_>;
}
impl cpp20_random_access_iterator_int_ptr_Impl for Ptr<cpp20_random_access_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
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
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_4(&self) -> Ptr<forward_sized_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: i32) -> forward_sized_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl forward_sized_iterator_int_ptr_Impl for Ptr<forward_sized_iterator_int_ptr_> {}
pub trait random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: i32) -> random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: i32) -> random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: isize) -> Ptr<random_access_iterator_int_ptr_>;
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_int_ptr_>,
    ) -> Ptr<random_access_iterator_int_ptr_>;
}
impl random_access_iterator_int_ptr_Impl for Ptr<random_access_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<random_access_iterator_int_ptr_> {
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
    fn operator_deref(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn operator_inc_4(&self) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: i32) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_dec_6(&self) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_dec_7(&self, mut _a0: i32) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add(&self, mut n: isize) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_sub_9(&self, mut n: isize) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_sub_10(&self, other: Ptr<rvalue_iterator_int_ptr_>) -> isize {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn operator_cmp(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> std::cmp::Ordering;
    fn operator_eq(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> bool;
}
impl rvalue_iterator_int_ptr_Impl for Ptr<rvalue_iterator_int_ptr_> {
    fn operator_cmp(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.it_.clone())),
                &(_a0.with(|__s| __s.it_.clone())),
            )));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow());
            }
        }
        return std::cmp::Ordering::Equal;
    }
    fn operator_eq(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> bool {
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
        assert!(!(other.with(|__s| __s.moved_from_)));
        field!(other, moved_from_).write(true);
        field!((*self), moved_from_).write(false);
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
