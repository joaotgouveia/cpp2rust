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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_int__rebind_int_ptr_ {}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct test_allocator_int_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_int_ {
    pub unsafe fn move_from(a: *mut test_allocator_int_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).moved.prefix_inc();
        }
        assert!(
            ((((*a).data_)
                != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1)))
                && (((*a).id_)
                    != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1))))
                && (!(c"moving from destroyed allocator".as_ptr()).is_null())
        );
        (*a).id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_value_2));
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
    pub unsafe fn construct_16_int(&mut self, mut p: *mut i32, val: *mut i32) {
        if !((self.stats_).is_null()) {
            (*self.stats_).construct_count.prefix_inc();
        }
        (unsafe {
            let ___location: *mut i32 = (unsafe { to_address_5(p) });
            let ___args: *mut i32 = val;
            construct_at_6(___location, ___args)
        });
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
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_8))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_9)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_8))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_9))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_10)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_7)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_8)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_9)) = 0_usize;
    }
}
pub static mut outstanding_bytes_7: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_8: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_9: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_10: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_char_ {}
impl min_allocator_char_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            (unsafe { <std_allocator_char_>::default().allocate(n) })
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_char__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        (unsafe { <std_allocator_char_>::default().deallocate(p.ptr_, n) });
    }
}
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
const _: () = assert!(true, "HasStdErase<std::vector<int>>");
const _: () = assert!(!(false), "!HasStdErase<std::flat_map<int, int>>");
pub unsafe fn test_11() -> bool {
    (unsafe { test_12() });
    (unsafe { test_13() });
    (unsafe { test_14() });
    {
        (unsafe { test_15() });
        (unsafe { test_16() });
    }
    (unsafe { test_17() });
    (unsafe { test_18() });
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
    (unsafe { test_11() });
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_char__std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_1);
    std::cell::LazyCell::force(&*&raw const moved_value_2);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_7);
    std::cell::LazyCell::force(&*&raw const alloc_count_8);
    std::cell::LazyCell::force(&*&raw const dealloc_count_9);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_10);
}
