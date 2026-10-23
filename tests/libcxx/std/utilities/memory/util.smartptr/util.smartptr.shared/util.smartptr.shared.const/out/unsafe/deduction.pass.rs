extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_shared_ptr_A___std_shared_ptr_A__ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_shared_ptr_const_A___std_shared_ptr_const_A__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct D {}
impl D {
    pub unsafe fn operator_call(&self, mut ptr: *const A) {
        {
            let __p = ptr;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(__p.cast_mut()))
            }
        };
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
        let mut s0: () = unimplemented!();
        let mut w: () = unimplemented!();
        let mut s: () = unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s0: () = unimplemented!();
        let mut w: () = unimplemented!();
        let mut s: () = unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut u: () = std_unique_ptr_A__std_default_delete_A__::new({
            (Box::leak(Box::new(<A>::default())) as *mut A)
        });
        let mut uPointee: *mut A = unimplemented!();
        let mut s: () = unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut u: () = std_unique_ptr_const_A__std_default_delete_const_A__::new({
            (Box::leak(Box::new(<A>::default())) as *mut A).cast_const()
        });
        let mut uPointee: *const A = unimplemented!();
        let mut s: () = unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut u: () = unimplemented!();
        let mut uPointee: *mut A = unimplemented!();
        let mut s: () = unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut u: () = unimplemented!();
        let mut uPointee: *const A = unimplemented!();
        let mut s: () = unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
