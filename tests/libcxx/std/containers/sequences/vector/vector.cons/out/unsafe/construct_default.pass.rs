extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
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
pub static mut destructed_value_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { -1_i32 });
pub static mut moved_value_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 2147483647 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_2(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_3(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_2(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
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
            operator_eq_2(
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
pub unsafe fn operator_eq_4(x: *const test_allocator_int_, y: *const test_allocator_int_) -> bool {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_int__rebind_int_ {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_int_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_int_ {
    pub unsafe fn new_1(mut data: i32) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_int_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        assert!(
            ((((*a).data_)
                != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0)))
                && (((*a).id_)
                    != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0))))
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
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0)))
        );
        assert!(
            ((self.id_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut i32, mut s: u32) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size =
                (((*self.stats_).allocated_size as u32).wrapping_sub(s)) as i32;
        }
        unimplemented!();
    }
    pub unsafe fn destroy(&mut self, mut p: *mut i32) {
        if !((self.stats_).is_null()) {
            (*self.stats_).destroy_count.prefix_inc();
        };
    }
}
impl std::cmp::PartialEq for test_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_4(
                self as *const test_allocator_int_,
                other as *const test_allocator_int_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_int_ {}
impl Clone for test_allocator_int_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_int_::copy_from(self as *const test_allocator_int_) }
    }
}
impl Default for test_allocator_int_ {
    fn default() -> Self {
        unsafe {
            test_allocator_int_ {
                data_: 0,
                id_: 0,
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_5(
    x: *const test_allocator_NotConstructible_,
    y: *const test_allocator_NotConstructible_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_NotConstructible__rebind_NotConstructible_ {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_NotConstructible_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_NotConstructible_ {
    pub unsafe fn new_1(mut data: i32) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_NotConstructible_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        assert!(
            ((((*a).data_)
                != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0)))
                && (((*a).id_)
                    != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0))))
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
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0)))
        );
        assert!(
            ((self.id_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut NotConstructible, mut s: u32) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size =
                (((*self.stats_).allocated_size as u32).wrapping_sub(s)) as i32;
        }
        unimplemented!();
    }
    pub unsafe fn destroy(&mut self, mut p: *mut NotConstructible) {
        if !((self.stats_).is_null()) {
            (*self.stats_).destroy_count.prefix_inc();
        };
    }
}
impl std::cmp::PartialEq for test_allocator_NotConstructible_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_5(
                self as *const test_allocator_NotConstructible_,
                other as *const test_allocator_NotConstructible_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_NotConstructible_ {}
impl Clone for test_allocator_NotConstructible_ {
    fn clone(&self) -> Self {
        unsafe {
            test_allocator_NotConstructible_::copy_from(
                self as *const test_allocator_NotConstructible_,
            )
        }
    }
}
impl Default for test_allocator_NotConstructible_ {
    fn default() -> Self {
        unsafe {
            test_allocator_NotConstructible_ {
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
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct limited_alloc_handle_10_ {
    pub outstanding_: usize,
    pub last_alloc_: *mut ::libc::c_void,
}
impl limited_alloc_handle_10_ {
    pub unsafe fn deallocate_2_int(&mut self, mut ptr: *mut i32, mut N: usize) {
        if ((ptr as *mut ::libc::c_void) == (self.last_alloc_)) {
            self.last_alloc_ = std::ptr::null_mut();
            assert!(((self.outstanding_) >= (N)));
            self.outstanding_ = (self.outstanding_).wrapping_sub(N);
        }
        unimplemented!();
    }
}
impl Default for limited_alloc_handle_10_ {
    fn default() -> Self {
        unsafe {
            limited_alloc_handle_10_ {
                outstanding_: 0_usize,
                last_alloc_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn make_thread_unsafe_shared_6()
-> detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ {
    let mut ptr: detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ =
        <detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__>::default();
    let _dtor_ptr = ScopedDestructorUnsafe::new(
        &raw mut ptr,
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__::destructor,
    );
    let mut alloc: () = unimplemented!();
    ptr.block = unimplemented!();
    unimplemented!();
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__::copy_from({ &ptr });
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            ref_count: 1_usize,
            content: <limited_alloc_handle_10_>::default(),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block {
    pub ref_count: usize,
    pub content: limited_alloc_handle_10_,
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block {
    fn default() -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block {
                ref_count: 1_usize,
                content: <limited_alloc_handle_10_>::default(),
            }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ {
    block: *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block,
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ {
    pub unsafe fn copy_from(
        other: *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__,
    ) -> Self {
        let mut this = Self {
            block: (*other).block,
        };
        (*this.block).ref_count.prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*self.block).ref_count.prefix_dec();
        if (((*self.block).ref_count) != (0_usize)) {
            return;
        };
        let mut alloc: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_arrow_7(&mut self) -> *mut limited_alloc_handle_10_ {
        return (&mut (*self.block).content as *mut limited_alloc_handle_10_);
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__,
    ) -> *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ {
        self.block = (*_a0).block;
        return &mut (*(self as *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__));
    }
}
impl Clone for detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ {
    fn clone(&self) -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__::copy_from(
                self as *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__,
            )
        }
    }
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ {
    fn default() -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ {
                block: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct limited_allocator_int__10__rebind_int_ {}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct limited_allocator_int__10_ {
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__,
}
impl limited_allocator_int__10_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            handle_: (unsafe { make_thread_unsafe_shared_6() }),
        };
        this
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut i32, mut n: usize) {
        (unsafe {
            limited_alloc_handle_10_::deallocate_2_int(
                &mut (*(unsafe {
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__::operator_arrow_7(
                        &mut self.handle_,
                    )
                })),
                p,
                n,
            )
        });
    }
}
impl limited_allocator_int__10_ {
    pub unsafe fn destructor(&mut self) {
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__::destructor(&mut self.handle_);
    }
}
impl Default for limited_allocator_int__10_ {
    fn default() -> Self {
        unsafe { limited_allocator_int__10_::new() }
    }
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct NotConstructible {}
impl NotConstructible {}
impl std::cmp::PartialEq for NotConstructible {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_7(
                self as *const NotConstructible,
                other as *const NotConstructible,
            )
        }
    }
}
impl std::cmp::Eq for NotConstructible {}
pub unsafe fn operator_eq_7(_a0: *const NotConstructible, _a1: *const NotConstructible) -> bool {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_9))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_10)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_9))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_10))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_11)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_8)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_9)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_10)) = 0_usize;
    }
}
pub static mut outstanding_bytes_8: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_9: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_10: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_11: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_sub_12(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
}
pub unsafe fn operator_eq_13(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_14(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_13(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut i32,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn to_bool(&self) -> bool {
        return !((self.ptr_).is_null());
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_inc_8(
        &mut self,
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
}
impl std::cmp::PartialEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_13(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_8();
    }
}
pub unsafe fn operator_eq_15(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(
    min_pointer_int__std_integral_constant_unsigned_long__0__,
    new_1,
    value
)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *const i32,
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_1(mut p: min_pointer_int__std_integral_constant_unsigned_long__0__) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
}
impl std::cmp::PartialEq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_15(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *const ::libc::c_void,
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_16(
    mut x: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<NotConstructible>()) as i64
        as isize);
}
pub unsafe fn operator_eq_17(
    mut x: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_18(
    mut x: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_17(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut NotConstructible,
}
impl min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn to_bool(&self) -> bool {
        return !((self.ptr_).is_null());
    }
    pub unsafe fn operator_deref(&self) -> *mut NotConstructible {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_inc_8(
        &mut self,
    ) -> *mut min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self
            as *mut min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__));
    }
}
impl std::cmp::PartialEq
    for min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__
{
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_17(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_8();
    }
}
pub unsafe fn operator_eq_19(
    mut x: min_pointer_const_NotConstructible__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_NotConstructible__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(
    min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
    new_1,
    value
)]
pub struct min_pointer_const_NotConstructible__std_integral_constant_unsigned_long__0__ {
    ptr_: *const NotConstructible,
}
impl min_pointer_const_NotConstructible__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_1(
        mut p: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
    ) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
}
impl std::cmp::PartialEq
    for min_pointer_const_NotConstructible__std_integral_constant_unsigned_long__0__
{
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_19(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_NotConstructible__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_20(mut _a0: min_allocator_int_, mut _a1: min_allocator_int_) -> bool {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
impl min_allocator_int_ {
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
impl std::cmp::PartialEq for min_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_20(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_int_ {}
pub unsafe fn operator_eq_21(
    mut _a0: min_allocator_NotConstructible_,
    mut _a1: min_allocator_NotConstructible_,
) -> bool {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_NotConstructible_ {}
impl min_allocator_NotConstructible_ {
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
impl std::cmp::PartialEq for min_allocator_NotConstructible_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_21(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_NotConstructible_ {}
pub unsafe fn operator_eq_22(
    mut _a0: explicit_allocator_int_,
    mut _a1: explicit_allocator_int_,
) -> bool {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct explicit_allocator_int_ {}
impl explicit_allocator_int_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut i32, mut n: usize) {
        unimplemented!();
    }
}
impl std::cmp::PartialEq for explicit_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_22(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for explicit_allocator_int_ {}
impl Default for explicit_allocator_int_ {
    fn default() -> Self {
        unsafe { explicit_allocator_int_::new() }
    }
}
pub unsafe fn operator_eq_23(
    mut _a0: explicit_allocator_NotConstructible_,
    mut _a1: explicit_allocator_NotConstructible_,
) -> bool {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct explicit_allocator_NotConstructible_ {}
impl explicit_allocator_NotConstructible_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut NotConstructible, mut n: usize) {
        unimplemented!();
    }
}
impl std::cmp::PartialEq for explicit_allocator_NotConstructible_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_23(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for explicit_allocator_NotConstructible_ {}
impl Default for explicit_allocator_NotConstructible_ {
    fn default() -> Self {
        unsafe { explicit_allocator_NotConstructible_::new() }
    }
}
pub unsafe fn test0_24() {
    const _: () = assert!(true, "(noexcept(C{{}}))");;
    let mut c: Vec<i32> = Vec::new();
    const _: () = assert!(true, "");;
    assert!(c.is_empty());
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    let mut c1: Vec<i32> = Vec::new();
    const _: () = assert!(true, "");;
    assert!(c1.is_empty());
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_25() {
    const _: () = assert!(true, "(noexcept(C{{}}))");;
    let mut c: Vec<NotConstructible> = Vec::new();
    const _: () = assert!(true, "");;
    assert!(c.is_empty());
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    let mut c1: Vec<NotConstructible> = Vec::new();
    const _: () = assert!(true, "");;
    assert!(c1.is_empty());
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_26() {
    const _: () = assert!(true, "(noexcept(C{{}}))");;
    let mut c: Vec<i32> = Vec::new();
    const _: () = assert!(true, "");;
    assert!(c.is_empty());
    assert!(
        (unsafe {
            let _arg0: min_allocator_int_ = unimplemented!();
            operator_eq_20(_arg0, <min_allocator_int_>::default())
        })
    );
    const _: () = assert!(true, "");;
    let mut c1: Vec<i32> = Vec::new();
    const _: () = assert!(true, "");;
    assert!(c1.is_empty());
    assert!(
        (unsafe {
            let _arg0: min_allocator_int_ = unimplemented!();
            operator_eq_20(_arg0, <min_allocator_int_>::default())
        })
    );
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_27() {
    const _: () = assert!(true, "(noexcept(C{{}}))");;
    let mut c: Vec<NotConstructible> = Vec::new();
    const _: () = assert!(true, "");;
    assert!(c.is_empty());
    assert!(
        (unsafe {
            let _arg0: min_allocator_NotConstructible_ = unimplemented!();
            operator_eq_21(_arg0, <min_allocator_NotConstructible_>::default())
        })
    );
    const _: () = assert!(true, "");;
    let mut c1: Vec<NotConstructible> = Vec::new();
    const _: () = assert!(true, "");;
    assert!(c1.is_empty());
    assert!(
        (unsafe {
            let _arg0: min_allocator_NotConstructible_ = unimplemented!();
            operator_eq_21(_arg0, <min_allocator_NotConstructible_>::default())
        })
    );
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_28() {
    const _: () = assert!(true, "(noexcept(C{{}}))");;
    let mut c: Vec<i32> = Vec::new();
    const _: () = assert!(true, "");;
    assert!(c.is_empty());
    assert!(
        (unsafe {
            let _arg0: explicit_allocator_int_ = unimplemented!();
            operator_eq_22(_arg0, explicit_allocator_int_::new())
        })
    );
    const _: () = assert!(true, "");;
    let mut c1: Vec<i32> = Vec::new();
    const _: () = assert!(true, "");;
    assert!(c1.is_empty());
    assert!(
        (unsafe {
            let _arg0: explicit_allocator_int_ = unimplemented!();
            operator_eq_22(_arg0, explicit_allocator_int_::new())
        })
    );
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_29() {
    const _: () = assert!(true, "(noexcept(C{{}}))");;
    let mut c: Vec<NotConstructible> = Vec::new();
    const _: () = assert!(true, "");;
    assert!(c.is_empty());
    assert!(
        (unsafe {
            let _arg0: explicit_allocator_NotConstructible_ = unimplemented!();
            operator_eq_23(_arg0, explicit_allocator_NotConstructible_::new())
        })
    );
    const _: () = assert!(true, "");;
    let mut c1: Vec<NotConstructible> = Vec::new();
    const _: () = assert!(true, "");;
    assert!(c1.is_empty());
    assert!(
        (unsafe {
            let _arg0: explicit_allocator_NotConstructible_ = unimplemented!();
            operator_eq_23(_arg0, explicit_allocator_NotConstructible_::new())
        })
    );
    const _: () = assert!(true, "");;
}
pub unsafe fn test1_30(a: *const test_allocator_int_) {
    const _: () = assert!(true, "(noexcept(C{{typename C::allocator_type{{}}}}))");;
    let mut c: Vec<i32> = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(c.is_empty());
    assert!(
        (unsafe {
            let mut _x: test_allocator_int_ = unimplemented!();
            let _y: *const test_allocator_int_ = a;
            operator_eq_4(&mut _x, _y)
        })
    );
    const _: () = assert!(true, "");;
}
pub unsafe fn test1_31(a: *const test_allocator_NotConstructible_) {
    const _: () = assert!(true, "(noexcept(C{{typename C::allocator_type{{}}}}))");;
    let mut c: Vec<NotConstructible> = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(c.is_empty());
    assert!(
        (unsafe {
            let mut _x: test_allocator_NotConstructible_ = unimplemented!();
            let _y: *const test_allocator_NotConstructible_ = a;
            operator_eq_5(&mut _x, _y)
        })
    );
    const _: () = assert!(true, "");;
}
pub unsafe fn test1_32(a: *const min_allocator_int_) {
    const _: () = assert!(true, "(noexcept(C{{typename C::allocator_type{{}}}}))");;
    let mut c: Vec<i32> = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(c.is_empty());
    assert!(
        (unsafe {
            let _arg0: min_allocator_int_ = unimplemented!();
            let _arg1: min_allocator_int_ = (*a);
            operator_eq_20(_arg0, _arg1)
        })
    );
    const _: () = assert!(true, "");;
}
pub unsafe fn test1_33(a: *const min_allocator_NotConstructible_) {
    const _: () = assert!(true, "(noexcept(C{{typename C::allocator_type{{}}}}))");;
    let mut c: Vec<NotConstructible> = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(c.is_empty());
    assert!(
        (unsafe {
            let _arg0: min_allocator_NotConstructible_ = unimplemented!();
            let _arg1: min_allocator_NotConstructible_ = (*a);
            operator_eq_21(_arg0, _arg1)
        })
    );
    const _: () = assert!(true, "");;
}
pub unsafe fn test1_34(a: *const explicit_allocator_int_) {
    const _: () = assert!(true, "(noexcept(C{{typename C::allocator_type{{}}}}))");;
    let mut c: Vec<i32> = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(c.is_empty());
    assert!(
        (unsafe {
            let _arg0: explicit_allocator_int_ = unimplemented!();
            let _arg1: explicit_allocator_int_ = (*a);
            operator_eq_22(_arg0, _arg1)
        })
    );
    const _: () = assert!(true, "");;
}
pub unsafe fn test1_35(a: *const explicit_allocator_NotConstructible_) {
    const _: () = assert!(true, "(noexcept(C{{typename C::allocator_type{{}}}}))");;
    let mut c: Vec<NotConstructible> = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(c.is_empty());
    assert!(
        (unsafe {
            let _arg0: explicit_allocator_NotConstructible_ = unimplemented!();
            let _arg1: explicit_allocator_NotConstructible_ = (*a);
            operator_eq_23(_arg0, _arg1)
        })
    );
    const _: () = assert!(true, "");;
}
pub unsafe fn tests_36() -> bool {
    {
        (unsafe { test0_24() });
        (unsafe { test0_25() });
        (unsafe {
            let mut _a: test_allocator_int_ = test_allocator_int_::new_1({ 3 });
            test1_30(&mut _a)
        });
        (unsafe {
            let mut _a: test_allocator_NotConstructible_ =
                test_allocator_NotConstructible_::new_1({ 5 });
            test1_31(&mut _a)
        });
    }
    {
        let mut v: Vec<i32> = Vec::new();
        assert!(v.is_empty());
    }
    {
        (unsafe { test0_26() });
        (unsafe { test0_27() });
        (unsafe {
            let mut _a: min_allocator_int_ = <min_allocator_int_>::default();
            test1_32(&mut _a)
        });
        (unsafe {
            let mut _a: min_allocator_NotConstructible_ =
                <min_allocator_NotConstructible_>::default();
            test1_33(&mut _a)
        });
    }
    {
        let mut v: Vec<i32> = Vec::new();
        assert!(v.is_empty());
    }
    {
        (unsafe { test0_28() });
        (unsafe { test0_29() });
        (unsafe {
            let mut _a: explicit_allocator_int_ = explicit_allocator_int_::new();
            test1_34(&mut _a)
        });
        (unsafe {
            let mut _a: explicit_allocator_NotConstructible_ =
                explicit_allocator_NotConstructible_::new();
            test1_35(&mut _a)
        });
    }
    {
        let mut v: Vec<i32> = Vec::new();
        assert!(v.is_empty());
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
    (unsafe { tests_36() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_0);
    std::cell::LazyCell::force(&*&raw const moved_value_1);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_8);
    std::cell::LazyCell::force(&*&raw const alloc_count_9);
    std::cell::LazyCell::force(&*&raw const dealloc_count_10);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_11);
}
