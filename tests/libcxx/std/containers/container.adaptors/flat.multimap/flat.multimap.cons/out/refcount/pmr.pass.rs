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
pub fn operator_ne_0(
    x: Ptr<cpp17_input_iterator_const_std_pair_int__long__ptr__const_std_pair_int__long__ptr_>,
    y: Ptr<cpp17_input_iterator_const_std_pair_int__long__ptr__const_std_pair_int__long__ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_const_std_pair_int__long__ptr__const_std_pair_int__long__ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_std_pair_int__long__ptr__const_std_pair_int__long__ptr_ {
    pub fn new(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(
        _a0: Ptr<
            cpp17_input_iterator_const_std_pair_int__long__ptr__const_std_pair_int__long__ptr_,
        >,
    ) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
pub fn operator_ne_1(
    x: Ptr<cpp17_input_iterator_const_std_pair_int__int__ptr__const_std_pair_int__int__ptr_>,
    y: Ptr<cpp17_input_iterator_const_std_pair_int__int__ptr__const_std_pair_int__int__ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_const_std_pair_int__int__ptr__const_std_pair_int__int__ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_std_pair_int__int__ptr__const_std_pair_int__int__ptr_ {
    pub fn new(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(
        _a0: Ptr<cpp17_input_iterator_const_std_pair_int__int__ptr__const_std_pair_int__int__ptr_>,
    ) -> Self {
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
pub fn base_2(i: Ptr<cpp20_input_iterator_const_std_pair_int__long__ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_input_iterator_const_std_pair_int__long__ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_const_std_pair_int__long__ptr_ {
    pub fn new(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<cpp20_input_iterator_const_std_pair_int__long__ptr_>) -> Self {
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
#[byte_size(8)]
pub struct sentinel_wrapper_cpp20_input_iterator_const_std_pair_int__long__ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_cpp20_input_iterator_const_std_pair_int__long__ptr__ {
    pub fn new_1(it: Ptr<cpp20_input_iterator_const_std_pair_int__long__ptr_>) -> Self {
        Self {
            base_: ({ base_2((it).clone()) }),
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
pub fn iter_swap_6(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_7(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) }).reinterpret_cast::<()>();
}
pub fn operator_eq_8(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_9(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
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
            operator_cmp_9(
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
            operator_eq_8(
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct test_allocator_statistics {
    #[offset(0)]
    pub time_to_throw: (),
    #[offset(4)]
    pub throw_after: (),
    #[offset(8)]
    pub count: (),
    #[offset(12)]
    pub alloc_count: (),
    #[offset(16)]
    pub allocated_size: (),
    #[offset(20)]
    pub construct_count: (),
    #[offset(24)]
    pub destroy_count: (),
    #[offset(28)]
    pub copied: (),
    #[offset(32)]
    pub moved: (),
    #[offset(36)]
    pub converted: (),
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        {
            test_allocator_statistics {
                time_to_throw: 0_(),
                throw_after: 2147483647_(),
                count: 0_(),
                alloc_count: 0_(),
                allocated_size: 0_(),
                construct_count: 0_(),
                destroy_count: 0_(),
                copied: 0_(),
                moved: 0_(),
                converted: 0_(),
            }
        }
    }
}
thread_local!(
    pub static destructed_value_10: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_11: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_12(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_13(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_12(_x, _y)
    });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(16)]
pub struct test_allocator_void_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_void_ {
    pub fn new_1(mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: 0_(),
            id_: 0_(),
            stats_: (stats).clone(),
        }
    }
    pub fn new_2(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_3(mut data: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: (stats).clone(),
        }
    }
    pub fn new_4(mut data: (), mut id: ()) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_5(mut data: (), mut id: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: (stats).clone(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_void_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }
    }
}
impl std::cmp::PartialEq for test_allocator_void_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_12(
                Rc::new(RefCell::new(test_allocator_void_ {
                    data_: self.data_.clone(),
                    id_: self.id_.clone(),
                    stats_: self.stats_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_allocator_void_ {
                    data_: other.data_.clone(),
                    id_: other.id_.clone(),
                    stats_: other.stats_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_void_ {}
impl Clone for test_allocator_void_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_void_> = Rc::new(RefCell::new(test_allocator_void_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_void_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_void_ {
    fn default() -> Self {
        {
            test_allocator_void_ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Ctor_Tag {}
#[derive(Record, ByteRepr, FnPtrArg, CopyAssign, MoveAssign, Default)]
#[byte_size(1)]
pub struct Tag_X {}
impl Tag_X {
    pub fn new_1(mut _a0: Ctor_Tag) -> Self {
        Self {}
    }
    pub fn new_2(mut _a0: Ctor_Tag, _a1: Ptr<Tag_X>) -> Self {
        Self {}
    }
    pub fn new_3(mut _a0: Ctor_Tag, _a1: Ptr<Tag_X>) -> Self {
        Self {}
    }
}
pub fn operator_eq_14(a: Ptr<test_less_int_>, b: Ptr<test_less_int_>) -> () {
    return ({ a.with(|__s| __s.data_) } == { b.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct test_less_int_ {
    #[offset(0)]
    pub data_: (),
}
impl test_less_int_ {
    pub fn new_1() -> Self {
        Self { data_: 0_() }
    }
    pub fn new_2(mut data: ()) -> Self {
        Self { data_: data }
    }
}
impl std::cmp::PartialEq for test_less_int_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_14(
                Rc::new(RefCell::new(test_less_int_ {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_less_int_ {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_less_int_ {}
impl Default for test_less_int_ {
    fn default() -> Self {
        { test_less_int_::new_1() }
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
    {
        let mr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let pa: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let m1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let m2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vm: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vm: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ks: () = unimplemented!();
        let vs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        let mr1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mr2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mo: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
    {
        let vs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let mr1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mr2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mo: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mo: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        let __decomp_15 : Value<std_flat_multimap_int__int__test_less_int___std_vector_int__std_pmr_polymorphic_allocator_int____std_vector_int__std_pmr_polymorphic_allocator_int____containers > = Rc::new(RefCell::new(( { (*m.borrow()) . extract ( ) } )  )) ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let __decomp_16 : Value<std_flat_multimap_int__int__test_less_int___std_vector_int__std_pmr_polymorphic_allocator_int____std_vector_int__std_pmr_polymorphic_allocator_int____containers > = Rc::new(RefCell::new(( { (*mo.borrow()) . extract ( ) } )  )) ;
        unimplemented!();
        unimplemented!();
    }
    {
        let mr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vm: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let il: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vm: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let il: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_0: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_1: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_int__long_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
            },
            {
                let __tmp_2: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_3: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_int__long_::new_1({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() })
            },
            {
                let __tmp_4: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_5: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_int__long_::new_1({ __tmp_4.as_pointer() }, { __tmp_5.as_pointer() })
            },
            {
                let __tmp_6: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_7: Value<()> = Rc::new(RefCell::new(4_()));
                std_pair_int__long_::new_1({ __tmp_6.as_pointer() }, { __tmp_7.as_pointer() })
            },
            {
                let __tmp_8: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_9: Value<()> = Rc::new(RefCell::new(5_()));
                std_pair_int__long_::new_1({ __tmp_8.as_pointer() }, { __tmp_9.as_pointer() })
            },
            {
                let __tmp_10: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_11: Value<()> = Rc::new(RefCell::new(6_()));
                std_pair_int__long_::new_1({ __tmp_10.as_pointer() }, { __tmp_11.as_pointer() })
            },
            {
                let __tmp_12: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_13: Value<()> = Rc::new(RefCell::new(7_()));
                std_pair_int__long_::new_1({ __tmp_12.as_pointer() }, { __tmp_13.as_pointer() })
            },
            {
                let __tmp_14: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_15: Value<()> = Rc::new(RefCell::new(8_()));
                std_pair_int__long_::new_1({ __tmp_14.as_pointer() }, { __tmp_15.as_pointer() })
            },
            {
                let __tmp_16: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_17: Value<()> = Rc::new(RefCell::new(9_()));
                std_pair_int__long_::new_1({ __tmp_16.as_pointer() }, { __tmp_17.as_pointer() })
            },
        ])));
        let expected: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_18: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_19: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_int__long_::new_1({ __tmp_18.as_pointer() }, { __tmp_19.as_pointer() })
            },
            {
                let __tmp_20: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_21: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_int__long_::new_1({ __tmp_20.as_pointer() }, { __tmp_21.as_pointer() })
            },
            {
                let __tmp_22: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_23: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_int__long_::new_1({ __tmp_22.as_pointer() }, { __tmp_23.as_pointer() })
            },
            {
                let __tmp_24: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_25: Value<()> = Rc::new(RefCell::new(4_()));
                std_pair_int__long_::new_1({ __tmp_24.as_pointer() }, { __tmp_25.as_pointer() })
            },
            {
                let __tmp_26: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_27: Value<()> = Rc::new(RefCell::new(5_()));
                std_pair_int__long_::new_1({ __tmp_26.as_pointer() }, { __tmp_27.as_pointer() })
            },
            {
                let __tmp_28: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_29: Value<()> = Rc::new(RefCell::new(7_()));
                std_pair_int__long_::new_1({ __tmp_28.as_pointer() }, { __tmp_29.as_pointer() })
            },
            {
                let __tmp_30: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_31: Value<()> = Rc::new(RefCell::new(6_()));
                std_pair_int__long_::new_1({ __tmp_30.as_pointer() }, { __tmp_31.as_pointer() })
            },
            {
                let __tmp_32: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_33: Value<()> = Rc::new(RefCell::new(8_()));
                std_pair_int__long_::new_1({ __tmp_32.as_pointer() }, { __tmp_33.as_pointer() })
            },
            {
                let __tmp_34: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_35: Value<()> = Rc::new(RefCell::new(9_()));
                std_pair_int__long_::new_1({ __tmp_34.as_pointer() }, { __tmp_35.as_pointer() })
            },
        ])));
        {
            let mr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let vm: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let vm: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let expected: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_36: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_37: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_int__int_::new_1({ __tmp_36.as_pointer() }, { __tmp_37.as_pointer() })
            },
            {
                let __tmp_38: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_39: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_int__int_::new_1({ __tmp_38.as_pointer() }, { __tmp_39.as_pointer() })
            },
            {
                let __tmp_40: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_41: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_int__int_::new_1({ __tmp_40.as_pointer() }, { __tmp_41.as_pointer() })
            },
            {
                let __tmp_42: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_43: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_int__int_::new_1({ __tmp_42.as_pointer() }, { __tmp_43.as_pointer() })
            },
        ])));
        let mr1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mr2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mo: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
    {
        let vs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mr1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mr2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mo: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_44: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_45: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_int__long_::new_1({ __tmp_44.as_pointer() }, { __tmp_45.as_pointer() })
            },
            {
                let __tmp_46: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_47: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_int__long_::new_1({ __tmp_46.as_pointer() }, { __tmp_47.as_pointer() })
            },
            {
                let __tmp_48: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_49: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_int__long_::new_1({ __tmp_48.as_pointer() }, { __tmp_49.as_pointer() })
            },
            {
                let __tmp_50: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_51: Value<()> = Rc::new(RefCell::new(4_()));
                std_pair_int__long_::new_1({ __tmp_50.as_pointer() }, { __tmp_51.as_pointer() })
            },
            {
                let __tmp_52: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_53: Value<()> = Rc::new(RefCell::new(5_()));
                std_pair_int__long_::new_1({ __tmp_52.as_pointer() }, { __tmp_53.as_pointer() })
            },
            {
                let __tmp_54: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_55: Value<()> = Rc::new(RefCell::new(6_()));
                std_pair_int__long_::new_1({ __tmp_54.as_pointer() }, { __tmp_55.as_pointer() })
            },
            {
                let __tmp_56: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_57: Value<()> = Rc::new(RefCell::new(7_()));
                std_pair_int__long_::new_1({ __tmp_56.as_pointer() }, { __tmp_57.as_pointer() })
            },
            {
                let __tmp_58: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_59: Value<()> = Rc::new(RefCell::new(8_()));
                std_pair_int__long_::new_1({ __tmp_58.as_pointer() }, { __tmp_59.as_pointer() })
            },
            {
                let __tmp_60: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_61: Value<()> = Rc::new(RefCell::new(9_()));
                std_pair_int__long_::new_1({ __tmp_60.as_pointer() }, { __tmp_61.as_pointer() })
            },
        ])));
        let expected: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_62: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_63: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_int__long_::new_1({ __tmp_62.as_pointer() }, { __tmp_63.as_pointer() })
            },
            {
                let __tmp_64: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_65: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_int__long_::new_1({ __tmp_64.as_pointer() }, { __tmp_65.as_pointer() })
            },
            {
                let __tmp_66: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_67: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_int__long_::new_1({ __tmp_66.as_pointer() }, { __tmp_67.as_pointer() })
            },
            {
                let __tmp_68: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_69: Value<()> = Rc::new(RefCell::new(4_()));
                std_pair_int__long_::new_1({ __tmp_68.as_pointer() }, { __tmp_69.as_pointer() })
            },
            {
                let __tmp_70: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_71: Value<()> = Rc::new(RefCell::new(5_()));
                std_pair_int__long_::new_1({ __tmp_70.as_pointer() }, { __tmp_71.as_pointer() })
            },
            {
                let __tmp_72: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_73: Value<()> = Rc::new(RefCell::new(7_()));
                std_pair_int__long_::new_1({ __tmp_72.as_pointer() }, { __tmp_73.as_pointer() })
            },
            {
                let __tmp_74: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_75: Value<()> = Rc::new(RefCell::new(6_()));
                std_pair_int__long_::new_1({ __tmp_74.as_pointer() }, { __tmp_75.as_pointer() })
            },
            {
                let __tmp_76: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_77: Value<()> = Rc::new(RefCell::new(8_()));
                std_pair_int__long_::new_1({ __tmp_76.as_pointer() }, { __tmp_77.as_pointer() })
            },
            {
                let __tmp_78: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_79: Value<()> = Rc::new(RefCell::new(9_()));
                std_pair_int__long_::new_1({ __tmp_78.as_pointer() }, { __tmp_79.as_pointer() })
            },
        ])));
        {
            let mr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let vm: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let vm: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vm: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ks: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vm: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ks: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vm: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_80: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_81: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_int__int_::new_1({ __tmp_80.as_pointer() }, { __tmp_81.as_pointer() })
            },
            {
                let __tmp_82: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_83: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_int__int_::new_1({ __tmp_82.as_pointer() }, { __tmp_83.as_pointer() })
            },
            {
                let __tmp_84: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_85: Value<()> = Rc::new(RefCell::new(4_()));
                std_pair_int__int_::new_1({ __tmp_84.as_pointer() }, { __tmp_85.as_pointer() })
            },
            {
                let __tmp_86: Value<()> = Rc::new(RefCell::new(5_()));
                let __tmp_87: Value<()> = Rc::new(RefCell::new(5_()));
                std_pair_int__int_::new_1({ __tmp_86.as_pointer() }, { __tmp_87.as_pointer() })
            },
        ])));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vm: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([{
            let __tmp_88: Value<()> = Rc::new(RefCell::new(42_()));
            let __tmp_89: Value<()> = Rc::new(RefCell::new(42_()));
            std_pair_int__int_::new_1({ __tmp_88.as_pointer() }, { __tmp_89.as_pointer() })
        }])));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vm: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_90: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_91: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_int__int_::new_1({ __tmp_90.as_pointer() }, { __tmp_91.as_pointer() })
            },
            {
                let __tmp_92: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_93: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_int__int_::new_1({ __tmp_92.as_pointer() }, { __tmp_93.as_pointer() })
            },
            {
                let __tmp_94: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_95: Value<()> = Rc::new(RefCell::new(4_()));
                std_pair_int__int_::new_1({ __tmp_94.as_pointer() }, { __tmp_95.as_pointer() })
            },
            {
                let __tmp_96: Value<()> = Rc::new(RefCell::new(5_()));
                let __tmp_97: Value<()> = Rc::new(RefCell::new(5_()));
                std_pair_int__int_::new_1({ __tmp_96.as_pointer() }, { __tmp_97.as_pointer() })
            },
        ])));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vm: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([{
            let __tmp_98: Value<()> = Rc::new(RefCell::new(42_()));
            let __tmp_99: Value<()> = Rc::new(RefCell::new(42_()));
            std_pair_int__int_::new_1({ __tmp_98.as_pointer() }, { __tmp_99.as_pointer() })
        }])));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub trait Tag_XImpl {
    fn copy_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X>;
    fn move_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X>;
}
impl Tag_XImpl for Ptr<Tag_X> {
    fn copy_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X> {
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X> {
        return (*self).clone();
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
pub trait cpp17_input_iterator_const_std_pair_int__int__ptr__const_std_pair_int__int__ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_5(
        &self,
    ) -> Ptr<cpp17_input_iterator_const_std_pair_int__int__ptr__const_std_pair_int__int__ptr_>;
    fn operator_post_inc_6(
        &self,
        mut _a0: (),
    ) -> cpp17_input_iterator_const_std_pair_int__int__ptr__const_std_pair_int__int__ptr_ {
        unimplemented!()
    }
}
impl cpp17_input_iterator_const_std_pair_int__int__ptr__const_std_pair_int__int__ptr_Impl
    for Ptr<cpp17_input_iterator_const_std_pair_int__int__ptr__const_std_pair_int__int__ptr_>
{
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_5(
        &self,
    ) -> Ptr<cpp17_input_iterator_const_std_pair_int__int__ptr__const_std_pair_int__int__ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait cpp17_input_iterator_const_std_pair_int__long__ptr__const_std_pair_int__long__ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_5(
        &self,
    ) -> Ptr<cpp17_input_iterator_const_std_pair_int__long__ptr__const_std_pair_int__long__ptr_>;
    fn operator_post_inc_6(
        &self,
        mut _a0: (),
    ) -> cpp17_input_iterator_const_std_pair_int__long__ptr__const_std_pair_int__long__ptr_ {
        unimplemented!()
    }
}
impl cpp17_input_iterator_const_std_pair_int__long__ptr__const_std_pair_int__long__ptr_Impl
    for Ptr<cpp17_input_iterator_const_std_pair_int__long__ptr__const_std_pair_int__long__ptr_>
{
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_5(
        &self,
    ) -> Ptr<cpp17_input_iterator_const_std_pair_int__long__ptr__const_std_pair_int__long__ptr_>
    {
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
pub trait cpp20_input_iterator_const_std_pair_int__long__ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_input_iterator_const_std_pair_int__long__ptr_>,
    ) -> Ptr<cpp20_input_iterator_const_std_pair_int__long__ptr_>;
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_inc_5(&self) -> Ptr<cpp20_input_iterator_const_std_pair_int__long__ptr_>;
    fn operator_post_inc_6(&self, mut _a0: ()) {
        unimplemented!()
    }
}
impl cpp20_input_iterator_const_std_pair_int__long__ptr_Impl
    for Ptr<cpp20_input_iterator_const_std_pair_int__long__ptr_>
{
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_input_iterator_const_std_pair_int__long__ptr_>,
    ) -> Ptr<cpp20_input_iterator_const_std_pair_int__long__ptr_> {
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
    fn operator_inc_5(&self) -> Ptr<cpp20_input_iterator_const_std_pair_int__long__ptr_> {
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
pub trait sentinel_wrapper_cpp20_input_iterator_const_std_pair_int__long__ptr__Impl {
    fn operator_eq(&self, other: Ptr<cpp20_input_iterator_const_std_pair_int__long__ptr_>) -> ();
}
impl sentinel_wrapper_cpp20_input_iterator_const_std_pair_int__long__ptr__Impl
    for Ptr<sentinel_wrapper_cpp20_input_iterator_const_std_pair_int__long__ptr__>
{
    fn operator_eq(&self, other: Ptr<cpp20_input_iterator_const_std_pair_int__long__ptr_>) -> () {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_2((other).clone()) }) });
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
pub trait test_allocator_statisticsImpl {
    fn clear(&self);
    fn clear_ctor_counters(&self);
}
impl test_allocator_statisticsImpl for Ptr<test_allocator_statistics> {
    fn clear(&self) {
        unimplemented!();
        field!((*self), count).write(0_());
        field!((*self), time_to_throw).write(0_());
        field!((*self), alloc_count).write(0_());
        field!((*self), allocated_size).write(0_());
        field!((*self), construct_count).write(0_());
        field!((*self), destroy_count).write(0_());
        field!((*self), throw_after).write(2147483647_());
        ({ test_allocator_statisticsImpl::clear_ctor_counters(self) });
    }
    fn clear_ctor_counters(&self) {
        field!((*self), copied).write(0_());
        field!((*self), moved).write(0_());
        field!((*self), converted).write(0_());
    }
}
pub trait test_allocator_void_Impl {
    fn destructor(&self);
    fn get_id(&self) -> ();
    fn get_data(&self) -> ();
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_>;
}
impl test_allocator_void_Impl for Ptr<test_allocator_void_> {
    fn destructor(&self) {
        field!((*self), data_).write(destructed_value_10.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_10.with(|rc| *rc.borrow()));
    }
    fn get_id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
    fn get_data(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_> {
        field!((*self), data_).write({ _a0.with(|__s| __s.data_) });
        field!((*self), id_).write({ _a0.with(|__s| __s.id_) });
        field!((*self), stats_).write({ _a0.with(|__s| __s.stats_.clone()) });
        return (*self).clone();
    }
}
pub trait test_less_int_Impl {
    fn operator_call(&self, a: Ptr<()>, b: Ptr<()>) -> ();
}
impl test_less_int_Impl for Ptr<test_less_int_> {
    fn operator_call(&self, a: Ptr<()>, b: Ptr<()>) -> () {
        return ({ (a.read()) } < { (b.read()) });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = destructed_value_10.with(|_| ());
    let _ = moved_value_11.with(|_| ());
}
