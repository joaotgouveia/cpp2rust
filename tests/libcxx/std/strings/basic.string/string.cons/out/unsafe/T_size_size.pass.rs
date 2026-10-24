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
pub unsafe fn operator_eq_5(x: *const test_allocator_char_, y: *const test_allocator_char_) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_char__rebind_char_ {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_char_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_char_ {
    pub unsafe fn new_1(mut data: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
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
}
impl std::cmp::PartialEq for test_allocator_char_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_5(
                self as *const test_allocator_char_,
                other as *const test_allocator_char_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_char_ {}
impl Clone for test_allocator_char_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_char_::copy_from(self as *const test_allocator_char_) }
    }
}
impl Default for test_allocator_char_ {
    fn default() -> Self {
        unsafe {
            test_allocator_char_ {
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_7))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_8))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_9)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_6)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_7)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_8)) = 0_();
    }
}
pub static mut outstanding_bytes_6: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_7: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_8: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_9: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_char__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_char__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn pointer_to(
        t: *mut (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *const ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn new_2(mut p: min_pointer_char__std_integral_constant_unsigned_long__0__) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
    }
    pub unsafe fn pointer_to(
        t: *const (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_10(mut _a0: min_allocator_char_, mut _a1: min_allocator_char_) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_char_ {}
impl min_allocator_char_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_char__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
impl std::cmp::PartialEq for min_allocator_char_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_10(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_char_ {}
pub unsafe fn operator_eq_11(mut _a0: safe_allocator_char_, mut _a1: safe_allocator_char_) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct safe_allocator_char_ {}
impl safe_allocator_char_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        let mut memory: *mut () = unimplemented!();
        {
            (unsafe {
                memset_12(
                    (memory as ()),
                    0_(),
                    (::std::mem::size_of::<()>() as ()).wrapping_mul(n),
                )
            });
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        {
            (unsafe {
                let mut _value: () = (unsafe {
                    memset_12(
                        (p as ()),
                        0_(),
                        (::std::mem::size_of::<()>() as ()).wrapping_mul(n),
                    )
                });
                DoNotOptimize_0(&mut _value)
            });
        }
        unimplemented!();
    }
}
impl std::cmp::PartialEq for safe_allocator_char_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_11(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for safe_allocator_char_ {}
impl Default for safe_allocator_char_ {
    fn default() -> Self {
        unsafe { safe_allocator_char_::new() }
    }
}
pub unsafe fn test_13(mut sv: (), mut pos: (), mut n: ()) {
    if ((pos) <= (unimplemented!())) {
        let mut s2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut rlen: () = (*unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_14(mut sv: (), mut pos: (), mut n: ()) {
    if ((pos) <= (unimplemented!())) {
        let mut s2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut rlen: () = (*unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_15(mut sv: (), mut pos: (), mut n: ()) {
    if ((pos) <= (unimplemented!())) {
        let mut s2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut rlen: () = (*unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_16(mut sv: (), mut pos: (), mut n: ()) {
    if ((pos) <= (unimplemented!())) {
        let mut s2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut rlen: () = (*unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_17(mut sv: (), mut pos: (), mut n: (), a: *const ()) {
    if ((pos) <= (unimplemented!())) {
        let mut s2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut rlen: () = (*unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_18(mut sv: (), mut pos: (), mut n: (), a: *const test_allocator_char_) {
    if ((pos) <= (unimplemented!())) {
        let mut s2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut rlen: () = (*unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_19(mut sv: (), mut pos: (), mut n: (), a: *const min_allocator_char_) {
    if ((pos) <= (unimplemented!())) {
        let mut s2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut rlen: () = (*unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_20(mut sv: (), mut pos: (), mut n: (), a: *const safe_allocator_char_) {
    if ((pos) <= (unimplemented!())) {
        let mut s2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut rlen: () = (*unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_string_21(a: *const ()) {
    (unsafe { test_13(unimplemented!(), 0_(), 0_()) });
    (unsafe { test_13(unimplemented!(), 0_(), 1_()) });
    (unsafe { test_13(unimplemented!(), 1_(), 0_()) });
    (unsafe { test_13(unimplemented!(), 1_(), 1_()) });
    (unsafe { test_13(unimplemented!(), 1_(), 2_()) });
    (unsafe { test_13(unimplemented!(), 0_(), 0_()) });
    (unsafe { test_13(unimplemented!(), 0_(), 1_()) });
    (unsafe { test_13(unimplemented!(), 50_(), 0_()) });
    (unsafe { test_13(unimplemented!(), 50_(), 1_()) });
    (unsafe { test_13(unimplemented!(), 50_(), 10_()) });
    (unsafe { test_13(unimplemented!(), 50_(), 100_()) });
    (unsafe {
        let mut _a: () = unimplemented!();
        test_17(unimplemented!(), 0_(), 0_(), &mut _a)
    });
    (unsafe {
        let mut _a: () = unimplemented!();
        test_17(unimplemented!(), 0_(), 1_(), &mut _a)
    });
    (unsafe {
        let mut _a: () = unimplemented!();
        test_17(unimplemented!(), 1_(), 0_(), &mut _a)
    });
    (unsafe {
        let mut _a: () = unimplemented!();
        test_17(unimplemented!(), 1_(), 1_(), &mut _a)
    });
    (unsafe {
        let mut _a: () = unimplemented!();
        test_17(unimplemented!(), 1_(), 2_(), &mut _a)
    });
    (unsafe {
        let mut _a: () = unimplemented!();
        test_17(unimplemented!(), 0_(), 0_(), &mut _a)
    });
    (unsafe {
        let mut _a: () = unimplemented!();
        test_17(unimplemented!(), 0_(), 1_(), &mut _a)
    });
    (unsafe {
        let mut _a: () = unimplemented!();
        test_17(unimplemented!(), 50_(), 0_(), &mut _a)
    });
    (unsafe {
        let mut _a: () = unimplemented!();
        test_17(unimplemented!(), 50_(), 1_(), &mut _a)
    });
    (unsafe {
        let mut _a: () = unimplemented!();
        test_17(unimplemented!(), 50_(), 10_(), &mut _a)
    });
    (unsafe {
        let mut _a: () = unimplemented!();
        test_17(unimplemented!(), 50_(), 100_(), &mut _a)
    });
}
pub unsafe fn test_string_22(a: *const test_allocator_char_) {
    (unsafe { test_14(unimplemented!(), 0_(), 0_()) });
    (unsafe { test_14(unimplemented!(), 0_(), 1_()) });
    (unsafe { test_14(unimplemented!(), 1_(), 0_()) });
    (unsafe { test_14(unimplemented!(), 1_(), 1_()) });
    (unsafe { test_14(unimplemented!(), 1_(), 2_()) });
    (unsafe { test_14(unimplemented!(), 0_(), 0_()) });
    (unsafe { test_14(unimplemented!(), 0_(), 1_()) });
    (unsafe { test_14(unimplemented!(), 50_(), 0_()) });
    (unsafe { test_14(unimplemented!(), 50_(), 1_()) });
    (unsafe { test_14(unimplemented!(), 50_(), 10_()) });
    (unsafe { test_14(unimplemented!(), 50_(), 100_()) });
    (unsafe {
        let mut _a: test_allocator_char_ = test_allocator_char_::copy_from({ a });
        test_18(unimplemented!(), 0_(), 0_(), &mut _a)
    });
    (unsafe {
        let mut _a: test_allocator_char_ = test_allocator_char_::copy_from({ a });
        test_18(unimplemented!(), 0_(), 1_(), &mut _a)
    });
    (unsafe {
        let mut _a: test_allocator_char_ = test_allocator_char_::copy_from({ a });
        test_18(unimplemented!(), 1_(), 0_(), &mut _a)
    });
    (unsafe {
        let mut _a: test_allocator_char_ = test_allocator_char_::copy_from({ a });
        test_18(unimplemented!(), 1_(), 1_(), &mut _a)
    });
    (unsafe {
        let mut _a: test_allocator_char_ = test_allocator_char_::copy_from({ a });
        test_18(unimplemented!(), 1_(), 2_(), &mut _a)
    });
    (unsafe {
        let mut _a: test_allocator_char_ = test_allocator_char_::copy_from({ a });
        test_18(unimplemented!(), 0_(), 0_(), &mut _a)
    });
    (unsafe {
        let mut _a: test_allocator_char_ = test_allocator_char_::copy_from({ a });
        test_18(unimplemented!(), 0_(), 1_(), &mut _a)
    });
    (unsafe {
        let mut _a: test_allocator_char_ = test_allocator_char_::copy_from({ a });
        test_18(unimplemented!(), 50_(), 0_(), &mut _a)
    });
    (unsafe {
        let mut _a: test_allocator_char_ = test_allocator_char_::copy_from({ a });
        test_18(unimplemented!(), 50_(), 1_(), &mut _a)
    });
    (unsafe {
        let mut _a: test_allocator_char_ = test_allocator_char_::copy_from({ a });
        test_18(unimplemented!(), 50_(), 10_(), &mut _a)
    });
    (unsafe {
        let mut _a: test_allocator_char_ = test_allocator_char_::copy_from({ a });
        test_18(unimplemented!(), 50_(), 100_(), &mut _a)
    });
}
pub unsafe fn test_string_23(a: *const min_allocator_char_) {
    (unsafe { test_15(unimplemented!(), 0_(), 0_()) });
    (unsafe { test_15(unimplemented!(), 0_(), 1_()) });
    (unsafe { test_15(unimplemented!(), 1_(), 0_()) });
    (unsafe { test_15(unimplemented!(), 1_(), 1_()) });
    (unsafe { test_15(unimplemented!(), 1_(), 2_()) });
    (unsafe { test_15(unimplemented!(), 0_(), 0_()) });
    (unsafe { test_15(unimplemented!(), 0_(), 1_()) });
    (unsafe { test_15(unimplemented!(), 50_(), 0_()) });
    (unsafe { test_15(unimplemented!(), 50_(), 1_()) });
    (unsafe { test_15(unimplemented!(), 50_(), 10_()) });
    (unsafe { test_15(unimplemented!(), 50_(), 100_()) });
    (unsafe {
        let mut _a: min_allocator_char_ = (*a);
        test_19(unimplemented!(), 0_(), 0_(), &mut _a)
    });
    (unsafe {
        let mut _a: min_allocator_char_ = (*a);
        test_19(unimplemented!(), 0_(), 1_(), &mut _a)
    });
    (unsafe {
        let mut _a: min_allocator_char_ = (*a);
        test_19(unimplemented!(), 1_(), 0_(), &mut _a)
    });
    (unsafe {
        let mut _a: min_allocator_char_ = (*a);
        test_19(unimplemented!(), 1_(), 1_(), &mut _a)
    });
    (unsafe {
        let mut _a: min_allocator_char_ = (*a);
        test_19(unimplemented!(), 1_(), 2_(), &mut _a)
    });
    (unsafe {
        let mut _a: min_allocator_char_ = (*a);
        test_19(unimplemented!(), 0_(), 0_(), &mut _a)
    });
    (unsafe {
        let mut _a: min_allocator_char_ = (*a);
        test_19(unimplemented!(), 0_(), 1_(), &mut _a)
    });
    (unsafe {
        let mut _a: min_allocator_char_ = (*a);
        test_19(unimplemented!(), 50_(), 0_(), &mut _a)
    });
    (unsafe {
        let mut _a: min_allocator_char_ = (*a);
        test_19(unimplemented!(), 50_(), 1_(), &mut _a)
    });
    (unsafe {
        let mut _a: min_allocator_char_ = (*a);
        test_19(unimplemented!(), 50_(), 10_(), &mut _a)
    });
    (unsafe {
        let mut _a: min_allocator_char_ = (*a);
        test_19(unimplemented!(), 50_(), 100_(), &mut _a)
    });
}
pub unsafe fn test_string_24(a: *const safe_allocator_char_) {
    (unsafe { test_16(unimplemented!(), 0_(), 0_()) });
    (unsafe { test_16(unimplemented!(), 0_(), 1_()) });
    (unsafe { test_16(unimplemented!(), 1_(), 0_()) });
    (unsafe { test_16(unimplemented!(), 1_(), 1_()) });
    (unsafe { test_16(unimplemented!(), 1_(), 2_()) });
    (unsafe { test_16(unimplemented!(), 0_(), 0_()) });
    (unsafe { test_16(unimplemented!(), 0_(), 1_()) });
    (unsafe { test_16(unimplemented!(), 50_(), 0_()) });
    (unsafe { test_16(unimplemented!(), 50_(), 1_()) });
    (unsafe { test_16(unimplemented!(), 50_(), 10_()) });
    (unsafe { test_16(unimplemented!(), 50_(), 100_()) });
    (unsafe {
        let mut _a: safe_allocator_char_ = (*a);
        test_20(unimplemented!(), 0_(), 0_(), &mut _a)
    });
    (unsafe {
        let mut _a: safe_allocator_char_ = (*a);
        test_20(unimplemented!(), 0_(), 1_(), &mut _a)
    });
    (unsafe {
        let mut _a: safe_allocator_char_ = (*a);
        test_20(unimplemented!(), 1_(), 0_(), &mut _a)
    });
    (unsafe {
        let mut _a: safe_allocator_char_ = (*a);
        test_20(unimplemented!(), 1_(), 1_(), &mut _a)
    });
    (unsafe {
        let mut _a: safe_allocator_char_ = (*a);
        test_20(unimplemented!(), 1_(), 2_(), &mut _a)
    });
    (unsafe {
        let mut _a: safe_allocator_char_ = (*a);
        test_20(unimplemented!(), 0_(), 0_(), &mut _a)
    });
    (unsafe {
        let mut _a: safe_allocator_char_ = (*a);
        test_20(unimplemented!(), 0_(), 1_(), &mut _a)
    });
    (unsafe {
        let mut _a: safe_allocator_char_ = (*a);
        test_20(unimplemented!(), 50_(), 0_(), &mut _a)
    });
    (unsafe {
        let mut _a: safe_allocator_char_ = (*a);
        test_20(unimplemented!(), 50_(), 1_(), &mut _a)
    });
    (unsafe {
        let mut _a: safe_allocator_char_ = (*a);
        test_20(unimplemented!(), 50_(), 10_(), &mut _a)
    });
    (unsafe {
        let mut _a: safe_allocator_char_ = (*a);
        test_20(unimplemented!(), 50_(), 100_(), &mut _a)
    });
}
pub unsafe fn test_25() -> () {
    (unsafe {
        let mut _a: () = unimplemented!();
        test_string_21(&mut _a)
    });
    (unsafe {
        let mut _a: test_allocator_char_ = <test_allocator_char_>::default();
        test_string_22(&mut _a)
    });
    (unsafe {
        let mut _a: test_allocator_char_ = test_allocator_char_::new_1({ 8_() });
        test_string_22(&mut _a)
    });
    (unsafe {
        let mut _a: min_allocator_char_ = <min_allocator_char_>::default();
        test_string_23(&mut _a)
    });
    (unsafe {
        let mut _a: safe_allocator_char_ = safe_allocator_char_::new();
        test_string_24(&mut _a)
    });
    {
        let mut s: () = unimplemented!();
        let mut sv: () = unimplemented!();
        let mut arr: [(); 5] = std::mem::transmute(*b"IJKL\0");
        let mut s1: () = unimplemented!();
        unimplemented!();
        let mut s2: () = unimplemented!();
        unimplemented!();
        let mut s3: () = unimplemented!();
        unimplemented!();
        let mut s4: () = unimplemented!();
        unimplemented!();
        let mut s5: () = unimplemented!();
        unimplemented!();
        let mut s6: () = unimplemented!();
        unimplemented!();
        let mut s7: () = unimplemented!();
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
    (unsafe { test_25() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_1);
    std::cell::LazyCell::force(&*&raw const moved_value_2);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_6);
    std::cell::LazyCell::force(&*&raw const alloc_count_7);
    std::cell::LazyCell::force(&*&raw const dealloc_count_8);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_9);
}
