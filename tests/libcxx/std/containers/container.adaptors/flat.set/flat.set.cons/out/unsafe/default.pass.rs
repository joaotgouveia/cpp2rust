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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct explicit_allocator_int_ {}
impl explicit_allocator_int_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut i32 {
        return ((unsafe { <std_allocator_int_>::default().allocate(n) }) as *mut i32);
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut i32, mut n: usize) {
        (unsafe { <std_allocator_int_>::default().deallocate(p, n) });
    }
}
impl Default for explicit_allocator_int_ {
    fn default() -> Self {
        unsafe { explicit_allocator_int_::new() }
    }
}
impl explicit_allocator_int_ptr_ {
    pub unsafe fn new_1(mut _a0: explicit_allocator_int_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct explicit_allocator_int_ptr_ {}
impl explicit_allocator_int_ptr_ {
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut *mut i32 {
        return ((unsafe { <std_allocator_int_ptr_>::default().allocate(n) }) as *mut *mut i32);
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut i32, mut n: usize) {
        (unsafe { <std_allocator_int_ptr_>::default().deallocate(p, n) });
    }
}
pub unsafe fn operator_eq_4(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_5(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_6(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_7(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_8(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_9(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_10(_a0: *const MoveOnly, _a1: *const MoveOnly) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct MoveOnly {
    data_: i32,
}
impl MoveOnly {
    pub unsafe fn new(mut data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| unsafe { 1 });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(x: *mut MoveOnly) -> Self {
        let mut this = Self { data_: (*x).data_ };
        (*x).data_ = 0;
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut MoveOnly) -> *mut MoveOnly {
        self.data_ = (*x).data_;
        (*x).data_ = 0;
        return &mut (*(self as *mut MoveOnly));
    }
    pub unsafe fn get(&self) -> i32 {
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
        unsafe { operator_cmp_10(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_4(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        unsafe { MoveOnly::new(None) }
    }
}
pub unsafe fn operator_eq_11(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_12(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_13(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_14(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_15(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_16(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_17(
    _a0: *const TrivialMoveOnly,
    _a1: *const TrivialMoveOnly,
) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct TrivialMoveOnly {
    data_: i32,
}
impl TrivialMoveOnly {
    pub unsafe fn new(mut data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| unsafe { 1 });
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
    pub unsafe fn get(&self) -> i32 {
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
            operator_cmp_17(
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
            operator_eq_11(
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
pub static mut destructed_value_18: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { -1_i32 });
pub static mut moved_value_19: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 2147483647 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_20(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_21(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_20(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_18));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_18));
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
            operator_eq_20(
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
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct other_allocator_MoveOnly_ {
    data_: i32,
}
impl other_allocator_MoveOnly_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { data_: -1_i32 };
        this
    }
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut MoveOnly {
        return (unsafe { <std_allocator_MoveOnly_>::default().allocate(n) });
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut MoveOnly, mut s: usize) {
        (unsafe { <std_allocator_MoveOnly_>::default().deallocate(p, s) });
    }
}
impl Default for other_allocator_MoveOnly_ {
    fn default() -> Self {
        unsafe { other_allocator_MoveOnly_::new_1() }
    }
}
impl other_allocator_MoveOnly_ptr_ {
    pub unsafe fn new_1(a: *const other_allocator_MoveOnly_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct other_allocator_MoveOnly_ptr_ {
    data_: i32,
}
impl other_allocator_MoveOnly_ptr_ {
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut *mut MoveOnly {
        return (unsafe { <std_allocator_MoveOnly_ptr_>::default().allocate(n) });
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut MoveOnly, mut s: usize) {
        (unsafe { <std_allocator_MoveOnly_ptr_>::default().deallocate(p, s) });
    }
}
impl Default for other_allocator_MoveOnly_ptr_ {
    fn default() -> Self {
        unsafe {
            other_allocator_MoveOnly_ptr_ {
                data_: <i32>::default(),
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
pub struct DefaultCtableComp {
    pub default_constructed_: bool,
}
impl DefaultCtableComp {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            default_constructed_: false,
        };
        this.default_constructed_ = true;
        this
    }
    pub unsafe fn operator_call(&self, mut _a0: i32, mut _a1: i32) -> bool {
        return false;
    }
}
impl Default for DefaultCtableComp {
    fn default() -> Self {
        unsafe { DefaultCtableComp::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ThrowingCtorComp {}
impl ThrowingCtorComp {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
}
impl Default for ThrowingCtorComp {
    fn default() -> Self {
        unsafe { ThrowingCtorComp::new() }
    }
}
pub unsafe fn test_22() {
    {
        let mut m: std_flat_set_int__std_less_int___std_vector_int__std_allocator_int___ =
            <std_flat_set_int__std_less_int___std_vector_int__std_allocator_int___>::default();
        assert!(
            (unsafe {
                ( m as std__Flat_set_impl_int__std_less_int___std_vector_int__std_allocator_int____false_ ) . empty  ( )
            })
        );
    }
    {
        let mut m: std_flat_set_int__std_less_int___std_vector_int__std_allocator_int___ =
            <std_flat_set_int__std_less_int___std_vector_int__std_allocator_int___>::default();
        assert!(
            (unsafe {
                ( m as std__Flat_set_impl_int__std_less_int___std_vector_int__std_allocator_int____false_ ) . empty  ( )
            })
        );
    }
    {
        let mut m: std_flat_set_int__DefaultCtableComp__std_vector_int__min_allocator_int___ =
            <std_flat_set_int__DefaultCtableComp__std_vector_int__min_allocator_int___>::default();
        assert!(
            (unsafe {
                ( m as std__Flat_set_impl_int__DefaultCtableComp__std_vector_int__min_allocator_int____false_ ) . empty  ( )
            })
        );
        assert!(
            (unsafe {
                ( m as std__Flat_set_impl_int__DefaultCtableComp__std_vector_int__min_allocator_int____false_ ) . begin  ( )
            }) == (unsafe {
                ( m as std__Flat_set_impl_int__DefaultCtableComp__std_vector_int__min_allocator_int____false_ ) . end  ( )
            })
        );
        assert!(( unsafe { ( m as std__Flat_set_impl_int__DefaultCtableComp__std_vector_int__min_allocator_int____false_ ) . key_comp  ( ) } )  . default_constructed_   );
    }
    {
        {
            let mut m : std_flat_set_int__DefaultCtableComp__std_vector_int__explicit_allocator_int___ = <std_flat_set_int__DefaultCtableComp__std_vector_int__explicit_allocator_int___ >::default() ;
            assert!(
                (unsafe {
                    ( m as std__Flat_set_impl_int__DefaultCtableComp__std_vector_int__explicit_allocator_int____false_ ) . empty  ( )
                })
            );
            assert!(( unsafe { ( m as std__Flat_set_impl_int__DefaultCtableComp__std_vector_int__explicit_allocator_int____false_ ) . key_comp  ( ) } )  . default_constructed_   );
        }
        {
            let mut a1: explicit_allocator_int_ = explicit_allocator_int_::new();
            let mut m : std_flat_set_int__DefaultCtableComp__std_vector_int__explicit_allocator_int___ = std_flat_set_int__DefaultCtableComp__std_vector_int__explicit_allocator_int___ :: new_1 ( { & a1 } , )  ;
            assert!(
                (unsafe {
                    ( m as std__Flat_set_impl_int__DefaultCtableComp__std_vector_int__explicit_allocator_int____false_ ) . empty  ( )
                })
            );
            assert!(( unsafe { ( m as std__Flat_set_impl_int__DefaultCtableComp__std_vector_int__explicit_allocator_int____false_ ) . key_comp  ( ) } )  . default_constructed_   );
        }
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_default_constructible_v<C>");;
        let mut c : std_flat_set_MoveOnly__std_less_MoveOnly___std_vector_MoveOnly__other_allocator_MoveOnly___ = <std_flat_set_MoveOnly__std_less_MoveOnly___std_vector_MoveOnly__other_allocator_MoveOnly___ >::default() ;
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_default_constructible_v<C>");;
        let mut c : std_flat_set_MoveOnly__ThrowingCtorComp__std_vector_MoveOnly__std_allocator_MoveOnly___ = <std_flat_set_MoveOnly__ThrowingCtorComp__std_vector_MoveOnly__std_allocator_MoveOnly___ >::default() ;
    }
}
pub unsafe fn test_23() {
    {
        let mut m: std_flat_set_int__std_less_int___std_deque_int__std_allocator_int___ =
            <std_flat_set_int__std_less_int___std_deque_int__std_allocator_int___>::default();
        assert!(
            (unsafe {
                ( m as std__Flat_set_impl_int__std_less_int___std_deque_int__std_allocator_int____false_ ) . empty  ( )
            })
        );
    }
    {
        let mut m: std_flat_set_int__std_less_int___std_deque_int__std_allocator_int___ =
            <std_flat_set_int__std_less_int___std_deque_int__std_allocator_int___>::default();
        assert!(
            (unsafe {
                ( m as std__Flat_set_impl_int__std_less_int___std_deque_int__std_allocator_int____false_ ) . empty  ( )
            })
        );
    }
    {
        let mut m: std_flat_set_int__DefaultCtableComp__std_deque_int__min_allocator_int___ =
            <std_flat_set_int__DefaultCtableComp__std_deque_int__min_allocator_int___>::default();
        assert!(
            (unsafe {
                ( m as std__Flat_set_impl_int__DefaultCtableComp__std_deque_int__min_allocator_int____false_ ) . empty  ( )
            })
        );
        assert!(
            (unsafe {
                ( m as std__Flat_set_impl_int__DefaultCtableComp__std_deque_int__min_allocator_int____false_ ) . begin  ( )
            }) == (unsafe {
                ( m as std__Flat_set_impl_int__DefaultCtableComp__std_deque_int__min_allocator_int____false_ ) . end  ( )
            })
        );
        assert!(( unsafe { ( m as std__Flat_set_impl_int__DefaultCtableComp__std_deque_int__min_allocator_int____false_ ) . key_comp  ( ) } )  . default_constructed_   );
    }
    {
        {
            let mut m : std_flat_set_int__DefaultCtableComp__std_deque_int__explicit_allocator_int___ = <std_flat_set_int__DefaultCtableComp__std_deque_int__explicit_allocator_int___ >::default() ;
            assert!(
                (unsafe {
                    ( m as std__Flat_set_impl_int__DefaultCtableComp__std_deque_int__explicit_allocator_int____false_ ) . empty  ( )
                })
            );
            assert!(( unsafe { ( m as std__Flat_set_impl_int__DefaultCtableComp__std_deque_int__explicit_allocator_int____false_ ) . key_comp  ( ) } )  . default_constructed_   );
        }
        {
            let mut a1: explicit_allocator_int_ = explicit_allocator_int_::new();
            let mut m : std_flat_set_int__DefaultCtableComp__std_deque_int__explicit_allocator_int___ = std_flat_set_int__DefaultCtableComp__std_deque_int__explicit_allocator_int___ :: new_1 ( { & a1 } , )  ;
            assert!(
                (unsafe {
                    ( m as std__Flat_set_impl_int__DefaultCtableComp__std_deque_int__explicit_allocator_int____false_ ) . empty  ( )
                })
            );
            assert!(( unsafe { ( m as std__Flat_set_impl_int__DefaultCtableComp__std_deque_int__explicit_allocator_int____false_ ) . key_comp  ( ) } )  . default_constructed_   );
        }
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_default_constructible_v<C>");;
        let mut c : std_flat_set_MoveOnly__std_less_MoveOnly___std_deque_MoveOnly__other_allocator_MoveOnly___ = <std_flat_set_MoveOnly__std_less_MoveOnly___std_deque_MoveOnly__other_allocator_MoveOnly___ >::default() ;
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_default_constructible_v<C>");;
        let mut c : std_flat_set_MoveOnly__ThrowingCtorComp__std_deque_MoveOnly__std_allocator_MoveOnly___ = <std_flat_set_MoveOnly__ThrowingCtorComp__std_deque_MoveOnly__std_allocator_MoveOnly___ >::default() ;
    }
}
pub unsafe fn test_24() -> bool {
    (unsafe { test_22() });
    {
        (unsafe { test_23() });
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
    (unsafe { test_24() });
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
    std::cell::LazyCell::force(&*&raw const destructed_value_18);
    std::cell::LazyCell::force(&*&raw const moved_value_19);
}
