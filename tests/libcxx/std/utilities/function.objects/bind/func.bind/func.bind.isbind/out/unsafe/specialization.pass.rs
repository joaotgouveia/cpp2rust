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
pub struct MyBind {}
impl MyBind {
    pub unsafe fn operator_call(&self, mut x: (), mut y: ()) -> () {
        return (((10_()) * (x)) + (y));
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
    {
        let mut f: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut x: ()| -> () {
            return (((10_()) * (x)) + (9_()));
        });
        let mut bindexpr: MyBind = <MyBind>::default();
        let mut bound: std___bind__lambda_at_specialization_pass_cpp_32_14__ref__MyBind_ref_ =
            unimplemented!();
        unimplemented!();
    }
    {
        let mut f: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut x: ()| -> () {
            return (((10_()) * (x)) + (9_()));
        });
        let bindexpr: MyBind = <MyBind>::default();
        let mut bound: std___bind__lambda_at_specialization_pass_cpp_38_14__ref__const_MyBind_ref_ =
            unimplemented!();
        unimplemented!();
    }
    {
        let mut f: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut x: ()| -> () {
            return (((10_()) * (x)) + (9_()));
        });
        let mut bindexpr: MyBind = <MyBind>::default();
        let mut bound: std___bind__lambda_at_specialization_pass_cpp_44_14__ref__MyBind_ =
            unimplemented!();
        unimplemented!();
    }
    {
        let mut f: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut x: ()| -> () {
            return (((10_()) * (x)) + (9_()));
        });
        let bindexpr: MyBind = <MyBind>::default();
        let mut bound: std___bind__lambda_at_specialization_pass_cpp_50_14__ref__const_MyBind_ =
            unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
