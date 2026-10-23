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
pub static mut destructed_value_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_2(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_3(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_2(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
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
            operator_eq_2(
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
pub unsafe fn operator_eq_4(x: *const test_allocator_int_, y: *const test_allocator_int_) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_int__rebind_int_ {}
#[repr(C)]
#[derive(FnPtrArg, DestructorUnsafe)]
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
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
    }
}
impl std::cmp::PartialEq for test_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_4(
                self as *const test_allocator_int_,
                other as *const test_allocator_int_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_int_ {}
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
pub unsafe fn operator_eq_5(x: *const other_allocator_int_, y: *const other_allocator_int_) -> () {
    return (((*x).data_) == ((*y).data_));
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
}
impl std::cmp::PartialEq for other_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_5(
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
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_10(mut _a0: min_allocator_int_, mut _a1: min_allocator_int_) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
impl std::cmp::PartialEq for min_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_10(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_int_ {}
pub unsafe fn test_11() -> () {
    {
        let t0: [(); 10] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_()];
        let t1: [(); 4] = [10_(), 11_(), 12_(), 13_()];
        let mut c0: () = {
            let mut __tmp_0: test_allocator_int_ = test_allocator_int_::new_1({ 10_() });
            std_forward_list_int__test_allocator_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                { &mut __tmp_0 },
            )
        };
        let mut c1: () = {
            let mut __tmp_1: test_allocator_int_ = test_allocator_int_::new_1({ 10_() });
            std_forward_list_int__test_allocator_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                { &mut __tmp_1 },
            )
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let t0: [(); 10] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_()];
        let t1: [(); 4] = [10_(), 11_(), 12_(), 13_()];
        let mut c0: () = {
            let mut __tmp_2: test_allocator_int_ = test_allocator_int_::new_1({ 10_() });
            std_forward_list_int__test_allocator_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                { &mut __tmp_2 },
            )
        };
        let mut c1: () = {
            let mut __tmp_3: test_allocator_int_ = test_allocator_int_::new_1({ 11_() });
            std_forward_list_int__test_allocator_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                { &mut __tmp_3 },
            )
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let t0: [(); 4] = [10_(), 11_(), 12_(), 13_()];
        let t1: [(); 10] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_()];
        let mut c0: () = {
            let mut __tmp_4: test_allocator_int_ = test_allocator_int_::new_1({ 10_() });
            std_forward_list_int__test_allocator_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                { &mut __tmp_4 },
            )
        };
        let mut c1: () = {
            let mut __tmp_5: test_allocator_int_ = test_allocator_int_::new_1({ 10_() });
            std_forward_list_int__test_allocator_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                { &mut __tmp_5 },
            )
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let t0: [(); 4] = [10_(), 11_(), 12_(), 13_()];
        let t1: [(); 10] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_()];
        let mut c0: () = {
            let mut __tmp_6: test_allocator_int_ = test_allocator_int_::new_1({ 10_() });
            std_forward_list_int__test_allocator_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                { &mut __tmp_6 },
            )
        };
        let mut c1: () = {
            let mut __tmp_7: test_allocator_int_ = test_allocator_int_::new_1({ 11_() });
            std_forward_list_int__test_allocator_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                { &mut __tmp_7 },
            )
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let t0: [(); 10] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_()];
        let t1: [(); 4] = [10_(), 11_(), 12_(), 13_()];
        let mut c0: () = {
            let mut __tmp_8: other_allocator_int_ = other_allocator_int_::new_1({ 10_() });
            std_forward_list_int__other_allocator_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                { &mut __tmp_8 },
            )
        };
        let mut c1: () = {
            let mut __tmp_9: other_allocator_int_ = other_allocator_int_::new_1({ 10_() });
            std_forward_list_int__other_allocator_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                { &mut __tmp_9 },
            )
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let t0: [(); 10] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_()];
        let t1: [(); 4] = [10_(), 11_(), 12_(), 13_()];
        let mut c0: () = {
            let mut __tmp_10: other_allocator_int_ = other_allocator_int_::new_1({ 10_() });
            std_forward_list_int__other_allocator_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                { &mut __tmp_10 },
            )
        };
        let mut c1: () = {
            let mut __tmp_11: other_allocator_int_ = other_allocator_int_::new_1({ 11_() });
            std_forward_list_int__other_allocator_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                { &mut __tmp_11 },
            )
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let t0: [(); 4] = [10_(), 11_(), 12_(), 13_()];
        let t1: [(); 10] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_()];
        let mut c0: () = {
            let mut __tmp_12: other_allocator_int_ = other_allocator_int_::new_1({ 10_() });
            std_forward_list_int__other_allocator_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                { &mut __tmp_12 },
            )
        };
        let mut c1: () = {
            let mut __tmp_13: other_allocator_int_ = other_allocator_int_::new_1({ 10_() });
            std_forward_list_int__other_allocator_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                { &mut __tmp_13 },
            )
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let t0: [(); 4] = [10_(), 11_(), 12_(), 13_()];
        let t1: [(); 10] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_()];
        let mut c0: () = {
            let mut __tmp_14: other_allocator_int_ = other_allocator_int_::new_1({ 10_() });
            std_forward_list_int__other_allocator_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                { &mut __tmp_14 },
            )
        };
        let mut c1: () = {
            let mut __tmp_15: other_allocator_int_ = other_allocator_int_::new_1({ 11_() });
            std_forward_list_int__other_allocator_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                { &mut __tmp_15 },
            )
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let t0: [(); 10] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_()];
        let t1: [(); 4] = [10_(), 11_(), 12_(), 13_()];
        let mut c0: () = {
            let mut __tmp_16: min_allocator_int_ = <min_allocator_int_>::default();
            std_forward_list_int__min_allocator_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                { &mut __tmp_16 },
            )
        };
        let mut c1: () = {
            let mut __tmp_17: min_allocator_int_ = <min_allocator_int_>::default();
            std_forward_list_int__min_allocator_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                { &mut __tmp_17 },
            )
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let t0: [(); 4] = [10_(), 11_(), 12_(), 13_()];
        let t1: [(); 10] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_()];
        let mut c0: () = {
            let mut __tmp_18: min_allocator_int_ = <min_allocator_int_>::default();
            std_forward_list_int__min_allocator_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                { &mut __tmp_18 },
            )
        };
        let mut c1: () = {
            let mut __tmp_19: min_allocator_int_ = <min_allocator_int_>::default();
            std_forward_list_int__min_allocator_int__::new_1(
                { unimplemented!() },
                { unimplemented!() },
                { &mut __tmp_19 },
            )
        };
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
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_0);
    std::cell::LazyCell::force(&*&raw const moved_value_1);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_6);
    std::cell::LazyCell::force(&*&raw const alloc_count_7);
    std::cell::LazyCell::force(&*&raw const dealloc_count_8);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_9);
}
