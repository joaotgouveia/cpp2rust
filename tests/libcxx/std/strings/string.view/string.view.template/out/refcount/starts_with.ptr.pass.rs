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
    pub fn eq(mut c1: i8, mut c2: i8) -> bool {
        return ((c1 as i32) == (c2 as i32));
    }
    pub fn lt(mut c1: i8, mut c2: i8) -> bool {
        return ((c1 as i32) < (c2 as i32));
    }
    pub fn compare(mut s1: Ptr<i8>, mut s2: Ptr<i8>, mut n: usize) -> i32 {
        'loop_: while (n != 0) {
            if ({
                let _c1: i8 = (s1.read());
                let _c2: i8 = (s2.read());
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return -1_i32;
            }
            if ({
                let _c1: i8 = (s2.read());
                let _c2: i8 = (s1.read());
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return 1;
            }
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return 0;
    }
    pub fn length(mut s: Ptr<i8>) -> usize {
        let len: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while !({
            let _c1: i8 = (s.read());
            let _c2: i8 = 0_i8;
            constexpr_char_traits_char_::eq(_c1, _c2)
        }) {
            (*len.borrow_mut()).prefix_inc();
            s.prefix_inc();
        }
        return (*len.borrow());
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
    {
        let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"abcde");
        let sv0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let svNot: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
    }
    {
        let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"abcde");
        let sv0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let svNot: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv0.starts_with(\"\")");;
        const _: () = assert!(true, "!sv0.starts_with(\"a\")");;
        const _: () = assert!(true, "sv1.starts_with(\"\")");;
        const _: () = assert!(true, "sv1.starts_with(\"a\")");;
        const _: () = assert!(true, "!sv1.starts_with(\"ab\")");;
        const _: () = assert!(true, "!sv1.starts_with(\"abc\")");;
        const _: () = assert!(true, "!sv1.starts_with(\"abcd\")");;
        const _: () = assert!(true, "!sv1.starts_with(\"abcde\")");;
        const _: () = assert!(true, "!sv1.starts_with(\"def\")");;
        const _: () = assert!(true, "sv2.starts_with(s + 5)");;
        const _: () = assert!(true, "sv2.starts_with(\"a\")");;
        const _: () = assert!(true, "sv2.starts_with(\"ab\")");;
        const _: () = assert!(true, "!sv2.starts_with(\"abc\")");;
        const _: () = assert!(true, "!sv2.starts_with(\"abcd\")");;
        const _: () = assert!(true, "!sv2.starts_with(\"abcde\")");;
        const _: () = assert!(true, "!sv2.starts_with(\"def\")");;
        const _: () = assert!(true, "svNot.starts_with(\"\")");;
        const _: () = assert!(true, "!svNot.starts_with(\"a\")");;
        const _: () = assert!(true, "!svNot.starts_with(\"ab\")");;
        const _: () = assert!(true, "!svNot.starts_with(\"abc\")");;
        const _: () = assert!(true, "!svNot.starts_with(\"abcd\")");;
        const _: () = assert!(true, "!svNot.starts_with(\"abcde\")");;
        const _: () = assert!(true, "svNot.starts_with(\"def\")");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
