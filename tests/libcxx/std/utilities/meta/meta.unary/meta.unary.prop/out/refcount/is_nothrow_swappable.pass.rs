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
pub fn swap_0(_a0: Ptr<MyNS_A>, _a1: Ptr<MyNS_A>) {}
pub fn swap_1(_a0: Ptr<MyNS_B>, _a1: Ptr<MyNS_B>) {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MyNS_M {}
pub fn swap_2(_a0: Ptr<MyNS_M>, _a1: Ptr<MyNS_M>) {}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(1)]
pub struct MyNS_ThrowingMove {}
impl MyNS_ThrowingMove {
    pub fn move_from(_a0: Ptr<MyNS_ThrowingMove>) -> Self {
        Self {}
    }
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
    {
        const _: () = assert!(true, "std::is_nothrow_swappable<int>::value");;
        const _: () = assert!(true, "std::is_nothrow_swappable<int&>::value");;
        const _: () = assert!(true, "!std::is_nothrow_swappable<M>::value");;
        const _: () = assert!(true, "!std::is_nothrow_swappable<M&&>::value");;
    }
    {
        const _: () = assert!(true, "std::is_nothrow_swappable<A>::value");;
        const _: () = assert!(
            true,
            "!std::is_nothrow_swappable<B>::value\n                      && std::is_swappable<B>::value"
        );;
        const _: () = assert!(
            true,
            "!std::is_nothrow_swappable<ThrowingMove>::value\n                      && std::is_swappable<ThrowingMove>::value"
        );;
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_swappable<const A>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_swappable<void>::value");;
        const _: () = assert!(true, "!std::is_nothrow_swappable<int() const>::value");;
        const _: () = assert!(
            true,
            "!std::is_nothrow_swappable<int(int, ...) const &>::value"
        );;
    }
    {
        const _: () = assert!(true, "std::is_nothrow_swappable_v<int>");;
        const _: () = assert!(true, "!std::is_nothrow_swappable_v<void>");;
    }
    return 0_();
}
pub trait MyNS_ThrowingMoveImpl {
    fn move_assign(&self, _a0: Ptr<MyNS_ThrowingMove>) -> Ptr<MyNS_ThrowingMove>;
}
impl MyNS_ThrowingMoveImpl for Ptr<MyNS_ThrowingMove> {
    fn move_assign(&self, _a0: Ptr<MyNS_ThrowingMove>) -> Ptr<MyNS_ThrowingMove> {
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
