extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_constexpr_0(_a0: std_destroying_delete_t) -> bool {
    let _a0: Value<std_destroying_delete_t> = Rc::new(RefCell::new(_a0));
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
    const _: () = assert!(
        true,
        "std::is_default_constructible<std::destroying_delete_t>::value"
    );;
    const _: () = assert!(true, "!test_convertible<std::destroying_delete_t>()");;
    let dd: Value<std_destroying_delete_t> =
        Rc::new(RefCell::new(<std_destroying_delete_t>::default()));
    const _: () = assert!(true, "&dd != &std::destroying_delete");;
    const _: () = assert!(true, "test_constexpr(std::destroying_delete)");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
