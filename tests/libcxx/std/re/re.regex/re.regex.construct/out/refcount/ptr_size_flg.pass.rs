extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(mut p: Ptr<()>, mut len: (), mut f: (), mut mc: ()) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"\\(a\\)"),
            5_(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"\\(a[bc]\\)"),
            9_(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"\\(a\\([bc]\\)\\)"),
            13_(),
            unimplemented!(),
            2_(),
        )
    });
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"(a([bc]))"),
            9_(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"\\(a\\)"),
            5_(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"\\(a[bc]\\)"),
            9_(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"\\(a\\([bc]\\)\\)"),
            13_(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"(a([bc]))"),
            9_(),
            unimplemented!(),
            2_(),
        )
    });
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"\\(a\\)"),
            5_(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"\\(a[bc]\\)"),
            9_(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"\\(a\\([bc]\\)\\)"),
            13_(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"(a([bc]))"),
            9_(),
            unimplemented!(),
            2_(),
        )
    });
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"\\(a\\)"),
            5_(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"\\(a[bc]\\)"),
            9_(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"\\(a\\([bc]\\)\\)"),
            13_(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"(a([bc]))"),
            9_(),
            unimplemented!(),
            2_(),
        )
    });
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"\\(a\\)"),
            5_(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"\\(a[bc]\\)"),
            9_(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"\\(a\\([bc]\\)\\)"),
            13_(),
            unimplemented!(),
            2_(),
        )
    });
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"(a([bc]))"),
            9_(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"\\(a\\)"),
            5_(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"\\(a[bc]\\)"),
            9_(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"\\(a\\([bc]\\)\\)"),
            13_(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test_0(
            Ptr::<()>::from_string_literal(b"(a([bc]))"),
            9_(),
            unimplemented!(),
            2_(),
        )
    });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
