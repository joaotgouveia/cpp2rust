extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_();
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_byte_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_allocator_statistics {
    pub time_to_throw: (),
    pub throw_after: (),
    pub count: (),
    pub alloc_count: (),
    pub allocated_size: (),
    pub construct_count: (),
    pub destroy_count: (),
    pub copied: (),
    pub moved: (),
    pub converted: (),
}
impl test_allocator_statistics {
    pub unsafe fn clear(&mut self) {
        unimplemented!();
        self.count = 0_();
        self.time_to_throw = 0_();
        self.alloc_count = 0_();
        self.allocated_size = 0_();
        self.construct_count = 0_();
        self.destroy_count = 0_();
        self.throw_after = 2147483647_();
        (unsafe { test_allocator_statistics::clear_ctor_counters(self) });
    }
    pub unsafe fn clear_ctor_counters(&mut self) {
        self.copied = 0_();
        self.moved = 0_();
        self.converted = 0_();
    }
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        unsafe {
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
pub static mut destructed_value_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_5: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_6(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_7(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_6(_x, _y)
    });
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct test_allocator_void_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_void_ {
    pub unsafe fn new_1(mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: 0_(),
            id_: 0_(),
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_2(mut data: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_3(mut data: (), mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_4(mut data: (), mut id: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_5(
        mut data: (),
        mut id: (),
        mut stats: *mut test_allocator_statistics,
    ) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: stats,
        };
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_void_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4));
    }
    pub unsafe fn get_id(&self) -> () {
        return self.id_;
    }
    pub unsafe fn get_data(&self) -> () {
        return self.data_;
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const test_allocator_void_,
    ) -> *mut test_allocator_void_ {
        self.data_ = (*_a0).data_;
        self.id_ = (*_a0).id_;
        self.stats_ = (*_a0).stats_;
        return &mut (*(self as *mut test_allocator_void_));
    }
}
impl std::cmp::PartialEq for test_allocator_void_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_6(
                self as *const test_allocator_void_,
                other as *const test_allocator_void_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_void_ {}
impl Clone for test_allocator_void_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_void_::copy_from(self as *const test_allocator_void_) }
    }
}
impl Default for test_allocator_void_ {
    fn default() -> Self {
        unsafe {
            test_allocator_void_ {
                data_: 0_(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_std_byte_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std_byte_ {
    pub unsafe fn copy_from(a: *const test_allocator_std_byte_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        unimplemented!();
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).copied.prefix_inc();
        }
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4));
    }
}
impl Clone for test_allocator_std_byte_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_std_byte_::copy_from(self as *const test_allocator_std_byte_) }
    }
}
impl Default for test_allocator_std_byte_ {
    fn default() -> Self {
        unsafe {
            test_allocator_std_byte_ {
                data_: 0_(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Ctor_Tag {}
#[repr(C)]
#[derive(FnPtrArg, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
pub struct Tag_X {}
impl Tag_X {
    pub unsafe fn new_1(mut _a0: Ctor_Tag) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_2(mut _a0: Ctor_Tag, _a1: *const Tag_X) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_3(mut _a0: Ctor_Tag, _a1: *mut Tag_X) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Tag_X) -> *mut Tag_X {
        return &mut (*(self as *mut Tag_X));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Tag_X) -> *mut Tag_X {
        return &mut (*(self as *mut Tag_X));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct support_double_move_tracker {
    moved_from_: (),
}
impl support_double_move_tracker {
    pub unsafe fn new() -> Self {
        let mut this = Self { moved_from_: false };
        this
    }
    pub unsafe fn move_from(other: *mut support_double_move_tracker) -> Self {
        let mut this = Self { moved_from_: false };
        unimplemented!();
        (*other).moved_from_ = true;
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        other: *mut support_double_move_tracker,
    ) -> *mut support_double_move_tracker {
        unimplemented!();
        (*other).moved_from_ = true;
        self.moved_from_ = false;
        return &mut (*(self as *mut support_double_move_tracker));
    }
}
impl Default for support_double_move_tracker {
    fn default() -> Self {
        unsafe { support_double_move_tracker::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_std_allocator_std_byte___min_allocator_std_byte___test_allocator_std_byte__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_input_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr____types_type_list_bidirectional_iterator_int_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr____types_type_list_forward_iterator_int_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr____types_type_list_cpp20_input_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr___
{}
pub unsafe fn for_each_8(
    mut _a0 : types_type_list_std_allocator_std_byte___min_allocator_std_byte___test_allocator_std_byte__,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            (unsafe { f.spec::<fn()>(2).call() });
            0_()
        };
        swallow_9(_arg0, _arg1, _arg2)
    });
}
pub unsafe fn for_each_10(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_input_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            (unsafe { f.spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            (unsafe { f.spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            (unsafe { f.spec::<fn()>(4).call() });
            0_()
        };
        let _arg5: () = {
            (unsafe { f.spec::<fn()>(5).call() });
            0_()
        };
        let _arg6: () = {
            (unsafe { f.spec::<fn()>(6).call() });
            0_()
        };
        swallow_11(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6)
    });
}
pub unsafe fn swallow_11(
    mut _a0: (),
    mut _a1: (),
    mut _a2: (),
    mut _a3: (),
    mut _a4: (),
    mut _a5: (),
    mut _a6: (),
) {
}
pub unsafe fn swallow_9(mut _a0: (), mut _a1: (), mut _a2: ()) {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp17_output_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct forward_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_int_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct bidirectional_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_int_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct random_access_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_random_access_iterator_int_ptr_ {
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.it_.offset((n) as isize));
    }
}
const _: () = assert!(
    true,
    "std::random_access_iterator<cpp20_random_access_iterator<int*>>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct contiguous_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_int_ptr_ {}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_input_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp20_input_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_input_iterator_int_ptr_,
    ) -> *mut cpp20_input_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_input_iterator_int_ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
}
const _: () = assert!(true, "std::input_iterator<cpp20_input_iterator<int*>>");
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_output_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_output_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp20_output_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_output_iterator_int_ptr_,
    ) -> *mut cpp20_output_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_output_iterator_int_ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp20_output_iterator<int*>, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct IteratorOpCounts {
    pub increments: (),
    pub decrements: (),
    pub zero_moves: (),
    pub equal_cmps: (),
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        unsafe {
            IteratorOpCounts {
                increments: 0_(),
                decrements: 0_(),
                zero_moves: 0_(),
                equal_cmps: 0_(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_int_ptr_ {
    base_: *mut (),
}
impl sentinel_wrapper_int_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_contiguous_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_contiguous_iterator_int_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_random_access_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_random_access_iterator_int_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_bidirectional_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_bidirectional_iterator_int_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_forward_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_forward_iterator_int_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_cpp20_input_iterator_int_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ {}
pub unsafe fn operator_eq_12(
    _a0: *const forward_sized_iterator_int_ptr_,
    _a1: *const forward_sized_iterator_int_ptr_,
) -> () {
    return (((*_a0).it_) == ((*_a1).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct forward_sized_iterator_int_ptr_ {
    pub it_: *mut (),
}
impl forward_sized_iterator_int_ptr_ {}
impl std::cmp::PartialEq for forward_sized_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_12(
                self as *const forward_sized_iterator_int_ptr_,
                other as *const forward_sized_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_sized_iterator_int_ptr_ {}
impl Default for forward_sized_iterator_int_ptr_ {
    fn default() -> Self {
        unsafe {
            forward_sized_iterator_int_ptr_ {
                it_: std::ptr::null_mut(),
            }
        }
    }
}
const _: () = assert!(true, "std::forward_iterator<forward_sized_iterator<>>");
const _: () = assert!(
    true,
    "std::sized_sentinel_for<forward_sized_iterator<>, forward_sized_iterator<>>"
);
pub unsafe fn operator_add_13(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_14(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_13(_i, n)
    });
}
pub unsafe fn iter_swap_15(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_16(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*unimplemented!());
}
pub unsafe fn operator_eq_17(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_18(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return std::cmp::Ord::cmp(&((*lhs).ptr_), &((*rhs).ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct adl_Iterator {
    ptr_: *mut (),
    iter_moves_: *mut (),
    iter_swaps_: *mut (),
}
impl adl_Iterator {
    unsafe fn new_1(mut p: *mut (), mut iter_moves: *mut (), mut iter_swaps: *mut ()) -> Self {
        let mut this = Self {
            ptr_: p,
            iter_moves_: iter_moves,
            iter_swaps_: iter_swaps,
        };
        this
    }
    pub unsafe fn TrackMoves(mut p: *mut (), iter_moves: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { (iter_moves) }, { std::ptr::null_mut() });
    }
    pub unsafe fn TrackSwaps_4(iter_swaps: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ std::ptr::null_mut() }, { std::ptr::null_mut() }, {
            (iter_swaps)
        });
    }
    pub unsafe fn TrackSwaps_5(mut p: *mut (), iter_swaps: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { std::ptr::null_mut() }, { (iter_swaps) });
    }
    pub unsafe fn iter_moves(&self) -> () {
        unimplemented!();
        return (*self.iter_moves_);
    }
    pub unsafe fn iter_swaps(&self) -> () {
        unimplemented!();
        return (*self.iter_swaps_);
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn operator_sub_10(&self, mut n: ()) -> adl_Iterator {
        return adl_Iterator::new_1(
            { self.ptr_.offset(-((n) as isize)) },
            { self.iter_moves_ },
            { self.iter_swaps_ },
        );
    }
    pub unsafe fn operator_sub_11(&self, mut rhs: adl_Iterator) -> () {
        return (((self.ptr_ as usize - rhs.ptr_ as usize) / ::std::mem::size_of::<()>()) as ()
            as ());
    }
    pub unsafe fn operator_add_assign(&mut self, mut n: ()) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_sub_assign(&mut self, mut n: ()) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_inc_14(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_inc_15(&mut self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_inc();
        return prev;
    }
    pub unsafe fn operator_dec_16(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_dec();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_dec_17(&mut self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_dec();
        return prev;
    }
}
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_18(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_17(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::Eq for adl_Iterator {}
impl Default for adl_Iterator {
    fn default() -> Self {
        unsafe {
            adl_Iterator {
                ptr_: std::ptr::null_mut(),
                iter_moves_: std::ptr::null_mut(),
                iter_swaps_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct rvalue_iterator_int_ptr_ {
    it_: *mut *mut (),
}
impl rvalue_iterator_int_ptr_ {
    pub unsafe fn operator_cmp(&self, _a0: *const rvalue_iterator_int_ptr_) -> () {
        {
            let mut cmp: () = std::cmp::Ord::cmp(
                &((*(self as *const rvalue_iterator_int_ptr_)).it_),
                &((*_a0).it_),
            );
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    pub unsafe fn operator_eq(&self, _a0: *const rvalue_iterator_int_ptr_) -> () {
        return (((*(self as *const rvalue_iterator_int_ptr_)).it_) == ((*_a0).it_));
    }
}
impl std::cmp::Ord for rvalue_iterator_int_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            rvalue_iterator_int_ptr_::operator_cmp(self, other as *const rvalue_iterator_int_ptr_)
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
        unsafe {
            rvalue_iterator_int_ptr_::operator_cmp(self, other as *const rvalue_iterator_int_ptr_)
                == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for rvalue_iterator_int_ptr_ {}
const _: () = assert!(true, "std::random_access_iterator<rvalue_iterator<int*>>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub unsafe fn new(mut what: *const ()) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn test_impl_19() {
    let mut a: () = unimplemented!();
    {
        let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub unsafe fn test_impl_20() {
    let mut a: () = unimplemented!();
    {
        let mut r: Range_contiguous_iterator_int_ptr__ =
            <Range_contiguous_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_contiguous_iterator_int_ptr__ =
            <Range_contiguous_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_contiguous_iterator_int_ptr__ =
            <Range_contiguous_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_contiguous_iterator_int_ptr__ =
            <Range_contiguous_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let mut r: Range_contiguous_iterator_int_ptr__ =
                <Range_contiguous_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_contiguous_iterator_int_ptr__ =
                <Range_contiguous_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub unsafe fn test_impl_21() {
    let mut a: () = unimplemented!();
    {
        let mut r: Range_random_access_iterator_int_ptr__ =
            <Range_random_access_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_random_access_iterator_int_ptr__ =
            <Range_random_access_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_random_access_iterator_int_ptr__ =
            <Range_random_access_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_random_access_iterator_int_ptr__ =
            <Range_random_access_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let mut r: Range_random_access_iterator_int_ptr__ =
                <Range_random_access_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_random_access_iterator_int_ptr__ =
                <Range_random_access_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub unsafe fn test_impl_22() {
    let mut a: () = unimplemented!();
    {
        let mut r: Range_bidirectional_iterator_int_ptr__ =
            <Range_bidirectional_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_bidirectional_iterator_int_ptr__ =
            <Range_bidirectional_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_bidirectional_iterator_int_ptr__ =
            <Range_bidirectional_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_bidirectional_iterator_int_ptr__ =
            <Range_bidirectional_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let mut r: Range_bidirectional_iterator_int_ptr__ =
                <Range_bidirectional_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_bidirectional_iterator_int_ptr__ =
                <Range_bidirectional_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub unsafe fn test_impl_23() {
    let mut a: () = unimplemented!();
    {
        let mut r: Range_forward_iterator_int_ptr__ = <Range_forward_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_forward_iterator_int_ptr__ = <Range_forward_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_forward_iterator_int_ptr__ = <Range_forward_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_forward_iterator_int_ptr__ = <Range_forward_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let mut r: Range_forward_iterator_int_ptr__ =
                <Range_forward_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_forward_iterator_int_ptr__ =
                <Range_forward_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub unsafe fn test_impl_24() {
    let mut a: () = unimplemented!();
    {
        let mut r: Range_cpp20_input_iterator_int_ptr__ =
            <Range_cpp20_input_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_cpp20_input_iterator_int_ptr__ =
            <Range_cpp20_input_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_cpp20_input_iterator_int_ptr__ =
            <Range_cpp20_input_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_cpp20_input_iterator_int_ptr__ =
            <Range_cpp20_input_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let mut r: Range_cpp20_input_iterator_int_ptr__ =
                <Range_cpp20_input_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_cpp20_input_iterator_int_ptr__ =
                <Range_cpp20_input_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub unsafe fn test_impl_25() {
    let mut a: () = unimplemented!();
    {
        let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
                <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
                <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub unsafe fn test_impl_26() {
    let mut a: min_allocator_std_byte_ = <min_allocator_std_byte_>::default();
    {
        let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub unsafe fn test_impl_27() {
    let mut a: min_allocator_std_byte_ = <min_allocator_std_byte_>::default();
    {
        let mut r: Range_contiguous_iterator_int_ptr__ =
            <Range_contiguous_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_contiguous_iterator_int_ptr__ =
            <Range_contiguous_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_contiguous_iterator_int_ptr__ =
            <Range_contiguous_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_contiguous_iterator_int_ptr__ =
            <Range_contiguous_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let mut r: Range_contiguous_iterator_int_ptr__ =
                <Range_contiguous_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_contiguous_iterator_int_ptr__ =
                <Range_contiguous_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub unsafe fn test_impl_28() {
    let mut a: min_allocator_std_byte_ = <min_allocator_std_byte_>::default();
    {
        let mut r: Range_random_access_iterator_int_ptr__ =
            <Range_random_access_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_random_access_iterator_int_ptr__ =
            <Range_random_access_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_random_access_iterator_int_ptr__ =
            <Range_random_access_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_random_access_iterator_int_ptr__ =
            <Range_random_access_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let mut r: Range_random_access_iterator_int_ptr__ =
                <Range_random_access_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_random_access_iterator_int_ptr__ =
                <Range_random_access_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub unsafe fn test_impl_29() {
    let mut a: min_allocator_std_byte_ = <min_allocator_std_byte_>::default();
    {
        let mut r: Range_bidirectional_iterator_int_ptr__ =
            <Range_bidirectional_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_bidirectional_iterator_int_ptr__ =
            <Range_bidirectional_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_bidirectional_iterator_int_ptr__ =
            <Range_bidirectional_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_bidirectional_iterator_int_ptr__ =
            <Range_bidirectional_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let mut r: Range_bidirectional_iterator_int_ptr__ =
                <Range_bidirectional_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_bidirectional_iterator_int_ptr__ =
                <Range_bidirectional_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub unsafe fn test_impl_30() {
    let mut a: min_allocator_std_byte_ = <min_allocator_std_byte_>::default();
    {
        let mut r: Range_forward_iterator_int_ptr__ = <Range_forward_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_forward_iterator_int_ptr__ = <Range_forward_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_forward_iterator_int_ptr__ = <Range_forward_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_forward_iterator_int_ptr__ = <Range_forward_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let mut r: Range_forward_iterator_int_ptr__ =
                <Range_forward_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_forward_iterator_int_ptr__ =
                <Range_forward_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub unsafe fn test_impl_31() {
    let mut a: min_allocator_std_byte_ = <min_allocator_std_byte_>::default();
    {
        let mut r: Range_cpp20_input_iterator_int_ptr__ =
            <Range_cpp20_input_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_cpp20_input_iterator_int_ptr__ =
            <Range_cpp20_input_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_cpp20_input_iterator_int_ptr__ =
            <Range_cpp20_input_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_cpp20_input_iterator_int_ptr__ =
            <Range_cpp20_input_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let mut r: Range_cpp20_input_iterator_int_ptr__ =
                <Range_cpp20_input_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_cpp20_input_iterator_int_ptr__ =
                <Range_cpp20_input_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub unsafe fn test_impl_32() {
    let mut a: min_allocator_std_byte_ = <min_allocator_std_byte_>::default();
    {
        let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
                <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
                <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub unsafe fn test_impl_33() {
    let mut a: test_allocator_std_byte_ = <test_allocator_std_byte_>::default();
    let _dtor_a = ScopedDestructorUnsafe::new(&raw mut a, test_allocator_std_byte_::destructor);
    {
        let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_int_ptr_ = <Range_int_ptr_>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub unsafe fn test_impl_34() {
    let mut a: test_allocator_std_byte_ = <test_allocator_std_byte_>::default();
    let _dtor_a = ScopedDestructorUnsafe::new(&raw mut a, test_allocator_std_byte_::destructor);
    {
        let mut r: Range_contiguous_iterator_int_ptr__ =
            <Range_contiguous_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_contiguous_iterator_int_ptr__ =
            <Range_contiguous_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_contiguous_iterator_int_ptr__ =
            <Range_contiguous_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_contiguous_iterator_int_ptr__ =
            <Range_contiguous_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let mut r: Range_contiguous_iterator_int_ptr__ =
                <Range_contiguous_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_contiguous_iterator_int_ptr__ =
                <Range_contiguous_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub unsafe fn test_impl_35() {
    let mut a: test_allocator_std_byte_ = <test_allocator_std_byte_>::default();
    let _dtor_a = ScopedDestructorUnsafe::new(&raw mut a, test_allocator_std_byte_::destructor);
    {
        let mut r: Range_random_access_iterator_int_ptr__ =
            <Range_random_access_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_random_access_iterator_int_ptr__ =
            <Range_random_access_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_random_access_iterator_int_ptr__ =
            <Range_random_access_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_random_access_iterator_int_ptr__ =
            <Range_random_access_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let mut r: Range_random_access_iterator_int_ptr__ =
                <Range_random_access_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_random_access_iterator_int_ptr__ =
                <Range_random_access_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub unsafe fn test_impl_36() {
    let mut a: test_allocator_std_byte_ = <test_allocator_std_byte_>::default();
    let _dtor_a = ScopedDestructorUnsafe::new(&raw mut a, test_allocator_std_byte_::destructor);
    {
        let mut r: Range_bidirectional_iterator_int_ptr__ =
            <Range_bidirectional_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_bidirectional_iterator_int_ptr__ =
            <Range_bidirectional_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_bidirectional_iterator_int_ptr__ =
            <Range_bidirectional_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_bidirectional_iterator_int_ptr__ =
            <Range_bidirectional_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let mut r: Range_bidirectional_iterator_int_ptr__ =
                <Range_bidirectional_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_bidirectional_iterator_int_ptr__ =
                <Range_bidirectional_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub unsafe fn test_impl_37() {
    let mut a: test_allocator_std_byte_ = <test_allocator_std_byte_>::default();
    let _dtor_a = ScopedDestructorUnsafe::new(&raw mut a, test_allocator_std_byte_::destructor);
    {
        let mut r: Range_forward_iterator_int_ptr__ = <Range_forward_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_forward_iterator_int_ptr__ = <Range_forward_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_forward_iterator_int_ptr__ = <Range_forward_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_forward_iterator_int_ptr__ = <Range_forward_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let mut r: Range_forward_iterator_int_ptr__ =
                <Range_forward_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_forward_iterator_int_ptr__ =
                <Range_forward_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub unsafe fn test_impl_38() {
    let mut a: test_allocator_std_byte_ = <test_allocator_std_byte_>::default();
    let _dtor_a = ScopedDestructorUnsafe::new(&raw mut a, test_allocator_std_byte_::destructor);
    {
        let mut r: Range_cpp20_input_iterator_int_ptr__ =
            <Range_cpp20_input_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_cpp20_input_iterator_int_ptr__ =
            <Range_cpp20_input_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_cpp20_input_iterator_int_ptr__ =
            <Range_cpp20_input_iterator_int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_cpp20_input_iterator_int_ptr__ =
            <Range_cpp20_input_iterator_int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let mut r: Range_cpp20_input_iterator_int_ptr__ =
                <Range_cpp20_input_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_cpp20_input_iterator_int_ptr__ =
                <Range_cpp20_input_iterator_int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
pub unsafe fn test_impl_39() {
    let mut a: test_allocator_std_byte_ = <test_allocator_std_byte_>::default();
    let _dtor_a = ScopedDestructorUnsafe::new(&raw mut a, test_allocator_std_byte_::destructor);
    {
        let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, std::allocator<std::byte>>>"
        );;
    }
    {
        let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
        let mut elements: () = r;
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
        );;
    }
    {
        let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
            <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
        let mut elements: () = (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
        );;
    }
    {
        {
            let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
                <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&, Allocator>>"
            );;
        }
        {
            let mut r: Range_cpp17_input_iterator_int_ptr__int_ptr__ =
                <Range_cpp17_input_iterator_int_ptr__int_ptr__>::default();
            let mut elements: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::same_as<decltype(elements), std::ranges::elements_of<Range&&, Allocator>>"
            );;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Range_int_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Range_contiguous_iterator_int_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Range_random_access_iterator_int_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Range_bidirectional_iterator_int_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Range_forward_iterator_int_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Range_cpp20_input_iterator_int_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Range_cpp17_input_iterator_int_ptr__int_ptr__ {}
pub unsafe fn test_40() -> () {
    (unsafe {
        for_each_8 ( <types_type_list_std_allocator_std_byte___min_allocator_std_byte___test_allocator_std_byte__ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { for_each_10 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_input_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_impl_19 ( ) } )  ;
 } , | | { ( unsafe { test_impl_20 ( ) } )  ;
 } , | | { ( unsafe { test_impl_21 ( ) } )  ;
 } , | | { ( unsafe { test_impl_22 ( ) } )  ;
 } , | | { ( unsafe { test_impl_23 ( ) } )  ;
 } , | | { ( unsafe { test_impl_24 ( ) } )  ;
 } , | | { ( unsafe { test_impl_25 ( ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_41 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_input_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_impl_26 ( ) } )  ;
 } , | | { ( unsafe { test_impl_27 ( ) } )  ;
 } , | | { ( unsafe { test_impl_28 ( ) } )  ;
 } , | | { ( unsafe { test_impl_29 ( ) } )  ;
 } , | | { ( unsafe { test_impl_30 ( ) } )  ;
 } , | | { ( unsafe { test_impl_31 ( ) } )  ;
 } , | | { ( unsafe { test_impl_32 ( ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_42 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_input_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_impl_33 ( ) } )  ;
 } , | | { ( unsafe { test_impl_34 ( ) } )  ;
 } , | | { ( unsafe { test_impl_35 ( ) } )  ;
 } , | | { ( unsafe { test_impl_36 ( ) } )  ;
 } , | | { ( unsafe { test_impl_37 ( ) } )  ;
 } , | | { ( unsafe { test_impl_38 ( ) } )  ;
 } , | | { ( unsafe { test_impl_39 ( ) } )  ;
 } ) , ) } )  ;
 } ) , )
    });
    return true;
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_40() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
    std::cell::LazyCell::force(&*&raw const destructed_value_4);
    std::cell::LazyCell::force(&*&raw const moved_value_5);
}
