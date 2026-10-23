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
pub struct NoexceptSwap {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MayThrowSwap {}
const _: () = assert!(true, "ADLSwapNoexcept<std::unexpected<NoexceptSwap>>");
const _: () = assert!(!(false), "!ADLSwapNoexcept<std::unexpected<MayThrowSwap>>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NonSwappable {}
const _: () = assert!(true, "std::is_swappable_v<std::unexpected<int>>");
const _: () = assert!(true, "std::is_swappable_v<std::unexpected<MayThrowSwap>>");
const _: () = assert!(true, "!std::is_swappable_v<std::unexpected<NonSwappable>>");
pub fn swap_0(x: Ptr<ADLSwap>, y: Ptr<ADLSwap>) {
    ({
        let ___x: Ptr<()> = field_ptr!(x, i);
        let ___y: Ptr<()> = field_ptr!(y, i);
        swap_1(___x, ___y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct ADLSwap {
    #[offset(0)]
    pub i: (),
}
impl ADLSwap {
    pub fn new(mut ii: ()) -> Self {
        Self { i: ii }
    }
}
pub fn test_2() -> () {
    let unex1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let unex2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
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
    ({ test_2() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
