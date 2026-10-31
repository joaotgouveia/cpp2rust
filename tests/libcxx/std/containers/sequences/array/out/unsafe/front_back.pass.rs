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
pub struct test_macros_detail_is_same_double_ref__double_ref_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_double_ref__const_double_ref_ {}
pub unsafe fn tests_2() -> bool {
    {
        let mut c: Vec<f64> = Vec::from([1_f64, 2_f64, 3.5_f64]);
        let r1: () = unimplemented!();
        assert!(((*r1) == (1_f64)));
        (*r1) = 5.5_f64;
        assert!(
            ((*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<f64> = &mut c;
                __v.as_mut_ptr().add(__idx)
            }) == (5.5_f64))
        );
        let r2: () = unimplemented!();
        assert!(((*r2) == (3.5_f64)));
        (*r2) = 7.5_f64;
        assert!(
            ((*{
                let __idx = 2_usize as usize;
                let __v: &mut Vec<f64> = &mut c;
                __v.as_mut_ptr().add(__idx)
            }) == (7.5_f64))
        );
    }
    {
        let mut c: Vec<f64> = std::array::from_fn::<_, 0, _>(|_| 0_f64).to_vec();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        {}
    }
    {
        let mut c: Vec<f64> = std::array::from_fn::<_, 0, _>(|_| 0_f64).to_vec();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        {}
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { tests_2() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
