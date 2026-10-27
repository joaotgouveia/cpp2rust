extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MyNS_A {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MyNS_B {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MyNS_C {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MyNS_D {}
pub fn swap_0(_a0: Ptr<MyNS_A>, _a1: Ptr<MyNS_A>) {}
pub fn swap_1(_a0: Ptr<MyNS_A>, _a1: Ptr<MyNS_B>) {}
pub fn swap_2(_a0: Ptr<MyNS_B>, _a1: Ptr<MyNS_A>) {}
pub fn swap_3(_a0: Ptr<MyNS_A>, _a1: Ptr<MyNS_C>) {}
pub fn swap_4(_a0: Ptr<MyNS_C>, _a1: Ptr<MyNS_A>) {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MyNS_M {}
pub fn swap_5(_a0: Ptr<MyNS_M>, _a1: Ptr<MyNS_M>) {}
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
        const _: () = assert!(true, "!std::is_nothrow_swappable_with<int, int>::value");;
        const _: () = assert!(true, "std::is_nothrow_swappable_with<int&, int&>::value");;
        const _: () = assert!(true, "std::is_nothrow_swappable_with<M, M>::value");;
        const _: () = assert!(
            true,
            "std::is_swappable_with<A&, A&>::value &&\n                      !std::is_nothrow_swappable_with<A&, A&>::value"
        );;
    }
    {
        const _: () = assert!(true, "std::is_nothrow_swappable_with<A&, B&>::value");;
        const _: () = assert!(
            true,
            "!std::is_nothrow_swappable_with<A&, C&>::value &&\n                      std::is_swappable_with<A&, C&>::value"
        );;
        const _: () = assert!(true, "!std::is_nothrow_swappable_with<D&, C&>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_swappable_with_v<void, int>");;
        const _: () = assert!(true, "!std::is_nothrow_swappable_with_v<int, void>");;
        const _: () = assert!(
            true,
            "!std::is_nothrow_swappable_with_v<const void, const volatile void>"
        );;
    }
    {
        const _: () = assert!(true, "std::is_nothrow_swappable_with_v<int&, int&>");;
        const _: () = assert!(true, "!std::is_nothrow_swappable_with_v<int&&, int&&>");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
