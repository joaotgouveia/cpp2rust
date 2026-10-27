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
pub unsafe fn assertEmpty_21(a: *const ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn assertEmpty_22(a: *const ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
pub unsafe fn assertContains_28(a: *const (), mut value: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn assertContains_29(a: *const (), mut value: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
pub static mut count_32: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copied_33: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moved_34: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut const_copied_35: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe, Default)]
pub struct small_type_0_ {
    pub value: (),
}
impl small_type_0_ {
    pub unsafe fn new_1(mut val: ()) -> Self {
        let mut this = Self { value: val };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_32)).prefix_inc();
        this
    }
    pub unsafe fn new_2(other: *const small_type_0_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        this.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_32)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_33)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut const_copied_35)).prefix_inc();
        this
    }
    pub unsafe fn move_from(other: *mut small_type_0_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        this.value = (*other).value;
        (*other).value = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_32)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_34)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.value = -1_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_32)).prefix_dec();
    }
}
impl Clone for small_type_0_ {
    fn clone(&self) -> Self {
        unsafe { small_type_0_::new_2(self as *const small_type_0_) }
    }
}
pub static mut count_37: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copied_38: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moved_39: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut const_copied_40: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe, Default)]
pub struct small_type_1_ {
    pub value: (),
}
impl small_type_1_ {
    pub unsafe fn new_1(mut val: ()) -> Self {
        let mut this = Self { value: val };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_37)).prefix_inc();
        this
    }
    pub unsafe fn new_2(other: *const small_type_1_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        this.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_37)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_38)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut const_copied_40)).prefix_inc();
        this
    }
    pub unsafe fn move_from(other: *mut small_type_1_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        this.value = (*other).value;
        (*other).value = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_37)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_39)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.value = -1_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_37)).prefix_dec();
    }
}
impl Clone for small_type_1_ {
    fn clone(&self) -> Self {
        unsafe { small_type_1_::new_2(self as *const small_type_1_) }
    }
}
pub static mut count_42: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copied_43: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moved_44: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut const_copied_45: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe, Default)]
pub struct small_type_2_ {
    pub value: (),
}
impl small_type_2_ {
    pub unsafe fn new_1(mut val: ()) -> Self {
        let mut this = Self { value: val };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_42)).prefix_inc();
        this
    }
    pub unsafe fn new_2(other: *const small_type_2_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        this.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_42)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_43)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut const_copied_45)).prefix_inc();
        this
    }
    pub unsafe fn move_from(other: *mut small_type_2_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        this.value = (*other).value;
        (*other).value = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_42)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_44)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.value = -1_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_42)).prefix_dec();
    }
}
impl Clone for small_type_2_ {
    fn clone(&self) -> Self {
        unsafe { small_type_2_::new_2(self as *const small_type_2_) }
    }
}
pub static mut count_47: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copied_48: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moved_49: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut const_copied_50: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe, Default)]
pub struct large_type_0_ {
    pub value: (),
    data: [(); 10],
}
impl large_type_0_ {
    pub unsafe fn new_1(mut val: ()) -> Self {
        let mut this = Self {
            value: val,
            data: [unimplemented!(); 10],
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_47)).prefix_inc();
        this.data[(0_()) as usize] = 0_();
        this
    }
    pub unsafe fn new_2(other: *const large_type_0_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
            data: [unimplemented!(); 10],
        };
        this.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_47)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_48)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut const_copied_50)).prefix_inc();
        this
    }
    pub unsafe fn move_from(other: *mut large_type_0_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
            data: [unimplemented!(); 10],
        };
        this.value = (*other).value;
        (*other).value = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_47)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_49)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.value = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_47)).prefix_dec();
    }
}
impl Clone for large_type_0_ {
    fn clone(&self) -> Self {
        unsafe { large_type_0_::new_2(self as *const large_type_0_) }
    }
}
pub static mut count_52: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copied_53: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut const_copied_55: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe, Default)]
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
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_52)).prefix_inc();
        this.data[(0_()) as usize] = 0_();
        this
    }
    pub unsafe fn new_2(other: *const large_type_1_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
            data: [unimplemented!(); 10],
        };
        this.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_52)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_53)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut const_copied_55)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.value = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_52)).prefix_dec();
    }
}
impl Clone for large_type_1_ {
    fn clone(&self) -> Self {
        unsafe { large_type_1_::new_2(self as *const large_type_1_) }
    }
}
pub static mut count_57: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copied_58: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut const_copied_60: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe, Default)]
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
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_57)).prefix_inc();
        this.data[(0_()) as usize] = 0_();
        this
    }
    pub unsafe fn new_2(other: *const large_type_2_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
            data: [unimplemented!(); 10],
        };
        this.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_57)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_58)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut const_copied_60)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.value = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_57)).prefix_dec();
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
pub unsafe fn throwMyAnyExpression_62() {
    unimplemented!();
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct small_throws_on_copy {
    pub value: (),
}
impl small_throws_on_copy {
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_63)) = {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_64)) = {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_65)) = 0_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_65))
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_64))
        };
    }
    pub unsafe fn new_1(mut val: Option<()>) -> Self {
        let mut val: () = val.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { value: val };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_63)).prefix_inc();
        this
    }
    pub unsafe fn new_2(mut _a0: (), mut val: (), mut _a2: ()) -> Self {
        let mut this = Self { value: val };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_63)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(_a0: *const small_throws_on_copy) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        (unsafe { throwMyAnyExpression_62() });
        this
    }
    pub unsafe fn move_from(other: *mut small_throws_on_copy) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        this.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_63)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_65)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_63)).prefix_dec();
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
pub static mut count_63: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copied_64: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moved_65: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct large_throws_on_copy {
    pub value: (),
    data: [(); 10],
}
impl large_throws_on_copy {
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_66)) = {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_67)) = {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_68)) = 0_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_68))
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_67))
        };
    }
    pub unsafe fn new_1(mut val: Option<()>) -> Self {
        let mut val: () = val.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self {
            value: val,
            data: [unimplemented!(); 10],
        };
        this.data[(0_()) as usize] = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_66)).prefix_inc();
        this
    }
    pub unsafe fn new_2(mut _a0: (), mut val: (), mut _a2: ()) -> Self {
        let mut this = Self {
            value: val,
            data: [unimplemented!(); 10],
        };
        this.data[(0_()) as usize] = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_66)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(_a0: *const large_throws_on_copy) -> Self {
        let mut this = Self {
            value: 0_(),
            data: [unimplemented!(); 10],
        };
        (unsafe { throwMyAnyExpression_62() });
        this
    }
    pub unsafe fn move_from(other: *mut large_throws_on_copy) -> Self {
        let mut this = Self {
            value: 0_(),
            data: [unimplemented!(); 10],
        };
        this.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_66)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_68)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_66)).prefix_dec();
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
pub static mut count_66: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copied_67: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moved_68: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct throws_on_move {
    pub value: (),
}
impl throws_on_move {
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_69)) = {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_70)) = {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_71)) = 0_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_71))
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_70))
        };
    }
    pub unsafe fn new_1(mut val: Option<()>) -> Self {
        let mut val: () = val.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { value: val };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_69)).prefix_inc();
        this
    }
    pub unsafe fn new_2(mut _a0: (), mut val: (), mut _a2: ()) -> Self {
        let mut this = Self { value: val };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_69)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(other: *const throws_on_move) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        this.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_69)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_70)).prefix_inc();
        this
    }
    pub unsafe fn move_from(_a0: *mut throws_on_move) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        (unsafe { throwMyAnyExpression_62() });
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_69)).prefix_dec();
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
pub static mut count_69: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copied_70: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moved_71: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
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
pub unsafe fn test_move_assign_72() {
    unimplemented!();
    unimplemented!();
    {
        let mut s1: small_type_1_ = small_type_1_::new_1({ 1_() });
        let _dtor_s1 = ScopedDestructorUnsafe::new(&raw mut s1, small_type_1_::destructor);
        let mut a: () = unimplemented!();
        let mut s2: small_type_2_ = small_type_2_::new_1({ 2_() });
        let _dtor_s2 = ScopedDestructorUnsafe::new(&raw mut s2, small_type_2_::destructor);
        let mut a2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { assertContains_28(&a, 2_()) });
        if unimplemented!() {
            (unsafe { assertContains_28(&a2, 0_()) });
        }
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_move_assign_73() {
    unimplemented!();
    unimplemented!();
    {
        let mut s1: large_type_1_ = large_type_1_::new_1({ 1_() });
        let _dtor_s1 = ScopedDestructorUnsafe::new(&raw mut s1, large_type_1_::destructor);
        let mut a: () = unimplemented!();
        let mut s2: large_type_2_ = large_type_2_::new_1({ 2_() });
        let _dtor_s2 = ScopedDestructorUnsafe::new(&raw mut s2, large_type_2_::destructor);
        let mut a2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { assertContains_29(&a, 2_()) });
        if unimplemented!() {
            (unsafe { assertContains_29(&a2, 0_()) });
        }
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_move_assign_74() {
    unimplemented!();
    unimplemented!();
    {
        let mut s1: small_type_0_ = small_type_0_::new_1({ 1_() });
        let _dtor_s1 = ScopedDestructorUnsafe::new(&raw mut s1, small_type_0_::destructor);
        let mut a: () = unimplemented!();
        let mut s2: large_type_0_ = large_type_0_::new_1({ 2_() });
        let _dtor_s2 = ScopedDestructorUnsafe::new(&raw mut s2, large_type_0_::destructor);
        let mut a2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { assertContains_30(&a, 2_()) });
        if unimplemented!() {
            (unsafe { assertContains_30(&a2, 0_()) });
        }
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_move_assign_75() {
    unimplemented!();
    unimplemented!();
    {
        let mut s1: large_type_0_ = large_type_0_::new_1({ 1_() });
        let _dtor_s1 = ScopedDestructorUnsafe::new(&raw mut s1, large_type_0_::destructor);
        let mut a: () = unimplemented!();
        let mut s2: small_type_0_ = small_type_0_::new_1({ 2_() });
        let _dtor_s2 = ScopedDestructorUnsafe::new(&raw mut s2, small_type_0_::destructor);
        let mut a2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { assertContains_31(&a, 2_()) });
        if unimplemented!() {
            (unsafe { assertContains_31(&a2, 0_()) });
        }
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_move_assign_empty_76() {
    unimplemented!();
    {
        let mut a: () = unimplemented!();
        let mut a2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { assertContains_31(&a, 1_()) });
        if unimplemented!() {
            (unsafe { assertContains_31(&a2, 0_()) });
        }
        unimplemented!();
    }
    unimplemented!();
    {
        let mut a: () = unimplemented!();
        let mut a2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { assertEmpty_22(&a) });
        (unsafe { assertEmpty_21(&a2) });
    }
    unimplemented!();
}
pub unsafe fn test_move_assign_empty_77() {
    unimplemented!();
    {
        let mut a: () = unimplemented!();
        let mut a2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { assertContains_30(&a, 1_()) });
        if unimplemented!() {
            (unsafe { assertContains_30(&a2, 0_()) });
        }
        unimplemented!();
    }
    unimplemented!();
    {
        let mut a: () = unimplemented!();
        let mut a2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { assertEmpty_23(&a) });
        (unsafe { assertEmpty_21(&a2) });
    }
    unimplemented!();
}
pub unsafe fn test_move_assign_noexcept_78() {
    let mut a1: () = unimplemented!();
    let mut a2: () = unimplemented!();
    const _: () = assert!(true, "");;
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
    (unsafe { test_move_assign_noexcept_78() });
    (unsafe { test_move_assign_72() });
    (unsafe { test_move_assign_73() });
    (unsafe { test_move_assign_74() });
    (unsafe { test_move_assign_75() });
    (unsafe { test_move_assign_empty_76() });
    (unsafe { test_move_assign_empty_77() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_32);
    std::cell::LazyCell::force(&*&raw const copied_33);
    std::cell::LazyCell::force(&*&raw const moved_34);
    std::cell::LazyCell::force(&*&raw const const_copied_35);
    std::cell::LazyCell::force(&*&raw const count_37);
    std::cell::LazyCell::force(&*&raw const copied_38);
    std::cell::LazyCell::force(&*&raw const moved_39);
    std::cell::LazyCell::force(&*&raw const const_copied_40);
    std::cell::LazyCell::force(&*&raw const count_42);
    std::cell::LazyCell::force(&*&raw const copied_43);
    std::cell::LazyCell::force(&*&raw const moved_44);
    std::cell::LazyCell::force(&*&raw const const_copied_45);
    std::cell::LazyCell::force(&*&raw const count_47);
    std::cell::LazyCell::force(&*&raw const copied_48);
    std::cell::LazyCell::force(&*&raw const moved_49);
    std::cell::LazyCell::force(&*&raw const const_copied_50);
    std::cell::LazyCell::force(&*&raw const count_52);
    std::cell::LazyCell::force(&*&raw const copied_53);
    std::cell::LazyCell::force(&*&raw const const_copied_55);
    std::cell::LazyCell::force(&*&raw const count_57);
    std::cell::LazyCell::force(&*&raw const copied_58);
    std::cell::LazyCell::force(&*&raw const const_copied_60);
    std::cell::LazyCell::force(&*&raw const count_63);
    std::cell::LazyCell::force(&*&raw const copied_64);
    std::cell::LazyCell::force(&*&raw const moved_65);
    std::cell::LazyCell::force(&*&raw const count_66);
    std::cell::LazyCell::force(&*&raw const copied_67);
    std::cell::LazyCell::force(&*&raw const moved_68);
    std::cell::LazyCell::force(&*&raw const count_69);
    std::cell::LazyCell::force(&*&raw const copied_70);
    std::cell::LazyCell::force(&*&raw const moved_71);
}
