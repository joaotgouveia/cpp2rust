extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct ThrowsMove {}
impl ThrowsMove {
    pub unsafe fn move_from(_a0: *mut ThrowsMove) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct NoCopy {}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct MoveOnly {
    pub value: (),
}
impl MoveOnly {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
    pub unsafe fn move_from(_a0: *mut MoveOnly) -> Self {
        let mut this = Self {
            value: (*_a0).value,
        };
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct MoveOnlyNT {
    pub value: (),
}
impl MoveOnlyNT {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
    pub unsafe fn move_from(other: *mut MoveOnlyNT) -> Self {
        let mut this = Self {
            value: (*other).value,
        };
        (*other).value = -1_();
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct NTMove {
    pub value: (),
}
impl NTMove {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
    pub unsafe fn move_from(that: *mut NTMove) -> Self {
        let mut this = Self {
            value: (*that).value,
        };
        (*that).value = -1_();
        this
    }
}
const _: () = assert!(true, "!std::is_trivially_move_constructible<NTMove>::value");
const _: () = assert!(true, "std::is_move_constructible<NTMove>::value");
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct TMove {
    pub value: (),
}
impl TMove {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
    pub unsafe fn move_from(_a0: *mut TMove) -> Self {
        let mut this = Self {
            value: (*_a0).value,
        };
        this
    }
}
const _: () = assert!(true, "std::is_trivially_move_constructible<TMove>::value");
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct TMoveNTCopy {
    pub value: (),
}
impl TMoveNTCopy {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
    pub unsafe fn copy_from(that: *const TMoveNTCopy) -> Self {
        let mut this = Self {
            value: (*that).value,
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut TMoveNTCopy) -> Self {
        let mut this = Self {
            value: (*_a0).value,
        };
        this
    }
}
impl Clone for TMoveNTCopy {
    fn clone(&self) -> Self {
        unsafe { TMoveNTCopy::copy_from(self as *const TMoveNTCopy) }
    }
}
const _: () = assert!(
    true,
    "std::is_trivially_move_constructible<TMoveNTCopy>::value"
);
pub unsafe fn test_move_noexcept_0() {
    {
        const _: () = assert!(true, "std::is_nothrow_move_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_nothrow_move_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_move_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_move_constructible<V>::value");;
    }
}
pub unsafe fn test_move_ctor_sfinae_1() {
    {
        const _: () = assert!(true, "std::is_move_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_move_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_move_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_move_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_move_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_trivially_move_constructible<V>::value");;
        const _: () = assert!(true, "std::is_move_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_move_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_move_constructible<V>::value");;
    }
}
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
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct Result_TMove_ {
    pub index: (),
    pub value: TMove,
}
impl Result_TMove_ {
    pub unsafe fn move_from(_a0: *mut Result_TMove_) -> Self {
        let mut this = Self {
            index: (*_a0).index,
            value: TMove::move_from({ &mut (*_a0).value }),
        };
        this
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct Result_TMoveNTCopy_ {
    pub index: (),
    pub value: TMoveNTCopy,
}
pub unsafe fn test_move_ctor_basic_2() {
    {
        let mut v: () = {
            let mut __tmp_0: () = 42_();
            std_variant_int_::new(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_3)).clone() },
                { &mut __tmp_0 },
            )
        };
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut v: () = {
            let mut __tmp_1: () = 42_();
            std_variant_int__long_::new(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_4)).clone() },
                { &mut __tmp_1 },
            )
        };
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut v: () = {
            let mut __tmp_2: () = 42_();
            std_variant_MoveOnly_::new(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_3)).clone() },
                { &mut __tmp_2 },
            )
        };
        unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut v: () = {
            let mut __tmp_3: () = 42_();
            std_variant_int__MoveOnly_::new(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_4)).clone() },
                { &mut __tmp_3 },
            )
        };
        unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut v: () = {
            let mut __tmp_4: () = 42_();
            std_variant_MoveOnlyNT_::new(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_3)).clone() },
                { &mut __tmp_4 },
            )
        };
        unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut v: () = {
            let mut __tmp_5: () = 42_();
            std_variant_int__MoveOnlyNT_::new(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_4)).clone() },
                { &mut __tmp_5 },
            )
        };
        unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut test: anon_5 = <anon_5>::default();
        let result: Result_int_ = (unsafe { anon_5::operator_call(&test) });
        const _: () = assert!(true, "result.index == 0");;
        const _: () = assert!(true, "result.value == 42");;
    }
    {
        let mut test: anon_7 = <anon_7>::default();
        let result: Result_long_ = (unsafe { anon_7::operator_call(&test) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42");;
    }
    {
        let mut test: anon_9 = <anon_9>::default();
        let result: Result_TMove_ = (unsafe { anon_9::operator_call(&test) });
        const _: () = assert!(true, "result.index == 0");;
        const _: () = assert!(true, "result.value.value == 42");;
    }
    {
        let mut test: anon_11 = <anon_11>::default();
        let result: Result_TMove_ = (unsafe { anon_11::operator_call(&test) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value.value == 42");;
    }
    {
        let mut test: anon_13 = <anon_13>::default();
        let result: Result_TMoveNTCopy_ = (unsafe { anon_13::operator_call(&test) });
        const _: () = assert!(true, "result.index == 0");;
        const _: () = assert!(true, "result.value.value == 42");;
    }
    {
        let mut test: anon_15 = <anon_15>::default();
        let result: Result_TMoveNTCopy_ = (unsafe { anon_15::operator_call(&test) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value.value == 42");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_5 {}
impl anon_5 {
    pub unsafe fn operator_call(&self) -> Result_int_ {
        let mut v: () = {
            let mut __tmp_6: () = 42_();
            std_variant_int_::new(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_3)).clone() },
                { &mut __tmp_6 },
            )
        };
        let mut v2: () = unimplemented!();
        return Result_int_ {
            index: unimplemented!(),
            value: ((*(unsafe { get_6(&mut *(&raw mut v2).cast::<()>()) })) as ()),
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_7 {}
impl anon_7 {
    pub unsafe fn operator_call(&self) -> Result_long_ {
        let mut v: () = {
            let mut __tmp_7: () = 42_();
            std_variant_int__long_::new(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_4)).clone() },
                { &mut __tmp_7 },
            )
        };
        let mut v2: () = unimplemented!();
        return Result_long_ {
            index: unimplemented!(),
            value: ((*(unsafe { get_8(&mut *(&raw mut v2).cast::<()>()) })) as ()),
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_9 {}
impl anon_9 {
    pub unsafe fn operator_call(&self) -> Result_TMove_ {
        let mut v: () = {
            let mut __tmp_8: () = 42_();
            std_variant_TMove_::new(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_3)).clone() },
                { &mut __tmp_8 },
            )
        };
        let mut v2: () = unimplemented!();
        return Result_TMove_ {
            index: unimplemented!(),
            value: TMove::move_from({ (unsafe { get_10(&mut *(&raw mut v2).cast::<()>()) }) }),
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_11 {}
impl anon_11 {
    pub unsafe fn operator_call(&self) -> Result_TMove_ {
        let mut v: () = {
            let mut __tmp_9: () = 42_();
            std_variant_int__TMove_::new(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_4)).clone() },
                { &mut __tmp_9 },
            )
        };
        let mut v2: () = unimplemented!();
        return Result_TMove_ {
            index: unimplemented!(),
            value: TMove::move_from({ (unsafe { get_12(&mut *(&raw mut v2).cast::<()>()) }) }),
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_13 {}
impl anon_13 {
    pub unsafe fn operator_call(&self) -> Result_TMoveNTCopy_ {
        let mut v: () = {
            let mut __tmp_10: () = 42_();
            std_variant_TMoveNTCopy_::new(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_3)).clone() },
                { &mut __tmp_10 },
            )
        };
        let mut v2: () = unimplemented!();
        return Result_TMoveNTCopy_ {
            index: unimplemented!(),
            value: TMoveNTCopy::move_from({
                (unsafe { get_14(&mut *(&raw mut v2).cast::<()>()) })
            }),
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_15 {}
impl anon_15 {
    pub unsafe fn operator_call(&self) -> Result_TMoveNTCopy_ {
        let mut v: () = {
            let mut __tmp_11: () = 42_();
            std_variant_int__TMoveNTCopy_::new(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_4)).clone() },
                { &mut __tmp_11 },
            )
        };
        let mut v2: () = unimplemented!();
        return Result_TMoveNTCopy_ {
            index: unimplemented!(),
            value: TMoveNTCopy::move_from({
                (unsafe { get_16(&mut *(&raw mut v2).cast::<()>()) })
            }),
        };
    }
}
pub unsafe fn test_move_ctor_valueless_by_exception_17() {}
pub unsafe fn test_constexpr_ctor_imp_18(v: *const ()) {
    let mut copy: () = unimplemented!();
    let mut v2: std_variant_long__void_ptr__const_int_ = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_ctor_imp_19(v: *const ()) {
    let mut copy: () = unimplemented!();
    let mut v2: std_variant_long__void_ptr__const_int_ = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_ctor_imp_20(v: *const ()) {
    let mut copy: () = unimplemented!();
    let mut v2: std_variant_long__void_ptr__const_int_ = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_ctor_imp_21(v: *const ()) {
    let mut copy: () = unimplemented!();
    let mut v2: std_variant_long__NonTrivialMoveCtor__void_ptr_ = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_ctor_imp_22(v: *const ()) {
    let mut copy: () = unimplemented!();
    let mut v2: std_variant_long__NonTrivialMoveCtor__void_ptr_ = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_ctor_imp_23(v: *const ()) {
    let mut copy: () = unimplemented!();
    let mut v2: std_variant_long__NonTrivialMoveCtor__void_ptr_ = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_move_ctor_trivial_24() {
    const _: () = assert!(true, "std::is_trivially_copyable<V>::value");;
    const _: () = assert!(true, "std::is_trivially_move_constructible<V>::value");;
    (unsafe {
        let mut _v: () = 42_();
        test_constexpr_ctor_imp_18(&mut _v)
    });
    (unsafe {
        let mut _v: () = Default::default();
        test_constexpr_ctor_imp_19(&mut _v)
    });
    (unsafe {
        let mut _v: () = 101_();
        test_constexpr_ctor_imp_20(&mut _v)
    });
}
pub unsafe fn operator_eq_25(x: *const NonTrivialMoveCtor, y: *const NonTrivialMoveCtor) -> () {
    return (((*x).i) == ((*y).i));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct NonTrivialMoveCtor {
    pub i: (),
}
impl NonTrivialMoveCtor {
    pub unsafe fn new(mut ii: ()) -> Self {
        let mut this = Self { i: ii };
        this
    }
    pub unsafe fn move_from(other: *mut NonTrivialMoveCtor) -> Self {
        let mut this = Self { i: (*other).i };
        this
    }
}
impl std::cmp::PartialEq for NonTrivialMoveCtor {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_25(
                self as *const NonTrivialMoveCtor,
                other as *const NonTrivialMoveCtor,
            )
        }
    }
}
impl std::cmp::Eq for NonTrivialMoveCtor {}
impl Default for NonTrivialMoveCtor {
    fn default() -> Self {
        unsafe { NonTrivialMoveCtor { i: 0_() } }
    }
}
pub unsafe fn test_constexpr_move_ctor_non_trivial_26() {
    const _: () = assert!(true, "!std::is_trivially_move_constructible<V>::value");;
    (unsafe {
        let mut _v: () = 42_();
        test_constexpr_ctor_imp_21(&mut _v)
    });
    (unsafe {
        let mut _v: () = NonTrivialMoveCtor::new({ 5_() });
        test_constexpr_ctor_imp_22(&mut _v)
    });
    (unsafe {
        let mut _v: () = Default::default();
        test_constexpr_ctor_imp_23(&mut _v)
    });
}
pub unsafe fn non_constexpr_test_27() {
    (unsafe { test_move_ctor_valueless_by_exception_17() });
}
pub unsafe fn cxx17_constexpr_test_28() -> () {
    (unsafe { test_move_noexcept_0() });
    (unsafe { test_move_ctor_sfinae_1() });
    (unsafe { test_constexpr_move_ctor_trivial_24() });
    return true;
}
pub unsafe fn cxx20_constexpr_test_29() -> () {
    (unsafe { test_move_ctor_basic_2() });
    (unsafe { test_constexpr_move_ctor_non_trivial_26() });
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
