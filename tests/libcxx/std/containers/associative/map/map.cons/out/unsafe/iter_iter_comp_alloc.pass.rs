extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(a: *const test_less_int_, b: *const test_less_int_) -> () {
    return (((*a).data_) == ((*b).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_less_int_ {
    pub data_: (),
}
impl test_less_int_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { data_: 0_() };
        this
    }
    pub unsafe fn new_2(mut data: ()) -> Self {
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn operator_call(&self, a: *const (), b: *const ()) -> () {
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
impl Default for test_less_int_ {
    fn default() -> Self {
        unsafe { test_less_int_::new_1() }
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
pub unsafe fn operator_eq_5(
    x: *const test_allocator_std_pair_const_int__double__,
    y: *const test_allocator_std_pair_const_int__double__,
) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std_pair_const_int__double___rebind_std_pair_const_int__double__ {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_std_pair_const_int__double__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std_pair_const_int__double__ {
    pub unsafe fn new_1(mut data: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_std_pair_const_int__double__) -> Self {
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
}
impl std::cmp::PartialEq for test_allocator_std_pair_const_int__double__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_5(
                self as *const test_allocator_std_pair_const_int__double__,
                other as *const test_allocator_std_pair_const_int__double__,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_std_pair_const_int__double__ {}
impl Clone for test_allocator_std_pair_const_int__double__ {
    fn clone(&self) -> Self {
        unsafe {
            test_allocator_std_pair_const_int__double__::copy_from(
                self as *const test_allocator_std_pair_const_int__double__,
            )
        }
    }
}
impl Default for test_allocator_std_pair_const_int__double__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std_pair_const_int__double__ {
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
pub struct min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_10(
    mut _a0: min_allocator_std_pair_const_int__double__,
    mut _a1: min_allocator_std_pair_const_int__double__,
) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__double__ {}
impl std::cmp::PartialEq for min_allocator_std_pair_const_int__double__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_10(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_std_pair_const_int__double__ {}
pub unsafe fn operator_eq_11(
    mut _a0: explicit_allocator_std_pair_const_int__double__,
    mut _a1: explicit_allocator_std_pair_const_int__double__,
) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct explicit_allocator_std_pair_const_int__double__ {}
impl explicit_allocator_std_pair_const_int__double__ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
}
impl std::cmp::PartialEq for explicit_allocator_std_pair_const_int__double__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_11(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for explicit_allocator_std_pair_const_int__double__ {}
impl Default for explicit_allocator_std_pair_const_int__double__ {
    fn default() -> Self {
        unsafe { explicit_allocator_std_pair_const_int__double__::new() }
    }
}
pub unsafe fn test_12() -> () {
    {
        let mut ar: [(); 9] = [
            {
                let mut __tmp_0: () = 1_();
                let mut __tmp_1: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_0 }, { &mut __tmp_1 })
            },
            {
                let mut __tmp_2: () = 1_();
                let mut __tmp_3: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_2 }, { &mut __tmp_3 })
            },
            {
                let mut __tmp_4: () = 1_();
                let mut __tmp_5: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_4 }, { &mut __tmp_5 })
            },
            {
                let mut __tmp_6: () = 2_();
                let mut __tmp_7: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_6 }, { &mut __tmp_7 })
            },
            {
                let mut __tmp_8: () = 2_();
                let mut __tmp_9: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_8 }, { &mut __tmp_9 })
            },
            {
                let mut __tmp_10: () = 2_();
                let mut __tmp_11: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_10 }, { &mut __tmp_11 })
            },
            {
                let mut __tmp_12: () = 3_();
                let mut __tmp_13: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_12 }, { &mut __tmp_13 })
            },
            {
                let mut __tmp_14: () = 3_();
                let mut __tmp_15: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_14 }, { &mut __tmp_15 })
            },
            {
                let mut __tmp_16: () = 3_();
                let mut __tmp_17: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_16 }, { &mut __tmp_17 })
            },
        ];
        let mut m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ar: [(); 9] = [
            {
                let mut __tmp_18: () = 1_();
                let mut __tmp_19: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_18 }, { &mut __tmp_19 })
            },
            {
                let mut __tmp_20: () = 1_();
                let mut __tmp_21: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_20 }, { &mut __tmp_21 })
            },
            {
                let mut __tmp_22: () = 1_();
                let mut __tmp_23: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_22 }, { &mut __tmp_23 })
            },
            {
                let mut __tmp_24: () = 2_();
                let mut __tmp_25: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_24 }, { &mut __tmp_25 })
            },
            {
                let mut __tmp_26: () = 2_();
                let mut __tmp_27: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_26 }, { &mut __tmp_27 })
            },
            {
                let mut __tmp_28: () = 2_();
                let mut __tmp_29: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_28 }, { &mut __tmp_29 })
            },
            {
                let mut __tmp_30: () = 3_();
                let mut __tmp_31: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_30 }, { &mut __tmp_31 })
            },
            {
                let mut __tmp_32: () = 3_();
                let mut __tmp_33: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_32 }, { &mut __tmp_33 })
            },
            {
                let mut __tmp_34: () = 3_();
                let mut __tmp_35: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_34 }, { &mut __tmp_35 })
            },
        ];
        let mut m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ar: [(); 9] = [
            {
                let mut __tmp_36: () = 1_();
                let mut __tmp_37: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_36 }, { &mut __tmp_37 })
            },
            {
                let mut __tmp_38: () = 1_();
                let mut __tmp_39: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_38 }, { &mut __tmp_39 })
            },
            {
                let mut __tmp_40: () = 1_();
                let mut __tmp_41: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_40 }, { &mut __tmp_41 })
            },
            {
                let mut __tmp_42: () = 2_();
                let mut __tmp_43: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_42 }, { &mut __tmp_43 })
            },
            {
                let mut __tmp_44: () = 2_();
                let mut __tmp_45: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_44 }, { &mut __tmp_45 })
            },
            {
                let mut __tmp_46: () = 2_();
                let mut __tmp_47: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_46 }, { &mut __tmp_47 })
            },
            {
                let mut __tmp_48: () = 3_();
                let mut __tmp_49: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_48 }, { &mut __tmp_49 })
            },
            {
                let mut __tmp_50: () = 3_();
                let mut __tmp_51: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_50 }, { &mut __tmp_51 })
            },
            {
                let mut __tmp_52: () = 3_();
                let mut __tmp_53: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_52 }, { &mut __tmp_53 })
            },
        ];
        {
            let mut a: min_allocator_std_pair_const_int__double__ =
                <min_allocator_std_pair_const_int__double__>::default();
            let mut m: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut a: explicit_allocator_std_pair_const_int__double__ =
                explicit_allocator_std_pair_const_int__double__::new();
            let mut m: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
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
    (unsafe { test_12() });
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
