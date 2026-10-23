extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct NoCopy {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CopyOnly {}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct MoveOnly {}
impl MoveOnly {
    pub unsafe fn move_from(_a0: *mut MoveOnly) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct MoveOnlyNT {}
impl MoveOnlyNT {
    pub unsafe fn move_from(_a0: *mut MoveOnlyNT) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(
    VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, DestructorUnsafe, Default,
)]
pub struct CopyAssign {
    pub value: (),
    pub alive: *mut (),
    pub copy_construct: *mut (),
    pub copy_assign: *mut (),
    pub move_construct: *mut (),
    pub move_assign: *mut (),
}
impl CopyAssign {
    pub unsafe fn new(
        mut v: (),
        mut alv: *mut (),
        mut cpy_ctr: *mut (),
        mut cpy_assi: *mut (),
        mut move_ctr: *mut (),
        mut move_assi: *mut (),
    ) -> Self {
        let mut this = Self {
            value: v,
            alive: alv,
            copy_construct: cpy_ctr,
            copy_assign: cpy_assi,
            move_construct: move_ctr,
            move_assign: move_assi,
        };
        (*this.alive).prefix_inc();
        this
    }
    pub unsafe fn copy_from(o: *const CopyAssign) -> Self {
        let mut this = Self {
            value: (*o).value,
            alive: (*o).alive,
            copy_construct: (*o).copy_construct,
            copy_assign: (*o).copy_assign,
            move_construct: (*o).move_construct,
            move_assign: (*o).move_assign,
        };
        (*this.alive).prefix_inc();
        (*this.copy_construct).prefix_inc();
        this
    }
    pub unsafe fn move_from(o: *mut CopyAssign) -> Self {
        let mut this = Self {
            value: (*o).value,
            alive: (*o).alive,
            copy_construct: (*o).copy_construct,
            copy_assign: (*o).copy_assign,
            move_construct: (*o).move_construct,
            move_assign: (*o).move_assign,
        };
        (*o).value = -1_();
        (*this.alive).prefix_inc();
        (*this.move_construct).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, o: *const CopyAssign) -> *mut CopyAssign {
        self.value = (*o).value;
        self.alive = (*o).alive;
        self.copy_construct = (*o).copy_construct;
        self.copy_assign = (*o).copy_assign;
        self.move_construct = (*o).move_construct;
        self.move_assign = (*o).move_assign;
        (*self.copy_assign).prefix_inc();
        return &mut (*(self as *mut CopyAssign));
    }
    pub unsafe fn move_assign(&mut self, o: *mut CopyAssign) -> *mut CopyAssign {
        self.value = (*o).value;
        self.alive = (*o).alive;
        self.copy_construct = (*o).copy_construct;
        self.copy_assign = (*o).copy_assign;
        self.move_construct = (*o).move_construct;
        self.move_assign = (*o).move_assign;
        (*o).value = -1_();
        (*self.move_assign).prefix_inc();
        return &mut (*(self as *mut CopyAssign));
    }
    pub unsafe fn destructor(&mut self) {
        (*self.alive).prefix_dec();
    }
}
impl Clone for CopyAssign {
    fn clone(&self) -> Self {
        unsafe { CopyAssign::copy_from(self as *const CopyAssign) }
    }
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct CopyMaybeThrows {}
impl CopyMaybeThrows {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct CopyDoesThrow {}
impl CopyDoesThrow {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct NTCopyAssign {
    pub value: (),
}
impl NTCopyAssign {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
    pub unsafe fn copy_assign(&mut self, that: *const NTCopyAssign) -> *mut NTCopyAssign {
        self.value = (*that).value;
        return &mut (*(self as *mut NTCopyAssign));
    }
}
const _: () = assert!(
    true,
    "!std::is_trivially_copy_assignable<NTCopyAssign>::value"
);
const _: () = assert!(true, "std::is_copy_assignable<NTCopyAssign>::value");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TCopyAssign {
    pub value: (),
}
impl TCopyAssign {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
}
const _: () = assert!(
    true,
    "std::is_trivially_copy_assignable<TCopyAssign>::value"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveAssignUnsafe, Default)]
pub struct TCopyAssignNTMoveAssign {
    pub value: (),
}
impl TCopyAssignNTMoveAssign {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        that: *mut TCopyAssignNTMoveAssign,
    ) -> *mut TCopyAssignNTMoveAssign {
        self.value = (*that).value;
        (*that).value = -1_();
        return &mut (*(self as *mut TCopyAssignNTMoveAssign));
    }
}
const _: () = assert!(
    true,
    "std::is_trivially_copy_assignable_v<TCopyAssignNTMoveAssign>"
);
pub unsafe fn test_copy_assignment_not_noexcept_0() {
    {
        const _: () = assert!(true, "!std::is_nothrow_copy_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_copy_assignable<V>::value");;
    }
}
pub unsafe fn test_copy_assignment_sfinae_1() {
    {
        const _: () = assert!(true, "std::is_copy_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_copy_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_copy_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_copy_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_copy_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_copy_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_trivially_copy_assignable<V>::value");;
        const _: () = assert!(true, "std::is_copy_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_copy_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_copy_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_copy_assignable<V>::value");;
    }
}
pub unsafe fn test_copy_assignment_empty_empty_2() {}
pub unsafe fn test_copy_assignment_non_empty_empty_3() {}
pub unsafe fn test_copy_assignment_empty_non_empty_4() {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Result_int_ {
    pub index: (),
    pub value: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Result_long_ {
    pub index: (),
    pub value: (),
}
pub unsafe fn test_copy_assignment_same_index_5() {
    {
        let mut v1: () = 43_();
        let mut v2: () = 42_();
        let vref: *mut () = &mut (*unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut v1: () = 43_();
        let mut v2: () = 42_();
        let vref: *mut () = &mut (*unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut alive: () = 0_();
        let mut copy_construct: () = 0_();
        let mut copy_assign: () = 0_();
        let mut move_construct: () = 0_();
        let mut move_assign: () = 0_();
        let mut v1: () = unimplemented!();
        let mut v2: () = unimplemented!();
        let vref: *mut () = &mut (*unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut test: anon_6 = <anon_6>::default();
        let result: Result_int_ = (unsafe { anon_6::operator_call(&test) });
        const _: () = assert!(true, "result.index == 0");;
        const _: () = assert!(true, "result.value == 42");;
    }
    {
        let mut test: anon_8 = <anon_8>::default();
        let result: Result_long_ = (unsafe { anon_8::operator_call(&test) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42l");;
    }
    {
        let mut test: anon_10 = <anon_10>::default();
        let result: Result_int_ = (unsafe { anon_10::operator_call(&test) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42");;
    }
    {
        let mut test: anon_12 = <anon_12>::default();
        let result: Result_int_ = (unsafe { anon_12::operator_call(&test) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_6 {}
impl anon_6 {
    pub unsafe fn operator_call(&self) -> Result_int_ {
        let mut v: () = 43_();
        let mut v2: () = 42_();
        unimplemented!();
        return Result_int_ {
            index: unimplemented!(),
            value: ((*(unsafe { get_7(&mut v) })) as ()),
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_8 {}
impl anon_8 {
    pub unsafe fn operator_call(&self) -> Result_long_ {
        let mut v: () = 43_();
        let mut v2: () = 42_();
        unimplemented!();
        return Result_long_ {
            index: unimplemented!(),
            value: ((*(unsafe { get_9(&mut v) })) as ()),
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_10 {}
impl anon_10 {
    pub unsafe fn operator_call(&self) -> Result_int_ {
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        return Result_int_ {
            index: unimplemented!(),
            value: (*(unsafe { get_11(&mut v) })).value,
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_12 {}
impl anon_12 {
    pub unsafe fn operator_call(&self) -> Result_int_ {
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        return Result_int_ {
            index: unimplemented!(),
            value: (*(unsafe { get_13(&mut v) })).value,
        };
    }
}
pub unsafe fn test_copy_assignment_different_index_14() {
    {
        let mut v1: () = 43_();
        let mut v2: () = 42_();
        let vref: *mut () = &mut (*unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut alive: () = 0_();
        let mut copy_construct: () = 0_();
        let mut copy_assign: () = 0_();
        let mut move_construct: () = 0_();
        let mut move_assign: () = 0_();
        let mut v1: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let vref: *mut () = &mut (*unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut test: anon_15 = <anon_15>::default();
        let result: Result_long_ = (unsafe { anon_15::operator_call(&test) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42l");;
    }
    {
        let mut test: anon_16 = <anon_16>::default();
        let result: Result_int_ = (unsafe { anon_16::operator_call(&test) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_15 {}
impl anon_15 {
    pub unsafe fn operator_call(&self) -> Result_long_ {
        let mut v: () = 43_();
        let mut v2: () = 42_();
        unimplemented!();
        return Result_long_ {
            index: unimplemented!(),
            value: ((*(unsafe { get_9(&mut v) })) as ()),
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_16 {}
impl anon_16 {
    pub unsafe fn operator_call(&self) -> Result_int_ {
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        return Result_int_ {
            index: unimplemented!(),
            value: (*(unsafe { get_11(&mut v) })).value,
        };
    }
}
pub unsafe fn test_assignment_throw_17() {}
pub unsafe fn test_constexpr_assign_imp_18(v: *mut (), new_value: *mut ()) {
    let cp: std_variant_long__void_ptr__int_ = (*new_value);
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_assign_imp_19(v: *mut (), new_value: *mut ()) {
    let cp: std_variant_long__void_ptr__int_ = (*new_value);
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_assign_imp_20(v: *mut (), new_value: *mut ()) {
    let cp: std_variant_long__void_ptr__int_ = (*new_value);
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_assign_imp_21(v: *mut (), new_value: *mut ()) {
    let cp: std_variant_long__void_ptr__NonTrivialCopyAssign_ = (*new_value);
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_assign_imp_22(v: *mut (), new_value: *mut ()) {
    let cp: std_variant_long__void_ptr__NonTrivialCopyAssign_ = (*new_value);
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_assign_imp_23(v: *mut (), new_value: *mut NonTrivialCopyAssign) {
    let cp: std_variant_long__void_ptr__NonTrivialCopyAssign_ = (*new_value);
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_copy_assignment_trivial_24() {
    const _: () = assert!(true, "std::is_trivially_copyable<V>::value");;
    const _: () = assert!(true, "std::is_trivially_copy_assignable<V>::value");;
    (unsafe {
        let mut _v: () = 42_();
        let mut _new_value: () = 101_();
        test_constexpr_assign_imp_18(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = Default::default();
        let mut _new_value: () = 101_();
        test_constexpr_assign_imp_18(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = 42_();
        let mut _new_value: () = Default::default();
        test_constexpr_assign_imp_19(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = 42_();
        let mut _new_value: () = 101_();
        test_constexpr_assign_imp_20(&mut _v, &mut _new_value)
    });
}
pub unsafe fn operator_eq_25(x: *const NonTrivialCopyAssign, y: *const NonTrivialCopyAssign) -> () {
    return (((*x).i) == ((*y).i));
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe)]
pub struct NonTrivialCopyAssign {
    pub i: (),
}
impl NonTrivialCopyAssign {
    pub unsafe fn new(mut ii: ()) -> Self {
        let mut this = Self { i: ii };
        this
    }
    pub unsafe fn copy_from(other: *const NonTrivialCopyAssign) -> Self {
        let mut this = Self { i: (*other).i };
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        o: *const NonTrivialCopyAssign,
    ) -> *mut NonTrivialCopyAssign {
        self.i = (*o).i;
        return &mut (*(self as *mut NonTrivialCopyAssign));
    }
}
impl std::cmp::PartialEq for NonTrivialCopyAssign {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_25(
                self as *const NonTrivialCopyAssign,
                other as *const NonTrivialCopyAssign,
            )
        }
    }
}
impl std::cmp::Eq for NonTrivialCopyAssign {}
impl Clone for NonTrivialCopyAssign {
    fn clone(&self) -> Self {
        unsafe { NonTrivialCopyAssign::copy_from(self as *const NonTrivialCopyAssign) }
    }
}
impl Default for NonTrivialCopyAssign {
    fn default() -> Self {
        unsafe { NonTrivialCopyAssign { i: 0_() } }
    }
}
pub unsafe fn test_constexpr_copy_assignment_non_trivial_26() {
    const _: () = assert!(true, "!std::is_trivially_copyable<V>::value");;
    const _: () = assert!(true, "!std::is_trivially_copy_assignable<V>::value");;
    (unsafe {
        let mut _v: () = 42_();
        let mut _new_value: () = 101_();
        test_constexpr_assign_imp_21(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = Default::default();
        let mut _new_value: () = 101_();
        test_constexpr_assign_imp_21(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = 42_();
        let mut _new_value: () = Default::default();
        test_constexpr_assign_imp_22(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = 42_();
        let mut _new_value: NonTrivialCopyAssign = NonTrivialCopyAssign::new({ 5_() });
        test_constexpr_assign_imp_23(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = NonTrivialCopyAssign::new({ 3_() });
        let mut _new_value: NonTrivialCopyAssign = NonTrivialCopyAssign::new({ 5_() });
        test_constexpr_assign_imp_23(&mut _v, &mut _new_value)
    });
}
pub unsafe fn non_constexpr_test_27() {
    (unsafe { test_copy_assignment_empty_empty_2() });
    (unsafe { test_copy_assignment_non_empty_empty_3() });
    (unsafe { test_copy_assignment_empty_non_empty_4() });
    (unsafe { test_assignment_throw_17() });
}
pub unsafe fn cxx17_constexpr_test_28() -> () {
    (unsafe { test_copy_assignment_sfinae_1() });
    (unsafe { test_copy_assignment_not_noexcept_0() });
    (unsafe { test_constexpr_copy_assignment_trivial_24() });
    return true;
}
pub unsafe fn cxx20_constexpr_test_29() -> () {
    (unsafe { test_copy_assignment_same_index_5() });
    (unsafe { test_copy_assignment_different_index_14() });
    (unsafe { test_constexpr_copy_assignment_non_trivial_26() });
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
    (unsafe { non_constexpr_test_27() });
    (unsafe { cxx17_constexpr_test_28() });
    (unsafe { cxx20_constexpr_test_29() });
    const _: () = assert!(true, "cxx17_constexpr_test()");;
    const _: () = assert!(true, "cxx20_constexpr_test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
