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
pub struct test_macros_detail_is_same_std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_optional_std_unique_ptr_const_int__std_default_delete_const_int_____std_optional_std_unique_ptr_const_int__std_default_delete_const_int____
{}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_optional_double___std_optional_double__ {}
pub static mut value_4: std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { true });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct mandate_same_std_optional_int___std_optional_int__ {}
pub static mut value_5: std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { true });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct mandate_same_std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
pub static mut value_6: std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { true });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct mandate_same_std_optional_std_unique_ptr_const_int__std_default_delete_const_int_____std_optional_std_unique_ptr_const_int__std_default_delete_const_int____
{}
pub static mut can_make_optional_explicit_impl_7: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_4))
    });
pub static mut can_make_optional_explicit_impl_8: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_4))
    });
pub static mut can_make_optional_explicit_impl_9: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_5))
    });
pub static mut can_make_optional_explicit_impl_10: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_5))
    });
pub static mut can_make_optional_explicit_impl_11: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_6))
    });
pub static mut can_make_optional_explicit_impl_12: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut can_make_optional_explicit_impl_13: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut can_make_optional_explicit_impl_14: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut can_make_optional_explicit_15: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut can_make_optional_explicit_impl_7))
    });
pub static mut can_make_optional_explicit_16: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut can_make_optional_explicit_impl_8))
    });
pub static mut can_make_optional_explicit_17: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut can_make_optional_explicit_impl_9))
    });
pub static mut can_make_optional_explicit_18: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut can_make_optional_explicit_impl_10))
    });
pub static mut can_make_optional_explicit_19: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut can_make_optional_explicit_impl_11))
    });
pub static mut can_make_optional_explicit_20: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut can_make_optional_explicit_impl_12))
    });
pub static mut can_make_optional_explicit_21: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut can_make_optional_explicit_impl_13))
    });
pub static mut can_make_optional_explicit_22: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut can_make_optional_explicit_impl_14))
    });
const _: () = assert!(true, "can_make_optional_explicit<int, long>");
const _: () = assert!(true, "can_make_optional_explicit<int, const long&>");
const _: () = assert!(true, "can_make_optional_explicit<std::string, const char*>");
const _: () = assert!(
    true,
    "can_make_optional_explicit<std::string, const char* const&>"
);
const _: () = assert!(
    true,
    "can_make_optional_explicit<std::unique_ptr<const int>, std::unique_ptr<int>>"
);
const _: () = assert!(
    true,
    "!can_make_optional_explicit<std::unique_ptr<const int>, const std::unique_ptr<int>>"
);
const _: () = assert!(
    true,
    "!can_make_optional_explicit<std::unique_ptr<const int>, std::unique_ptr<int>&>"
);
const _: () = assert!(
    true,
    "!can_make_optional_explicit<std::unique_ptr<const int>, const std::unique_ptr<int>&>"
);
pub unsafe fn test_ref_23() {
    let mut i: i32 = 0;
    let mut opt: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((*unimplemented!()) == (0)));
}
pub unsafe fn test_ref_24() {
    let mut i: f64 = 0_f64;
    let mut opt: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((*unimplemented!()) == (0_f64)));
}
pub unsafe fn test_25() -> bool {
    {
        let opt: () = unimplemented!();
        const _: () = assert!(true, "*opt == int(\'a\')");;
    }
    let test_string: FnPtr<fn()> = FnPtr::<fn()>::new(|| unsafe {
        {
            let mut s: Vec<libc::c_char> = std::slice::from_raw_parts(
                c"123".as_ptr(),
                libcc2rs::c_str_len(c"123".as_ptr()) + 1,
            )
            .to_vec();
            let mut opt: () = unimplemented!();
            assert!(
                (*unimplemented!())
                    == std::slice::from_raw_parts(
                        c"123".as_ptr(),
                        libcc2rs::c_str_len(c"123".as_ptr()) + 1
                    )
                    .to_vec()
            );
        }
        {
            let mut opt: () = unimplemented!();
            assert!(
                (*unimplemented!())
                    == std::slice::from_raw_parts(
                        c"XXXX".as_ptr(),
                        libcc2rs::c_str_len(c"XXXX".as_ptr()) + 1
                    )
                    .to_vec()
            );
        }
    });
    {
        (unsafe { test_string.call() });
    }
    let test_unique_ptr: FnPtr<fn()> = FnPtr::<fn()>::new(|| unsafe {
        let mut s: Option<Box<i32>> = Some(Box::new(3));
        let mut opt: () = unimplemented!();
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
    (unsafe { test_ref_23() });
    (unsafe { test_ref_24() });
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
    (unsafe { test_25() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const value_4);
    std::cell::LazyCell::force(&*&raw const value_5);
    std::cell::LazyCell::force(&*&raw const value_6);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_impl_7);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_impl_8);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_impl_9);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_impl_10);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_impl_11);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_impl_12);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_impl_13);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_impl_14);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_15);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_16);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_17);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_18);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_19);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_20);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_21);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_22);
}
