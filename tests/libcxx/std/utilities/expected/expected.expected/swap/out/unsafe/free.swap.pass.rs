extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn swap_0(x: *mut ADLSwap, y: *mut ADLSwap) {
    (unsafe {
        let ___x: *mut () = &mut (*x).i;
        let ___y: *mut () = &mut (*y).i;
        swap_1(___x, ___y)
    });
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
pub unsafe fn swap_2(x: *mut TrackedMove_true_, y: *mut TrackedMove_true_) {
    (unsafe {
        let ___x: *mut () = &mut (*x).i;
        let ___y: *mut () = &mut (*y).i;
        swap_1(___x, ___y)
    });
    (unsafe {
        let ___x: *mut () = &mut (*x).numberOfMoves;
        let ___y: *mut () = &mut (*y).numberOfMoves;
        swap_1(___x, ___y)
    });
    (*x).swapCalled = true;
    (*y).swapCalled = true;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe)]
pub struct TrackedMove_true_ {
    pub i: (),
    pub numberOfMoves: (),
    pub swapCalled: (),
}
impl TrackedMove_true_ {
    pub unsafe fn new(mut ii: ()) -> Self {
        let mut this = Self {
            i: ii,
            numberOfMoves: 0_(),
            swapCalled: false,
        };
        this
    }
    pub unsafe fn move_from(other: *mut TrackedMove_true_) -> Self {
        let mut this = Self {
            i: (*other).i,
            numberOfMoves: (*other).numberOfMoves,
            swapCalled: (*other).swapCalled,
        };
        this.numberOfMoves.prefix_inc();
        this
    }
}
impl Default for TrackedMove_true_ {
    fn default() -> Self {
        unsafe {
            TrackedMove_true_ {
                i: unimplemented!(),
                numberOfMoves: 0_(),
                swapCalled: false,
            }
        }
    }
}
pub unsafe fn swap_3(x: *mut TrackedMove_false_, y: *mut TrackedMove_false_) {
    (unsafe {
        let ___x: *mut () = &mut (*x).i;
        let ___y: *mut () = &mut (*y).i;
        swap_1(___x, ___y)
    });
    (unsafe {
        let ___x: *mut () = &mut (*x).numberOfMoves;
        let ___y: *mut () = &mut (*y).numberOfMoves;
        swap_1(___x, ___y)
    });
    (*x).swapCalled = true;
    (*y).swapCalled = true;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe)]
