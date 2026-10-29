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
pub struct Empty {}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct S {
    pub a: (),
    pub k: (),
    pub e: Empty,
}
impl S {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            a: unimplemented!(),
            k: (*unimplemented!()),
            e: (*unimplemented!()),
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut S) -> Self {
        let mut this = Self {
            a: unimplemented!(),
            k: (*_a0).k,
            e: (*_a0).e,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut S) -> *mut S {
        unimplemented!();
        self.k = (*_a0).k;
        self.e = (*_a0).e;
        return &mut (*(self as *mut S));
    }
}
impl Default for S {
    fn default() -> Self {
        unsafe { S::new() }
    }
}
pub unsafe fn getP_0() -> () {
    return unimplemented!();
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
        let mut t: () = unimplemented!();
        unimplemented!();
        (*unimplemented!()) = 2_();
        unimplemented!();
    }
    {
        let mut t: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (*unimplemented!()) = 4_();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut d: () = 1.5_f64;
        let mut t: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (*unimplemented!()) = 2.5_f64;
        unimplemented!();
        (*unimplemented!()) = 4_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(
            true,
            "std::get<0> ( std::make_tuple ( 0.0f, 1, 2.0, 3L )) == 0"
        );;
        const _: () = assert!(
            true,
            "std::get<1> ( std::make_tuple ( 0.0f, 1, 2.0, 3L )) == 1"
        );;
        const _: () = assert!(
            true,
            "std::get<2> ( std::make_tuple ( 0.0f, 1, 2.0, 3L )) == 2"
        );;
        const _: () = assert!(
            true,
            "std::get<3> ( std::make_tuple ( 0.0f, 1, 2.0, 3L )) == 3"
        );;
        const _: () = assert!(true, "S().k == 1");;
        const _: () = assert!(true, "std::get<1>(getP()) == 4");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
