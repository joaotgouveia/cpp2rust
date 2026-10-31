extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(a: *const test_less_int_, b: *const test_less_int_) -> bool {
    return (((*a).data_) == ((*b).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_less_int_ {
    pub data_: i32,
}
impl test_less_int_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { data_: 0 };
        this
    }
    pub unsafe fn new_2(mut data: i32) -> Self {
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn operator_call(&self, a: *const i32, b: *const i32) -> bool {
        return ((*a) < (*b));
    }
}
impl std::cmp::PartialEq for test_less_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_0(
                self as *const test_less_int_,
                other as *const test_less_int_,
            )
        }
    }
}
impl std::cmp::Eq for test_less_int_ {}
impl Callable2<*const i32, *const i32, bool> for test_less_int_ {
    fn call(&self, a1: *const i32, a2: *const i32) -> bool {
        unsafe { test_less_int_::operator_call(self, a1, a2) }
    }
}
impl Default for test_less_int_ {
    fn default() -> Self {
        unsafe { test_less_int_::new_1() }
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
pub struct min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (i32, i32),
}
impl min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__int__ {}
pub unsafe fn operator_lt_9(left: *const NonTrivial, right: *const NonTrivial) -> bool {
    return (((*left).i) < ((*right).i));
}
pub unsafe fn operator_eq_10(left: *const NonTrivial, right: *const NonTrivial) -> bool {
    return (((*left).i) == ((*right).i));
}
#[repr(C)]
#[derive(
    VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, Default,
)]
#[cpp_from(i32, new, value)]
pub struct NonTrivial {
    pub i: i32,
}
impl NonTrivial {
    pub unsafe fn new(mut arg_i: i32) -> Self {
        let mut this = Self { i: arg_i };
        this
    }
    pub unsafe fn copy_from(other: *const NonTrivial) -> Self {
        let mut this = Self { i: (*other).i };
        this
    }
    pub unsafe fn move_from(other: *mut NonTrivial) -> Self {
        let mut this = Self { i: (*other).i };
        (*other).i = 0;
        this
    }
    pub unsafe fn copy_assign(&mut self, other: *const NonTrivial) -> *mut NonTrivial {
        if (((self as *mut NonTrivial).cast_const()) != (other)) {
            self.i = (*other).i;
        }
        return &mut (*(self as *mut NonTrivial));
    }
    pub unsafe fn move_assign(&mut self, other: *mut NonTrivial) -> *mut NonTrivial {
        self.i = (*other).i;
        (*other).i = 0;
        return &mut (*(self as *mut NonTrivial));
    }
}
impl std::cmp::Ord for NonTrivial {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_9(self as *const NonTrivial, other as *const NonTrivial) {
                std::cmp::Ordering::Less
            } else if operator_lt_9(other as *const NonTrivial, self as *const NonTrivial) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for NonTrivial {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for NonTrivial {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_10(self as *const NonTrivial, other as *const NonTrivial) }
    }
}
impl std::cmp::Eq for NonTrivial {}
impl Clone for NonTrivial {
    fn clone(&self) -> Self {
        unsafe { NonTrivial::copy_from(self as *const NonTrivial) }
    }
}
const _: () = assert!(
    true,
    "!std::is_trivially_copy_assignable<NonTrivial>::value"
);
const _: () = assert!(
    true,
    "!std::is_trivially_default_constructible<NonTrivial>::value"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(Vec<*mut ::libc::c_void > , new, mut_ptr)]
