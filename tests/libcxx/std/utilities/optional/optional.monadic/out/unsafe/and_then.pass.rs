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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CLVal {}
impl CLVal {
    pub unsafe fn operator_call_2(&mut self, _a0: *const i32) -> () {
        return unimplemented!();
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CRVal {}
impl CRVal {
    pub unsafe fn operator_call_4(&mut self, _a0: *const i32) -> () {
        return unimplemented!();
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CRefQual {}
impl CRefQual {
    pub unsafe fn operator_call_2(&self, mut _a0: i32) -> () {
        return unimplemented!();
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RVCRefQual {}
impl RVCRefQual {
    pub unsafe fn operator_call_4(&self, mut _a0: i32) -> () {
        return unimplemented!();
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NOCLVal {}
impl NOCLVal {
    pub unsafe fn operator_call_2(&mut self, _a0: *const i32) -> () {
        return unimplemented!();
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NOCRVal {}
impl NOCRVal {
    pub unsafe fn operator_call_4(&mut self, _a0: *const i32) -> () {
        return unimplemented!();
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NOCRefQual {}
impl NOCRefQual {
    pub unsafe fn operator_call_2(&self, mut _a0: i32) -> () {
        return unimplemented!();
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NORVCRefQual {}
impl NORVCRefQual {
    pub unsafe fn operator_call_4(&self, mut _a0: i32) -> () {
        return unimplemented!();
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
