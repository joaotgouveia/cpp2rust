extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(sv0: ()) -> () {
    let sv0: Value<()> = Rc::new(RefCell::new(sv0));
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return (unimplemented!() == unimplemented!()) && (unimplemented!() == unimplemented!());
}
pub fn test_1(sv0: ()) -> () {
    let sv0: Value<()> = Rc::new(RefCell::new(sv0));
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return (unimplemented!() == unimplemented!()) && (unimplemented!() == unimplemented!());
}
pub fn test_2(sv0: ()) -> () {
    let sv0: Value<()> = Rc::new(RefCell::new(sv0));
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return (unimplemented!() == unimplemented!()) && (unimplemented!() == unimplemented!());
}
pub fn test_3(sv0: ()) -> () {
    let sv0: Value<()> = Rc::new(RefCell::new(sv0));
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return (unimplemented!() == unimplemented!()) && (unimplemented!() == unimplemented!());
}
pub fn test_4(sv0: ()) -> () {
    let sv0: Value<()> = Rc::new(RefCell::new(sv0));
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return (unimplemented!() == unimplemented!()) && (unimplemented!() == unimplemented!());
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "test<std::string_view>({{\"abc\", 3}})");;
    const _: () = assert!(true, "test<std::u8string_view>({{u8\"abc\", 3}})");;
    const _: () = assert!(true, "test<std::u16string_view>({{u\"abc\", 3}})");;
    const _: () = assert!(true, "test<std::u32string_view>({{U\"abc\", 3}})");;
    const _: () = assert!(true, "test<std::wstring_view>({{L\"abc\", 3}})");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
