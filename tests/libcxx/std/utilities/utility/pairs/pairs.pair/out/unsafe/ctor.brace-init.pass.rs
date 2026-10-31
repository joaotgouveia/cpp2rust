extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct ExplicitT {
    pub value: i32,
}
impl ExplicitT {
    pub unsafe fn new(mut x: i32) -> Self {
        let mut this = Self { value: x };
        this
    }
    pub unsafe fn copy_from(o: *const ExplicitT) -> Self {
        let mut this = Self { value: (*o).value };
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const ExplicitT) -> *mut ExplicitT {
        self.value = (*_a0).value;
        return &mut (*(self as *mut ExplicitT));
    }
}
impl Clone for ExplicitT {
    fn clone(&self) -> Self {
        unsafe { ExplicitT::copy_from(self as *const ExplicitT) }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct ImplicitT {
    pub value: i32,
}
impl ImplicitT {
    pub unsafe fn new(mut x: i32) -> Self {
        let mut this = Self { value: x };
        this
    }
    pub unsafe fn copy_from(o: *const ImplicitT) -> Self {
        let mut this = Self { value: (*o).value };
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const ImplicitT) -> *mut ImplicitT {
        self.value = (*_a0).value;
        return &mut (*(self as *mut ImplicitT));
    }
}
impl Clone for ImplicitT {
    fn clone(&self) -> Self {
        unsafe { ImplicitT::copy_from(self as *const ImplicitT) }
    }
}
pub unsafe fn can_construct_with_brace_init_0(mut _a0: i32) -> bool {
    return true;
}
pub unsafe fn can_construct_with_brace_init_1(__args: Box<[VaArg]>) -> bool {
    return false;
}
pub unsafe fn can_construct_with_ctad_brace_init_2(__args: Box<[VaArg]>) -> bool {
    return false;
}
pub unsafe fn can_construct_with_ctad_brace_init_3(__args: Box<[VaArg]>) -> bool {
    return false;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BraceInit {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoBraceInit {}
impl NoBraceInit {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ExplicitBraceInit {}
pub unsafe fn explicit_vs_implicit_brace_init_4(
    mut _a0: (ExplicitBraceInit, ExplicitBraceInit),
) -> i32 {
    return 1;
}
pub unsafe fn explicit_vs_implicit_brace_init_5(mut _a0: (BraceInit, BraceInit)) -> i32 {
    return 2;
}
pub unsafe fn test_6() -> bool {
    {
        let mut p1: (ExplicitT, BraceInit) = (ExplicitT::new({ 42 }), <BraceInit>::default());
        assert!(((p1.0.value) == (42)));
        let mut p2: (ExplicitT, BraceInit) = (ExplicitT::new({ 42 }), <BraceInit>::default());
        assert!(((p2.0.value) == (42)));
    }
    {
        let mut p1: (BraceInit, ExplicitT) = (<BraceInit>::default(), ExplicitT::new({ 42 }));
        assert!(((p1.1.value) == (42)));
        let mut p2: (BraceInit, ExplicitT) = (<BraceInit>::default(), ExplicitT::new({ 42 }));
        assert!(((p2.1.value) == (42)));
    }
    {
        let mut p: (BraceInit, BraceInit) = (<BraceInit>::default(), <BraceInit>::default());
        &(p);
    }
    {
        let mut p: (ImplicitT, BraceInit) = (ImplicitT::new({ 42 }), <BraceInit>::default());
        assert!(((p.0.value) == (42)));
    }
    {
        let mut p: (BraceInit, ImplicitT) = (<BraceInit>::default(), ImplicitT::new({ 42 }));
        assert!(((p.1.value) == (42)));
    }
    {
        let mut p: (BraceInit, BraceInit) = (<BraceInit>::default(), <BraceInit>::default());
        &(p);
    }
    {
        const _: () = assert!(true, "can_construct_with_brace_init<BraceInit>(0)");;
        const _: () = assert!(true, "!can_construct_with_brace_init<NoBraceInit>(0)");;
        const _: () = assert!(true, "!can_construct_with_ctad_brace_init<BraceInit>(0)");;
        const _: () = assert!(true, "!can_construct_with_ctad_brace_init<int>(0)");;
    }
    {
        assert!(
            ((unsafe {
                explicit_vs_implicit_brace_init_5((<BraceInit>::default(), <BraceInit>::default()))
            }) == (2))
        );
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_6() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
