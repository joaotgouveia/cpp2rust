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
pub struct Buffer_Counted__1_ {
    pub buffer: [(); 8],
}
impl Buffer_Counted__1_ {
    pub unsafe fn begin(&mut self) -> *mut Counted {
        return (self.buffer.as_mut_ptr() as *mut Counted);
    }
    pub unsafe fn end(&mut self) -> *mut Counted {
        return (unsafe { Buffer_Counted__1_::begin(self) }).offset((1_()) as isize);
    }
}
impl Default for Buffer_Counted__1_ {
    fn default() -> Self {
        unsafe {
            Buffer_Counted__1_ {
                buffer: [
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                ],
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Buffer_Counted__5_ {
    pub buffer: [(); 40],
}
impl Buffer_Counted__5_ {
    pub unsafe fn begin(&mut self) -> *mut Counted {
        return (self.buffer.as_mut_ptr() as *mut Counted);
    }
    pub unsafe fn end(&mut self) -> *mut Counted {
        return (unsafe { Buffer_Counted__5_::begin(self) }).offset((5_()) as isize);
    }
}
impl Default for Buffer_Counted__5_ {
    fn default() -> Self {
        unsafe {
            Buffer_Counted__5_ {
                buffer: [
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                ],
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Buffer_double__3_ {
    pub buffer: [(); 24],
}
impl Buffer_double__3_ {
    pub unsafe fn begin(&mut self) -> *mut () {
        return (self.buffer.as_mut_ptr() as *mut ());
    }
    pub unsafe fn end(&mut self) -> *mut () {
        return (unsafe { Buffer_double__3_::begin(self) }).offset((3_()) as isize);
    }
}
impl Default for Buffer_double__3_ {
    fn default() -> Self {
        unsafe {
            Buffer_double__3_ {
                buffer: [
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                ],
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Buffer_Counted__3_ {
    pub buffer: [(); 24],
}
impl Buffer_Counted__3_ {
    pub unsafe fn begin(&mut self) -> *mut Counted {
        return (self.buffer.as_mut_ptr() as *mut Counted);
    }
    pub unsafe fn end(&mut self) -> *mut Counted {
        return (unsafe { Buffer_Counted__3_::begin(self) }).offset((3_()) as isize);
    }
}
impl Default for Buffer_Counted__3_ {
    fn default() -> Self {
        unsafe {
            Buffer_Counted__3_ {
                buffer: [
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                ],
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Buffer_int__3_ {
    pub buffer: [(); 12],
}
impl Buffer_int__3_ {
    pub unsafe fn begin(&mut self) -> *mut () {
        return (self.buffer.as_mut_ptr() as *mut ());
    }
    pub unsafe fn end(&mut self) -> *mut () {
        return (unsafe { Buffer_int__3_::begin(self) }).offset((3_()) as isize);
    }
}
impl Default for Buffer_int__3_ {
    fn default() -> Self {
        unsafe {
            Buffer_int__3_ {
                buffer: [
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                ],
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Buffer_MoveOnly__4_ {
    pub buffer: [(); 16],
}
impl Buffer_MoveOnly__4_ {
    pub unsafe fn begin(&mut self) -> *mut MoveOnly {
        return (self.buffer.as_mut_ptr() as *mut MoveOnly);
    }
    pub unsafe fn end(&mut self) -> *mut MoveOnly {
        return (unsafe { Buffer_MoveOnly__4_::begin(self) }).offset((4_()) as isize);
    }
    pub unsafe fn size(&self) -> () {
        return 4_();
    }
}
impl Default for Buffer_MoveOnly__4_ {
    fn default() -> Self {
        unsafe {
            Buffer_MoveOnly__4_ {
                buffer: [
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                ],
            }
        }
    }
}
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_in_out_result_Counted_ptr__Counted_ptr___std_ranges_in_out_result_Counted_ptr__Counted_ptr__
{}
pub unsafe fn operator_eq_6(l: *const Counted, r: *const Counted) -> () {
    return (((*l).value) == ((*r).value));
}
pub unsafe fn operator_ne_7(l: *const Counted, r: *const Counted) -> () {
    return !(unsafe {
        let _l: *const Counted = l;
        let _r: *const Counted = r;
        operator_eq_6(_l, _r)
    });
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct Counted {
    pub value: (),
    pub moved_from: (),
}
impl Counted {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            value: unimplemented!(),
            moved_from: false,
        };
        (unsafe { Counted::check_throw(&mut this) });
        (unsafe { Counted::increase_counters(&mut this) });
        this
    }
    pub unsafe fn new_2(mut v: ()) -> Self {
        let mut this = Self {
            value: v,
            moved_from: false,
        };
        (unsafe { Counted::check_throw(&mut this) });
        (unsafe { Counted::increase_counters(&mut this) });
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut current_objects_1)).prefix_dec();
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut current_objects_1)) = {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut total_objects_2)) = 0_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut total_objects_2))
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_copies_3)) = {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut total_moves_4)) = 0_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut total_moves_4))
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on_5)) = -1_();
    }
    pub unsafe fn copy_from(rhs: *const Counted) -> Self {
        let mut this = Self {
            value: (*rhs).value,
            moved_from: false,
        };
        (unsafe { Counted::check_throw(&mut this) });
        (unsafe { Counted::increase_counters(&mut this) });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_copies_3)).prefix_inc();
        this
    }
    pub unsafe fn move_from(rhs: *mut Counted) -> Self {
        let mut this = Self {
            value: (*rhs).value,
            moved_from: false,
        };
        (unsafe { Counted::check_throw(&mut this) });
        (unsafe { Counted::increase_counters(&mut this) });
        (*rhs).moved_from = true;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_moves_4)).prefix_inc();
        this
    }
    unsafe fn check_throw(&mut self) {
        if ((*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on_5))
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut total_objects_2)))
        {
            unimplemented!();
        }
    }
    unsafe fn increase_counters(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut current_objects_1)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_objects_2)).prefix_inc();
    }
}
impl std::cmp::PartialEq for Counted {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_6(self as *const Counted, other as *const Counted) }
    }
}
impl std::cmp::Eq for Counted {}
impl Clone for Counted {
    fn clone(&self) -> Self {
        unsafe { Counted::copy_from(self as *const Counted) }
    }
}
impl Default for Counted {
    fn default() -> Self {
        unsafe { Counted::new_1() }
    }
}
pub static mut current_objects_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut total_objects_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut total_copies_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut total_moves_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut throw_on_5: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { -1_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct overload_compare_iterator_int_ptr_ {
    it_: *mut (),
}
impl overload_compare_iterator_int_ptr_ {
    pub unsafe fn new_1(mut it: *mut ()) -> Self {
        let mut this = Self { it_: it };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_9(&mut self) -> *mut overload_compare_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut overload_compare_iterator_int_ptr_));
    }
    pub unsafe fn operator_eq(&self, other: *const overload_compare_iterator_int_ptr_) -> () {
        return ((self.it_) == ((*other).it_));
    }
    pub unsafe fn operator_ne(&self, other: *const overload_compare_iterator_int_ptr_) -> () {
        return !(unsafe { overload_compare_iterator_int_ptr_::operator_eq(self, other) });
    }
}
impl std::cmp::PartialEq for overload_compare_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            overload_compare_iterator_int_ptr_::operator_eq(
                self,
                other as *const overload_compare_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for overload_compare_iterator_int_ptr_ {}
pub unsafe fn operator_eq_8(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_9(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_10(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_11(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_12(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_13(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_14(_a0: *const MoveOnly, _a1: *const MoveOnly) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct MoveOnly {
    data_: (),
}
impl MoveOnly {
    pub unsafe fn new(mut data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| unsafe { 1_() });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(x: *mut MoveOnly) -> Self {
        let mut this = Self { data_: (*x).data_ };
        (*x).data_ = 0_();
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut MoveOnly) -> *mut MoveOnly {
        self.data_ = (*x).data_;
        (*x).data_ = 0_();
        return &mut (*(self as *mut MoveOnly));
    }
    pub unsafe fn get(&self) -> () {
        return self.data_;
    }
    pub unsafe fn operator_add(&self, x: *const MoveOnly) -> MoveOnly {
        return MoveOnly::new({ Some(((self.data_) + ((*x).data_))) });
    }
    pub unsafe fn operator_mul(&self, x: *const MoveOnly) -> MoveOnly {
        return MoveOnly::new({ Some(((self.data_) * ((*x).data_))) });
    }
}
impl std::cmp::Ord for MoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_14(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_8(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        unsafe { MoveOnly::new(None) }
    }
}
pub unsafe fn operator_eq_15(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_16(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_17(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_18(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_19(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_20(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_21(_a0: *const TrivialMoveOnly, _a1: *const TrivialMoveOnly) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct TrivialMoveOnly {
    data_: (),
}
impl TrivialMoveOnly {
    pub unsafe fn new(mut data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| unsafe { 1_() });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(_a0: *mut TrivialMoveOnly) -> Self {
        let mut this = Self {
            data_: (*_a0).data_,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut TrivialMoveOnly) -> *mut TrivialMoveOnly {
        self.data_ = (*_a0).data_;
        return &mut (*(self as *mut TrivialMoveOnly));
    }
    pub unsafe fn get(&self) -> () {
        return self.data_;
    }
    pub unsafe fn operator_add(&self, x: *const TrivialMoveOnly) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({ Some(((self.data_) + ((*x).data_))) });
    }
    pub unsafe fn operator_mul(&self, x: *const TrivialMoveOnly) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({ Some(((self.data_) * ((*x).data_))) });
    }
}
impl std::cmp::Ord for TrivialMoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_21(
                self as *const TrivialMoveOnly,
                other as *const TrivialMoveOnly,
            )
        }
    }
}
impl std::cmp::PartialOrd for TrivialMoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for TrivialMoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_15(
                self as *const TrivialMoveOnly,
                other as *const TrivialMoveOnly,
            )
        }
    }
}
impl std::cmp::Eq for TrivialMoveOnly {}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        unsafe { TrivialMoveOnly::new(None) }
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
pub static mut destructed_value_22: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_23: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_24(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_25(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_24(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_22));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_22));
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
            operator_eq_24(
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
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct MutableCopy {
    pub val: (),
    pub alloc_constructed: (),
}
impl MutableCopy {
    pub unsafe fn new_1(mut _val: ()) -> Self {
        let mut this = Self {
            val: _val,
            alloc_constructed: false,
        };
        this
    }
    pub unsafe fn new_2(mut _a0: (), _a1: *const test_allocator_int_, o: *mut MutableCopy) -> Self {
        let mut this = Self {
            val: (*o).val,
            alloc_constructed: true,
        };
        this
    }
}
impl Default for MutableCopy {
    fn default() -> Self {
        unsafe {
            MutableCopy {
                val: unimplemented!(),
                alloc_constructed: false,
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConstCopy {
    pub val: (),
    pub alloc_constructed: (),
}
impl ConstCopy {
    pub unsafe fn new_1(mut _val: ()) -> Self {
        let mut this = Self {
            val: _val,
            alloc_constructed: false,
        };
        this
    }
    pub unsafe fn new_2(mut _a0: (), _a1: *const test_allocator_int_, o: *const ConstCopy) -> Self {
        let mut this = Self {
            val: (*o).val,
            alloc_constructed: true,
        };
        this
    }
}
impl Default for ConstCopy {
    fn default() -> Self {
        unsafe {
            ConstCopy {
                val: unimplemented!(),
                alloc_constructed: false,
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe)]
pub struct MutableMove {
    pub val: (),
    pub alloc_constructed: (),
}
impl MutableMove {
    pub unsafe fn new_1(mut _val: ()) -> Self {
        let mut this = Self {
            val: _val,
            alloc_constructed: false,
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut MutableMove) -> Self {
        let mut this = Self {
            val: (*_a0).val,
            alloc_constructed: (*_a0).alloc_constructed,
        };
        this
    }
    pub unsafe fn new_2(mut _a0: (), _a1: *const test_allocator_int_, o: *mut MutableMove) -> Self {
        let mut this = Self {
            val: (*o).val,
            alloc_constructed: true,
        };
        this
    }
}
impl Default for MutableMove {
    fn default() -> Self {
        unsafe {
            MutableMove {
                val: unimplemented!(),
                alloc_constructed: false,
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe)]
pub struct ConstMove {
    pub val: (),
    pub alloc_constructed: (),
}
impl ConstMove {
    pub unsafe fn new_1(mut _val: ()) -> Self {
        let mut this = Self {
            val: _val,
            alloc_constructed: false,
        };
        this
    }
    pub unsafe fn move_from(o: *const ConstMove) -> Self {
        let mut this = Self {
            val: (*o).val,
            alloc_constructed: false,
        };
        this
    }
    pub unsafe fn new_2(mut _a0: (), _a1: *const test_allocator_int_, o: *const ConstMove) -> Self {
        let mut this = Self {
            val: (*o).val,
            alloc_constructed: true,
        };
        this
    }
}
impl Default for ConstMove {
    fn default() -> Self {
        unsafe {
            ConstMove {
                val: unimplemented!(),
                alloc_constructed: false,
            }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct TracedCopyMove {
    pub nonConstCopy: (),
    pub constCopy: (),
    pub nonConstMove: (),
    pub constMove: (),
    pub alloc_constructed: (),
}
impl TracedCopyMove {
    pub unsafe fn new_1(other: *const TracedCopyMove) -> Self {
        let mut this = Self {
            nonConstCopy: (*other).nonConstCopy,
            constCopy: (((*other).constCopy) + (1_())),
            nonConstMove: (*other).nonConstMove,
            constMove: (*other).constMove,
            alloc_constructed: false,
        };
        this
    }
    pub unsafe fn new_2(other: *mut TracedCopyMove) -> Self {
        let mut this = Self {
            nonConstCopy: (((*other).nonConstCopy) + (1_())),
            constCopy: (*other).constCopy,
            nonConstMove: (*other).nonConstMove,
            constMove: (*other).constMove,
            alloc_constructed: false,
        };
        this
    }
    pub unsafe fn new_3(other: *mut TracedCopyMove) -> Self {
        let mut this = Self {
            nonConstCopy: (*other).nonConstCopy,
            constCopy: (*other).constCopy,
            nonConstMove: (((*other).nonConstMove) + (1_())),
            constMove: (*other).constMove,
            alloc_constructed: false,
        };
        this
    }
    pub unsafe fn new_4(other: *const TracedCopyMove) -> Self {
        let mut this = Self {
            nonConstCopy: (*other).nonConstCopy,
            constCopy: (*other).constCopy,
            nonConstMove: (*other).nonConstMove,
            constMove: (((*other).constMove) + (1_())),
            alloc_constructed: false,
        };
        this
    }
}
impl Clone for TracedCopyMove {
    fn clone(&self) -> Self {
        unsafe { TracedCopyMove::new_1(self as *const TracedCopyMove) }
    }
}
impl Default for TracedCopyMove {
    fn default() -> Self {
        unsafe {
            TracedCopyMove {
                nonConstCopy: 0_(),
                constCopy: 0_(),
                nonConstMove: 0_(),
                constMove: 0_(),
                alloc_constructed: false,
            }
        }
    }
}
pub unsafe fn nonConstCopyCtrCalled_26(obj: *const TracedCopyMove) -> () {
    return (((((*obj).nonConstCopy) == (1_())) && (((*obj).constCopy) == (0_())))
        && (((*obj).constMove) == (0_())))
        && (((*obj).nonConstMove) == (0_()));
}
pub unsafe fn constMoveCtrCalled_27(obj: *const TracedCopyMove) -> () {
    return (((((*obj).nonConstMove) == (0_())) && (((*obj).constMove) == (1_())))
        && (((*obj).constCopy) == (0_())))
        && (((*obj).nonConstCopy) == (0_()));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoConstructorFromInt {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CvtFromTupleRef {}
impl CvtFromTupleRef {
    pub unsafe fn new_1(other: *mut ()) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ExplicitCtrFromTupleRef {}
impl ExplicitCtrFromTupleRef {
    pub unsafe fn new_1(other: *mut ()) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CvtFromConstTupleRefRef {}
impl CvtFromConstTupleRefRef {
    pub unsafe fn new_1(other: *const ()) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ExplicitCtrFromConstTupleRefRef {}
impl ExplicitCtrFromConstTupleRefRef {
    pub unsafe fn new_1(other: *mut ()) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct CopyAssign {
    pub val: (),
}
impl CopyAssign {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, CopyAssignUnsafe, Default)]
pub struct ConstCopyAssign {
    pub val: (),
}
impl ConstCopyAssign {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
    pub unsafe fn copy_assign(&self, other: *const ConstCopyAssign) -> *const ConstCopyAssign {
        (*(&raw const self.val).cast_mut()) = (*other).val;
        return &(*(self as *const ConstCopyAssign));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveAssignUnsafe, Default)]
pub struct MoveAssign {
    pub val: (),
}
impl MoveAssign {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut MoveAssign) -> *mut MoveAssign {
        self.val = (*_a0).val;
        return &mut (*(self as *mut MoveAssign));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveAssignUnsafe, Default)]
pub struct ConstMoveAssign {
    pub val: (),
}
impl ConstMoveAssign {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
    pub unsafe fn move_assign(&self, other: *mut ConstMoveAssign) -> *const ConstMoveAssign {
        (*(&raw const self.val).cast_mut()) = (*other).val;
        return &(*(self as *const ConstMoveAssign));
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct TracedAssignment {
    pub copyAssign: (),
    pub constCopyAssign: (),
    pub moveAssign: (),
    pub constMoveAssign: (),
}
impl TracedAssignment {
    pub unsafe fn operator_assign_2(
        &mut self,
        _a0: *const TracedAssignment,
    ) -> *mut TracedAssignment {
        self.copyAssign.postfix_inc();
        return &mut (*(self as *mut TracedAssignment));
    }
    pub unsafe fn operator_assign_3(
        &self,
        _a0: *const TracedAssignment,
    ) -> *const TracedAssignment {
        (*(&raw const self.constCopyAssign).cast_mut()).postfix_inc();
        return &(*(self as *const TracedAssignment));
    }
    pub unsafe fn operator_assign_4(
        &mut self,
        _a0: *mut TracedAssignment,
    ) -> *mut TracedAssignment {
        self.moveAssign.postfix_inc();
        return &mut (*(self as *mut TracedAssignment));
    }
    pub unsafe fn operator_assign_5(&self, _a0: *mut TracedAssignment) -> *const TracedAssignment {
        (*(&raw const self.constMoveAssign).cast_mut()).postfix_inc();
        return &(*(self as *const TracedAssignment));
    }
}
impl Default for TracedAssignment {
    fn default() -> Self {
        unsafe {
            TracedAssignment {
                copyAssign: 0_(),
                constCopyAssign: 0_(),
                moveAssign: 0_(),
                constMoveAssign: 0_(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct support_double_move_tracker {
    moved_from_: (),
}
impl support_double_move_tracker {
    pub unsafe fn new() -> Self {
        let mut this = Self { moved_from_: false };
        this
    }
    pub unsafe fn move_from(other: *mut support_double_move_tracker) -> Self {
        let mut this = Self { moved_from_: false };
        unimplemented!();
        (*other).moved_from_ = true;
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        other: *mut support_double_move_tracker,
    ) -> *mut support_double_move_tracker {
        unimplemented!();
        (*other).moved_from_ = true;
        self.moved_from_ = false;
        return &mut (*(self as *mut support_double_move_tracker));
    }
}
impl Default for support_double_move_tracker {
    fn default() -> Self {
        unsafe { support_double_move_tracker::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp17_output_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_eq_28(
    x: *const forward_iterator_Counted_ptr_,
    y: *const forward_iterator_Counted_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn base_29(i: *const forward_iterator_Counted_ptr_) -> *mut Counted {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct forward_iterator_Counted_ptr_ {
    it_: *mut Counted,
    tracker_: support_double_move_tracker,
}
impl forward_iterator_Counted_ptr_ {
    pub unsafe fn new_1(mut it: *mut Counted) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_Counted_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_Counted_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_Counted_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for forward_iterator_Counted_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_28(
                self as *const forward_iterator_Counted_ptr_,
                other as *const forward_iterator_Counted_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_Counted_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_random_access_iterator_int_ptr_ {
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.it_.offset((n) as isize));
    }
}
const _: () = assert!(
    true,
    "std::random_access_iterator<cpp20_random_access_iterator<int*>>"
);
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_input_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp20_input_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_input_iterator_int_ptr_,
    ) -> *mut cpp20_input_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_input_iterator_int_ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
}
pub unsafe fn base_30(i: *const cpp20_input_iterator_const_int_ptr_) -> *const () {
    return (*i).it_;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_input_iterator_const_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_const_int_ptr_ {
    pub unsafe fn new(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut cpp20_input_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_input_iterator_const_int_ptr_,
    ) -> *mut cpp20_input_iterator_const_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_input_iterator_const_int_ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *const () {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_5(&mut self) -> *mut cpp20_input_iterator_const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp20_input_iterator_const_int_ptr_));
    }
}
const _: () = assert!(true, "std::input_iterator<cpp20_input_iterator<int*>>");
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_output_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_output_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp20_output_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_output_iterator_int_ptr_,
    ) -> *mut cpp20_output_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_output_iterator_int_ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp20_output_iterator<int*>, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct IteratorOpCounts {
    pub increments: (),
    pub decrements: (),
    pub zero_moves: (),
    pub equal_cmps: (),
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        unsafe {
            IteratorOpCounts {
                increments: 0_(),
                decrements: 0_(),
                zero_moves: 0_(),
                equal_cmps: 0_(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_forward_iterator_Counted_ptr__ {
    base_: *mut Counted,
}
impl sentinel_wrapper_forward_iterator_Counted_ptr__ {
    pub unsafe fn new_1(it: *const forward_iterator_Counted_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_29(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const forward_iterator_Counted_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_29(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp20_input_iterator_const_int_ptr__ {
    base_: *const (),
}
impl sentinel_wrapper_cpp20_input_iterator_const_int_ptr__ {
    pub unsafe fn new_1(it: *const cpp20_input_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_30(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp20_input_iterator_const_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_30(other) }));
    }
}
pub unsafe fn operator_eq_31(
    _a0: *const forward_sized_iterator_int_ptr_,
    _a1: *const forward_sized_iterator_int_ptr_,
) -> () {
    return (((*_a0).it_) == ((*_a1).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct forward_sized_iterator_int_ptr_ {
    pub it_: *mut (),
}
impl forward_sized_iterator_int_ptr_ {}
impl std::cmp::PartialEq for forward_sized_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_31(
                self as *const forward_sized_iterator_int_ptr_,
                other as *const forward_sized_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_sized_iterator_int_ptr_ {}
impl Default for forward_sized_iterator_int_ptr_ {
    fn default() -> Self {
        unsafe {
            forward_sized_iterator_int_ptr_ {
                it_: std::ptr::null_mut(),
            }
        }
    }
}
const _: () = assert!(true, "std::forward_iterator<forward_sized_iterator<>>");
const _: () = assert!(
    true,
    "std::sized_sentinel_for<forward_sized_iterator<>, forward_sized_iterator<>>"
);
pub unsafe fn operator_add_32(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_33(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_32(_i, n)
    });
}
pub unsafe fn iter_swap_34(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_35(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut *(&raw mut (*unimplemented!())).cast::<()>();
}
pub unsafe fn operator_eq_36(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_37(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return std::cmp::Ord::cmp(&((*lhs).ptr_), &((*rhs).ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct adl_Iterator {
    ptr_: *mut (),
    iter_moves_: *mut (),
    iter_swaps_: *mut (),
}
impl adl_Iterator {
    unsafe fn new_1(mut p: *mut (), mut iter_moves: *mut (), mut iter_swaps: *mut ()) -> Self {
        let mut this = Self {
            ptr_: p,
            iter_moves_: iter_moves,
            iter_swaps_: iter_swaps,
        };
        this
    }
    pub unsafe fn TrackMoves(mut p: *mut (), iter_moves: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { (iter_moves) }, { std::ptr::null_mut() });
    }
    pub unsafe fn TrackSwaps_4(iter_swaps: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ std::ptr::null_mut() }, { std::ptr::null_mut() }, {
            (iter_swaps)
        });
    }
    pub unsafe fn TrackSwaps_5(mut p: *mut (), iter_swaps: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { std::ptr::null_mut() }, { (iter_swaps) });
    }
    pub unsafe fn iter_moves(&self) -> () {
        unimplemented!();
        return (*self.iter_moves_);
    }
    pub unsafe fn iter_swaps(&self) -> () {
        unimplemented!();
        return (*self.iter_swaps_);
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn operator_sub_10(&self, mut n: ()) -> adl_Iterator {
        return adl_Iterator::new_1(
            { self.ptr_.offset(-((n) as isize)) },
            { self.iter_moves_ },
            { self.iter_swaps_ },
        );
    }
    pub unsafe fn operator_sub_11(&self, mut rhs: adl_Iterator) -> () {
        return (((self.ptr_ as usize - rhs.ptr_ as usize) / ::std::mem::size_of::<()>()) as ()
            as ());
    }
    pub unsafe fn operator_add_assign(&mut self, mut n: ()) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_sub_assign(&mut self, mut n: ()) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_inc_14(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_inc_15(&mut self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_inc();
        return prev;
    }
    pub unsafe fn operator_dec_16(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_dec();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_dec_17(&mut self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_dec();
        return prev;
    }
}
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_37(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_36(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::Eq for adl_Iterator {}
impl Default for adl_Iterator {
    fn default() -> Self {
        unsafe {
            adl_Iterator {
                ptr_: std::ptr::null_mut(),
                iter_moves_: std::ptr::null_mut(),
                iter_swaps_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct rvalue_iterator_int_ptr_ {
    it_: *mut *mut (),
}
impl rvalue_iterator_int_ptr_ {
    pub unsafe fn operator_cmp(&self, _a0: *const rvalue_iterator_int_ptr_) -> () {
        {
            let mut cmp: () = std::cmp::Ord::cmp(
                &((*(self as *const rvalue_iterator_int_ptr_)).it_),
                &((*_a0).it_),
            );
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    pub unsafe fn operator_eq(&self, _a0: *const rvalue_iterator_int_ptr_) -> () {
        return (((*(self as *const rvalue_iterator_int_ptr_)).it_) == ((*_a0).it_));
    }
}
impl std::cmp::Ord for rvalue_iterator_int_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            rvalue_iterator_int_ptr_::operator_cmp(self, other as *const rvalue_iterator_int_ptr_)
        }
    }
}
impl std::cmp::PartialOrd for rvalue_iterator_int_ptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for rvalue_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            rvalue_iterator_int_ptr_::operator_cmp(self, other as *const rvalue_iterator_int_ptr_)
                == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for rvalue_iterator_int_ptr_ {}
const _: () = assert!(true, "std::random_access_iterator<rvalue_iterator<int*>>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub unsafe fn new(mut what: *const ()) -> Self {
        let mut this = Self {};
        this
    }
}
const _: () = assert!(
    true,
    "std::is_class_v<decltype(std::ranges::uninitialized_move)>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<decltype(std::ranges::uninitialized_move), int*, int*, long*, long*>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NotConvertibleFromInt {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<decltype(std::ranges::uninitialized_move), int*, int*, NotConvertibleFromInt*,\n                                   NotConvertibleFromInt*>"
);
pub unsafe fn test_38() -> () {
    let n: () = 3_();
    let mut alloc: () = unimplemented!();
    {
        let mut in_: [MutableMove; 3] = [
            MutableMove::new_1({ 1_() }),
            MutableMove::new_1({ 2_() }),
            MutableMove::new_1({ 3_() }),
        ];
        let mut out: *mut MutableMove = unimplemented!();
        let mut result: () = (unsafe {
            let ___ifirst: *mut MutableMove = in_.as_mut_ptr();
            let ___ilast: *mut MutableMove = in_.as_mut_ptr().offset((n) as isize);
            let ___ofirst: *mut MutableMove = out;
            let ___olast: *mut MutableMove = out.offset((n) as isize);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                ___ifirst, ___ilast, ___ofirst, ___olast,
            )
        });
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) != (n)) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
    }
    {
        let mut in_: [MutableMove; 3] = [
            MutableMove::new_1({ 1_() }),
            MutableMove::new_1({ 2_() }),
            MutableMove::new_1({ 3_() }),
        ];
        let mut out: *mut MutableMove = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                &mut in_,
                &mut out_range,
            )
        });
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) != (n)) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
    }
    {
        let m: () = 2_();
        let mut in_: [MutableMove; 3] = [
            MutableMove::new_1({ 1_() }),
            MutableMove::new_1({ 2_() }),
            MutableMove::new_1({ 3_() }),
        ];
        let mut out: *mut MutableMove = unimplemented!();
        let mut result: () = (unsafe {
            let ___ifirst: *mut MutableMove = in_.as_mut_ptr();
            let ___ilast: *mut MutableMove = in_.as_mut_ptr().offset((n) as isize);
            let ___ofirst: *mut MutableMove = out;
            let ___olast: *mut MutableMove = out.offset((m) as isize);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                ___ifirst, ___ilast, ___ofirst, ___olast,
            )
        });
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) != (m)) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
    }
    {
        let N: () = 5_();
        let mut in_: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
        let mut out: [(); 5] = [6_(), 7_(), 8_(), 9_(), 10_()];
        unimplemented!();
        (unsafe {
            let ___ifirst: *mut () = in_.as_mut_ptr();
            let ___ilast: *mut () = in_.as_mut_ptr().offset((1_()) as isize);
            let ___ofirst: *mut () = out.as_mut_ptr();
            let ___olast: *mut () = out.as_mut_ptr().offset((N) as isize);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                ___ifirst, ___ilast, ___ofirst, ___olast,
            )
        });
        unimplemented!();
        unimplemented!();
        (unsafe {
            let ___ifirst: *mut () = in_.as_mut_ptr();
            let ___ilast: *mut () = in_.as_mut_ptr().offset((N) as isize);
            let ___ofirst: *mut () = out.as_mut_ptr();
            let ___olast: *mut () = out.as_mut_ptr().offset((N) as isize);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                ___ifirst, ___ilast, ___ofirst, ___olast,
            )
        });
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
    {
        let mut in_: [Counted; 1] = [Counted::new_1()];
        let mut out: Buffer_Counted__1_ = <Buffer_Counted__1_>::default();
        (unsafe { Counted::reset() });
        {
            let mut result: () = (unsafe {
                let ___ifirst: *mut Counted = in_.as_mut_ptr();
                let ___ilast: *mut Counted = in_.as_mut_ptr();
                let ___ofirst: *mut Counted = (unsafe { Buffer_Counted__1_::begin(&mut out) });
                let ___olast: *mut Counted = (unsafe { Buffer_Counted__1_::end(&mut out) });
                (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                    ___ifirst, ___ilast, ___ofirst, ___olast,
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut view: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                    &mut view, &mut out,
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut it: forward_iterator_Counted_ptr_ =
                forward_iterator_Counted_ptr_::new_1({ in_.as_mut_ptr() });
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                let ___ifirst: forward_iterator_Counted_ptr_ = unimplemented!();
                let ___ilast: sentinel_wrapper_forward_iterator_Counted_ptr__ = unimplemented!();
                let ___ofirst: *mut Counted = (unsafe { Buffer_Counted__1_::begin(&mut out) });
                let ___olast: *mut Counted = (unsafe { Buffer_Counted__1_::end(&mut out) });
                (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                    ___ifirst, ___ilast, ___ofirst, ___olast,
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut it: forward_iterator_Counted_ptr_ =
                forward_iterator_Counted_ptr_::new_1({ in_.as_mut_ptr() });
            let mut range: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                    &mut range, &mut out,
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { Counted::reset() });
    }
    {
        let N: () = 5_();
        let mut in_: [Counted; 5] = [
            Counted::new_2({ 1_() }),
            Counted::new_2({ 2_() }),
            Counted::new_2({ 3_() }),
            Counted::new_2({ 4_() }),
            Counted::new_2({ 5_() }),
        ];
        let mut out: Buffer_Counted__5_ = <Buffer_Counted__5_>::default();
        (unsafe { Counted::reset() });
        let mut result: () = (unsafe {
            let ___ifirst: *mut Counted = in_.as_mut_ptr();
            let ___ilast: *mut Counted = in_.as_mut_ptr().offset((N) as isize);
            let ___ofirst: *mut Counted = (unsafe { Buffer_Counted__5_::begin(&mut out) });
            let ___olast: *mut Counted = (unsafe { Buffer_Counted__5_::end(&mut out) });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                ___ifirst, ___ilast, ___ofirst, ___olast,
            )
        });
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    (unsafe { Counted::reset() });
    {
        let N: () = 5_();
        let mut in_: [Counted; 5] = [
            Counted::new_2({ 1_() }),
            Counted::new_2({ 2_() }),
            Counted::new_2({ 3_() }),
            Counted::new_2({ 4_() }),
            Counted::new_2({ 5_() }),
        ];
        let mut out: Buffer_Counted__5_ = <Buffer_Counted__5_>::default();
        (unsafe { Counted::reset() });
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                &mut range, &mut out,
            )
        });
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    (unsafe { Counted::reset() });
    {
        let N: () = 3_();
        let mut in_: [Counted; 5] = [
            Counted::new_2({ 1_() }),
            Counted::new_2({ 2_() }),
            Counted::new_2({ 3_() }),
            Counted::new_2({ 4_() }),
            Counted::new_2({ 5_() }),
        ];
        let mut out: Buffer_Counted__5_ = <Buffer_Counted__5_>::default();
        (unsafe { Counted::reset() });
        let mut iter: () = std_counted_iterator_Counted_ptr_::new_1({ in_.as_mut_ptr() }, { N });
        let mut result: () = (unsafe {
            let ___ofirst: *mut Counted = (unsafe { Buffer_Counted__5_::begin(&mut out) });
            let ___olast: *mut Counted = (unsafe { Buffer_Counted__5_::end(&mut out) });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                unimplemented!(),
                unimplemented!(),
                ___ofirst,
                ___olast,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    (unsafe { Counted::reset() });
    {
        let N: () = 3_();
        let mut in_: [Counted; 5] = [
            Counted::new_2({ 1_() }),
            Counted::new_2({ 2_() }),
            Counted::new_2({ 3_() }),
            Counted::new_2({ 4_() }),
            Counted::new_2({ 5_() }),
        ];
        let mut out: Buffer_Counted__5_ = <Buffer_Counted__5_>::default();
        (unsafe { Counted::reset() });
        let mut view: () =
            (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut counted_40))(&mut in_, &N) });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                &mut view, &mut out,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    (unsafe { Counted::reset() });
    {
        let N: () = 3_();
        let mut in_: [Counted; 5] = [
            Counted::new_2({ 1_() }),
            Counted::new_2({ 2_() }),
            Counted::new_2({ 3_() }),
            Counted::new_2({ 4_() }),
            Counted::new_2({ 5_() }),
        ];
        let mut out: Buffer_Counted__5_ = <Buffer_Counted__5_>::default();
        (unsafe { Counted::reset() });
        let mut range: () = unimplemented!();
        let mut view: () =
            (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut reverse_41))(&mut range) });
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                &mut view, &mut out,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut expected: [Counted; 3] = [
            Counted::new_2({ 3_() }),
            Counted::new_2({ 2_() }),
            Counted::new_2({ 1_() }),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    (unsafe { Counted::reset() });
    {
        let N: () = 3_();
        let mut in_: [(); 3] = [1_(), 2_(), 3_()];
        let mut out: Buffer_double__3_ = <Buffer_double__3_>::default();
        (unsafe {
            let ___ifirst: *mut () = in_.as_mut_ptr();
            let ___ilast: *mut () = in_.as_mut_ptr().offset((N) as isize);
            let ___ofirst: *mut () = (unsafe { Buffer_double__3_::begin(&mut out) });
            let ___olast: *mut () = (unsafe { Buffer_double__3_::end(&mut out) });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                ___ifirst, ___ilast, ___ofirst, ___olast,
            )
        });
        unimplemented!();
    }
    {
        let N: () = 3_();
        let mut in_: [(); 3] = [1_(), 2_(), 3_()];
        let mut out: Buffer_double__3_ = <Buffer_double__3_>::default();
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                &mut in_, &mut out,
            )
        });
        unimplemented!();
    }
    {
        let M: () = 3_();
        let N: () = 5_();
        let mut in_: [Counted; 5] = [
            Counted::new_2({ 1_() }),
            Counted::new_2({ 2_() }),
            Counted::new_2({ 3_() }),
            Counted::new_2({ 4_() }),
            Counted::new_2({ 5_() }),
        ];
        let mut out: Buffer_Counted__3_ = <Buffer_Counted__3_>::default();
        (unsafe { Counted::reset() });
        let mut result: () = (unsafe {
            let ___ifirst: *mut Counted = in_.as_mut_ptr();
            let ___ilast: *mut Counted = in_.as_mut_ptr().offset((N) as isize);
            let ___ofirst: *mut Counted = (unsafe { Buffer_Counted__3_::begin(&mut out) });
            let ___olast: *mut Counted = (unsafe { Buffer_Counted__3_::end(&mut out) });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                ___ifirst, ___ilast, ___ofirst, ___olast,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let M: () = 3_();
        let N: () = 5_();
        let mut in_: [Counted; 5] = [
            Counted::new_2({ 1_() }),
            Counted::new_2({ 2_() }),
            Counted::new_2({ 3_() }),
            Counted::new_2({ 4_() }),
            Counted::new_2({ 5_() }),
        ];
        let mut out: Buffer_Counted__3_ = <Buffer_Counted__3_>::default();
        (unsafe { Counted::reset() });
        let mut range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                &mut range, &mut out,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let N: () = 3_();
        let mut in_: [(); 3] = [1_(), 2_(), 3_()];
        let mut out: Buffer_int__3_ = <Buffer_int__3_>::default();
        let mut iter_moves: () = 0_();
        let mut begin: adl_Iterator =
            (unsafe { adl_Iterator::TrackMoves(in_.as_mut_ptr(), &mut iter_moves) });
        let mut end: adl_Iterator = (unsafe {
            adl_Iterator::TrackMoves(in_.as_mut_ptr().offset((N) as isize), &mut iter_moves)
        });
        (unsafe {
            let ___ofirst: *mut () = (unsafe { Buffer_int__3_::begin(&mut out) });
            let ___olast: *mut () = (unsafe { Buffer_int__3_::end(&mut out) });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                begin, end, ___ofirst, ___olast,
            )
        });
        unimplemented!();
        iter_moves = 0_();
        let mut range: () = unimplemented!();
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                &mut range, &mut out,
            )
        });
        unimplemented!();
        iter_moves = 0_();
    }
    {
        const _: () = assert!(true, "!std::is_copy_constructible_v<MoveOnlyIter>");;
        let N: () = 3_();
        const _: () = assert!(true, "std::ranges::input_range<MoveOnlyRange>");;
        let mut in_: MoveOnlyRange_42 = <MoveOnlyRange_42>::default();
        {
            let mut out: Buffer_int__3_ = <Buffer_int__3_>::default();
            (unsafe {
                let ___ifirst: cpp20_input_iterator_const_int_ptr_ =
                    (unsafe { MoveOnlyRange_42::begin(&in_) });
                let ___ilast: sentinel_wrapper_cpp20_input_iterator_const_int_ptr__ =
                    (unsafe { MoveOnlyRange_42::end(&in_) });
                let ___ofirst: *mut () = (unsafe { Buffer_int__3_::begin(&mut out) });
                let ___olast: *mut () = (unsafe { Buffer_int__3_::end(&mut out) });
                (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                    ___ifirst, ___ilast, ___ofirst, ___olast,
                )
            });
        }
        {
            let mut out: Buffer_int__3_ = <Buffer_int__3_>::default();
            (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                    &mut in_, &mut out,
                )
            });
        }
    }
    {
        {
            let mut a: [MoveOnly; 4] = [
                MoveOnly::new({ Some(1_()) }),
                MoveOnly::new({ Some(2_()) }),
                MoveOnly::new({ Some(3_()) }),
                MoveOnly::new({ Some(4_()) }),
            ];
            let mut out: Buffer_MoveOnly__4_ = <Buffer_MoveOnly__4_>::default();
            (unsafe {
                let ___ifirst: *mut MoveOnly = unimplemented!();
                let ___ilast: *mut MoveOnly = unimplemented!();
                let ___ofirst: *mut MoveOnly = unimplemented!();
                let ___olast: *mut MoveOnly = unimplemented!();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                    ___ifirst, ___ilast, ___ofirst, ___olast,
                )
            });
            unimplemented!();
        }
        {
            let mut a: [MoveOnly; 4] = [
                MoveOnly::new({ Some(1_()) }),
                MoveOnly::new({ Some(2_()) }),
                MoveOnly::new({ Some(3_()) }),
                MoveOnly::new({ Some(4_()) }),
            ];
            let mut out: Buffer_MoveOnly__4_ = <Buffer_MoveOnly__4_>::default();
            (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                    &mut a, &mut out,
                )
            });
            unimplemented!();
        }
    }
    {
        let N: () = 5_();
        {
            let mut pool: [(); 20] = [
                0_(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
            ];
            let mut p: *mut () = (pool.as_mut_ptr() as *mut ());
            let mut p_end: *mut () = (pool.as_mut_ptr() as *mut ()).offset((N) as isize);
            let mut array: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
            (unsafe {
                let ___ifirst: overload_compare_iterator_int_ptr_ =
                    overload_compare_iterator_int_ptr_::new_1({ array.as_mut_ptr() });
                let ___ilast: overload_compare_iterator_int_ptr_ =
                    overload_compare_iterator_int_ptr_::new_1({
                        array.as_mut_ptr().offset((N) as isize)
                    });
                (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                    ___ifirst, ___ilast, p, p_end,
                )
            });
            let mut i: () = 0_();
            'loop_: while ((i) != (N)) {
                unimplemented!();
                i.prefix_inc();
            }
        }
        {
            let mut pool: [(); 20] = [
                0_(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
            ];
            let mut p: *mut () = (pool.as_mut_ptr() as *mut ());
            let mut p_end: *mut () = (pool.as_mut_ptr() as *mut ()).offset((N) as isize);
            let mut array: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
            (unsafe {
                let ___ifirst: *mut () = array.as_mut_ptr();
                let ___ilast: *mut () = array.as_mut_ptr().offset((N) as isize);
                (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_move_39))(
                    ___ifirst,
                    ___ilast,
                    overload_compare_iterator_int_ptr_::new_1({ p }),
                    overload_compare_iterator_int_ptr_::new_1({ p_end }),
                )
            });
            let mut i: () = 0_();
            'loop_: while ((i) != (N)) {
                unimplemented!();
                i.prefix_inc();
            }
        }
    }
    (unsafe { test_38() });
    return 0_();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct MoveOnlyRange_42 {
    pub buffer: [(); 3],
}
impl MoveOnlyRange_42 {
    pub unsafe fn begin(&self) -> cpp20_input_iterator_const_int_ptr_ {
        return cpp20_input_iterator_const_int_ptr_::new({ self.buffer.as_ptr() });
    }
    pub unsafe fn end(&self) -> sentinel_wrapper_cpp20_input_iterator_const_int_ptr__ {
        return {
            let mut __tmp_0: cpp20_input_iterator_const_int_ptr_ =
                cpp20_input_iterator_const_int_ptr_::new({ self.buffer.as_ptr() });
            sentinel_wrapper_cpp20_input_iterator_const_int_ptr__::new_1({ &mut __tmp_0 })
        };
    }
}
impl Default for MoveOnlyRange_42 {
    fn default() -> Self {
        unsafe {
            MoveOnlyRange_42 {
                buffer: [1_(), 2_(), 3_()],
            }
        }
    }
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct test_allocator_int_;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const current_objects_1);
    std::cell::LazyCell::force(&*&raw const total_objects_2);
    std::cell::LazyCell::force(&*&raw const total_copies_3);
    std::cell::LazyCell::force(&*&raw const total_moves_4);
    std::cell::LazyCell::force(&*&raw const throw_on_5);
    std::cell::LazyCell::force(&*&raw const destructed_value_22);
    std::cell::LazyCell::force(&*&raw const moved_value_23);
}
