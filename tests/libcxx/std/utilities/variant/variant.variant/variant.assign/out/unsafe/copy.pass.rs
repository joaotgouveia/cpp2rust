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
    pub value: i32,
    pub alive: *mut i32,
    pub copy_construct: *mut i32,
    pub copy_assign: *mut i32,
    pub move_construct: *mut i32,
    pub move_assign: *mut i32,
}
impl CopyAssign {
    pub unsafe fn new(
        mut v: i32,
        mut alv: *mut i32,
        mut cpy_ctr: *mut i32,
        mut cpy_assi: *mut i32,
        mut move_ctr: *mut i32,
        mut move_assi: *mut i32,
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
        (*o).value = -1_i32;
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
        (*o).value = -1_i32;
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
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct NTCopyAssign {
    pub value: i32,
}
impl NTCopyAssign {
    pub unsafe fn new(mut v: i32) -> Self {
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
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct TCopyAssign {
    pub value: i32,
}
impl TCopyAssign {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        this
    }
}
const _: () = assert!(
    true,
    "std::is_trivially_copy_assignable<TCopyAssign>::value"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, MoveAssignUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct TCopyAssignNTMoveAssign {
    pub value: i32,
}
impl TCopyAssignNTMoveAssign {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        that: *mut TCopyAssignNTMoveAssign,
    ) -> *mut TCopyAssignNTMoveAssign {
        self.value = (*that).value;
        (*that).value = -1_i32;
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
    pub index: usize,
    pub value: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Result_long_ {
    pub index: usize,
    pub value: i64,
}
pub unsafe fn test_copy_assignment_same_index_5() {
    {
        let mut v1: () = unimplemented!();
        let mut v2: () = unimplemented!();
        let vref: *mut () = &mut (*unimplemented!());
        assert!(((vref) == (&mut v1 as *mut ())));
        assert!(((unimplemented!()) == (0_usize)));
        assert!(((*unimplemented!()) == (42)));
    }
    {
        let mut v1: () = unimplemented!();
        let mut v2: () = unimplemented!();
        let vref: *mut () = &mut (*unimplemented!());
        assert!(((vref) == (&mut v1 as *mut ())));
        assert!(((unimplemented!()) == (1_usize)));
        assert!(((*unimplemented!()) == (42_())));
    }
    {
        let mut alive: i32 = 0;
        let mut copy_construct: i32 = 0;
        let mut copy_assign: i32 = 0;
        let mut move_construct: i32 = 0;
        let mut move_assign: i32 = 0;
        let mut v1: () = unimplemented!();
        let mut v2: () = unimplemented!();
        let vref: *mut () = &mut (*unimplemented!());
        assert!(((vref) == (&mut v1 as *mut ())));
        assert!(((unimplemented!()) == (1_usize)));
        assert!((((*unimplemented!()).value) == (42)));
        assert!(((copy_construct) == (0)));
        assert!(((move_construct) == (0)));
        assert!(((copy_assign) == (1)));
    }
    {
        let mut test: anon_6 = <anon_6>::default();
        let result: Result_int_ = (unsafe { anon_6::operator_call(&test) });
        const _: () = assert!(true, "result.index == 0");;
        const _: () = assert!(true, "result.value == 42");;
    }
    {
        let mut test: anon_7 = <anon_7>::default();
        let result: Result_long_ = (unsafe { anon_7::operator_call(&test) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42l");;
    }
    {
        let mut test: anon_8 = <anon_8>::default();
        let result: Result_int_ = (unsafe { anon_8::operator_call(&test) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42");;
    }
    {
        let mut test: anon_9 = <anon_9>::default();
        let result: Result_int_ = (unsafe { anon_9::operator_call(&test) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_6 {}
impl anon_6 {
    pub unsafe fn operator_call(&self) -> Result_int_ {
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        return Result_int_ {
            index: unimplemented!(),
            value: ((*unimplemented!()) as i32),
        };
    }
}
impl Callable0<Result_int_> for anon_6 {
    fn call(&self) -> Result_int_ {
        unsafe { anon_6::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_7 {}
impl anon_7 {
    pub unsafe fn operator_call(&self) -> Result_long_ {
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        return Result_long_ {
            index: unimplemented!(),
            value: ((*unimplemented!()) as i64),
        };
    }
}
impl Callable0<Result_long_> for anon_7 {
    fn call(&self) -> Result_long_ {
        unsafe { anon_7::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_8 {}
impl anon_8 {
    pub unsafe fn operator_call(&self) -> Result_int_ {
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        return Result_int_ {
            index: unimplemented!(),
            value: (*unimplemented!()).value,
        };
    }
}
impl Callable0<Result_int_> for anon_8 {
    fn call(&self) -> Result_int_ {
        unsafe { anon_8::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_9 {}
impl anon_9 {
    pub unsafe fn operator_call(&self) -> Result_int_ {
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        return Result_int_ {
            index: unimplemented!(),
            value: (*unimplemented!()).value,
        };
    }
}
impl Callable0<Result_int_> for anon_9 {
    fn call(&self) -> Result_int_ {
        unsafe { anon_9::operator_call(self) }
    }
}
pub unsafe fn test_copy_assignment_different_index_10() {
    {
        let mut v1: () = unimplemented!();
        let mut v2: () = unimplemented!();
        let vref: *mut () = &mut (*unimplemented!());
        assert!(((vref) == (&mut v1 as *mut ())));
        assert!(((unimplemented!()) == (1_usize)));
        assert!(((*unimplemented!()) == (42_())));
    }
    {
        let mut alive: i32 = 0;
        let mut copy_construct: i32 = 0;
        let mut copy_assign: i32 = 0;
        let mut move_construct: i32 = 0;
        let mut move_assign: i32 = 0;
        let mut v1: () = unimplemented!();
        let mut v2: () = unimplemented!();
        assert!(((copy_construct) == (0)));
        assert!(((move_construct) == (0)));
        assert!(((alive) == (1)));
        let vref: *mut () = &mut (*unimplemented!());
        assert!(((vref) == (&mut v1 as *mut ())));
        assert!(((unimplemented!()) == (1_usize)));
        assert!((((*unimplemented!()).value) == (42)));
        assert!(((alive) == (2)));
        assert!(((copy_construct) == (1)));
        assert!(((move_construct) == (1)));
        assert!(((copy_assign) == (0)));
    }
    {
        let mut test: anon_11 = <anon_11>::default();
        let result: Result_long_ = (unsafe { anon_11::operator_call(&test) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42l");;
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
pub struct anon_11 {}
impl anon_11 {
    pub unsafe fn operator_call(&self) -> Result_long_ {
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        return Result_long_ {
            index: unimplemented!(),
            value: ((*unimplemented!()) as i64),
        };
    }
}
impl Callable0<Result_long_> for anon_11 {
    fn call(&self) -> Result_long_ {
        unsafe { anon_11::operator_call(self) }
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
            value: (*unimplemented!()).value,
        };
    }
}
impl Callable0<Result_int_> for anon_12 {
    fn call(&self) -> Result_int_ {
        unsafe { anon_12::operator_call(self) }
    }
}
pub unsafe fn test_assignment_throw_13() {}
pub unsafe fn test_constexpr_assign_imp_14(v: *mut (), new_value: *mut i64) {
    let cp: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!()) == (0_u64 as usize)));
    assert!(((*unimplemented!()) == (*unimplemented!())));
}
pub unsafe fn test_constexpr_assign_imp_15(v: *mut (), new_value: *mut ()) {
    let cp: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!()) == (1_u64 as usize)));
    assert!(((*unimplemented!()) == (*unimplemented!())));
}
pub unsafe fn test_constexpr_assign_imp_16(v: *mut (), new_value: *mut i32) {
    let cp: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!()) == (2_u64 as usize)));
    assert!(((*unimplemented!()) == (*unimplemented!())));
}
pub unsafe fn test_constexpr_assign_imp_17(v: *mut (), new_value: *mut i64) {
    let cp: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!()) == (0_u64 as usize)));
    assert!(((*unimplemented!()) == (*unimplemented!())));
}
pub unsafe fn test_constexpr_assign_imp_18(v: *mut (), new_value: *mut ()) {
    let cp: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!()) == (1_u64 as usize)));
    assert!(((*unimplemented!()) == (*unimplemented!())));
}
pub unsafe fn test_constexpr_assign_imp_19(v: *mut (), new_value: *mut NonTrivialCopyAssign) {
    let cp: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!()) == (2_u64 as usize)));
    assert!(
        (unsafe {
            let _x: *const NonTrivialCopyAssign = unimplemented!();
            operator_eq_20(_x, unimplemented!())
        })
    );
}
pub unsafe fn test_constexpr_copy_assignment_trivial_21() {
    const _: () = assert!(true, "std::is_trivially_copyable<V>::value");;
    const _: () = assert!(true, "std::is_trivially_copy_assignable<V>::value");;
    (unsafe {
        let mut _v: () = unimplemented!();
        let mut _new_value: i64 = 101_i64;
        test_constexpr_assign_imp_14(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = unimplemented!();
        let mut _new_value: i64 = 101_i64;
        test_constexpr_assign_imp_14(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = unimplemented!();
        let mut _new_value: () = Default::default();
        test_constexpr_assign_imp_15(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = unimplemented!();
        let mut _new_value: i32 = 101;
        test_constexpr_assign_imp_16(&mut _v, &mut _new_value)
    });
}
pub unsafe fn operator_eq_20(
    x: *const NonTrivialCopyAssign,
    y: *const NonTrivialCopyAssign,
) -> bool {
    return (((*x).i) == ((*y).i));
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe)]
#[cpp_from(i32, new, value)]
pub struct NonTrivialCopyAssign {
    pub i: i32,
}
impl NonTrivialCopyAssign {
    pub unsafe fn new(mut ii: i32) -> Self {
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
            operator_eq_20(
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
        unsafe { NonTrivialCopyAssign { i: 0 } }
    }
}
pub unsafe fn test_constexpr_copy_assignment_non_trivial_22() {
    const _: () = assert!(true, "!std::is_trivially_copyable<V>::value");;
    const _: () = assert!(true, "!std::is_trivially_copy_assignable<V>::value");;
    (unsafe {
        let mut _v: () = unimplemented!();
        let mut _new_value: i64 = 101_i64;
        test_constexpr_assign_imp_17(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = unimplemented!();
        let mut _new_value: i64 = 101_i64;
        test_constexpr_assign_imp_17(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = unimplemented!();
        let mut _new_value: () = Default::default();
        test_constexpr_assign_imp_18(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = unimplemented!();
        let mut _new_value: NonTrivialCopyAssign = NonTrivialCopyAssign::new({ 5 });
        test_constexpr_assign_imp_19(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = unimplemented!();
        let mut _new_value: NonTrivialCopyAssign = NonTrivialCopyAssign::new({ 5 });
        test_constexpr_assign_imp_19(&mut _v, &mut _new_value)
    });
}
pub unsafe fn non_constexpr_test_23() {
    (unsafe { test_copy_assignment_empty_empty_2() });
    (unsafe { test_copy_assignment_non_empty_empty_3() });
    (unsafe { test_copy_assignment_empty_non_empty_4() });
    (unsafe { test_assignment_throw_13() });
}
pub unsafe fn cxx17_constexpr_test_24() -> bool {
    (unsafe { test_copy_assignment_sfinae_1() });
    (unsafe { test_copy_assignment_not_noexcept_0() });
    (unsafe { test_constexpr_copy_assignment_trivial_21() });
    return true;
}
pub unsafe fn cxx20_constexpr_test_25() -> bool {
    (unsafe { test_copy_assignment_same_index_5() });
    (unsafe { test_copy_assignment_different_index_10() });
    (unsafe { test_constexpr_copy_assignment_non_trivial_22() });
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
    (unsafe { non_constexpr_test_23() });
    (unsafe { cxx17_constexpr_test_24() });
    (unsafe { cxx20_constexpr_test_25() });
    const _: () = assert!(true, "cxx17_constexpr_test()");;
    const _: () = assert!(true, "cxx20_constexpr_test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
