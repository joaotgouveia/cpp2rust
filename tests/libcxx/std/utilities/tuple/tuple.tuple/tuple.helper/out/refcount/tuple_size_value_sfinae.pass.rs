extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn has_value_0(mut _a0: i64) -> bool {
    return false;
}
pub fn has_value_1(mut _a0: i64) -> bool {
    return false;
}
pub fn has_value_2(mut _a0: i64) -> bool {
    return false;
}
pub fn has_value_3() -> bool {
    return ({ has_value_0(0_i64) });
}
pub fn has_value_4() -> bool {
    return ({ has_value_1(0_i64) });
}
pub fn has_value_5() -> bool {
    return ({ has_value_2(0_i64) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Dummy {}
impl CppMove for Dummy {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    const _: () = assert!(true, "has_value<std::tuple<int> const>()");;
    const _: () = assert!(true, "has_value<std::pair<int, long> volatile>()");;
    const _: () = assert!(true, "!has_value<int>()");;
    const _: () = assert!(true, "!has_value<const int>()");;
    const _: () = assert!(true, "!has_value<volatile void>()");;
    const _: () = assert!(true, "!has_value<const volatile std::tuple<int>&>()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
