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
                memset_2(___s, 0_(), ___n)
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
                memset_2(___s, 0_(), ___n)
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
pub struct LVal {}
impl LVal {
    pub unsafe fn operator_call_1(&mut self, _a0: *mut ()) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CLVal {}
impl CLVal {
    pub unsafe fn operator_call_2(&mut self, _a0: *const ()) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RVal {}
impl RVal {
    pub unsafe fn operator_call_3(&mut self, _a0: *mut ()) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CRVal {}
impl CRVal {
    pub unsafe fn operator_call_4(&mut self, _a0: *const ()) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RefQual {}
impl RefQual {
    pub unsafe fn operator_call_1(&mut self, mut _a0: ()) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CRefQual {}
impl CRefQual {
    pub unsafe fn operator_call_2(&self, mut _a0: ()) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RVRefQual {}
impl RVRefQual {
    pub unsafe fn operator_call_3(&mut self, mut _a0: ()) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RVCRefQual {}
impl RVCRefQual {
    pub unsafe fn operator_call_4(&self, mut _a0: ()) -> () {
        return unimplemented!();
    }
}
const _: () = assert!(
    !(false),
    "!has_or_else<const std::expected<std::unique_ptr<int>, int>&, int()>"
);
const _: () = assert!(
    !(false),
    "!has_or_else<const std::expected<std::unique_ptr<int>, int>&&, int()>"
);
const _: () = assert!(
    true,
    "has_or_else<std::expected<int, MoveOnlyErrorType>&, std::expected<int, int>(MoveOnlyErrorType &)>"
);
const _: () = assert!(
    true,
    "has_or_else<const std::expected<int, MoveOnlyErrorType>&, std::expected<int, int>(const MoveOnlyErrorType &)>"
);
const _: () = assert!(
    true,
    "has_or_else<std::expected<int, MoveOnlyErrorType>&&, std::expected<int, int>(MoveOnlyErrorType&&)>"
);
const _: () = assert!(
    true,
    "has_or_else<const std::expected<int, MoveOnlyErrorType>&&, std::expected<int, int>(const MoveOnlyErrorType&&)>"
);
pub unsafe fn test_val_types_3() {
    {
        {
            let mut e: () = unimplemented!();
            let mut val: () = unimplemented!();
            unimplemented!();
        }
        {
            let mut e: () = unimplemented!();
            let mut l: RefQual = <RefQual>::default();
            let mut val: () = unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let e: () = unimplemented!();
            let mut val: () = unimplemented!();
            unimplemented!();
        }
        {
            let e: () = unimplemented!();
            let l: CRefQual = <CRefQual>::default();
            let mut val: () = unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut e: () = unimplemented!();
            let mut val: () = (unsafe {
                let mut ___f: RVal = <RVal>::default();
                (*unimplemented!()).or_else(&mut ___f)
            });
            unimplemented!();
        }
        {
            let mut e: () = unimplemented!();
            let mut val: () = (unsafe {
                let mut ___f: RVRefQual = <RVRefQual>::default();
                (*unimplemented!()).or_else(&mut ___f)
            });
            unimplemented!();
        }
    }
    {
        {
            let e: () = unimplemented!();
            let mut val: () = (unsafe {
                let mut ___f: CRVal = <CRVal>::default();
                (*unimplemented!()).or_else(&mut ___f)
            });
            unimplemented!();
        }
        {
            let e: () = unimplemented!();
            let l: RVCRefQual = <RVCRefQual>::default();
            let mut val: () = (unsafe { (*unimplemented!()).or_else(&(*unimplemented!())) });
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NonConst {}
impl NonConst {
    pub unsafe fn non_const(&mut self) -> () {
        return unimplemented!();
    }
}
pub unsafe fn test_sfinae_4() {
    let mut e: () = unimplemented!();
    let mut l: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |x: *mut NonConst| -> () {
            return (unsafe { NonConst::non_const(&mut (*x)) });
        },
        |x: *mut NonConst| -> () {
            return (unsafe { NonConst::non_const(&mut (*x)) });
        }
    );
    &(unimplemented!());
    &(unsafe { (*unimplemented!()).or_else(&mut l) });
}
pub unsafe fn test_move_only_error_type_5() {
    {
        let mut e: () = unimplemented!();
        let mut l: FnPtr<fn(*mut MoveOnlyErrorType) -> ()> =
            FnPtr::<fn(*mut MoveOnlyErrorType) -> ()>::new(|_a0: *mut MoveOnlyErrorType| -> () {
                unsafe {
                    return unimplemented!();
                }
            });
        &(unimplemented!());
    }
    {
        let e: () = unimplemented!();
        let mut l: FnPtr<fn(*const MoveOnlyErrorType) -> ()> =
            FnPtr::<fn(*const MoveOnlyErrorType) -> ()>::new(
                |_a0: *const MoveOnlyErrorType| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                },
            );
        &(unimplemented!());
    }
    {
        let mut e: () = unimplemented!();
        let mut l: FnPtr<fn(*mut MoveOnlyErrorType) -> ()> =
            FnPtr::<fn(*mut MoveOnlyErrorType) -> ()>::new(|_a0: *mut MoveOnlyErrorType| -> () {
                unsafe {
                    return unimplemented!();
                }
            });
        &(unsafe { (*unimplemented!()).or_else(&mut l) });
    }
    {
        let e: () = unimplemented!();
        let mut l: FnPtr<fn(*const MoveOnlyErrorType) -> ()> =
            FnPtr::<fn(*const MoveOnlyErrorType) -> ()>::new(
                |_a0: *const MoveOnlyErrorType| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                },
            );
        &(unsafe { (*unimplemented!()).or_else(&mut l) });
    }
}
pub unsafe fn test_6() -> () {
    (unsafe { test_sfinae_4() });
    (unsafe { test_val_types_3() });
    (unsafe { test_move_only_error_type_5() });
    let mut e: () = unimplemented!();
    let ce: *const () = &e;
    let never_called: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut _a0: ()| -> () {
        unsafe {
            unimplemented!();
            return unimplemented!();
        }
    });
    &(unimplemented!());
    &(unsafe { (*unimplemented!()).or_else(&never_called) });
    &(unimplemented!());
    &(unsafe {
        let ___f: *const FnPtr<fn(()) -> ()> = &never_called;
        (*unimplemented!()).or_else(___f)
    });
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
    (unsafe { test_6() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
