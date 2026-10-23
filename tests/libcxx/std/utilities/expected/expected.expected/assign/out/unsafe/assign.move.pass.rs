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
pub struct TracedBase_false__state {
    pub copyCtorCalled: (),
    pub copyAssignCalled: (),
    pub moveCtorCalled: (),
    pub moveAssignCalled: (),
    pub dtorCalled: (),
}
impl Default for TracedBase_false__state {
    fn default() -> Self {
        unsafe {
            TracedBase_false__state {
                copyCtorCalled: false,
                copyAssignCalled: false,
                moveCtorCalled: false,
                moveAssignCalled: false,
                dtorCalled: false,
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, DestructorUnsafe)]
pub struct TracedBase_false__true_ {
    pub state_: *mut TracedBase_false__state,
    pub copiedFromInt: (),
    pub movedFromInt: (),
    pub copiedFromTmp: (),
    pub movedFromTmp: (),
    pub data_: (),
}
impl TracedBase_false__true_ {
    pub unsafe fn new_1(s: *mut TracedBase_false__state, mut ii: ()) -> Self {
        let mut this = Self {
            state_: (s),
            copiedFromInt: false,
            movedFromInt: false,
            copiedFromTmp: false,
            movedFromTmp: false,
            data_: ii,
        };
        this
    }
    pub unsafe fn move_from(other: *mut TracedBase_false__true_) -> Self {
        let mut this = Self {
            state_: (*other).state_,
            copiedFromInt: false,
            movedFromInt: false,
            copiedFromTmp: false,
            movedFromTmp: false,
            data_: (*other).data_,
        };
        if !(this.state_).is_null() {
            (*this.state_).moveCtorCalled = true;
        } else {
            this.movedFromTmp = true;
        }
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        other: *mut TracedBase_false__true_,
    ) -> *mut TracedBase_false__true_ {
        self.data_ = (*other).data_;
        (*self.state_).moveAssignCalled = true;
        return &mut (*(self as *mut TracedBase_false__true_));
    }
    pub unsafe fn destructor(&mut self) {
        if !(self.state_).is_null() {
            (*self.state_).dtorCalled = true;
        }
    }
}
impl Default for TracedBase_false__true_ {
    fn default() -> Self {
        unsafe {
            TracedBase_false__true_ {
                state_: std::ptr::null_mut(),
                copiedFromInt: false,
                movedFromInt: false,
                copiedFromTmp: false,
                movedFromTmp: false,
                data_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct TracedBase_true__state {
    pub copyCtorCalled: (),
    pub copyAssignCalled: (),
    pub moveCtorCalled: (),
    pub moveAssignCalled: (),
    pub dtorCalled: (),
}
impl Default for TracedBase_true__state {
    fn default() -> Self {
        unsafe {
            TracedBase_true__state {
                copyCtorCalled: false,
                copyAssignCalled: false,
                moveCtorCalled: false,
                moveAssignCalled: false,
                dtorCalled: false,
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, DestructorUnsafe)]
pub struct TracedBase_true__true_ {
    pub state_: *mut TracedBase_true__state,
    pub copiedFromInt: (),
    pub movedFromInt: (),
    pub copiedFromTmp: (),
    pub movedFromTmp: (),
    pub data_: (),
}
impl TracedBase_true__true_ {
    pub unsafe fn new_1(s: *mut TracedBase_true__state, mut ii: ()) -> Self {
        let mut this = Self {
            state_: (s),
            copiedFromInt: false,
            movedFromInt: false,
            copiedFromTmp: false,
            movedFromTmp: false,
            data_: ii,
        };
        this
    }
    pub unsafe fn move_from(other: *mut TracedBase_true__true_) -> Self {
        let mut this = Self {
            state_: (*other).state_,
            copiedFromInt: false,
            movedFromInt: false,
            copiedFromTmp: false,
            movedFromTmp: false,
            data_: (*other).data_,
        };
        if !(this.state_).is_null() {
            (*this.state_).moveCtorCalled = true;
        } else {
            this.movedFromTmp = true;
        }
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        other: *mut TracedBase_true__true_,
    ) -> *mut TracedBase_true__true_ {
        self.data_ = (*other).data_;
        (*self.state_).moveAssignCalled = true;
        return &mut (*(self as *mut TracedBase_true__true_));
    }
    pub unsafe fn destructor(&mut self) {
        if !(self.state_).is_null() {
            (*self.state_).dtorCalled = true;
        }
    }
}
impl Default for TracedBase_true__true_ {
    fn default() -> Self {
        unsafe {
            TracedBase_true__true_ {
                state_: std::ptr::null_mut(),
                copiedFromInt: false,
                movedFromInt: false,
                copiedFromTmp: false,
                movedFromTmp: false,
                data_: unimplemented!(),
            }
        }
    }
}
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
pub unsafe fn operator_eq_2(_a0: *const TailClobberer_0_, _a1: *const TailClobberer_0_) -> () {
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
            operator_eq_2(
                self as *const TailClobberer_0_,
                other as *const TailClobberer_0_,
            )
        }
    }
}
impl std::cmp::Eq for TailClobberer_0_ {}
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
                memset_3(___s, 0_(), ___n)
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
                memset_3(___s, 0_(), ___n)
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
#[derive(FnPtrArg)]
pub struct CheckForInvalidWrites_true__false_ {
    pub please_dont_overwrite_me: [(); 7],
}
impl CheckForInvalidWrites_true__false_ {
    pub unsafe fn new_1(mut _a0: ()) -> Self {
        let mut this = Self {
            please_dont_overwrite_me: [true, true, true, true, true, true, true],
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        other: *mut CheckForInvalidWrites_true__false_,
    ) -> *mut CheckForInvalidWrites_true__false_ {
        unimplemented!();
        return &mut (*(self as *mut CheckForInvalidWrites_true__false_));
    }
    pub unsafe fn check(&mut self) -> () {
        'loop_: for i in IterRange::new(
            self.please_dont_overwrite_me.as_ptr(),
            self.please_dont_overwrite_me
                .as_ptr()
                .offset((7_()) as isize),
        ) {
            let mut i: () = (*i);
            if !(i) {
                return false;
            }
        }
        return true;
    }
}
impl Default for CheckForInvalidWrites_true__false_ {
    fn default() -> Self {
        unsafe {
            CheckForInvalidWrites_true__false_ {
                please_dont_overwrite_me: [true, true, true, true, true, true, true],
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct CheckForInvalidWrites_false__false_ {
    pub please_dont_overwrite_me: [(); 7],
}
impl CheckForInvalidWrites_false__false_ {
    pub unsafe fn new_1(mut _a0: ()) -> Self {
        let mut this = Self {
            please_dont_overwrite_me: [true, true, true, true, true, true, true],
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        other: *mut CheckForInvalidWrites_false__false_,
    ) -> *mut CheckForInvalidWrites_false__false_ {
        unimplemented!();
        return &mut (*(self as *mut CheckForInvalidWrites_false__false_));
    }
    pub unsafe fn check(&mut self) -> () {
        'loop_: for i in IterRange::new(
            self.please_dont_overwrite_me.as_ptr(),
            self.please_dont_overwrite_me
                .as_ptr()
                .offset((7_()) as isize),
        ) {
            let mut i: () = (*i);
            if !(i) {
                return false;
            }
        }
        return true;
    }
}
impl Default for CheckForInvalidWrites_false__false_ {
    fn default() -> Self {
        unsafe {
            CheckForInvalidWrites_false__false_ {
                please_dont_overwrite_me: [true, true, true, true, true, true, true],
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveAssignUnsafe, Default)]
pub struct NotMoveConstructible {}
impl NotMoveConstructible {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut NotMoveConstructible,
    ) -> *mut NotMoveConstructible {
        return &mut (*(self as *mut NotMoveConstructible));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct NotMoveAssignable {}
impl NotMoveAssignable {
    pub unsafe fn move_from(_a0: *mut NotMoveAssignable) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct MoveCtorMayThrow {}
impl MoveCtorMayThrow {
    pub unsafe fn move_from(_a0: *mut MoveCtorMayThrow) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut MoveCtorMayThrow) -> *mut MoveCtorMayThrow {
        return &mut (*(self as *mut MoveCtorMayThrow));
    }
}
const _: () = assert!(true, "std::is_move_assignable_v<std::expected<int, int>>");
const _: () = assert!(
    true,
    "!std::is_move_assignable_v<std::expected<NotMoveAssignable, int>>"
);
const _: () = assert!(
    true,
    "!std::is_move_assignable_v<std::expected<NotMoveConstructible, int>>"
);
const _: () = assert!(
    true,
    "!std::is_move_assignable_v<std::expected<int, NotMoveAssignable>>"
);
const _: () = assert!(
    true,
    "!std::is_move_assignable_v<std::expected<int, NotMoveConstructible>>"
);
const _: () = assert!(
    true,
    "std::is_move_assignable_v<std::expected<MoveCtorMayThrow, int>>"
);
const _: () = assert!(
    true,
    "std::is_move_assignable_v<std::expected<int, MoveCtorMayThrow>>"
);
const _: () = assert!(
    true,
    "!std::is_move_assignable_v<std::expected<MoveCtorMayThrow, MoveCtorMayThrow>>"
);
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct MoveAssignMayThrow {}
impl MoveAssignMayThrow {
    pub unsafe fn move_from(_a0: *mut MoveAssignMayThrow) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut MoveAssignMayThrow) -> *mut MoveAssignMayThrow {
        return &mut (*(self as *mut MoveAssignMayThrow));
    }
}
const _: () = assert!(
    true,
    "std::is_nothrow_move_assignable_v<std::expected<int, int>>"
);
const _: () = assert!(
    true,
    "!std::is_nothrow_move_assignable_v<std::expected<MoveAssignMayThrow, int>>"
);
const _: () = assert!(
    true,
    "!std::is_nothrow_move_assignable_v<std::expected<MoveCtorMayThrow, int>>"
);
const _: () = assert!(
    true,
    "!std::is_nothrow_move_assignable_v<std::expected<int, MoveAssignMayThrow>>"
);
const _: () = assert!(
    true,
    "!std::is_nothrow_move_assignable_v<std::expected<int, MoveCtorMayThrow>>"
);
pub unsafe fn test_4() -> () {
    {
        let mut oldState: TracedBase_false__state = <TracedBase_false__state>::default();
        let mut newState: TracedBase_false__state = <TracedBase_false__state>::default();
        let mut e1: () = unimplemented!();
        let mut e2: () = unimplemented!();
        let x: *mut () = &mut (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(x), std::expected<Traced, int>&>"
        );;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut oldState: TracedBase_true__state = <TracedBase_true__state>::default();
        let mut newState: TracedBase_false__state = <TracedBase_false__state>::default();
        let mut e1: () = unimplemented!();
        let mut e2: () = unimplemented!();
        let x: *mut () = &mut (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(x), std::expected<TracedNoexcept, Traced>&>"
        );;
        unimplemented!();
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
        let mut oldState: TracedBase_false__state = <TracedBase_false__state>::default();
        let mut newState: TracedBase_true__state = <TracedBase_true__state>::default();
        let mut e1: () = unimplemented!();
        let mut e2: () = unimplemented!();
        let x: *mut () = &mut (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(x), std::expected<Traced, TracedNoexcept>&>"
        );;
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
        let mut oldState: TracedBase_true__state = <TracedBase_true__state>::default();
        let mut newState: TracedBase_false__state = <TracedBase_false__state>::default();
        let mut e1: () = unimplemented!();
        let mut e2: () = unimplemented!();
        let x: *mut () = &mut (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(x), std::expected<Traced, TracedNoexcept>&>"
        );;
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
        let mut oldState: TracedBase_false__state = <TracedBase_false__state>::default();
        let mut newState: TracedBase_true__state = <TracedBase_true__state>::default();
        let mut e1: () = unimplemented!();
        let mut e2: () = unimplemented!();
        let x: *mut () = &mut (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(x), std::expected<TracedNoexcept, Traced>&>"
        );;
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
        let mut oldState: TracedBase_false__state = <TracedBase_false__state>::default();
        let mut newState: TracedBase_false__state = <TracedBase_false__state>::default();
        let mut e1: () = unimplemented!();
        let mut e2: () = unimplemented!();
        let x: *mut () = &mut (*unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(x), std::expected<int, Traced>&>"
        );;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        {
            let mut e1: CheckForInvalidWrites_true__false_ =
                CheckForInvalidWrites_true__false_::new_1({ unimplemented!() });
            let mut e2: CheckForInvalidWrites_true__false_ =
                <CheckForInvalidWrites_true__false_>::default();
            (unsafe { CheckForInvalidWrites_true__false_::move_assign(&mut e1, &mut e2) });
            unimplemented!();
            unimplemented!();
        }
        {
            let mut e1: CheckForInvalidWrites_false__false_ =
                CheckForInvalidWrites_false__false_::new_1({ unimplemented!() });
            let mut e2: CheckForInvalidWrites_false__false_ =
                <CheckForInvalidWrites_false__false_>::default();
            (unsafe { CheckForInvalidWrites_false__false_::move_assign(&mut e1, &mut e2) });
            unimplemented!();
            unimplemented!();
        }
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
