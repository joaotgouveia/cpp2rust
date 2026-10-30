extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_long__long_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct three_way_random_access_iterator_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl three_way_random_access_iterator_const_int_ptr_ {
    pub fn move_from(_a0: Ptr<three_way_random_access_iterator_const_int_ptr_>) -> Self {
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
pub fn operator_eq_2(
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
        operator_eq_2(__lhs.clone(), __rhs.clone())
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
pub fn operator_add_3(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_4(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_3(_i, n)
    });
}
pub fn iter_swap_5(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_6(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_7(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_8(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
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
        operator_cmp_8(__lhs.clone(), __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for adl_Iterator {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_7(__lhs.clone(), __rhs.clone())
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__
{
    #[offset(0)]
    #[byte_size(24)]
    data_: (),
}
impl MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__ { pub fn move_from ( _a0 : Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  > , ) -> Self { Self { data_ : unimplemented!()  , }  } }
const _: () = assert!(!(false), "!HasAllocatorType<MinSequenceContainer<int>>");
const _: () = assert!(!(false), "!HasEmplace<MinSequenceContainer<int>>");
pub fn alloc_max_size_9(a: Ptr<()>) -> () {
    return unimplemented!();
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
impl CppEq for test_allocator_void_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_12(__lhs.clone(), __rhs.clone())
    }
}
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct limited_alloc_handle_10_ {
    #[offset(0)]
    pub outstanding_: (),
    #[offset(8)]
    #[byte_size(8)]
    pub last_alloc_: (),
}
impl Default for limited_alloc_handle_10_ {
    fn default() -> Self {
        {
            limited_alloc_handle_10_ {
                outstanding_: 0_(),
                last_alloc_: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct limited_alloc_handle_18446744073709551615_ {
    #[offset(0)]
    pub outstanding_: (),
    #[offset(8)]
    #[byte_size(8)]
    pub last_alloc_: (),
}
impl Default for limited_alloc_handle_18446744073709551615_ {
    fn default() -> Self {
        {
            limited_alloc_handle_18446744073709551615_ {
                outstanding_: 0_(),
                last_alloc_: unimplemented!(),
            }
        }
    }
}
pub fn make_thread_unsafe_shared_14() -> detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ {
    let ptr: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__> = Rc::new(
        RefCell::new(<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__>::default()),
    );
    let _dtor_ptr = ScopedDestructor::new(&ptr, |__p| __p.destructor());
    let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*ptr.borrow_mut()).block = unimplemented!();
    unimplemented!();
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__::copy_from({
        ptr.as_pointer()
    });
}
pub fn make_thread_unsafe_shared_15()
-> detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ {
    let ptr: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__> =
        Rc::new(RefCell::new(
            <detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__>::default(
            ),
        ));
    let _dtor_ptr = ScopedDestructor::new(&ptr, |__p| __p.destructor());
    let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*ptr.borrow_mut()).block = unimplemented!();
    unimplemented!();
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__::copy_from(
        { ptr.as_pointer() },
    );
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block {
    pub fn new() -> Self {
        Self {
            ref_count: 1_(),
            content: <limited_alloc_handle_10_>::default(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block {
    #[offset(0)]
    pub ref_count: (),
    #[offset(8)]
    #[byte_size(16)]
    pub content: limited_alloc_handle_10_,
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block {
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block {
                ref_count: 1_(),
                content: <limited_alloc_handle_10_>::default(),
            }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(8)]
pub struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ {
    #[offset(0)]
    #[byte_size(8)]
    block: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block>,
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ {
    pub fn copy_from(
        other: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__>,
    ) -> Self {
        let __this: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__> =
            Rc::new(RefCell::new(Self {
                block: other.with(|__s| __s.block.clone()),
            }));
        let this: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__> =
            __this.as_pointer();
        field!(this.with(|__s| __s.block.clone()), ref_count).with_mut(|__v| __v.prefix_inc());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ {
    fn clone(&self) -> Self {
        let __src: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__> = Rc::new(
            RefCell::new(detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ {
                block: self.block.clone(),
            }),
        );
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__::copy_from(__src.as_pointer())
    }
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ {
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ { block : Ptr::<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block>::null()  , }
        }
    }
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block {
    pub fn new() -> Self {
        Self {
            ref_count: 1_(),
            content: <limited_alloc_handle_18446744073709551615_>::default(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block {
    #[offset(0)]
    pub ref_count: (),
    #[offset(8)]
    #[byte_size(16)]
    pub content: limited_alloc_handle_18446744073709551615_,
}
impl Default
    for detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block
{
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block { ref_count : 1_()  , content : <limited_alloc_handle_18446744073709551615_  >::default() , }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(8)]
pub struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ {
    #[offset(0)]
    #[byte_size(8)]
    block: Ptr<
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block,
    >,
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ {
    pub fn copy_from(
        other: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__>,
    ) -> Self {
        let __this: Value<
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__,
        > = Rc::new(RefCell::new(Self {
            block: other.with(|__s| __s.block.clone()),
        }));
        let this: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__> =
            __this.as_pointer();
        field!(this.with(|__s| __s.block.clone()), ref_count).with_mut(|__v| __v.prefix_inc());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ {
    fn clone(&self) -> Self {
        let __src: Value<
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__,
        > = Rc::new(RefCell::new(
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ {
                block: self.block.clone(),
            },
        ));
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__::copy_from(
            __src.as_pointer(),
        )
    }
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ {
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ { block : Ptr::<detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block>::null()  , }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_int__10__rebind_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct limited_allocator_int__10_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__,
}
impl limited_allocator_int__10_ {
    pub fn new() -> Self {
        Self {
            handle_: ({ make_thread_unsafe_shared_14() }),
        }
    }
}
impl Default for limited_allocator_int__10_ {
    fn default() -> Self {
        { limited_allocator_int__10_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_int__18446744073709551615__rebind_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct limited_allocator_int__18446744073709551615_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__,
}
impl limited_allocator_int__18446744073709551615_ {
    pub fn new() -> Self {
        Self {
            handle_: ({ make_thread_unsafe_shared_15() }),
        }
    }
}
impl Default for limited_allocator_int__18446744073709551615_ {
    fn default() -> Self {
        { limited_allocator_int__18446744073709551615_::new() }
    }
}
pub fn test_16() -> () {
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let mut max_dist: () = unimplemented!();
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let mut max_dist: () = unimplemented!();
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
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
    ({ test_16() });
    return 0_();
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct three_way_random_access_iterator_int_ptr_;
pub trait MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl
{
    fn operator_assign_6 ( &self , il : ()  , ) -> Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  >{
        unimplemented!()
    }
    fn assign_8(&self, il: ()) {
        unimplemented!()
    }
    fn assign_9(&self, mut n: (), mut t: ()) {
        unimplemented!()
    }
    fn begin_11(&self) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn begin_12(&self) -> three_way_random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn cbegin(&self) -> three_way_random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn end_14(&self) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn end_15(&self) -> three_way_random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn cend(&self) -> three_way_random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn size(&self) -> () {
        unimplemented!()
    }
    fn empty(&self) -> () {
        unimplemented!()
    }
    fn clear(&self) {
        unimplemented!()
    }
    fn insert_21(
        &self,
        mut p: three_way_random_access_iterator_const_int_ptr_,
        mut value: (),
    ) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn insert_22(
        &self,
        mut p: three_way_random_access_iterator_const_int_ptr_,
        mut n: (),
        mut value: (),
    ) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn insert_23(
        &self,
        mut p: three_way_random_access_iterator_const_int_ptr_,
        il: (),
    ) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn erase_25(
        &self,
        mut first: three_way_random_access_iterator_const_int_ptr_,
        mut last: three_way_random_access_iterator_const_int_ptr_,
    ) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn erase_26(
        &self,
        mut iter: three_way_random_access_iterator_const_int_ptr_,
    ) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn to_vector_iterator(&self, mut cit: three_way_random_access_iterator_const_int_ptr_) -> () {
        unimplemented!()
    }
    fn from_vector_iterator(&self, it: ()) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn move_assign ( &self , _a0 : Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  > , ) -> Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  > ;
}
impl MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl for Ptr<MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__> {
fn move_assign ( &self , _a0 : Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  > , ) -> Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  > { unimplemented!()  ;
 return ((* self )  ).clone()  ;
 } }
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
pub trait detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__Impl {
    fn destructor(&self);
    fn operator_deref_4(&self) -> Ptr<limited_alloc_handle_10_> {
        unimplemented!()
    }
    fn operator_arrow_5(&self) -> Ptr<limited_alloc_handle_10_> {
        unimplemented!()
    }
    fn operator_deref_6(&self) -> Ptr<limited_alloc_handle_10_> {
        unimplemented!()
    }
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_10_>;
    fn get_8(&self) -> Ptr<limited_alloc_handle_10_> {
        unimplemented!()
    }
    fn get_9(&self) -> Ptr<limited_alloc_handle_10_> {
        unimplemented!()
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__>;
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__Impl
    for Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__>
{
    fn destructor(&self) {
        field!((*self).with(|__s| __s.block.clone()), ref_count).with_mut(|__v| __v.prefix_dec());
        if ((*self)
            .with(|__s| __s.block.clone())
            .with(|__s| __s.ref_count)
            != 0_())
        {
            return;
        };
        let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_10_> {
        return (field_ptr!((*self).with(|__s| __s.block.clone()), content));
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__> {
        field!((*self), block).write({ _a0.with(|__s| __s.block.clone()) });
        return (*self).clone();
    }
}
pub trait detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__Impl {
    fn destructor(&self);
    fn operator_deref_4(&self) -> Ptr<limited_alloc_handle_18446744073709551615_> {
        unimplemented!()
    }
    fn operator_arrow_5(&self) -> Ptr<limited_alloc_handle_18446744073709551615_> {
        unimplemented!()
    }
    fn operator_deref_6(&self) -> Ptr<limited_alloc_handle_18446744073709551615_> {
        unimplemented!()
    }
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_18446744073709551615_>;
    fn get_8(&self) -> Ptr<limited_alloc_handle_18446744073709551615_> {
        unimplemented!()
    }
    fn get_9(&self) -> Ptr<limited_alloc_handle_18446744073709551615_> {
        unimplemented!()
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__>;
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__Impl
    for Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__>
{
    fn destructor(&self) {
        field!((*self).with(|__s| __s.block.clone()), ref_count).with_mut(|__v| __v.prefix_dec());
        if ((*self)
            .with(|__s| __s.block.clone())
            .with(|__s| __s.ref_count)
            != 0_())
        {
            return;
        };
        let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_18446744073709551615_> {
        return (field_ptr!((*self).with(|__s| __s.block.clone()), content));
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__> {
        field!((*self), block).write({ _a0.with(|__s| __s.block.clone()) });
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
pub trait limited_alloc_handle_10_Impl {
    fn deallocate_2_int(&self, ptr: Ptr<()>, N: ());
}
impl limited_alloc_handle_10_Impl for Ptr<limited_alloc_handle_10_> {
    fn deallocate_2_int(&self, mut ptr: Ptr<()>, mut N: ()) {
        if ({ (ptr).to_any() } == { (*self).with(|__s| __s.last_alloc_.clone()) }) {
            field!((*self), last_alloc_).write(unimplemented!());
            unimplemented!();
            field!((*self), outstanding_)
                .write({ ((*self).with(|__s| __s.outstanding_)).wrapping_sub(N) });
        }
        unimplemented!();
    }
}
pub trait limited_alloc_handle_18446744073709551615_Impl {
    fn deallocate_2_int(&self, ptr: Ptr<()>, N: ());
}
impl limited_alloc_handle_18446744073709551615_Impl
    for Ptr<limited_alloc_handle_18446744073709551615_>
{
    fn deallocate_2_int(&self, mut ptr: Ptr<()>, mut N: ()) {
        if ({ (ptr).to_any() } == { (*self).with(|__s| __s.last_alloc_.clone()) }) {
            field!((*self), last_alloc_).write(unimplemented!());
            unimplemented!();
            field!((*self), outstanding_)
                .write({ ((*self).with(|__s| __s.outstanding_)).wrapping_sub(N) });
        }
        unimplemented!();
    }
}
pub trait limited_allocator_int__10_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<()>, n: ());
    fn max_size(&self) -> ();
    fn getHandle(&self) -> Ptr<limited_alloc_handle_10_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_int__10_Impl for Ptr<limited_allocator_int__10_> {
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        ({
            limited_alloc_handle_10_Impl::deallocate_2_int(
                &({
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__Impl::operator_arrow_7(
                        &field_ptr!((*self), handle_),
                    )
                }),
                (p).clone(),
                n,
            )
        });
    }
    fn max_size(&self) -> () {
        return 10_();
    }
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait limited_allocator_int__18446744073709551615_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<()>, n: ());
    fn max_size(&self) -> ();
    fn getHandle(&self) -> Ptr<limited_alloc_handle_18446744073709551615_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_int__18446744073709551615_Impl
    for Ptr<limited_allocator_int__18446744073709551615_>
{
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        ({
            limited_alloc_handle_18446744073709551615_Impl::deallocate_2_int(
                &({
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__Impl :: operator_arrow_7 ( &field_ptr!((* self )  , handle_)  , )
                }),
                (p).clone(),
                n,
            )
        });
    }
    fn max_size(&self) -> () {
        return 18446744073709551615_();
    }
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
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
pub trait three_way_random_access_iterator_const_int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<three_way_random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_dec_8(&self) -> Ptr<three_way_random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: ()) -> three_way_random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> three_way_random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<three_way_random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<three_way_random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<three_way_random_access_iterator_const_int_ptr_>,
    ) -> Ptr<three_way_random_access_iterator_const_int_ptr_>;
}
impl three_way_random_access_iterator_const_int_ptr_Impl
    for Ptr<three_way_random_access_iterator_const_int_ptr_>
{
    fn move_assign(
        &self,
        _a0: Ptr<three_way_random_access_iterator_const_int_ptr_>,
    ) -> Ptr<three_way_random_access_iterator_const_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = destructed_value_10.with(|_| ());
    let _ = moved_value_11.with(|_| ());
}
