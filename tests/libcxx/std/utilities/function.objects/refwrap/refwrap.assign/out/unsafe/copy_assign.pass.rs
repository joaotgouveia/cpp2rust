extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct functor1 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct convertible_to_int_ref {
    pub val: (),
}
impl convertible_to_int_ref {
    pub unsafe fn to_pmut(&mut self) -> *mut () {
        return &mut self.val;
    }
    pub unsafe fn to_pconst(&self) -> *const () {
        return &self.val;
    }
}
impl Default for convertible_to_int_ref {
    fn default() -> Self {
        unsafe { convertible_to_int_ref { val: 0_() } }
    }
}
pub unsafe fn test_0(t: *mut Option<unsafe fn()>) {
    let mut r: () = std_reference_wrapper_void__ptr____::new({ t });
    let mut t2: Option<unsafe fn()> = (*t);
    let mut r2: () = std_reference_wrapper_void__ptr____::new({ &mut t2 });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_1(t: *mut functor1) {
    let mut r: () = std_reference_wrapper_functor1_::new({ t });
    let mut t2: functor1 = (*t);
    let mut r2: () = std_reference_wrapper_functor1_::new({ &mut t2 });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_2(t: *mut ()) {
    let mut r: () = std_reference_wrapper_int_::new({ t });
    let mut t2: () = (*t);
    let mut r2: () = std_reference_wrapper_int_::new({ &mut t2 });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_3(t: *const ()) {
    let mut r: () = std_reference_wrapper_const_int_::new({ t });
    let t2: () = (*t);
    let mut r2: () = std_reference_wrapper_const_int_::new({ &t2 });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_4(t: *mut convertible_to_int_ref) {
    let mut r: () = std_reference_wrapper_convertible_to_int_ref_::new({ t });
    let mut t2: convertible_to_int_ref = (*t);
    let mut r2: () = std_reference_wrapper_convertible_to_int_ref_::new({ &mut t2 });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_5(t: *const convertible_to_int_ref) {
    let mut r: () = std_reference_wrapper_const_convertible_to_int_ref_::new({ t });
    let t2: convertible_to_int_ref = (*t);
    let mut r2: () = std_reference_wrapper_const_convertible_to_int_ref_::new({ &t2 });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn f_6() {}
pub unsafe fn g_7() {}
pub unsafe fn test_function_8() {
    let mut r: () = std_reference_wrapper_void____::new({ Some(f_6) });
    let mut r2: () = std_reference_wrapper_void____::new({ Some(g_7) });
    unimplemented!();
    unimplemented!();
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
    let mut fp: Option<unsafe fn()> = Some(f_6);
    (unsafe { test_0(&mut fp) });
    (unsafe { test_function_8() });
    let mut f1: functor1 = <functor1>::default();
    (unsafe { test_1(&mut f1) });
    let mut i: () = 0_();
    (unsafe { test_2(&mut i) });
    let j: () = 0_();
    (unsafe { test_3(&j) });
    let mut convi: convertible_to_int_ref = <convertible_to_int_ref>::default();
    (unsafe { test_4(&mut convi) });
    let convic: convertible_to_int_ref = <convertible_to_int_ref>::default();
    (unsafe { test_5(&convic) });
    {
        const _: () = assert!(true, "(std::is_assignable<Ref&, int&>::value)");;
        const _: () = assert!(true, "(!std::is_assignable<Ref&, int>::value)");;
        const _: () = assert!(true, "(!std::is_assignable<Ref&, int&&>::value)");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
