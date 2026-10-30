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
pub struct min_pointer_bool__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_bool__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn to_(&self) -> () {
        return !((self.ptr_).is_null());
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self
            as *mut min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__::operator_sub_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_bool_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_unsigned_long_ {}
impl min_allocator_unsigned_long_ {
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_bool__unsigned_char__signed_char_ {
    data_: (),
}
impl sized_allocator_bool__unsigned_char__signed_char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct sized_allocator_unsigned_long__unsigned_char__signed_char_ {
    data_: (),
}
impl sized_allocator_unsigned_long__unsigned_char__signed_char_ {
    pub unsafe fn new(mut d: Option<()>) -> Self {
        let mut d: () = d.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { data_: d };
        this
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return (unimplemented!()).wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
impl Default for sized_allocator_unsigned_long__unsigned_char__signed_char_ {
    fn default() -> Self {
        unsafe { sized_allocator_unsigned_long__unsigned_char__signed_char_::new(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_bool__unsigned_short__short_ {
    data_: (),
}
impl sized_allocator_bool__unsigned_short__short_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct sized_allocator_unsigned_long__unsigned_short__short_ {
    data_: (),
}
impl sized_allocator_unsigned_long__unsigned_short__short_ {
    pub unsafe fn new(mut d: Option<()>) -> Self {
        let mut d: () = d.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { data_: d };
        this
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return (unimplemented!()).wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
impl Default for sized_allocator_unsigned_long__unsigned_short__short_ {
    fn default() -> Self {
        unsafe { sized_allocator_unsigned_long__unsigned_short__short_::new(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_bool__unsigned_int__int_ {
    data_: (),
}
impl sized_allocator_bool__unsigned_int__int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct sized_allocator_unsigned_long__unsigned_int__int_ {
    data_: (),
}
impl sized_allocator_unsigned_long__unsigned_int__int_ {
    pub unsafe fn new(mut d: Option<()>) -> Self {
        let mut d: () = d.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { data_: d };
        this
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return (unimplemented!()).wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
impl Default for sized_allocator_unsigned_long__unsigned_int__int_ {
    fn default() -> Self {
        unsafe { sized_allocator_unsigned_long__unsigned_int__int_::new(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_bool__unsigned_long__long_ {
    data_: (),
}
impl sized_allocator_bool__unsigned_long__long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct sized_allocator_unsigned_long__unsigned_long__long_ {
    data_: (),
}
impl sized_allocator_unsigned_long__unsigned_long__long_ {
    pub unsafe fn new(mut d: Option<()>) -> Self {
        let mut d: () = d.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { data_: d };
        this
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return (unimplemented!() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
impl Default for sized_allocator_unsigned_long__unsigned_long__long_ {
    fn default() -> Self {
        unsafe { sized_allocator_unsigned_long__unsigned_long__long_::new(None) }
    }
}
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_bool__rebind_unsigned_long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_bool__rebind_bool_ {}
#[repr(C)]
#[derive(FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_bool_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_bool_ {
    pub unsafe fn new_1(mut data: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: std::ptr::null_mut(),
        };
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
impl Default for test_allocator_bool_ {
    fn default() -> Self {
        unsafe {
            test_allocator_bool_ {
                data_: unimplemented!(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
impl test_allocator_unsigned_long_ {
    pub unsafe fn new_1(a: *const test_allocator_bool_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).converted.prefix_inc();
        }
        this
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_unsigned_long_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_unsigned_long_ {
    pub unsafe fn copy_from(a: *const test_allocator_unsigned_long_) -> Self {
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
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut s: ()) {
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_sub(s);
        }
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return (((2147483647_()).wrapping_mul((2_() as ())) as ()).wrapping_add((1_() as ())))
            .wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
impl Clone for test_allocator_unsigned_long_ {
    fn clone(&self) -> Self {
        unsafe {
            test_allocator_unsigned_long_::copy_from(self as *const test_allocator_unsigned_long_)
        }
    }
}
impl Default for test_allocator_unsigned_long_ {
    fn default() -> Self {
        unsafe {
            test_allocator_unsigned_long_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct other_allocator_bool_ {
    data_: (),
}
impl other_allocator_bool_ {
    pub unsafe fn new_1(mut i: ()) -> Self {
        let mut this = Self { data_: i };
        this
    }
}
impl Default for other_allocator_bool_ {
    fn default() -> Self {
        unsafe {
            other_allocator_bool_ {
                data_: unimplemented!(),
            }
        }
    }
}
impl other_allocator_unsigned_long_ {
    pub unsafe fn new_1(a: *const other_allocator_bool_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct other_allocator_unsigned_long_ {
    data_: (),
}
impl other_allocator_unsigned_long_ {
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut s: ()) {
        unimplemented!();
    }
}
impl Default for other_allocator_unsigned_long_ {
    fn default() -> Self {
        unsafe {
            other_allocator_unsigned_long_ {
                data_: unimplemented!(),
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
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct limited_alloc_handle_18446744073709551615_ {
    pub outstanding_: (),
    pub last_alloc_: (),
}
impl limited_alloc_handle_18446744073709551615_ {
    pub unsafe fn deallocate_2_unsigned_long(&mut self, mut ptr: *mut (), mut N: ()) {
        if ((ptr as ()) == (self.last_alloc_)) {
            self.last_alloc_ = unimplemented!();
            unimplemented!();
            self.outstanding_ = (self.outstanding_).wrapping_sub(N);
        }
        unimplemented!();
    }
}
impl Default for limited_alloc_handle_18446744073709551615_ {
    fn default() -> Self {
        unsafe {
            limited_alloc_handle_18446744073709551615_ {
                outstanding_: 0_(),
                last_alloc_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct limited_alloc_handle_302405640552615600_ {
    pub outstanding_: (),
    pub last_alloc_: (),
}
impl limited_alloc_handle_302405640552615600_ {
    pub unsafe fn deallocate_2_unsigned_long(&mut self, mut ptr: *mut (), mut N: ()) {
        if ((ptr as ()) == (self.last_alloc_)) {
            self.last_alloc_ = unimplemented!();
            unimplemented!();
            self.outstanding_ = (self.outstanding_).wrapping_sub(N);
        }
        unimplemented!();
    }
}
impl Default for limited_alloc_handle_302405640552615600_ {
    fn default() -> Self {
        unsafe {
            limited_alloc_handle_302405640552615600_ {
                outstanding_: 0_(),
                last_alloc_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct limited_alloc_handle_292805461487453200_ {
    pub outstanding_: (),
    pub last_alloc_: (),
}
impl limited_alloc_handle_292805461487453200_ {
    pub unsafe fn deallocate_2_unsigned_long(&mut self, mut ptr: *mut (), mut N: ()) {
        if ((ptr as ()) == (self.last_alloc_)) {
            self.last_alloc_ = unimplemented!();
            unimplemented!();
            self.outstanding_ = (self.outstanding_).wrapping_sub(N);
        }
        unimplemented!();
    }
}
impl Default for limited_alloc_handle_292805461487453200_ {
    fn default() -> Self {
        unsafe {
            limited_alloc_handle_292805461487453200_ {
                outstanding_: 0_(),
                last_alloc_: unimplemented!(),
            }
        }
    }
}
pub unsafe fn make_thread_unsafe_shared_8()
-> detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ {
    let mut ptr: detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ =
        <detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__>::default();
    let _dtor_ptr = ScopedDestructorUnsafe::new(
        &raw mut ptr,
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__::destructor,
    );
    let mut alloc: () = unimplemented!();
    ptr.block = unimplemented!();
    unimplemented!();
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__::copy_from(
        { &ptr },
    );
}
pub unsafe fn make_thread_unsafe_shared_9()
-> detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__ {
    let mut ptr: detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__ =
        <detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__>::default();
    let _dtor_ptr = ScopedDestructorUnsafe::new(
        &raw mut ptr,
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__::destructor,
    );
    let mut alloc: () = unimplemented!();
    ptr.block = unimplemented!();
    unimplemented!();
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__::copy_from({
        &ptr
    });
}
pub unsafe fn make_thread_unsafe_shared_10()
-> detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__ {
    let mut ptr: detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__ =
        <detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__>::default();
    let _dtor_ptr = ScopedDestructorUnsafe::new(
        &raw mut ptr,
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__::destructor,
    );
    let mut alloc: () = unimplemented!();
    ptr.block = unimplemented!();
    unimplemented!();
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__::copy_from({
        &ptr
    });
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            ref_count: 1_(),
            content: <limited_alloc_handle_18446744073709551615_>::default(),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block {
    pub ref_count: (),
    pub content: limited_alloc_handle_18446744073709551615_,
}
impl Default
    for detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block
{
    fn default() -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block { ref_count : 1_() , content : <limited_alloc_handle_18446744073709551615_ >::default() , }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ { block : *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block , }
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ {
    pub unsafe fn copy_from(
        other: *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__,
    ) -> Self {
        let mut this = Self {
            block: (*other).block,
        };
        (*this.block).ref_count.prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*self.block).ref_count.prefix_dec();
        if (((*self.block).ref_count) != (0_())) {
            return;
        };
        let mut alloc: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_arrow_7(&mut self) -> *mut limited_alloc_handle_18446744073709551615_ {
        return (&mut (*self.block).content as *mut limited_alloc_handle_18446744073709551615_);
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__,
    ) -> *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ {
        self.block = (*_a0).block;
        return &mut (*(self
            as *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__));
    }
}
impl Clone for detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ {
    fn clone(&self) -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__::copy_from(self as *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__)
        }
    }
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ {
    fn default() -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ {
                block: std::ptr::null_mut(),
            }
        }
    }
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            ref_count: 1_(),
            content: <limited_alloc_handle_302405640552615600_>::default(),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block {
    pub ref_count: (),
    pub content: limited_alloc_handle_302405640552615600_,
}
impl Default
    for detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block
{
    fn default() -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block { ref_count : 1_() , content : <limited_alloc_handle_302405640552615600_ >::default() , }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__ { block : *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block , }
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__ {
    pub unsafe fn copy_from(
        other: *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__,
    ) -> Self {
        let mut this = Self {
            block: (*other).block,
        };
        (*this.block).ref_count.prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*self.block).ref_count.prefix_dec();
        if (((*self.block).ref_count) != (0_())) {
            return;
        };
        let mut alloc: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_arrow_7(&mut self) -> *mut limited_alloc_handle_302405640552615600_ {
        return (&mut (*self.block).content as *mut limited_alloc_handle_302405640552615600_);
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__,
    ) -> *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__ {
        self.block = (*_a0).block;
        return &mut (*(self
            as *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__));
    }
}
impl Clone for detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__ {
    fn clone(&self) -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__::copy_from(self as *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__)
        }
    }
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__ {
    fn default() -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__ {
                block: std::ptr::null_mut(),
            }
        }
    }
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            ref_count: 1_(),
            content: <limited_alloc_handle_292805461487453200_>::default(),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block {
    pub ref_count: (),
    pub content: limited_alloc_handle_292805461487453200_,
}
impl Default
    for detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block
{
    fn default() -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block { ref_count : 1_() , content : <limited_alloc_handle_292805461487453200_ >::default() , }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__ { block : *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block , }
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__ {
    pub unsafe fn copy_from(
        other: *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__,
    ) -> Self {
        let mut this = Self {
            block: (*other).block,
        };
        (*this.block).ref_count.prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*self.block).ref_count.prefix_dec();
        if (((*self.block).ref_count) != (0_())) {
            return;
        };
        let mut alloc: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_arrow_7(&mut self) -> *mut limited_alloc_handle_292805461487453200_ {
        return (&mut (*self.block).content as *mut limited_alloc_handle_292805461487453200_);
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__,
    ) -> *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__ {
        self.block = (*_a0).block;
        return &mut (*(self
            as *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__));
    }
}
impl Clone for detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__ {
    fn clone(&self) -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__::copy_from(self as *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__)
        }
    }
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__ {
    fn default() -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__ {
                block: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct limited_allocator_bool__18446744073709551615__rebind_unsigned_long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct limited_allocator_bool__18446744073709551615__rebind_bool_ {}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct limited_allocator_bool__18446744073709551615_ {
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__,
}
impl limited_allocator_bool__18446744073709551615_ {}
impl limited_allocator_bool__18446744073709551615_ {
    pub unsafe fn destructor(&mut self) {
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__::destructor(
            &mut self.handle_,
        );
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct limited_allocator_unsigned_long__18446744073709551615_ {
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__,
}
impl limited_allocator_unsigned_long__18446744073709551615_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            handle_: (unsafe { make_thread_unsafe_shared_8() }),
        };
        this
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        (unsafe {
            let _ptr: *mut () = p;
            let _N: () = n;
            limited_alloc_handle_18446744073709551615_::deallocate_2_unsigned_long(
                &mut (*(unsafe {
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ :: operator_arrow_7 ( &mut self . handle_   , )
                })),
                _ptr,
                _N,
            )
        });
    }
    pub unsafe fn max_size(&self) -> () {
        return 18446744073709551615_();
    }
}
impl limited_allocator_unsigned_long__18446744073709551615_ {
    pub unsafe fn destructor(&mut self) {
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__::destructor(
            &mut self.handle_,
        );
    }
}
impl Default for limited_allocator_unsigned_long__18446744073709551615_ {
    fn default() -> Self {
        unsafe { limited_allocator_unsigned_long__18446744073709551615_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct limited_allocator_bool__302405640552615600__rebind_unsigned_long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct limited_allocator_bool__302405640552615600__rebind_bool_ {}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct limited_allocator_bool__302405640552615600_ {
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__,
}
impl limited_allocator_bool__302405640552615600_ {}
impl limited_allocator_bool__302405640552615600_ {
    pub unsafe fn destructor(&mut self) {
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__::destructor(
            &mut self.handle_,
        );
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct limited_allocator_unsigned_long__302405640552615600_ {
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__,
}
impl limited_allocator_unsigned_long__302405640552615600_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            handle_: (unsafe { make_thread_unsafe_shared_9() }),
        };
        this
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        (unsafe {
            let _ptr: *mut () = p;
            let _N: () = n;
            limited_alloc_handle_302405640552615600_::deallocate_2_unsigned_long(
                &mut (*(unsafe {
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__ :: operator_arrow_7 ( &mut self . handle_   , )
                })),
                _ptr,
                _N,
            )
        });
    }
    pub unsafe fn max_size(&self) -> () {
        return 302405640552615600_();
    }
}
impl limited_allocator_unsigned_long__302405640552615600_ {
    pub unsafe fn destructor(&mut self) {
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__::destructor(
            &mut self.handle_,
        );
    }
}
impl Default for limited_allocator_unsigned_long__302405640552615600_ {
    fn default() -> Self {
        unsafe { limited_allocator_unsigned_long__302405640552615600_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct limited_allocator_bool__292805461487453200__rebind_unsigned_long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct limited_allocator_bool__292805461487453200__rebind_bool_ {}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct limited_allocator_bool__292805461487453200_ {
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__,
}
impl limited_allocator_bool__292805461487453200_ {}
impl limited_allocator_bool__292805461487453200_ {
    pub unsafe fn destructor(&mut self) {
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__::destructor(
            &mut self.handle_,
        );
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct limited_allocator_unsigned_long__292805461487453200_ {
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__,
}
impl limited_allocator_unsigned_long__292805461487453200_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            handle_: (unsafe { make_thread_unsafe_shared_10() }),
        };
        this
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        (unsafe {
            let _ptr: *mut () = p;
            let _N: () = n;
            limited_alloc_handle_292805461487453200_::deallocate_2_unsigned_long(
                &mut (*(unsafe {
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__ :: operator_arrow_7 ( &mut self . handle_   , )
                })),
                _ptr,
                _N,
            )
        });
    }
    pub unsafe fn max_size(&self) -> () {
        return 292805461487453200_();
    }
}
impl limited_allocator_unsigned_long__292805461487453200_ {
    pub unsafe fn destructor(&mut self) {
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__::destructor(
            &mut self.handle_,
        );
    }
}
impl Default for limited_allocator_unsigned_long__292805461487453200_ {
    fn default() -> Self {
        unsafe { limited_allocator_unsigned_long__292805461487453200_::new() }
    }
}
pub unsafe fn test_11(v: *const ()) {
    let max_dist: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_12(v: *const ()) {
    let max_dist: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_13(v: *const ()) {
    let max_dist: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_14(v: *const ()) {
    let max_dist: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_15(v: *const ()) {
    let max_dist: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_16(v: *const ()) {
    let max_dist: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_17(v: *const ()) {
    let max_dist: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_18(v: *const ()) {
    let max_dist: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_19(v: *const ()) {
    let max_dist: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_20(v: *const ()) {
    let max_dist: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_21(v: *const ()) {
    let max_dist: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn tests_22() -> () {
    {
        (unsafe {
            let mut _v: () = unimplemented!();
            test_11(&mut _v)
        });
        (unsafe {
            let mut _v: () = unimplemented!();
            test_11(&mut _v)
        });
        (unsafe {
            let mut _v: () = unimplemented!();
            test_12(&mut _v)
        });
        (unsafe {
            let mut _v: () = unimplemented!();
            test_13(&mut _v)
        });
        (unsafe {
            let mut _v: () = unimplemented!();
            test_14(&mut _v)
        });
        (unsafe {
            let mut _v: () = unimplemented!();
            test_15(&mut _v)
        });
        (unsafe {
            let mut _v: () = unimplemented!();
            test_16(&mut _v)
        });
        (unsafe {
            let mut _v: () = unimplemented!();
            test_17(&mut _v)
        });
        (unsafe {
            let mut _v: () = unimplemented!();
            test_18(&mut _v)
        });
        (unsafe {
            let mut _v: () = unimplemented!();
            test_19(&mut _v)
        });
    }
    {
        (unsafe {
            let mut _v: () = unimplemented!();
            test_20(&mut _v)
        });
        (unsafe {
            let mut _v: () = unimplemented!();
            test_21(&mut _v)
        });
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { tests_22() });
    const _: () = assert!(true, "tests()");;
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
