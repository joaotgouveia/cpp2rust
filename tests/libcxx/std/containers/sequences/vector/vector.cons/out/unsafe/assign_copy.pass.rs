extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn DoNotOptimize_0(value: *const ()) -> *const () {
    c"";
    return value;
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
pub static mut destructed_value_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_3(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_4(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_3(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1));
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
                data_: 0_(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_5(x: *const test_allocator_int_, y: *const test_allocator_int_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_6(x: *const test_allocator_int_, y: *const test_allocator_int_) -> () {
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
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_int_ {
    pub unsafe fn new_1(mut data: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1));
    }
    pub unsafe fn allocate(&mut self, mut n: (), mut _a1: Option<()>) -> *mut () {
        let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
        unimplemented!();
        if !((self.stats_).is_null()) {
            if (((*self.stats_).time_to_throw) >= ((*self.stats_).throw_after)) {
                unimplemented!();
            }
            (*self.stats_).time_to_throw.prefix_inc();
            (*self.stats_).alloc_count.prefix_inc();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_add(n);
        }
        return unimplemented!();
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
    pub unsafe fn destroy(&mut self, mut p: *mut ()) {
        if !((self.stats_).is_null()) {
            (*self.stats_).destroy_count.prefix_inc();
        };
    }
    pub unsafe fn construct_16_const_int_ref(&mut self, mut p: *mut (), val: *const ()) {
        if !((self.stats_).is_null()) {
            (*self.stats_).construct_count.prefix_inc();
        }
        unimplemented!();
    }
    pub unsafe fn construct_16_int_ref(&mut self, mut p: *mut (), val: *mut ()) {
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
                data_: unimplemented!(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_7(x: *const other_allocator_int_, y: *const other_allocator_int_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_8(x: *const other_allocator_int_, y: *const other_allocator_int_) -> () {
    return !(unsafe {
        let _x: *const other_allocator_int_ = x;
        let _y: *const other_allocator_int_ = y;
        operator_eq_7(_x, _y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct other_allocator_int_ {
    data_: (),
}
impl other_allocator_int_ {
    pub unsafe fn new_1(mut i: ()) -> Self {
        let mut this = Self { data_: i };
        this
    }
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut s: ()) {
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
        unsafe {
            other_allocator_int_ {
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_10))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_11))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_12)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_9)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_10)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_11)) = 0_();
    }
}
pub static mut outstanding_bytes_9: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_10: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_12: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_sub_13(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_14(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_15(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_14(_x, y)
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
    pub unsafe fn to_(&self) -> () {
        return !((self.ptr_).is_null());
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
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
        unsafe { operator_eq_14(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_16(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_17(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) != (y.ptr_));
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
pub unsafe fn operator_eq_18(mut _a0: min_allocator_int_, mut _a1: min_allocator_int_) -> () {
    return true;
}
pub unsafe fn operator_ne_19(mut x: min_allocator_int_, mut y: min_allocator_int_) -> () {
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
impl std::cmp::PartialEq for min_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_18(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_int_ {}
pub unsafe fn operator_eq_20(mut _a0: safe_allocator_int_, mut _a1: safe_allocator_int_) -> () {
    return true;
}
pub unsafe fn operator_ne_21(mut x: safe_allocator_int_, mut y: safe_allocator_int_) -> () {
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
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        let mut memory: *mut () = unimplemented!();
        {
            unimplemented!();
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        {
            (unsafe {
                let mut _value: () = unimplemented!();
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
) -> () {
    return ((unsafe { MaybePOCCAAllocator_int__false_::id(&(*lhs)) })
        == (unsafe { MaybePOCCAAllocator_int__false_::id(&(*rhs)) }));
}
pub unsafe fn operator_ne_23(
    lhs: *const MaybePOCCAAllocator_int__false_,
    rhs: *const MaybePOCCAAllocator_int__false_,
) -> () {
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
    id_: (),
    copy_assigned_into_: *mut (),
}
impl MaybePOCCAAllocator_int__false_ {
    pub unsafe fn new_1(mut id: (), mut copy_assigned_into: *mut ()) -> Self {
        let mut this = Self {
            id_: id,
            copy_assigned_into_: copy_assigned_into,
        };
        this
    }
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut ptr: *mut (), mut n: ()) {
        unimplemented!();
    }
    pub unsafe fn id(&self) -> () {
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
                id_: 0_(),
                copy_assigned_into_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_24(
    lhs: *const MaybePOCCAAllocator_int__true_,
    rhs: *const MaybePOCCAAllocator_int__true_,
) -> () {
    return ((unsafe { MaybePOCCAAllocator_int__true_::id(&(*lhs)) })
        == (unsafe { MaybePOCCAAllocator_int__true_::id(&(*rhs)) }));
}
pub unsafe fn operator_ne_25(
    lhs: *const MaybePOCCAAllocator_int__true_,
    rhs: *const MaybePOCCAAllocator_int__true_,
) -> () {
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
    id_: (),
    copy_assigned_into_: *mut (),
}
impl MaybePOCCAAllocator_int__true_ {
    pub unsafe fn new_1(mut id: (), mut copy_assigned_into: *mut ()) -> Self {
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
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut ptr: *mut (), mut n: ()) {
        unimplemented!();
    }
    pub unsafe fn id(&self) -> () {
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
                id_: 0_(),
                copy_assigned_into_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn tests_26() -> () {
    {
        let mut l: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut l: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut copy_assigned_into: () = false;
        let mut l: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut copy_assigned_into: () = false;
        let mut l: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut copy_assigned_into: () = false;
        let mut l: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut l: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut l: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
    (unsafe { tests_26() });
    const _: () = assert!(true, "tests()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_1);
    std::cell::LazyCell::force(&*&raw const moved_value_2);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_9);
    std::cell::LazyCell::force(&*&raw const alloc_count_10);
    std::cell::LazyCell::force(&*&raw const dealloc_count_11);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_12);
}
