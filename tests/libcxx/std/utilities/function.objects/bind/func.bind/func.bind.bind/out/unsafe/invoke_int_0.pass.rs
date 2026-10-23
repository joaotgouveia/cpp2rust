extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn f_0() -> () {
    return 1_();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A_int_0 {}
impl A_int_0 {
    pub unsafe fn operator_call_1(&mut self) -> () {
        return 4_();
    }
    pub unsafe fn operator_call_2(&self) -> () {
        return 5_();
    }
}
pub unsafe fn test_all_1() -> () {
    (unsafe { test_2((unsafe { bind_3(Some(f_0)) }), 1_()) });
    (unsafe {
        test_4(
            (unsafe {
                let mut ___f: Option<unsafe fn() -> ()> = (Some(f_0));
                bind_5(&mut ___f)
            }),
            1_(),
        )
    });
    (unsafe {
        test_6(
            (unsafe {
                let mut ___f: A_int_0 = <A_int_0>::default();
                bind_7(&mut ___f)
            }),
            4_(),
        )
    });
    (unsafe {
        let mut _f: std___bind_A_int_0_ = (unsafe {
            let mut ___f: A_int_0 = <A_int_0>::default();
            bind_7(&mut ___f)
        });
        test_const_8(&mut _f, 5_())
    });
    (unsafe { test_9((unsafe { bind_10(Some(f_0)) }), 1_()) });
    (unsafe {
        test_11(
            (unsafe {
                let mut ___f: Option<unsafe fn() -> ()> = (Some(f_0));
                bind_12(&mut ___f)
            }),
            1_(),
        )
    });
    (unsafe {
        test_13(
            (unsafe {
                let mut ___f: A_int_0 = <A_int_0>::default();
                bind_14(&mut ___f)
            }),
            4_(),
        )
    });
    (unsafe {
        let mut _f: std___bind_r_int__A_int_0_ = (unsafe {
            let mut ___f: A_int_0 = <A_int_0>::default();
            bind_14(&mut ___f)
        });
        test_const_15(&mut _f, 5_())
    });
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
    (unsafe { test_all_1() });
    const _: () = assert!(true, "test_all()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
