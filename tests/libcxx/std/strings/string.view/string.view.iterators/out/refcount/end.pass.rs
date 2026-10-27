extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(s: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    let cs: Ptr<()> = s.as_pointer();
    let mut e: () = unimplemented!();
    let mut ce1: () = unimplemented!();
    let mut ce2: () = unimplemented!();
    if unimplemented!() {
        unimplemented!();
        unimplemented!();
        unimplemented!();
    } else {
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_1(s: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    let cs: Ptr<()> = s.as_pointer();
    let mut e: () = unimplemented!();
    let mut ce1: () = unimplemented!();
    let mut ce2: () = unimplemented!();
    if unimplemented!() {
        unimplemented!();
        unimplemented!();
        unimplemented!();
    } else {
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_2(s: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    let cs: Ptr<()> = s.as_pointer();
    let mut e: () = unimplemented!();
    let mut ce1: () = unimplemented!();
    let mut ce2: () = unimplemented!();
    if unimplemented!() {
        unimplemented!();
        unimplemented!();
        unimplemented!();
    } else {
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_3(s: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    let cs: Ptr<()> = s.as_pointer();
    let mut e: () = unimplemented!();
    let mut ce1: () = unimplemented!();
    let mut ce2: () = unimplemented!();
    if unimplemented!() {
        unimplemented!();
        unimplemented!();
        unimplemented!();
    } else {
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_4(s: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    let cs: Ptr<()> = s.as_pointer();
    let mut e: () = unimplemented!();
    let mut ce1: () = unimplemented!();
    let mut ce2: () = unimplemented!();
    if unimplemented!() {
        unimplemented!();
        unimplemented!();
        unimplemented!();
    } else {
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    ({ test_0(unimplemented!()) });
    ({ test_1(unimplemented!()) });
    ({ test_2(unimplemented!()) });
    ({ test_0(unimplemented!()) });
    ({ test_3(unimplemented!()) });
    ({ test_1(unimplemented!()) });
    ({ test_2(unimplemented!()) });
    ({ test_4(unimplemented!()) });
    ({ test_4(unimplemented!()) });
    {
        let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let u8sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let u16sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let u32sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv.begin() != sv.end()");;
        const _: () = assert!(true, "u8sv.begin() != u8sv.end()");;
        const _: () = assert!(true, "u16sv.begin() != u16sv.end()");;
        const _: () = assert!(true, "u32sv.begin() != u32sv.end()");;
        const _: () = assert!(true, "sv.begin() != sv.cend()");;
        const _: () = assert!(true, "u8sv.begin() != u8sv.cend()");;
        const _: () = assert!(true, "u16sv.begin() != u16sv.cend()");;
        const _: () = assert!(true, "u32sv.begin() != u32sv.cend()");;
        {
            let wsv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(true, "wsv.begin() != wsv.end()");;
            const _: () = assert!(true, "wsv.begin() != wsv.cend()");;
        }
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
