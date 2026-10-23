extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct S {
    pub i: (),
}
impl S {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { i: 0_() };
        this
    }
    pub unsafe fn new_2(mut j: ()) -> Self {
        let mut this = Self { i: j };
        this
    }
    pub unsafe fn operator_eq(&self, mut x: ()) -> () {
        return ((self.i) == (x));
    }
}
impl Default for S {
    fn default() -> Self {
        unsafe { S::new_1() }
    }
}
pub unsafe fn test_0() -> () {
    {
        let mut p1: () = {
            let mut __tmp_0: () = 3_();
            let mut __tmp_1: () = 4_();
            std_pair_int__short_::new({ &mut __tmp_0 }, { &mut __tmp_1 })
        };
        let mut p2: () = {
            let mut __tmp_2: () = 5_();
            let mut __tmp_3: () = 6_();
            std_pair_int__short_::new({ &mut __tmp_2 }, { &mut __tmp_3 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p1: () = {
            let mut __tmp_4: () = 3_();
            let mut __tmp_5: S = S::new_2({ 4_() });
            std_pair_int__S_::new({ &mut __tmp_4 }, { &mut __tmp_5 })
        };
        let mut p2: () = {
            let mut __tmp_6: () = 5_();
            let mut __tmp_7: S = S::new_2({ 6_() });
            std_pair_int__S_::new({ &mut __tmp_6 }, { &mut __tmp_7 })
        };
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
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
