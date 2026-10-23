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
pub struct indirect_less {}
impl indirect_less {
    pub unsafe fn operator_call_1_class_std_unique_ptr_int_(
        &self,
        x: *const (),
        y: *const (),
    ) -> () {
        return ((*(*x)) < (*(*y)));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct first_only {}
impl first_only {
    pub unsafe fn operator_call(&self, x: *const (), y: *const ()) -> () {
        return (((*x).first) < ((*y).first));
    }
}
pub unsafe fn generate_sawtooth_0(mut N: (), mut M: ()) -> () {
    let mut v: () = unimplemented!();
    let mut x: () = 0_();
    let mut ver: () = 0_();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        (*unimplemented!()) = std_pair_int__int_::new({ &mut x }, { &mut ver });
        if ((x.prefix_inc()) == (M)) {
            x = 0_();
            ver.prefix_inc();
        }
        i.prefix_inc();
    }
    return unimplemented!();
}
pub unsafe fn test_1() -> () {
    let N: () = 1000_();
    let M: () = 10_();
    {
        let mut v: () = (unsafe { generate_sawtooth_0(N, M) });
        unimplemented!();
        unimplemented!();
    }
    {
        {
            let mut v: () = (unsafe { generate_sawtooth_0(N, M) });
            let mut randomness: () = unimplemented!();
            let mut i: () = 0_();
            'loop_: while ((i) < ((N) - (M))) {
                unimplemented!();
                i += M;
            }
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut v: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
    (unsafe { test_1() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
