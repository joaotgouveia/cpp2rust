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
pub struct limited_alloc_handle_1_ {
    pub outstanding_: usize,
    pub last_alloc_: *mut ::libc::c_void,
}
impl limited_alloc_handle_1_ {
    pub unsafe fn allocate_1_class_NotConstructible_ptr(
        &mut self,
        mut N: usize,
    ) -> *mut *mut NotConstructible {
        if (((N).wrapping_add(self.outstanding_)) > (1_u64 as usize)) {
            std::process::abort();
        }
        let mut alloc: *mut *mut NotConstructible = unimplemented!();
        self.last_alloc_ = (alloc as *mut ::libc::c_void);
        self.outstanding_ = (self.outstanding_).wrapping_add(N);
        return alloc;
    }
    pub unsafe fn allocate_1_class_NotConstructible(
        &mut self,
        mut N: usize,
    ) -> *mut NotConstructible {
        if (((N).wrapping_add(self.outstanding_)) > (1_u64 as usize)) {
            std::process::abort();
        }
        let mut alloc: *mut NotConstructible = unimplemented!();
        self.last_alloc_ = (alloc as *mut ::libc::c_void);
        self.outstanding_ = (self.outstanding_).wrapping_add(N);
        return alloc;
    }
    pub unsafe fn deallocate_2_class_NotConstructible(
        &mut self,
        mut ptr: *mut NotConstructible,
        mut N: usize,
    ) {
        if ((ptr as *mut ::libc::c_void) == (self.last_alloc_)) {
            self.last_alloc_ = std::ptr::null_mut();
            assert!(((self.outstanding_) >= (N)));
            self.outstanding_ = (self.outstanding_).wrapping_sub(N);
        }
        unimplemented!();
    }
    pub unsafe fn deallocate_2_class_NotConstructible_ptr(
        &mut self,
        mut ptr: *mut *mut NotConstructible,
        mut N: usize,
    ) {
        if ((ptr as *mut ::libc::c_void) == (self.last_alloc_)) {
            self.last_alloc_ = std::ptr::null_mut();
            assert!(((self.outstanding_) >= (N)));
            self.outstanding_ = (self.outstanding_).wrapping_sub(N);
        }
        unimplemented!();
    }
}
impl Default for limited_alloc_handle_1_ {
    fn default() -> Self {
        unsafe {
            limited_alloc_handle_1_ {
                outstanding_: 0_usize,
                last_alloc_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn make_thread_unsafe_shared_4()
-> detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__ {
    let mut ptr: detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__ =
        <detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__>::default();
    let _dtor_ptr = ScopedDestructorUnsafe::new(
        &raw mut ptr,
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__::destructor,
    );
    let mut alloc: () = unimplemented!();
    ptr.block = unimplemented!();
    unimplemented!();
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__::copy_from({ &ptr });
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_1___control_block {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            ref_count: 1_usize,
            content: <limited_alloc_handle_1_>::default(),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_1___control_block {
    pub ref_count: usize,
    pub content: limited_alloc_handle_1_,
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_1___control_block {
    fn default() -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_1___control_block {
                ref_count: 1_usize,
                content: <limited_alloc_handle_1_>::default(),
            }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__ {
    block: *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_1___control_block,
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__ {
    pub unsafe fn copy_from(
        other: *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__,
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
    pub unsafe fn operator_arrow_7(&mut self) -> *mut limited_alloc_handle_1_ {
        return (&mut (*self.block).content as *mut limited_alloc_handle_1_);
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__,
    ) -> *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__ {
        self.block = (*_a0).block;
        return &mut (*(self as *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__));
    }
}
impl Clone for detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__ {
    fn clone(&self) -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__::copy_from(
                self as *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__,
            )
        }
    }
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__ {
    fn default() -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__ {
                block: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct limited_allocator_NotConstructible__1__rebind_NotConstructible_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct limited_allocator_NotConstructible__1__rebind_NotConstructible_ptr_ {}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct limited_allocator_NotConstructible__1_ {
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__,
}
impl limited_allocator_NotConstructible__1_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            handle_: (unsafe { make_thread_unsafe_shared_4() }),
        };
        this
    }
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut NotConstructible {
        return (unsafe {
            limited_alloc_handle_1_::allocate_1_class_NotConstructible(
                &mut (*(unsafe {
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__::operator_arrow_7(
                        &mut self.handle_,
                    )
                })),
                n,
            )
        });
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut NotConstructible, mut n: usize) {
        (unsafe {
            limited_alloc_handle_1_::deallocate_2_class_NotConstructible(
                &mut (*(unsafe {
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__::operator_arrow_7(
                        &mut self.handle_,
                    )
                })),
                p,
                n,
            )
        });
    }
    pub unsafe fn max_size(&self) -> usize {
        return (1_u64 as usize);
    }
}
impl limited_allocator_NotConstructible__1_ {
    pub unsafe fn destructor(&mut self) {
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__::destructor(&mut self.handle_);
    }
}
impl Default for limited_allocator_NotConstructible__1_ {
    fn default() -> Self {
        unsafe { limited_allocator_NotConstructible__1_::new() }
    }
}
impl limited_allocator_NotConstructible_ptr__1_ {
    pub unsafe fn new_1(other: *const limited_allocator_NotConstructible__1_) -> Self {
        let mut this = Self {
            handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__::copy_from({
                &(*other).handle_
            }),
        };
        this
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct limited_allocator_NotConstructible_ptr__1_ {
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__,
}
impl limited_allocator_NotConstructible_ptr__1_ {
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut *mut NotConstructible {
        return (unsafe {
            limited_alloc_handle_1_::allocate_1_class_NotConstructible_ptr(
                &mut (*(unsafe {
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__::operator_arrow_7(
                        &mut self.handle_,
                    )
                })),
                n,
            )
        });
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut NotConstructible, mut n: usize) {
        (unsafe {
            limited_alloc_handle_1_::deallocate_2_class_NotConstructible_ptr(
                &mut (*(unsafe {
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__::operator_arrow_7(
                        &mut self.handle_,
                    )
                })),
                p,
                n,
            )
        });
    }
}
impl limited_allocator_NotConstructible_ptr__1_ {
    pub unsafe fn destructor(&mut self) {
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_1__::destructor(&mut self.handle_);
    }
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct NotConstructible {}
impl NotConstructible {}
impl std::cmp::PartialEq for NotConstructible {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_5(
                self as *const NotConstructible,
                other as *const NotConstructible,
            )
        }
    }
}
impl std::cmp::Eq for NotConstructible {}
pub unsafe fn operator_eq_5(_a0: *const NotConstructible, _a1: *const NotConstructible) -> bool {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_7))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_8)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_7))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_8))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_9)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_6)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_7)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_8)) = 0_usize;
    }
}
pub static mut outstanding_bytes_6: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_7: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_8: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_9: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_sub_10(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
}
pub unsafe fn operator_eq_11(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_12(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_11(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut i32,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut i32) -> Self {
        let mut this = Self { ptr_: p };
        this
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
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: isize,
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
        unsafe { operator_eq_11(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_8();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_13(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize)
        / ::std::mem::size_of::<min_pointer_int__std_integral_constant_unsigned_long__0__>())
        as i64 as isize);
}
pub unsafe fn operator_lt_14(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) < (y.ptr_));
}
pub unsafe fn operator_eq_15(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_16(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_15(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut min_pointer_int__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut min_pointer_int__std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn to_bool ( &self , ) -> bool { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_inc_8 ( &mut self , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add_assign ( &mut self , mut n : isize , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_add ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_sub_assign ( &mut self , mut n : isize , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_sub ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add ( &self , mut n : isize , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } pub unsafe fn operator_sub ( &self , mut n : isize , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: operator_sub_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } }
impl std::cmp::Ord for  min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn cmp(&self, other: &Self) -> std::cmp::Ordering { unsafe { if operator_lt_14(self.clone(), other.clone()) { std::cmp::Ordering::Less } else if operator_lt_14(other.clone(), self.clone()) { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Equal } }}}
impl std::cmp::PartialOrd for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }}
impl std::cmp::PartialEq for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_15(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { unsafe fn cpp_inc(&mut self) { self.operator_inc_8(); } }
pub unsafe fn operator_sub_17(
    mut x: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<NotConstructible>()) as i64
        as isize);
}
pub unsafe fn operator_eq_18(
    mut x: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_19(
    mut x: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_18(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut NotConstructible,
}
impl min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut NotConstructible) -> Self {
        let mut this = Self { ptr_: p };
        this
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
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ =
            (*(self
                as *const min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , )
        });
        return tmp;
    }
}
impl std::cmp::PartialEq
    for min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__
{
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_18(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_8();
    }
}
pub unsafe fn operator_sub_20(
    mut x : min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize)
        / ::std::mem::size_of::<
            min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
        >()) as i64 as isize);
}
pub unsafe fn operator_lt_21(
    mut x : min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) < (y.ptr_));
}
pub unsafe fn operator_eq_22(
    mut x : min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_23(
    mut x : min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_22(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn to_bool ( &self , ) -> bool { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_inc_8 ( &mut self , ) -> *mut min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add_assign ( &mut self , mut n : isize , ) -> *mut min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_add ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_sub_assign ( &mut self , mut n : isize , ) -> *mut min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_sub ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add ( &self , mut n : isize , ) -> min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } pub unsafe fn operator_sub ( &self , mut n : isize , ) -> min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: operator_sub_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } }
impl std::cmp::Ord for  min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn cmp(&self, other: &Self) -> std::cmp::Ordering { unsafe { if operator_lt_21(self.clone(), other.clone()) { std::cmp::Ordering::Less } else if operator_lt_21(other.clone(), self.clone()) { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Equal } }}}
impl std::cmp::PartialOrd for min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }}
impl std::cmp::PartialEq for min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_22(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { unsafe fn cpp_inc(&mut self) { self.operator_inc_8(); } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
impl min_allocator_int_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_NotConstructible_ {}
impl min_allocator_NotConstructible_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ {
        return min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn new_1(mut _a0: min_allocator_int_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(min_allocator_int_, new_1, value)]
pub struct min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn allocate ( &mut self , mut n : usize , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
impl min_allocator_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn new_1(mut _a0: min_allocator_NotConstructible_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(min_allocator_NotConstructible_, new_1, value)]
pub struct min_allocator_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___ {
}
impl min_allocator_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn allocate ( &mut self , mut n : usize , ) -> min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___ {}
pub unsafe fn test_24() {
    let mut d: Vec<i32> = Vec::new();
    assert!(((unimplemented!()) == (0_usize)));
    const _: () = assert!(true, "");;
    let mut d1: Vec<i32> = Vec::new();
    assert!(((unimplemented!()) == (0_usize)));
    const _: () = assert!(true, "");;
}
pub unsafe fn test_25() {
    let mut d: Vec<NotConstructible> = Vec::new();
    assert!(((unimplemented!()) == (0_usize)));
    const _: () = assert!(true, "");;
    let mut d1: Vec<NotConstructible> = Vec::new();
    assert!(((unimplemented!()) == (0_usize)));
    const _: () = assert!(true, "");;
}
pub unsafe fn test_26() {
    let mut d: Vec<i32> = Vec::new();
    assert!(((unimplemented!()) == (0_usize)));
    const _: () = assert!(true, "");;
    let mut d1: Vec<i32> = Vec::new();
    assert!(((unimplemented!()) == (0_usize)));
    const _: () = assert!(true, "");;
}
pub unsafe fn test_27() {
    let mut d: Vec<NotConstructible> = Vec::new();
    assert!(((unimplemented!()) == (0_usize)));
    const _: () = assert!(true, "");;
    let mut d1: Vec<NotConstructible> = Vec::new();
    assert!(((unimplemented!()) == (0_usize)));
    const _: () = assert!(true, "");;
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
    (unsafe { test_24() });
    (unsafe { test_25() });
    (unsafe { test_26() });
    (unsafe { test_27() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_0);
    std::cell::LazyCell::force(&*&raw const moved_value_1);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_6);
    std::cell::LazyCell::force(&*&raw const alloc_count_7);
    std::cell::LazyCell::force(&*&raw const dealloc_count_8);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_9);
}
