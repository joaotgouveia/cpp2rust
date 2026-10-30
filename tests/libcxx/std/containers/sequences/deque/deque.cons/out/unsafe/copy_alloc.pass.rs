extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn DoNotOptimize_0(value: *const *mut ::libc::c_void) -> *const *mut ::libc::c_void {
    c"";
    return value;
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
pub static mut destructed_value_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { -1_i32 });
pub static mut moved_value_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 2147483647 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_3(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_4(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_3(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1));
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
            operator_eq_3(
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
pub unsafe fn operator_eq_5(x: *const test_allocator_int_, y: *const test_allocator_int_) -> bool {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_int__rebind_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_int__rebind_int_ptr_ {}
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
                != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1)))
                && (((*a).id_)
                    != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1))))
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
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1)))
        );
        assert!(
            ((self.id_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1));
    }
    pub unsafe fn allocate(
        &mut self,
        mut n: u32,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut i32 {
        let mut _a1: *const ::libc::c_void = _a1.unwrap_or_else(|| unsafe { std::ptr::null() });
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1)))
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
        return (unsafe { <std_allocator_int_>::default().allocate((n as usize)) });
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut i32, mut s: u32) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size =
                (((*self.stats_).allocated_size as u32).wrapping_sub(s)) as i32;
        }
        (unsafe { <std_allocator_int_>::default().deallocate(p, (s as usize)) });
    }
    pub unsafe fn max_size(&self) -> u32 {
        return ((((((2147483647_u32).wrapping_mul((2_u32 as u32)) as u32)
            .wrapping_add((1_u32 as u32))) as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    }
    pub unsafe fn destroy(&mut self, mut p: *mut i32) {
        if !((self.stats_).is_null()) {
            (*self.stats_).destroy_count.prefix_inc();
        };
    }
    pub unsafe fn construct_16_int_ref(&mut self, mut p: *mut i32, val: *mut i32) {
        if !((self.stats_).is_null()) {
            (*self.stats_).construct_count.prefix_inc();
        }
        (unsafe {
            let ___location: *mut i32 = (unsafe { to_address_6(p) });
            let ___args: *mut i32 = val;
            construct_at_7(___location, ___args)
        });
    }
    pub unsafe fn construct_16_const_int_ref(&mut self, mut p: *mut i32, val: *const i32) {
        if !((self.stats_).is_null()) {
            (*self.stats_).construct_count.prefix_inc();
        }
        (unsafe {
            let ___location: *mut i32 = (unsafe { to_address_6(p) });
            let ___args: *const i32 = val;
            construct_at_8(___location, ___args)
        });
    }
}
impl std::cmp::PartialEq for test_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_5(
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
                data_: <i32>::default(),
                id_: 0,
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
impl test_allocator_int_ptr_ {
    pub unsafe fn new_1(a: *const test_allocator_int_) -> Self {
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
#[derive(FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_int_ptr_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_int_ptr_ {
    pub unsafe fn destructor(&mut self) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1)))
        );
        assert!(
            ((self.id_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1));
    }
    pub unsafe fn allocate(
        &mut self,
        mut n: u32,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut *mut i32 {
        let mut _a1: *const ::libc::c_void = _a1.unwrap_or_else(|| unsafe { std::ptr::null() });
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1)))
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
        return (unsafe { <std_allocator_int_ptr_>::default().allocate((n as usize)) });
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut i32, mut s: u32) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size =
                (((*self.stats_).allocated_size as u32).wrapping_sub(s)) as i32;
        }
        (unsafe { <std_allocator_int_ptr_>::default().deallocate(p, (s as usize)) });
    }
}
impl Default for test_allocator_int_ptr_ {
    fn default() -> Self {
        unsafe {
            test_allocator_int_ptr_ {
                data_: <i32>::default(),
                id_: <i32>::default(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_9(
    x: *const other_allocator_int_,
    y: *const other_allocator_int_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct other_allocator_int_ {
    data_: i32,
}
impl other_allocator_int_ {
    pub unsafe fn new_1(mut i: i32) -> Self {
        let mut this = Self { data_: i };
        this
    }
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut i32 {
        return (unsafe { <std_allocator_int_>::default().allocate(n) });
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut i32, mut s: usize) {
        (unsafe { <std_allocator_int_>::default().deallocate(p, s) });
    }
}
impl std::cmp::PartialEq for other_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_9(
                self as *const other_allocator_int_,
                other as *const other_allocator_int_,
            )
        }
    }
}
impl std::cmp::Eq for other_allocator_int_ {}
impl Default for other_allocator_int_ {
    fn default() -> Self {
        unsafe {
            other_allocator_int_ {
                data_: <i32>::default(),
            }
        }
    }
}
impl other_allocator_int_ptr_ {
    pub unsafe fn new_1(a: *const other_allocator_int_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct other_allocator_int_ptr_ {
    data_: i32,
}
impl other_allocator_int_ptr_ {
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut *mut i32 {
        return (unsafe { <std_allocator_int_ptr_>::default().allocate(n) });
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut i32, mut s: usize) {
        (unsafe { <std_allocator_int_ptr_>::default().deallocate(p, s) });
    }
}
impl Default for other_allocator_int_ptr_ {
    fn default() -> Self {
        unsafe {
            other_allocator_int_ptr_ {
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_11))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_12)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_11))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_12))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_13)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_10)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_11)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_12)) = 0_usize;
    }
}
pub static mut outstanding_bytes_10: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_11: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_12: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_13: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_eq_14(mut _a0: min_allocator_int_, mut _a1: min_allocator_int_) -> bool {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
impl min_allocator_int_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            (unsafe { <std_allocator_int_>::default().allocate(n) })
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        (unsafe { <std_allocator_int_>::default().deallocate(p.ptr_, n) });
    }
}
impl std::cmp::PartialEq for min_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_14(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_int_ {}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn new_1(mut _a0: min_allocator_int_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn allocate ( &mut self , mut n : usize , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { ( unsafe { <std_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ >::default() . allocate  ( n , ) } )  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        (unsafe {
            <std_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___>::default()
                .deallocate(p.ptr_, n)
        });
    }
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
pub unsafe fn operator_eq_15(mut _a0: safe_allocator_int_, mut _a1: safe_allocator_int_) -> bool {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct safe_allocator_int_ {}
impl safe_allocator_int_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut i32 {
        let mut memory: *mut i32 = (unsafe { <std_allocator_int_>::default().allocate(n) });
        {
            {
                let byte_0 = (memory as *mut ::libc::c_void) as *mut u8;
                for offset in
                    0..((::std::mem::size_of::<i32>() as u64).wrapping_mul((n as u64)) as usize)
                {
                    *byte_0.offset(offset as isize) = 0 as u8;
                }
                (memory as *mut ::libc::c_void)
            };
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut i32, mut n: usize) {
        {
            (unsafe {
                let mut _value: *mut ::libc::c_void = ({
                    let byte_0 = (p as *mut ::libc::c_void) as *mut u8;
                    for offset in
                        0..((::std::mem::size_of::<i32>() as u64).wrapping_mul((n as u64)) as usize)
                    {
                        *byte_0.offset(offset as isize) = 0 as u8;
                    }
                    (p as *mut ::libc::c_void)
                } as *mut ::libc::c_void);
                DoNotOptimize_0(&mut _value)
            });
        }
        (unsafe { <std_allocator_int_>::default().deallocate(p, n) });
    }
}
impl std::cmp::PartialEq for safe_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_15(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for safe_allocator_int_ {}
impl Default for safe_allocator_int_ {
    fn default() -> Self {
        unsafe { safe_allocator_int_::new() }
    }
}
impl safe_allocator_int_ptr_ {
    pub unsafe fn new_1(mut _a0: safe_allocator_int_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct safe_allocator_int_ptr_ {}
impl safe_allocator_int_ptr_ {
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut *mut i32 {
        let mut memory: *mut *mut i32 =
            (unsafe { <std_allocator_int_ptr_>::default().allocate(n) });
        {
            {
                let byte_0 = (memory as *mut ::libc::c_void) as *mut u8;
                for offset in 0..((::std::mem::size_of::<*mut i32>() as u64)
                    .wrapping_mul((n as u64)) as usize)
                {
                    *byte_0.offset(offset as isize) = 0 as u8;
                }
                (memory as *mut ::libc::c_void)
            };
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut i32, mut n: usize) {
        {
            (unsafe {
                let mut _value: *mut ::libc::c_void = ({
                    let byte_0 = (p as *mut ::libc::c_void) as *mut u8;
                    for offset in 0..((::std::mem::size_of::<*mut i32>() as u64)
                        .wrapping_mul((n as u64)) as usize)
                    {
                        *byte_0.offset(offset as isize) = 0 as u8;
                    }
                    (p as *mut ::libc::c_void)
                } as *mut ::libc::c_void);
                DoNotOptimize_0(&mut _value)
            });
        }
        (unsafe { <std_allocator_int_ptr_>::default().deallocate(p, n) });
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let mut ab: [i32; 15] = [3, 4, 2, 8, 0, 1, 44, 34, 45, 96, 80, 1, 13, 31, 45];
        let mut an: *mut i32 = ab.as_mut_ptr().offset(
            ((::std::mem::size_of::<[i32; 15]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize))) as isize,
        );
        (unsafe {
            let mut _x: std_deque_int__test_allocator_int__ = {
                let mut __tmp_0: test_allocator_int_ = test_allocator_int_::new_1({ 3 });
                std_deque_int__test_allocator_int__::new_1({ ab.as_mut_ptr() }, { an }, {
                    &mut __tmp_0
                })
            };
            let mut _a: test_allocator_int_ = test_allocator_int_::new_1({ 4 });
            test_16(&mut _x, &mut _a)
        });
    }
    {
        let mut ab: [i32; 15] = [3, 4, 2, 8, 0, 1, 44, 34, 45, 96, 80, 1, 13, 31, 45];
        let mut an: *mut i32 = ab.as_mut_ptr().offset(
            ((::std::mem::size_of::<[i32; 15]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize))) as isize,
        );
        (unsafe {
            let mut _x: std_deque_int__other_allocator_int__ = {
                let mut __tmp_1: other_allocator_int_ = other_allocator_int_::new_1({ 3 });
                std_deque_int__other_allocator_int__::new_1({ ab.as_mut_ptr() }, { an }, {
                    &mut __tmp_1
                })
            };
            let mut _a: other_allocator_int_ = other_allocator_int_::new_1({ 4 });
            test_17(&mut _x, &mut _a)
        });
    }
    {
        let mut ab: [i32; 15] = [3, 4, 2, 8, 0, 1, 44, 34, 45, 96, 80, 1, 13, 31, 45];
        let mut an: *mut i32 = ab.as_mut_ptr().offset(
            ((::std::mem::size_of::<[i32; 15]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize))) as isize,
        );
        (unsafe {
            let mut _x: std_deque_int__min_allocator_int__ = {
                let mut __tmp_2: min_allocator_int_ = <min_allocator_int_>::default();
                std_deque_int__min_allocator_int__::new_1({ ab.as_mut_ptr() }, { an }, {
                    &mut __tmp_2
                })
            };
            let mut _a: min_allocator_int_ = <min_allocator_int_>::default();
            test_18(&mut _x, &mut _a)
        });
    }
    {
        let mut ab: [i32; 15] = [3, 4, 2, 8, 0, 1, 44, 34, 45, 96, 80, 1, 13, 31, 45];
        let mut an: *mut i32 = ab.as_mut_ptr().offset(
            ((::std::mem::size_of::<[i32; 15]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize))) as isize,
        );
        (unsafe {
            let mut _x: std_deque_int__safe_allocator_int__ = {
                let mut __tmp_3: safe_allocator_int_ = safe_allocator_int_::new();
                std_deque_int__safe_allocator_int__::new_1({ ab.as_mut_ptr() }, { an }, {
                    &mut __tmp_3
                })
            };
            let mut _a: safe_allocator_int_ = safe_allocator_int_::new();
            test_19(&mut _x, &mut _a)
        });
    }
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_1);
    std::cell::LazyCell::force(&*&raw const moved_value_2);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_10);
    std::cell::LazyCell::force(&*&raw const alloc_count_11);
    std::cell::LazyCell::force(&*&raw const dealloc_count_12);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_13);
}
