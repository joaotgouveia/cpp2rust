extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor)]
#[byte_size(1)]
pub struct A {}
impl A {
    pub fn new() -> Self {
        Self {}
    }
    pub fn move_from(_a0: Ptr<A>) -> Self {
        Self {}
    }
}
impl Default for A {
    fn default() -> Self {
        { A::new() }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(1)]
pub struct legacy {}
impl legacy {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for legacy {
    fn default() -> Self {
        { legacy::new() }
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
    let mut i: () = 0_();
    let mut ci: () = 0_();
    let mut l: legacy = legacy::new();
    let mut a: A = A::new();
    let mut ca: A = A::new();
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::move_if_noexcept(i)), int&&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::move_if_noexcept(ci)), const int&&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::move_if_noexcept(a)), A&&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::move_if_noexcept(ca)), const A&&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::move_if_noexcept(l)), const legacy&>::value)"
    );;
    let mut i1: () = 23_();
    let mut i2: () = i1;
    const _: () = assert!(true, "i2 == 23");;
    return 0_();
}
pub trait AImpl {
    fn copy_assign(&self, _a0: Ptr<A>) -> Ptr<A> {
        unimplemented!()
    }
}
impl AImpl for Ptr<A> {}
pub fn __cpp2rust_init_globals() {}
