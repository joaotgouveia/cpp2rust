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
pub struct min_allocator_bool_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_unsigned_long_ {}
impl min_allocator_unsigned_long_ {
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        (unsafe {
            let ___p: *mut u64 = p.ptr_;
            let ___n: usize = n;
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_bool__unsigned_char__signed_char_ {
    data_: i32,
}
impl sized_allocator_bool__unsigned_char__signed_char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct sized_allocator_unsigned_long__unsigned_char__signed_char_ {
    data_: i32,
}
impl sized_allocator_unsigned_long__unsigned_char__signed_char_ {
    pub unsafe fn new(mut d: Option<i32>) -> Self {
        let mut d: i32 = d.unwrap_or_else(|| unsafe { 0 });
        let mut this = Self { data_: d };
        this
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut u64, mut n: u8) {
        (unsafe {
            let ___p: *mut u64 = p;
            let ___n: usize = (n as usize);
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
    pub unsafe fn max_size(&self) -> u8 {
        return (((<u8>::MAX as usize).wrapping_div((::std::mem::size_of::<u64>() as usize)))
            as u8);
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
    data_: i32,
}
impl sized_allocator_bool__unsigned_short__short_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct sized_allocator_unsigned_long__unsigned_short__short_ {
    data_: i32,
}
impl sized_allocator_unsigned_long__unsigned_short__short_ {
    pub unsafe fn new(mut d: Option<i32>) -> Self {
        let mut d: i32 = d.unwrap_or_else(|| unsafe { 0 });
        let mut this = Self { data_: d };
        this
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut u64, mut n: u16) {
        (unsafe {
            let ___p: *mut u64 = p;
            let ___n: usize = (n as usize);
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
    pub unsafe fn max_size(&self) -> u16 {
        return (((<u16>::MAX as usize).wrapping_div((::std::mem::size_of::<u64>() as usize)))
            as u16);
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
    data_: i32,
}
impl sized_allocator_bool__unsigned_int__int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct sized_allocator_unsigned_long__unsigned_int__int_ {
    data_: i32,
}
impl sized_allocator_unsigned_long__unsigned_int__int_ {
    pub unsafe fn new(mut d: Option<i32>) -> Self {
        let mut d: i32 = d.unwrap_or_else(|| unsafe { 0 });
        let mut this = Self { data_: d };
        this
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut u64, mut n: u32) {
        (unsafe {
            let ___p: *mut u64 = p;
            let ___n: usize = (n as usize);
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
    pub unsafe fn max_size(&self) -> u32 {
        return (((<u32>::MAX as usize).wrapping_div((::std::mem::size_of::<u64>() as usize)))
            as u32);
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
    data_: i32,
}
impl sized_allocator_bool__unsigned_long__long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct sized_allocator_unsigned_long__unsigned_long__long_ {
    data_: i32,
}
impl sized_allocator_unsigned_long__unsigned_long__long_ {
    pub unsafe fn new(mut d: Option<i32>) -> Self {
        let mut d: i32 = d.unwrap_or_else(|| unsafe { 0 });
        let mut this = Self { data_: d };
        this
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut u64, mut n: u64) {
        (unsafe {
            let ___p: *mut u64 = p;
            let ___n: usize = (n as usize);
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
    pub unsafe fn max_size(&self) -> u64 {
        return (<u64>::MAX as u64).wrapping_div((::std::mem::size_of::<u64>() as u64));
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_bool__rebind_unsigned_long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_bool__rebind_bool_ {}
#[repr(C)]
#[derive(FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_bool_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_bool_ {
    pub unsafe fn new_1(mut data: i32) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: std::ptr::null_mut(),
        };
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
impl Default for test_allocator_bool_ {
    fn default() -> Self {
        unsafe {
            test_allocator_bool_ {
                data_: <i32>::default(),
                id_: 0,
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
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_unsigned_long_ {
    pub unsafe fn copy_from(a: *const test_allocator_unsigned_long_) -> Self {
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
    pub unsafe fn deallocate(&mut self, mut p: *mut u64, mut s: u32) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size =
                (((*self.stats_).allocated_size as u32).wrapping_sub(s)) as i32;
        }
        (unsafe {
            let ___p: *mut u64 = p;
            let ___n: usize = (s as usize);
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
    pub unsafe fn max_size(&self) -> u32 {
        return ((((((2147483647_u32).wrapping_mul((2_u32 as u32)) as u32)
            .wrapping_add((1_u32 as u32))) as usize)
            .wrapping_div((::std::mem::size_of::<u64>() as usize))) as u32);
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
                data_: <i32>::default(),
                id_: <i32>::default(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct other_allocator_bool_ {
    data_: i32,
}
impl other_allocator_bool_ {
    pub unsafe fn new_1(mut i: i32) -> Self {
        let mut this = Self { data_: i };
        this
    }
}
impl Default for other_allocator_bool_ {
    fn default() -> Self {
        unsafe {
            other_allocator_bool_ {
                data_: <i32>::default(),
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
    data_: i32,
}
impl other_allocator_unsigned_long_ {
    pub unsafe fn deallocate(&mut self, mut p: *mut u64, mut s: usize) {
        (unsafe {
            let ___p: *mut u64 = p;
            let ___n: usize = s;
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
}
impl Default for other_allocator_unsigned_long_ {
    fn default() -> Self {
        unsafe {
            other_allocator_unsigned_long_ {
                data_: <i32>::default(),
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
    pub outstanding_: usize,
    pub last_alloc_: *mut ::libc::c_void,
}
impl limited_alloc_handle_18446744073709551615_ {
    pub unsafe fn deallocate_2_unsigned_long(&mut self, mut ptr: *mut u64, mut N: usize) {
        if ((ptr as *mut ::libc::c_void) == (self.last_alloc_)) {
            self.last_alloc_ = std::ptr::null_mut();
            assert!(((self.outstanding_) >= (N)));
            self.outstanding_ = (self.outstanding_).wrapping_sub(N);
        }
        (unsafe {
            let ___p: *mut u64 = ptr;
            let ___n: usize = N;
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
}
impl Default for limited_alloc_handle_18446744073709551615_ {
    fn default() -> Self {
        unsafe {
            limited_alloc_handle_18446744073709551615_ {
                outstanding_: 0_usize,
                last_alloc_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct limited_alloc_handle_302405640552615600_ {
    pub outstanding_: usize,
    pub last_alloc_: *mut ::libc::c_void,
}
impl limited_alloc_handle_302405640552615600_ {
    pub unsafe fn deallocate_2_unsigned_long(&mut self, mut ptr: *mut u64, mut N: usize) {
        if ((ptr as *mut ::libc::c_void) == (self.last_alloc_)) {
            self.last_alloc_ = std::ptr::null_mut();
            assert!(((self.outstanding_) >= (N)));
            self.outstanding_ = (self.outstanding_).wrapping_sub(N);
        }
        (unsafe {
            let ___p: *mut u64 = ptr;
            let ___n: usize = N;
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
}
impl Default for limited_alloc_handle_302405640552615600_ {
    fn default() -> Self {
        unsafe {
            limited_alloc_handle_302405640552615600_ {
                outstanding_: 0_usize,
                last_alloc_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct limited_alloc_handle_292805461487453200_ {
    pub outstanding_: usize,
    pub last_alloc_: *mut ::libc::c_void,
}
impl limited_alloc_handle_292805461487453200_ {
    pub unsafe fn deallocate_2_unsigned_long(&mut self, mut ptr: *mut u64, mut N: usize) {
        if ((ptr as *mut ::libc::c_void) == (self.last_alloc_)) {
            self.last_alloc_ = std::ptr::null_mut();
            assert!(((self.outstanding_) >= (N)));
            self.outstanding_ = (self.outstanding_).wrapping_sub(N);
        }
        (unsafe {
            let ___p: *mut u64 = ptr;
            let ___n: usize = N;
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
}
impl Default for limited_alloc_handle_292805461487453200_ {
    fn default() -> Self {
        unsafe {
            limited_alloc_handle_292805461487453200_ {
                outstanding_: 0_usize,
                last_alloc_: std::ptr::null_mut(),
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
    let mut alloc : std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block_ = <std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block_ >::default() ;
    ptr.block = (unsafe {
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block__::allocate_1 ( & mut alloc , 1_usize , )
    });
    (unsafe {
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block__::construct ( & mut alloc , ptr . block  , )
    });
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
    let mut alloc : std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block_ = <std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block_ >::default() ;
    ptr.block = (unsafe {
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block__::allocate_1 ( & mut alloc , 1_usize , )
    });
    (unsafe {
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block__::construct ( & mut alloc , ptr . block  , )
    });
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
    let mut alloc : std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block_ = <std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block_ >::default() ;
    ptr.block = (unsafe {
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block__::allocate_1 ( & mut alloc , 1_usize , )
    });
    (unsafe {
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block__::construct ( & mut alloc , ptr . block  , )
    });
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__::copy_from({
        &ptr
    });
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            ref_count: 1_usize,
            content: <limited_alloc_handle_18446744073709551615_>::default(),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block {
    pub ref_count: usize,
    pub content: limited_alloc_handle_18446744073709551615_,
}
impl Default
    for detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block
{
    fn default() -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block { ref_count : 1_usize , content : <limited_alloc_handle_18446744073709551615_ >::default() , }
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
        if (((*self.block).ref_count) != (0_usize)) {
            return;
        };
        let mut alloc : std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block_ = <std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block_ >::default() ;
        (unsafe {
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block__::destroy ( & mut alloc , self . block  , )
        });
        (unsafe {
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block__::deallocate ( & mut alloc , self . block  , 1_usize , )
        });
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
            ref_count: 1_usize,
            content: <limited_alloc_handle_302405640552615600_>::default(),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block {
    pub ref_count: usize,
    pub content: limited_alloc_handle_302405640552615600_,
}
impl Default
    for detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block
{
    fn default() -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block { ref_count : 1_usize , content : <limited_alloc_handle_302405640552615600_ >::default() , }
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
        if (((*self.block).ref_count) != (0_usize)) {
            return;
        };
        let mut alloc : std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block_ = <std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block_ >::default() ;
        (unsafe {
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block__::destroy ( & mut alloc , self . block  , )
        });
        (unsafe {
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block__::deallocate ( & mut alloc , self . block  , 1_usize , )
        });
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
            ref_count: 1_usize,
            content: <limited_alloc_handle_292805461487453200_>::default(),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block {
    pub ref_count: usize,
    pub content: limited_alloc_handle_292805461487453200_,
}
impl Default
    for detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block
{
    fn default() -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block { ref_count : 1_usize , content : <limited_alloc_handle_292805461487453200_ >::default() , }
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
        if (((*self.block).ref_count) != (0_usize)) {
            return;
        };
        let mut alloc : std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block_ = <std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block_ >::default() ;
        (unsafe {
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block__::destroy ( & mut alloc , self . block  , )
        });
        (unsafe {
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block__::deallocate ( & mut alloc , self . block  , 1_usize , )
        });
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
    pub unsafe fn deallocate(&mut self, mut p: *mut u64, mut n: usize) {
        (unsafe {
            let _ptr: *mut u64 = p;
            let _N: usize = n;
            limited_alloc_handle_18446744073709551615_::deallocate_2_unsigned_long(
                &mut (*(unsafe {
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ :: operator_arrow_7 ( &mut self . handle_   , )
                })),
                _ptr,
                _N,
            )
        });
    }
    pub unsafe fn max_size(&self) -> usize {
        return (18446744073709551615_u64 as usize);
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
    pub unsafe fn deallocate(&mut self, mut p: *mut u64, mut n: usize) {
        (unsafe {
            let _ptr: *mut u64 = p;
            let _N: usize = n;
            limited_alloc_handle_302405640552615600_::deallocate_2_unsigned_long(
                &mut (*(unsafe {
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__ :: operator_arrow_7 ( &mut self . handle_   , )
                })),
                _ptr,
                _N,
            )
        });
    }
    pub unsafe fn max_size(&self) -> usize {
        return (302405640552615600_u64 as usize);
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
    pub unsafe fn deallocate(&mut self, mut p: *mut u64, mut n: usize) {
        (unsafe {
            let _ptr: *mut u64 = p;
            let _N: usize = n;
            limited_alloc_handle_292805461487453200_::deallocate_2_unsigned_long(
                &mut (*(unsafe {
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__ :: operator_arrow_7 ( &mut self . handle_   , )
                })),
                _ptr,
                _N,
            )
        });
    }
    pub unsafe fn max_size(&self) -> usize {
        return (292805461487453200_u64 as usize);
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
pub unsafe fn test_11(v: *const Vec<bool>) {
    let max_dist: usize = (<i64>::MAX as usize);
    assert!(((unsafe { (*v).max_size() }) <= (max_dist)));
}
pub unsafe fn test_12(v: *const Vec<bool>) {
    let max_dist: usize = (<i64>::MAX as usize);
    assert!(((unsafe { (*v).max_size() }) <= (max_dist)));
}
pub unsafe fn test_13(v: *const Vec<bool>) {
    let max_dist: usize = (<i64>::MAX as usize);
    assert!(((unsafe { (*v).max_size() }) <= (max_dist)));
}
pub unsafe fn test_14(v: *const Vec<bool>) {
    let max_dist: usize = (<i64>::MAX as usize);
    assert!(((unsafe { (*v).max_size() }) <= (max_dist)));
}
pub unsafe fn test_15(v: *const Vec<bool>) {
    let max_dist: usize = (<i64>::MAX as usize);
    assert!(((unsafe { (*v).max_size() }) <= (max_dist)));
}
pub unsafe fn test_16(v: *const Vec<bool>) {
    let max_dist: usize = (<i64>::MAX as usize);
    assert!(((unsafe { (*v).max_size() }) <= (max_dist)));
}
pub unsafe fn test_17(v: *const Vec<bool>) {
    let max_dist: usize = (<i64>::MAX as usize);
    assert!(((unsafe { (*v).max_size() }) <= (max_dist)));
}
pub unsafe fn test_18(v: *const Vec<bool>) {
    let max_dist: usize = (<i64>::MAX as usize);
    assert!(((unsafe { (*v).max_size() }) <= (max_dist)));
}
pub unsafe fn test_19(v: *const Vec<bool>) {
    let max_dist: usize = (<i64>::MAX as usize);
    assert!(((unsafe { (*v).max_size() }) <= (max_dist)));
}
pub unsafe fn test_20(v: *const Vec<bool>) {
    let max_dist: usize = (<i64>::MAX as usize);
    assert!(((unsafe { (*v).max_size() }) <= (max_dist)));
}
pub unsafe fn tests_21() -> bool {
    {
        (unsafe {
            let mut _v: Vec<bool> = Vec::new();
            test_22(&mut _v)
        });
        (unsafe {
            let mut _v: Vec<bool> = Vec::new();
            test_22(&mut _v)
        });
        (unsafe {
            let mut _v: Vec<bool> = Vec::new();
            test_11(&mut _v)
        });
        (unsafe {
            let mut _v: Vec<bool> = {
                let mut __tmp_0: test_allocator_bool_ = test_allocator_bool_::new_1({ 1 });
                std_vector_bool__test_allocator_bool__::new_1({ &mut __tmp_0 })
            };
            test_12(&mut _v)
        });
        (unsafe {
            let mut _v: Vec<bool> = {
                let mut __tmp_1: other_allocator_bool_ = other_allocator_bool_::new_1({ 5 });
                std_vector_bool__other_allocator_bool__::new_1({ &mut __tmp_1 })
            };
            test_13(&mut _v)
        });
        (unsafe {
            let mut _v: Vec<bool> = Vec::new();
            test_14(&mut _v)
        });
        (unsafe {
            let mut _v: Vec<bool> = Vec::new();
            test_15(&mut _v)
        });
        (unsafe {
            let mut _v: Vec<bool> = Vec::new();
            test_16(&mut _v)
        });
        (unsafe {
            let mut _v: Vec<bool> = Vec::new();
            test_17(&mut _v)
        });
        (unsafe {
            let mut _v: Vec<bool> = Vec::new();
            test_18(&mut _v)
        });
    }
    {
        (unsafe {
            let mut _v: Vec<bool> = Vec::new();
            test_19(&mut _v)
        });
        (unsafe {
            let mut _v: Vec<bool> = Vec::new();
            test_20(&mut _v)
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { tests_21() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
    std::cell::LazyCell::force(&*&raw const destructed_value_4);
    std::cell::LazyCell::force(&*&raw const moved_value_5);
}
