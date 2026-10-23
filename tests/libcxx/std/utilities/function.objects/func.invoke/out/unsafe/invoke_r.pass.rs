extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() -> () {
    {
        let mut f: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
            unsafe {
                return ((i) + (3_()));
            }
        });
        unimplemented!();
    }
    {
        let mut f: FnPtr<fn(()) -> *mut ()> =
            FnPtr::<fn(()) -> *mut ()>::new(|mut _a0: ()| -> *mut () {
                unsafe {
                    return std::ptr::null_mut();
                }
            });
        const _: () = assert!(true, "can_invoke_r<char*, decltype(f), int>");;
        const _: () = assert!(true, "can_invoke_r<void*, decltype(f), int>");;
        const _: () = assert!(true, "can_invoke_r<void,  decltype(f), int>");;
        const _: () = assert!(!(false), "!can_invoke_r<char*, decltype(f), void*>");;
        const _: () = assert!(!(false), "!can_invoke_r<char*, decltype(f)>");;
        const _: () = assert!(!(false), "!can_invoke_r<int*,  decltype(f), int>");;
        const _: () = assert!(!(false), "!can_invoke_r<void,  decltype(f), void*>");;
    }
    {
        let mut f: FnPtr<fn(()) -> *mut ()> =
            FnPtr::<fn(()) -> *mut ()>::new(|mut _a0: ()| -> *mut () {
                unsafe {
                    return std::ptr::null_mut();
                }
            });
        let mut g: FnPtr<fn(()) -> *mut ()> =
            FnPtr::<fn(()) -> *mut ()>::new(|mut _a0: ()| -> *mut () {
                unsafe {
                    return std::ptr::null_mut();
                }
            });
        const _: () = assert!(true, "noexcept(std::invoke_r<char*>(f, 0))");;
        const _: () = assert!(true, "!noexcept(std::invoke_r<char*>(g, 0))");;
        const _: () = assert!(
            true,
            "!noexcept(std::invoke_r<ConversionNotNoexcept>(f, 0))"
        );;
        const _: () = assert!(
            true,
            "!noexcept(std::invoke_r<ConversionNotNoexcept>(g, 0))"
        );;
    }
    {
        let mut check: FnPtr<Generic> = lambda_unsafe!(
            Generic,
            {},
            || {
                let mut was_called: () = false;
                let mut f: FnPtr<fn(()) -> *mut ()> = lambda_unsafe!(
                    {
                        let was_called: *mut () = &mut was_called;
                    },
                    |mut _a0: ()| -> *mut () {
                        (*was_called) = true;
                        return std::ptr::null_mut();
                    }
                );
                unimplemented!();
                unimplemented!();
                const _: () = assert!(
                    true,
                    "std::is_void_v<decltype(std::invoke_r<CV_Void>(f, 3))>"
                );;
            },
            || {
                let mut was_called: () = false;
                let mut f: FnPtr<fn(()) -> *mut ()> = lambda_unsafe!(
                    {
                        let was_called: *mut () = &mut was_called;
                    },
                    |mut _a0: ()| -> *mut () {
                        (*was_called) = true;
                        return std::ptr::null_mut();
                    }
                );
                (unsafe {
                    let mut ___args: () = 3_();
                    invoke_r_2(&mut f, &mut ___args)
                });
                unimplemented!();
                const _: () = assert!(
                    true,
                    "std::is_void_v<decltype(std::invoke_r<CV_Void>(f, 3))>"
                );;
            }
        );
        (unsafe { check.spec::<fn()>(0).call() });
        (unsafe { check.spec::<fn()>(1).call() });
    }
    {
        {
            let mut was_called: () = false;
            let mut f: FnPtr<fn(NonCopyable_3)> = lambda_unsafe!(
                {
                    let was_called: *mut () = &mut was_called;
                },
                |mut _a0: NonCopyable_3| {
                    (*was_called) = true;
                }
            );
            unimplemented!();
            unimplemented!();
        }
        {
            let mut was_called: () = false;
            let mut f: FnPtr<fn(NonCopyable_3) -> ()> = lambda_unsafe!(
                {
                    let was_called: *mut () = &mut was_called;
                },
                |mut _a0: NonCopyable_3| -> () {
                    (*was_called) = true;
                    return 0_();
                }
            );
            &(unimplemented!());
            unimplemented!();
        }
        {
            let mut was_called: () = false;
            unimplemented!();
            unimplemented!();
        }
        {
            let mut was_called: () = false;
            &(unimplemented!());
            unimplemented!();
        }
    }
    {
        let mut f: FnPtr<fn() -> Convertible_6> =
            FnPtr::<fn() -> Convertible_6>::new(|| -> Convertible_6 {
                unsafe {
                    return <Convertible_6>::default();
                }
            });
        let mut result: () = unimplemented!();
        unimplemented!();
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConversionNotNoexcept_1 {}
impl ConversionNotNoexcept_1 {
    pub unsafe fn new(mut _a0: *mut ()) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct NonCopyable_3 {}
impl NonCopyable_3 {
    pub unsafe fn move_from(_a0: *mut NonCopyable_3) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MoveOnlyVoidFunction_4 {
    pub was_called: *mut (),
}
impl MoveOnlyVoidFunction_4 {
    pub unsafe fn operator_call(&mut self) {
        (*self.was_called) = true;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MoveOnlyIntFunction_5 {
    pub was_called: *mut (),
}
impl MoveOnlyIntFunction_5 {
    pub unsafe fn operator_call(&mut self) -> () {
        (*self.was_called) = true;
        return 0_();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Convertible_6 {}
impl Convertible_6 {
    pub unsafe fn to_(&self) -> () {
        return 42_();
    }
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
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
