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
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_usize;
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_1: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_2: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_3: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_byte_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_allocator_statistics {
    pub time_to_throw: i32,
    pub throw_after: i32,
    pub count: i32,
    pub alloc_count: i32,
    pub allocated_size: i32,
    pub construct_count: i32,
    pub destroy_count: i32,
    pub copied: i32,
    pub moved: i32,
    pub converted: i32,
}
impl test_allocator_statistics {
    pub unsafe fn clear(&mut self) {
        assert!(
            ((self.count) == (0)) && (!(c"clearing leaking allocator data?".as_ptr()).is_null())
        );
        self.count = 0;
        self.time_to_throw = 0;
        self.alloc_count = 0;
        self.allocated_size = 0;
        self.construct_count = 0;
        self.destroy_count = 0;
        self.throw_after = 2147483647;
        (unsafe { test_allocator_statistics::clear_ctor_counters(self) });
    }
    pub unsafe fn clear_ctor_counters(&mut self) {
        self.copied = 0;
        self.moved = 0;
        self.converted = 0;
    }
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        unsafe {
            test_allocator_statistics {
                time_to_throw: 0,
                throw_after: 2147483647,
                count: 0,
                alloc_count: 0,
                allocated_size: 0,
                construct_count: 0,
                destroy_count: 0,
                copied: 0,
                moved: 0,
                converted: 0,
            }
        }
    }
}
pub static mut destructed_value_4: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { -1_i32 });
pub static mut moved_value_5: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 2147483647 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_6(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_7(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_6(_x, _y)
    });
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct test_allocator_void_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_void_ {
    pub unsafe fn new_1(mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: 0,
            id_: 0,
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_2(mut data: i32) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_3(mut data: i32, mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_4(mut data: i32, mut id: i32) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_5(
        mut data: i32,
        mut id: i32,
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
    pub unsafe fn get_id(&self) -> i32 {
        return self.id_;
    }
    pub unsafe fn get_data(&self) -> i32 {
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
                data_: 0,
                id_: 0,
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_std_byte_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std_byte_ {
    pub unsafe fn copy_from(a: *const test_allocator_std_byte_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        assert!(
            ((((*a).data_)
                != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4)))
                && (((*a).id_)
                    != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4))))
                && (!(c"copying from destroyed allocator".as_ptr()).is_null())
        );
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).copied.prefix_inc();
        }
        this
    }
    pub unsafe fn destructor(&mut self) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4)))
        );
        assert!(
            ((self.id_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4)))
        );
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
                data_: 0,
                id_: 0,
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Ctor_Tag {}
#[repr(C)]
#[derive(FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
#[cpp_from(Ctor_Tag, new_1, value)]
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
    moved_from_: bool,
}
impl support_double_move_tracker {
    pub unsafe fn new() -> Self {
        let mut this = Self { moved_from_: false };
        this
    }
    pub unsafe fn move_from(other: *mut support_double_move_tracker) -> Self {
        let mut this = Self { moved_from_: false };
        assert!(!((*other).moved_from_));
        (*other).moved_from_ = true;
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        other: *mut support_double_move_tracker,
    ) -> *mut support_double_move_tracker {
        assert!(!((*other).moved_from_));
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
        let _arg0: i32 = {
            (unsafe { f.spec::<fn()>(0).call() });
            0
        };
        let _arg1: i32 = {
            (unsafe { f.spec::<fn()>(1).call() });
            0
        };
        let _arg2: i32 = {
            (unsafe { f.spec::<fn()>(2).call() });
            0
        };
        swallow_9(_arg0, _arg1, _arg2)
    });
}
pub unsafe fn for_each_10(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_input_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe { f.spec::<fn()>(0).call() });
            0
        };
        let _arg1: i32 = {
            (unsafe { f.spec::<fn()>(1).call() });
            0
        };
        let _arg2: i32 = {
            (unsafe { f.spec::<fn()>(2).call() });
            0
        };
        let _arg3: i32 = {
            (unsafe { f.spec::<fn()>(3).call() });
            0
        };
        let _arg4: i32 = {
            (unsafe { f.spec::<fn()>(4).call() });
            0
        };
        let _arg5: i32 = {
            (unsafe { f.spec::<fn()>(5).call() });
            0
        };
        let _arg6: i32 = {
            (unsafe { f.spec::<fn()>(6).call() });
            0
        };
        swallow_11(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6)
    });
}
pub unsafe fn for_each_12(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_input_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe { f.spec::<fn()>(0).call() });
            0
        };
        let _arg1: i32 = {
            (unsafe { f.spec::<fn()>(1).call() });
            0
        };
        let _arg2: i32 = {
            (unsafe { f.spec::<fn()>(2).call() });
            0
        };
        let _arg3: i32 = {
            (unsafe { f.spec::<fn()>(3).call() });
            0
        };
        let _arg4: i32 = {
            (unsafe { f.spec::<fn()>(4).call() });
            0
        };
        let _arg5: i32 = {
            (unsafe { f.spec::<fn()>(5).call() });
            0
        };
        let _arg6: i32 = {
            (unsafe { f.spec::<fn()>(6).call() });
            0
        };
        swallow_11(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6)
    });
}
pub unsafe fn for_each_13(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_input_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe { f.spec::<fn()>(0).call() });
            0
        };
        let _arg1: i32 = {
            (unsafe { f.spec::<fn()>(1).call() });
            0
        };
        let _arg2: i32 = {
            (unsafe { f.spec::<fn()>(2).call() });
            0
        };
        let _arg3: i32 = {
            (unsafe { f.spec::<fn()>(3).call() });
            0
        };
        let _arg4: i32 = {
            (unsafe { f.spec::<fn()>(4).call() });
            0
        };
        let _arg5: i32 = {
            (unsafe { f.spec::<fn()>(5).call() });
            0
        };
        let _arg6: i32 = {
            (unsafe { f.spec::<fn()>(6).call() });
            0
        };
        swallow_11(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6)
    });
}
pub unsafe fn swallow_11(
    mut _a0: i32,
    mut _a1: i32,
    mut _a2: i32,
    mut _a3: i32,
    mut _a4: i32,
    mut _a5: i32,
    mut _a6: i32,
) {
}
pub unsafe fn swallow_9(mut _a0: i32, mut _a1: i32, mut _a2: i32) {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_output_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp17_output_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp17_output_iterator_int_ptr_,
    ) -> *mut cpp17_output_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp17_output_iterator_int_ptr_));
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp17_input_iterator_int_ptr__int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp17_input_iterator_int_ptr__int_ptr_,
    ) -> *mut cpp17_input_iterator_int_ptr__int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp17_input_iterator_int_ptr__int_ptr_));
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl forward_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut forward_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut forward_iterator_int_ptr_,
    ) -> *mut forward_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut forward_iterator_int_ptr_));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct bidirectional_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut bidirectional_iterator_int_ptr_,
    ) -> *mut bidirectional_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut bidirectional_iterator_int_ptr_));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut random_access_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_int_ptr_,
    ) -> *mut random_access_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl cpp20_random_access_iterator_int_ptr_ {
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_index(&self, mut n: isize) -> *mut i32 {
        return &mut (*self.it_.offset((n) as isize));
    }
    pub unsafe fn move_from(_a0: *mut cpp20_random_access_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_random_access_iterator_int_ptr_,
    ) -> *mut cpp20_random_access_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_random_access_iterator_int_ptr_));
    }
}
const _: () = assert!(
    true,
    "std::random_access_iterator<cpp20_random_access_iterator<int*>>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct contiguous_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut contiguous_iterator_int_ptr_,
    ) -> *mut contiguous_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut contiguous_iterator_int_ptr_));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_input_iterator_int_ptr_ {
    it_: *mut i32,
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
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.it_);
    }
}
const _: () = assert!(true, "std::input_iterator<cpp20_input_iterator<int*>>");
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_output_iterator_int_ptr_ {
    it_: *mut i32,
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
    pub unsafe fn operator_deref(&self) -> *mut i32 {
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
    pub increments: usize,
    pub decrements: usize,
    pub zero_moves: usize,
    pub equal_cmps: usize,
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        unsafe {
            IteratorOpCounts {
                increments: 0_usize,
                decrements: 0_usize,
                zero_moves: 0_usize,
                equal_cmps: 0_usize,
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_int_ptr_ {
    base_: *mut i32,
}
impl sentinel_wrapper_int_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_contiguous_iterator_int_ptr__ {
    base_: *mut i32,
}
impl sentinel_wrapper_contiguous_iterator_int_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_random_access_iterator_int_ptr__ {
    base_: *mut i32,
}
impl sentinel_wrapper_random_access_iterator_int_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_bidirectional_iterator_int_ptr__ {
    base_: *mut i32,
}
impl sentinel_wrapper_bidirectional_iterator_int_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_forward_iterator_int_ptr__ {
    base_: *mut i32,
}
impl sentinel_wrapper_forward_iterator_int_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
    base_: *mut i32,
}
impl sentinel_wrapper_cpp20_input_iterator_int_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ {
    base_: *mut i32,
}
impl sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ {}
pub unsafe fn operator_eq_14(
    _a0: *const forward_sized_iterator_int_ptr_,
    _a1: *const forward_sized_iterator_int_ptr_,
) -> bool {
    return (((*_a0).it_) == ((*_a1).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct forward_sized_iterator_int_ptr_ {
    pub it_: *mut i32,
}
impl forward_sized_iterator_int_ptr_ {}
impl std::cmp::PartialEq for forward_sized_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_14(
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
pub unsafe fn operator_add_15(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_16(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_15(_i, n)
    });
}
pub unsafe fn iter_swap_17(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap_unsafe!(a.ptr_, b.ptr_);
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_18(mut iter: adl_Iterator) -> *mut i32 {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_19(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> bool {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_20(
    lhs: *const adl_Iterator,
    rhs: *const adl_Iterator,
) -> std::cmp::Ordering {
    return std::cmp::Ord::cmp(&((*lhs).ptr_), &((*rhs).ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct adl_Iterator {
    ptr_: *mut i32,
    iter_moves_: *mut i32,
    iter_swaps_: *mut i32,
}
impl adl_Iterator {
    unsafe fn new_1(mut p: *mut i32, mut iter_moves: *mut i32, mut iter_swaps: *mut i32) -> Self {
        let mut this = Self {
            ptr_: p,
            iter_moves_: iter_moves,
            iter_swaps_: iter_swaps,
        };
        this
    }
    pub unsafe fn TrackMoves(mut p: *mut i32, iter_moves: *mut i32) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { (iter_moves) }, { std::ptr::null_mut() });
    }
    pub unsafe fn TrackSwaps_4(iter_swaps: *mut i32) -> adl_Iterator {
        return adl_Iterator::new_1({ std::ptr::null_mut() }, { std::ptr::null_mut() }, {
            (iter_swaps)
        });
    }
    pub unsafe fn TrackSwaps_5(mut p: *mut i32, iter_swaps: *mut i32) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { std::ptr::null_mut() }, { (iter_swaps) });
    }
    pub unsafe fn iter_moves(&self) -> i32 {
        assert!(!(self.iter_moves_).is_null());
        return (*self.iter_moves_);
    }
    pub unsafe fn iter_swaps(&self) -> i32 {
        assert!(!(self.iter_swaps_).is_null());
        return (*self.iter_swaps_);
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_index(&self, mut n: isize) -> *mut i32 {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn operator_sub_10(&self, mut n: isize) -> adl_Iterator {
        return adl_Iterator::new_1(
            { self.ptr_.offset(-((n) as isize)) },
            { self.iter_moves_ },
            { self.iter_swaps_ },
        );
    }
    pub unsafe fn operator_sub_11(&self, mut rhs: adl_Iterator) -> isize {
        return (((self.ptr_ as usize - rhs.ptr_ as usize) / ::std::mem::size_of::<i32>()) as i64
            as isize);
    }
    pub unsafe fn operator_add_assign(&mut self, mut n: isize) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_sub_assign(&mut self, mut n: isize) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_inc_14(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_inc_15(&mut self, mut _a0: i32) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_inc();
        return prev;
    }
    pub unsafe fn operator_dec_16(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_dec();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_dec_17(&mut self, mut _a0: i32) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_dec();
        return prev;
    }
}
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_20(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_19(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::Eq for adl_Iterator {}
impl UnsafeCppAdvance for adl_Iterator {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_14();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_16();
    }
}
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
    it_: *mut *mut i32,
}
impl rvalue_iterator_int_ptr_ {
    pub unsafe fn operator_cmp(&self, _a0: *const rvalue_iterator_int_ptr_) -> std::cmp::Ordering {
        {
            let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(
                &((*(self as *const rvalue_iterator_int_ptr_)).it_),
                &((*_a0).it_),
            );
            if !(cmp == std::cmp::Ordering::Equal) {
                return cmp;
            }
        }
        return std::cmp::Ordering::Equal;
    }
    pub unsafe fn operator_eq(&self, _a0: *const rvalue_iterator_int_ptr_) -> bool {
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
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(*const libc::c_char , new, value)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub unsafe fn new(mut what: *const libc::c_char) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn test_impl_21() {
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
        let mut elements: () = r;
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
        let mut elements: () = r;
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
pub unsafe fn test_impl_22() {
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
        let mut elements: () = r;
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
        let mut elements: () = r;
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
pub unsafe fn test_impl_23() {
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
        let mut elements: () = r;
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
        let mut elements: () = r;
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
pub unsafe fn test_impl_24() {
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
        let mut elements: () = r;
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
        let mut elements: () = r;
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
pub unsafe fn test_impl_25() {
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
        let mut elements: () = r;
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
        let mut elements: () = r;
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
pub unsafe fn test_impl_26() {
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
        let mut elements: () = r;
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
        let mut elements: () = r;
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
pub unsafe fn test_impl_27() {
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
        let mut elements: () = r;
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
        let mut elements: () = r;
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
pub unsafe fn test_impl_28() {
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
        let mut elements: () = r;
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
        let mut elements: () = r;
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
pub unsafe fn test_impl_29() {
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
        let mut elements: () = r;
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
        let mut elements: () = r;
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
pub unsafe fn test_impl_30() {
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
        let mut elements: () = r;
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
        let mut elements: () = r;
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
pub unsafe fn test_impl_31() {
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
        let mut elements: () = r;
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
        let mut elements: () = r;
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
pub unsafe fn test_impl_32() {
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
        let mut elements: () = r;
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
        let mut elements: () = r;
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
pub unsafe fn test_impl_33() {
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
        let mut elements: () = r;
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
        let mut elements: () = r;
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
pub unsafe fn test_impl_34() {
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
        let mut elements: () = r;
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
        let mut elements: () = r;
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
pub unsafe fn test_impl_35() {
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
        let mut elements: () = r;
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
        let mut elements: () = r;
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
pub unsafe fn test_impl_36() {
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
        let mut elements: () = r;
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
        let mut elements: () = r;
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
pub unsafe fn test_impl_37() {
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
        let mut elements: () = r;
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
        let mut elements: () = r;
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
pub unsafe fn test_impl_38() {
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
        let mut elements: () = r;
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
        let mut elements: () = r;
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
pub unsafe fn test_impl_39() {
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
        let mut elements: () = r;
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
        let mut elements: () = r;
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
pub unsafe fn test_impl_40() {
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
        let mut elements: () = r;
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
        let mut elements: () = r;
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
pub unsafe fn test_impl_41() {
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
        let mut elements: () = r;
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
        let mut elements: () = r;
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
pub unsafe fn test_42() -> bool {
    (unsafe {
        for_each_8 ( <types_type_list_std_allocator_std_byte___min_allocator_std_byte___test_allocator_std_byte__ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { for_each_10 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_input_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_impl_21 ( ) } )  ;
 } , | | { ( unsafe { test_impl_22 ( ) } )  ;
 } , | | { ( unsafe { test_impl_23 ( ) } )  ;
 } , | | { ( unsafe { test_impl_24 ( ) } )  ;
 } , | | { ( unsafe { test_impl_25 ( ) } )  ;
 } , | | { ( unsafe { test_impl_26 ( ) } )  ;
 } , | | { ( unsafe { test_impl_27 ( ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_12 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_input_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_impl_28 ( ) } )  ;
 } , | | { ( unsafe { test_impl_29 ( ) } )  ;
 } , | | { ( unsafe { test_impl_30 ( ) } )  ;
 } , | | { ( unsafe { test_impl_31 ( ) } )  ;
 } , | | { ( unsafe { test_impl_32 ( ) } )  ;
 } , | | { ( unsafe { test_impl_33 ( ) } )  ;
 } , | | { ( unsafe { test_impl_34 ( ) } )  ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_13 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp20_input_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { test_impl_35 ( ) } )  ;
 } , | | { ( unsafe { test_impl_36 ( ) } )  ;
 } , | | { ( unsafe { test_impl_37 ( ) } )  ;
 } , | | { ( unsafe { test_impl_38 ( ) } )  ;
 } , | | { ( unsafe { test_impl_39 ( ) } )  ;
 } , | | { ( unsafe { test_impl_40 ( ) } )  ;
 } , | | { ( unsafe { test_impl_41 ( ) } )  ;
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_42() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
    std::cell::LazyCell::force(&*&raw const destructed_value_4);
    std::cell::LazyCell::force(&*&raw const moved_value_5);
}
