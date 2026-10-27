extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(LHS: *const TypeID, RHS: *const TypeID) -> () {
    return (((*LHS).m_id) == ((*RHS).m_id));
}
pub unsafe fn operator_ne_1(LHS: *const TypeID, RHS: *const TypeID) -> () {
    return (((*LHS).m_id) != ((*RHS).m_id));
}
pub unsafe fn makeTypeIDImp_2() -> *const TypeID {
    return &(*std::cell::LazyCell::force_mut(&mut *&raw mut id_3));
}
pub unsafe fn makeTypeIDImp_4() -> *const TypeID {
    return &(*std::cell::LazyCell::force_mut(&mut *&raw mut id_5));
}
pub unsafe fn makeTypeIDImp_6() -> *const TypeID {
    return &(*std::cell::LazyCell::force_mut(&mut *&raw mut id_7));
}
pub unsafe fn makeTypeIDImp_8() -> *const TypeID {
    return &(*std::cell::LazyCell::force_mut(&mut *&raw mut id_9));
}
pub unsafe fn makeTypeIDImp_10() -> *const TypeID {
    return &(*std::cell::LazyCell::force_mut(&mut *&raw mut id_11));
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct TypeID {
    m_id: *const (),
}
impl TypeID {
    pub unsafe fn name(&self) -> () {
        return unimplemented!();
    }
    unsafe fn new(mut xid: *const ()) -> Self {
        let mut this = Self { m_id: xid };
        this
    }
}
impl std::cmp::PartialEq for TypeID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const TypeID, other as *const TypeID) }
    }
}
impl std::cmp::Eq for TypeID {}
static mut id_3: std::cell::LazyCell<TypeID> = std::cell::LazyCell::new(|| unsafe {
    TypeID::new({ c"const TypeID &makeTypeIDImp() [T = ArgumentListID<>]".as_ptr() })
});
static mut id_5: std::cell::LazyCell<TypeID> = std::cell::LazyCell::new(|| unsafe {
    TypeID::new({
        c"const TypeID &makeTypeIDImp() [T = ArgumentListID<const small_tracked_t &>]".as_ptr()
    })
});
static mut id_7: std::cell::LazyCell<TypeID> = std::cell::LazyCell::new(|| unsafe {
    TypeID::new({
        c"const TypeID &makeTypeIDImp() [T = ArgumentListID<small_tracked_t &&>]".as_ptr()
    })
});
static mut id_9: std::cell::LazyCell<TypeID> = std::cell::LazyCell::new(|| unsafe {
    TypeID::new({
        c"const TypeID &makeTypeIDImp() [T = ArgumentListID<const large_tracked_t &>]".as_ptr()
    })
});
static mut id_11: std::cell::LazyCell<TypeID> = std::cell::LazyCell::new(|| unsafe {
    TypeID::new({
        c"const TypeID &makeTypeIDImp() [T = ArgumentListID<large_tracked_t &&>]".as_ptr()
    })
});
pub unsafe fn makeArgumentID_12() -> *const TypeID {
    return (unsafe { makeTypeIDImp_2() });
}
pub unsafe fn makeArgumentID_13() -> *const TypeID {
    return (unsafe { makeTypeIDImp_4() });
}
pub unsafe fn makeArgumentID_14() -> *const TypeID {
    return (unsafe { makeTypeIDImp_6() });
}
pub unsafe fn makeArgumentID_15() -> *const TypeID {
    return (unsafe { makeTypeIDImp_8() });
}
pub unsafe fn makeArgumentID_16() -> *const TypeID {
    return (unsafe { makeTypeIDImp_10() });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IsSmallObject_small_tracked_t_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IsSmallObject_large_tracked_t_ {}
pub unsafe fn containsType_17(a: *const ()) -> () {
    return unimplemented!();
}
pub unsafe fn containsType_18(a: *const ()) -> () {
    return unimplemented!();
}
pub unsafe fn containsType_19(a: *const ()) -> () {
    return unimplemented!();
}
pub unsafe fn containsType_20(a: *const ()) -> () {
    return unimplemented!();
}
pub unsafe fn containsType_21(a: *const ()) -> () {
    return unimplemented!();
}
pub unsafe fn containsType_22(a: *const ()) -> () {
    return unimplemented!();
}
pub unsafe fn assertEmpty_23(a: *const ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn has_value_member_24(mut _a0: ()) -> () {
    return true;
}
pub unsafe fn has_value_member_25(mut _a0: ()) -> () {
    return true;
}
pub unsafe fn has_value_member_26(mut _a0: ()) -> () {
    return true;
}
pub unsafe fn has_value_member_27(mut _a0: ()) -> () {
    return true;
}
pub unsafe fn has_value_member_28(mut _a0: ()) -> () {
    return true;
}
pub unsafe fn has_value_member_29(mut _a0: ()) -> () {
    return true;
}
pub unsafe fn assertContains_30(a: *const (), mut value: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn assertContains_31(a: *const (), mut value: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn assertContains_32(a: *const (), mut value: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn assertContains_33(a: *const (), mut value: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn assertContains_34(a: *const (), mut value: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn assertContains_35(a: *const (), mut value: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub static mut count_36: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copied_37: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moved_38: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut const_copied_39: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut non_const_copied_40: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe, Default)]
pub struct small_type_0_ {
    pub value: (),
}
impl small_type_0_ {
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_37)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_38)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut const_copied_39)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut non_const_copied_40)) = 0_();
    }
    pub unsafe fn new_1(mut val: ()) -> Self {
        let mut this = Self { value: val };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_36)).prefix_inc();
        this
    }
    pub unsafe fn new_2(other: *const small_type_0_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        this.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_36)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_37)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut const_copied_39)).prefix_inc();
        this
    }
    pub unsafe fn move_from(other: *mut small_type_0_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        this.value = (*other).value;
        (*other).value = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_36)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_38)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.value = -1_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_36)).prefix_dec();
    }
}
impl Clone for small_type_0_ {
    fn clone(&self) -> Self {
        unsafe { small_type_0_::new_2(self as *const small_type_0_) }
    }
}
pub static mut count_41: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copied_42: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moved_43: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut const_copied_44: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe, Default)]
pub struct small_type_1_ {
    pub value: (),
}
impl small_type_1_ {
    pub unsafe fn new_1(mut val: ()) -> Self {
        let mut this = Self { value: val };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_41)).prefix_inc();
        this
    }
    pub unsafe fn new_2(other: *const small_type_1_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        this.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_41)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_42)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut const_copied_44)).prefix_inc();
        this
    }
    pub unsafe fn move_from(other: *mut small_type_1_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        this.value = (*other).value;
        (*other).value = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_41)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_43)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.value = -1_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_41)).prefix_dec();
    }
}
impl Clone for small_type_1_ {
    fn clone(&self) -> Self {
        unsafe { small_type_1_::new_2(self as *const small_type_1_) }
    }
}
pub static mut count_46: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copied_47: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moved_48: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut const_copied_49: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe, Default)]
pub struct small_type_2_ {
    pub value: (),
}
impl small_type_2_ {
    pub unsafe fn new_1(mut val: ()) -> Self {
        let mut this = Self { value: val };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_46)).prefix_inc();
        this
    }
    pub unsafe fn new_2(other: *const small_type_2_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        this.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_46)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_47)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut const_copied_49)).prefix_inc();
        this
    }
    pub unsafe fn move_from(other: *mut small_type_2_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        this.value = (*other).value;
        (*other).value = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_46)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_48)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.value = -1_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_46)).prefix_dec();
    }
}
impl Clone for small_type_2_ {
    fn clone(&self) -> Self {
        unsafe { small_type_2_::new_2(self as *const small_type_2_) }
    }
}
pub static mut count_51: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copied_52: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moved_53: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut const_copied_54: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut non_const_copied_55: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe, Default)]
pub struct large_type_0_ {
    pub value: (),
    data: [(); 10],
}
impl large_type_0_ {
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_52)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_53)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut const_copied_54)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut non_const_copied_55)) = 0_();
    }
    pub unsafe fn new_1(mut val: ()) -> Self {
        let mut this = Self {
            value: val,
            data: [unimplemented!(); 10],
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_51)).prefix_inc();
        this.data[(0_()) as usize] = 0_();
        this
    }
    pub unsafe fn new_2(other: *const large_type_0_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
            data: [unimplemented!(); 10],
        };
        this.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_51)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_52)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut const_copied_54)).prefix_inc();
        this
    }
    pub unsafe fn move_from(other: *mut large_type_0_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
            data: [unimplemented!(); 10],
        };
        this.value = (*other).value;
        (*other).value = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_51)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_53)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.value = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_51)).prefix_dec();
    }
}
impl Clone for large_type_0_ {
    fn clone(&self) -> Self {
        unsafe { large_type_0_::new_2(self as *const large_type_0_) }
    }
}
pub static mut count_56: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copied_57: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moved_58: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut const_copied_59: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe, Default)]
pub struct large_type_1_ {
    pub value: (),
    data: [(); 10],
}
impl large_type_1_ {
    pub unsafe fn new_1(mut val: ()) -> Self {
        let mut this = Self {
            value: val,
            data: [unimplemented!(); 10],
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_56)).prefix_inc();
        this.data[(0_()) as usize] = 0_();
        this
    }
    pub unsafe fn new_2(other: *const large_type_1_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
            data: [unimplemented!(); 10],
        };
        this.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_56)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_57)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut const_copied_59)).prefix_inc();
        this
    }
    pub unsafe fn move_from(other: *mut large_type_1_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
            data: [unimplemented!(); 10],
        };
        this.value = (*other).value;
        (*other).value = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_56)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_58)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.value = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_56)).prefix_dec();
    }
}
impl Clone for large_type_1_ {
    fn clone(&self) -> Self {
        unsafe { large_type_1_::new_2(self as *const large_type_1_) }
    }
}
pub static mut count_61: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copied_62: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moved_63: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut const_copied_64: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe, Default)]
pub struct large_type_2_ {
    pub value: (),
    data: [(); 10],
}
impl large_type_2_ {
    pub unsafe fn new_1(mut val: ()) -> Self {
        let mut this = Self {
            value: val,
            data: [unimplemented!(); 10],
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_61)).prefix_inc();
        this.data[(0_()) as usize] = 0_();
        this
    }
    pub unsafe fn new_2(other: *const large_type_2_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
            data: [unimplemented!(); 10],
        };
        this.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_61)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_62)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut const_copied_64)).prefix_inc();
        this
    }
    pub unsafe fn move_from(other: *mut large_type_2_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
            data: [unimplemented!(); 10],
        };
        this.value = (*other).value;
        (*other).value = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_61)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_63)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.value = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_61)).prefix_dec();
    }
}
impl Clone for large_type_2_ {
    fn clone(&self) -> Self {
        unsafe { large_type_2_::new_2(self as *const large_type_2_) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct my_any_exception {}
pub unsafe fn throwMyAnyExpression_66() {
    unimplemented!();
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct small_throws_on_copy {
    pub value: (),
}
impl small_throws_on_copy {
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_67)) = {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_68)) = {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_69)) = 0_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_69))
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_68))
        };
    }
    pub unsafe fn new_1(mut val: Option<()>) -> Self {
        let mut val: () = val.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { value: val };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_67)).prefix_inc();
        this
    }
    pub unsafe fn new_2(mut _a0: (), mut val: (), mut _a2: ()) -> Self {
        let mut this = Self { value: val };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_67)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(_a0: *const small_throws_on_copy) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        (unsafe { throwMyAnyExpression_66() });
        this
    }
    pub unsafe fn move_from(other: *mut small_throws_on_copy) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        this.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_67)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_69)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_67)).prefix_dec();
    }
}
impl Clone for small_throws_on_copy {
    fn clone(&self) -> Self {
        unsafe { small_throws_on_copy::copy_from(self as *const small_throws_on_copy) }
    }
}
impl Default for small_throws_on_copy {
    fn default() -> Self {
        unsafe { small_throws_on_copy::new_1(None) }
    }
}
pub static mut count_67: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copied_68: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moved_69: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct large_throws_on_copy {
    pub value: (),
    data: [(); 10],
}
impl large_throws_on_copy {
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_70)) = {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_71)) = {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_72)) = 0_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_72))
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_71))
        };
    }
    pub unsafe fn new_1(mut val: Option<()>) -> Self {
        let mut val: () = val.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self {
            value: val,
            data: [unimplemented!(); 10],
        };
        this.data[(0_()) as usize] = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_70)).prefix_inc();
        this
    }
    pub unsafe fn new_2(mut _a0: (), mut val: (), mut _a2: ()) -> Self {
        let mut this = Self {
            value: val,
            data: [unimplemented!(); 10],
        };
        this.data[(0_()) as usize] = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_70)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(_a0: *const large_throws_on_copy) -> Self {
        let mut this = Self {
            value: 0_(),
            data: [unimplemented!(); 10],
        };
        (unsafe { throwMyAnyExpression_66() });
        this
    }
    pub unsafe fn move_from(other: *mut large_throws_on_copy) -> Self {
        let mut this = Self {
            value: 0_(),
            data: [unimplemented!(); 10],
        };
        this.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_70)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_72)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_70)).prefix_dec();
    }
}
impl Clone for large_throws_on_copy {
    fn clone(&self) -> Self {
        unsafe { large_throws_on_copy::copy_from(self as *const large_throws_on_copy) }
    }
}
impl Default for large_throws_on_copy {
    fn default() -> Self {
        unsafe { large_throws_on_copy::new_1(None) }
    }
}
pub static mut count_70: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copied_71: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moved_72: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct throws_on_move {
    pub value: (),
}
impl throws_on_move {
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_73)) = {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_74)) = {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_75)) = 0_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_75))
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_74))
        };
    }
    pub unsafe fn new_1(mut val: Option<()>) -> Self {
        let mut val: () = val.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { value: val };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_73)).prefix_inc();
        this
    }
    pub unsafe fn new_2(mut _a0: (), mut val: (), mut _a2: ()) -> Self {
        let mut this = Self { value: val };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_73)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(other: *const throws_on_move) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        this.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_73)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_74)).prefix_inc();
        this
    }
    pub unsafe fn move_from(_a0: *mut throws_on_move) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        (unsafe { throwMyAnyExpression_66() });
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_73)).prefix_dec();
    }
}
impl Clone for throws_on_move {
    fn clone(&self) -> Self {
        unsafe { throws_on_move::copy_from(self as *const throws_on_move) }
    }
}
impl Default for throws_on_move {
    fn default() -> Self {
        unsafe { throws_on_move::new_1(None) }
    }
}
pub static mut count_73: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copied_74: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moved_75: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct small_tracked_t {
    pub arg_types: *const TypeID,
}
impl small_tracked_t {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            arg_types: (unsafe { makeArgumentID_12() }),
        };
        this
    }
    pub unsafe fn copy_from(_a0: *const small_tracked_t) -> Self {
        let mut this = Self {
            arg_types: (unsafe { makeArgumentID_13() }),
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut small_tracked_t) -> Self {
        let mut this = Self {
            arg_types: (unsafe { makeArgumentID_14() }),
        };
        this
    }
}
impl Clone for small_tracked_t {
    fn clone(&self) -> Self {
        unsafe { small_tracked_t::copy_from(self as *const small_tracked_t) }
    }
}
impl Default for small_tracked_t {
    fn default() -> Self {
        unsafe { small_tracked_t::new() }
    }
}
const _: () = assert!(true, "IsSmallObject<small_tracked_t>::value");
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct large_tracked_t {
    pub arg_types: *const TypeID,
    pub dummy: [(); 9],
}
impl large_tracked_t {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            arg_types: (unsafe { makeArgumentID_12() }),
            dummy: [unimplemented!(); 9],
        };
        this.dummy[(0_()) as usize] = 42_();
        this
    }
    pub unsafe fn copy_from(_a0: *const large_tracked_t) -> Self {
        let mut this = Self {
            arg_types: (unsafe { makeArgumentID_15() }),
            dummy: [unimplemented!(); 9],
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut large_tracked_t) -> Self {
        let mut this = Self {
            arg_types: (unsafe { makeArgumentID_16() }),
            dummy: [unimplemented!(); 9],
        };
        this
    }
}
impl Clone for large_tracked_t {
    fn clone(&self) -> Self {
        unsafe { large_tracked_t::copy_from(self as *const large_tracked_t) }
    }
}
impl Default for large_tracked_t {
    fn default() -> Self {
        unsafe { large_tracked_t::new() }
    }
}
const _: () = assert!(true, "!IsSmallObject<large_tracked_t>::value");
pub unsafe fn test_swap_76() {
    unimplemented!();
    unimplemented!();
    {
        let mut a1: () = unimplemented!();
        let mut a2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { assertContains_32(&a1, 2_()) });
        (unsafe { assertContains_33(&a2, 1_()) });
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_swap_77() {
    unimplemented!();
    unimplemented!();
    {
        let mut a1: () = unimplemented!();
        let mut a2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { assertContains_34(&a1, 2_()) });
        (unsafe { assertContains_35(&a2, 1_()) });
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_swap_78() {
    unimplemented!();
    unimplemented!();
    {
        let mut a1: () = unimplemented!();
        let mut a2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { assertContains_31(&a1, 2_()) });
        (unsafe { assertContains_30(&a2, 1_()) });
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_swap_79() {
    unimplemented!();
    unimplemented!();
    {
        let mut a1: () = unimplemented!();
        let mut a2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { assertContains_30(&a1, 2_()) });
        (unsafe { assertContains_31(&a2, 1_()) });
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_swap_empty_80() {
    unimplemented!();
    {
        let mut a1: () = unimplemented!();
        let mut a2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { assertContains_30(&a2, 1_()) });
        (unsafe { assertEmpty_23(&a1) });
    }
    unimplemented!();
    {
        let mut a1: () = unimplemented!();
        let mut a2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { assertContains_30(&a2, 1_()) });
        (unsafe { assertEmpty_23(&a1) });
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_swap_empty_81() {
    unimplemented!();
    {
        let mut a1: () = unimplemented!();
        let mut a2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { assertContains_31(&a2, 1_()) });
        (unsafe { assertEmpty_23(&a1) });
    }
    unimplemented!();
    {
        let mut a1: () = unimplemented!();
        let mut a2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { assertContains_31(&a2, 1_()) });
        (unsafe { assertEmpty_23(&a1) });
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_noexcept_82() {
    let mut a1: () = unimplemented!();
    let mut a2: () = unimplemented!();
    const _: () = assert!(true, "");;
}
pub unsafe fn test_self_swap_83() {
    {
        let mut a: () = unimplemented!();
        unimplemented!();
        (unsafe { assertEmpty_23(&a) });
    }
    {
        let mut a: () = unimplemented!();
        (unsafe { small_type_0_::reset() });
        unimplemented!();
        (unsafe { assertContains_30(&a, 42_()) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    {
        let mut a: () = unimplemented!();
        (unsafe { large_type_0_::reset() });
        unimplemented!();
        (unsafe { assertContains_31(&a, 42_()) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
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
    (unsafe { test_noexcept_82() });
    (unsafe { test_swap_empty_80() });
    (unsafe { test_swap_empty_81() });
    (unsafe { test_swap_76() });
    (unsafe { test_swap_77() });
    (unsafe { test_swap_78() });
    (unsafe { test_swap_79() });
    (unsafe { test_self_swap_83() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_36);
    std::cell::LazyCell::force(&*&raw const copied_37);
    std::cell::LazyCell::force(&*&raw const moved_38);
    std::cell::LazyCell::force(&*&raw const const_copied_39);
    std::cell::LazyCell::force(&*&raw const non_const_copied_40);
    std::cell::LazyCell::force(&*&raw const count_41);
    std::cell::LazyCell::force(&*&raw const copied_42);
    std::cell::LazyCell::force(&*&raw const moved_43);
    std::cell::LazyCell::force(&*&raw const const_copied_44);
    std::cell::LazyCell::force(&*&raw const count_46);
    std::cell::LazyCell::force(&*&raw const copied_47);
    std::cell::LazyCell::force(&*&raw const moved_48);
    std::cell::LazyCell::force(&*&raw const const_copied_49);
    std::cell::LazyCell::force(&*&raw const count_51);
    std::cell::LazyCell::force(&*&raw const copied_52);
    std::cell::LazyCell::force(&*&raw const moved_53);
    std::cell::LazyCell::force(&*&raw const const_copied_54);
    std::cell::LazyCell::force(&*&raw const non_const_copied_55);
    std::cell::LazyCell::force(&*&raw const count_56);
    std::cell::LazyCell::force(&*&raw const copied_57);
    std::cell::LazyCell::force(&*&raw const moved_58);
    std::cell::LazyCell::force(&*&raw const const_copied_59);
    std::cell::LazyCell::force(&*&raw const count_61);
    std::cell::LazyCell::force(&*&raw const copied_62);
    std::cell::LazyCell::force(&*&raw const moved_63);
    std::cell::LazyCell::force(&*&raw const const_copied_64);
    std::cell::LazyCell::force(&*&raw const count_67);
    std::cell::LazyCell::force(&*&raw const copied_68);
    std::cell::LazyCell::force(&*&raw const moved_69);
    std::cell::LazyCell::force(&*&raw const count_70);
    std::cell::LazyCell::force(&*&raw const copied_71);
    std::cell::LazyCell::force(&*&raw const moved_72);
    std::cell::LazyCell::force(&*&raw const count_73);
    std::cell::LazyCell::force(&*&raw const copied_74);
    std::cell::LazyCell::force(&*&raw const moved_75);
}
