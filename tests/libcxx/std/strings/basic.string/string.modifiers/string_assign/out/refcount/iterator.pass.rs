extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn DoNotOptimize_0(value: Ptr<()>) -> Ptr<()> {
    b"";
    return (value).clone();
}
pub fn is_string_asan_correct_1(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_2(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_3(_a0: Ptr<()>) -> () {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_5.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_6.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_7.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_4.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_5.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_6.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_4: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_5: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_6: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_7: Value<()> = Rc::new(RefCell::new(false));
);
pub fn operator_sub_8(
    mut x: min_pointer_char__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_char__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_eq_9(
    mut x: min_pointer_char__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_char__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_char__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_char__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn pointer_to(t: Ptr<()>) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
impl std::cmp::PartialEq for min_pointer_char__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_9(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_char__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_2(mut p: min_pointer_char__std_integral_constant_unsigned_long__0__) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
    pub fn pointer_to(
        t: Ptr<()>,
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct safe_allocator_char_ {}
impl safe_allocator_char_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for safe_allocator_char_ {
    fn default() -> Self {
        { safe_allocator_char_::new() }
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
pub fn operator_ne_11(
    x: Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_>,
    y: Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
    pub fn new(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
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
pub fn test_19(s: (), mut first: Ptr<()>, mut last: Ptr<()>, mut expected: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_20(
    s: (),
    mut first: cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    mut last: cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    mut expected: (),
) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_21(s: (), mut first: Ptr<()>, mut last: Ptr<()>, mut expected: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_22(s: (), mut first: Ptr<()>, mut last: Ptr<()>, mut expected: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_23(
    s: (),
    mut first: cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    mut last: cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    mut expected: (),
) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_24(s: (), mut first: Ptr<()>, mut last: Ptr<()>, mut expected: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_25(s: (), mut first: Ptr<()>, mut last: Ptr<()>, mut expected: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_26(
    s: (),
    mut first: cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    mut last: cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    mut expected: (),
) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_27(s: (), mut first: Ptr<()>, mut last: Ptr<()>, mut expected: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_string_28() {
    {
        let mut s: Ptr<()> =
            Ptr::<()>::from_string_literal(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = (s).clone();
            test_19(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((1_()) as isize);
            test_19(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((10_()) as isize);
            test_19(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((52_()) as isize);
            test_19(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = (s).clone();
            test_19(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((1_()) as isize);
            test_19(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((10_()) as isize);
            test_19(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((52_()) as isize);
            test_19(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = (s).clone();
            test_19(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((1_()) as isize);
            test_19(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((10_()) as isize);
            test_19(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((52_()) as isize);
            test_19(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = (s).clone();
            test_19(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((1_()) as isize);
            test_19(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((10_()) as isize);
            test_19(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((52_()) as isize);
            test_19(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            test_20(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((1_()) as isize)
                });
            test_20(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((10_()) as isize)
                });
            test_20(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((52_()) as isize)
                });
            test_20(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            test_20(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((1_()) as isize)
                });
            test_20(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((10_()) as isize)
                });
            test_20(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((52_()) as isize)
                });
            test_20(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            test_20(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((1_()) as isize)
                });
            test_20(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((10_()) as isize)
                });
            test_20(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((52_()) as isize)
                });
            test_20(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            test_20(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((1_()) as isize)
                });
            test_20(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((10_()) as isize)
                });
            test_20(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((52_()) as isize)
                });
            test_20(unimplemented!(), _first, _last, unimplemented!())
        });
    }
    {
        let mut s_short: () = unimplemented!();
        let mut s_long: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<Box<[u8]>> = Rc::new(RefCell::new(u8::array_from_literal(b"ABCD\0")));
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut sneaky: () = unimplemented!();
        unimplemented!();
        let mut expected: () = unimplemented!();
        ({
            let _s: () = unimplemented!();
            let _first: Ptr<()> = unimplemented!();
            let _last: Ptr<()> = unimplemented!()
                .offset((unimplemented!()) as isize)
                .offset((1_()) as isize);
            test_21(_s, _first, _last, unimplemented!())
        });
    }
}
pub fn test_string_29() {
    {
        let mut s: Ptr<()> =
            Ptr::<()>::from_string_literal(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = (s).clone();
            test_22(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((1_()) as isize);
            test_22(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((10_()) as isize);
            test_22(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((52_()) as isize);
            test_22(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = (s).clone();
            test_22(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((1_()) as isize);
            test_22(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((10_()) as isize);
            test_22(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((52_()) as isize);
            test_22(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = (s).clone();
            test_22(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((1_()) as isize);
            test_22(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((10_()) as isize);
            test_22(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((52_()) as isize);
            test_22(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = (s).clone();
            test_22(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((1_()) as isize);
            test_22(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((10_()) as isize);
            test_22(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((52_()) as isize);
            test_22(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            test_23(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((1_()) as isize)
                });
            test_23(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((10_()) as isize)
                });
            test_23(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((52_()) as isize)
                });
            test_23(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            test_23(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((1_()) as isize)
                });
            test_23(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((10_()) as isize)
                });
            test_23(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((52_()) as isize)
                });
            test_23(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            test_23(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((1_()) as isize)
                });
            test_23(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((10_()) as isize)
                });
            test_23(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((52_()) as isize)
                });
            test_23(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            test_23(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((1_()) as isize)
                });
            test_23(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((10_()) as isize)
                });
            test_23(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((52_()) as isize)
                });
            test_23(unimplemented!(), _first, _last, unimplemented!())
        });
    }
    {
        let mut s_short: () = unimplemented!();
        let mut s_long: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<Box<[u8]>> = Rc::new(RefCell::new(u8::array_from_literal(b"ABCD\0")));
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut sneaky: () = unimplemented!();
        unimplemented!();
        let mut expected: () = unimplemented!();
        ({
            let _s: () = unimplemented!();
            let _first: Ptr<()> = unimplemented!();
            let _last: Ptr<()> = unimplemented!()
                .offset((unimplemented!()) as isize)
                .offset((1_()) as isize);
            test_24(_s, _first, _last, unimplemented!())
        });
    }
}
pub fn test_string_30() {
    {
        let mut s: Ptr<()> =
            Ptr::<()>::from_string_literal(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = (s).clone();
            test_25(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((1_()) as isize);
            test_25(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((10_()) as isize);
            test_25(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((52_()) as isize);
            test_25(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = (s).clone();
            test_25(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((1_()) as isize);
            test_25(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((10_()) as isize);
            test_25(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((52_()) as isize);
            test_25(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = (s).clone();
            test_25(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((1_()) as isize);
            test_25(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((10_()) as isize);
            test_25(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((52_()) as isize);
            test_25(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = (s).clone();
            test_25(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((1_()) as isize);
            test_25(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((10_()) as isize);
            test_25(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: Ptr<()> = (s).clone();
            let _last: Ptr<()> = s.offset((52_()) as isize);
            test_25(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            test_26(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((1_()) as isize)
                });
            test_26(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((10_()) as isize)
                });
            test_26(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((52_()) as isize)
                });
            test_26(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            test_26(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((1_()) as isize)
                });
            test_26(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((10_()) as isize)
                });
            test_26(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((52_()) as isize)
                });
            test_26(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            test_26(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((1_()) as isize)
                });
            test_26(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((10_()) as isize)
                });
            test_26(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((52_()) as isize)
                });
            test_26(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            test_26(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((1_()) as isize)
                });
            test_26(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((10_()) as isize)
                });
            test_26(unimplemented!(), _first, _last, unimplemented!())
        });
        ({
            let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ (s).clone() });
            let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
                cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                    s.offset((52_()) as isize)
                });
            test_26(unimplemented!(), _first, _last, unimplemented!())
        });
    }
    {
        let mut s_short: () = unimplemented!();
        let mut s_long: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<Box<[u8]>> = Rc::new(RefCell::new(u8::array_from_literal(b"ABCD\0")));
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut sneaky: () = unimplemented!();
        unimplemented!();
        let mut expected: () = unimplemented!();
        ({
            let _s: () = unimplemented!();
            let _first: Ptr<()> = unimplemented!();
            let _last: Ptr<()> = unimplemented!()
                .offset((unimplemented!()) as isize)
                .offset((1_()) as isize);
            test_27(_s, _first, _last, unimplemented!())
        });
    }
}
pub fn test_31() -> () {
    ({ test_string_28() });
    ({ test_string_29() });
    ({ test_string_30() });
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
    ({ test_31() });
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
pub trait cpp17_input_iterator_const_char_ptr__const_char_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_>;
    fn operator_post_inc_6(
        &self,
        mut _a0: (),
    ) -> cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
        unimplemented!()
    }
}
impl cpp17_input_iterator_const_char_ptr__const_char_ptr_Impl
    for Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_>
{
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_> {
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
pub trait min_allocator_char_Impl {
    fn allocate(&self, n: ()) -> min_pointer_char__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_char__std_integral_constant_unsigned_long__0__, n: ());
}
impl min_allocator_char_Impl for Ptr<min_allocator_char_> {
    fn allocate(&self, mut n: ()) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_char__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_pointer_char__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_8(&self) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(&self, n: ()) -> min_pointer_char__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, n: ()) -> Ptr<()>;
}
impl min_pointer_char__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__>
{
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_char__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_char__std_integral_constant_unsigned_long__0__Impl::operator_add_assign(
                &tmp.as_pointer(),
                n,
            )
        });
        return (*tmp.borrow()).clone();
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
}
pub trait min_pointer_const_char__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_char__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
}
pub trait min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
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
pub trait safe_allocator_char_Impl {
    fn allocate(&self, n: ()) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, n: ());
}
impl safe_allocator_char_Impl for Ptr<safe_allocator_char_> {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        let memory: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
        {
            ({
                memset_10(
                    (*memory.borrow()).to_any(),
                    0_(),
                    (::std::mem::size_of::<()>() as ()).wrapping_mul(n),
                )
            });
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        {
            ({
                let _value: Value<()> = Rc::new(RefCell::new(
                    ({
                        memset_10(
                            (p).to_any(),
                            0_(),
                            (::std::mem::size_of::<()>() as ()).wrapping_mul(n),
                        )
                    }),
                ));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        unimplemented!();
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
    let _ = outstanding_bytes_4.with(|_| ());
    let _ = alloc_count_5.with(|_| ());
    let _ = dealloc_count_6.with(|_| ());
    let _ = disable_default_constructor_7.with(|_| ());
}
