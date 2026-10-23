extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_pair_char_ptr__void__ptr______std_pair_char_ptr__void__ptr_____
{}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_pair_std_reference_wrapper_int___void__ptr______std_pair_std_reference_wrapper_int___void__ptr_____
{}
pub unsafe fn dummy_2() {}
pub unsafe fn test_decay_3() {
    let mut arr: [(); 1] = [unimplemented!()];
    let mut pr: () = {
        let mut __tmp_0: *mut () = arr.as_mut_ptr();
        let mut __tmp_1: Option<unsafe fn()> = Some(dummy_2);
        std_pair_char_ptr__void__ptr____::new({ &mut __tmp_0 }, { &mut __tmp_1 })
    };
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn test_unwrap_4() {
    let mut n: () = 0_();
    let mut pr: () = {
        let mut __tmp_2: () = unimplemented!();
        std_pair_std_reference_wrapper_int___void__ptr____::new({ &mut __tmp_2 }, { Some(dummy_2) })
    };
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "!std::is_same_v<decltype(pr), decltype(std::make_pair(std::ref(n), dummy))>"
    );;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_5() -> () {
    (unsafe { test_decay_3() });
    {
        (unsafe { test_unwrap_4() });
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
    (unsafe { test_5() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
