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
pub struct constexpr_char_traits_char_ {}
impl constexpr_char_traits_char_ {
    pub fn lt(mut c1: (), mut c2: ()) -> () {
        return (c1 < c2);
    }
    pub fn compare(mut s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> () {
        'loop_: while (n != 0) {
            if ({
                let _c1: () = (s1.read());
                let _c2: () = (s2.read());
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return -1_();
            }
            if ({
                let _c1: () = (s2.read());
                let _c2: () = (s1.read());
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return 1_();
            }
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return 0_();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {
        let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"abcde");
        let sv0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv5: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let svNot: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
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
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"abcde");
        let sv0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv5: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let svNot: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv0.ends_with(sv0)");;
        const _: () = assert!(true, "!sv0.ends_with(sv1)");;
        const _: () = assert!(true, "sv1.ends_with(sv0)");;
        const _: () = assert!(true, "sv1.ends_with(sv1)");;
        const _: () = assert!(true, "!sv1.ends_with(sv2)");;
        const _: () = assert!(true, "!sv1.ends_with(sv3)");;
        const _: () = assert!(true, "!sv1.ends_with(sv4)");;
        const _: () = assert!(true, "!sv1.ends_with(sv5)");;
        const _: () = assert!(true, "!sv1.ends_with(svNot)");;
        const _: () = assert!(true, "sv2.ends_with(sv0)");;
        const _: () = assert!(true, "sv2.ends_with(sv1)");;
        const _: () = assert!(true, "sv2.ends_with(sv2)");;
        const _: () = assert!(true, "!sv2.ends_with(sv3)");;
        const _: () = assert!(true, "!sv2.ends_with(sv4)");;
        const _: () = assert!(true, "!sv2.ends_with(sv5)");;
        const _: () = assert!(true, "!sv2.ends_with(svNot)");;
        const _: () = assert!(true, "svNot.ends_with(sv0)");;
        const _: () = assert!(true, "!svNot.ends_with(sv1)");;
        const _: () = assert!(true, "!svNot.ends_with(sv2)");;
        const _: () = assert!(true, "!svNot.ends_with(sv3)");;
        const _: () = assert!(true, "!svNot.ends_with(sv4)");;
        const _: () = assert!(true, "!svNot.ends_with(sv5)");;
        const _: () = assert!(true, "svNot.ends_with(svNot)");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
