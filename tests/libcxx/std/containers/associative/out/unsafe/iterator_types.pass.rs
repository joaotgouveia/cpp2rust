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
pub struct min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { ptr_ : *mut std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0___ , }
impl min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___,
}
impl min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { ptr_ : *mut std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ , }
impl min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std___tree_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut std___tree_node_int__min_pointer_void__std_integral_constant_unsigned_long__0___,
}
impl min_pointer_std___tree_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__int__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0____
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0____
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std___tree_node_int__min_pointer_void__std_integral_constant_unsigned_long__0____
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
pub static mut destructed_value_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_5: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_6(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_7(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_6(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4));
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
                data_: 0_(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std_pair_const_int__int___rebind_std___tree_node_std___value_type_int__int___void_ptr__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std_pair_const_int__int___rebind_std_pair_const_int__int__ {}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct test_allocator_std_pair_const_int__int__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std_pair_const_int__int__ {}
impl Default for test_allocator_std_pair_const_int__int__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std_pair_const_int__int__ {
                data_: unimplemented!(),
                id_: unimplemented!(),
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
pub struct test_allocator_int__rebind_std___tree_node_int__void_ptr__ {}
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std___tree_node_std___value_type_int__int___void_ptr___rebind_std___tree_node_base_void_ptr__
{}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct test_allocator_std___tree_node_std___value_type_int__int___void_ptr__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std___tree_node_std___value_type_int__int___void_ptr__ {}
impl Default for test_allocator_std___tree_node_std___value_type_int__int___void_ptr__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std___tree_node_std___value_type_int__int___void_ptr__ {
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std___tree_node_int__void_ptr___rebind_std___tree_node_base_void_ptr__ {
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct test_allocator_std___tree_node_int__void_ptr__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std___tree_node_int__void_ptr__ {}
impl Default for test_allocator_std___tree_node_int__void_ptr__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std___tree_node_int__void_ptr__ {
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
pub unsafe fn testMap_8() {
    {
        const _: () = assert!(
            true,
            "(std::is_same<typename It::value_type, ValueTp>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::reference, ValueTp&>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<typename It::pointer, PtrT>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::difference_type, Diff>::value)"
        );;
    }
    {
        const _: () = assert!(
            true,
            "(std::is_same<typename It::value_type, ValueTp>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::reference, ValueTp const&>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<typename It::pointer, CPtrT>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::difference_type, Diff>::value)"
        );;
    }
}
pub unsafe fn testMap_9() {
    {
        const _: () = assert!(
            true,
            "(std::is_same<typename It::value_type, ValueTp>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::reference, ValueTp&>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<typename It::pointer, PtrT>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::difference_type, Diff>::value)"
        );;
    }
    {
        const _: () = assert!(
            true,
            "(std::is_same<typename It::value_type, ValueTp>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::reference, ValueTp const&>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<typename It::pointer, CPtrT>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::difference_type, Diff>::value)"
        );;
    }
}
pub unsafe fn testMap_10() {
    {
        const _: () = assert!(
            true,
            "(std::is_same<typename It::value_type, ValueTp>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::reference, ValueTp&>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<typename It::pointer, PtrT>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::difference_type, Diff>::value)"
        );;
    }
    {
        const _: () = assert!(
            true,
            "(std::is_same<typename It::value_type, ValueTp>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::reference, ValueTp const&>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<typename It::pointer, CPtrT>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::difference_type, Diff>::value)"
        );;
    }
}
pub unsafe fn testMap_11() {
    {
        const _: () = assert!(
            true,
            "(std::is_same<typename It::value_type, ValueTp>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::reference, ValueTp&>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<typename It::pointer, PtrT>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::difference_type, Diff>::value)"
        );;
    }
    {
        const _: () = assert!(
            true,
            "(std::is_same<typename It::value_type, ValueTp>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::reference, ValueTp const&>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<typename It::pointer, CPtrT>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::difference_type, Diff>::value)"
        );;
    }
}
pub unsafe fn testMap_12() {
    {
        const _: () = assert!(
            true,
            "(std::is_same<typename It::value_type, ValueTp>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::reference, ValueTp&>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<typename It::pointer, PtrT>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::difference_type, Diff>::value)"
        );;
    }
    {
        const _: () = assert!(
            true,
            "(std::is_same<typename It::value_type, ValueTp>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::reference, ValueTp const&>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<typename It::pointer, CPtrT>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::difference_type, Diff>::value)"
        );;
    }
}
pub unsafe fn testMap_13() {
    {
        const _: () = assert!(
            true,
            "(std::is_same<typename It::value_type, ValueTp>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::reference, ValueTp&>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<typename It::pointer, PtrT>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::difference_type, Diff>::value)"
        );;
    }
    {
        const _: () = assert!(
            true,
            "(std::is_same<typename It::value_type, ValueTp>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::reference, ValueTp const&>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<typename It::pointer, CPtrT>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::difference_type, Diff>::value)"
        );;
    }
}
pub unsafe fn testSet_14() {
    const _: () = assert!(
        true,
        "(std::is_same<typename Set::iterator, typename Set::const_iterator>::value)"
    );;
    {
        const _: () = assert!(
            true,
            "(std::is_same<typename It::value_type, ValueTp>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::reference, ValueTp const&>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<typename It::pointer, CPtrT>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::difference_type, Diff>::value)"
        );;
    }
}
pub unsafe fn testSet_15() {
    const _: () = assert!(
        true,
        "(std::is_same<typename Set::iterator, typename Set::const_iterator>::value)"
    );;
    {
        const _: () = assert!(
            true,
            "(std::is_same<typename It::value_type, ValueTp>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::reference, ValueTp const&>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<typename It::pointer, CPtrT>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::difference_type, Diff>::value)"
        );;
    }
}
pub unsafe fn testSet_16() {
    const _: () = assert!(
        true,
        "(std::is_same<typename Set::iterator, typename Set::const_iterator>::value)"
    );;
    {
        const _: () = assert!(
            true,
            "(std::is_same<typename It::value_type, ValueTp>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::reference, ValueTp const&>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<typename It::pointer, CPtrT>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::difference_type, Diff>::value)"
        );;
    }
}
pub unsafe fn testSet_17() {
    const _: () = assert!(
        true,
        "(std::is_same<typename Set::iterator, typename Set::const_iterator>::value)"
    );;
    {
        const _: () = assert!(
            true,
            "(std::is_same<typename It::value_type, ValueTp>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::reference, ValueTp const&>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<typename It::pointer, CPtrT>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::difference_type, Diff>::value)"
        );;
    }
}
pub unsafe fn testSet_18() {
    const _: () = assert!(
        true,
        "(std::is_same<typename Set::iterator, typename Set::const_iterator>::value)"
    );;
    {
        const _: () = assert!(
            true,
            "(std::is_same<typename It::value_type, ValueTp>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::reference, ValueTp const&>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<typename It::pointer, CPtrT>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::difference_type, Diff>::value)"
        );;
    }
}
pub unsafe fn testSet_19() {
    const _: () = assert!(
        true,
        "(std::is_same<typename Set::iterator, typename Set::const_iterator>::value)"
    );;
    {
        const _: () = assert!(
            true,
            "(std::is_same<typename It::value_type, ValueTp>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::reference, ValueTp const&>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<typename It::pointer, CPtrT>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<typename It::difference_type, Diff>::value)"
        );;
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    {
        (unsafe { testMap_8() });
    }
    {
        (unsafe { testMap_9() });
    }
    {
        (unsafe { testMap_10() });
    }
    {
        (unsafe { testMap_11() });
    }
    {
        (unsafe { testMap_12() });
    }
    {
        (unsafe { testMap_13() });
    }
    {
        (unsafe { testSet_14() });
    }
    {
        (unsafe { testSet_15() });
    }
    {
        (unsafe { testSet_16() });
    }
    {
        (unsafe { testSet_17() });
    }
    {
        (unsafe { testSet_18() });
    }
    {
        (unsafe { testSet_19() });
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
    std::cell::LazyCell::force(&*&raw const destructed_value_4);
    std::cell::LazyCell::force(&*&raw const moved_value_5);
}
