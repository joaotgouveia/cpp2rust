extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_path_method_0() {
    let p: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __v: Vec<i8> = (Ptr::<i8>::from_string_literal(b"foo/bar/baz.exe") as Ptr<i8>)
            .path_bytes()
            .into_iter()
            .map(|__b| __b as i8)
            .collect();
        __v.push(0);
        __v
    }));
    let p2: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __v: Vec<i8> = (Ptr::<i8>::from_string_literal(b"abc") as Ptr<i8>)
            .path_bytes()
            .into_iter()
            .map(|__b| __b as i8)
            .collect();
        __v.push(0);
        __v
    }));
    {
        let nce: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(
            true,
            "std::is_same<decltype(e.path()),  const path&>::value"
        );;
        const _: () = assert!(
            true,
            "std::is_same<decltype(nce.path()), const path&>::value"
        );;
        const _: () = assert!(true, "noexcept(e.path()) && noexcept(nce.path())");;
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let pref: Ptr<Vec<i8>> = unimplemented!();
        assert!(unimplemented!());
        assert!(({ (pref).clone() } == { (unimplemented!()) }));
        unimplemented!();
        assert!(unimplemented!());
        assert!(({ (pref).clone() } == { (unimplemented!()) }));
    }
}
pub fn test_path_conversion_1() {
    let p: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __v: Vec<i8> = (Ptr::<i8>::from_string_literal(b"foo/bar/baz.exe") as Ptr<i8>)
            .path_bytes()
            .into_iter()
            .map(|__b| __b as i8)
            .collect();
        __v.push(0);
        __v
    }));
    let p2: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __v: Vec<i8> = (Ptr::<i8>::from_string_literal(b"abc") as Ptr<i8>)
            .path_bytes()
            .into_iter()
            .map(|__b| __b as i8)
            .collect();
        __v.push(0);
        __v
    }));
    {
        let nce: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(
            true,
            "std::is_convertible<directory_entry&,        path const&>::value"
        );;
        const _: () = assert!(
            true,
            "std::is_convertible<directory_entry const&,  path const&>::value"
        );;
        const _: () = assert!(
            true,
            "std::is_convertible<directory_entry &&,      path const&>::value"
        );;
        const _: () = assert!(
            true,
            "std::is_convertible<directory_entry const&&, path const&>::value"
        );;
        const _: () = assert!(
            true,
            "!std::is_convertible<directory_entry&,        path&>::value"
        );;
        const _: () = assert!(
            true,
            "!std::is_convertible<directory_entry const&,  path&>::value"
        );;
        const _: () = assert!(
            true,
            "!std::is_convertible<directory_entry &&,      path&>::value"
        );;
        const _: () = assert!(
            true,
            "!std::is_convertible<directory_entry const&&, path&>::value"
        );;
        const _: () = assert!(
            true,
            "noexcept(e.operator fs::path const&()) &&\n                  noexcept(e.operator fs::path const&())"
        );;
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let pref: Ptr<Vec<i8>> = unimplemented!();
        assert!(({ (pref).clone() } == { (unimplemented!()) }));
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let pref: Ptr<Vec<i8>> = unimplemented!();
        assert!(({ (pref).clone() } == { (unimplemented!()) }));
        unimplemented!();
        assert!(unimplemented!());
        assert!(({ (pref).clone() } == { (unimplemented!()) }));
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
    ({ test_path_method_0() });
    ({ test_path_conversion_1() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
