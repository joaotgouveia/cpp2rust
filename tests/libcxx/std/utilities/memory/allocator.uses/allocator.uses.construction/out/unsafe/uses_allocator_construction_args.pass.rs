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
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
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
    pub unsafe fn copy_from(a: *const test_allocator_int_) -> Self {
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
    }
    pub unsafe fn get_data(&self) -> () {
        return self.data_;
    }
}
impl Clone for test_allocator_int_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_int_::copy_from(self as *const test_allocator_int_) }
    }
}
impl Default for test_allocator_int_ {
    fn default() -> Self {
        unsafe {
            test_allocator_int_ {
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
pub type RefType = ();
pub const RefType_LValue: RefType = 0;
pub const RefType_ConstLValue: RefType = 1;
pub const RefType_RValue: RefType = 2;
pub const RefType_ConstRValue: RefType = 3;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct UsesAllocArgT {
    pub allocator_constructed_: (),
    pub a_: test_allocator_int_,
    pub alloc_: *const test_allocator_int_,
    pub val_ptr_: *const (),
    pub ref_type_: RefType,
}
impl UsesAllocArgT {
    pub unsafe fn new_1(mut _a0: (), alloc: *const test_allocator_int_) -> Self {
        let mut this = Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: alloc,
            val_ptr_: std::ptr::null(),
            ref_type_: RefType_LValue,
        };
        this
    }
    pub unsafe fn new_2(mut _a0: (), alloc: *const test_allocator_int_, val: *mut ()) -> Self {
        let mut this = Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: alloc,
            val_ptr_: (val).cast_const(),
            ref_type_: RefType_LValue,
        };
        this
    }
    pub unsafe fn new_3(mut _a0: (), alloc: *const test_allocator_int_, val: *const ()) -> Self {
        let mut this = Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: alloc,
            val_ptr_: (val),
            ref_type_: RefType_ConstLValue,
        };
        this
    }
    pub unsafe fn new_4(mut _a0: (), alloc: *const test_allocator_int_, val: *mut ()) -> Self {
        let mut this = Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: alloc,
            val_ptr_: (val).cast_const(),
            ref_type_: RefType_RValue,
        };
        this
    }
    pub unsafe fn new_5(mut _a0: (), alloc: *const test_allocator_int_, val: *const ()) -> Self {
        let mut this = Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: alloc,
            val_ptr_: (val),
            ref_type_: RefType_ConstRValue,
        };
        this
    }
}
impl UsesAllocArgT {
    pub unsafe fn destructor(&mut self) {
        test_allocator_int_::destructor(&mut self.a_);
    }
}
impl Default for UsesAllocArgT {
    fn default() -> Self {
        unsafe {
            UsesAllocArgT {
                allocator_constructed_: false,
                a_: <test_allocator_int_>::default(),
                alloc_: &self.a_,
                val_ptr_: std::ptr::null(),
                ref_type_: RefType_LValue,
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct UsesAllocLast {
    pub allocator_constructed_: (),
    pub a_: test_allocator_int_,
    pub alloc_: *const test_allocator_int_,
    pub val_ptr_: *const (),
    pub ref_type_: RefType,
}
impl UsesAllocLast {
    pub unsafe fn new_1(alloc: *const test_allocator_int_) -> Self {
        let mut this = Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: alloc,
            val_ptr_: std::ptr::null(),
            ref_type_: RefType_LValue,
        };
        this
    }
    pub unsafe fn new_2(val: *mut (), alloc: *const test_allocator_int_) -> Self {
        let mut this = Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: alloc,
            val_ptr_: (val).cast_const(),
            ref_type_: RefType_LValue,
        };
        this
    }
    pub unsafe fn new_3(val: *const (), alloc: *const test_allocator_int_) -> Self {
        let mut this = Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: alloc,
            val_ptr_: (val),
            ref_type_: RefType_ConstLValue,
        };
        this
    }
    pub unsafe fn new_4(val: *mut (), alloc: *const test_allocator_int_) -> Self {
        let mut this = Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: alloc,
            val_ptr_: (val).cast_const(),
            ref_type_: RefType_RValue,
        };
        this
    }
    pub unsafe fn new_5(val: *const (), alloc: *const test_allocator_int_) -> Self {
        let mut this = Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: alloc,
            val_ptr_: (val),
            ref_type_: RefType_ConstRValue,
        };
        this
    }
}
impl UsesAllocLast {
    pub unsafe fn destructor(&mut self) {
        test_allocator_int_::destructor(&mut self.a_);
    }
}
impl Default for UsesAllocLast {
    fn default() -> Self {
        unsafe {
            UsesAllocLast {
                allocator_constructed_: false,
                a_: <test_allocator_int_>::default(),
                alloc_: &self.a_,
                val_ptr_: std::ptr::null(),
                ref_type_: RefType_LValue,
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct NotAllocatorAware {
    pub allocator_constructed_: (),
}
impl NotAllocatorAware {
    pub unsafe fn new_1(_a0: *const test_allocator_int_) -> Self {
        let mut this = Self {
            allocator_constructed_: true,
        };
        this
    }
    pub unsafe fn new_2(_a0: *const test_allocator_int_, mut _a1: ()) -> Self {
        let mut this = Self {
            allocator_constructed_: true,
        };
        this
    }
}
impl Default for NotAllocatorAware {
    fn default() -> Self {
        unsafe {
            NotAllocatorAware {
                allocator_constructed_: false,
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToPair {}
impl ConvertibleToPair {
    pub unsafe fn to_(&self) -> () {
        return unimplemented!();
    }
}
pub unsafe fn test_uses_allocator_construction_args_4(args: *mut test_allocator_int_) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe { uses_allocator_construction_args_5(args) });
}
pub unsafe fn test_uses_allocator_construction_args_6(args: *mut test_allocator_int_) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe { uses_allocator_construction_args_7(args) });
}
pub unsafe fn test_uses_allocator_construction_args_8(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
    args_2: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args_1: *mut () = args_1;
        let ___args_2: *mut () = args_2;
        uses_allocator_construction_args_9(___alloc, ___args_1, ___args_2)
    });
}
pub unsafe fn test_uses_allocator_construction_args_10(
    args_0: *mut test_allocator_int_,
    args_1: *const (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *const () = args_1;
        uses_allocator_construction_args_11(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_12(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut () = args_1;
        uses_allocator_construction_args_13(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_14(
    args_0: *mut test_allocator_int_,
    args_1: *const (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *const () = args_1;
        uses_allocator_construction_args_15(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_16(args: *mut test_allocator_int_) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe { uses_allocator_construction_args_17(args) });
}
pub unsafe fn test_uses_allocator_construction_args_18(args: *mut test_allocator_int_) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe { uses_allocator_construction_args_19(args) });
}
pub unsafe fn test_uses_allocator_construction_args_20(args: *mut test_allocator_int_) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe { uses_allocator_construction_args_21(args) });
}
pub unsafe fn test_uses_allocator_construction_args_22(
    args_0: *mut test_allocator_int_,
    args_1: *const (),
    args_2: *mut (),
    args_3: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args_1: *const () = args_1;
        let ___args_2: *mut () = args_2;
        let ___args_3: *mut () = args_3;
        uses_allocator_construction_args_23(___alloc, ___args_1, ___args_2, ___args_3)
    });
}
pub unsafe fn test_uses_allocator_construction_args_24(args: *mut test_allocator_int_) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe { uses_allocator_construction_args_25(args) });
}
pub unsafe fn test_uses_allocator_construction_args_26(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
    args_2: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args_1: *mut () = args_1;
        let ___args_2: *mut () = args_2;
        uses_allocator_construction_args_27(___alloc, ___args_1, ___args_2)
    });
}
pub unsafe fn test_uses_allocator_construction_args_28(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
    args_2: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args_1: *mut () = args_1;
        let ___args_2: *mut () = args_2;
        uses_allocator_construction_args_29(___alloc, ___args_1, ___args_2)
    });
}
pub unsafe fn test_uses_allocator_construction_args_30(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut () = args_1;
        uses_allocator_construction_args_31(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_32(
    args_0: *mut test_allocator_int_,
    args_1: *const (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *const () = args_1;
        uses_allocator_construction_args_33(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_34(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut () = args_1;
        uses_allocator_construction_args_35(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_36(
    args_0: *mut test_allocator_int_,
    args_1: *const (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *const () = args_1;
        uses_allocator_construction_args_37(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_38(
    args_0: *mut test_allocator_int_,
    args_1: *mut ConvertibleToPair,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut ConvertibleToPair = args_1;
        uses_allocator_construction_args_39(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_40(
    args_0: *mut test_allocator_int_,
    args_1: *mut ConvertibleToPair,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut ConvertibleToPair = args_1;
        uses_allocator_construction_args_41(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_42(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut () = args_1;
        uses_allocator_construction_args_43(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_44(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut () = args_1;
        uses_allocator_construction_args_45(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_46(args: *mut test_allocator_int_) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe { uses_allocator_construction_args_47(args) });
}
pub unsafe fn test_uses_allocator_construction_args_48(args: *mut test_allocator_int_) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe { uses_allocator_construction_args_49(args) });
}
pub unsafe fn test_uses_allocator_construction_args_50(args: *mut test_allocator_int_) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe { uses_allocator_construction_args_51(args) });
}
pub unsafe fn test_uses_allocator_construction_args_52(
    args_0: *mut test_allocator_int_,
    args_1: *const (),
    args_2: *mut (),
    args_3: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args_1: *const () = args_1;
        let ___args_2: *mut () = args_2;
        let ___args_3: *mut () = args_3;
        uses_allocator_construction_args_53(___alloc, ___args_1, ___args_2, ___args_3)
    });
}
pub unsafe fn test_uses_allocator_construction_args_54(args: *mut test_allocator_int_) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe { uses_allocator_construction_args_55(args) });
}
pub unsafe fn test_uses_allocator_construction_args_56(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
    args_2: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args_1: *mut () = args_1;
        let ___args_2: *mut () = args_2;
        uses_allocator_construction_args_57(___alloc, ___args_1, ___args_2)
    });
}
pub unsafe fn test_uses_allocator_construction_args_58(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
    args_2: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args_1: *mut () = args_1;
        let ___args_2: *mut () = args_2;
        uses_allocator_construction_args_59(___alloc, ___args_1, ___args_2)
    });
}
pub unsafe fn test_uses_allocator_construction_args_60(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut () = args_1;
        uses_allocator_construction_args_61(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_62(
    args_0: *mut test_allocator_int_,
    args_1: *const (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *const () = args_1;
        uses_allocator_construction_args_63(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_64(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut () = args_1;
        uses_allocator_construction_args_65(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_66(
    args_0: *mut test_allocator_int_,
    args_1: *const (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *const () = args_1;
        uses_allocator_construction_args_67(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_68(
    args_0: *mut test_allocator_int_,
    args_1: *mut ConvertibleToPair,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut ConvertibleToPair = args_1;
        uses_allocator_construction_args_69(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_70(
    args_0: *mut test_allocator_int_,
    args_1: *mut ConvertibleToPair,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut ConvertibleToPair = args_1;
        uses_allocator_construction_args_71(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_72(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut () = args_1;
        uses_allocator_construction_args_73(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_74(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut () = args_1;
        uses_allocator_construction_args_75(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_76(args: *mut test_allocator_int_) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe { uses_allocator_construction_args_77(args) });
}
pub unsafe fn test_uses_allocator_construction_args_78(args: *mut test_allocator_int_) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe { uses_allocator_construction_args_79(args) });
}
pub unsafe fn test_uses_allocator_construction_args_80(args: *mut test_allocator_int_) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe { uses_allocator_construction_args_81(args) });
}
pub unsafe fn test_uses_allocator_construction_args_82(
    args_0: *mut test_allocator_int_,
    args_1: *const (),
    args_2: *mut (),
    args_3: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args_1: *const () = args_1;
        let ___args_2: *mut () = args_2;
        let ___args_3: *mut () = args_3;
        uses_allocator_construction_args_83(___alloc, ___args_1, ___args_2, ___args_3)
    });
}
pub unsafe fn test_uses_allocator_construction_args_84(args: *mut test_allocator_int_) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe { uses_allocator_construction_args_85(args) });
}
pub unsafe fn test_uses_allocator_construction_args_86(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
    args_2: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args_1: *mut () = args_1;
        let ___args_2: *mut () = args_2;
        uses_allocator_construction_args_87(___alloc, ___args_1, ___args_2)
    });
}
pub unsafe fn test_uses_allocator_construction_args_88(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
    args_2: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args_1: *mut () = args_1;
        let ___args_2: *mut () = args_2;
        uses_allocator_construction_args_89(___alloc, ___args_1, ___args_2)
    });
}
pub unsafe fn test_uses_allocator_construction_args_90(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut () = args_1;
        uses_allocator_construction_args_91(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_92(
    args_0: *mut test_allocator_int_,
    args_1: *const (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *const () = args_1;
        uses_allocator_construction_args_93(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_94(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut () = args_1;
        uses_allocator_construction_args_95(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_96(
    args_0: *mut test_allocator_int_,
    args_1: *const (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *const () = args_1;
        uses_allocator_construction_args_97(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_98(
    args_0: *mut test_allocator_int_,
    args_1: *mut ConvertibleToPair,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut ConvertibleToPair = args_1;
        uses_allocator_construction_args_99(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_100(
    args_0: *mut test_allocator_int_,
    args_1: *mut ConvertibleToPair,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut ConvertibleToPair = args_1;
        uses_allocator_construction_args_101(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_102(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut () = args_1;
        uses_allocator_construction_args_103(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_104(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut () = args_1;
        uses_allocator_construction_args_105(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_106(args: *mut test_allocator_int_) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe { uses_allocator_construction_args_107(args) });
}
pub unsafe fn test_uses_allocator_construction_args_108(args: *mut test_allocator_int_) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe { uses_allocator_construction_args_109(args) });
}
pub unsafe fn test_uses_allocator_construction_args_110(args: *mut test_allocator_int_) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe { uses_allocator_construction_args_111(args) });
}
pub unsafe fn test_uses_allocator_construction_args_112(
    args_0: *mut test_allocator_int_,
    args_1: *const (),
    args_2: *mut (),
    args_3: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args_1: *const () = args_1;
        let ___args_2: *mut () = args_2;
        let ___args_3: *mut () = args_3;
        uses_allocator_construction_args_113(___alloc, ___args_1, ___args_2, ___args_3)
    });
}
pub unsafe fn test_uses_allocator_construction_args_114(args: *mut test_allocator_int_) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe { uses_allocator_construction_args_115(args) });
}
pub unsafe fn test_uses_allocator_construction_args_116(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
    args_2: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args_1: *mut () = args_1;
        let ___args_2: *mut () = args_2;
        uses_allocator_construction_args_117(___alloc, ___args_1, ___args_2)
    });
}
pub unsafe fn test_uses_allocator_construction_args_118(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
    args_2: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args_1: *mut () = args_1;
        let ___args_2: *mut () = args_2;
        uses_allocator_construction_args_119(___alloc, ___args_1, ___args_2)
    });
}
pub unsafe fn test_uses_allocator_construction_args_120(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut () = args_1;
        uses_allocator_construction_args_121(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_122(
    args_0: *mut test_allocator_int_,
    args_1: *const (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *const () = args_1;
        uses_allocator_construction_args_123(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_124(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut () = args_1;
        uses_allocator_construction_args_125(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_126(
    args_0: *mut test_allocator_int_,
    args_1: *const (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *const () = args_1;
        uses_allocator_construction_args_127(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_128(
    args_0: *mut test_allocator_int_,
    args_1: *mut ConvertibleToPair,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut ConvertibleToPair = args_1;
        uses_allocator_construction_args_129(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_130(
    args_0: *mut test_allocator_int_,
    args_1: *mut ConvertibleToPair,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut ConvertibleToPair = args_1;
        uses_allocator_construction_args_131(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_132(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut () = args_1;
        uses_allocator_construction_args_133(___alloc, ___args)
    });
}
pub unsafe fn test_uses_allocator_construction_args_134(
    args_0: *mut test_allocator_int_,
    args_1: *mut (),
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return (unsafe {
        let ___alloc: *const test_allocator_int_ = args_0;
        let ___args: *mut () = args_1;
        uses_allocator_construction_args_135(___alloc, ___args)
    });
}
pub unsafe fn testOne_136() {
    let mut a: test_allocator_int_ = test_allocator_int_::new_1({ 12_() });
    let _dtor_a = ScopedDestructorUnsafe::new(&raw mut a, test_allocator_int_::destructor);
    {
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_16(&mut a) });
        unimplemented!();
    }
    {
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_18(&mut a) });
        unimplemented!();
    }
    {
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_20(&mut a) });
    }
    {
        let mut ret: () = (unsafe {
            let mut _args_2: () = unimplemented!();
            let mut _args_3: () = unimplemented!();
            test_uses_allocator_construction_args_22(
                &mut a,
                &(*std::cell::LazyCell::force_mut(&mut *&raw mut piecewise_construct_137)),
                &mut _args_2,
                &mut _args_3,
            )
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_24(&mut a) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut val: () = 0_();
        let mut ret: () = (unsafe {
            let _args_1: *mut () = &mut val;
            let _args_2: *mut () = &mut val;
            test_uses_allocator_construction_args_26(&mut a, _args_1, _args_2)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut val: () = 0_();
        let mut ret: () = (unsafe {
            let _args_1: *mut () = &mut *(&raw mut val).cast::<()>();
            let _args_2: *mut () = &mut *(&raw mut val).cast::<()>();
            test_uses_allocator_construction_args_28(&mut a, _args_1, _args_2)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_30(&mut a, &mut p) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        let mut ret: () =
            (unsafe { test_uses_allocator_construction_args_32(&mut a, &(*unimplemented!())) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        let mut ret: () = (unsafe {
            test_uses_allocator_construction_args_34(&mut a, &mut *(&raw mut p).cast::<()>())
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        {
            let mut ret: () = (unsafe { test_uses_allocator_construction_args_4(&mut a) });
            &(ret);
        }
        {
            let mut ret: () = (unsafe { test_uses_allocator_construction_args_6(&mut a) });
            &(ret);
        }
        {
            let mut up: () = 1_();
            let mut vp: () = 2_();
            let mut ret: () = (unsafe {
                let _args_1: *mut () = &mut *(&raw mut up).cast::<()>();
                let _args_2: *mut () = &mut *(&raw mut vp).cast::<()>();
                test_uses_allocator_construction_args_8(&mut a, _args_1, _args_2)
            });
            &(ret);
        }
        {
            let p: () = unimplemented!();
            let mut ret: () = (unsafe { test_uses_allocator_construction_args_10(&mut a, &p) });
            &(ret);
        }
        {
            let mut p: () = unimplemented!();
            let mut ret: () = (unsafe {
                test_uses_allocator_construction_args_12(&mut a, &mut *(&raw mut p).cast::<()>())
            });
            &(ret);
        }
        {
            let p: () = unimplemented!();
            let mut ret: () = (unsafe { test_uses_allocator_construction_args_14(&mut a, &p) });
            &(ret);
        }
    }
    {
        let mut p: () = unimplemented!();
        let mut ret: () =
            (unsafe { test_uses_allocator_construction_args_36(&mut a, unimplemented!()) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ctp: ConvertibleToPair = <ConvertibleToPair>::default();
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_38(&mut a, &mut ctp) });
        let mut v: () = (unsafe { (*(unsafe { get_138(&mut ret) }))() });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ctp: ConvertibleToPair = <ConvertibleToPair>::default();
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_40(&mut a, &mut ctp) });
        let mut v: () = (unsafe { (*(unsafe { get_139(&mut ret) }))() });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut pair_like: () = unimplemented!();
        let mut ret: () =
            (unsafe { test_uses_allocator_construction_args_42(&mut a, &mut pair_like) });
        let mut pair: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut i: () = 5_();
        let mut r: () = unimplemented!();
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_44(&mut a, &mut r) });
        let mut pair: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testOne_142() {
    let mut a: test_allocator_int_ = test_allocator_int_::new_1({ 12_() });
    let _dtor_a = ScopedDestructorUnsafe::new(&raw mut a, test_allocator_int_::destructor);
    {
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_46(&mut a) });
        unimplemented!();
    }
    {
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_48(&mut a) });
        unimplemented!();
    }
    {
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_50(&mut a) });
    }
    {
        let mut ret: () = (unsafe {
            let mut _args_2: () = unimplemented!();
            let mut _args_3: () = unimplemented!();
            test_uses_allocator_construction_args_52(
                &mut a,
                &(*std::cell::LazyCell::force_mut(&mut *&raw mut piecewise_construct_137)),
                &mut _args_2,
                &mut _args_3,
            )
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_54(&mut a) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut val: () = 0_();
        let mut ret: () = (unsafe {
            let _args_1: *mut () = &mut val;
            let _args_2: *mut () = &mut val;
            test_uses_allocator_construction_args_56(&mut a, _args_1, _args_2)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut val: () = 0_();
        let mut ret: () = (unsafe {
            let _args_1: *mut () = &mut *(&raw mut val).cast::<()>();
            let _args_2: *mut () = &mut *(&raw mut val).cast::<()>();
            test_uses_allocator_construction_args_58(&mut a, _args_1, _args_2)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_60(&mut a, &mut p) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        let mut ret: () =
            (unsafe { test_uses_allocator_construction_args_62(&mut a, &(*unimplemented!())) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        let mut ret: () = (unsafe {
            test_uses_allocator_construction_args_64(&mut a, &mut *(&raw mut p).cast::<()>())
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        {
            let mut ret: () = (unsafe { test_uses_allocator_construction_args_4(&mut a) });
            &(ret);
        }
        {
            let mut ret: () = (unsafe { test_uses_allocator_construction_args_6(&mut a) });
            &(ret);
        }
        {
            let mut up: () = 1_();
            let mut vp: () = 2_();
            let mut ret: () = (unsafe {
                let _args_1: *mut () = &mut *(&raw mut up).cast::<()>();
                let _args_2: *mut () = &mut *(&raw mut vp).cast::<()>();
                test_uses_allocator_construction_args_8(&mut a, _args_1, _args_2)
            });
            &(ret);
        }
        {
            let p: () = unimplemented!();
            let mut ret: () = (unsafe { test_uses_allocator_construction_args_10(&mut a, &p) });
            &(ret);
        }
        {
            let mut p: () = unimplemented!();
            let mut ret: () = (unsafe {
                test_uses_allocator_construction_args_12(&mut a, &mut *(&raw mut p).cast::<()>())
            });
            &(ret);
        }
        {
            let p: () = unimplemented!();
            let mut ret: () = (unsafe { test_uses_allocator_construction_args_14(&mut a, &p) });
            &(ret);
        }
    }
    {
        let mut p: () = unimplemented!();
        let mut ret: () =
            (unsafe { test_uses_allocator_construction_args_66(&mut a, unimplemented!()) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ctp: ConvertibleToPair = <ConvertibleToPair>::default();
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_68(&mut a, &mut ctp) });
        let mut v: () = (unsafe { (*(unsafe { get_143(&mut ret) }))() });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ctp: ConvertibleToPair = <ConvertibleToPair>::default();
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_70(&mut a, &mut ctp) });
        let mut v: () = (unsafe { (*(unsafe { get_144(&mut ret) }))() });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut pair_like: () = unimplemented!();
        let mut ret: () =
            (unsafe { test_uses_allocator_construction_args_72(&mut a, &mut pair_like) });
        let mut pair: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut i: () = 5_();
        let mut r: () = unimplemented!();
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_74(&mut a, &mut r) });
        let mut pair: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testOne_147() {
    let mut a: test_allocator_int_ = test_allocator_int_::new_1({ 12_() });
    let _dtor_a = ScopedDestructorUnsafe::new(&raw mut a, test_allocator_int_::destructor);
    {
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_76(&mut a) });
        unimplemented!();
    }
    {
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_78(&mut a) });
        unimplemented!();
    }
    {
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_80(&mut a) });
    }
    {
        let mut ret: () = (unsafe {
            let mut _args_2: () = unimplemented!();
            let mut _args_3: () = unimplemented!();
            test_uses_allocator_construction_args_82(
                &mut a,
                &(*std::cell::LazyCell::force_mut(&mut *&raw mut piecewise_construct_137)),
                &mut _args_2,
                &mut _args_3,
            )
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_84(&mut a) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut val: () = 0_();
        let mut ret: () = (unsafe {
            let _args_1: *mut () = &mut val;
            let _args_2: *mut () = &mut val;
            test_uses_allocator_construction_args_86(&mut a, _args_1, _args_2)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut val: () = 0_();
        let mut ret: () = (unsafe {
            let _args_1: *mut () = &mut *(&raw mut val).cast::<()>();
            let _args_2: *mut () = &mut *(&raw mut val).cast::<()>();
            test_uses_allocator_construction_args_88(&mut a, _args_1, _args_2)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_90(&mut a, &mut p) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        let mut ret: () =
            (unsafe { test_uses_allocator_construction_args_92(&mut a, &(*unimplemented!())) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        let mut ret: () = (unsafe {
            test_uses_allocator_construction_args_94(&mut a, &mut *(&raw mut p).cast::<()>())
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        {
            let mut ret: () = (unsafe { test_uses_allocator_construction_args_4(&mut a) });
            &(ret);
        }
        {
            let mut ret: () = (unsafe { test_uses_allocator_construction_args_6(&mut a) });
            &(ret);
        }
        {
            let mut up: () = 1_();
            let mut vp: () = 2_();
            let mut ret: () = (unsafe {
                let _args_1: *mut () = &mut *(&raw mut up).cast::<()>();
                let _args_2: *mut () = &mut *(&raw mut vp).cast::<()>();
                test_uses_allocator_construction_args_8(&mut a, _args_1, _args_2)
            });
            &(ret);
        }
        {
            let p: () = unimplemented!();
            let mut ret: () = (unsafe { test_uses_allocator_construction_args_10(&mut a, &p) });
            &(ret);
        }
        {
            let mut p: () = unimplemented!();
            let mut ret: () = (unsafe {
                test_uses_allocator_construction_args_12(&mut a, &mut *(&raw mut p).cast::<()>())
            });
            &(ret);
        }
        {
            let p: () = unimplemented!();
            let mut ret: () = (unsafe { test_uses_allocator_construction_args_14(&mut a, &p) });
            &(ret);
        }
    }
    {
        let mut p: () = unimplemented!();
        let mut ret: () =
            (unsafe { test_uses_allocator_construction_args_96(&mut a, unimplemented!()) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ctp: ConvertibleToPair = <ConvertibleToPair>::default();
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_98(&mut a, &mut ctp) });
        let mut v: () = (unsafe { (*(unsafe { get_148(&mut ret) }))() });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ctp: ConvertibleToPair = <ConvertibleToPair>::default();
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_100(&mut a, &mut ctp) });
        let mut v: () = (unsafe { (*(unsafe { get_149(&mut ret) }))() });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut pair_like: () = unimplemented!();
        let mut ret: () =
            (unsafe { test_uses_allocator_construction_args_102(&mut a, &mut pair_like) });
        let mut pair: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut i: () = 5_();
        let mut r: () = unimplemented!();
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_104(&mut a, &mut r) });
        let mut pair: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testOne_152() {
    let mut a: test_allocator_int_ = test_allocator_int_::new_1({ 12_() });
    let _dtor_a = ScopedDestructorUnsafe::new(&raw mut a, test_allocator_int_::destructor);
    {
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_106(&mut a) });
        unimplemented!();
    }
    {
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_108(&mut a) });
        unimplemented!();
    }
    {
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_110(&mut a) });
    }
    {
        let mut ret: () = (unsafe {
            let mut _args_2: () = unimplemented!();
            let mut _args_3: () = unimplemented!();
            test_uses_allocator_construction_args_112(
                &mut a,
                &(*std::cell::LazyCell::force_mut(&mut *&raw mut piecewise_construct_137)),
                &mut _args_2,
                &mut _args_3,
            )
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_114(&mut a) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut val: () = 0_();
        let mut ret: () = (unsafe {
            let _args_1: *mut () = &mut val;
            let _args_2: *mut () = &mut val;
            test_uses_allocator_construction_args_116(&mut a, _args_1, _args_2)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut val: () = 0_();
        let mut ret: () = (unsafe {
            let _args_1: *mut () = &mut *(&raw mut val).cast::<()>();
            let _args_2: *mut () = &mut *(&raw mut val).cast::<()>();
            test_uses_allocator_construction_args_118(&mut a, _args_1, _args_2)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_120(&mut a, &mut p) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        let mut ret: () =
            (unsafe { test_uses_allocator_construction_args_122(&mut a, &(*unimplemented!())) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        let mut ret: () = (unsafe {
            test_uses_allocator_construction_args_124(&mut a, &mut *(&raw mut p).cast::<()>())
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        {
            let mut ret: () = (unsafe { test_uses_allocator_construction_args_4(&mut a) });
            &(ret);
        }
        {
            let mut ret: () = (unsafe { test_uses_allocator_construction_args_6(&mut a) });
            &(ret);
        }
        {
            let mut up: () = 1_();
            let mut vp: () = 2_();
            let mut ret: () = (unsafe {
                let _args_1: *mut () = &mut *(&raw mut up).cast::<()>();
                let _args_2: *mut () = &mut *(&raw mut vp).cast::<()>();
                test_uses_allocator_construction_args_8(&mut a, _args_1, _args_2)
            });
            &(ret);
        }
        {
            let p: () = unimplemented!();
            let mut ret: () = (unsafe { test_uses_allocator_construction_args_10(&mut a, &p) });
            &(ret);
        }
        {
            let mut p: () = unimplemented!();
            let mut ret: () = (unsafe {
                test_uses_allocator_construction_args_12(&mut a, &mut *(&raw mut p).cast::<()>())
            });
            &(ret);
        }
        {
            let p: () = unimplemented!();
            let mut ret: () = (unsafe { test_uses_allocator_construction_args_14(&mut a, &p) });
            &(ret);
        }
    }
    {
        let mut p: () = unimplemented!();
        let mut ret: () =
            (unsafe { test_uses_allocator_construction_args_126(&mut a, unimplemented!()) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ctp: ConvertibleToPair = <ConvertibleToPair>::default();
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_128(&mut a, &mut ctp) });
        let mut v: () = (unsafe { (*(unsafe { get_153(&mut ret) }))() });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ctp: ConvertibleToPair = <ConvertibleToPair>::default();
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_130(&mut a, &mut ctp) });
        let mut v: () = (unsafe { (*(unsafe { get_154(&mut ret) }))() });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut pair_like: () = unimplemented!();
        let mut ret: () =
            (unsafe { test_uses_allocator_construction_args_132(&mut a, &mut pair_like) });
        let mut pair: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut i: () = 5_();
        let mut r: () = unimplemented!();
        let mut ret: () = (unsafe { test_uses_allocator_construction_args_134(&mut a, &mut r) });
        let mut pair: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Foo_140 {
    pub i: (),
}
impl Default for Foo_140 {
    fn default() -> Self {
        unsafe { Foo_140 { i: 5_() } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Bar_141 {
    pub i: (),
}
impl Bar_141 {
    pub unsafe fn new(mut foo: Foo_140) -> Self {
        let mut this = Self { i: foo.i };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Foo_145 {
    pub i: (),
}
impl Default for Foo_145 {
    fn default() -> Self {
        unsafe { Foo_145 { i: 5_() } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Bar_146 {
    pub i: (),
}
impl Bar_146 {
    pub unsafe fn new(mut foo: Foo_145) -> Self {
        let mut this = Self { i: foo.i };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Foo_150 {
    pub i: (),
}
impl Default for Foo_150 {
    fn default() -> Self {
        unsafe { Foo_150 { i: 5_() } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Bar_151 {
    pub i: (),
}
impl Bar_151 {
    pub unsafe fn new(mut foo: Foo_150) -> Self {
        let mut this = Self { i: foo.i };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Foo_155 {
    pub i: (),
}
impl Default for Foo_155 {
    fn default() -> Self {
        unsafe { Foo_155 { i: 5_() } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Bar_156 {
    pub i: (),
}
impl Bar_156 {
    pub unsafe fn new(mut foo: Foo_155) -> Self {
        let mut this = Self { i: foo.i };
        this
    }
}
pub unsafe fn test_157() -> () {
    (unsafe { testOne_136() });
    (unsafe { testOne_142() });
    (unsafe { testOne_147() });
    (unsafe { testOne_152() });
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
    (unsafe { test_157() });
    const _: () = assert!(true, "test()");;
    0
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_0);
    std::cell::LazyCell::force(&*&raw const moved_value_1);
}
