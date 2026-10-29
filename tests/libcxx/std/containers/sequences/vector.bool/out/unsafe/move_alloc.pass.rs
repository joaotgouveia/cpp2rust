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
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn to_(&self) -> () {
        return !((self.ptr_).is_null());
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self
            as *mut min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__::operator_add_assign(
                &mut tmp, n,
            )
        });
        return tmp;
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
impl min_allocator_bool_ {
    pub unsafe fn new_1(mut _a0: min_allocator_unsigned_long_) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn operator_eq_4(mut _a0: min_allocator_bool_, mut _a1: min_allocator_bool_) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_bool_ {}
impl std::cmp::PartialEq for min_allocator_bool_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_4(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_bool_ {}
impl min_allocator_unsigned_long_ {
    pub unsafe fn new_1(mut _a0: min_allocator_bool_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_unsigned_long_ {}
impl min_allocator_unsigned_long_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        return min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
impl min_allocator_bool_ {}
impl min_allocator_unsigned_long_ {}
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
pub static mut destructed_value_5: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_6: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_7(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_8(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_7(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_5));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_5));
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
            operator_eq_7(
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
impl test_allocator_bool_ {
    pub unsafe fn new_1(a: *const test_allocator_unsigned_long_) -> Self {
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
pub unsafe fn operator_eq_9(x: *const test_allocator_bool_, y: *const test_allocator_bool_) -> () {
    return (((*x).data_) == ((*y).data_));
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
    pub unsafe fn new_2(mut data: ()) -> Self {
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_5));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_5));
    }
}
impl std::cmp::PartialEq for test_allocator_bool_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_9(
                self as *const test_allocator_bool_,
                other as *const test_allocator_bool_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_bool_ {}
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_5));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_5));
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
impl other_allocator_bool_ {
    pub unsafe fn new_1(a: *const other_allocator_unsigned_long_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
pub unsafe fn operator_eq_10(
    x: *const other_allocator_bool_,
    y: *const other_allocator_bool_,
) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct other_allocator_bool_ {
    data_: (),
}
impl other_allocator_bool_ {
    pub unsafe fn new_2(mut i: ()) -> Self {
        let mut this = Self { data_: i };
        this
    }
}
impl std::cmp::PartialEq for other_allocator_bool_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_10(
                self as *const other_allocator_bool_,
                other as *const other_allocator_bool_,
            )
        }
    }
}
impl std::cmp::Eq for other_allocator_bool_ {}
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
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        return unimplemented!();
    }
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
pub unsafe fn test_11(a: *const (), a0: *const ()) {
    let mut v: () = unimplemented!();
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while ((i) < (5_())) {
        unimplemented!();
        unimplemented!();
        i = (i).wrapping_add(2_());
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_12(a: *const (), a0: *const ()) {
    let mut v: () = unimplemented!();
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while ((i) < (17_())) {
        unimplemented!();
        unimplemented!();
        i = (i).wrapping_add(2_());
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_13(a: *const (), a0: *const ()) {
    let mut v: () = unimplemented!();
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while ((i) < (65_())) {
        unimplemented!();
        unimplemented!();
        i = (i).wrapping_add(2_());
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_14(a: *const (), a0: *const ()) {
    let mut v: () = unimplemented!();
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while ((i) < (299_())) {
        unimplemented!();
        unimplemented!();
        i = (i).wrapping_add(2_());
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_15(a: *const test_allocator_bool_, a0: *const test_allocator_bool_) {
    let mut v: () = unimplemented!();
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while ((i) < (5_())) {
        unimplemented!();
        unimplemented!();
        i = (i).wrapping_add(2_());
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    if (unsafe {
        let _x: *const test_allocator_bool_ = a;
        let _y: *const test_allocator_bool_ = a0;
        operator_eq_9(_x, _y)
    }) {
        unimplemented!();
    } else {
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn test_16(a: *const test_allocator_bool_, a0: *const test_allocator_bool_) {
    let mut v: () = unimplemented!();
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while ((i) < (17_())) {
        unimplemented!();
        unimplemented!();
        i = (i).wrapping_add(2_());
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    if (unsafe {
        let _x: *const test_allocator_bool_ = a;
        let _y: *const test_allocator_bool_ = a0;
        operator_eq_9(_x, _y)
    }) {
        unimplemented!();
    } else {
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn test_17(a: *const test_allocator_bool_, a0: *const test_allocator_bool_) {
    let mut v: () = unimplemented!();
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while ((i) < (65_())) {
        unimplemented!();
        unimplemented!();
        i = (i).wrapping_add(2_());
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    if (unsafe {
        let _x: *const test_allocator_bool_ = a;
        let _y: *const test_allocator_bool_ = a0;
        operator_eq_9(_x, _y)
    }) {
        unimplemented!();
    } else {
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn test_18(a: *const test_allocator_bool_, a0: *const test_allocator_bool_) {
    let mut v: () = unimplemented!();
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while ((i) < (299_())) {
        unimplemented!();
        unimplemented!();
        i = (i).wrapping_add(2_());
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    if (unsafe {
        let _x: *const test_allocator_bool_ = a;
        let _y: *const test_allocator_bool_ = a0;
        operator_eq_9(_x, _y)
    }) {
        unimplemented!();
    } else {
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn test_19(a: *const other_allocator_bool_, a0: *const other_allocator_bool_) {
    let mut v: () = unimplemented!();
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while ((i) < (5_())) {
        unimplemented!();
        unimplemented!();
        i = (i).wrapping_add(2_());
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    if (unsafe {
        let _x: *const other_allocator_bool_ = a;
        let _y: *const other_allocator_bool_ = a0;
        operator_eq_10(_x, _y)
    }) {
        unimplemented!();
    } else {
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn test_20(a: *const other_allocator_bool_, a0: *const other_allocator_bool_) {
    let mut v: () = unimplemented!();
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while ((i) < (17_())) {
        unimplemented!();
        unimplemented!();
        i = (i).wrapping_add(2_());
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    if (unsafe {
        let _x: *const other_allocator_bool_ = a;
        let _y: *const other_allocator_bool_ = a0;
        operator_eq_10(_x, _y)
    }) {
        unimplemented!();
    } else {
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn test_21(a: *const other_allocator_bool_, a0: *const other_allocator_bool_) {
    let mut v: () = unimplemented!();
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while ((i) < (65_())) {
        unimplemented!();
        unimplemented!();
        i = (i).wrapping_add(2_());
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    if (unsafe {
        let _x: *const other_allocator_bool_ = a;
        let _y: *const other_allocator_bool_ = a0;
        operator_eq_10(_x, _y)
    }) {
        unimplemented!();
    } else {
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn test_22(a: *const other_allocator_bool_, a0: *const other_allocator_bool_) {
    let mut v: () = unimplemented!();
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while ((i) < (299_())) {
        unimplemented!();
        unimplemented!();
        i = (i).wrapping_add(2_());
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    if (unsafe {
        let _x: *const other_allocator_bool_ = a;
        let _y: *const other_allocator_bool_ = a0;
        operator_eq_10(_x, _y)
    }) {
        unimplemented!();
    } else {
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn test_23(a: *const min_allocator_bool_, a0: *const min_allocator_bool_) {
    let mut v: () = unimplemented!();
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while ((i) < (5_())) {
        unimplemented!();
        unimplemented!();
        i = (i).wrapping_add(2_());
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_24(a: *const min_allocator_bool_, a0: *const min_allocator_bool_) {
    let mut v: () = unimplemented!();
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while ((i) < (17_())) {
        unimplemented!();
        unimplemented!();
        i = (i).wrapping_add(2_());
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_25(a: *const min_allocator_bool_, a0: *const min_allocator_bool_) {
    let mut v: () = unimplemented!();
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while ((i) < (65_())) {
        unimplemented!();
        unimplemented!();
        i = (i).wrapping_add(2_());
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_26(a: *const min_allocator_bool_, a0: *const min_allocator_bool_) {
    let mut v: () = unimplemented!();
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while ((i) < (299_())) {
        unimplemented!();
        unimplemented!();
        i = (i).wrapping_add(2_());
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn tests_27() -> () {
    {
        (unsafe {
            let mut _a: () = unimplemented!();
            let mut _a0: () = unimplemented!();
            test_11(&mut _a, &mut _a0)
        });
        (unsafe {
            let mut _a: () = unimplemented!();
            let mut _a0: () = unimplemented!();
            test_12(&mut _a, &mut _a0)
        });
        (unsafe {
            let mut _a: () = unimplemented!();
            let mut _a0: () = unimplemented!();
            test_13(&mut _a, &mut _a0)
        });
        (unsafe {
            let mut _a: () = unimplemented!();
            let mut _a0: () = unimplemented!();
            test_14(&mut _a, &mut _a0)
        });
    }
    {
        (unsafe {
            let mut _a: test_allocator_bool_ = test_allocator_bool_::new_2({ 5_() });
            let mut _a0: test_allocator_bool_ = test_allocator_bool_::new_2({ 5_() });
            test_15(&mut _a, &mut _a0)
        });
        (unsafe {
            let mut _a: test_allocator_bool_ = test_allocator_bool_::new_2({ 5_() });
            let mut _a0: test_allocator_bool_ = test_allocator_bool_::new_2({ 5_() });
            test_16(&mut _a, &mut _a0)
        });
        (unsafe {
            let mut _a: test_allocator_bool_ = test_allocator_bool_::new_2({ 5_() });
            let mut _a0: test_allocator_bool_ = test_allocator_bool_::new_2({ 5_() });
            test_17(&mut _a, &mut _a0)
        });
        (unsafe {
            let mut _a: test_allocator_bool_ = test_allocator_bool_::new_2({ 5_() });
            let mut _a0: test_allocator_bool_ = test_allocator_bool_::new_2({ 5_() });
            test_18(&mut _a, &mut _a0)
        });
        (unsafe {
            let mut _a: test_allocator_bool_ = test_allocator_bool_::new_2({ 5_() });
            let mut _a0: test_allocator_bool_ = test_allocator_bool_::new_2({ 6_() });
            test_15(&mut _a, &mut _a0)
        });
        (unsafe {
            let mut _a: test_allocator_bool_ = test_allocator_bool_::new_2({ 5_() });
            let mut _a0: test_allocator_bool_ = test_allocator_bool_::new_2({ 6_() });
            test_16(&mut _a, &mut _a0)
        });
        (unsafe {
            let mut _a: test_allocator_bool_ = test_allocator_bool_::new_2({ 5_() });
            let mut _a0: test_allocator_bool_ = test_allocator_bool_::new_2({ 6_() });
            test_17(&mut _a, &mut _a0)
        });
        (unsafe {
            let mut _a: test_allocator_bool_ = test_allocator_bool_::new_2({ 5_() });
            let mut _a0: test_allocator_bool_ = test_allocator_bool_::new_2({ 6_() });
            test_18(&mut _a, &mut _a0)
        });
    }
    {
        (unsafe {
            let mut _a: other_allocator_bool_ = other_allocator_bool_::new_2({ 5_() });
            let mut _a0: other_allocator_bool_ = other_allocator_bool_::new_2({ 5_() });
            test_19(&mut _a, &mut _a0)
        });
        (unsafe {
            let mut _a: other_allocator_bool_ = other_allocator_bool_::new_2({ 5_() });
            let mut _a0: other_allocator_bool_ = other_allocator_bool_::new_2({ 5_() });
            test_20(&mut _a, &mut _a0)
        });
        (unsafe {
            let mut _a: other_allocator_bool_ = other_allocator_bool_::new_2({ 5_() });
            let mut _a0: other_allocator_bool_ = other_allocator_bool_::new_2({ 5_() });
            test_21(&mut _a, &mut _a0)
        });
        (unsafe {
            let mut _a: other_allocator_bool_ = other_allocator_bool_::new_2({ 5_() });
            let mut _a0: other_allocator_bool_ = other_allocator_bool_::new_2({ 5_() });
            test_22(&mut _a, &mut _a0)
        });
        (unsafe {
            let mut _a: other_allocator_bool_ = other_allocator_bool_::new_2({ 5_() });
            let mut _a0: other_allocator_bool_ = other_allocator_bool_::new_2({ 3_() });
            test_19(&mut _a, &mut _a0)
        });
        (unsafe {
            let mut _a: other_allocator_bool_ = other_allocator_bool_::new_2({ 5_() });
            let mut _a0: other_allocator_bool_ = other_allocator_bool_::new_2({ 3_() });
            test_20(&mut _a, &mut _a0)
        });
        (unsafe {
            let mut _a: other_allocator_bool_ = other_allocator_bool_::new_2({ 5_() });
            let mut _a0: other_allocator_bool_ = other_allocator_bool_::new_2({ 3_() });
            test_21(&mut _a, &mut _a0)
        });
        (unsafe {
            let mut _a: other_allocator_bool_ = other_allocator_bool_::new_2({ 5_() });
            let mut _a0: other_allocator_bool_ = other_allocator_bool_::new_2({ 3_() });
            test_22(&mut _a, &mut _a0)
        });
    }
    {
        (unsafe {
            let mut _a: min_allocator_bool_ = <min_allocator_bool_>::default();
            let mut _a0: min_allocator_bool_ = <min_allocator_bool_>::default();
            test_23(&mut _a, &mut _a0)
        });
        (unsafe {
            let mut _a: min_allocator_bool_ = <min_allocator_bool_>::default();
            let mut _a0: min_allocator_bool_ = <min_allocator_bool_>::default();
            test_24(&mut _a, &mut _a0)
        });
        (unsafe {
            let mut _a: min_allocator_bool_ = <min_allocator_bool_>::default();
            let mut _a0: min_allocator_bool_ = <min_allocator_bool_>::default();
            test_25(&mut _a, &mut _a0)
        });
        (unsafe {
            let mut _a: min_allocator_bool_ = <min_allocator_bool_>::default();
            let mut _a0: min_allocator_bool_ = <min_allocator_bool_>::default();
            test_26(&mut _a, &mut _a0)
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
    (unsafe { tests_27() });
    const _: () = assert!(true, "tests()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
    std::cell::LazyCell::force(&*&raw const destructed_value_5);
    std::cell::LazyCell::force(&*&raw const moved_value_6);
}
