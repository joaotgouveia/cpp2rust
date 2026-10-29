extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(mut f: (), mut expected: ()) {
    unimplemented!();
}
pub unsafe fn test_1(mut f: (), mut expected: ()) {
    unimplemented!();
}
pub unsafe fn test_2(mut f: (), mut expected: ()) {
    unimplemented!();
}
pub unsafe fn test_3(mut f: (), mut expected: ()) {
    unimplemented!();
}
pub unsafe fn test_const_4(f: *const (), mut expected: ()) {
    unimplemented!();
}
pub unsafe fn test_const_5(f: *const (), mut expected: ()) {
    unimplemented!();
}
pub unsafe fn f_6() -> () {
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
pub unsafe fn test_all_7() -> () {
    (unsafe { test_0((unsafe { bind_8(Some(f_6)) }), 1_()) });
    (unsafe {
        test_0(
            (unsafe {
                let mut ___f: Option<unsafe fn() -> ()> = (Some(f_6));
                bind_9(&mut ___f)
            }),
            1_(),
        )
    });
    (unsafe {
        test_1(
            (unsafe {
                let mut ___f: A_int_0 = <A_int_0>::default();
                bind_10(&mut ___f)
            }),
            4_(),
        )
    });
    (unsafe {
        let mut _f: () = (unsafe {
            let mut ___f: A_int_0 = <A_int_0>::default();
            bind_10(&mut ___f)
        });
        test_const_4(&mut _f, 5_())
    });
    (unsafe { test_2(unimplemented!(), 1_()) });
    (unsafe { test_2(unimplemented!(), 1_()) });
    (unsafe { test_3(unimplemented!(), 4_()) });
    (unsafe {
        let mut _f: () = unimplemented!();
        test_const_5(&mut _f, 5_())
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
    (unsafe { test_all_7() });
    const _: () = assert!(true, "test_all()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