pub struct TrackedMove_false_ {
    pub i: (),
    pub numberOfMoves: (),
    pub swapCalled: (),
}
impl TrackedMove_false_ {
    pub unsafe fn new(mut ii: ()) -> Self {
        let mut this = Self {
            i: ii,
            numberOfMoves: 0_(),
            swapCalled: false,
        };
        this
    }
    pub unsafe fn move_from(other: *mut TrackedMove_false_) -> Self {
        let mut this = Self {
            i: (*other).i,
            numberOfMoves: (*other).numberOfMoves,
            swapCalled: (*other).swapCalled,
        };
        this.numberOfMoves.prefix_inc();
        this
    }
}
impl Default for TrackedMove_false_ {
    fn default() -> Self {
        unsafe {
            TrackedMove_false_ {
                i: unimplemented!(),
                numberOfMoves: 0_(),
                swapCalled: false,
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
pub unsafe fn operator_eq_4(_a0: *const TailClobberer_0_, _a1: *const TailClobberer_0_) -> () {
    return (((*_a0).b) == ((*_a1).b));
}
pub unsafe fn swap_5(_a0: *mut TailClobberer_0_, _a1: *mut TailClobberer_0_) {}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe)]
pub struct TailClobberer_0_ {
    b: (),
}
impl TailClobberer_0_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            b: unimplemented!(),
        };
        {
            (unsafe {
                let ___s: () = (&raw mut this as ());
                let ___n: () = ::std::mem::size_of::<TailClobberer_0_>();
                memset_6(___s, 0_(), ___n)
            });
        }
        this.b = false;
        this
    }
    pub unsafe fn move_from(_a0: *mut TailClobberer_0_) -> Self {
        let mut this = Self { b: (*_a0).b };
        this
    }
}
impl std::cmp::PartialEq for TailClobberer_0_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_4(
                self as *const TailClobberer_0_,
                other as *const TailClobberer_0_,
            )
        }
    }
}
impl std::cmp::Eq for TailClobberer_0_ {}
impl Default for TailClobberer_0_ {
    fn default() -> Self {
        unsafe { TailClobberer_0_::new_1() }
    }
}
pub unsafe fn operator_eq_7(_a0: *const TailClobberer_1_, _a1: *const TailClobberer_1_) -> () {
    return (((*_a0).b) == ((*_a1).b));
}
pub unsafe fn swap_8(_a0: *mut TailClobberer_1_, _a1: *mut TailClobberer_1_) {}
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
            (unsafe {
                let ___s: () = (&raw mut this as ());
                let ___n: () = ::std::mem::size_of::<TailClobberer_1_>();
                memset_6(___s, 1_(), ___n)
            });
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
            operator_eq_7(
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
impl TailClobbererNonTrivialMove_0__true__false_ {
    pub unsafe fn move_from(_a0: *mut TailClobbererNonTrivialMove_0__true__false_) -> Self {
        let mut this = Self {};
        this
    }
}
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
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct TailClobbererNonTrivialMove_1__false__false_ {}
impl TailClobbererNonTrivialMove_1__false__false_ {
    pub unsafe fn move_from(_a0: *mut TailClobbererNonTrivialMove_1__false__false_) -> Self {
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
            (unsafe {
                let ___s: () = (&raw mut this as ());
                let ___n: () = ::std::mem::size_of::<BoolWithPadding>();
                memset_6(___s, 0_(), ___n)
            });
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
            (unsafe {
                let ___s: () = (&raw mut this as ());
                let ___n: () = ::std::mem::size_of::<IntWithoutPadding>();
                memset_6(___s, 0_(), ___n)
            });
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NotSwappable {}
const _: () = assert!(true, "std::is_swappable_v<std::expected<int, int>>");
const _: () = assert!(
    true,
    "!std::is_swappable_v<std::expected<NotSwappable, int>>"
);
const _: () = assert!(
    true,
    "!std::is_swappable_v<std::expected<int, NotSwappable>>"
);
pub unsafe fn swap_9(_a0: *mut NotMoveConstructible, _a1: *mut NotMoveConstructible) {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct NotMoveConstructible {}
const _: () = assert!(
    true,
    "!std::is_swappable_v<std::expected<NotMoveConstructible, int>>"
);
const _: () = assert!(
    true,
    "!std::is_swappable_v<std::expected<int, NotMoveConstructible>>"
);
pub unsafe fn swap_10(_a0: *mut MoveMayThrow, _a1: *mut MoveMayThrow) {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct MoveMayThrow {}
impl MoveMayThrow {}
const _: () = assert!(
    true,
    "std::is_swappable_v<std::expected<MoveMayThrow, int>>"
);
const _: () = assert!(
    true,
    "std::is_swappable_v<std::expected<int, MoveMayThrow>>"
);
const _: () = assert!(
    true,
    "!std::is_swappable_v<std::expected<MoveMayThrow, MoveMayThrow>>"
);
const _: () = assert!(true, "std::is_nothrow_swappable_v<std::expected<int, int>>");
const _: () = assert!(
    true,
    "!std::is_nothrow_swappable_v<std::expected<MoveMayThrow, int>>"
);
const _: () = assert!(
    true,
    "!std::is_nothrow_swappable_v<std::expected<int, MoveMayThrow>>"
);
pub unsafe fn swap_11(_a0: *mut SwapMayThrow, _a1: *mut SwapMayThrow) {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SwapMayThrow {}
const _: () = assert!(
    true,
    "!std::is_nothrow_swappable_v<std::expected<SwapMayThrow, int>>"
);
const _: () = assert!(
    true,
    "!std::is_nothrow_swappable_v<std::expected<int, SwapMayThrow>>"
);
pub unsafe fn test_12() -> () {
    {
        let mut x: () = unimplemented!();
        let mut y: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut x: () = unimplemented!();
        let mut y: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
        unimplemented!();
        unimplemented!();
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
        unimplemented!();
        unimplemented!();
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
        unimplemented!();
        unimplemented!();
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
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        {
            let mut x: () = unimplemented!();
            let mut y: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut x: () = unimplemented!();
            let mut y: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    return true;
}
pub unsafe fn testException_13() {}
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
    (unsafe { test_12() });
    const _: () = assert!(true, "test()");;
    (unsafe { testException_13() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
