extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
const _: () = assert!(true, "std::is_empty_v<std::ranges::dangling>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct S_0_ {}
impl CppMove for S_0_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct S_1_ {}
impl CppMove for S_1_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct S_2_ {}
impl CppMove for S_2_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(
    true,
    "std::is_nothrow_constructible_v<std::ranges::dangling>"
);
const _: () = assert!(
    true,
    "std::is_nothrow_constructible_v<std::ranges::dangling, S<0>>"
);
const _: () = assert!(
    true,
    "std::is_nothrow_constructible_v<std::ranges::dangling, S<0>, S<1>>"
);
const _: () = assert!(
    true,
    "std::is_nothrow_constructible_v<std::ranges::dangling, S<0>, S<1>, S<2>>"
);
pub fn test_dangling_0() -> bool {
    let a: Value<std_ranges_dangling> = Rc::new(RefCell::new(<std_ranges_dangling>::default()));
    let b: Value<std_ranges_dangling> = Rc::new(RefCell::new(<S_0_>::default()));
    let c: Value<std_ranges_dangling> = Rc::new(RefCell::new({
        let __tmp_0: Value<S_0_> = Rc::new(RefCell::new(<S_0_>::default()));
        let __tmp_1: Value<S_1_> = Rc::new(RefCell::new(<S_1_>::default()));
        std_ranges_dangling::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
    }));
    let d: Value<std_ranges_dangling> = Rc::new(RefCell::new({
        let __tmp_2: Value<S_0_> = Rc::new(RefCell::new(<S_0_>::default()));
        let __tmp_3: Value<S_1_> = Rc::new(RefCell::new(<S_1_>::default()));
        let __tmp_4: Value<S_2_> = Rc::new(RefCell::new(<S_2_>::default()));
        std_ranges_dangling::new_2({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() }, {
            __tmp_4.as_pointer()
        })
    }));
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    const _: () = assert!(true, "test_dangling()");;
    ({ test_dangling_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
