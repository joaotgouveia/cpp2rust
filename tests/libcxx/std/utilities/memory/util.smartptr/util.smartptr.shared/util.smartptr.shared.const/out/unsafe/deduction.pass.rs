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
pub struct test_macros_detail_is_same_std_shared_ptr_A___std_shared_ptr_A__ {}
pub type anon_1 = u32;
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let mut s0: () = unimplemented!();
        let mut w: () = unimplemented!();
        let mut s: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(((unimplemented!()) == (2_i64)));
        assert!(((unimplemented!()) == (2_i64)));
        assert!(((unimplemented!()) == (unimplemented!())));
    }
    {
        let mut s0: () = unimplemented!();
        let mut w: () = unimplemented!();
        let mut s: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(((unimplemented!()) == (2_i64)));
        assert!(((unimplemented!()) == (2_i64)));
        assert!(((unimplemented!()) == (unimplemented!())));
    }
    {
        let mut u: Option<Box<A>> = Some(Box::from_raw(
            (Box::leak(Box::new(<A>::default())) as *mut A),
        ));
        let mut uPointee: *mut A = u
            .as_deref_mut()
            .map_or(::std::ptr::null_mut(), |v| v as *mut A);
        let mut s: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(u.is_none());
        assert!(((unimplemented!()) == (uPointee)));
    }
    {
        let mut u: Option<Box<A>> = Some(Box::from_raw(
            (Box::leak(Box::new(<A>::default())) as *mut A).cast_const(),
        ));
        let mut uPointee: *const A = u
            .as_deref_mut()
            .map_or(::std::ptr::null_mut(), |v| v as *mut A);
        let mut s: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(u.is_none());
        assert!(((unimplemented!()) == (uPointee)));
    }
    {
        let mut u: Option<Box<A>> = unimplemented!();
        let mut uPointee: *mut A = u
            .as_deref_mut()
            .map_or(::std::ptr::null_mut(), |v| v as *mut A);
        let mut s: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(u.is_none());
        assert!(((unimplemented!()) == (uPointee)));
    }
    {
        let mut u: Option<Box<A>> = unimplemented!();
        let mut uPointee: *const A = u
            .as_deref_mut()
            .map_or(::std::ptr::null_mut(), |v| v as *mut A);
        let mut s: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(u.is_none());
        assert!(((unimplemented!()) == (uPointee)));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
