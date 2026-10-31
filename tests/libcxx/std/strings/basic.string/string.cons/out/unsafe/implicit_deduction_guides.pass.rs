extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_basic_string_char__std_char_traits_char___std_allocator_char____std_basic_string_char__std_char_traits_char___std_allocator_char___
{}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t____std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___
{}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_basic_string_char__std_char_traits_char___test_allocator_char____std_basic_string_char__std_char_traits_char___test_allocator_char___
{}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_basic_string_wchar_t__std_char_traits_wchar_t___test_allocator_wchar_t____std_basic_string_wchar_t__std_char_traits_wchar_t___test_allocator_wchar_t___
{}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_basic_string_wchar_t__constexpr_char_traits_wchar_t___test_allocator_wchar_t____std_basic_string_wchar_t__constexpr_char_traits_wchar_t___test_allocator_wchar_t___
{}
pub type anon_5 = u32;
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_basic_string_wchar_t__constexpr_char_traits_wchar_t___std_allocator_wchar_t____std_basic_string_wchar_t__constexpr_char_traits_wchar_t___std_allocator_wchar_t___
{}
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
pub static mut destructed_value_6: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { -1_i32 });
pub static mut moved_value_7: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 2147483647 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_8(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_9(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_8(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6));
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
            operator_eq_8(
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
pub struct test_allocator_char_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_char_ {
    pub unsafe fn new_1(mut data: i32) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_char_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        assert!(
            ((((*a).data_)
                != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6)))
                && (((*a).id_)
                    != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6))))
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
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6)))
        );
        assert!(
            ((self.id_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6));
    }
    pub unsafe fn allocate(
        &mut self,
        mut n: u32,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut libc::c_char {
        let mut _a1: *const ::libc::c_void = _a1.unwrap_or_else(|| unsafe { std::ptr::null() });
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6)))
        );
        if !((self.stats_).is_null()) {
            if (((*self.stats_).time_to_throw) >= ((*self.stats_).throw_after)) {
                std::process::abort();
            }
            (*self.stats_).time_to_throw.prefix_inc();
            (*self.stats_).alloc_count.prefix_inc();
            (*self.stats_).allocated_size =
                (((*self.stats_).allocated_size as u32).wrapping_add(n)) as i32;
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut libc::c_char, mut s: u32) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size =
                (((*self.stats_).allocated_size as u32).wrapping_sub(s)) as i32;
        }
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> u32 {
        return ((((((2147483647_u32).wrapping_mul((2_u32 as u32)) as u32)
            .wrapping_add((1_u32 as u32))) as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
    }
}
impl Clone for test_allocator_char_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_char_::copy_from(self as *const test_allocator_char_) }
    }
}
impl Default for test_allocator_char_ {
    fn default() -> Self {
        unsafe {
            test_allocator_char_ {
                data_: 0,
                id_: 0,
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_10(
    x: *const test_allocator_wchar_t_,
    y: *const test_allocator_wchar_t_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct test_allocator_wchar_t_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_wchar_t_ {
    pub unsafe fn new_1(mut data: i32) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_wchar_t_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        assert!(
            ((((*a).data_)
                != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6)))
                && (((*a).id_)
                    != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6))))
                && (!(c"copying from destroyed allocator".as_ptr()).is_null())
        );
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).copied.prefix_inc();
        }
        this
    }
    pub unsafe fn move_from(a: *mut test_allocator_wchar_t_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).moved.prefix_inc();
        }
        assert!(
            ((((*a).data_)
                != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6)))
                && (((*a).id_)
                    != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6))))
                && (!(c"moving from destroyed allocator".as_ptr()).is_null())
        );
        (*a).id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_value_7));
        this
    }
    pub unsafe fn destructor(&mut self) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6)))
        );
        assert!(
            ((self.id_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6));
    }
    pub unsafe fn allocate(
        &mut self,
        mut n: u32,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut i32 {
        let mut _a1: *const ::libc::c_void = _a1.unwrap_or_else(|| unsafe { std::ptr::null() });
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6)))
        );
        if !((self.stats_).is_null()) {
            if (((*self.stats_).time_to_throw) >= ((*self.stats_).throw_after)) {
                std::process::abort();
            }
            (*self.stats_).time_to_throw.prefix_inc();
            (*self.stats_).alloc_count.prefix_inc();
            (*self.stats_).allocated_size =
                (((*self.stats_).allocated_size as u32).wrapping_add(n)) as i32;
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut i32, mut s: u32) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size =
                (((*self.stats_).allocated_size as u32).wrapping_sub(s)) as i32;
        }
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> u32 {
        return ((((((2147483647_u32).wrapping_mul((2_u32 as u32)) as u32)
            .wrapping_add((1_u32 as u32))) as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    }
}
impl std::cmp::PartialEq for test_allocator_wchar_t_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_10(
                self as *const test_allocator_wchar_t_,
                other as *const test_allocator_wchar_t_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_wchar_t_ {}
impl Clone for test_allocator_wchar_t_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_wchar_t_::copy_from(self as *const test_allocator_wchar_t_) }
    }
}
impl Default for test_allocator_wchar_t_ {
    fn default() -> Self {
        unsafe {
            test_allocator_wchar_t_ {
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
pub unsafe fn operator_ne_11(
    x: *const cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    y: *const cpp17_input_iterator_const_char_ptr__const_char_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
    it_: *const libc::c_char,
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
    pub unsafe fn new(mut it: *const libc::c_char) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const libc::c_char {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_5(
        &mut self,
    ) -> *mut cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_const_char_ptr__const_char_ptr_));
    }
    pub unsafe fn move_from(
        _a0: *mut cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    ) -> *mut cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp17_input_iterator_const_char_ptr__const_char_ptr_));
    }
}
impl UnsafeCppAdvance for cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_5();
    }
}
pub unsafe fn operator_ne_12(
    x: *const cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_,
    y: *const cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {
    it_: *const i32,
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {
    pub unsafe fn new(mut it: *const i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const i32 {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_5(
        &mut self,
    ) -> *mut cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_));
    }
    pub unsafe fn move_from(
        _a0: *mut cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_,
    ) -> *mut cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_));
    }
}
impl UnsafeCppAdvance for cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_5();
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
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
pub unsafe fn operator_eq_13(
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
            operator_eq_13(
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
pub unsafe fn operator_add_14(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_15(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_14(_i, n)
    });
}
pub unsafe fn iter_swap_16(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap_unsafe!(a.ptr_, b.ptr_);
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_17(mut iter: adl_Iterator) -> *mut i32 {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_18(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> bool {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_19(
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
        unsafe { operator_cmp_19(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_18(self as *const adl_Iterator, other as *const adl_Iterator) }
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct constexpr_char_traits_wchar_t_ {}
impl constexpr_char_traits_wchar_t_ {
    pub unsafe fn assign_1(c1: *mut i32, c2: *const i32) {
        (*c1) = (*c2);
    }
    pub unsafe fn eq(mut c1: i32, mut c2: i32) -> bool {
        return ((c1) == (c2));
    }
    pub unsafe fn lt(mut c1: i32, mut c2: i32) -> bool {
        return ((c1) < (c2));
    }
    pub unsafe fn compare(mut s1: *const i32, mut s2: *const i32, mut n: usize) -> i32 {
        'loop_: while (n != 0) {
            if (unsafe {
                let _c1: i32 = (*s1);
                let _c2: i32 = (*s2);
                constexpr_char_traits_wchar_t_::lt(_c1, _c2)
            }) {
                return -1_i32;
            }
            if (unsafe {
                let _c1: i32 = (*s2);
                let _c2: i32 = (*s1);
                constexpr_char_traits_wchar_t_::lt(_c1, _c2)
            }) {
                return 1;
            }
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return 0;
    }
    pub unsafe fn length(mut s: *const i32) -> usize {
        let mut len: usize = 0_usize;
        'loop_: while !(unsafe {
            let _c1: i32 = (*s);
            let _c2: i32 = 0_i32;
            constexpr_char_traits_wchar_t_::eq(_c1, _c2)
        }) {
            len.prefix_inc();
            s.prefix_inc();
        }
        return len;
    }
    pub unsafe fn copy(mut s1: *mut i32, mut s2: *const i32, mut n: usize) -> *mut i32 {
        {
            assert!(
                ((s2) < ((s1).cast_const())) || ((s2) >= ((s1.offset((n) as isize)).cast_const()))
            );
        }
        let mut r: *mut i32 = s1;
        'loop_: while (n != 0) {
            (unsafe {
                let _c1: *mut i32 = &mut (*s1);
                let _c2: *const i32 = &(*s2);
                constexpr_char_traits_wchar_t_::assign_1(_c1, _c2)
            });
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return r;
    }
}
pub unsafe fn test_20() -> bool {
    {}
    {}
    {
        let mut s: Vec<libc::c_char> = vec![('a' as libc::c_char); (6_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once((0 as libc::c_char)))
            .collect();
        const _: () = assert!(true, "");;
        assert!(
            s == std::slice::from_raw_parts(
                c"aaaaaa".as_ptr(),
                libcc2rs::c_str_len(c"aaaaaa".as_ptr()) + 1
            )
            .to_vec()
        );
        let mut w: Vec<i32> = vec![(98 as i32); (2_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_i32))
            .collect();
        const _: () = assert!(true, "");;
        assert!(
            w == std::slice::from_raw_parts(
                [98 as i32, 98 as i32, 0 as i32,].as_ptr(),
                libcc2rs::c_str_len([98 as i32, 98 as i32, 0 as i32,].as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<libc::c_char> = vec![('a' as libc::c_char); (6_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once((0 as libc::c_char)))
            .collect();
        const _: () = assert!(true, "");;
        assert!(
            s == std::slice::from_raw_parts(
                c"aaaaaa".as_ptr(),
                libcc2rs::c_str_len(c"aaaaaa".as_ptr()) + 1
            )
            .to_vec()
        );
        let mut w: Vec<i32> = vec![(98 as i32); (2_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_i32))
            .collect();
        const _: () = assert!(true, "");;
        assert!(
            w == std::slice::from_raw_parts(
                [98 as i32, 98 as i32, 0 as i32,].as_ptr(),
                libcc2rs::c_str_len([98 as i32, 98 as i32, 0 as i32,].as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let sin: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"abc".as_ptr(), libcc2rs::c_str_len(c"abc".as_ptr()) + 1)
                .to_vec();
        let mut s: Vec<libc::c_char> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            s == std::slice::from_raw_parts(
                c"bc".as_ptr(),
                libcc2rs::c_str_len(c"bc".as_ptr()) + 1
            )
            .to_vec()
        );
        let win: Vec<i32> = std::slice::from_raw_parts(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
            ]
            .as_ptr(),
            libcc2rs::c_str_len(
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
                ]
                .as_ptr(),
            ) + 1,
        )
        .to_vec();
        let mut w: Vec<i32> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            w == std::slice::from_raw_parts(
                [100 as i32, 101 as i32, 102 as i32, 0 as i32,].as_ptr(),
                libcc2rs::c_str_len([100 as i32, 101 as i32, 102 as i32, 0 as i32,].as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let sin: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"abc".as_ptr(), libcc2rs::c_str_len(c"abc".as_ptr()) + 1)
                .to_vec();
        let mut s: Vec<libc::c_char> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            s == std::slice::from_raw_parts(
                c"bc".as_ptr(),
                libcc2rs::c_str_len(c"bc".as_ptr()) + 1
            )
            .to_vec()
        );
        let win: Vec<i32> = std::slice::from_raw_parts(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
            ]
            .as_ptr(),
            libcc2rs::c_str_len(
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
                ]
                .as_ptr(),
            ) + 1,
        )
        .to_vec();
        let mut w: Vec<i32> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            w == std::slice::from_raw_parts(
                [100 as i32, 101 as i32, 102 as i32, 0 as i32,].as_ptr(),
                libcc2rs::c_str_len([100 as i32, 101 as i32, 102 as i32, 0 as i32,].as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let sin: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"abc".as_ptr(), libcc2rs::c_str_len(c"abc".as_ptr()) + 1)
                .to_vec();
        let mut s: Vec<libc::c_char> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            s == std::slice::from_raw_parts(
                c"bc".as_ptr(),
                libcc2rs::c_str_len(c"bc".as_ptr()) + 1
            )
            .to_vec()
        );
        let win: Vec<i32> = std::slice::from_raw_parts(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
            ]
            .as_ptr(),
            libcc2rs::c_str_len(
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
                ]
                .as_ptr(),
            ) + 1,
        )
        .to_vec();
        let mut w: Vec<i32> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            w == std::slice::from_raw_parts(
                [99 as i32, 100 as i32, 101 as i32, 0 as i32,].as_ptr(),
                libcc2rs::c_str_len([99 as i32, 100 as i32, 101 as i32, 0 as i32,].as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let sin: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"abc".as_ptr(), libcc2rs::c_str_len(c"abc".as_ptr()) + 1)
                .to_vec();
        let mut s: Vec<libc::c_char> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            s == std::slice::from_raw_parts(
                c"bc".as_ptr(),
                libcc2rs::c_str_len(c"bc".as_ptr()) + 1
            )
            .to_vec()
        );
        let win: Vec<i32> = std::slice::from_raw_parts(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
            ]
            .as_ptr(),
            libcc2rs::c_str_len(
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
                ]
                .as_ptr(),
            ) + 1,
        )
        .to_vec();
        let mut w: Vec<i32> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            w == std::slice::from_raw_parts(
                [99 as i32, 100 as i32, 101 as i32, 0 as i32,].as_ptr(),
                libcc2rs::c_str_len([99 as i32, 100 as i32, 101 as i32, 0 as i32,].as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"abc".as_ptr(), 2_usize as usize)
                .to_vec()
                .iter()
                .copied()
                .chain(std::iter::once((0 as libc::c_char)))
                .collect();
        const _: () = assert!(true, "");;
        assert!(
            s == std::slice::from_raw_parts(
                c"ab".as_ptr(),
                libcc2rs::c_str_len(c"ab".as_ptr()) + 1
            )
            .to_vec()
        );
        let mut w: Vec<i32> = std::slice::from_raw_parts(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
            ]
            .as_ptr(),
            3_usize as usize,
        )
        .to_vec()
        .iter()
        .copied()
        .chain(std::iter::once(0_i32))
        .collect();
        const _: () = assert!(true, "");;
        assert!(
            w == std::slice::from_raw_parts(
                [97 as i32, 98 as i32, 99 as i32, 0 as i32,].as_ptr(),
                libcc2rs::c_str_len([97 as i32, 98 as i32, 99 as i32, 0 as i32,].as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"abc".as_ptr(), 2_usize as usize)
                .to_vec()
                .iter()
                .copied()
                .chain(std::iter::once((0 as libc::c_char)))
                .collect();
        const _: () = assert!(true, "");;
        assert!(
            s == std::slice::from_raw_parts(
                c"ab".as_ptr(),
                libcc2rs::c_str_len(c"ab".as_ptr()) + 1
            )
            .to_vec()
        );
        let mut w: Vec<i32> = std::slice::from_raw_parts(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
            ]
            .as_ptr(),
            (3_u32 as usize) as usize,
        )
        .to_vec()
        .iter()
        .copied()
        .chain(std::iter::once(0_i32))
        .collect();
        const _: () = assert!(true, "");;
        assert!(
            w == std::slice::from_raw_parts(
                [97 as i32, 98 as i32, 99 as i32, 0 as i32,].as_ptr(),
                libcc2rs::c_str_len([97 as i32, 98 as i32, 99 as i32, 0 as i32,].as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"abc".as_ptr(), libcc2rs::c_str_len(c"abc".as_ptr()) + 1)
                .to_vec();
        const _: () = assert!(true, "");;
        assert!(
            s == std::slice::from_raw_parts(
                c"abc".as_ptr(),
                libcc2rs::c_str_len(c"abc".as_ptr()) + 1
            )
            .to_vec()
        );
        let mut w: Vec<i32> = std::slice::from_raw_parts(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
            ]
            .as_ptr(),
            libcc2rs::c_str_len(
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
                ]
                .as_ptr(),
            ) + 1,
        )
        .to_vec();
        const _: () = assert!(true, "");;
        assert!(
            w == std::slice::from_raw_parts(
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32,
                        0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"abc".as_ptr(), libcc2rs::c_str_len(c"abc".as_ptr()) + 1)
                .to_vec();
        const _: () = assert!(true, "");;
        assert!(
            s == std::slice::from_raw_parts(
                c"abc".as_ptr(),
                libcc2rs::c_str_len(c"abc".as_ptr()) + 1
            )
            .to_vec()
        );
        let mut w: Vec<i32> = std::slice::from_raw_parts(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
            ]
            .as_ptr(),
            libcc2rs::c_str_len(
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
                ]
                .as_ptr(),
            ) + 1,
        )
        .to_vec();
        const _: () = assert!(true, "");;
        assert!(
            w == std::slice::from_raw_parts(
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32,
                        0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut input: *const libc::c_char = c"abcdef".as_ptr();
        let mut s: Vec<libc::c_char> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            s == std::slice::from_raw_parts(
                c"abc".as_ptr(),
                libcc2rs::c_str_len(c"abc".as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        {
            let mut input: *const libc::c_char = c"abcdef".as_ptr();
            let mut s: Vec<libc::c_char> = unimplemented!();
            const _: () = assert!(true, "");;
            assert!(
                s == std::slice::from_raw_parts(
                    c"abc".as_ptr(),
                    libcc2rs::c_str_len(c"abc".as_ptr()) + 1
                )
                .to_vec()
            );
        }
        {
            let mut input: *const i32 = [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
            ]
            .as_ptr();
            let mut s: Vec<i32> = unimplemented!();
            const _: () = assert!(true, "");;
            assert!(
                s == std::slice::from_raw_parts(
                    [97 as i32, 98 as i32, 99 as i32, 0 as i32,].as_ptr(),
                    libcc2rs::c_str_len([97 as i32, 98 as i32, 99 as i32, 0 as i32,].as_ptr()) + 1
                )
                .to_vec()
            );
        }
    }
    {
        let sin: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"abc".as_ptr(), libcc2rs::c_str_len(c"abc".as_ptr()) + 1)
                .to_vec();
        let mut s: Vec<libc::c_char> = sin.clone();
        const _: () = assert!(true, "");;
        assert!(
            s == std::slice::from_raw_parts(
                c"abc".as_ptr(),
                libcc2rs::c_str_len(c"abc".as_ptr()) + 1
            )
            .to_vec()
        );
        let win: Vec<i32> = std::slice::from_raw_parts(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
            ]
            .as_ptr(),
            libcc2rs::c_str_len(
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
                ]
                .as_ptr(),
            ) + 1,
        )
        .to_vec();
        let mut w: Vec<i32> = win.clone();
        const _: () = assert!(true, "");;
        assert!(
            w == std::slice::from_raw_parts(
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32,
                        0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let sin: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"abc".as_ptr(), libcc2rs::c_str_len(c"abc".as_ptr()) + 1)
                .to_vec();
        let mut s: Vec<libc::c_char> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            s == std::slice::from_raw_parts(
                c"abc".as_ptr(),
                libcc2rs::c_str_len(c"abc".as_ptr()) + 1
            )
            .to_vec()
        );
        let win: Vec<i32> = std::slice::from_raw_parts(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
            ]
            .as_ptr(),
            libcc2rs::c_str_len(
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
                ]
                .as_ptr(),
            ) + 1,
        )
        .to_vec();
        let mut w: Vec<i32> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            w == std::slice::from_raw_parts(
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32,
                        0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut sin: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"abc".as_ptr(), libcc2rs::c_str_len(c"abc".as_ptr()) + 1)
                .to_vec();
        let mut s: Vec<libc::c_char> = std::mem::take(&mut sin);
        const _: () = assert!(true, "");;
        assert!(
            s == std::slice::from_raw_parts(
                c"abc".as_ptr(),
                libcc2rs::c_str_len(c"abc".as_ptr()) + 1
            )
            .to_vec()
        );
        let mut win: Vec<i32> = std::slice::from_raw_parts(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
            ]
            .as_ptr(),
            libcc2rs::c_str_len(
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
                ]
                .as_ptr(),
            ) + 1,
        )
        .to_vec();
        let mut w: Vec<i32> = std::mem::take(&mut win);
        const _: () = assert!(true, "");;
        assert!(
            w == std::slice::from_raw_parts(
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32,
                        0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut sin: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"abc".as_ptr(), libcc2rs::c_str_len(c"abc".as_ptr()) + 1)
                .to_vec();
        let mut s: Vec<libc::c_char> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            s == std::slice::from_raw_parts(
                c"abc".as_ptr(),
                libcc2rs::c_str_len(c"abc".as_ptr()) + 1
            )
            .to_vec()
        );
        let mut win: Vec<i32> = std::slice::from_raw_parts(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
            ]
            .as_ptr(),
            libcc2rs::c_str_len(
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
                ]
                .as_ptr(),
            ) + 1,
        )
        .to_vec();
        let mut w: Vec<i32> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            w == std::slice::from_raw_parts(
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32,
                        0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<libc::c_char> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            s == std::slice::from_raw_parts(
                c"abc".as_ptr(),
                libcc2rs::c_str_len(c"abc".as_ptr()) + 1
            )
            .to_vec()
        );
        let mut w: Vec<i32> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            w == std::slice::from_raw_parts(
                [97 as i32, 98 as i32, 99 as i32, 0 as i32,].as_ptr(),
                libcc2rs::c_str_len([97 as i32, 98 as i32, 99 as i32, 0 as i32,].as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<libc::c_char> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            s == std::slice::from_raw_parts(
                c"abc".as_ptr(),
                libcc2rs::c_str_len(c"abc".as_ptr()) + 1
            )
            .to_vec()
        );
        let mut w: Vec<i32> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            w == std::slice::from_raw_parts(
                [97 as i32, 98 as i32, 99 as i32, 0 as i32,].as_ptr(),
                libcc2rs::c_str_len([97 as i32, 98 as i32, 99 as i32, 0 as i32,].as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let mut sv: () = unimplemented!();
        let mut s: Vec<libc::c_char> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            s == std::slice::from_raw_parts(
                c"abc".as_ptr(),
                libcc2rs::c_str_len(c"abc".as_ptr()) + 1
            )
            .to_vec()
        );
        let mut BSV: () = unimplemented!();
        let mut w: Vec<i32> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            w == std::slice::from_raw_parts(
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32,
                        0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut sv: () = unimplemented!();
        let mut s: Vec<libc::c_char> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            s == std::slice::from_raw_parts(
                c"abc".as_ptr(),
                libcc2rs::c_str_len(c"abc".as_ptr()) + 1
            )
            .to_vec()
        );
        let mut BSV: () = unimplemented!();
        let mut w: Vec<i32> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            w == std::slice::from_raw_parts(
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32,
                        0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s0: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"abc".as_ptr(), libcc2rs::c_str_len(c"abc".as_ptr()) + 1)
                .to_vec();
        let mut s: Vec<libc::c_char> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            s == std::slice::from_raw_parts(c"b".as_ptr(), libcc2rs::c_str_len(c"b".as_ptr()) + 1)
                .to_vec()
        );
        let mut w0: Vec<i32> = std::slice::from_raw_parts(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
            ]
            .as_ptr(),
            libcc2rs::c_str_len(
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
                ]
                .as_ptr(),
            ) + 1,
        )
        .to_vec();
        let mut w: Vec<i32> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            w == std::slice::from_raw_parts(
                [99 as i32, 100 as i32, 0 as i32,].as_ptr(),
                libcc2rs::c_str_len([99 as i32, 100 as i32, 0 as i32,].as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s0: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"abc".as_ptr(), libcc2rs::c_str_len(c"abc".as_ptr()) + 1)
                .to_vec();
        let mut s: Vec<libc::c_char> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            s == std::slice::from_raw_parts(c"b".as_ptr(), libcc2rs::c_str_len(c"b".as_ptr()) + 1)
                .to_vec()
        );
        let mut w0: Vec<i32> = std::slice::from_raw_parts(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
            ]
            .as_ptr(),
            libcc2rs::c_str_len(
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 0 as i32,
                ]
                .as_ptr(),
            ) + 1,
        )
        .to_vec();
        let mut w: Vec<i32> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            w == std::slice::from_raw_parts(
                [99 as i32, 100 as i32, 0 as i32,].as_ptr(),
                libcc2rs::c_str_len([99 as i32, 100 as i32, 0 as i32,].as_ptr()) + 1
            )
            .to_vec()
        );
    }
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
    (unsafe { test_20() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_6);
    std::cell::LazyCell::force(&*&raw const moved_value_7);
}
