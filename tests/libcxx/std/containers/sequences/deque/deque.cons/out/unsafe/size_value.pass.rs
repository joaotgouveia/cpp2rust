extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn is_double_ended_contiguous_container_asan_correct_0(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_double_ended_contiguous_container_asan_correct_1(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_double_ended_contiguous_container_asan_correct_2(_a0: *const ()) -> () {
    return true;
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
pub static mut destructed_value_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_5(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_6(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_5(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_3));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_3));
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
            operator_eq_5(
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
pub struct limited_alloc_handle_4096_ {
    pub outstanding_: (),
    pub last_alloc_: (),
}
impl limited_alloc_handle_4096_ {
    pub unsafe fn allocate_1_int_ptr(&mut self, mut N: ()) -> *mut *mut () {
        if (((N).wrapping_add(self.outstanding_)) > (4096_())) {
            unimplemented!();
        }
        let mut alloc: *mut *mut () = unimplemented!();
        self.last_alloc_ = (alloc as ());
        self.outstanding_ = (self.outstanding_).wrapping_add(N);
        return alloc;
    }
    pub unsafe fn allocate_1_int(&mut self, mut N: ()) -> *mut () {
        if (((N).wrapping_add(self.outstanding_)) > (4096_())) {
            unimplemented!();
        }
        let mut alloc: *mut () = unimplemented!();
        self.last_alloc_ = (alloc as ());
        self.outstanding_ = (self.outstanding_).wrapping_add(N);
        return alloc;
    }
    pub unsafe fn deallocate_2_int_ptr(&mut self, mut ptr: *mut *mut (), mut N: ()) {
        if ((ptr as ()) == (self.last_alloc_)) {
            self.last_alloc_ = unimplemented!();
            unimplemented!();
            self.outstanding_ = (self.outstanding_).wrapping_sub(N);
        }
        unimplemented!();
    }
    pub unsafe fn deallocate_2_int(&mut self, mut ptr: *mut (), mut N: ()) {
        if ((ptr as ()) == (self.last_alloc_)) {
            self.last_alloc_ = unimplemented!();
            unimplemented!();
            self.outstanding_ = (self.outstanding_).wrapping_sub(N);
        }
        unimplemented!();
    }
}
impl Default for limited_alloc_handle_4096_ {
    fn default() -> Self {
        unsafe {
            limited_alloc_handle_4096_ {
                outstanding_: 0_(),
                last_alloc_: unimplemented!(),
            }
        }
    }
}
pub unsafe fn make_thread_unsafe_shared_7()
-> detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__ {
    let mut ptr: detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__ =
        <detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__>::default();
    let _dtor_ptr = ScopedDestructorUnsafe::new(
        &raw mut ptr,
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__::destructor,
    );
    let mut alloc: () = unimplemented!();
    ptr.block = unimplemented!();
    (unsafe {
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096___control_block__::construct ( & mut alloc , ptr . block  , )
    });
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__::copy_from({ &ptr });
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096___control_block {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            ref_count: 1_(),
            content: <limited_alloc_handle_4096_>::default(),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096___control_block {
    pub ref_count: (),
    pub content: limited_alloc_handle_4096_,
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096___control_block {
    fn default() -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096___control_block {
                ref_count: 1_(),
                content: <limited_alloc_handle_4096_>::default(),
            }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__ {
    block: *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096___control_block,
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__ {
    pub unsafe fn copy_from(
        other: *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__,
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
        (unsafe {
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096___control_block__::destroy ( & mut alloc , self . block  , )
        });
        unimplemented!();
    }
    pub unsafe fn operator_arrow_7(&mut self) -> *mut limited_alloc_handle_4096_ {
        return (&mut (*self.block).content as *mut limited_alloc_handle_4096_);
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__,
    ) -> *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__ {
        self.block = (*_a0).block;
        return &mut (*(self as *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__));
    }
}
impl Clone for detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__ {
    fn clone(&self) -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__::copy_from(
                self as *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__,
            )
        }
    }
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__ {
    fn default() -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__ {
                block: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct limited_allocator_int__4096__rebind_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct limited_allocator_int__4096__rebind_int_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct limited_allocator_int__4096__rebind_const_int_ptr_ {}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct limited_allocator_int__4096_ {
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__,
}
impl limited_allocator_int__4096_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            handle_: (unsafe { make_thread_unsafe_shared_7() }),
        };
        this
    }
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        return (unsafe {
            limited_alloc_handle_4096_::allocate_1_int(
                &mut (*(unsafe {
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__::operator_arrow_7(
                        &mut self.handle_,
                    )
                })),
                n,
            )
        });
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        (unsafe {
            limited_alloc_handle_4096_::deallocate_2_int(
                &mut (*(unsafe {
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__::operator_arrow_7(
                        &mut self.handle_,
                    )
                })),
                p,
                n,
            )
        });
    }
}
impl limited_allocator_int__4096_ {
    pub unsafe fn destructor(&mut self) {
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__::destructor(&mut self.handle_);
    }
}
impl Default for limited_allocator_int__4096_ {
    fn default() -> Self {
        unsafe { limited_allocator_int__4096_::new() }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct limited_allocator_int_ptr__4096_ {
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__,
}
impl limited_allocator_int_ptr__4096_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            handle_: (unsafe { make_thread_unsafe_shared_7() }),
        };
        this
    }
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut *mut () {
        return (unsafe {
            limited_alloc_handle_4096_::allocate_1_int_ptr(
                &mut (*(unsafe {
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__::operator_arrow_7(
                        &mut self.handle_,
                    )
                })),
                n,
            )
        });
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut (), mut n: ()) {
        (unsafe {
            limited_alloc_handle_4096_::deallocate_2_int_ptr(
                &mut (*(unsafe {
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__::operator_arrow_7(
                        &mut self.handle_,
                    )
                })),
                p,
                n,
            )
        });
    }
}
impl limited_allocator_int_ptr__4096_ {
    pub unsafe fn destructor(&mut self) {
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__::destructor(&mut self.handle_);
    }
}
impl Default for limited_allocator_int_ptr__4096_ {
    fn default() -> Self {
        unsafe { limited_allocator_int_ptr__4096_::new() }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct limited_allocator_const_int_ptr__4096_ {
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__,
}
impl limited_allocator_const_int_ptr__4096_ {}
impl limited_allocator_const_int_ptr__4096_ {
    pub unsafe fn destructor(&mut self) {
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_4096__::destructor(&mut self.handle_);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_9))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_10))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_11)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_8)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_9)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_10)) = 0_();
    }
}
pub static mut outstanding_bytes_8: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_9: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_10: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_sub_12(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_13(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_14(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_13(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn new_2(mut _a0: ()) -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn operator_inc_8(
        &mut self,
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_int__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_int__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_int__std_integral_constant_unsigned_long__0__::operator_add_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
}
impl std::cmp::PartialEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_13(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_15(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_16(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_1(mut p: min_pointer_int__std_integral_constant_unsigned_long__0__) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const () {
        return &(*self.ptr_);
    }
    pub unsafe fn operator_inc_9(
        &mut self,
    ) -> *mut min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self
            as *mut min_pointer_const_int__std_integral_constant_unsigned_long__0__));
    }
}
impl std::cmp::PartialEq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_16(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_17(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize)
        / ::std::mem::size_of::<min_pointer_int__std_integral_constant_unsigned_long__0__>())
        as () as ());
}
pub unsafe fn operator_eq_18(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_19(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_18(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut min_pointer_int__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut min_pointer_int__std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ { return self . ptr_   ;
 } pub unsafe fn operator_inc_8 ( &mut self , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_dec_10 ( &mut self , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_dec() ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add_assign ( &mut self , mut n : () , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_add ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_sub_assign ( &mut self , mut n : () , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_sub ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add ( &self , mut n : () , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } pub unsafe fn operator_sub ( &self , mut n : () , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: operator_sub_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } }
impl std::cmp::PartialEq for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_18(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_20(
    mut x : min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize)
        / ::std::mem::size_of::<min_pointer_int__std_integral_constant_unsigned_long__0__>())
        as () as ());
}
pub unsafe fn operator_eq_21(
    mut x : min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *const min_pointer_int__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { pub unsafe fn new_1 ( mut p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : ( p . ptr_  ) .cast_const() , } ;
 this } pub unsafe fn operator_deref ( &self , ) -> *const min_pointer_int__std_integral_constant_unsigned_long__0__ { return & ( * self . ptr_   )  ;
 } pub unsafe fn operator_inc_9 ( &mut self , ) -> *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add_assign ( &mut self , mut n : () , ) -> *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_add ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_sub_assign ( &mut self , mut n : () , ) -> *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_sub ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } }
impl std::cmp::PartialEq for min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_21(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut min_pointer_const_int__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *const min_pointer_const_int__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
impl min_allocator_int_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn allocate ( &mut self , mut n : () , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_const_int__std_integral_constant_unsigned_long__0___ {}
pub unsafe fn test_22(mut n: (), x: *const ()) {
    let mut d: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = {
        let mut __tmp_0: () = unimplemented!();
        std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024_ :: new_1 ( { & mut __tmp_0 } , )
    };
    let mut e: () = {
        let mut __tmp_1: () = unimplemented!();
        std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024_ :: new_1 ( { & mut __tmp_1 } , )
    };
    'loop_: while !(unimplemented!()) {
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_23(mut n: (), x: *const ()) {
    let mut d: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = {
        let mut __tmp_2: () = unimplemented!();
        std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024_ :: new_1 ( { & mut __tmp_2 } , )
    };
    let mut e: () = {
        let mut __tmp_3: () = unimplemented!();
        std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024_ :: new_1 ( { & mut __tmp_3 } , )
    };
    'loop_: while !(unimplemented!()) {
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_24(mut n: (), x: *const ()) {
    let mut d: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = {
        let mut __tmp_4: () = unimplemented!();
        std___deque_iterator_int__min_pointer_const_int__std_integral_constant_unsigned_long__0____const_int_ref__min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024_ :: new_1 ( { & mut __tmp_4 } , )
    };
    let mut e: () = {
        let mut __tmp_5: () = unimplemented!();
        std___deque_iterator_int__min_pointer_const_int__std_integral_constant_unsigned_long__0____const_int_ref__min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024_ :: new_1 ( { & mut __tmp_5 } , )
    };
    'loop_: while !(unimplemented!()) {
        unimplemented!();
        unimplemented!();
    }
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
    (unsafe {
        let mut _x: () = 5_();
        test_22(0_(), &mut _x)
    });
    (unsafe {
        let mut _x: () = 10_();
        test_22(1_(), &mut _x)
    });
    (unsafe {
        let mut _x: () = 11_();
        test_22(10_(), &mut _x)
    });
    (unsafe {
        let mut _x: () = -11_();
        test_22(1023_(), &mut _x)
    });
    (unsafe {
        let mut _x: () = 25_();
        test_22(1024_(), &mut _x)
    });
    (unsafe {
        let mut _x: () = 0_();
        test_22(1025_(), &mut _x)
    });
    (unsafe {
        let mut _x: () = 110_();
        test_22(2047_(), &mut _x)
    });
    (unsafe {
        let mut _x: () = -500_();
        test_22(2048_(), &mut _x)
    });
    (unsafe {
        let mut _x: () = 654_();
        test_22(2049_(), &mut _x)
    });
    (unsafe {
        let mut _x: () = 78_();
        test_22(4095_(), &mut _x)
    });
    (unsafe {
        let mut _x: () = 1165_();
        test_22(4096_(), &mut _x)
    });
    (unsafe {
        let mut _x: () = 157_();
        test_22(4097_(), &mut _x)
    });
    (unsafe {
        let mut _x: () = 90_();
        test_23(4095_(), &mut _x)
    });
    (unsafe {
        let mut _x: () = 90_();
        test_24(4095_(), &mut _x)
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_3);
    std::cell::LazyCell::force(&*&raw const moved_value_4);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_8);
    std::cell::LazyCell::force(&*&raw const alloc_count_9);
    std::cell::LazyCell::force(&*&raw const dealloc_count_10);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_11);
}
