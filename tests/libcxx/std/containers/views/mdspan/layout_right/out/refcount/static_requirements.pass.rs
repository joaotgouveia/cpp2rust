extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int__int_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned_int__unsigned_int_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_layout_right__std_layout_right_ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_layout_right_mapping_std_extents_int____std_layout_right_mapping_std_extents_int___
{}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_signed_char__signed_char_ {}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned_char__unsigned_char_ {}
pub type anon_7 = ();
pub const anon_7_value: anon_7 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_layout_right_mapping_std_extents_signed_char__4__5____std_layout_right_mapping_std_extents_signed_char__4__5___
{}
pub type anon_8 = ();
pub const anon_8_value: anon_8 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_layout_right_mapping_std_extents_unsigned_int__18446744073709551615__4____std_layout_right_mapping_std_extents_unsigned_int__18446744073709551615__4___
{}
pub type anon_9 = ();
pub const anon_9_value: anon_9 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_layout_right_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615____std_layout_right_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615___
{}
pub fn test_mapping_requirements_10(_a0: ()) {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
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
pub fn test_mapping_requirements_11(_a0: ()) {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
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
pub fn test_mapping_requirements_12(_a0: ()) {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
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
pub fn test_mapping_requirements_13(_a0: ()) {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
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
pub fn test_layout_mapping_requirements_14() {
    ({ test_mapping_requirements_10(<()>::default()) });
}
pub fn test_layout_mapping_requirements_15() {
    ({ test_mapping_requirements_11(<()>::default()) });
}
pub fn test_layout_mapping_requirements_16() {
    ({ test_mapping_requirements_12(<()>::default()) });
}
pub fn test_layout_mapping_requirements_17() {
    ({ test_mapping_requirements_13(<()>::default()) });
}
pub fn test_layout_mapping_right_18() {
    ({ test_layout_mapping_requirements_14() });
}
pub fn test_layout_mapping_right_19() {
    ({ test_layout_mapping_requirements_15() });
}
pub fn test_layout_mapping_right_20() {
    ({ test_layout_mapping_requirements_16() });
}
pub fn test_layout_mapping_right_21() {
    ({ test_layout_mapping_requirements_17() });
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    let mut D: () = dynamic_extent_22.with(|rc| *rc.borrow());
    ({ test_layout_mapping_right_18() });
    ({ test_layout_mapping_right_19() });
    ({ test_layout_mapping_right_20() });
    ({ test_layout_mapping_right_21() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
