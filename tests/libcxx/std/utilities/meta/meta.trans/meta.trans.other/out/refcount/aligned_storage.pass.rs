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
pub struct test_macros_detail_is_same_std_aligned_storage_10__1__type__std_aligned_storage_10__1__type_
{}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_aligned_storage_10__2__type__std_aligned_storage_10__2__type_
{}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_aligned_storage_10__4__type__std_aligned_storage_10__4__type_
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_aligned_storage_10__8__type__std_aligned_storage_10__8__type_
{}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_aligned_storage_10__16__type__std_aligned_storage_10__16__type_
{}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_aligned_storage_10__32__type__std_aligned_storage_10__32__type_
{}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_aligned_storage_20__32__type__std_aligned_storage_20__32__type_
{}
pub type anon_7 = ();
pub const anon_7_value: anon_7 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_aligned_storage_40__32__type__std_aligned_storage_40__32__type_
{}
pub type anon_8 = ();
pub const anon_8_value: anon_8 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_aligned_storage_12__16__type__std_aligned_storage_12__16__type_
{}
pub type anon_9 = ();
pub const anon_9_value: anon_9 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_aligned_storage_1__1__type__std_aligned_storage_1__1__type_
{}
pub type anon_10 = ();
pub const anon_10_value: anon_10 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_aligned_storage_2__2__type__std_aligned_storage_2__2__type_
{}
pub type anon_11 = ();
pub const anon_11_value: anon_11 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_aligned_storage_3__2__type__std_aligned_storage_3__2__type_
{}
pub type anon_12 = ();
pub const anon_12_value: anon_12 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_aligned_storage_4__4__type__std_aligned_storage_4__4__type_
{}
pub type anon_13 = ();
pub const anon_13_value: anon_13 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_aligned_storage_5__4__type__std_aligned_storage_5__4__type_
{}
pub type anon_14 = ();
pub const anon_14_value: anon_14 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_aligned_storage_7__4__type__std_aligned_storage_7__4__type_
{}
pub type anon_15 = ();
pub const anon_15_value: anon_15 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_aligned_storage_8__8__type__std_aligned_storage_8__8__type_
{}
pub type anon_16 = ();
pub const anon_16_value: anon_16 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_aligned_storage_9__8__type__std_aligned_storage_9__8__type_
{}
pub type anon_17 = ();
pub const anon_17_value: anon_17 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_aligned_storage_15__8__type__std_aligned_storage_15__8__type_
{}
pub type anon_18 = ();
pub const anon_18_value: anon_18 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_aligned_storage_16__16__type__std_aligned_storage_16__16__type_
{}
pub type anon_19 = ();
pub const anon_19_value: anon_19 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_aligned_storage_17__16__type__std_aligned_storage_17__16__type_
{}
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
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 1");;
        const _: () = assert!((10usize == 10_()), "sizeof(T1) == 10");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 2");;
        const _: () = assert!((10usize == 10_()), "sizeof(T1) == 10");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 4");;
        const _: () = assert!((12usize == 12_()), "sizeof(T1) == 12");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 8");;
        const _: () = assert!((16usize == 16_()), "sizeof(T1) == 16");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 16");;
        const _: () = assert!((16usize == 16_()), "sizeof(T1) == 16");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 32");;
        const _: () = assert!((32usize == 32_()), "sizeof(T1) == 32");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 32");;
        const _: () = assert!((32usize == 32_()), "sizeof(T1) == 32");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 32");;
        const _: () = assert!((64usize == 64_()), "sizeof(T1) == 64");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 16");;
        const _: () = assert!((16usize == 16_()), "sizeof(T1) == 16");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 1");;
        const _: () = assert!((1usize == 1_()), "sizeof(T1) == 1");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 2");;
        const _: () = assert!((2usize == 2_()), "sizeof(T1) == 2");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 2");;
        const _: () = assert!((4usize == 4_()), "sizeof(T1) == 4");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 4");;
        const _: () = assert!((4usize == 4_()), "sizeof(T1) == 4");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 4");;
        const _: () = assert!((8usize == 8_()), "sizeof(T1) == 8");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 4");;
        const _: () = assert!((8usize == 8_()), "sizeof(T1) == 8");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 8");;
        const _: () = assert!((8usize == 8_()), "sizeof(T1) == 8");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 8");;
        const _: () = assert!((16usize == 16_()), "sizeof(T1) == 16");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 8");;
        const _: () = assert!((16usize == 16_()), "sizeof(T1) == 16");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        let mut alignment: () = if (16usize > 16_()) { 16_() } else { 16usize };
        const _: () = assert!(true, "std::alignment_of<T1>::value == alignment");;
        const _: () = assert!((16usize == 16_()), "sizeof(T1) == 16");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        let mut alignment: () = if (16usize > 16_()) { 16_() } else { 16usize };
        const _: () = assert!(true, "std::alignment_of<T1>::value == alignment");;
        const _: () = assert!(true, "sizeof(T1) == 16 + alignment");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 8");;
        const _: () = assert!((16usize == 16_()), "sizeof(T1) == 16");;
    }
    {
        let mut Align: () = 8192_();
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == Align");;
        const _: () = assert!(true, "sizeof(T1) == Align");;
    }
    {
        let mut Align: () = 65536_();
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == Align");;
        const _: () = assert!(true, "sizeof(T1) == Align");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
