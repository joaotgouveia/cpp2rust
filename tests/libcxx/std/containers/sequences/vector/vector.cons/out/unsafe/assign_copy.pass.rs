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
pub unsafe fn operator_ne_6(x: *const test_allocator_int_, y: *const test_allocator_int_) -> bool {
    return !(unsafe {
        let _x: *const test_allocator_int_ = x;
        let _y: *const test_allocator_int_ = y;
        operator_eq_5(_x, _y)
    });
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
        return unimplemented!();
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
        unimplemented!();
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
    pub unsafe fn construct_16_const_int_ref(&mut self, mut p: *mut i32, val: *const i32) {
        if !((self.stats_).is_null()) {
            (*self.stats_).construct_count.prefix_inc();
        }
        unimplemented!();
    }
    pub unsafe fn construct_16_int_ref(&mut self, mut p: *mut i32, val: *mut i32) {
        if !((self.stats_).is_null()) {
            (*self.stats_).construct_count.prefix_inc();
        }
        unimplemented!();
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
                data_: 0_i32,
                id_: 0,
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_7(
    x: *const other_allocator_int_,
    y: *const other_allocator_int_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_8(
    x: *const other_allocator_int_,
    y: *const other_allocator_int_,
) -> bool {
    return !(unsafe {
        let _x: *const other_allocator_int_ = x;
        let _y: *const other_allocator_int_ = y;
        operator_eq_7(_x, _y)
    });
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
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut i32, mut s: usize) {
        unimplemented!();
    }
}
impl std::cmp::PartialEq for other_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_7(
                self as *const other_allocator_int_,
                other as *const other_allocator_int_,
            )
        }
    }
}
impl std::cmp::Eq for other_allocator_int_ {}
impl Default for other_allocator_int_ {
    fn default() -> Self {
        unsafe { other_allocator_int_ { data_: 0_i32 } }
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_10))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_11)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_10))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_11))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_12)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_9)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_10)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_11)) = 0_usize;
    }
}
pub static mut outstanding_bytes_9: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_10: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_11: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_12: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_sub_13(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
}
pub unsafe fn operator_eq_14(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_15(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_14(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(*mut ::libc::c_void , new_1, value)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut i32,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_2(mut p: *mut i32) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn new_1(mut _a0: *mut ::libc::c_void) -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
        this
    }
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
        unsafe { operator_eq_14(self.clone(), other.clone()) }
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
pub unsafe fn operator_eq_16(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_17(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) != (y.ptr_));
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
    pub unsafe fn operator_deref(&self) -> *const i32 {
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
impl UnsafeCppAdvance for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_9();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *const ::libc::c_void,
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_18(mut _a0: min_allocator_int_, mut _a1: min_allocator_int_) -> bool {
    return true;
}
pub unsafe fn operator_ne_19(mut x: min_allocator_int_, mut y: min_allocator_int_) -> bool {
    return !(unsafe {
        let _arg0: min_allocator_int_ = x;
        operator_eq_18(_arg0, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
impl min_allocator_int_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_2({
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
impl std::cmp::PartialEq for min_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_18(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_int_ {}
pub unsafe fn operator_eq_20(mut _a0: safe_allocator_int_, mut _a1: safe_allocator_int_) -> bool {
    return true;
}
pub unsafe fn operator_ne_21(mut x: safe_allocator_int_, mut y: safe_allocator_int_) -> bool {
    return !(unsafe {
        let _arg0: safe_allocator_int_ = x;
        operator_eq_20(_arg0, y)
    });
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
        let mut memory: *mut i32 = unimplemented!();
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
        unimplemented!();
    }
}
impl std::cmp::PartialEq for safe_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_20(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for safe_allocator_int_ {}
impl Default for safe_allocator_int_ {
    fn default() -> Self {
        unsafe { safe_allocator_int_::new() }
    }
}
pub unsafe fn operator_eq_22(
    lhs: *const MaybePOCCAAllocator_int__false_,
    rhs: *const MaybePOCCAAllocator_int__false_,
) -> bool {
    return ((unsafe { MaybePOCCAAllocator_int__false_::id(&(*lhs)) })
        == (unsafe { MaybePOCCAAllocator_int__false_::id(&(*rhs)) }));
}
pub unsafe fn operator_ne_23(
    lhs: *const MaybePOCCAAllocator_int__false_,
    rhs: *const MaybePOCCAAllocator_int__false_,
) -> bool {
    return !(unsafe {
        let _lhs: *const MaybePOCCAAllocator_int__false_ = lhs;
        let _rhs: *const MaybePOCCAAllocator_int__false_ = rhs;
        operator_eq_22(_lhs, _rhs)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MaybePOCCAAllocator_int__false__rebind_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct MaybePOCCAAllocator_int__false_ {
    id_: i32,
    copy_assigned_into_: *mut bool,
}
impl MaybePOCCAAllocator_int__false_ {
    pub unsafe fn new_1(mut id: i32, mut copy_assigned_into: *mut bool) -> Self {
        let mut this = Self {
            id_: id,
            copy_assigned_into_: copy_assigned_into,
        };
        this
    }
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut i32 {
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut ptr: *mut i32, mut n: usize) {
        unimplemented!();
    }
    pub unsafe fn id(&self) -> i32 {
        return self.id_;
    }
}
impl std::cmp::PartialEq for MaybePOCCAAllocator_int__false_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_22(
                self as *const MaybePOCCAAllocator_int__false_,
                other as *const MaybePOCCAAllocator_int__false_,
            )
        }
    }
}
impl std::cmp::Eq for MaybePOCCAAllocator_int__false_ {}
impl Default for MaybePOCCAAllocator_int__false_ {
    fn default() -> Self {
        unsafe {
            MaybePOCCAAllocator_int__false_ {
                id_: 0,
                copy_assigned_into_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_24(
    lhs: *const MaybePOCCAAllocator_int__true_,
    rhs: *const MaybePOCCAAllocator_int__true_,
) -> bool {
    return ((unsafe { MaybePOCCAAllocator_int__true_::id(&(*lhs)) })
        == (unsafe { MaybePOCCAAllocator_int__true_::id(&(*rhs)) }));
}
pub unsafe fn operator_ne_25(
    lhs: *const MaybePOCCAAllocator_int__true_,
    rhs: *const MaybePOCCAAllocator_int__true_,
) -> bool {
    return !(unsafe {
        let _lhs: *const MaybePOCCAAllocator_int__true_ = lhs;
        let _rhs: *const MaybePOCCAAllocator_int__true_ = rhs;
        operator_eq_24(_lhs, _rhs)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MaybePOCCAAllocator_int__true__rebind_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CopyAssignUnsafe)]
pub struct MaybePOCCAAllocator_int__true_ {
    id_: i32,
    copy_assigned_into_: *mut bool,
}
impl MaybePOCCAAllocator_int__true_ {
    pub unsafe fn new_1(mut id: i32, mut copy_assigned_into: *mut bool) -> Self {
        let mut this = Self {
            id_: id,
            copy_assigned_into_: copy_assigned_into,
        };
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        a: *const MaybePOCCAAllocator_int__true_,
    ) -> *mut MaybePOCCAAllocator_int__true_ {
        self.id_ = (unsafe { MaybePOCCAAllocator_int__true_::id(&(*a)) });
        if !(self.copy_assigned_into_).is_null() {
            (*self.copy_assigned_into_) = true;
        }
        return &mut (*(self as *mut MaybePOCCAAllocator_int__true_));
    }
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut i32 {
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut ptr: *mut i32, mut n: usize) {
        unimplemented!();
    }
    pub unsafe fn id(&self) -> i32 {
        return self.id_;
    }
}
impl std::cmp::PartialEq for MaybePOCCAAllocator_int__true_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_24(
                self as *const MaybePOCCAAllocator_int__true_,
                other as *const MaybePOCCAAllocator_int__true_,
            )
        }
    }
}
impl std::cmp::Eq for MaybePOCCAAllocator_int__true_ {}
impl Default for MaybePOCCAAllocator_int__true_ {
    fn default() -> Self {
        unsafe {
            MaybePOCCAAllocator_int__true_ {
                id_: 0,
                copy_assigned_into_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn tests_26() -> bool {
    {
        let mut l: Vec<i32> = vec![2; 3_usize as usize];
        let mut l2: Vec<i32> = unimplemented!();
        l2 = l.clone();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let mut _x: test_allocator_int_ = unimplemented!();
                let mut _y: test_allocator_int_ = test_allocator_int_::new_1({ 3 });
                operator_eq_5(&mut _x, &mut _y)
            })
        );
    }
    {
        let mut l: Vec<i32> = vec![2; 3_usize as usize];
        let mut l2: Vec<i32> = unimplemented!();
        l2 = l.clone();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let mut _x: other_allocator_int_ = unimplemented!();
                let mut _y: other_allocator_int_ = other_allocator_int_::new_1({ 5 });
                operator_eq_7(&mut _x, &mut _y)
            })
        );
    }
    {
        let mut copy_assigned_into: bool = false;
        let mut l: Vec<i32> = vec![2; 3_usize as usize];
        let mut l2: Vec<i32> = unimplemented!();
        assert!(!(copy_assigned_into));
        l2 = l.clone();
        assert!(!(copy_assigned_into));
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let mut _lhs: MaybePOCCAAllocator_int__false_ = unimplemented!();
                let mut _rhs: MaybePOCCAAllocator_int__false_ =
                    MaybePOCCAAllocator_int__false_::new_1({ 3 }, { std::ptr::null_mut() });
                operator_eq_22(&mut _lhs, &mut _rhs)
            })
        );
    }
    {
        let mut copy_assigned_into: bool = false;
        let mut l: Vec<i32> = vec![2; 3_usize as usize];
        let mut l2: Vec<i32> = unimplemented!();
        assert!(!(copy_assigned_into));
        l2 = l.clone();
        assert!(copy_assigned_into);
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let mut _lhs: MaybePOCCAAllocator_int__true_ = unimplemented!();
                let mut _rhs: MaybePOCCAAllocator_int__true_ =
                    MaybePOCCAAllocator_int__true_::new_1({ 5 }, { std::ptr::null_mut() });
                operator_eq_24(&mut _lhs, &mut _rhs)
            })
        );
    }
    {
        let mut copy_assigned_into: bool = false;
        let mut l: Vec<i32> = vec![2; 3_usize as usize];
        let mut l2: Vec<i32> = unimplemented!();
        assert!(!(copy_assigned_into));
        l2 = l.clone();
        assert!(copy_assigned_into);
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let mut _lhs: MaybePOCCAAllocator_int__true_ = unimplemented!();
                let mut _rhs: MaybePOCCAAllocator_int__true_ =
                    MaybePOCCAAllocator_int__true_::new_1({ 5 }, { std::ptr::null_mut() });
                operator_eq_24(&mut _lhs, &mut _rhs)
            })
        );
    }
    {
        let mut l: Vec<i32> = vec![2; 3_usize as usize];
        let mut l2: Vec<i32> = unimplemented!();
        l2 = l.clone();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let _arg0: min_allocator_int_ = unimplemented!();
                operator_eq_18(_arg0, <min_allocator_int_>::default())
            })
        );
    }
    {
        let mut l: Vec<i32> = vec![2; 3_usize as usize];
        let mut l2: Vec<i32> = unimplemented!();
        l2 = l.clone();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let _arg0: safe_allocator_int_ = unimplemented!();
                operator_eq_20(_arg0, safe_allocator_int_::new())
            })
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
    (unsafe { tests_26() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_1);
    std::cell::LazyCell::force(&*&raw const moved_value_2);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_9);
    std::cell::LazyCell::force(&*&raw const alloc_count_10);
    std::cell::LazyCell::force(&*&raw const dealloc_count_11);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_12);
}
