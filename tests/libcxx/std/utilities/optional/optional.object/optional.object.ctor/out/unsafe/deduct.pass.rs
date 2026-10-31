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
pub struct test_macros_detail_is_same_std_optional_int___std_optional_int__ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_optional_A___std_optional_A__ {}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_optional_const_int_ptr___std_optional_const_int_ptr__ {}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_optional_int_ptr___std_optional_int_ptr__ {}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_optional_char___std_optional_char__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A {}
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
        let mut opt: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        assert!(((*unimplemented!()) == (5)));
    }
    {
        let mut opt: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
    }
    {
        let mut __tmp_0: i32 = 5;
        let source: *const i32 = &mut __tmp_0;
        let mut opt: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        assert!(((*unimplemented!()) == (5)));
    }
    {
        let mut source: *const i32 = std::ptr::null();
        let mut opt: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        assert!((*unimplemented!()).is_null());
    }
    {
        let mut source: [i32; 3] = [1, 2, 3];
        let mut opt: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        assert!(((*(*unimplemented!()).offset((0) as isize)) == (1)));
    }
    {
        let mut source: () = unimplemented!();
        let mut opt: () = source.clone();
        const _: () = assert!(true, "");;
        assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
        assert!((((*unimplemented!()) as i32) == ((*unimplemented!()) as i32)));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
