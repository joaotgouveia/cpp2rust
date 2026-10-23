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
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
pub unsafe fn operator_eq_2(LHS: *const TypeID, RHS: *const TypeID) -> () {
    return (((*LHS).m_id) == ((*RHS).m_id));
}
pub unsafe fn operator_ne_3(LHS: *const TypeID, RHS: *const TypeID) -> () {
    return (((*LHS).m_id) != ((*RHS).m_id));
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
pub unsafe fn makeTypeIDImp_12() -> *const TypeID {
    return &(*std::cell::LazyCell::force_mut(&mut *&raw mut id_13));
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct TypeID {
    m_id: *const (),
}
impl TypeID {
    pub unsafe fn name(&self) -> () {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { self.m_id },
            None,
        );
    }
    unsafe fn new(mut xid: *const ()) -> Self {
        let mut this = Self { m_id: xid };
        this
    }
}
impl std::cmp::PartialEq for TypeID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_2(self as *const TypeID, other as *const TypeID) }
    }
}
impl std::cmp::Eq for TypeID {}
static mut id_5: std::cell::LazyCell<TypeID> = std::cell::LazyCell::new(|| unsafe {
    TypeID::new({ c"const TypeID &makeTypeIDImp() [T = ArgumentListID<>]".as_ptr() })
});
static mut id_7: std::cell::LazyCell<TypeID> = std::cell::LazyCell::new(|| unsafe {
    TypeID::new({
        c"const TypeID &makeTypeIDImp() [T = ArgumentListID<const small_tracked_t &>]".as_ptr()
    })
});
static mut id_9: std::cell::LazyCell<TypeID> = std::cell::LazyCell::new(|| unsafe {
    TypeID::new({
        c"const TypeID &makeTypeIDImp() [T = ArgumentListID<small_tracked_t &&>]".as_ptr()
    })
});
static mut id_11: std::cell::LazyCell<TypeID> = std::cell::LazyCell::new(|| unsafe {
    TypeID::new({
        c"const TypeID &makeTypeIDImp() [T = ArgumentListID<const large_tracked_t &>]".as_ptr()
    })
});
static mut id_13: std::cell::LazyCell<TypeID> = std::cell::LazyCell::new(|| unsafe {
    TypeID::new({
        c"const TypeID &makeTypeIDImp() [T = ArgumentListID<large_tracked_t &&>]".as_ptr()
    })
});
pub unsafe fn makeArgumentID_14() -> *const TypeID {
    return (unsafe { makeTypeIDImp_4() });
}
pub unsafe fn makeArgumentID_15() -> *const TypeID {
    return (unsafe { makeTypeIDImp_6() });
}
pub unsafe fn makeArgumentID_16() -> *const TypeID {
    return (unsafe { makeTypeIDImp_8() });
}
pub unsafe fn makeArgumentID_17() -> *const TypeID {
    return (unsafe { makeTypeIDImp_10() });
}
pub unsafe fn makeArgumentID_18() -> *const TypeID {
    return (unsafe { makeTypeIDImp_12() });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IsSmallObject_small_tracked_t_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IsSmallObject_large_tracked_t_ {}
pub static mut count_19: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copied_20: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moved_21: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut const_copied_22: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut non_const_copied_23: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe, Default)]
pub struct small_type_0_ {
    pub value: (),
}
impl small_type_0_ {
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_20)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_21)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut const_copied_22)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut non_const_copied_23)) = 0_();
    }
    pub unsafe fn new_1(mut val: ()) -> Self {
        let mut this = Self { value: val };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_19)).prefix_inc();
        this
    }
    pub unsafe fn new_2(other: *const small_type_0_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        this.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_19)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_20)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut const_copied_22)).prefix_inc();
        this
    }
    pub unsafe fn move_from(other: *mut small_type_0_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        this.value = (*other).value;
        (*other).value = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_19)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_21)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.value = -1_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_19)).prefix_dec();
    }
}
impl Clone for small_type_0_ {
    fn clone(&self) -> Self {
        unsafe { small_type_0_::new_2(self as *const small_type_0_) }
    }
}
pub static mut count_24: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copied_25: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moved_26: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut const_copied_27: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut non_const_copied_28: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe, Default)]
pub struct large_type_0_ {
    pub value: (),
    data: [(); 10],
}
impl large_type_0_ {
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_25)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_26)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut const_copied_27)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut non_const_copied_28)) = 0_();
    }
    pub unsafe fn new_1(mut val: ()) -> Self {
        let mut this = Self {
            value: val,
            data: [unimplemented!(); 10],
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_24)).prefix_inc();
        this.data[(0_()) as usize] = 0_();
        this
    }
    pub unsafe fn new_2(other: *const large_type_0_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
            data: [unimplemented!(); 10],
        };
        this.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_24)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_25)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut const_copied_27)).prefix_inc();
        this
    }
    pub unsafe fn move_from(other: *mut large_type_0_) -> Self {
        let mut this = Self {
            value: unimplemented!(),
            data: [unimplemented!(); 10],
        };
        this.value = (*other).value;
        (*other).value = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_24)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_26)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.value = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_24)).prefix_dec();
    }
}
impl Clone for large_type_0_ {
    fn clone(&self) -> Self {
        unsafe { large_type_0_::new_2(self as *const large_type_0_) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct my_any_exception {}
pub unsafe fn throwMyAnyExpression_29() {
    unimplemented!();
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct small_throws_on_copy {
    pub value: (),
}
impl small_throws_on_copy {
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_30)) = {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_31)) = {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_32)) = 0_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_32))
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_31))
        };
    }
    pub unsafe fn new_1(mut val: Option<()>) -> Self {
        let mut val: () = val.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { value: val };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_30)).prefix_inc();
        this
    }
    pub unsafe fn new_2(mut _a0: (), mut val: (), mut _a2: ()) -> Self {
        let mut this = Self { value: val };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_30)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(_a0: *const small_throws_on_copy) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        (unsafe { throwMyAnyExpression_29() });
        this
    }
    pub unsafe fn move_from(other: *mut small_throws_on_copy) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        this.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_30)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_32)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_30)).prefix_dec();
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
pub static mut count_30: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copied_31: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moved_32: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct large_throws_on_copy {
    pub value: (),
    data: [(); 10],
}
impl large_throws_on_copy {
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_33)) = {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_34)) = {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_35)) = 0_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_35))
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_34))
        };
    }
    pub unsafe fn new_1(mut val: Option<()>) -> Self {
        let mut val: () = val.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self {
            value: val,
            data: [unimplemented!(); 10],
        };
        this.data[(0_()) as usize] = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_33)).prefix_inc();
        this
    }
    pub unsafe fn new_2(mut _a0: (), mut val: (), mut _a2: ()) -> Self {
        let mut this = Self {
            value: val,
            data: [unimplemented!(); 10],
        };
        this.data[(0_()) as usize] = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_33)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(_a0: *const large_throws_on_copy) -> Self {
        let mut this = Self {
            value: 0_(),
            data: [unimplemented!(); 10],
        };
        (unsafe { throwMyAnyExpression_29() });
        this
    }
    pub unsafe fn move_from(other: *mut large_throws_on_copy) -> Self {
        let mut this = Self {
            value: 0_(),
            data: [unimplemented!(); 10],
        };
        this.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_33)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_35)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_33)).prefix_dec();
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
pub static mut count_33: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copied_34: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moved_35: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct throws_on_move {
    pub value: (),
}
impl throws_on_move {
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_36)) = {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_37)) = {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_38)) = 0_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_38))
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_37))
        };
    }
    pub unsafe fn new_1(mut val: Option<()>) -> Self {
        let mut val: () = val.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { value: val };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_36)).prefix_inc();
        this
    }
    pub unsafe fn new_2(mut _a0: (), mut val: (), mut _a2: ()) -> Self {
        let mut this = Self { value: val };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_36)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(other: *const throws_on_move) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        this.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_36)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_37)).prefix_inc();
        this
    }
    pub unsafe fn move_from(_a0: *mut throws_on_move) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        (unsafe { throwMyAnyExpression_29() });
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_36)).prefix_dec();
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
pub static mut count_36: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copied_37: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moved_38: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct small_tracked_t {
    pub arg_types: *const TypeID,
}
impl small_tracked_t {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            arg_types: (unsafe { makeArgumentID_14() }),
        };
        this
    }
    pub unsafe fn copy_from(_a0: *const small_tracked_t) -> Self {
        let mut this = Self {
            arg_types: (unsafe { makeArgumentID_15() }),
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut small_tracked_t) -> Self {
        let mut this = Self {
            arg_types: (unsafe { makeArgumentID_16() }),
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
            arg_types: (unsafe { makeArgumentID_14() }),
            dummy: [unimplemented!(); 9],
        };
        this.dummy[(0_()) as usize] = 42_();
        this
    }
    pub unsafe fn copy_from(_a0: *const large_tracked_t) -> Self {
        let mut this = Self {
            arg_types: (unsafe { makeArgumentID_17() }),
            dummy: [unimplemented!(); 9],
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut large_tracked_t) -> Self {
        let mut this = Self {
            arg_types: (unsafe { makeArgumentID_18() }),
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
pub unsafe fn test_cast_is_noexcept_39() {
    let mut a: () = unimplemented!();
    const _: () = assert!(true, "");;
    let ca: *const () = &a;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_cast_return_type_40() {
    let mut a: () = unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let ca: *const () = &a;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_cast_nullptr_41() {
    let mut a: *mut () = std::ptr::null_mut();
    unimplemented!();
    unimplemented!();
    let mut ca: *const () = std::ptr::null();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cast_empty_42() {
    {
        let mut a: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let ca: *const () = &a;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a: () = 42_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let ca: *const () = &a;
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_cast_43() {
    unimplemented!();
    (unsafe { small_type_0_::reset() });
    {
        let mut a: () = small_type_0_::new_1({ 42_() });
        let ca: *const () = &a;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut v: *mut small_type_0_ = unimplemented!();
        unimplemented!();
        unimplemented!();
        (*v).value = 999_();
        let mut cv: *const small_type_0_ = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        cv = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        cv = (unsafe { any_cast_44((ca)) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
}
pub unsafe fn test_cast_45() {
    unimplemented!();
    (unsafe { large_type_0_::reset() });
    {
        let mut a: () = large_type_0_::new_1({ 42_() });
        let ca: *const () = &a;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut v: *mut large_type_0_ = unimplemented!();
        unimplemented!();
        unimplemented!();
        (*v).value = 999_();
        let mut cv: *const large_type_0_ = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        cv = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        cv = (unsafe { any_cast_46((ca)) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
}
pub unsafe fn test_cast_non_copyable_type_47() {
    let mut a: () = 42_();
    let ca: *const () = &a;
    unimplemented!();
    unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct NoCopy_48 {}
pub unsafe fn test_cast_array_49() {
    let mut arr: [(); 3] = [unimplemented!(); 3];
    let mut a: () = std_any::new_1({ &mut arr });
    unimplemented!();
    let mut p: *mut [(); 3] = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_fn_50() {}
pub unsafe fn test_cast_function_pointer_51() {
    let mut a: () = std_any::new_2({ Some(test_fn_50) });
    unimplemented!();
    let mut fn_ptr: Option<unsafe fn()> = unimplemented!();
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
    (unsafe { test_cast_is_noexcept_39() });
    (unsafe { test_cast_return_type_40() });
    (unsafe { test_cast_nullptr_41() });
    (unsafe { test_cast_empty_42() });
    (unsafe { test_cast_43() });
    (unsafe { test_cast_45() });
    (unsafe { test_cast_non_copyable_type_47() });
    (unsafe { test_cast_array_49() });
    (unsafe { test_cast_function_pointer_51() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_19);
    std::cell::LazyCell::force(&*&raw const copied_20);
    std::cell::LazyCell::force(&*&raw const moved_21);
    std::cell::LazyCell::force(&*&raw const const_copied_22);
    std::cell::LazyCell::force(&*&raw const non_const_copied_23);
    std::cell::LazyCell::force(&*&raw const count_24);
    std::cell::LazyCell::force(&*&raw const copied_25);
    std::cell::LazyCell::force(&*&raw const moved_26);
    std::cell::LazyCell::force(&*&raw const const_copied_27);
    std::cell::LazyCell::force(&*&raw const non_const_copied_28);
    std::cell::LazyCell::force(&*&raw const count_30);
    std::cell::LazyCell::force(&*&raw const copied_31);
    std::cell::LazyCell::force(&*&raw const moved_32);
    std::cell::LazyCell::force(&*&raw const count_33);
    std::cell::LazyCell::force(&*&raw const copied_34);
    std::cell::LazyCell::force(&*&raw const moved_35);
    std::cell::LazyCell::force(&*&raw const count_36);
    std::cell::LazyCell::force(&*&raw const copied_37);
    std::cell::LazyCell::force(&*&raw const moved_38);
}
