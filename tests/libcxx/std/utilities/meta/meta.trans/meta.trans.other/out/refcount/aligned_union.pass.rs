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
pub struct test_macros_detail_is_same_std_aligned_storage_12__4__type__std_aligned_storage_12__4__type_
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_aligned_storage_16__8__type__std_aligned_storage_16__8__type_
{}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_aligned_storage_4__4__type__std_aligned_storage_4__4__type_
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
        const _: () = assert!(true, "std::alignment_of<T1>::value == 2");;
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
        const _: () = assert!((4usize == 4_()), "sizeof(T1) == 4");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