pub struct tracking_allocator_std_pair_const_int__int__ {
    allocs_: *mut Vec<*mut ::libc::c_void>,
}
impl tracking_allocator_std_pair_const_int__int__ {
    pub unsafe fn new(allocs: *mut Vec<*mut ::libc::c_void>) -> Self {
        let mut this = Self { allocs_: (allocs) };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoOp {}
impl NoOp {
    pub unsafe fn operator_call(&mut self) {}
}
impl Callable0<()> for NoOp {
    fn call(&self) -> () {
        unsafe { NoOp::operator_call(&mut *(&raw const *self).cast_mut()) }
    }
}
pub unsafe fn test_alloc_11(
    lhs_alloc: Option<*const ()>,
    rhs_alloc: Option<*const ()>,
    mut check_alloc_invariant: Option<NoOp>,
) -> bool {
    let mut __lhs_alloc_default: Option<()> = None;
    let mut lhs_alloc: *const () = lhs_alloc
        .unwrap_or_else(|| unsafe { __lhs_alloc_default.insert(unimplemented!()) as *const () });
    let mut __rhs_alloc_default: Option<()> = None;
    let mut rhs_alloc: *const () = rhs_alloc
        .unwrap_or_else(|| unsafe { __rhs_alloc_default.insert(unimplemented!()) as *const () });
    let mut check_alloc_invariant: NoOp =
        check_alloc_invariant.unwrap_or_else(|| unsafe { <NoOp>::default() });
    {
        {
            let mut arr: [(i32, i32); 3] = [(1, 1), (2, 3), (3, 6)];
            let orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            let mut copy: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            copy = orig.clone();
            assert!(((copy.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(copy.begin(), 3_isize) == copy.end());
            assert!(((orig.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(orig.begin(), 3_isize) == orig.end());
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            let mut copy: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            copy = orig.clone();
            assert!(((copy.len()) == (0_usize)));
            assert!(copy.begin() == copy.end());
            assert!(((orig.len()) == (0_usize)));
            assert!(orig.begin() == orig.end());
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut arr: [(i32, i32); 3] = [(1, 1), (2, 3), (3, 6)];
            let orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            let mut copy: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            copy = orig.clone();
            assert!(((copy.len()) == (0_usize)));
            assert!(copy.begin() == copy.end());
            assert!(((orig.len()) == (0_usize)));
            assert!(orig.begin() == orig.end());
        }
    }
    {
        {
            let mut arr: [(i32, i32); 3] = [(1, 1), (2, 3), (3, 6)];
            let mut orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            orig = orig.clone();
            assert!(((orig.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(orig.begin(), 3_isize) == orig.end());
        }
        {
            let mut orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            orig = orig.clone();
            assert!(((orig.len()) == (0_usize)));
            assert!(orig.begin() == orig.end());
        }
    }
    {
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(i32, i32); 3] = [(1, 1), (2, 3), (3, 6)];
            let orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            let mut rhs_arr: [(i32, i32); 2] = [(4, 2), (5, 1)];
            let mut copy: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            copy = orig.clone();
            assert!(((copy.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(copy.begin(), 3_isize) == copy.end());
            assert!(((orig.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(orig.begin(), 3_isize) == orig.end());
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(i32, i32); 3] = [(1, 1), (2, 3), (3, 6)];
            let orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            let mut rhs_arr: [(i32, i32); 3] = [(4, 2), (5, 1), (6, 0)];
            let mut copy: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            copy = orig.clone();
            assert!(((copy.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(copy.begin(), 3_isize) == copy.end());
            assert!(((orig.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(orig.begin(), 3_isize) == orig.end());
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(i32, i32); 3] = [(1, 1), (2, 3), (3, 6)];
            let orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            let mut rhs_arr: [(i32, i32); 4] = [(4, 2), (5, 1), (6, 0), (7, 9)];
            let mut copy: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            copy = orig.clone();
            assert!(((copy.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(copy.begin(), 3_isize) == copy.end());
            assert!(((orig.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(orig.begin(), 3_isize) == orig.end());
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
    }
    {
        let mut orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
        let mut i: i32 = 0;
        'loop_: while ((i) != (50)) {
            (*orig.subscript(&i)) = ((i) + (3));
            i.prefix_inc();
        }
        let mut copy: UnsafeMap<i32, (i32, i32)> = unimplemented!();
        copy = orig.clone();
        let mut i: i32 = 0;
        'loop_: for v in IterRange::new(copy.begin(), copy.end()) {
            let mut v: (i32, i32) = {
                let __a0 = (*v.iter_deref()).clone();
                ((*&__a0.0), (*&__a0.1))
            };
            assert!(unimplemented!());
            i.prefix_inc();
        }
    }
    (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
    return true;
}
pub unsafe fn test_alloc_12(
    lhs_alloc: Option<*const min_allocator_std_pair_const_int__int__>,
    rhs_alloc: Option<*const min_allocator_std_pair_const_int__int__>,
    mut check_alloc_invariant: Option<NoOp>,
) -> bool {
    let mut __lhs_alloc_default: Option<min_allocator_std_pair_const_int__int__> = None;
    let mut lhs_alloc: *const min_allocator_std_pair_const_int__int__ =
        lhs_alloc.unwrap_or_else(|| unsafe {
            __lhs_alloc_default.insert(<min_allocator_std_pair_const_int__int__>::default())
                as *const min_allocator_std_pair_const_int__int__
        });
    let mut __rhs_alloc_default: Option<min_allocator_std_pair_const_int__int__> = None;
    let mut rhs_alloc: *const min_allocator_std_pair_const_int__int__ =
        rhs_alloc.unwrap_or_else(|| unsafe {
            __rhs_alloc_default.insert(<min_allocator_std_pair_const_int__int__>::default())
                as *const min_allocator_std_pair_const_int__int__
        });
    let mut check_alloc_invariant: NoOp =
        check_alloc_invariant.unwrap_or_else(|| unsafe { <NoOp>::default() });
    {
        {
            let mut arr: [(i32, i32); 3] = [(1, 1), (2, 3), (3, 6)];
            let orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            let mut copy: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            copy = orig.clone();
            assert!(((copy.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(copy.begin(), 3_isize) == copy.end());
            assert!(((orig.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(orig.begin(), 3_isize) == orig.end());
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            let mut copy: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            copy = orig.clone();
            assert!(((copy.len()) == (0_usize)));
            assert!(copy.begin() == copy.end());
            assert!(((orig.len()) == (0_usize)));
            assert!(orig.begin() == orig.end());
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut arr: [(i32, i32); 3] = [(1, 1), (2, 3), (3, 6)];
            let orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            let mut copy: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            copy = orig.clone();
            assert!(((copy.len()) == (0_usize)));
            assert!(copy.begin() == copy.end());
            assert!(((orig.len()) == (0_usize)));
            assert!(orig.begin() == orig.end());
        }
    }
    {
        {
            let mut arr: [(i32, i32); 3] = [(1, 1), (2, 3), (3, 6)];
            let mut orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            orig = orig.clone();
            assert!(((orig.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(orig.begin(), 3_isize) == orig.end());
        }
        {
            let mut orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            orig = orig.clone();
            assert!(((orig.len()) == (0_usize)));
            assert!(orig.begin() == orig.end());
        }
    }
    {
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(i32, i32); 3] = [(1, 1), (2, 3), (3, 6)];
            let orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            let mut rhs_arr: [(i32, i32); 2] = [(4, 2), (5, 1)];
            let mut copy: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            copy = orig.clone();
            assert!(((copy.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(copy.begin(), 3_isize) == copy.end());
            assert!(((orig.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(orig.begin(), 3_isize) == orig.end());
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(i32, i32); 3] = [(1, 1), (2, 3), (3, 6)];
            let orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            let mut rhs_arr: [(i32, i32); 3] = [(4, 2), (5, 1), (6, 0)];
            let mut copy: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            copy = orig.clone();
            assert!(((copy.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(copy.begin(), 3_isize) == copy.end());
            assert!(((orig.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(orig.begin(), 3_isize) == orig.end());
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(i32, i32); 3] = [(1, 1), (2, 3), (3, 6)];
            let orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            let mut rhs_arr: [(i32, i32); 4] = [(4, 2), (5, 1), (6, 0), (7, 9)];
            let mut copy: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            copy = orig.clone();
            assert!(((copy.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(copy.begin(), 3_isize) == copy.end());
            assert!(((orig.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(orig.begin(), 3_isize) == orig.end());
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
    }
    {
        let mut orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
        let mut i: i32 = 0;
        'loop_: while ((i) != (50)) {
            (*orig.subscript(&i)) = ((i) + (3));
            i.prefix_inc();
        }
        let mut copy: UnsafeMap<i32, (i32, i32)> = unimplemented!();
        copy = orig.clone();
        let mut i: i32 = 0;
        'loop_: for v in IterRange::new(copy.begin(), copy.end()) {
            let mut v: (i32, i32) = {
                let __a0 = (*v.iter_deref()).clone();
                ((*&__a0.0), (*&__a0.1))
            };
            assert!(unimplemented!());
            i.prefix_inc();
        }
    }
    (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
    return true;
}
pub unsafe fn test_alloc_13(
    lhs_alloc: *const tracking_allocator_std_pair_const_int__int__,
    rhs_alloc: *const tracking_allocator_std_pair_const_int__int__,
    mut check_alloc_invariant: AssertEmpty_14,
) -> bool {
    {
        {
            let mut arr: [(i32, i32); 3] = [(1, 1), (2, 3), (3, 6)];
            let orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            let mut copy: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            copy = orig.clone();
            assert!(((copy.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(copy.begin(), 3_isize) == copy.end());
            assert!(((orig.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(orig.begin(), 3_isize) == orig.end());
        }
        (unsafe { AssertEmpty_14::operator_call(&mut check_alloc_invariant) });
        {
            let orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            let mut copy: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            copy = orig.clone();
            assert!(((copy.len()) == (0_usize)));
            assert!(copy.begin() == copy.end());
            assert!(((orig.len()) == (0_usize)));
            assert!(orig.begin() == orig.end());
        }
        (unsafe { AssertEmpty_14::operator_call(&mut check_alloc_invariant) });
        {
            let mut arr: [(i32, i32); 3] = [(1, 1), (2, 3), (3, 6)];
            let orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            let mut copy: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            copy = orig.clone();
            assert!(((copy.len()) == (0_usize)));
            assert!(copy.begin() == copy.end());
            assert!(((orig.len()) == (0_usize)));
            assert!(orig.begin() == orig.end());
        }
    }
    {
        {
            let mut arr: [(i32, i32); 3] = [(1, 1), (2, 3), (3, 6)];
            let mut orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            orig = orig.clone();
            assert!(((orig.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(orig.begin(), 3_isize) == orig.end());
        }
        {
            let mut orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            orig = orig.clone();
            assert!(((orig.len()) == (0_usize)));
            assert!(orig.begin() == orig.end());
        }
    }
    {
        (unsafe { AssertEmpty_14::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(i32, i32); 3] = [(1, 1), (2, 3), (3, 6)];
            let orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            let mut rhs_arr: [(i32, i32); 2] = [(4, 2), (5, 1)];
            let mut copy: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            copy = orig.clone();
            assert!(((copy.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(copy.begin(), 3_isize) == copy.end());
            assert!(((orig.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(orig.begin(), 3_isize) == orig.end());
        }
        (unsafe { AssertEmpty_14::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(i32, i32); 3] = [(1, 1), (2, 3), (3, 6)];
            let orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            let mut rhs_arr: [(i32, i32); 3] = [(4, 2), (5, 1), (6, 0)];
            let mut copy: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            copy = orig.clone();
            assert!(((copy.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(copy.begin(), 3_isize) == copy.end());
            assert!(((orig.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(orig.begin(), 3_isize) == orig.end());
        }
        (unsafe { AssertEmpty_14::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(i32, i32); 3] = [(1, 1), (2, 3), (3, 6)];
            let orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            let mut rhs_arr: [(i32, i32); 4] = [(4, 2), (5, 1), (6, 0), (7, 9)];
            let mut copy: UnsafeMap<i32, (i32, i32)> = unimplemented!();
            copy = orig.clone();
            assert!(((copy.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(copy.begin(), 3_isize) == copy.end());
            assert!(((orig.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(orig.begin(), 3_isize) == orig.end());
        }
        (unsafe { AssertEmpty_14::operator_call(&mut check_alloc_invariant) });
    }
    {
        let mut orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
        let mut i: i32 = 0;
        'loop_: while ((i) != (50)) {
            (*orig.subscript(&i)) = ((i) + (3));
            i.prefix_inc();
        }
        let mut copy: UnsafeMap<i32, (i32, i32)> = unimplemented!();
        copy = orig.clone();
        let mut i: i32 = 0;
        'loop_: for v in IterRange::new(copy.begin(), copy.end()) {
            let mut v: (i32, i32) = {
                let __a0 = (*v.iter_deref()).clone();
                ((*&__a0.0), (*&__a0.1))
            };
            assert!(unimplemented!());
            i.prefix_inc();
        }
    }
    (unsafe { AssertEmpty_14::operator_call(&mut check_alloc_invariant) });
    return true;
}
pub unsafe fn test_15() -> bool {
    (unsafe { test_alloc_11(None, None, None) });
    (unsafe { test_alloc_12(None, None, None) });
    {
        let mut lhs_allocs: Vec<*mut ::libc::c_void> = Vec::new();
        let mut rhs_allocs: Vec<*mut ::libc::c_void> = Vec::new();
        (unsafe {
            let mut _lhs_alloc: tracking_allocator_std_pair_const_int__int__ =
                tracking_allocator_std_pair_const_int__int__::new({ &mut lhs_allocs });
            let mut _rhs_alloc: tracking_allocator_std_pair_const_int__int__ =
                tracking_allocator_std_pair_const_int__int__::new({ &mut rhs_allocs });
            let _check_alloc_invariant: AssertEmpty_14 =
                AssertEmpty_14::new({ &mut lhs_allocs }, { &mut rhs_allocs });
            test_alloc_13(&mut _lhs_alloc, &mut _rhs_alloc, _check_alloc_invariant)
        });
    }
    {
        {
            let mut arr: [(NonTrivial, NonTrivial); 3] = [
                (NonTrivial::new({ 1 }), NonTrivial::new({ 1 })),
                (NonTrivial::new({ 2 }), NonTrivial::new({ 3 })),
                (NonTrivial::new({ 3 }), NonTrivial::new({ 6 })),
            ];
            let orig: UnsafeMap<NonTrivial, (NonTrivial, NonTrivial)> = unimplemented!();
            let mut copy: UnsafeMap<NonTrivial, (NonTrivial, NonTrivial)> = UnsafeMap::new();
            copy = orig.clone();
            assert!(((copy.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(copy.begin(), 3_isize) == copy.end());
            assert!(((orig.len()) == (3_usize)));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(libcc2rs::next_unsafe(orig.begin(), 3_isize) == orig.end());
        }
    }
    {
        let mut arr: [(i32, i32); 3] = [(1, 1), (2, 3), (3, 6)];
        let orig: UnsafeMap<i32, (i32, i32)> = unimplemented!();
        let mut copy: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
        copy = orig.clone();
        assert!(((copy.len()) == (3_usize)));
        assert!(
            (unsafe {
                let mut _a: test_less_int_ = unimplemented!();
                let mut _b: test_less_int_ = test_less_int_::new_2({ 3 });
                operator_eq_0(&mut _a, &mut _b)
            })
        );
        assert!(((orig.len()) == (3_usize)));
        assert!(
            (unsafe {
                let mut _a: test_less_int_ = unimplemented!();
                let mut _b: test_less_int_ = test_less_int_::new_2({ 3 });
                operator_eq_0(&mut _a, &mut _b)
            })
        );
    }
    {
        let mut in_: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
        let mut i: i32 = 0;
        'loop_: while ((i) != (32)) {
            (*in_.subscript(&i)) = i;
            i.prefix_inc();
        }
        let mut out: UnsafeMap<i32, (i32, i32)> = in_.clone();
        unimplemented!();
        out = in_.clone();
        unimplemented!();
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct AssertEmpty_14 {
    lhs_allocs_: *mut Vec<*mut ::libc::c_void>,
    rhs_allocs_: *mut Vec<*mut ::libc::c_void>,
}
impl AssertEmpty_14 {
    pub unsafe fn new(
        lhs_allocs: *mut Vec<*mut ::libc::c_void>,
        rhs_allocs: *mut Vec<*mut ::libc::c_void>,
    ) -> Self {
        let mut this = Self {
            lhs_allocs_: (lhs_allocs),
            rhs_allocs_: (rhs_allocs),
        };
        this
    }
    pub unsafe fn operator_call(&mut self) {
        assert!((*(self.lhs_allocs_).cast_const()).is_empty());
        assert!((*(self.rhs_allocs_).cast_const()).is_empty());
    }
}
impl Callable0<()> for AssertEmpty_14 {
    fn call(&self) -> () {
        unsafe { AssertEmpty_14::operator_call(&mut *(&raw const *self).cast_mut()) }
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
    (unsafe { test_15() });
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
