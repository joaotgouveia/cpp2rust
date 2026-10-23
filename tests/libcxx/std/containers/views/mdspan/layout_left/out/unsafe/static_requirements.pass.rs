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
pub struct test_macros_detail_is_same_int__int_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_int__unsigned_int_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_layout_left__std_layout_left_ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_layout_left_mapping_std_extents_int____std_layout_left_mapping_std_extents_int___
{}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_signed_char__signed_char_ {}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_char__unsigned_char_ {}
pub type anon_7 = ();
pub const anon_7_value: anon_7 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_layout_left_mapping_std_extents_signed_char__4__5____std_layout_left_mapping_std_extents_signed_char__4__5___
{}
pub type anon_8 = ();
pub const anon_8_value: anon_8 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_layout_left_mapping_std_extents_unsigned_int__18446744073709551615__4____std_layout_left_mapping_std_extents_unsigned_int__18446744073709551615__4___
{}
pub type anon_9 = ();
pub const anon_9_value: anon_9 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_layout_left_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615____std_layout_left_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615___
{}
pub unsafe fn test_mapping_requirements_10(mut _a0: ()) {
    const _: () = assert!(true, "std::__mdspan_detail::__is_extents_v<E>");;
    const _: () = assert!(true, "std::is_copy_constructible_v<M>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<M>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<M>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<M>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>().extents()), const E&>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>()(Idxs...)), typename M::index_type>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>().required_span_size()), typename M::index_type>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>().is_unique()), bool>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>().is_exhaustive()), bool>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>().is_strided()), bool>"
    );;
    {}
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(M::is_always_unique()), bool>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(M::is_always_exhaustive()), bool>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(M::is_always_strided()), bool>"
    );;
}
pub unsafe fn test_mapping_requirements_11(mut _a0: ()) {
    const _: () = assert!(true, "std::__mdspan_detail::__is_extents_v<E>");;
    const _: () = assert!(true, "std::is_copy_constructible_v<M>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<M>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<M>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<M>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>().extents()), const E&>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>()(Idxs...)), typename M::index_type>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>().required_span_size()), typename M::index_type>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>().is_unique()), bool>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>().is_exhaustive()), bool>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>().is_strided()), bool>"
    );;
    {
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::declval<M>().stride(0)), typename M::index_type>"
        );;
    }
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(M::is_always_unique()), bool>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(M::is_always_exhaustive()), bool>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(M::is_always_strided()), bool>"
    );;
}
pub unsafe fn test_mapping_requirements_12(mut _a0: ()) {
    const _: () = assert!(true, "std::__mdspan_detail::__is_extents_v<E>");;
    const _: () = assert!(true, "std::is_copy_constructible_v<M>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<M>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<M>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<M>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>().extents()), const E&>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>()(Idxs...)), typename M::index_type>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>().required_span_size()), typename M::index_type>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>().is_unique()), bool>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>().is_exhaustive()), bool>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>().is_strided()), bool>"
    );;
    {
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::declval<M>().stride(0)), typename M::index_type>"
        );;
    }
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(M::is_always_unique()), bool>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(M::is_always_exhaustive()), bool>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(M::is_always_strided()), bool>"
    );;
}
pub unsafe fn test_mapping_requirements_13(mut _a0: ()) {
    const _: () = assert!(true, "std::__mdspan_detail::__is_extents_v<E>");;
    const _: () = assert!(true, "std::is_copy_constructible_v<M>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<M>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<M>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<M>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>().extents()), const E&>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>()(Idxs...)), typename M::index_type>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>().required_span_size()), typename M::index_type>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>().is_unique()), bool>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>().is_exhaustive()), bool>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(std::declval<M>().is_strided()), bool>"
    );;
    {
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::declval<M>().stride(0)), typename M::index_type>"
        );;
    }
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(M::is_always_unique()), bool>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(M::is_always_exhaustive()), bool>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(M::is_always_strided()), bool>"
    );;
}
pub unsafe fn test_layout_mapping_requirements_14() {
    (unsafe { test_mapping_requirements_10(unsafe { std::mem::zeroed::<()>() }) });
}
pub unsafe fn test_layout_mapping_requirements_15() {
    (unsafe { test_mapping_requirements_11(unsafe { std::mem::zeroed::<()>() }) });
}
pub unsafe fn test_layout_mapping_requirements_16() {
    (unsafe { test_mapping_requirements_12(unsafe { std::mem::zeroed::<()>() }) });
}
pub unsafe fn test_layout_mapping_requirements_17() {
    (unsafe { test_mapping_requirements_13(unsafe { std::mem::zeroed::<()>() }) });
}
pub unsafe fn test_layout_mapping_left_18() {
    (unsafe { test_layout_mapping_requirements_14() });
}
pub unsafe fn test_layout_mapping_left_19() {
    (unsafe { test_layout_mapping_requirements_15() });
}
pub unsafe fn test_layout_mapping_left_20() {
    (unsafe { test_layout_mapping_requirements_16() });
}
pub unsafe fn test_layout_mapping_left_21() {
    (unsafe { test_layout_mapping_requirements_17() });
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
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_22));
    (unsafe { test_layout_mapping_left_18() });
    (unsafe { test_layout_mapping_left_19() });
    (unsafe { test_layout_mapping_left_20() });
    (unsafe { test_layout_mapping_left_21() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
