extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(
    mut p: Ptr<i8>,
    mut len: usize,
    mut f: std_regex_constants_syntax_option_type,
    mut mc: u32,
) {
    let r: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
        std_basic_regex_char__std_regex_traits_char__::new_1({ (p).clone() }, { len }, { f }),
    ));
    assert!(({ ((({ (*r.borrow()).flags() }) as u32) == (f as u32)) }));
    assert!(({ (({ (*r.borrow()).mark_count() }) == mc) }));
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
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"\\(a\\)"),
            5_usize,
            basic_1.with(|rc| *rc.borrow()),
            1_u32,
        )
    });
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"\\(a[bc]\\)"),
            9_usize,
            basic_1.with(|rc| *rc.borrow()),
            1_u32,
        )
    });
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"\\(a\\([bc]\\)\\)"),
            13_usize,
            basic_1.with(|rc| *rc.borrow()),
            2_u32,
        )
    });
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"(a([bc]))"),
            9_usize,
            basic_1.with(|rc| *rc.borrow()),
            0_u32,
        )
    });
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"\\(a\\)"),
            5_usize,
            extended_2.with(|rc| *rc.borrow()),
            0_u32,
        )
    });
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"\\(a[bc]\\)"),
            9_usize,
            extended_2.with(|rc| *rc.borrow()),
            0_u32,
        )
    });
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"\\(a\\([bc]\\)\\)"),
            13_usize,
            extended_2.with(|rc| *rc.borrow()),
            0_u32,
        )
    });
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"(a([bc]))"),
            9_usize,
            extended_2.with(|rc| *rc.borrow()),
            2_u32,
        )
    });
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"\\(a\\)"),
            5_usize,
            ECMAScript_3.with(|rc| *rc.borrow()),
            0_u32,
        )
    });
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"\\(a[bc]\\)"),
            9_usize,
            ECMAScript_3.with(|rc| *rc.borrow()),
            0_u32,
        )
    });
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"\\(a\\([bc]\\)\\)"),
            13_usize,
            ECMAScript_3.with(|rc| *rc.borrow()),
            0_u32,
        )
    });
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"(a([bc]))"),
            9_usize,
            ECMAScript_3.with(|rc| *rc.borrow()),
            2_u32,
        )
    });
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"\\(a\\)"),
            5_usize,
            awk_4.with(|rc| *rc.borrow()),
            0_u32,
        )
    });
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"\\(a[bc]\\)"),
            9_usize,
            awk_4.with(|rc| *rc.borrow()),
            0_u32,
        )
    });
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"\\(a\\([bc]\\)\\)"),
            13_usize,
            awk_4.with(|rc| *rc.borrow()),
            0_u32,
        )
    });
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"(a([bc]))"),
            9_usize,
            awk_4.with(|rc| *rc.borrow()),
            2_u32,
        )
    });
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"\\(a\\)"),
            5_usize,
            grep_5.with(|rc| *rc.borrow()),
            1_u32,
        )
    });
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"\\(a[bc]\\)"),
            9_usize,
            grep_5.with(|rc| *rc.borrow()),
            1_u32,
        )
    });
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"\\(a\\([bc]\\)\\)"),
            13_usize,
            grep_5.with(|rc| *rc.borrow()),
            2_u32,
        )
    });
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"(a([bc]))"),
            9_usize,
            grep_5.with(|rc| *rc.borrow()),
            0_u32,
        )
    });
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"\\(a\\)"),
            5_usize,
            egrep_6.with(|rc| *rc.borrow()),
            0_u32,
        )
    });
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"\\(a[bc]\\)"),
            9_usize,
            egrep_6.with(|rc| *rc.borrow()),
            0_u32,
        )
    });
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"\\(a\\([bc]\\)\\)"),
            13_usize,
            egrep_6.with(|rc| *rc.borrow()),
            0_u32,
        )
    });
    ({
        test_0(
            Ptr::<i8>::from_string_literal(b"(a([bc]))"),
            9_usize,
            egrep_6.with(|rc| *rc.borrow()),
            2_u32,
        )
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
