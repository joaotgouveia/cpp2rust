extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn swap_0(x: *mut ADLSwap, y: *mut ADLSwap) {
    unimplemented!();
    (*x).adlSwapCalled = true;
    (*y).adlSwapCalled = true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ADLSwap {
    pub i: (),
    pub adlSwapCalled: (),
}
impl ADLSwap {
    pub unsafe fn new(mut ii: ()) -> Self {
        let mut this = Self {
            i: ii,
            adlSwapCalled: false,
        };
        this
    }
}
impl Default for ADLSwap {
    fn default() -> Self {
        unsafe {
            ADLSwap {
                i: unimplemented!(),
                adlSwapCalled: false,
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct MoveOnlyErrorType {}
impl MoveOnlyErrorType {
    pub unsafe fn new(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_1(_a0: *mut MoveOnlyErrorType) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_2(_a0: *const MoveOnlyErrorType) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn operator_eq_1(_a0: *const TailClobberer_0_, _a1: *const TailClobberer_0_) -> () {
    return (((*_a0).b) == ((*_a1).b));
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct TailClobberer_0_ {
    b: (),
}
impl TailClobberer_0_ {
    pub unsafe fn move_from(_a0: *mut TailClobberer_0_) -> Self {
        let mut this = Self { b: (*_a0).b };
        this
    }
}
impl std::cmp::PartialEq for TailClobberer_0_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_1(
                self as *const TailClobberer_0_,
                other as *const TailClobberer_0_,
            )
        }
    }
}
impl std::cmp::Eq for TailClobberer_0_ {}
pub unsafe fn operator_eq_2(_a0: *const TailClobberer_1_, _a1: *const TailClobberer_1_) -> () {
    return (((*_a0).b) == ((*_a1).b));
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe)]
pub struct TailClobberer_1_ {
    b: (),
}
impl TailClobberer_1_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            b: unimplemented!(),
        };
        {
            unimplemented!();
        }
        this.b = false;
        this
    }
    pub unsafe fn move_from(_a0: *mut TailClobberer_1_) -> Self {
        let mut this = Self { b: (*_a0).b };
        this
    }
}
impl std::cmp::PartialEq for TailClobberer_1_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_2(
                self as *const TailClobberer_1_,
                other as *const TailClobberer_1_,
            )
        }
    }
}
impl std::cmp::Eq for TailClobberer_1_ {}
impl Default for TailClobberer_1_ {
    fn default() -> Self {
        unsafe { TailClobberer_1_::new_1() }
    }
}
const _: () = assert!(
    true,
    "!std::is_trivially_copy_constructible_v<TailClobberer<0>>"
);
const _: () = assert!(
    true,
    "std::is_trivially_move_constructible_v<TailClobberer<0>>"
);
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct TailClobbererNonTrivialMove_0__true__false_ {}
impl TailClobbererNonTrivialMove_0__true__false_ {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct TailClobbererNonTrivialMove_0__false__false_ {}
impl TailClobbererNonTrivialMove_0__false__false_ {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct TailClobbererNonTrivialMove_1__true__false_ {}
impl TailClobbererNonTrivialMove_1__true__false_ {
    pub unsafe fn move_from(_a0: *mut TailClobbererNonTrivialMove_1__true__false_) -> Self {
        let mut this = Self {};
        this
    }
}
const _: () = assert!(
    true,
    "!std::is_trivially_copy_constructible_v<TailClobbererNonTrivialMove<0>>"
);
const _: () = assert!(
    true,
    "std::is_move_constructible_v<TailClobbererNonTrivialMove<0>>"
);
const _: () = assert!(
    true,
    "!std::is_trivially_move_constructible_v<TailClobbererNonTrivialMove<0>>"
);
const _: () = assert!(
    true,
    "std::is_nothrow_move_constructible_v<TailClobbererNonTrivialMove<0, true>>"
);
const _: () = assert!(
    true,
    "!std::is_nothrow_move_constructible_v<TailClobbererNonTrivialMove<0, false>>"
);
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe)]
pub struct BoolWithPadding {
    val_: (),
}
impl BoolWithPadding {
    pub unsafe fn new_1() -> Self {
        let mut this = BoolWithPadding::new_2({ false });
        this
    }
    pub unsafe fn new_2(mut val: ()) -> Self {
        let mut this = Self {
            val_: unimplemented!(),
        };
        {
            unimplemented!();
        }
        this.val_ = val;
        this
    }
    pub unsafe fn copy_from(other: *const BoolWithPadding) -> Self {
        let mut this = BoolWithPadding::new_2({ (*other).val_ });
        this
    }
    pub unsafe fn copy_assign(&mut self, other: *const BoolWithPadding) -> *mut BoolWithPadding {
        self.val_ = (*other).val_;
        return &mut (*(self as *mut BoolWithPadding));
    }
    pub unsafe fn move_from(_a0: *mut BoolWithPadding) -> Self {
        let mut this = Self { val_: (*_a0).val_ };
        this
    }
}
impl Clone for BoolWithPadding {
    fn clone(&self) -> Self {
        unsafe { BoolWithPadding::copy_from(self as *const BoolWithPadding) }
    }
}
impl Default for BoolWithPadding {
    fn default() -> Self {
        unsafe { BoolWithPadding::new_1() }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe)]
