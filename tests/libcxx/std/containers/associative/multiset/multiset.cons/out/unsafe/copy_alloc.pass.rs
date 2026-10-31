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
pub unsafe fn operator_eq_4(mut _a0: min_allocator_int_, mut _a1: min_allocator_int_) -> bool {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
impl std::cmp::PartialEq for min_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_4(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_int_ {}
pub unsafe fn operator_eq_5(a: *const test_less_int_, b: *const test_less_int_) -> bool {
    return (((*a).data_) == ((*b).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_less_int_ {
    pub data_: i32,
}
impl test_less_int_ {
    pub unsafe fn new_1(mut data: i32) -> Self {
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
            operator_eq_5(
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
pub static mut destructed_value_6: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { -1_i32 });
pub static mut moved_value_7: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 2147483647 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_8(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_9(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_8(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6));
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
                data_: 0,
                id_: 0,
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_10(x: *const test_allocator_int_, y: *const test_allocator_int_) -> bool {
    return (((*x).data_) == ((*y).data_));
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
                != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6)))
                && (((*a).id_)
                    != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6))))
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
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6)))
        );
        assert!(
            ((self.id_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_6));
    }
}
impl std::cmp::PartialEq for test_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_10(
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
pub unsafe fn test_alloc_11(new_alloc: *const ()) {
    {
        let mut arr: [i32; 3] = [1, 2, 2];
        let orig: () = unimplemented!();
        let mut copy: () = unimplemented!();
        assert!(((unimplemented!()) == (3_())));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
        assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
        assert!(unimplemented!());
        assert!(((unimplemented!()) == (3_())));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
        assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
    }
    {
        let orig: () = unimplemented!();
        let mut copy: () = unimplemented!();
        assert!(((unimplemented!()) == (0_())));
        assert!(unimplemented!() == unimplemented!());
        assert!(((unimplemented!()) == (0_())));
        assert!(unimplemented!() == unimplemented!());
    }
    {
        let mut arr: [i32; 5] = [1, 2, 3, 3, 5];
        let orig: () = unimplemented!();
        let mut copy: () = unimplemented!();
        assert!(((unimplemented!()) == (5_())));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (3)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()) == (3)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()) == (5)));
        assert!(libcc2rs::next_unsafe(unimplemented!(), 5_isize) == unimplemented!());
        assert!(((unimplemented!()) == (5_())));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (3)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()) == (3)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()) == (5)));
        assert!(libcc2rs::next_unsafe(unimplemented!(), 5_isize) == unimplemented!());
    }
}
pub unsafe fn test_alloc_12(new_alloc: *const test_allocator_int_) {
    {
        let mut arr: [i32; 3] = [1, 2, 2];
        let orig: () = unimplemented!();
        let mut copy: () = unimplemented!();
        assert!(((unimplemented!()) == (3_())));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
        assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
        assert!(
            (unsafe {
                let mut _x: test_allocator_int_ = unimplemented!();
                let _y: *const test_allocator_int_ = new_alloc;
                operator_eq_10(&mut _x, _y)
            })
        );
        assert!(((unimplemented!()) == (3_())));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
        assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
    }
    {
        let orig: () = unimplemented!();
        let mut copy: () = unimplemented!();
        assert!(((unimplemented!()) == (0_())));
        assert!(unimplemented!() == unimplemented!());
        assert!(((unimplemented!()) == (0_())));
        assert!(unimplemented!() == unimplemented!());
    }
    {
        let mut arr: [i32; 5] = [1, 2, 3, 3, 5];
        let orig: () = unimplemented!();
        let mut copy: () = unimplemented!();
        assert!(((unimplemented!()) == (5_())));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (3)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()) == (3)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()) == (5)));
        assert!(libcc2rs::next_unsafe(unimplemented!(), 5_isize) == unimplemented!());
        assert!(((unimplemented!()) == (5_())));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (3)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()) == (3)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()) == (5)));
        assert!(libcc2rs::next_unsafe(unimplemented!(), 5_isize) == unimplemented!());
    }
}
pub unsafe fn test_alloc_13(new_alloc: *const min_allocator_int_) {
    {
        let mut arr: [i32; 3] = [1, 2, 2];
        let orig: () = unimplemented!();
        let mut copy: () = unimplemented!();
        assert!(((unimplemented!()) == (3_())));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
        assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
        assert!(
            (unsafe {
                let _arg0: min_allocator_int_ = unimplemented!();
                let _arg1: min_allocator_int_ = (*new_alloc);
                operator_eq_4(_arg0, _arg1)
            })
        );
        assert!(((unimplemented!()) == (3_())));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
        assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
    }
    {
        let orig: () = unimplemented!();
        let mut copy: () = unimplemented!();
        assert!(((unimplemented!()) == (0_())));
        assert!(unimplemented!() == unimplemented!());
        assert!(((unimplemented!()) == (0_())));
        assert!(unimplemented!() == unimplemented!());
    }
    {
        let mut arr: [i32; 5] = [1, 2, 3, 3, 5];
        let orig: () = unimplemented!();
        let mut copy: () = unimplemented!();
        assert!(((unimplemented!()) == (5_())));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (3)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()) == (3)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()) == (5)));
        assert!(libcc2rs::next_unsafe(unimplemented!(), 5_isize) == unimplemented!());
        assert!(((unimplemented!()) == (5_())));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (3)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()) == (3)));
        assert!(((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()) == (5)));
        assert!(libcc2rs::next_unsafe(unimplemented!(), 5_isize) == unimplemented!());
    }
}
pub unsafe fn test_14() -> bool {
    (unsafe {
        let mut _new_alloc: () = unimplemented!();
        test_alloc_11(&mut _new_alloc)
    });
    (unsafe {
        let mut _new_alloc: test_allocator_int_ = test_allocator_int_::new_1({ 25 });
        test_alloc_12(&mut _new_alloc)
    });
    (unsafe {
        let mut _new_alloc: min_allocator_int_ = <min_allocator_int_>::default();
        test_alloc_13(&mut _new_alloc)
    });
    {
        let mut arr: [i32; 3] = [1, 2, 2];
        let orig: () = unimplemented!();
        let mut copy: () = unimplemented!();
        assert!(((unimplemented!()) == (3_())));
        assert!(
            (unsafe {
                let mut _a: test_less_int_ = unimplemented!();
                let mut _b: test_less_int_ = test_less_int_::new_1({ 3 });
                operator_eq_5(&mut _a, &mut _b)
            })
        );
        assert!(((unimplemented!()) == (3_())));
        assert!(
            (unsafe {
                let mut _a: test_less_int_ = unimplemented!();
                let mut _b: test_less_int_ = test_less_int_::new_1({ 3 });
                operator_eq_5(&mut _a, &mut _b)
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
    (unsafe { test_14() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
    std::cell::LazyCell::force(&*&raw const destructed_value_6);
    std::cell::LazyCell::force(&*&raw const moved_value_7);
}
