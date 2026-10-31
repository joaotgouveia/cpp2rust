extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
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
}
impl Clone for test_allocator_int_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_int_::copy_from(self as *const test_allocator_int_) }
    }
}
impl Default for test_allocator_int_ {
    fn default() -> Self {
        unsafe {
            test_allocator_int_ {
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_6))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_7)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_6))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_7))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_8)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_5)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_6)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_7)) = 0_usize;
    }
}
pub static mut outstanding_bytes_5: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_6: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_7: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_8: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut i32,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
pub unsafe fn test0_9(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_10(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_11(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_12(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_13(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_14(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_15(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_16(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_17(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_18(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_19(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_20(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_21(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_22(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_23(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_24(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_25(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_26(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_27(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_28(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_29(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_30(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_31(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_32(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_33(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_34(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_35(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_36(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_37(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test0_38(
    mut s: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!() as usize)));
    assert!(unimplemented!());
}
pub unsafe fn test_39() {
    let mut is1: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (1));
    });
    let mut is2: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (2));
    });
    let mut is3: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (3));
    });
    let mut is4: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (4));
    });
    let mut True: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i32| -> bool {
        return true;
    });
    let mut False: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i32| -> bool {
        return false;
    });
    (unsafe { test0_9(unimplemented!(), is1.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_9(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_10(unimplemented!(), is2.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_9(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_10(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_11(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_9(unimplemented!(), is1.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_11(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_9(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_10(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_11(unimplemented!(), is3.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_12(unimplemented!(), is4.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_9(unimplemented!(), is1.copy_from(), unimplemented!(), 3_usize) });
    (unsafe { test0_10(unimplemented!(), is2.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_9(unimplemented!(), is1.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_10(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_11(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_9(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_10(unimplemented!(), is2.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_11(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe {
        test0_13(
            unimplemented!(),
            True.copy_from(),
            unimplemented!(),
            3_usize,
        )
    });
    (unsafe {
        test0_14(
            unimplemented!(),
            False.copy_from(),
            unimplemented!(),
            0_usize,
        )
    });
}
pub unsafe fn test_40() {
    let mut is1: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (1));
    });
    let mut is2: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (2));
    });
    let mut is3: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (3));
    });
    let mut is4: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (4));
    });
    let mut True: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i32| -> bool {
        return true;
    });
    let mut False: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i32| -> bool {
        return false;
    });
    (unsafe { test0_15(unimplemented!(), is1.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_15(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_16(unimplemented!(), is2.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_15(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_16(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_17(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_15(unimplemented!(), is1.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_17(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_15(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_16(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_17(unimplemented!(), is3.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_18(unimplemented!(), is4.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_15(unimplemented!(), is1.copy_from(), unimplemented!(), 3_usize) });
    (unsafe { test0_16(unimplemented!(), is2.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_15(unimplemented!(), is1.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_16(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_17(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_15(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_16(unimplemented!(), is2.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_17(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe {
        test0_19(
            unimplemented!(),
            True.copy_from(),
            unimplemented!(),
            3_usize,
        )
    });
    (unsafe {
        test0_20(
            unimplemented!(),
            False.copy_from(),
            unimplemented!(),
            0_usize,
        )
    });
}
pub unsafe fn test_41() {
    let mut is1: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (1));
    });
    let mut is2: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (2));
    });
    let mut is3: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (3));
    });
    let mut is4: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (4));
    });
    let mut True: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i32| -> bool {
        return true;
    });
    let mut False: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i32| -> bool {
        return false;
    });
    (unsafe { test0_21(unimplemented!(), is1.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_21(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_22(unimplemented!(), is2.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_21(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_22(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_23(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_21(unimplemented!(), is1.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_23(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_21(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_22(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_23(unimplemented!(), is3.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_24(unimplemented!(), is4.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_21(unimplemented!(), is1.copy_from(), unimplemented!(), 3_usize) });
    (unsafe { test0_22(unimplemented!(), is2.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_21(unimplemented!(), is1.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_22(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_23(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_21(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_22(unimplemented!(), is2.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_23(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe {
        test0_25(
            unimplemented!(),
            True.copy_from(),
            unimplemented!(),
            3_usize,
        )
    });
    (unsafe {
        test0_26(
            unimplemented!(),
            False.copy_from(),
            unimplemented!(),
            0_usize,
        )
    });
}
pub unsafe fn test_42() {
    let mut is1: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i64| -> bool {
        return ((v) == (1_i64));
    });
    let mut is2: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i64| -> bool {
        return ((v) == (2_i64));
    });
    let mut is3: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i64| -> bool {
        return ((v) == (3_i64));
    });
    let mut is4: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i64| -> bool {
        return ((v) == (4_i64));
    });
    let mut True: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i64| -> bool {
        return true;
    });
    let mut False: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i64| -> bool {
        return false;
    });
    (unsafe { test0_27(unimplemented!(), is1.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_27(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_28(unimplemented!(), is2.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_27(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_28(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_29(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_27(unimplemented!(), is1.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_29(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_27(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_28(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_29(unimplemented!(), is3.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_30(unimplemented!(), is4.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_27(unimplemented!(), is1.copy_from(), unimplemented!(), 3_usize) });
    (unsafe { test0_28(unimplemented!(), is2.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_27(unimplemented!(), is1.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_28(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_29(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_27(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_28(unimplemented!(), is2.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_29(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe {
        test0_31(
            unimplemented!(),
            True.copy_from(),
            unimplemented!(),
            3_usize,
        )
    });
    (unsafe {
        test0_32(
            unimplemented!(),
            False.copy_from(),
            unimplemented!(),
            0_usize,
        )
    });
}
pub unsafe fn test_43() {
    let mut is1: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: f64| -> bool {
        return ((v) == (1_f64));
    });
    let mut is2: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: f64| -> bool {
        return ((v) == (2_f64));
    });
    let mut is3: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: f64| -> bool {
        return ((v) == (3_f64));
    });
    let mut is4: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: f64| -> bool {
        return ((v) == (4_f64));
    });
    let mut True: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: f64| -> bool {
        return true;
    });
    let mut False: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: f64| -> bool {
        return false;
    });
    (unsafe { test0_33(unimplemented!(), is1.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_33(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_34(unimplemented!(), is2.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_33(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_34(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_35(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_33(unimplemented!(), is1.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_35(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_33(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_34(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_35(unimplemented!(), is3.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_36(unimplemented!(), is4.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_33(unimplemented!(), is1.copy_from(), unimplemented!(), 3_usize) });
    (unsafe { test0_34(unimplemented!(), is2.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_33(unimplemented!(), is1.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_34(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_35(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_33(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_34(unimplemented!(), is2.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_35(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe {
        test0_37(
            unimplemented!(),
            True.copy_from(),
            unimplemented!(),
            3_usize,
        )
    });
    (unsafe {
        test0_38(
            unimplemented!(),
            False.copy_from(),
            unimplemented!(),
            0_usize,
        )
    });
}
pub unsafe fn test_44() -> bool {
    (unsafe { test_39() });
    (unsafe { test_40() });
    (unsafe { test_41() });
    (unsafe { test_42() });
    (unsafe { test_43() });
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
    (unsafe { test_44() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_1);
    std::cell::LazyCell::force(&*&raw const moved_value_2);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_5);
    std::cell::LazyCell::force(&*&raw const alloc_count_6);
    std::cell::LazyCell::force(&*&raw const dealloc_count_7);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_8);
}
