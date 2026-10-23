extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    let mut t: () = 0_();
    (&(t));
}
pub fn test_1() {
    let mut t: () = 0_();
    (&(t));
}
pub fn test_2() {
    let mut t: () = 0_();
    (&(t));
}
pub fn test_3() {
    let mut t: () = 0_();
    (&(t));
}
pub fn test_4() {
    let mut t: () = 0_();
    (&(t));
}
pub fn test_5() {
    let mut t: () = 0_();
    (&(t));
}
pub fn test_6() {
    let mut t: () = 0_();
    (&(t));
}
pub fn test_7() {
    let mut t: () = 0_();
    (&(t));
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
    ({ test_2() });
    ({ test_3() });
    ({ test_4() });
    ({ test_5() });
    ({ test_6() });
    ({ test_7() });
    ({ test_0() });
    ({ test_1() });
    ({ test_2() });
    ({ test_3() });
    ({ test_4() });
    ({ test_5() });
    ({ test_6() });
    ({ test_7() });
    ({ test_0() });
    ({ test_3() });
    ({ test_3() });
    ({ test_3() });
    ({ test_4() });
    ({ test_7() });
    ({ test_7() });
    ({ test_7() });
    ({ test_3() });
    ({ test_7() });
    ({ test_3() });
    ({ test_7() });
    {
        let i1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (&(*i1.borrow_mut()));
    }
    let mut i: i64 = 0_i64;
    (&(i));
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::imaxabs(i)), std::intmax_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::imaxdiv(i, i)), std::imaxdiv_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::strtoimax(\"\", (char**)0, 0)), std::intmax_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::strtoumax(\"\", (char**)0, 0)), std::uintmax_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::wcstoimax(L\"\", (wchar_t**)0, 0)), std::intmax_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::wcstoumax(L\"\", (wchar_t**)0, 0)), std::uintmax_t>::value)"
    );;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
