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
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_void_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_deleter_base {}
pub static mut count_4: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_5: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct test_deleter_int_ {
    state_: (),
}
impl test_deleter_int_ {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct test_deleter_A_ {
    state_: (),
}
impl test_deleter_A_ {
    pub unsafe fn new_1(mut s: ()) -> Self {
        let mut this = Self { state_: s };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_4)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(d: *const test_deleter_A_) -> Self {
        let mut this = Self {
            state_: (*d).state_,
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_4)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_4)).prefix_dec();
        self.state_ = -1_();
    }
    pub unsafe fn state(&self) -> () {
        return self.state_;
    }
    pub unsafe fn operator_call(&mut self, mut p: *mut A) {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5)).prefix_inc();
        {
            let __p = p;
            if !__p.is_null() {
                A::destructor(&mut *__p);
                ::std::mem::drop(Box::from_raw(__p))
            }
        };
    }
}
impl Clone for test_deleter_A_ {
    fn clone(&self) -> Self {
        unsafe { test_deleter_A_::copy_from(self as *const test_deleter_A_) }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct test_deleter_const_A_ {
    state_: (),
}
impl test_deleter_const_A_ {
    pub unsafe fn new_1(mut s: ()) -> Self {
        let mut this = Self { state_: s };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_4)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(d: *const test_deleter_const_A_) -> Self {
        let mut this = Self {
            state_: (*d).state_,
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_4)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_4)).prefix_dec();
        self.state_ = -1_();
    }
    pub unsafe fn operator_call(&mut self, mut p: *const A) {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5)).prefix_inc();
        {
            let __p = p;
            if !__p.is_null() {
                A::destructor(&mut *__p);
                ::std::mem::drop(Box::from_raw(__p.cast_mut()))
            }
        };
    }
}
impl Clone for test_deleter_const_A_ {
    fn clone(&self) -> Self {
        unsafe { test_deleter_const_A_::copy_from(self as *const test_deleter_const_A_) }
    }
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
pub static mut destructed_value_6: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_7: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_8(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_9(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_8(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6));
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
            operator_eq_8(
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
#[repr(C)]
#[derive(FnPtrArg)]
pub struct test_allocator_int_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_int_ {}
impl Default for test_allocator_int_ {
    fn default() -> Self {
        unsafe {
            test_allocator_int_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct test_allocator_A_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_A_ {
    pub unsafe fn new_1(mut data: (), mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
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
        unimplemented!();
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).copied.prefix_inc();
        }
        this
    }
    pub unsafe fn move_from(a: *mut test_allocator_A_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).moved.prefix_inc();
        }
        unimplemented!();
        (*a).id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_value_7));
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6));
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
                data_: unimplemented!(),
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
pub struct bad_type {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct bad_deleter {}
impl bad_deleter {
    pub unsafe fn operator_call(&mut self, mut _a0: bad_type) {}
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct no_move_deleter {}
impl no_move_deleter {
    pub unsafe fn operator_call(&mut self, mut _a0: *mut ()) {}
}
const _: () = assert!(true, "!std::is_move_constructible<no_move_deleter>::value");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct no_nullptr_deleter {}
impl no_nullptr_deleter {
    pub unsafe fn operator_call_1(&self, mut _a0: *mut ()) {}
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct base {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct derived {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct A {}
impl A {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_10)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(_a0: *const A) -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_10)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_10)).prefix_dec();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const A) -> *mut A {
        return &mut (*(self as *mut A));
    }
}
impl Clone for A {
    fn clone(&self) -> Self {
        unsafe { A::copy_from(self as *const A) }
    }
}
impl Default for A {
    fn default() -> Self {
        unsafe { A::new() }
    }
}
pub static mut count_10: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
const _: () = assert!(
    true,
    "std::is_constructible<std::shared_ptr<int>,  std::nullptr_t, test_deleter<int>, test_allocator<int> >::value"
);
const _: () = assert!(
    true,
    "!std::is_constructible<std::shared_ptr<int>,  std::nullptr_t, bad_deleter, test_allocator<int> >::value"
);
const _: () = assert!(
    true,
    "!std::is_constructible<std::shared_ptr<int>,  std::nullptr_t, no_nullptr_deleter, test_allocator<int> >::value"
);
const _: () = assert!(
    true,
    "!std::is_constructible<std::shared_ptr<int>,  std::nullptr_t, no_move_deleter, test_allocator<int> >::value"
);
const _: () = assert!(
    true,
    "std::is_constructible<std::shared_ptr<int[]>,  std::nullptr_t, test_deleter<int>, test_allocator<int> >::value"
);
const _: () = assert!(
    true,
    "!std::is_constructible<std::shared_ptr<int[]>,  std::nullptr_t, bad_deleter, test_allocator<int> >::value"
);
const _: () = assert!(
    true,
    "!std::is_constructible<std::shared_ptr<int[]>,  std::nullptr_t, no_nullptr_deleter, test_allocator<int> >::value"
);
const _: () = assert!(
    true,
    "!std::is_constructible<std::shared_ptr<int[]>,  std::nullptr_t, no_move_deleter, test_allocator<int> >::value"
);
const _: () = assert!(
    true,
    "std::is_constructible<std::shared_ptr<int[5]>,  std::nullptr_t, test_deleter<int>, test_allocator<int> >::value"
);
const _: () = assert!(
    true,
    "!std::is_constructible<std::shared_ptr<int[5]>,  std::nullptr_t, bad_deleter, test_allocator<int> >::value"
);
const _: () = assert!(
    true,
    "!std::is_constructible<std::shared_ptr<int[5]>,  std::nullptr_t, no_nullptr_deleter, test_allocator<int> >::value"
);
const _: () = assert!(
    true,
    "!std::is_constructible<std::shared_ptr<int[5]>,  std::nullptr_t, no_move_deleter, test_allocator<int> >::value"
);
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
    let mut alloc_stats: test_allocator_statistics = <test_allocator_statistics>::default();
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut d: *mut test_deleter_A_ = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5)) = 0_();
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut d: *mut test_deleter_A_ = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5)) = 0_();
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut d: *mut test_deleter_A_ = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut p: () = unimplemented!();
        &(p);
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
    std::cell::LazyCell::force(&*&raw const count_4);
    std::cell::LazyCell::force(&*&raw const dealloc_count_5);
    std::cell::LazyCell::force(&*&raw const destructed_value_6);
    std::cell::LazyCell::force(&*&raw const moved_value_7);
    std::cell::LazyCell::force(&*&raw const count_10);
}
