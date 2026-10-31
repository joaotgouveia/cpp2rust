extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() -> bool {
    {
        let mut f: FnPtr<fn(i32) -> i32> = FnPtr::<fn(i32) -> i32>::new(|mut i: i32| -> i32 {
            unsafe {
                return ((i) + (3));
            }
        });
        assert!(((unimplemented!()) == (7)));
    }
    {
        let mut f: FnPtr<fn(i32) -> *mut libc::c_char> =
            FnPtr::<fn(i32) -> *mut libc::c_char>::new(|mut _a0: i32| -> *mut libc::c_char {
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
        let mut f: FnPtr<fn(i32) -> *mut libc::c_char> =
            FnPtr::<fn(i32) -> *mut libc::c_char>::new(|mut _a0: i32| -> *mut libc::c_char {
                unsafe {
                    return std::ptr::null_mut();
                }
            });
        let mut g: FnPtr<fn(i32) -> *mut libc::c_char> =
            FnPtr::<fn(i32) -> *mut libc::c_char>::new(|mut _a0: i32| -> *mut libc::c_char {
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
                let mut was_called: bool = false;
                let mut f: FnPtr<fn(i32) -> *mut libc::c_char> = lambda_unsafe!(
                    {
                        let was_called: *mut bool = &mut was_called;
                    },
                    |mut _a0: i32| -> *mut libc::c_char {
                        (*was_called) = true;
                        return std::ptr::null_mut();
                    }
                );
                unimplemented!();
                assert!(was_called);
                const _: () = assert!(
                    true,
                    "std::is_void_v<decltype(std::invoke_r<CV_Void>(f, 3))>"
                );;
            },
            || {
                let mut was_called: bool = false;
                let mut f: FnPtr<fn(i32) -> *mut libc::c_char> = lambda_unsafe!(
                    {
                        let was_called: *mut bool = &mut was_called;
                    },
                    |mut _a0: i32| -> *mut libc::c_char {
                        (*was_called) = true;
                        return std::ptr::null_mut();
                    }
                );
                unimplemented!();
                assert!(was_called);
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
            let mut was_called: bool = false;
            let mut f: FnPtr<fn(NonCopyable_2)> = lambda_unsafe!(
                {
                    let was_called: *mut bool = &mut was_called;
                },
                |mut _a0: NonCopyable_2| {
                    (*was_called) = true;
                }
            );
            unimplemented!();
            assert!(was_called);
        }
        {
            let mut was_called: bool = false;
            let mut f: FnPtr<fn(NonCopyable_2) -> i32> = lambda_unsafe!(
                {
                    let was_called: *mut bool = &mut was_called;
                },
                |mut _a0: NonCopyable_2| -> i32 {
                    (*was_called) = true;
                    return 0;
                }
            );
            &(unimplemented!());
            assert!(was_called);
        }
        {
            let mut was_called: bool = false;
            unimplemented!();
            assert!(was_called);
        }
        {
            let mut was_called: bool = false;
            &(unimplemented!());
            assert!(was_called);
        }
    }
    {
        let mut f: FnPtr<fn() -> Convertible_5> =
            FnPtr::<fn() -> Convertible_5>::new(|| -> Convertible_5 {
                unsafe {
                    return <Convertible_5>::default();
                }
            });
        let mut result: i32 = unimplemented!();
        assert!(((result) == (42)));
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(*mut libc::c_char , new, value)]
pub struct ConversionNotNoexcept_1 {}
impl ConversionNotNoexcept_1 {
    pub unsafe fn new(mut _a0: *mut libc::c_char) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct NonCopyable_2 {}
impl NonCopyable_2 {
    pub unsafe fn move_from(_a0: *mut NonCopyable_2) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MoveOnlyVoidFunction_3 {
    pub was_called: *mut bool,
}
impl MoveOnlyVoidFunction_3 {
    pub unsafe fn operator_call(&mut self) {
        (*self.was_called) = true;
    }
}
impl Callable0<()> for MoveOnlyVoidFunction_3 {
    fn call(&self) -> () {
        unsafe { MoveOnlyVoidFunction_3::operator_call(&mut *(&raw const *self).cast_mut()) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MoveOnlyIntFunction_4 {
    pub was_called: *mut bool,
}
impl MoveOnlyIntFunction_4 {
    pub unsafe fn operator_call(&mut self) -> i32 {
        (*self.was_called) = true;
        return 0;
    }
}
impl Callable0<i32> for MoveOnlyIntFunction_4 {
    fn call(&self) -> i32 {
        unsafe { MoveOnlyIntFunction_4::operator_call(&mut *(&raw const *self).cast_mut()) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(i32, to_i32)]
pub struct Convertible_5 {}
impl Convertible_5 {
    pub unsafe fn to_i32(&self) -> i32 {
        return 42;
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
