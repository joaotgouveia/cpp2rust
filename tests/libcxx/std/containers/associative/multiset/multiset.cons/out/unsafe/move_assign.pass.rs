extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_1(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_2(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_3(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_4(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_5(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_6(_a0: *const MoveOnly, _a1: *const MoveOnly) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe)]
#[cpp_from(i32, new, some_value)]
pub struct MoveOnly {
    data_: i32,
}
impl MoveOnly {
    pub unsafe fn new(mut data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| unsafe { 1 });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(x: *mut MoveOnly) -> Self {
        let mut this = Self { data_: (*x).data_ };
        (*x).data_ = 0;
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut MoveOnly) -> *mut MoveOnly {
        self.data_ = (*x).data_;
        (*x).data_ = 0;
        return &mut (*(self as *mut MoveOnly));
    }
    pub unsafe fn get(&self) -> i32 {
        return self.data_;
    }
    pub unsafe fn operator_add(&self, x: *const MoveOnly) -> MoveOnly {
        return MoveOnly::new({ Some(((self.data_) + ((*x).data_))) });
    }
    pub unsafe fn operator_mul(&self, x: *const MoveOnly) -> MoveOnly {
        return MoveOnly::new({ Some(((self.data_) * ((*x).data_))) });
    }
}
impl std::cmp::Ord for MoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_6(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        unsafe { MoveOnly::new(None) }
    }
}
pub unsafe fn operator_eq_7(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_8(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_9(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_10(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_11(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_12(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_13(
    _a0: *const TrivialMoveOnly,
    _a1: *const TrivialMoveOnly,
) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe)]
#[cpp_from(i32, new, some_value)]
pub struct TrivialMoveOnly {
    data_: i32,
}
impl TrivialMoveOnly {
    pub unsafe fn new(mut data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| unsafe { 1 });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(_a0: *mut TrivialMoveOnly) -> Self {
        let mut this = Self {
            data_: (*_a0).data_,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut TrivialMoveOnly) -> *mut TrivialMoveOnly {
        self.data_ = (*_a0).data_;
        return &mut (*(self as *mut TrivialMoveOnly));
    }
    pub unsafe fn get(&self) -> i32 {
        return self.data_;
    }
    pub unsafe fn operator_add(&self, x: *const TrivialMoveOnly) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({ Some(((self.data_) + ((*x).data_))) });
    }
    pub unsafe fn operator_mul(&self, x: *const TrivialMoveOnly) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({ Some(((self.data_) * ((*x).data_))) });
    }
}
impl std::cmp::Ord for TrivialMoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_13(
                self as *const TrivialMoveOnly,
                other as *const TrivialMoveOnly,
            )
        }
    }
}
impl std::cmp::PartialOrd for TrivialMoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for TrivialMoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_7(
                self as *const TrivialMoveOnly,
                other as *const TrivialMoveOnly,
            )
        }
    }
}
impl std::cmp::Eq for TrivialMoveOnly {}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        unsafe { TrivialMoveOnly::new(None) }
    }
}
pub unsafe fn operator_eq_14(a: *const test_less_MoveOnly_, b: *const test_less_MoveOnly_) -> bool {
    return (((*a).data_) == ((*b).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_less_MoveOnly_ {
    pub data_: i32,
}
impl test_less_MoveOnly_ {
    pub unsafe fn new_1(mut data: i32) -> Self {
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn operator_call(&self, a: *const MoveOnly, b: *const MoveOnly) -> bool {
        return (unsafe {
            let _x: *const MoveOnly = a;
            let _y: *const MoveOnly = b;
            operator_lt_2(_x, _y)
        });
    }
}
impl std::cmp::PartialEq for test_less_MoveOnly_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_14(
                self as *const test_less_MoveOnly_,
                other as *const test_less_MoveOnly_,
            )
        }
    }
}
impl std::cmp::Eq for test_less_MoveOnly_ {}
impl Callable2<*const MoveOnly, *const MoveOnly, bool> for test_less_MoveOnly_ {
    fn call(&self, a1: *const MoveOnly, a2: *const MoveOnly) -> bool {
        unsafe { test_less_MoveOnly_::operator_call(self, a1, a2) }
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
pub static mut destructed_value_15: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { -1_i32 });
pub static mut moved_value_16: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 2147483647 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_17(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_18(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_17(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15));
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
            operator_eq_17(
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
pub unsafe fn operator_eq_19(
    x: *const test_allocator_MoveOnly_,
    y: *const test_allocator_MoveOnly_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_MoveOnly__rebind_MoveOnly_ {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_MoveOnly_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_MoveOnly_ {
    pub unsafe fn new_1(mut data: i32) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_MoveOnly_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        assert!(
            ((((*a).data_)
                != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15)))
                && (((*a).id_)
                    != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15))))
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
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15)))
        );
        assert!(
            ((self.id_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15));
    }
}
impl std::cmp::PartialEq for test_allocator_MoveOnly_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_19(
                self as *const test_allocator_MoveOnly_,
                other as *const test_allocator_MoveOnly_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_MoveOnly_ {}
impl Clone for test_allocator_MoveOnly_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_MoveOnly_::copy_from(self as *const test_allocator_MoveOnly_) }
    }
}
impl Default for test_allocator_MoveOnly_ {
    fn default() -> Self {
        unsafe {
            test_allocator_MoveOnly_ {
                data_: 0_i32,
                id_: 0,
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_20(
    x: *const other_allocator_MoveOnly_,
    y: *const other_allocator_MoveOnly_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct other_allocator_MoveOnly_ {
    data_: i32,
}
impl other_allocator_MoveOnly_ {
    pub unsafe fn new_1(mut i: i32) -> Self {
        let mut this = Self { data_: i };
        this
    }
}
impl std::cmp::PartialEq for other_allocator_MoveOnly_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_20(
                self as *const other_allocator_MoveOnly_,
                other as *const other_allocator_MoveOnly_,
            )
        }
    }
}
impl std::cmp::Eq for other_allocator_MoveOnly_ {}
impl Default for other_allocator_MoveOnly_ {
    fn default() -> Self {
        unsafe { other_allocator_MoveOnly_ { data_: 0_i32 } }
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
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_22))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_23)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_22))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_23))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_24)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_21)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_22)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_23)) = 0_usize;
    }
}
pub static mut outstanding_bytes_21: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_22: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_23: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_24: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut MoveOnly,
}
impl min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_25(
    mut _a0: min_allocator_MoveOnly_,
    mut _a1: min_allocator_MoveOnly_,
) -> bool {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_MoveOnly_ {}
impl std::cmp::PartialEq for min_allocator_MoveOnly_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_25(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_MoveOnly_ {}
pub unsafe fn test_26() -> bool {
    {
        let mut a1: [MoveOnly; 9] = [
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(3) }),
        ];
        let mut m1: () = unimplemented!();
        let mut a2: [MoveOnly; 9] = [
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(3) }),
        ];
        let mut m2: () = unimplemented!();
        let mut m3: () = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let mut _x: test_allocator_MoveOnly_ = unimplemented!();
                let mut _y: test_allocator_MoveOnly_ = test_allocator_MoveOnly_::new_1({ 7 });
                operator_eq_19(&mut _x, &mut _y)
            })
        );
        assert!(
            (unsafe {
                let mut _a: test_less_MoveOnly_ = unimplemented!();
                let mut _b: test_less_MoveOnly_ = test_less_MoveOnly_::new_1({ 5 });
                operator_eq_14(&mut _a, &mut _b)
            })
        );
        assert!(unimplemented!());
    }
    {
        let mut a1: [MoveOnly; 9] = [
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(3) }),
        ];
        let mut m1: () = unimplemented!();
        let mut a2: [MoveOnly; 9] = [
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(3) }),
        ];
        let mut m2: () = unimplemented!();
        let mut m3: () = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let mut _x: test_allocator_MoveOnly_ = unimplemented!();
                let mut _y: test_allocator_MoveOnly_ = test_allocator_MoveOnly_::new_1({ 5 });
                operator_eq_19(&mut _x, &mut _y)
            })
        );
        assert!(
            (unsafe {
                let mut _a: test_less_MoveOnly_ = unimplemented!();
                let mut _b: test_less_MoveOnly_ = test_less_MoveOnly_::new_1({ 5 });
                operator_eq_14(&mut _a, &mut _b)
            })
        );
        const _: () = assert!(true, "");;
    }
    {
        let mut a1: [MoveOnly; 9] = [
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(3) }),
        ];
        let mut m1: () = unimplemented!();
        let mut a2: [MoveOnly; 9] = [
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(3) }),
        ];
        let mut m2: () = unimplemented!();
        let mut m3: () = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let mut _x: other_allocator_MoveOnly_ = unimplemented!();
                let mut _y: other_allocator_MoveOnly_ = other_allocator_MoveOnly_::new_1({ 7 });
                operator_eq_20(&mut _x, &mut _y)
            })
        );
        assert!(
            (unsafe {
                let mut _a: test_less_MoveOnly_ = unimplemented!();
                let mut _b: test_less_MoveOnly_ = test_less_MoveOnly_::new_1({ 5 });
                operator_eq_14(&mut _a, &mut _b)
            })
        );
        assert!(unimplemented!());
    }
    {
        let mut a1: [MoveOnly; 9] = [
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(3) }),
        ];
        let mut m1: () = unimplemented!();
        let mut a2: [MoveOnly; 9] = [
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(3) }),
        ];
        let mut m2: () = unimplemented!();
        let mut m3: () = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let _arg0: min_allocator_MoveOnly_ = unimplemented!();
                operator_eq_25(_arg0, <min_allocator_MoveOnly_>::default())
            })
        );
        assert!(
            (unsafe {
                let mut _a: test_less_MoveOnly_ = unimplemented!();
                let mut _b: test_less_MoveOnly_ = test_less_MoveOnly_::new_1({ 5 });
                operator_eq_14(&mut _a, &mut _b)
            })
        );
        assert!(unimplemented!());
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
    (unsafe { test_26() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_15);
    std::cell::LazyCell::force(&*&raw const moved_value_16);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_21);
    std::cell::LazyCell::force(&*&raw const alloc_count_22);
    std::cell::LazyCell::force(&*&raw const dealloc_count_23);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_24);
}
