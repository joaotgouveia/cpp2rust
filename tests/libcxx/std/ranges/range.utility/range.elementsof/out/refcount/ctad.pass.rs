extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_3: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_byte_ {}
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
    pub static destructed_value_4: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_5: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_6(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_7(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_6(_x, _y)
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
            operator_eq_6(
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
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_std_byte_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_std_byte_ {
    pub fn copy_from(a: Ptr<test_allocator_std_byte_>) -> Self {
        let __this: Value<test_allocator_std_byte_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_std_byte_> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for test_allocator_std_byte_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_std_byte_> =
            Rc::new(RefCell::new(test_allocator_std_byte_ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            }));
        test_allocator_std_byte_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_std_byte_ {
    fn default() -> Self {
        {
            test_allocator_std_byte_ {
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
pub struct types_type_list_std_allocator_std_byte___min_allocator_std_byte___test_allocator_std_byte__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_input_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__
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
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr____types_type_list_cpp20_input_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr___
{}
pub fn for_each_8(
    mut _a0 : types_type_list_std_allocator_std_byte___min_allocator_std_byte___test_allocator_std_byte__,
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
        swallow_9(_arg0, _arg1, _arg2)
    });
}
pub fn for_each_10(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_input_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__,
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
        let _arg6: () = {
            ({ (*f.borrow()).spec::<fn()>(6).call() });
            0_()
        };
        swallow_11(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6)
    });
}
pub fn swallow_11(
    mut _a0: (),
    mut _a1: (),
    mut _a2: (),
    mut _a3: (),
    mut _a4: (),
    mut _a5: (),
    mut _a6: (),
) {
}
pub fn swallow_9(mut _a0: (), mut _a1: (), mut _a2: ()) {}
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct forward_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl forward_iterator_int_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct bidirectional_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_int_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptr_ {}
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_int_ptr_ {}
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
#[byte_size(8)]
pub struct sentinel_wrapper_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_int_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_contiguous_iterator_int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_contiguous_iterator_int_ptr__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_random_access_iterator_int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_random_access_iterator_int_ptr__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_bidirectional_iterator_int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_bidirectional_iterator_int_ptr__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_forward_iterator_int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_forward_iterator_int_ptr__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_cpp20_input_iterator_int_ptr__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ {}
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
    return (unimplemented!()).reinterpret_cast::<()>();
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
pub fn test_impl_19() {
    let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let r: Value<Range_int_ptr_> = Rc::new(RefCell::new(<Range_int_ptr_>::default()));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let r: Value<Range_int_ptr_> = Rc::new(RefCell::new(<Range_int_ptr_>::default()));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let r: Value<Range_int_ptr_> = Rc::new(RefCell::new(<Range_int_ptr_>::default()));
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub fn test_impl_20() {
    let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let r: Value<Range_contiguous_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_contiguous_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_contiguous_iterator_int_ptr__ =
            <Range_contiguous_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let r: Value<Range_contiguous_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_contiguous_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_contiguous_iterator_int_ptr__ =
            <Range_contiguous_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let r: Value<Range_contiguous_iterator_int_ptr__> = Rc::new(RefCell::new(
                <Range_contiguous_iterator_int_ptr__>::default(),
            ));
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_contiguous_iterator_int_ptr__ =
                <Range_contiguous_iterator_int_ptr__>::default();
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub fn test_impl_21() {
    let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let r: Value<Range_random_access_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_random_access_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_random_access_iterator_int_ptr__ =
            <Range_random_access_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let r: Value<Range_random_access_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_random_access_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_random_access_iterator_int_ptr__ =
            <Range_random_access_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let r: Value<Range_random_access_iterator_int_ptr__> = Rc::new(RefCell::new(
                <Range_random_access_iterator_int_ptr__>::default(),
            ));
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_random_access_iterator_int_ptr__ =
                <Range_random_access_iterator_int_ptr__>::default();
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub fn test_impl_22() {
    let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let r: Value<Range_bidirectional_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_bidirectional_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_bidirectional_iterator_int_ptr__ =
            <Range_bidirectional_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let r: Value<Range_bidirectional_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_bidirectional_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_bidirectional_iterator_int_ptr__ =
            <Range_bidirectional_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let r: Value<Range_bidirectional_iterator_int_ptr__> = Rc::new(RefCell::new(
                <Range_bidirectional_iterator_int_ptr__>::default(),
            ));
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_bidirectional_iterator_int_ptr__ =
                <Range_bidirectional_iterator_int_ptr__>::default();
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub fn test_impl_23() {
    let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let r: Value<Range_forward_iterator_int_ptr__> =
            Rc::new(RefCell::new(<Range_forward_iterator_int_ptr__>::default()));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_forward_iterator_int_ptr__ = <Range_forward_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let r: Value<Range_forward_iterator_int_ptr__> =
            Rc::new(RefCell::new(<Range_forward_iterator_int_ptr__>::default()));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_forward_iterator_int_ptr__ = <Range_forward_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let r: Value<Range_forward_iterator_int_ptr__> =
                Rc::new(RefCell::new(<Range_forward_iterator_int_ptr__>::default()));
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_forward_iterator_int_ptr__ =
                <Range_forward_iterator_int_ptr__>::default();
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub fn test_impl_24() {
    let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let r: Value<Range_cpp20_input_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_cpp20_input_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_cpp20_input_iterator_int_ptr__ =
            <Range_cpp20_input_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let r: Value<Range_cpp20_input_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_cpp20_input_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_cpp20_input_iterator_int_ptr__ =
            <Range_cpp20_input_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let r: Value<Range_cpp20_input_iterator_int_ptr__> = Rc::new(RefCell::new(
                <Range_cpp20_input_iterator_int_ptr__>::default(),
            ));
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_cpp20_input_iterator_int_ptr__ =
                <Range_cpp20_input_iterator_int_ptr__>::default();
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub fn test_impl_25() {
    let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let r: Value<Range_cpp17_input_iterator_int_ptr__int_ptr__> = Rc::new(RefCell::new(
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let r: Value<Range_cpp17_input_iterator_int_ptr__int_ptr__> = Rc::new(RefCell::new(
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let r: Value<Range_cpp17_input_iterator_int_ptr__int_ptr__> = Rc::new(RefCell::new(
                <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default(),
            ));
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
                <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub fn test_impl_26() {
    let mut a: min_allocator_std_byte_ = <min_allocator_std_byte_>::default();
    {
        let r: Value<Range_int_ptr_> = Rc::new(RefCell::new(<Range_int_ptr_>::default()));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let r: Value<Range_int_ptr_> = Rc::new(RefCell::new(<Range_int_ptr_>::default()));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let r: Value<Range_int_ptr_> = Rc::new(RefCell::new(<Range_int_ptr_>::default()));
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub fn test_impl_27() {
    let mut a: min_allocator_std_byte_ = <min_allocator_std_byte_>::default();
    {
        let r: Value<Range_contiguous_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_contiguous_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_contiguous_iterator_int_ptr__ =
            <Range_contiguous_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let r: Value<Range_contiguous_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_contiguous_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_contiguous_iterator_int_ptr__ =
            <Range_contiguous_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let r: Value<Range_contiguous_iterator_int_ptr__> = Rc::new(RefCell::new(
                <Range_contiguous_iterator_int_ptr__>::default(),
            ));
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_contiguous_iterator_int_ptr__ =
                <Range_contiguous_iterator_int_ptr__>::default();
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub fn test_impl_28() {
    let mut a: min_allocator_std_byte_ = <min_allocator_std_byte_>::default();
    {
        let r: Value<Range_random_access_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_random_access_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_random_access_iterator_int_ptr__ =
            <Range_random_access_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let r: Value<Range_random_access_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_random_access_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_random_access_iterator_int_ptr__ =
            <Range_random_access_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let r: Value<Range_random_access_iterator_int_ptr__> = Rc::new(RefCell::new(
                <Range_random_access_iterator_int_ptr__>::default(),
            ));
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_random_access_iterator_int_ptr__ =
                <Range_random_access_iterator_int_ptr__>::default();
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub fn test_impl_29() {
    let mut a: min_allocator_std_byte_ = <min_allocator_std_byte_>::default();
    {
        let r: Value<Range_bidirectional_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_bidirectional_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_bidirectional_iterator_int_ptr__ =
            <Range_bidirectional_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let r: Value<Range_bidirectional_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_bidirectional_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_bidirectional_iterator_int_ptr__ =
            <Range_bidirectional_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let r: Value<Range_bidirectional_iterator_int_ptr__> = Rc::new(RefCell::new(
                <Range_bidirectional_iterator_int_ptr__>::default(),
            ));
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_bidirectional_iterator_int_ptr__ =
                <Range_bidirectional_iterator_int_ptr__>::default();
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub fn test_impl_30() {
    let mut a: min_allocator_std_byte_ = <min_allocator_std_byte_>::default();
    {
        let r: Value<Range_forward_iterator_int_ptr__> =
            Rc::new(RefCell::new(<Range_forward_iterator_int_ptr__>::default()));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_forward_iterator_int_ptr__ = <Range_forward_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let r: Value<Range_forward_iterator_int_ptr__> =
            Rc::new(RefCell::new(<Range_forward_iterator_int_ptr__>::default()));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_forward_iterator_int_ptr__ = <Range_forward_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let r: Value<Range_forward_iterator_int_ptr__> =
                Rc::new(RefCell::new(<Range_forward_iterator_int_ptr__>::default()));
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_forward_iterator_int_ptr__ =
                <Range_forward_iterator_int_ptr__>::default();
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub fn test_impl_31() {
    let mut a: min_allocator_std_byte_ = <min_allocator_std_byte_>::default();
    {
        let r: Value<Range_cpp20_input_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_cpp20_input_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_cpp20_input_iterator_int_ptr__ =
            <Range_cpp20_input_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let r: Value<Range_cpp20_input_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_cpp20_input_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_cpp20_input_iterator_int_ptr__ =
            <Range_cpp20_input_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let r: Value<Range_cpp20_input_iterator_int_ptr__> = Rc::new(RefCell::new(
                <Range_cpp20_input_iterator_int_ptr__>::default(),
            ));
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_cpp20_input_iterator_int_ptr__ =
                <Range_cpp20_input_iterator_int_ptr__>::default();
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub fn test_impl_32() {
    let mut a: min_allocator_std_byte_ = <min_allocator_std_byte_>::default();
    {
        let r: Value<Range_cpp17_input_iterator_int_ptr__int_ptr__> = Rc::new(RefCell::new(
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let r: Value<Range_cpp17_input_iterator_int_ptr__int_ptr__> = Rc::new(RefCell::new(
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let r: Value<Range_cpp17_input_iterator_int_ptr__int_ptr__> = Rc::new(RefCell::new(
                <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default(),
            ));
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
                <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub fn test_impl_33() {
    let a: Value<test_allocator_std_byte_> =
        Rc::new(RefCell::new(<test_allocator_std_byte_>::default()));
    let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
    {
        let r: Value<Range_int_ptr_> = Rc::new(RefCell::new(<Range_int_ptr_>::default()));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let r: Value<Range_int_ptr_> = Rc::new(RefCell::new(<Range_int_ptr_>::default()));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let r: Value<Range_int_ptr_> = Rc::new(RefCell::new(<Range_int_ptr_>::default()));
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub fn test_impl_34() {
    let a: Value<test_allocator_std_byte_> =
        Rc::new(RefCell::new(<test_allocator_std_byte_>::default()));
    let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
    {
        let r: Value<Range_contiguous_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_contiguous_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_contiguous_iterator_int_ptr__ =
            <Range_contiguous_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let r: Value<Range_contiguous_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_contiguous_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_contiguous_iterator_int_ptr__ =
            <Range_contiguous_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let r: Value<Range_contiguous_iterator_int_ptr__> = Rc::new(RefCell::new(
                <Range_contiguous_iterator_int_ptr__>::default(),
            ));
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_contiguous_iterator_int_ptr__ =
                <Range_contiguous_iterator_int_ptr__>::default();
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub fn test_impl_35() {
    let a: Value<test_allocator_std_byte_> =
        Rc::new(RefCell::new(<test_allocator_std_byte_>::default()));
    let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
    {
        let r: Value<Range_random_access_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_random_access_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_random_access_iterator_int_ptr__ =
            <Range_random_access_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let r: Value<Range_random_access_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_random_access_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_random_access_iterator_int_ptr__ =
            <Range_random_access_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let r: Value<Range_random_access_iterator_int_ptr__> = Rc::new(RefCell::new(
                <Range_random_access_iterator_int_ptr__>::default(),
            ));
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_random_access_iterator_int_ptr__ =
                <Range_random_access_iterator_int_ptr__>::default();
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub fn test_impl_36() {
    let a: Value<test_allocator_std_byte_> =
        Rc::new(RefCell::new(<test_allocator_std_byte_>::default()));
    let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
    {
        let r: Value<Range_bidirectional_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_bidirectional_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_bidirectional_iterator_int_ptr__ =
            <Range_bidirectional_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let r: Value<Range_bidirectional_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_bidirectional_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_bidirectional_iterator_int_ptr__ =
            <Range_bidirectional_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let r: Value<Range_bidirectional_iterator_int_ptr__> = Rc::new(RefCell::new(
                <Range_bidirectional_iterator_int_ptr__>::default(),
            ));
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_bidirectional_iterator_int_ptr__ =
                <Range_bidirectional_iterator_int_ptr__>::default();
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub fn test_impl_37() {
    let a: Value<test_allocator_std_byte_> =
        Rc::new(RefCell::new(<test_allocator_std_byte_>::default()));
    let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
    {
        let r: Value<Range_forward_iterator_int_ptr__> =
            Rc::new(RefCell::new(<Range_forward_iterator_int_ptr__>::default()));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_forward_iterator_int_ptr__ = <Range_forward_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let r: Value<Range_forward_iterator_int_ptr__> =
            Rc::new(RefCell::new(<Range_forward_iterator_int_ptr__>::default()));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_forward_iterator_int_ptr__ = <Range_forward_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let r: Value<Range_forward_iterator_int_ptr__> =
                Rc::new(RefCell::new(<Range_forward_iterator_int_ptr__>::default()));
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_forward_iterator_int_ptr__ =
                <Range_forward_iterator_int_ptr__>::default();
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub fn test_impl_38() {
    let a: Value<test_allocator_std_byte_> =
        Rc::new(RefCell::new(<test_allocator_std_byte_>::default()));
    let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
    {
        let r: Value<Range_cpp20_input_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_cpp20_input_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_cpp20_input_iterator_int_ptr__ =
            <Range_cpp20_input_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let r: Value<Range_cpp20_input_iterator_int_ptr__> = Rc::new(RefCell::new(
            <Range_cpp20_input_iterator_int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_cpp20_input_iterator_int_ptr__ =
            <Range_cpp20_input_iterator_int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let r: Value<Range_cpp20_input_iterator_int_ptr__> = Rc::new(RefCell::new(
                <Range_cpp20_input_iterator_int_ptr__>::default(),
            ));
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_cpp20_input_iterator_int_ptr__ =
                <Range_cpp20_input_iterator_int_ptr__>::default();
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub fn test_impl_39() {
    let a: Value<test_allocator_std_byte_> =
        Rc::new(RefCell::new(<test_allocator_std_byte_>::default()));
    let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
    {
        let r: Value<Range_cpp17_input_iterator_int_ptr__int_ptr__> = Rc::new(RefCell::new(
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let r: Value<Range_cpp17_input_iterator_int_ptr__int_ptr__> = Rc::new(RefCell::new(
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default(),
        ));
        let elements: Value<()> = Rc::new(RefCell::new((*r.borrow()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
        let elements: Value<()> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let r: Value<Range_cpp17_input_iterator_int_ptr__int_ptr__> = Rc::new(RefCell::new(
                <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default(),
            ));
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
                <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
            let elements: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Range_int_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Range_contiguous_iterator_int_ptr__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Range_random_access_iterator_int_ptr__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Range_bidirectional_iterator_int_ptr__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Range_forward_iterator_int_ptr__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Range_cpp20_input_iterator_int_ptr__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Range_cpp17_input_iterator_int_ptr__int_ptr__ {}
pub fn test_40() -> () {
    ({
        for_each_8 ( <types_type_list_std_allocator_std_byte___min_allocator_std_byte___test_allocator_std_byte__  >::default()  , lambda! ( Generic , { } , | | { ( { for_each_10 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_input_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__  >::default()  , lambda! ( Generic , { } , | | { ( { test_impl_19 ( ) } )  ;
 } , | | { ( { test_impl_20 ( ) } )  ;
 } , | | { ( { test_impl_21 ( ) } )  ;
 } , | | { ( { test_impl_22 ( ) } )  ;
 } , | | { ( { test_impl_23 ( ) } )  ;
 } , | | { ( { test_impl_24 ( ) } )  ;
 } , | | { ( { test_impl_25 ( ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_41 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_input_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__  >::default()  , lambda! ( Generic , { } , | | { ( { test_impl_26 ( ) } )  ;
 } , | | { ( { test_impl_27 ( ) } )  ;
 } , | | { ( { test_impl_28 ( ) } )  ;
 } , | | { ( { test_impl_29 ( ) } )  ;
 } , | | { ( { test_impl_30 ( ) } )  ;
 } , | | { ( { test_impl_31 ( ) } )  ;
 } , | | { ( { test_impl_32 ( ) } )  ;
 } )  , ) } )  ;
 } , | | { ( { for_each_42 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_input_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__  >::default()  , lambda! ( Generic , { } , | | { ( { test_impl_33 ( ) } )  ;
 } , | | { ( { test_impl_34 ( ) } )  ;
 } , | | { ( { test_impl_35 ( ) } )  ;
 } , | | { ( { test_impl_36 ( ) } )  ;
 } , | | { ( { test_impl_37 ( ) } )  ;
 } , | | { ( { test_impl_38 ( ) } )  ;
 } , | | { ( { test_impl_39 ( ) } )  ;
 } )  , ) } )  ;
 } )  , )
    });
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
    ({ test_40() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_std_byte__std_integral_constant_unsigned_long__0__;
pub trait Range_bidirectional_iterator_int_ptr__Impl {
    fn begin(&self) -> bidirectional_iterator_int_ptr_ {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_bidirectional_iterator_int_ptr__ {
        unimplemented!()
    }
}
impl Range_bidirectional_iterator_int_ptr__Impl for Ptr<Range_bidirectional_iterator_int_ptr__> {}
pub trait Range_contiguous_iterator_int_ptr__Impl {
    fn begin(&self) -> contiguous_iterator_int_ptr_ {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_contiguous_iterator_int_ptr__ {
        unimplemented!()
    }
}
impl Range_contiguous_iterator_int_ptr__Impl for Ptr<Range_contiguous_iterator_int_ptr__> {}
pub trait Range_cpp17_input_iterator_int_ptr__int_ptr__Impl {
    fn begin(&self) -> cpp17_input_iterator_int_ptr__int_ptr_ {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ {
        unimplemented!()
    }
}
impl Range_cpp17_input_iterator_int_ptr__int_ptr__Impl
    for Ptr<Range_cpp17_input_iterator_int_ptr__int_ptr__>
{
}
pub trait Range_cpp20_input_iterator_int_ptr__Impl {
    fn begin(&self) -> cpp20_input_iterator_int_ptr_ {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
        unimplemented!()
    }
}
impl Range_cpp20_input_iterator_int_ptr__Impl for Ptr<Range_cpp20_input_iterator_int_ptr__> {}
pub trait Range_forward_iterator_int_ptr__Impl {
    fn begin(&self) -> forward_iterator_int_ptr_ {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_forward_iterator_int_ptr__ {
        unimplemented!()
    }
}
impl Range_forward_iterator_int_ptr__Impl for Ptr<Range_forward_iterator_int_ptr__> {}
pub trait Range_int_ptr_Impl {
    fn begin(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_int_ptr_ {
        unimplemented!()
    }
}
impl Range_int_ptr_Impl for Ptr<Range_int_ptr_> {}
pub trait Range_random_access_iterator_int_ptr__Impl {
    fn begin(&self) -> random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn end(&self) -> sentinel_wrapper_random_access_iterator_int_ptr__ {
        unimplemented!()
    }
}
impl Range_random_access_iterator_int_ptr__Impl for Ptr<Range_random_access_iterator_int_ptr__> {}
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
pub trait bidirectional_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_int_ptr_> {
        unimplemented!()
    }
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
impl bidirectional_iterator_int_ptr_Impl for Ptr<bidirectional_iterator_int_ptr_> {}
pub trait contiguous_iterator_int_ptr_Impl {
    fn base(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_arrow(&self) -> () {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_int_ptr_> {
        unimplemented!()
    }
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
impl contiguous_iterator_int_ptr_Impl for Ptr<contiguous_iterator_int_ptr_> {}
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
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<forward_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_7(&self, mut _a0: ()) -> forward_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl forward_iterator_int_ptr_Impl for Ptr<forward_iterator_int_ptr_> {}
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
pub trait min_allocator_std_byte_Impl {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_std_byte__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_std_byte__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_byte_Impl for Ptr<min_allocator_std_byte_> {}
pub trait random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptr_> {
        unimplemented!()
    }
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
impl random_access_iterator_int_ptr_Impl for Ptr<random_access_iterator_int_ptr_> {}
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
pub trait sentinel_wrapper_bidirectional_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<bidirectional_iterator_int_ptr_>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_bidirectional_iterator_int_ptr__Impl
    for Ptr<sentinel_wrapper_bidirectional_iterator_int_ptr__>
{
}
pub trait sentinel_wrapper_contiguous_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<contiguous_iterator_int_ptr_>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_contiguous_iterator_int_ptr__Impl
    for Ptr<sentinel_wrapper_contiguous_iterator_int_ptr__>
{
}
pub trait sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__Impl
    for Ptr<sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__>
{
}
pub trait sentinel_wrapper_cpp20_input_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<cpp20_input_iterator_int_ptr_>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_cpp20_input_iterator_int_ptr__Impl
    for Ptr<sentinel_wrapper_cpp20_input_iterator_int_ptr__>
{
}
pub trait sentinel_wrapper_forward_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<forward_iterator_int_ptr_>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_forward_iterator_int_ptr__Impl
    for Ptr<sentinel_wrapper_forward_iterator_int_ptr__>
{
}
pub trait sentinel_wrapper_int_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_int_ptr_Impl for Ptr<sentinel_wrapper_int_ptr_> {}
pub trait sentinel_wrapper_random_access_iterator_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<random_access_iterator_int_ptr_>) -> () {
        unimplemented!()
    }
}
impl sentinel_wrapper_random_access_iterator_int_ptr__Impl
    for Ptr<sentinel_wrapper_random_access_iterator_int_ptr__>
{
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
pub trait test_allocator_std_byte_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<()>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_std_byte_Impl for Ptr<test_allocator_std_byte_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_4.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_4.with(|rc| *rc.borrow()));
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
        field!((*self), data_).write(destructed_value_4.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_4.with(|rc| *rc.borrow()));
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
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
    let _ = destructed_value_4.with(|_| ());
    let _ = moved_value_5.with(|_| ());
}
