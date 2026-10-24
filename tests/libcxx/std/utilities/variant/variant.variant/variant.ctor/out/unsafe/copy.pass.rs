extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct NonT {
    pub value: (),
}
impl NonT {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
    pub unsafe fn copy_from(o: *const NonT) -> Self {
        let mut this = Self { value: (*o).value };
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const NonT) -> *mut NonT {
        self.value = (*_a0).value;
        return &mut (*(self as *mut NonT));
    }
}
impl Clone for NonT {
    fn clone(&self) -> Self {
        unsafe { NonT::copy_from(self as *const NonT) }
    }
}
const _: () = assert!(true, "!std::is_trivially_copy_constructible<NonT>::value");
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct NoCopy {}
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
#[derive(VaArg, FnPtrArg, Default)]
pub struct NTCopy {
    pub value: (),
}
impl NTCopy {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
    pub unsafe fn copy_from(that: *const NTCopy) -> Self {
        let mut this = Self {
            value: (*that).value,
        };
        this
    }
}
impl Clone for NTCopy {
    fn clone(&self) -> Self {
        unsafe { NTCopy::copy_from(self as *const NTCopy) }
    }
}
const _: () = assert!(true, "!std::is_trivially_copy_constructible<NTCopy>::value");
const _: () = assert!(true, "std::is_copy_constructible<NTCopy>::value");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TCopy {
    pub value: (),
}
impl TCopy {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
}
const _: () = assert!(true, "std::is_trivially_copy_constructible<TCopy>::value");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct TCopyNTMove {
    pub value: (),
}
impl TCopyNTMove {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
    pub unsafe fn move_from(that: *mut TCopyNTMove) -> Self {
        let mut this = Self {
            value: (*that).value,
        };
        (*that).value = -1_();
        this
    }
}
const _: () = assert!(
    true,
    "std::is_trivially_copy_constructible<TCopyNTMove>::value"
);
pub unsafe fn test_copy_ctor_sfinae_0() {
    {
        const _: () = assert!(true, "std::is_copy_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_copy_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_copy_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_copy_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_copy_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_trivially_copy_constructible<V>::value");;
        const _: () = assert!(true, "std::is_copy_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_copy_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_copy_constructible<V>::value");;
    }
}
pub unsafe fn test_copy_ctor_basic_1() {
    {
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut v: () = unimplemented!();
        unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut v: () = unimplemented!();
        unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let v: () = unimplemented!();
        const _: () = assert!(true, "v.index() == 0");;
        let v2: () = unimplemented!();
        const _: () = assert!(true, "v2.index() == 0");;
        const _: () = assert!(true, "std::get<0>(v2) == 42");;
    }
    {
        let v: () = unimplemented!();
        const _: () = assert!(true, "v.index() == 1");;
        let v2: () = unimplemented!();
        const _: () = assert!(true, "v2.index() == 1");;
        const _: () = assert!(true, "std::get<1>(v2) == 42");;
    }
    {
        let v: () = unimplemented!();
        const _: () = assert!(true, "v.index() == 0");;
        let v2: () = unimplemented!();
        const _: () = assert!(true, "v2.index() == 0");;
        const _: () = assert!(true, "std::get<0>(v2).value == 42");;
    }
    {
        let v: () = unimplemented!();
        const _: () = assert!(true, "v.index() == 1");;
        let v2: () = unimplemented!();
        const _: () = assert!(true, "v2.index() == 1");;
        const _: () = assert!(true, "std::get<1>(v2).value == 42");;
    }
    {
        let v: () = unimplemented!();
        const _: () = assert!(true, "v.index() == 0");;
        let v2: () = unimplemented!();
        const _: () = assert!(true, "v2.index() == 0");;
        const _: () = assert!(true, "std::get<0>(v2).value == 42");;
    }
    {
        let v: () = unimplemented!();
        const _: () = assert!(true, "v.index() == 1");;
        let v2: () = unimplemented!();
        const _: () = assert!(true, "v2.index() == 1");;
        const _: () = assert!(true, "std::get<1>(v2).value == 42");;
    }
}
pub unsafe fn test_copy_ctor_valueless_by_exception_2() {}
pub unsafe fn test_constexpr_copy_ctor_imp_3(v: *const ()) {
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_copy_ctor_imp_4(v: *const ()) {
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_copy_ctor_imp_5(v: *const ()) {
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_copy_ctor_imp_6(v: *const ()) {
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_copy_ctor_imp_7(v: *const ()) {
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_copy_ctor_imp_8(v: *const ()) {
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_copy_ctor_trivial_9() {
    const _: () = assert!(true, "std::is_trivially_copyable<V>::value");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible<V>::value");;
    (unsafe {
        let mut _v: () = unimplemented!();
        test_constexpr_copy_ctor_imp_3(&mut _v)
    });
    (unsafe {
        let mut _v: () = unimplemented!();
        test_constexpr_copy_ctor_imp_4(&mut _v)
    });
    (unsafe {
        let mut _v: () = unimplemented!();
        test_constexpr_copy_ctor_imp_5(&mut _v)
    });
}
pub unsafe fn operator_eq_10(x: *const NonTrivialCopyCtor, y: *const NonTrivialCopyCtor) -> () {
    return (((*x).i) == ((*y).i));
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct NonTrivialCopyCtor {
    pub i: (),
}
impl NonTrivialCopyCtor {
    pub unsafe fn new(mut ii: ()) -> Self {
        let mut this = Self { i: ii };
        this
    }
    pub unsafe fn copy_from(other: *const NonTrivialCopyCtor) -> Self {
        let mut this = Self { i: (*other).i };
        this
    }
    pub unsafe fn move_from(other: *mut NonTrivialCopyCtor) -> Self {
        let mut this = Self { i: (*other).i };
        this
    }
}
impl std::cmp::PartialEq for NonTrivialCopyCtor {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_10(
                self as *const NonTrivialCopyCtor,
                other as *const NonTrivialCopyCtor,
            )
        }
    }
}
impl std::cmp::Eq for NonTrivialCopyCtor {}
impl Clone for NonTrivialCopyCtor {
    fn clone(&self) -> Self {
        unsafe { NonTrivialCopyCtor::copy_from(self as *const NonTrivialCopyCtor) }
    }
}
impl Default for NonTrivialCopyCtor {
    fn default() -> Self {
        unsafe { NonTrivialCopyCtor { i: 0_() } }
    }
}
pub unsafe fn test_constexpr_copy_ctor_non_trivial_11() {
    const _: () = assert!(true, "!std::is_trivially_copy_constructible<V>::value");;
    (unsafe {
        let mut _v: () = unimplemented!();
        test_constexpr_copy_ctor_imp_6(&mut _v)
    });
    (unsafe {
        let mut _v: () = unimplemented!();
        test_constexpr_copy_ctor_imp_7(&mut _v)
    });
    (unsafe {
        let mut _v: () = unimplemented!();
        test_constexpr_copy_ctor_imp_8(&mut _v)
    });
}
pub unsafe fn non_constexpr_test_12() {
    (unsafe { test_copy_ctor_valueless_by_exception_2() });
}
pub unsafe fn cxx17_constexpr_test_13() -> () {
    (unsafe { test_copy_ctor_sfinae_0() });
    (unsafe { test_constexpr_copy_ctor_trivial_9() });
    return true;
}
pub unsafe fn cxx20_constexpr_test_14() -> () {
    (unsafe { test_copy_ctor_basic_1() });
    (unsafe { test_constexpr_copy_ctor_non_trivial_11() });
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
    (unsafe { non_constexpr_test_12() });
    (unsafe { cxx17_constexpr_test_13() });
    (unsafe { cxx20_constexpr_test_14() });
    const _: () = assert!(true, "cxx17_constexpr_test()");;
    const _: () = assert!(true, "cxx20_constexpr_test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
