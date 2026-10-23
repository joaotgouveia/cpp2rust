extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_multimap_int__const_long__std_less_int___std_allocator_std_pair_const_int__const_long_____std_multimap_int__const_long__std_less_int___std_allocator_std_pair_const_int__const_long____
{}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_multimap_int__const_long__std_greater_int___std_allocator_std_pair_const_int__const_long_____std_multimap_int__const_long__std_greater_int___std_allocator_std_pair_const_int__const_long____
{}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_multimap_int__const_long__std_greater_int___test_allocator_std_pair_const_int__const_long_____std_multimap_int__const_long__std_greater_int___test_allocator_std_pair_const_int__const_long____
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_multimap_int__long__std_less_int___std_allocator_std_pair_const_int__long_____std_multimap_int__long__std_less_int___std_allocator_std_pair_const_int__long____
{}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_multimap_int__long__std_greater_int___std_allocator_std_pair_const_int__long_____std_multimap_int__long__std_greater_int___std_allocator_std_pair_const_int__long____
{}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_multimap_int__long__std_greater_int___test_allocator_std_pair_const_int__long_____std_multimap_int__long__std_greater_int___test_allocator_std_pair_const_int__long____
{}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_multimap_int__long__std_less_int___test_allocator_std_pair_const_int__long_____std_multimap_int__long__std_less_int___test_allocator_std_pair_const_int__long____
{}
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
pub static mut destructed_value_7: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_8: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_9(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_10(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_9(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_7));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_7));
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
            operator_eq_9(
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
impl test_allocator_std_pair_const_int__const_long__ {
    pub unsafe fn new_1(
        a: *const test_allocator_std___tree_node_std___value_type_int__const_long___void_ptr__,
    ) -> Self {
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std_pair_const_int__const_long___rebind_std_pair_const_int__const_long__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std_pair_const_int__const_long___rebind_std___tree_node_std___value_type_int__const_long___void_ptr__
{}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_std_pair_const_int__const_long__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std_pair_const_int__const_long__ {
    pub unsafe fn new_2(mut data: (), mut id: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_std_pair_const_int__const_long__) -> Self {
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_7));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_7));
    }
    pub unsafe fn get_id(&self) -> () {
        return self.id_;
    }
}
impl Clone for test_allocator_std_pair_const_int__const_long__ {
    fn clone(&self) -> Self {
        unsafe {
            test_allocator_std_pair_const_int__const_long__::copy_from(
                self as *const test_allocator_std_pair_const_int__const_long__,
            )
        }
    }
}
impl Default for test_allocator_std_pair_const_int__const_long__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std_pair_const_int__const_long__ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
impl test_allocator_std___tree_node_std___value_type_int__const_long___void_ptr__ {
    pub unsafe fn new_1(a: *const test_allocator_std_pair_const_int__const_long__) -> Self {
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std___tree_node_std___value_type_int__const_long___void_ptr___rebind_std___tree_node_base_void_ptr__
{}
#[repr(C)]
#[derive(FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_std___tree_node_std___value_type_int__const_long___void_ptr__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std___tree_node_std___value_type_int__const_long___void_ptr__ {
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_7));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_7));
    }
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
        mut _a1: Option<()>,
    ) -> *mut std___tree_node_std___value_type_int__const_long___void_ptr_ {
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
    pub unsafe fn deallocate(
        &mut self,
        mut p: *mut std___tree_node_std___value_type_int__const_long___void_ptr_,
        mut s: (),
    ) {
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_sub(s);
        }
        unimplemented!();
    }
}
impl Default for test_allocator_std___tree_node_std___value_type_int__const_long___void_ptr__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std___tree_node_std___value_type_int__const_long___void_ptr__ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct test_allocator_std___tree_node_base_void_ptr__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std___tree_node_base_void_ptr__ {}
impl Default for test_allocator_std___tree_node_base_void_ptr__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std___tree_node_base_void_ptr__ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
impl test_allocator_std_pair_const_int__long__ {
    pub unsafe fn new_1(
        a: *const test_allocator_std___tree_node_std___value_type_int__long___void_ptr__,
    ) -> Self {
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std_pair_const_int__long___rebind_std_pair_const_int__long__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std_pair_const_int__long___rebind_std___tree_node_std___value_type_int__long___void_ptr__
{}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_std_pair_const_int__long__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std_pair_const_int__long__ {
    pub unsafe fn new_2(mut data: (), mut id: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_std_pair_const_int__long__) -> Self {
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_7));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_7));
    }
    pub unsafe fn get_id(&self) -> () {
        return self.id_;
    }
}
impl Clone for test_allocator_std_pair_const_int__long__ {
    fn clone(&self) -> Self {
        unsafe {
            test_allocator_std_pair_const_int__long__::copy_from(
                self as *const test_allocator_std_pair_const_int__long__,
            )
        }
    }
}
impl Default for test_allocator_std_pair_const_int__long__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std_pair_const_int__long__ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
impl test_allocator_std___tree_node_std___value_type_int__long___void_ptr__ {
    pub unsafe fn new_1(a: *const test_allocator_std_pair_const_int__long__) -> Self {
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std___tree_node_std___value_type_int__long___void_ptr___rebind_std___tree_node_base_void_ptr__
{}
#[repr(C)]
#[derive(FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_std___tree_node_std___value_type_int__long___void_ptr__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std___tree_node_std___value_type_int__long___void_ptr__ {
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_7));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_7));
    }
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
        mut _a1: Option<()>,
    ) -> *mut std___tree_node_std___value_type_int__long___void_ptr_ {
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
    pub unsafe fn deallocate(
        &mut self,
        mut p: *mut std___tree_node_std___value_type_int__long___void_ptr_,
        mut s: (),
    ) {
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_sub(s);
        }
        unimplemented!();
    }
}
impl Default for test_allocator_std___tree_node_std___value_type_int__long___void_ptr__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std___tree_node_std___value_type_int__long___void_ptr__ {
                data_: unimplemented!(),
                id_: unimplemented!(),
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
pub unsafe fn test_11() -> () {
    {
        let arr: [(); 5] = [
            {
                let mut __tmp_0: () = 1_();
                let mut __tmp_1: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_0 }, { &mut __tmp_1 })
            },
            {
                let mut __tmp_2: () = 2_();
                let mut __tmp_3: () = 2_();
                std_pair_const_int__const_long_::new({ &mut __tmp_2 }, { &mut __tmp_3 })
            },
            {
                let mut __tmp_4: () = 1_();
                let mut __tmp_5: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_4 }, { &mut __tmp_5 })
            },
            {
                let mut __tmp_6: () = 2147483647_();
                let mut __tmp_7: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_6 }, { &mut __tmp_7 })
            },
            {
                let mut __tmp_8: () = 3_();
                let mut __tmp_9: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_8 }, { &mut __tmp_9 })
            },
        ];
        let mut m: () = unimplemented!();
        const _: () = assert!(true, "");;
        let expected_m: [(); 5] = [
            {
                let mut __tmp_10: () = 1_();
                let mut __tmp_11: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_10 }, { &mut __tmp_11 })
            },
            {
                let mut __tmp_12: () = 1_();
                let mut __tmp_13: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_12 }, { &mut __tmp_13 })
            },
            {
                let mut __tmp_14: () = 2_();
                let mut __tmp_15: () = 2_();
                std_pair_const_int__const_long_::new({ &mut __tmp_14 }, { &mut __tmp_15 })
            },
            {
                let mut __tmp_16: () = 3_();
                let mut __tmp_17: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_16 }, { &mut __tmp_17 })
            },
            {
                let mut __tmp_18: () = 2147483647_();
                let mut __tmp_19: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_18 }, { &mut __tmp_19 })
            },
        ];
        unimplemented!();
    }
    {
        let arr: [(); 5] = [
            {
                let mut __tmp_20: () = 1_();
                let mut __tmp_21: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_20 }, { &mut __tmp_21 })
            },
            {
                let mut __tmp_22: () = 2_();
                let mut __tmp_23: () = 2_();
                std_pair_const_int__const_long_::new({ &mut __tmp_22 }, { &mut __tmp_23 })
            },
            {
                let mut __tmp_24: () = 1_();
                let mut __tmp_25: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_24 }, { &mut __tmp_25 })
            },
            {
                let mut __tmp_26: () = 2147483647_();
                let mut __tmp_27: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_26 }, { &mut __tmp_27 })
            },
            {
                let mut __tmp_28: () = 3_();
                let mut __tmp_29: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_28 }, { &mut __tmp_29 })
            },
        ];
        let mut m: () = unimplemented!();
        const _: () = assert!(true, "");;
        let expected_m: [(); 5] = [
            {
                let mut __tmp_30: () = 2147483647_();
                let mut __tmp_31: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_30 }, { &mut __tmp_31 })
            },
            {
                let mut __tmp_32: () = 3_();
                let mut __tmp_33: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_32 }, { &mut __tmp_33 })
            },
            {
                let mut __tmp_34: () = 2_();
                let mut __tmp_35: () = 2_();
                std_pair_const_int__const_long_::new({ &mut __tmp_34 }, { &mut __tmp_35 })
            },
            {
                let mut __tmp_36: () = 1_();
                let mut __tmp_37: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_36 }, { &mut __tmp_37 })
            },
            {
                let mut __tmp_38: () = 1_();
                let mut __tmp_39: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_38 }, { &mut __tmp_39 })
            },
        ];
        unimplemented!();
    }
    {
        let arr: [(); 5] = [
            {
                let mut __tmp_40: () = 1_();
                let mut __tmp_41: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_40 }, { &mut __tmp_41 })
            },
            {
                let mut __tmp_42: () = 2_();
                let mut __tmp_43: () = 2_();
                std_pair_const_int__const_long_::new({ &mut __tmp_42 }, { &mut __tmp_43 })
            },
            {
                let mut __tmp_44: () = 1_();
                let mut __tmp_45: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_44 }, { &mut __tmp_45 })
            },
            {
                let mut __tmp_46: () = 2147483647_();
                let mut __tmp_47: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_46 }, { &mut __tmp_47 })
            },
            {
                let mut __tmp_48: () = 3_();
                let mut __tmp_49: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_48 }, { &mut __tmp_49 })
            },
        ];
        let mut m: () = unimplemented!();
        const _: () = assert!(true, "");;
        let expected_m: [(); 5] = [
            {
                let mut __tmp_50: () = 2147483647_();
                let mut __tmp_51: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_50 }, { &mut __tmp_51 })
            },
            {
                let mut __tmp_52: () = 3_();
                let mut __tmp_53: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_52 }, { &mut __tmp_53 })
            },
            {
                let mut __tmp_54: () = 2_();
                let mut __tmp_55: () = 2_();
                std_pair_const_int__const_long_::new({ &mut __tmp_54 }, { &mut __tmp_55 })
            },
            {
                let mut __tmp_56: () = 1_();
                let mut __tmp_57: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_56 }, { &mut __tmp_57 })
            },
            {
                let mut __tmp_58: () = 1_();
                let mut __tmp_59: () = 1_();
                std_pair_const_int__const_long_::new({ &mut __tmp_58 }, { &mut __tmp_59 })
            },
        ];
        unimplemented!();
        unimplemented!();
    }
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(true, "");;
        let expected_m: [(); 5] = [
            {
                let mut __tmp_60: () = 1_();
                let mut __tmp_61: () = 1_();
                std_pair_const_int__long_::new({ &mut __tmp_60 }, { &mut __tmp_61 })
            },
            {
                let mut __tmp_62: () = 1_();
                let mut __tmp_63: () = 1_();
                std_pair_const_int__long_::new({ &mut __tmp_62 }, { &mut __tmp_63 })
            },
            {
                let mut __tmp_64: () = 2_();
                let mut __tmp_65: () = 2_();
                std_pair_const_int__long_::new({ &mut __tmp_64 }, { &mut __tmp_65 })
            },
            {
                let mut __tmp_66: () = 3_();
                let mut __tmp_67: () = 1_();
                std_pair_const_int__long_::new({ &mut __tmp_66 }, { &mut __tmp_67 })
            },
            {
                let mut __tmp_68: () = 2147483647_();
                let mut __tmp_69: () = 1_();
                std_pair_const_int__long_::new({ &mut __tmp_68 }, { &mut __tmp_69 })
            },
        ];
        unimplemented!();
    }
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(true, "");;
        let expected_m: [(); 5] = [
            {
                let mut __tmp_70: () = 2147483647_();
                let mut __tmp_71: () = 1_();
                std_pair_const_int__long_::new({ &mut __tmp_70 }, { &mut __tmp_71 })
            },
            {
                let mut __tmp_72: () = 3_();
                let mut __tmp_73: () = 1_();
                std_pair_const_int__long_::new({ &mut __tmp_72 }, { &mut __tmp_73 })
            },
            {
                let mut __tmp_74: () = 2_();
                let mut __tmp_75: () = 2_();
                std_pair_const_int__long_::new({ &mut __tmp_74 }, { &mut __tmp_75 })
            },
            {
                let mut __tmp_76: () = 1_();
                let mut __tmp_77: () = 1_();
                std_pair_const_int__long_::new({ &mut __tmp_76 }, { &mut __tmp_77 })
            },
            {
                let mut __tmp_78: () = 1_();
                let mut __tmp_79: () = 1_();
                std_pair_const_int__long_::new({ &mut __tmp_78 }, { &mut __tmp_79 })
            },
        ];
        unimplemented!();
    }
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(true, "");;
        let expected_m: [(); 5] = [
            {
                let mut __tmp_80: () = 2147483647_();
                let mut __tmp_81: () = 1_();
                std_pair_const_int__long_::new({ &mut __tmp_80 }, { &mut __tmp_81 })
            },
            {
                let mut __tmp_82: () = 3_();
                let mut __tmp_83: () = 1_();
                std_pair_const_int__long_::new({ &mut __tmp_82 }, { &mut __tmp_83 })
            },
            {
                let mut __tmp_84: () = 2_();
                let mut __tmp_85: () = 2_();
                std_pair_const_int__long_::new({ &mut __tmp_84 }, { &mut __tmp_85 })
            },
            {
                let mut __tmp_86: () = 1_();
                let mut __tmp_87: () = 1_();
                std_pair_const_int__long_::new({ &mut __tmp_86 }, { &mut __tmp_87 })
            },
            {
                let mut __tmp_88: () = 1_();
                let mut __tmp_89: () = 1_();
                std_pair_const_int__long_::new({ &mut __tmp_88 }, { &mut __tmp_89 })
            },
        ];
        unimplemented!();
        unimplemented!();
    }
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(true, "");;
        let expected_m: [(); 5] = [
            {
                let mut __tmp_90: () = 1_();
                let mut __tmp_91: () = 1_();
                std_pair_const_int__long_::new({ &mut __tmp_90 }, { &mut __tmp_91 })
            },
            {
                let mut __tmp_92: () = 1_();
                let mut __tmp_93: () = 1_();
                std_pair_const_int__long_::new({ &mut __tmp_92 }, { &mut __tmp_93 })
            },
            {
                let mut __tmp_94: () = 2_();
                let mut __tmp_95: () = 2_();
                std_pair_const_int__long_::new({ &mut __tmp_94 }, { &mut __tmp_95 })
            },
            {
                let mut __tmp_96: () = 3_();
                let mut __tmp_97: () = 1_();
                std_pair_const_int__long_::new({ &mut __tmp_96 }, { &mut __tmp_97 })
            },
            {
                let mut __tmp_98: () = 2147483647_();
                let mut __tmp_99: () = 1_();
                std_pair_const_int__long_::new({ &mut __tmp_98 }, { &mut __tmp_99 })
            },
        ];
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
    (unsafe { test_11() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_7);
    std::cell::LazyCell::force(&*&raw const moved_value_8);
}
