extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_1(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_2(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_3(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_4(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_5(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_6(_a0: *const MoveOnly, _a1: *const MoveOnly) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct MoveOnly {
    data_: (),
}
impl MoveOnly {
    pub unsafe fn new(mut data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| unsafe { 1_() });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(x: *mut MoveOnly) -> Self {
        let mut this = Self { data_: (*x).data_ };
        (*x).data_ = 0_();
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut MoveOnly) -> *mut MoveOnly {
        self.data_ = (*x).data_;
        (*x).data_ = 0_();
        return &mut (*(self as *mut MoveOnly));
    }
    pub unsafe fn get(&self) -> () {
        return self.data_;
    }
    pub unsafe fn operator_add(&self, x: *const MoveOnly) -> MoveOnly {
        return MoveOnly::new({ Some(((self.data_) + ((*x).data_))) });
    }
    pub unsafe fn operator_mul(&self, x: *const MoveOnly) -> MoveOnly {
        return MoveOnly::new({ Some(((self.data_) * ((*x).data_))) });
    }
}
impl std::cmp::Ord for MoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_6(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        unsafe { MoveOnly::new(None) }
    }
}
pub unsafe fn operator_eq_7(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_8(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_9(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_10(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_11(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_12(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_13(_a0: *const TrivialMoveOnly, _a1: *const TrivialMoveOnly) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct TrivialMoveOnly {
    data_: (),
}
impl TrivialMoveOnly {
    pub unsafe fn new(mut data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| unsafe { 1_() });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(_a0: *mut TrivialMoveOnly) -> Self {
        let mut this = Self {
            data_: (*_a0).data_,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut TrivialMoveOnly) -> *mut TrivialMoveOnly {
        self.data_ = (*_a0).data_;
        return &mut (*(self as *mut TrivialMoveOnly));
    }
    pub unsafe fn get(&self) -> () {
        return self.data_;
    }
    pub unsafe fn operator_add(&self, x: *const TrivialMoveOnly) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({ Some(((self.data_) + ((*x).data_))) });
    }
    pub unsafe fn operator_mul(&self, x: *const TrivialMoveOnly) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({ Some(((self.data_) * ((*x).data_))) });
    }
}
impl std::cmp::Ord for TrivialMoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_13(
                self as *const TrivialMoveOnly,
                other as *const TrivialMoveOnly,
            )
        }
    }
}
impl std::cmp::PartialOrd for TrivialMoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for TrivialMoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_7(
                self as *const TrivialMoveOnly,
                other as *const TrivialMoveOnly,
            )
        }
    }
}
impl std::cmp::Eq for TrivialMoveOnly {}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        unsafe { TrivialMoveOnly::new(None) }
    }
}
pub unsafe fn swap_14(x: *mut ADLSwap, y: *mut ADLSwap) {
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
pub unsafe fn operator_eq_15(_a0: *const TailClobberer_0_, _a1: *const TailClobberer_0_) -> () {
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
            operator_eq_15(
                self as *const TailClobberer_0_,
                other as *const TailClobberer_0_,
            )
        }
    }
}
impl std::cmp::Eq for TailClobberer_0_ {}
pub unsafe fn operator_eq_16(_a0: *const TailClobberer_1_, _a1: *const TailClobberer_1_) -> () {
    return (((*_a0).b) == ((*_a1).b));
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe)]
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
    pub unsafe fn copy_from(_a0: *const TailClobberer_1_) -> Self {
        let mut this = TailClobberer_1_::new_1();
        this
    }
    pub unsafe fn move_from(_a0: *mut TailClobberer_1_) -> Self {
        let mut this = Self { b: (*_a0).b };
        this
    }
    pub unsafe fn new_2(mut _a0: ()) -> Self {
        let mut this = TailClobberer_1_::new_1();
        this
    }
}
impl std::cmp::PartialEq for TailClobberer_1_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_16(
                self as *const TailClobberer_1_,
                other as *const TailClobberer_1_,
            )
        }
    }
}
impl std::cmp::Eq for TailClobberer_1_ {}
impl Clone for TailClobberer_1_ {
    fn clone(&self) -> Self {
        unsafe { TailClobberer_1_::copy_from(self as *const TailClobberer_1_) }
    }
}
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
const _: () = assert!(
    true,
    "std::is_constructible_v<std::expected<void, int>, std::unexpect_t>"
);
const _: () = assert!(
    true,
    "std::is_constructible_v<std::expected<void, int>, std::unexpect_t, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct foo {}
const _: () = assert!(
    true,
    "!std::is_constructible_v<std::expected<void, foo>, std::unexpect_t, int>"
);
const _: () = assert!(true, "ImplicitlyConstructible<int, int>");
const _: () = assert!(
    !(false),
    "!ImplicitlyConstructible<std::expected<void, int>, std::unexpect_t>"
);
const _: () = assert!(
    !(false),
    "!ImplicitlyConstructible<std::expected<void, int>, std::unexpect_t, int>"
);
pub unsafe fn operator_eq_17(mi: *const CopyOnly, mut ii: ()) -> () {
    return (((*mi).i) == (ii));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CopyOnly {
    pub i: (),
}
impl CopyOnly {
    pub unsafe fn new(mut ii: ()) -> Self {
        let mut this = Self { i: ii };
        this
    }
}
pub unsafe fn testInt_18() {
    let mut e: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testInt_19() {
    let mut e: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testInt_20() {
    let mut e: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testInt_21() {
    let mut e: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testLValue_22() {
    let mut t: () = 5_();
    let mut e: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testLValue_23() {
    let mut t: CopyOnly = CopyOnly::new({ 5_() });
    let mut e: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testLValue_24() {
    let mut t: TailClobberer_1_ = TailClobberer_1_::new_2({ 5_() });
    let mut e: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRValue_25() {
    let mut e: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRValue_26() {
    let mut e: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRValue_27() {
    let mut e: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_28() -> () {
    (unsafe { testInt_18() });
    (unsafe { testInt_19() });
    (unsafe { testInt_20() });
    (unsafe { testInt_21() });
    (unsafe { testLValue_22() });
    (unsafe { testLValue_23() });
    (unsafe { testLValue_24() });
    (unsafe { testRValue_25() });
    (unsafe { testRValue_26() });
    (unsafe { testRValue_27() });
    {
        let mut e: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut e: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut e: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub unsafe fn testException_29() {}
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
    (unsafe { test_28() });
    const _: () = assert!(true, "test()");;
    (unsafe { testException_29() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
