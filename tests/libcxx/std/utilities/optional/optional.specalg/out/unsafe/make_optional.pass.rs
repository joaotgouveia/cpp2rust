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
pub struct test_macros_detail_is_same_std_optional_std_unique_ptr_int__std_default_delete_int_____std_optional_std_unique_ptr_int__std_default_delete_int____
{}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_optional_int_ptr___std_optional_int_ptr__ {}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_optional_const_int_ptr___std_optional_const_int_ptr__ {}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_optional_int___const_std_optional_int__ {}
pub type anon_5 = u32;
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
pub static mut value_6: std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { true });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct mandate_same_std_optional_int___std_optional_int__ {}
pub static mut value_7: std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { true });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct mandate_same_std_optional_std_unique_ptr_int__std_default_delete_int_____std_optional_std_unique_ptr_int__std_default_delete_int____
{}
pub static mut value_8: std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { true });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct mandate_same_std_optional_int_ptr___std_optional_int_ptr__ {}
pub static mut value_9: std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { true });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct mandate_same_std_optional_const_int_ptr___std_optional_const_int_ptr__ {}
pub static mut can_make_optional_implicit_10: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_6))
    });
pub static mut can_make_optional_implicit_11: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_6))
    });
pub static mut can_make_optional_implicit_12: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_6))
    });
pub static mut can_make_optional_implicit_13: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_6))
    });
pub static mut can_make_optional_implicit_14: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_7))
    });
pub static mut can_make_optional_implicit_15: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut can_make_optional_implicit_16: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut can_make_optional_implicit_17: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut can_make_optional_implicit_18: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_8))
    });
pub static mut can_make_optional_implicit_19: std::cell::LazyCell<bool> =
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
pub unsafe fn test_20() -> bool {
    {
        let mut arr: [i32; 10] = [
            0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
        ];
        let mut opt: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(((*unimplemented!()) == (arr.as_mut_ptr())));
    }
    {
        let opt: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "opt.value() == 2");;
    }
    {
        let mut opt: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(((*unimplemented!()) == (2)));
    }
    let test_string: FnPtr<fn()> = FnPtr::<fn()>::new(|| unsafe {
        let s: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"123".as_ptr(), libcc2rs::c_str_len(c"123".as_ptr()) + 1)
                .to_vec();
        let mut opt: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            (*unimplemented!())
                == std::slice::from_raw_parts(
                    c"123".as_ptr(),
                    libcc2rs::c_str_len(c"123".as_ptr()) + 1
                )
                .to_vec()
        );
    });
    {
        (unsafe { test_string.call() });
    }
    let test_unique_ptr: FnPtr<fn()> = FnPtr::<fn()>::new(|| unsafe {
        let mut s: Option<Box<i32>> = Some(Box::new(3));
        let mut opt: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            ((*(*unimplemented!())
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(s.is_none());
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_20() });
    const _: () = assert!(true, "test()");;
    return 0;
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
