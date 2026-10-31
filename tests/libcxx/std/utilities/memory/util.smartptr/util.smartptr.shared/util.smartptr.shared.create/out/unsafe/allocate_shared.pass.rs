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
pub struct bare_allocator_void_ {}
impl bare_allocator_void_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
}
impl Default for bare_allocator_void_ {
    fn default() -> Self {
        unsafe { bare_allocator_void_::new() }
    }
}
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
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_operator_hijacker__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut operator_hijacker,
}
impl min_pointer_operator_hijacker__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_void_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_operator_hijacker_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct complete_type_allocator_int_ {}
impl complete_type_allocator_int_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
}
impl Default for complete_type_allocator_int_ {
    fn default() -> Self {
        unsafe { complete_type_allocator_int_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct operator_hijacker {}
impl operator_hijacker {
    pub unsafe fn operator_lt(&self, _a0: *const operator_hijacker) -> bool {
        return true;
    }
    pub unsafe fn operator_eq(&self, _a0: *const operator_hijacker) -> bool {
        return true;
    }
    pub unsafe fn operator_call(&self) -> i32 {
        return 42;
    }
}
impl std::cmp::Ord for operator_hijacker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_hijacker::operator_lt(self, other as *const operator_hijacker) {
                std::cmp::Ordering::Less
            } else if operator_hijacker::operator_lt(other, self as *const operator_hijacker) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for operator_hijacker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for operator_hijacker {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_hijacker::operator_eq(self, other as *const operator_hijacker) }
    }
}
impl std::cmp::Eq for operator_hijacker {}
const _: () = assert!(
    true,
    "std::is_trivially_copyable<operator_hijacker>::value &&     //\n                  std::is_copy_constructible<operator_hijacker>::value && //\n                  std::is_move_constructible<operator_hijacker>::value && //\n                  std::is_copy_assignable<operator_hijacker>::value &&    //\n                  std::is_move_assignable<operator_hijacker>::value"
);
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
impl test_allocator_void_ {
    pub unsafe fn new_1(a: *const test_allocator_void_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        this
    }
}
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_void__rebind_void_ {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, DestructorUnsafe)]
pub struct test_allocator_void_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_void_ {
    pub unsafe fn new_2(mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: 0,
            id_: 0,
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_3(mut data: i32) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_4(mut data: i32, mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_5(mut data: i32, mut id: i32) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_6(
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
pub struct test_allocator_A__rebind_A_ {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_A_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_A_ {
    pub unsafe fn new_1(mut data: i32, mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: stats,
        };
        if !((stats).is_null()) {
            (*this.stats_).count.prefix_inc();
        }
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_A_) -> Self {
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
    pub unsafe fn destroy(&mut self, mut p: *mut A) {
        if !((self.stats_).is_null()) {
            (*self.stats_).destroy_count.prefix_inc();
        }
        (unsafe { A::destructor(&mut (*p)) });
    }
}
impl Clone for test_allocator_A_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_A_::copy_from(self as *const test_allocator_A_) }
    }
}
impl Default for test_allocator_A_ {
    fn default() -> Self {
        unsafe {
            test_allocator_A_ {
                data_: 0_i32,
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
pub static mut new_count_8: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe, Default)]
pub struct A {
    int_: i32,
    char_: libc::c_char,
}
impl A {
    pub unsafe fn new(mut i: i32, mut c: libc::c_char) -> Self {
        let mut this = Self { int_: i, char_: c };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_9)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(a: *const A) -> Self {
        let mut this = Self {
            int_: (*a).int_,
            char_: (*a).char_,
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_9)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_9)).prefix_dec();
    }
    pub unsafe fn get_int(&self) -> i32 {
        return self.int_;
    }
    pub unsafe fn get_char(&self) -> libc::c_char {
        return self.char_;
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const A) -> *mut A {
        self.int_ = (*_a0).int_;
        self.char_ = (*_a0).char_;
        return &mut (*(self as *mut A));
    }
}
impl Clone for A {
    fn clone(&self) -> Self {
        unsafe { A::copy_from(self as *const A) }
    }
}
pub static mut count_9: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct Zero {}
impl Zero {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_10)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(_a0: *const Zero) -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_10)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_10)).prefix_dec();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Zero) -> *mut Zero {
        return &mut (*(self as *mut Zero));
    }
}
impl Clone for Zero {
    fn clone(&self) -> Self {
        unsafe { Zero::copy_from(self as *const Zero) }
    }
}
impl Default for Zero {
    fn default() -> Self {
        unsafe { Zero::new() }
    }
}
pub static mut count_10: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe, Default)]
pub struct One {
    pub value: i32,
}
impl One {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_11)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(o: *const One) -> Self {
        let mut this = Self { value: (*o).value };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_11)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_11)).prefix_dec();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const One) -> *mut One {
        self.value = (*_a0).value;
        return &mut (*(self as *mut One));
    }
}
impl Clone for One {
    fn clone(&self) -> Self {
        unsafe { One::copy_from(self as *const One) }
    }
}
pub static mut count_11: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe, Default)]
pub struct Two {
    pub value: i32,
}
impl Two {
    pub unsafe fn new(mut v: i32, mut _a1: i32) -> Self {
        let mut this = Self { value: v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(o: *const Two) -> Self {
        let mut this = Self { value: (*o).value };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12)).prefix_dec();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Two) -> *mut Two {
        self.value = (*_a0).value;
        return &mut (*(self as *mut Two));
    }
}
impl Clone for Two {
    fn clone(&self) -> Self {
        unsafe { Two::copy_from(self as *const Two) }
    }
}
pub static mut count_12: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe, Default)]
pub struct Three {
    pub value: i32,
}
impl Three {
    pub unsafe fn new(mut v: i32, mut _a1: i32, mut _a2: i32) -> Self {
        let mut this = Self { value: v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_13)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(o: *const Three) -> Self {
        let mut this = Self { value: (*o).value };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_13)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_13)).prefix_dec();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Three) -> *mut Three {
        self.value = (*_a0).value;
        return &mut (*(self as *mut Three));
    }
}
impl Clone for Three {
    fn clone(&self) -> Self {
        unsafe { Three::copy_from(self as *const Three) }
    }
}
pub static mut count_13: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn test_14() {
    let bad: i32 = -1_i32;
    {
        let mut p: () = unimplemented!();
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_10)) == (1)));
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_10)) == (0)));
    {
        let i: i32 = 42;
        let mut p: () = unimplemented!();
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_11)) == (1)));
        assert!((((*unimplemented!()).value) == (i)));
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_11)) == (0)));
    {
        let i: i32 = 42;
        let mut p: () = unimplemented!();
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_12)) == (1)));
        assert!((((*unimplemented!()).value) == (i)));
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_12)) == (0)));
    {
        let i: i32 = 42;
        let mut p: () = unimplemented!();
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_13)) == (1)));
        assert!((((*unimplemented!()).value) == (i)));
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_13)) == (0)));
}
pub unsafe fn test_15() {
    let bad: i32 = -1_i32;
    {
        let mut p: () = unimplemented!();
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_10)) == (1)));
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_10)) == (0)));
    {
        let i: i32 = 42;
        let mut p: () = unimplemented!();
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_11)) == (1)));
        assert!((((*unimplemented!()).value) == (i)));
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_11)) == (0)));
    {
        let i: i32 = 42;
        let mut p: () = unimplemented!();
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_12)) == (1)));
        assert!((((*unimplemented!()).value) == (i)));
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_12)) == (0)));
    {
        let i: i32 = 42;
        let mut p: () = unimplemented!();
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_13)) == (1)));
        assert!((((*unimplemented!()).value) == (i)));
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_13)) == (0)));
}
pub unsafe fn test_16() {
    let bad: i32 = -1_i32;
    {
        let mut p: () = unimplemented!();
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_10)) == (1)));
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_10)) == (0)));
    {
        let i: i32 = 42;
        let mut p: () = unimplemented!();
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_11)) == (1)));
        assert!((((*unimplemented!()).value) == (i)));
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_11)) == (0)));
    {
        let i: i32 = 42;
        let mut p: () = unimplemented!();
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_12)) == (1)));
        assert!((((*unimplemented!()).value) == (i)));
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_12)) == (0)));
    {
        let i: i32 = 42;
        let mut p: () = unimplemented!();
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_13)) == (1)));
        assert!((((*unimplemented!()).value) == (i)));
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_13)) == (0)));
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
    (unsafe { test_14() });
    (unsafe { test_15() });
    (unsafe { test_16() });
    let mut alloc_stats: test_allocator_statistics = <test_allocator_statistics>::default();
    {
        let mut i: i32 = 67;
        let mut c: libc::c_char = ('e' as libc::c_char);
        let mut p: () = unimplemented!();
        assert!(((alloc_stats.alloc_count) == (1)));
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_9)) == (1)));
        assert!(((unsafe { A::get_int(&(*(unimplemented!()).cast_const()),) }) == (67)));
        assert!(
            (((unsafe { A::get_char(&(*(unimplemented!()).cast_const()),) }) as i32)
                == (('e' as libc::c_char) as i32))
        );
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_9)) == (0)));
    assert!(((alloc_stats.alloc_count) == (0)));
    {
        let mut i: i32 = 67;
        let mut c: libc::c_char = ('e' as libc::c_char);
        let mut p: () = unimplemented!();
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_9)) == (1)));
        assert!(((unsafe { A::get_int(&(*(unimplemented!()).cast_const()),) }) == (67)));
        assert!(
            (((unsafe { A::get_char(&(*(unimplemented!()).cast_const()),) }) as i32)
                == (('e' as libc::c_char) as i32))
        );
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_9)) == (0)));
    {
        let mut i: i32 = 68;
        let mut c: libc::c_char = ('f' as libc::c_char);
        let mut p: () = unimplemented!();
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_9)) == (1)));
        assert!(((unsafe { A::get_int(&(*(unimplemented!()).cast_const()),) }) == (68)));
        assert!(
            (((unsafe { A::get_char(&(*(unimplemented!()).cast_const()),) }) as i32)
                == (('f' as libc::c_char) as i32))
        );
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_9)) == (0)));
    {
        let mut p1: () = unimplemented!();
        let mut p2: () = unimplemented!();
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
    std::cell::LazyCell::force(&*&raw const destructed_value_4);
    std::cell::LazyCell::force(&*&raw const moved_value_5);
    std::cell::LazyCell::force(&*&raw const new_count_8);
    std::cell::LazyCell::force(&*&raw const count_9);
    std::cell::LazyCell::force(&*&raw const count_10);
    std::cell::LazyCell::force(&*&raw const count_11);
    std::cell::LazyCell::force(&*&raw const count_12);
    std::cell::LazyCell::force(&*&raw const count_13);
}
