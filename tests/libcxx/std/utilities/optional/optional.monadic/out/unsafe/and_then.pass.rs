extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_optional_int___std_optional_int__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LVal {}
impl LVal {
    pub unsafe fn operator_call_1(&mut self, _a0: *mut i32) -> () {
        return unimplemented!();
    }
}
impl Callable1<*mut i32, ()> for LVal {
    fn call(&self, a1: *mut i32) -> () {
        unsafe { LVal::operator_call_1(&mut *(&raw const *self).cast_mut(), a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CLVal {}
impl CLVal {
    pub unsafe fn operator_call_2(&mut self, _a0: *const i32) -> () {
        return unimplemented!();
    }
}
impl Callable1<*const i32, ()> for CLVal {
    fn call(&self, a1: *const i32) -> () {
        unsafe { CLVal::operator_call_2(&mut *(&raw const *self).cast_mut(), a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RVal {}
impl RVal {
    pub unsafe fn operator_call_3(&mut self, _a0: *mut i32) -> () {
        return unimplemented!();
    }
}
impl Callable1<*mut i32, ()> for RVal {
    fn call(&self, a1: *mut i32) -> () {
        unsafe { RVal::operator_call_3(&mut *(&raw const *self).cast_mut(), a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CRVal {}
impl CRVal {
    pub unsafe fn operator_call_4(&mut self, _a0: *const i32) -> () {
        return unimplemented!();
    }
}
impl Callable1<*const i32, ()> for CRVal {
    fn call(&self, a1: *const i32) -> () {
        unsafe { CRVal::operator_call_4(&mut *(&raw const *self).cast_mut(), a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RefQual {}
impl RefQual {
    pub unsafe fn operator_call_1(&mut self, mut _a0: i32) -> () {
        return unimplemented!();
    }
}
impl Callable1<i32, ()> for RefQual {
    fn call(&self, a1: i32) -> () {
        unsafe { RefQual::operator_call_1(&mut *(&raw const *self).cast_mut(), a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CRefQual {}
impl CRefQual {
    pub unsafe fn operator_call_2(&self, mut _a0: i32) -> () {
        return unimplemented!();
    }
}
impl Callable1<i32, ()> for CRefQual {
    fn call(&self, a1: i32) -> () {
        unsafe { CRefQual::operator_call_2(self, a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RVRefQual {}
impl RVRefQual {
    pub unsafe fn operator_call_3(&mut self, mut _a0: i32) -> () {
        return unimplemented!();
    }
}
impl Callable1<i32, ()> for RVRefQual {
    fn call(&self, a1: i32) -> () {
        unsafe { RVRefQual::operator_call_3(&mut *(&raw const *self).cast_mut(), a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RVCRefQual {}
impl RVCRefQual {
    pub unsafe fn operator_call_4(&self, mut _a0: i32) -> () {
        return unimplemented!();
    }
}
impl Callable1<i32, ()> for RVCRefQual {
    fn call(&self, a1: i32) -> () {
        unsafe { RVCRefQual::operator_call_4(self, a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NOLVal {}
impl NOLVal {
    pub unsafe fn operator_call_1(&mut self, _a0: *mut i32) -> () {
        return unimplemented!();
    }
}
impl Callable1<*mut i32, ()> for NOLVal {
    fn call(&self, a1: *mut i32) -> () {
        unsafe { NOLVal::operator_call_1(&mut *(&raw const *self).cast_mut(), a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NOCLVal {}
impl NOCLVal {
    pub unsafe fn operator_call_2(&mut self, _a0: *const i32) -> () {
        return unimplemented!();
    }
}
impl Callable1<*const i32, ()> for NOCLVal {
    fn call(&self, a1: *const i32) -> () {
        unsafe { NOCLVal::operator_call_2(&mut *(&raw const *self).cast_mut(), a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NORVal {}
impl NORVal {
    pub unsafe fn operator_call_3(&mut self, _a0: *mut i32) -> () {
        return unimplemented!();
    }
}
impl Callable1<*mut i32, ()> for NORVal {
    fn call(&self, a1: *mut i32) -> () {
        unsafe { NORVal::operator_call_3(&mut *(&raw const *self).cast_mut(), a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NOCRVal {}
impl NOCRVal {
    pub unsafe fn operator_call_4(&mut self, _a0: *const i32) -> () {
        return unimplemented!();
    }
}
impl Callable1<*const i32, ()> for NOCRVal {
    fn call(&self, a1: *const i32) -> () {
        unsafe { NOCRVal::operator_call_4(&mut *(&raw const *self).cast_mut(), a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NORefQual {}
impl NORefQual {
    pub unsafe fn operator_call_1(&mut self, mut _a0: i32) -> () {
        return unimplemented!();
    }
}
impl Callable1<i32, ()> for NORefQual {
    fn call(&self, a1: i32) -> () {
        unsafe { NORefQual::operator_call_1(&mut *(&raw const *self).cast_mut(), a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NOCRefQual {}
impl NOCRefQual {
    pub unsafe fn operator_call_2(&self, mut _a0: i32) -> () {
        return unimplemented!();
    }
}
impl Callable1<i32, ()> for NOCRefQual {
    fn call(&self, a1: i32) -> () {
        unsafe { NOCRefQual::operator_call_2(self, a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NORVRefQual {}
impl NORVRefQual {
    pub unsafe fn operator_call_3(&mut self, mut _a0: i32) -> () {
        return unimplemented!();
    }
}
impl Callable1<i32, ()> for NORVRefQual {
    fn call(&self, a1: i32) -> () {
        unsafe { NORVRefQual::operator_call_3(&mut *(&raw const *self).cast_mut(), a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NORVCRefQual {}
impl NORVCRefQual {
    pub unsafe fn operator_call_4(&self, mut _a0: i32) -> () {
        return unimplemented!();
    }
}
impl Callable1<i32, ()> for NORVCRefQual {
    fn call(&self, a1: i32) -> () {
        unsafe { NORVCRefQual::operator_call_4(self, a1) }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct NoCopy {}
impl NoCopy {
    pub unsafe fn copy_from(_a0: *const NoCopy) -> Self {
        let mut this = Self {};
        assert!(false);
        this
    }
    pub unsafe fn operator_call(&mut self, _a0: *const NoCopy) -> () {
        return unimplemented!();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const NoCopy) -> *mut NoCopy {
        return &mut (*(self as *mut NoCopy));
    }
}
impl Callable1<*const NoCopy, ()> for NoCopy {
    fn call(&self, a1: *const NoCopy) -> () {
        unsafe { NoCopy::operator_call(&mut *(&raw const *self).cast_mut(), a1) }
    }
}
impl Clone for NoCopy {
    fn clone(&self) -> Self {
        unsafe { NoCopy::copy_from(self as *const NoCopy) }
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
pub unsafe fn test_val_types_1() {
    {
        {
            let mut i: () = unimplemented!();
            assert!((unimplemented!() as bool));
            assert!(unimplemented!());
            const _: () = assert!(true, "");;
        }
        {
            let mut i: () = unimplemented!();
            let mut l: RefQual = <RefQual>::default();
            assert!((unimplemented!() as bool));
            let mut nl: NORefQual = <NORefQual>::default();
            assert!(unimplemented!());
            const _: () = assert!(true, "");;
        }
    }
    {
        {
            let i: () = unimplemented!();
            assert!((unimplemented!() as bool));
            assert!(unimplemented!());
            const _: () = assert!(true, "");;
        }
        {
            let i: () = unimplemented!();
            let l: CRefQual = <CRefQual>::default();
            assert!((unimplemented!() as bool));
            let nl: NOCRefQual = <NOCRefQual>::default();
            assert!(unimplemented!());
            const _: () = assert!(true, "");;
        }
    }
    {
        {
            let mut i: () = unimplemented!();
            assert!((unimplemented!() as bool));
            assert!(unimplemented!());
            const _: () = assert!(true, "");;
        }
        {
            let mut i: () = unimplemented!();
            assert!((unimplemented!() as bool));
            assert!(unimplemented!());
            const _: () = assert!(true, "");;
        }
    }
    {
        {
            let i: () = unimplemented!();
            assert!((unimplemented!() as bool));
            assert!(unimplemented!());
            const _: () = assert!(true, "");;
        }
        {
            let i: () = unimplemented!();
            let l: RVCRefQual = <RVCRefQual>::default();
            assert!((unimplemented!() as bool));
            let nl: NORVCRefQual = <NORVCRefQual>::default();
            assert!(unimplemented!());
            const _: () = assert!(true, "");;
        }
    }
}
pub unsafe fn test_sfinae_2() {
    let mut opt: () = unimplemented!();
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
    &(unimplemented!());
}
pub unsafe fn test_3() -> bool {
    (unsafe { test_val_types_1() });
    let mut opt: () = unimplemented!();
    let copt: *const () = &opt;
    let never_called: FnPtr<fn(i32) -> ()> = FnPtr::<fn(i32) -> ()>::new(|mut _a0: i32| -> () {
        unsafe {
            assert!(false);
            return unimplemented!();
        }
    });
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    let mut nc: () = unimplemented!();
    let cnc: *const () = &nc;
    &(unimplemented!());
    &(unimplemented!());
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
    (unsafe { test_3() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
