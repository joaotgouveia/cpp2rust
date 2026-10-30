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
            unimplemented!();
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
            operator_eq_1(
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Foo {}
const _: () = assert!(!(false), "!HasValueNoexcept<Foo>");
const _: () = assert!(true, "HasValueNoexcept<std::expected<int, int>>");
const _: () = assert!(true, "HasValueNoexcept<const std::expected<int, int>>");
pub unsafe fn test_2() -> () {
    {
        let e: () = unimplemented!();
        unimplemented!();
    }
    {
        let e: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut f1: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
            unsafe {
                return unimplemented!();
            }
        });
        let mut f2: FnPtr<fn() -> ()> = lambda_unsafe!(
            {
                let f1: *mut FnPtr<fn() -> ()> = &mut f1;
            },
            || -> () {
                return (unsafe {
                    let mut ___f: FnPtr<Generic> =
                        lambda_unsafe!(Generic, {}, |mut _a0: ()| -> () {
                            return 0_();
                        });
                    (unsafe { (*f1).call() }).transform_error(&mut ___f)
                });
            }
        );
        let mut e: () = (unsafe { f2.call() });
        unimplemented!();
    }
    {
        let e: () = unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
    }
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
    (unsafe { test_2() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