pub struct IntWithoutPadding {
    val_: (),
}
impl IntWithoutPadding {
    pub unsafe fn new_1() -> Self {
        let mut this = IntWithoutPadding::new_2({ 0_() });
        this
    }
    pub unsafe fn new_2(mut val: ()) -> Self {
        let mut this = Self {
            val_: unimplemented!(),
        };
        {
            unimplemented!();
        }
        this.val_ = val;
        this
    }
    pub unsafe fn copy_from(other: *const IntWithoutPadding) -> Self {
        let mut this = IntWithoutPadding::new_2({ (*other).val_ });
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        other: *const IntWithoutPadding,
    ) -> *mut IntWithoutPadding {
        self.val_ = (*other).val_;
        return &mut (*(self as *mut IntWithoutPadding));
    }
    pub unsafe fn move_from(_a0: *mut IntWithoutPadding) -> Self {
        let mut this = Self { val_: (*_a0).val_ };
        this
    }
}
impl Clone for IntWithoutPadding {
    fn clone(&self) -> Self {
        unsafe { IntWithoutPadding::copy_from(self as *const IntWithoutPadding) }
    }
}
impl Default for IntWithoutPadding {
    fn default() -> Self {
        unsafe { IntWithoutPadding::new_1() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CheckForInvalidWritesBaseImpl_true__false_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CheckForInvalidWritesBaseImpl_false__false_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CheckForInvalidWritesBaseImpl_true__true_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CheckForInvalidWritesBaseImpl_false__true_ {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct NonMovable {}
pub unsafe fn operator_eq_3(_a0: *const MovableNonTrivial, _a1: *const MovableNonTrivial) -> () {
    return (((*_a0).i) == ((*_a1).i));
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct MovableNonTrivial {
    pub i: (),
}
impl MovableNonTrivial {
    pub unsafe fn new(mut ii: ()) -> Self {
        let mut this = Self { i: ii };
        this
    }
    pub unsafe fn move_from(o: *mut MovableNonTrivial) -> Self {
        let mut this = Self { i: (*o).i };
        (*o).i = 0_();
        this
    }
}
impl std::cmp::PartialEq for MovableNonTrivial {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_3(
                self as *const MovableNonTrivial,
                other as *const MovableNonTrivial,
            )
        }
    }
}
impl std::cmp::Eq for MovableNonTrivial {}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct MoveMayThrow {}
impl MoveMayThrow {
    pub unsafe fn move_from(_a0: *mut MoveMayThrow) -> Self {
        let mut this = Self {};
        this
    }
}
const _: () = assert!(
    true,
    "std::is_move_constructible_v<std::expected<void, int>>"
);
const _: () = assert!(
    true,
    "std::is_move_constructible_v<std::expected<void, MovableNonTrivial>>"
);
const _: () = assert!(
    true,
    "!std::is_move_constructible_v<std::expected<void, NonMovable>>"
);
const _: () = assert!(
    true,
    "std::is_trivially_move_constructible_v<std::expected<void, int>>"
);
const _: () = assert!(
    true,
    "!std::is_trivially_move_constructible_v<std::expected<void, MovableNonTrivial>>"
);
const _: () = assert!(
    true,
    "std::is_nothrow_move_constructible_v<std::expected<int, int>>"
);
const _: () = assert!(
    true,
    "!std::is_nothrow_move_constructible_v<std::expected<MoveMayThrow, int>>"
);
const _: () = assert!(
    true,
    "!std::is_nothrow_move_constructible_v<std::expected<int, MoveMayThrow>>"
);
const _: () = assert!(
    true,
    "!std::is_nothrow_move_constructible_v<std::expected<MoveMayThrow, MoveMayThrow>>"
);
pub unsafe fn test_4() -> () {
    {
        let mut e1: () = unimplemented!();
        let mut e2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut e1: () = unimplemented!();
        let mut e2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut e1: () = unimplemented!();
        let mut e2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub unsafe fn testException_5() {}
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
    (unsafe { test_4() });
    const _: () = assert!(true, "test()");;
    (unsafe { testException_5() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
