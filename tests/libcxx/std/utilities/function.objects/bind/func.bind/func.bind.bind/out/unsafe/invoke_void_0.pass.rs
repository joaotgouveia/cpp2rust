extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut count_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn f_1() {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
}
pub unsafe fn g_2() -> () {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
    return 0_();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A_void_0 {}
impl A_void_0 {
    pub unsafe fn operator_call_1(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
    }
    pub unsafe fn operator_call_2(&self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)) += 2_();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A_int_0 {}
impl A_int_0 {
    pub unsafe fn operator_call_1(&mut self) -> () {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
        return 4_();
    }
    pub unsafe fn operator_call_2(&self) -> () {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)) += 2_();
        return 5_();
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
    (unsafe { test_3((unsafe { bind_4(Some(f_1)) })) });
    (unsafe {
        test_5(
            (unsafe {
                let mut ___f: Option<unsafe fn()> = (Some(f_1));
                bind_6(&mut ___f)
            }),
        )
    });
    (unsafe {
        test_7(
            (unsafe {
                let mut ___f: A_void_0 = <A_void_0>::default();
                bind_8(&mut ___f)
            }),
        )
    });
    (unsafe {
        let mut _f: std___bind_A_void_0_ = (unsafe {
            let mut ___f: A_void_0 = <A_void_0>::default();
            bind_8(&mut ___f)
        });
        test_const_9(&mut _f)
    });
    (unsafe { test_10((unsafe { bind_11(Some(f_1)) })) });
    (unsafe {
        test_12(
            (unsafe {
                let mut ___f: Option<unsafe fn()> = (Some(f_1));
                bind_13(&mut ___f)
            }),
        )
    });
    (unsafe {
        test_14(
            (unsafe {
                let mut ___f: A_void_0 = <A_void_0>::default();
                bind_15(&mut ___f)
            }),
        )
    });
    (unsafe {
        let mut _f: std___bind_r_void__A_void_0_ = (unsafe {
            let mut ___f: A_void_0 = <A_void_0>::default();
            bind_15(&mut ___f)
        });
        test_const_16(&mut _f)
    });
    (unsafe { test_17((unsafe { bind_18(Some(g_2)) })) });
    (unsafe {
        test_19(
            (unsafe {
                let mut ___f: Option<unsafe fn() -> ()> = (Some(g_2));
                bind_20(&mut ___f)
            }),
        )
    });
    (unsafe {
        test_21(
            (unsafe {
                let mut ___f: A_int_0 = <A_int_0>::default();
                bind_22(&mut ___f)
            }),
        )
    });
    (unsafe {
        let mut _f: std___bind_r_void__A_int_0_ = (unsafe {
            let mut ___f: A_int_0 = <A_int_0>::default();
            bind_22(&mut ___f)
        });
        test_const_23(&mut _f)
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_0);
}
