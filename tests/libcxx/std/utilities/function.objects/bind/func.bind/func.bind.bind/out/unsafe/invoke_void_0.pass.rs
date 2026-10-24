extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut count_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn test_1(mut f: ()) {
    let mut save_count: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0));
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_2(mut f: ()) {
    let mut save_count: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0));
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_3(mut f: ()) {
    let mut save_count: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0));
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_4(mut f: ()) {
    let mut save_count: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0));
    (unsafe { f() });
    unimplemented!();
}
pub unsafe fn test_5(mut f: ()) {
    let mut save_count: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0));
    (unsafe { f() });
    unimplemented!();
}
pub unsafe fn test_6(mut f: ()) {
    let mut save_count: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0));
    (unsafe { f() });
    unimplemented!();
}
pub unsafe fn test_7(mut f: ()) {
    let mut save_count: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0));
    (unsafe { f() });
    unimplemented!();
}
pub unsafe fn test_8(mut f: ()) {
    let mut save_count: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0));
    (unsafe { f() });
    unimplemented!();
}
pub unsafe fn test_9(mut f: ()) {
    let mut save_count: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0));
    (unsafe { f() });
    unimplemented!();
}
pub unsafe fn test_const_10(f: *const ()) {
    let mut save_count: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0));
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_const_11(f: *const ()) {
    let mut save_count: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0));
    (unsafe { (*f)() });
    unimplemented!();
}
pub unsafe fn test_const_12(f: *const ()) {
    let mut save_count: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0));
    (unsafe { (*f)() });
    unimplemented!();
}
pub unsafe fn f_13() {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
}
pub unsafe fn g_14() -> () {
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
    (unsafe { test_1((unsafe { bind_15(Some(f_13)) })) });
    (unsafe {
        test_2(
            (unsafe {
                let mut ___f: Option<unsafe fn()> = (Some(f_13));
                bind_16(&mut ___f)
            }),
        )
    });
    (unsafe {
        test_3(
            (unsafe {
                let mut ___f: A_void_0 = <A_void_0>::default();
                bind_17(&mut ___f)
            }),
        )
    });
    (unsafe {
        let mut _f: () = (unsafe {
            let mut ___f: A_void_0 = <A_void_0>::default();
            bind_17(&mut ___f)
        });
        test_const_10(&mut _f)
    });
    (unsafe { test_4(unimplemented!()) });
    (unsafe { test_5(unimplemented!()) });
    (unsafe { test_6(unimplemented!()) });
    (unsafe {
        let mut _f: () = unimplemented!();
        test_const_11(&mut _f)
    });
    (unsafe { test_7(unimplemented!()) });
    (unsafe { test_8(unimplemented!()) });
    (unsafe { test_9(unimplemented!()) });
    (unsafe {
        let mut _f: () = unimplemented!();
        test_const_12(&mut _f)
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_0);
}
