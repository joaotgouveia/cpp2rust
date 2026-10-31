extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn search_0(mut pat: Ptr<i8>, mut f: (), mut target: Ptr<i8>, mut expected: bool) {
    let re: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (unimplemented!() as i32) } == { (expected as i32) }));
    if expected {
        assert!((unimplemented!() == 1_()));
        assert!((unimplemented!() == 3_()));
        assert!(
            Ptr::<i8>::from_string_literal(b"foo").with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
    } else {
        assert!((unimplemented!() == 0_()));
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
        let mut pat: Ptr<i8> = Ptr::<i8>::from_string_literal(b"^foo");
        let mut target: Ptr<i8> = Ptr::<i8>::from_string_literal(b"foo");
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
    }
    {
        let mut pat: Ptr<i8> = Ptr::<i8>::from_string_literal(b"^foo");
        let mut target: Ptr<i8> = Ptr::<i8>::from_string_literal(b"\nfoo");
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
    }
    {
        let mut pat: Ptr<i8> = Ptr::<i8>::from_string_literal(b"^foo");
        let mut target: Ptr<i8> = Ptr::<i8>::from_string_literal(b"bar\nfoo");
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
    }
    {
        let mut pat: Ptr<i8> = Ptr::<i8>::from_string_literal(b"foo$");
        let mut target: Ptr<i8> = Ptr::<i8>::from_string_literal(b"foo");
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
    }
    {
        let mut pat: Ptr<i8> = Ptr::<i8>::from_string_literal(b"foo$");
        let mut target: Ptr<i8> = Ptr::<i8>::from_string_literal(b"foo\n");
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
    }
    {
        let mut pat: Ptr<i8> = Ptr::<i8>::from_string_literal(b"foo$");
        let mut target: Ptr<i8> = Ptr::<i8>::from_string_literal(b"foo\nbar");
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
    }
    {
        let mut pat: Ptr<i8> = Ptr::<i8>::from_string_literal(b"^foo");
        let mut target: Ptr<i8> = Ptr::<i8>::from_string_literal(b"foo");
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
    }
    {
        let mut pat: Ptr<i8> = Ptr::<i8>::from_string_literal(b"^foo");
        let mut target: Ptr<i8> = Ptr::<i8>::from_string_literal(b"\rfoo");
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
    }
    {
        let mut pat: Ptr<i8> = Ptr::<i8>::from_string_literal(b"^foo");
        let mut target: Ptr<i8> = Ptr::<i8>::from_string_literal(b"bar\rfoo");
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
    }
    {
        let mut pat: Ptr<i8> = Ptr::<i8>::from_string_literal(b"foo$");
        let mut target: Ptr<i8> = Ptr::<i8>::from_string_literal(b"foo");
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
    }
    {
        let mut pat: Ptr<i8> = Ptr::<i8>::from_string_literal(b"foo$");
        let mut target: Ptr<i8> = Ptr::<i8>::from_string_literal(b"foo\r");
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
    }
    {
        let mut pat: Ptr<i8> = Ptr::<i8>::from_string_literal(b"foo$");
        let mut target: Ptr<i8> = Ptr::<i8>::from_string_literal(b"foo\rbar");
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), true) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
        ({ search_0((pat).clone(), unimplemented!(), (target).clone(), false) });
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
