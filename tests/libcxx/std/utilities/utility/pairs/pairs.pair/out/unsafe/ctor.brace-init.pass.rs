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
    pub value: (),
}
impl ExplicitT {
    pub unsafe fn new(mut x: ()) -> Self {
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
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct ImplicitT {
    pub value: (),
}
impl ImplicitT {
    pub unsafe fn new(mut x: ()) -> Self {
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
pub unsafe fn can_construct_with_brace_init_0(mut _a0: ()) -> () {
    return true;
}
pub unsafe fn can_construct_with_brace_init_1(__args: Box<[VaArg]>) -> () {
    return false;
}
pub unsafe fn can_construct_with_ctad_brace_init_2(__args: Box<[VaArg]>) -> () {
    return false;
}
pub unsafe fn can_construct_with_ctad_brace_init_3(__args: Box<[VaArg]>) -> () {
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
pub unsafe fn explicit_vs_implicit_brace_init_4(mut _a0: ()) -> () {
    return 1_();
}
pub unsafe fn explicit_vs_implicit_brace_init_5(mut _a0: ()) -> () {
    return 2_();
}
pub unsafe fn test_6() -> () {
    {
        let mut p1: () = {
            let mut __tmp_0: ExplicitT = ExplicitT::new({ 42_() });
            let mut __tmp_1: BraceInit = <BraceInit>::default();
            std_pair_ExplicitT__BraceInit_::new({ &mut __tmp_0 }, { &mut __tmp_1 })
        };
        unimplemented!();
        let mut p2: () = {
            let mut __tmp_2: ExplicitT = ExplicitT::new({ 42_() });
            let mut __tmp_3: BraceInit = <BraceInit>::default();
            std_pair_ExplicitT__BraceInit_::new({ &mut __tmp_2 }, { &mut __tmp_3 })
        };
        unimplemented!();
    }
    {
        let mut p1: () = {
            let mut __tmp_4: BraceInit = <BraceInit>::default();
            let mut __tmp_5: ExplicitT = ExplicitT::new({ 42_() });
            std_pair_BraceInit__ExplicitT_::new({ &mut __tmp_4 }, { &mut __tmp_5 })
        };
        unimplemented!();
        let mut p2: () = {
            let mut __tmp_6: BraceInit = <BraceInit>::default();
            let mut __tmp_7: ExplicitT = ExplicitT::new({ 42_() });
            std_pair_BraceInit__ExplicitT_::new({ &mut __tmp_6 }, { &mut __tmp_7 })
        };
        unimplemented!();
    }
    {
        let mut p: () = {
            let mut __tmp_8: BraceInit = <BraceInit>::default();
            let mut __tmp_9: BraceInit = <BraceInit>::default();
            std_pair_BraceInit__BraceInit_::new({ &mut __tmp_8 }, { &mut __tmp_9 })
        };
        &(p);
    }
    {
        let mut p: () = {
            let mut __tmp_10: () = 42_();
            let mut __tmp_11: BraceInit = <BraceInit>::default();
            std_pair_ImplicitT__BraceInit_::new({ &mut __tmp_10 }, { &mut __tmp_11 })
        };
        unimplemented!();
    }
    {
        let mut p: () = {
            let mut __tmp_12: BraceInit = <BraceInit>::default();
            let mut __tmp_13: () = 42_();
            std_pair_BraceInit__ImplicitT_::new({ &mut __tmp_12 }, { &mut __tmp_13 })
        };
        unimplemented!();
    }
    {
        let mut p: () = {
            let mut __tmp_14: BraceInit = <BraceInit>::default();
            let mut __tmp_15: BraceInit = <BraceInit>::default();
            std_pair_BraceInit__BraceInit_::new({ &mut __tmp_14 }, { &mut __tmp_15 })
        };
        &(p);
    }
    {
        const _: () = assert!(true, "can_construct_with_brace_init<BraceInit>(0)");;
        const _: () = assert!(true, "!can_construct_with_brace_init<NoBraceInit>(0)");;
        const _: () = assert!(true, "!can_construct_with_ctad_brace_init<BraceInit>(0)");;
        const _: () = assert!(true, "!can_construct_with_ctad_brace_init<int>(0)");;
    }
    {
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
    (unsafe { test_6() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
