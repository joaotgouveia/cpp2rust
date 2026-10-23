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
pub struct test_macros_detail_is_same_std_filesystem_path_iterator__std_filesystem_path_iterator_ {
}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_filesystem_path__std_filesystem_path_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_filesystem_path_iterator_ref__std_filesystem_path_iterator_ref_
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_filesystem_path_ptr__const_std_filesystem_path_ptr_
{}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_string_ref__const_std_string_ref_ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_bool__bool_ {}
pub unsafe fn checkCollectionsEqual_6(
    mut start1: (),
    end1: (),
    mut start2: *const (),
    end2: *const (),
) -> () {
    'loop_: while (unimplemented!()) && ((start2) != (end2)) {
        if !(unimplemented!()) {
            return false;
        }
        unimplemented!();
        start2.prefix_inc();
    }
    return ((unimplemented!()) && ((start2) == (end2)));
}
pub unsafe fn checkCollectionsEqualBackwards_7(
    start1: (),
    mut end1: (),
    start2: *const (),
    mut end2: *const (),
) -> () {
    'loop_: while (unimplemented!()) && ((start2) != (end2)) {
        unimplemented!();
        end2.prefix_dec();
        if !(unimplemented!()) {
            return false;
        }
    }
    return ((unimplemented!()) && ((start2) == (end2)));
}
pub unsafe fn checkIteratorConcepts_8() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::bidirectional_iterator<It>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_same<Traits::iterator_category, std::input_iterator_tag>::value"
    );;
    const _: () = assert!(true, "std::is_same<Traits::pointer, path const*>::value");;
    const _: () = assert!(true, "std::is_same<Traits::reference, path>::value");;
    {
        let mut it: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let p: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
    }
}
pub unsafe fn checkBeginEndBasic_9() {
    {
        let p: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let p: () = unimplemented!();
        let mut default_constructed: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        let expect: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        let expect: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
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
    (unsafe { checkIteratorConcepts_8() });
    (unsafe { checkBeginEndBasic_9() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
