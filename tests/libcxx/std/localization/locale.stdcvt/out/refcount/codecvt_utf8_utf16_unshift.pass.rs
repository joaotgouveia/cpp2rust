extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

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
    {
        let c: Value<std_codecvt_utf8_utf16_wchar_t__1114111__0_> = Rc::new(RefCell::new(
            <std_codecvt_utf8_utf16_wchar_t__1114111__0_>::default(),
        ));
        let n: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            0_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ])));
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut np: Ptr<()> = Ptr::<()>::null();
        let mut r: () = unimplemented!();
        unimplemented!();
    }
    {
        let c: Value<std_codecvt_utf8_utf16_char16_t__1114111__0_> = Rc::new(RefCell::new(
            <std_codecvt_utf8_utf16_char16_t__1114111__0_>::default(),
        ));
        let n: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            0_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ])));
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut np: Ptr<()> = Ptr::<()>::null();
        let mut r: () = unimplemented!();
        unimplemented!();
    }
    {
        let c: Value<std_codecvt_utf8_utf16_char32_t__1114111__0_> = Rc::new(RefCell::new(
            <std_codecvt_utf8_utf16_char32_t__1114111__0_>::default(),
        ));
        let n: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            0_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ])));
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut np: Ptr<()> = Ptr::<()>::null();
        let mut r: () = unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
