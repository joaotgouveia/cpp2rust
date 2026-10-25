extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
struct A_int__nat {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct A_int_ {
    #[offset(0)]
    #[byte_size(8)]
    pub t_: Ptr<()>,
}
impl A_int_ {
    pub fn new(mut t: Ptr<()>) -> Self {
        Self { t_: (t).clone() }
    }
    pub fn pointer_to(et: Ptr<()>) -> A_int_ {
        return A_int_::new({ (et).clone() });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
struct A_long__nat {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct A_long_ {
    #[offset(0)]
    #[byte_size(8)]
    pub t_: Ptr<()>,
}
impl A_long_ {
    pub fn new(mut t: Ptr<()>) -> Self {
        Self { t_: (t).clone() }
    }
    pub fn pointer_to(et: Ptr<()>) -> A_long_ {
        return A_long_::new({ (et).clone() });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
struct A_void__nat {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct A_void_ {
    #[offset(0)]
    #[byte_size(8)]
    pub t_: (),
}
impl A_void_ {}
pub fn test_0() -> () {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(
        true,
        "std::is_same<Pointer, decltype(std::pointer_traits<Pointer>::pointer_to(obj))>::value"
    );;
    let mut p: A_int_ = unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_1() -> () {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(
        true,
        "std::is_same<Pointer, decltype(std::pointer_traits<Pointer>::pointer_to(obj))>::value"
    );;
    let mut p: A_long_ = unimplemented!();
    unimplemented!();
    return true;
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
    ({ test_0() });
    ({ test_1() });
    {
        &(0_());
    }
    const _: () = assert!(true, "test<A<int> >()");;
    const _: () = assert!(true, "test<A<long> >()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
