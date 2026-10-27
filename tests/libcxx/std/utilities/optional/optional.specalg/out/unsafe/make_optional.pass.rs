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
pub struct test_macros_detail_is_same_std_optional_int___std_optional_int__ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_optional_std_unique_ptr_int__std_default_delete_int_____std_optional_std_unique_ptr_int__std_default_delete_int____
{}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_optional_int_ptr___std_optional_int_ptr__ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_optional_const_int_ptr___std_optional_const_int_ptr__ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_optional_int___const_std_optional_int__ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_optional_std_string___std_optional_std_string__ {}
pub static mut value_6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { true });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct mandate_same_std_optional_int___std_optional_int__ {}
pub static mut value_7: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { true });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct mandate_same_std_optional_std_unique_ptr_int__std_default_delete_int_____std_optional_std_unique_ptr_int__std_default_delete_int____
{}
pub static mut value_8: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { true });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct mandate_same_std_optional_int_ptr___std_optional_int_ptr__ {}
pub static mut value_9: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { true });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct mandate_same_std_optional_const_int_ptr___std_optional_const_int_ptr__ {}
pub static mut can_make_optional_implicit_10: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_6))
    });
pub static mut can_make_optional_implicit_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_6))
    });
pub static mut can_make_optional_implicit_12: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_6))
    });
pub static mut can_make_optional_implicit_13: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_6))
    });
pub static mut can_make_optional_implicit_14: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_7))
    });
pub static mut can_make_optional_implicit_15: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut can_make_optional_implicit_16: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut can_make_optional_implicit_17: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut can_make_optional_implicit_18: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_8))
    });
pub static mut can_make_optional_implicit_19: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_9))
    });
const _: () = assert!(true, "can_make_optional_implicit<int>");
const _: () = assert!(true, "can_make_optional_implicit<const int>");
const _: () = assert!(true, "can_make_optional_implicit<int&>");
const _: () = assert!(true, "can_make_optional_implicit<const int&>");
const _: () = assert!(true, "can_make_optional_implicit<std::unique_ptr<int>>");
const _: () = assert!(
    true,
    "!can_make_optional_implicit<const std::unique_ptr<int>>"
);
const _: () = assert!(true, "!can_make_optional_implicit<std::unique_ptr<int>&>");
const _: () = assert!(
    true,
    "!can_make_optional_implicit<const std::unique_ptr<int>&>"
);
const _: () = assert!(true, "can_make_optional_implicit<int[42]>");
const _: () = assert!(true, "can_make_optional_implicit<const int (&)[42]>");
pub unsafe fn test_20() -> () {
    {
        let mut arr: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut opt: () = unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let opt: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "opt.value() == 2");;
    }
    {
        let mut opt: () = unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    let test_string: FnPtr<fn()> = FnPtr::<fn()>::new(|| unsafe {
        let s: () = unimplemented!();
        let mut opt: () = unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
    });
    {
        (unsafe { test_string.call() });
    }
    let test_unique_ptr: FnPtr<fn()> = FnPtr::<fn()>::new(|| unsafe {
        let mut s: () = unimplemented!();
        let mut opt: () = unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
    });
    {
        (unsafe { test_unique_ptr.call() });
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
    (unsafe { test_20() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const value_6);
    std::cell::LazyCell::force(&*&raw const value_7);
    std::cell::LazyCell::force(&*&raw const value_8);
    std::cell::LazyCell::force(&*&raw const value_9);
    std::cell::LazyCell::force(&*&raw const can_make_optional_implicit_10);
    std::cell::LazyCell::force(&*&raw const can_make_optional_implicit_11);
    std::cell::LazyCell::force(&*&raw const can_make_optional_implicit_12);
    std::cell::LazyCell::force(&*&raw const can_make_optional_implicit_13);
    std::cell::LazyCell::force(&*&raw const can_make_optional_implicit_14);
    std::cell::LazyCell::force(&*&raw const can_make_optional_implicit_15);
    std::cell::LazyCell::force(&*&raw const can_make_optional_implicit_16);
    std::cell::LazyCell::force(&*&raw const can_make_optional_implicit_17);
    std::cell::LazyCell::force(&*&raw const can_make_optional_implicit_18);
    std::cell::LazyCell::force(&*&raw const can_make_optional_implicit_19);
}
