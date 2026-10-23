extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn testType_0() {
    const _: () = assert!(true, "std::ranges::range<std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::range<const std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::view<std::ranges::empty_view<T>>");;
    let empty: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn testType_1() {
    const _: () = assert!(true, "std::ranges::range<std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::range<const std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::view<std::ranges::empty_view<T>>");;
    let empty: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn testType_2() {
    const _: () = assert!(true, "std::ranges::range<std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::range<const std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::view<std::ranges::empty_view<T>>");;
    let empty: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn testType_3() {
    const _: () = assert!(true, "std::ranges::range<std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::range<const std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::view<std::ranges::empty_view<T>>");;
    let empty: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn testType_4() {
    const _: () = assert!(true, "std::ranges::range<std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::range<const std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::view<std::ranges::empty_view<T>>");;
    let empty: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn testType_5() {
    const _: () = assert!(true, "std::ranges::range<std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::range<const std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::view<std::ranges::empty_view<T>>");;
    let empty: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Empty {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct BigType {
    #[offset(0)]
    #[byte_size(8)]
    pub buff: Box<[()]>,
}
impl Default for BigType {
    fn default() -> Self {
        {
            BigType {
                buff: (0..8).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
pub fn test_6() -> () {
    const _: () = assert!(!(false), "!ValidEmptyView<int&>");;
    const _: () = assert!(!(false), "!ValidEmptyView<void>");;
    ({ testType_0() });
    ({ testType_1() });
    ({ testType_2() });
    ({ testType_3() });
    ({ testType_4() });
    ({ testType_5() });
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
    ({ test_6() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
