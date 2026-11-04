extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"abcde");
    let sv0: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        <std_basic_string_view_char__std_char_traits_char__>::default(),
    ));
    let sv1: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_1({ s.offset((4) as isize) }, {
            1_u64
        }),
    ));
    let sv3: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_1({ s.offset((2) as isize) }, {
            3_u64
        }),
    ));
    let svNot: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_1(
            { Ptr::<i8>::from_string_literal(b"xyz") },
            { 3_u64 },
        ),
    ));
    ({
        cpp2rust_assert_fail_1(
            ({ (*sv0.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            !({ (*sv0.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"e")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ (*sv1.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            !({ (*sv1.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"d")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ (*sv1.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"e")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            !({ (*sv1.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"de")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            !({ (*sv1.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"cd")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            !({ (*sv1.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"cde")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            !({ (*sv1.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"bcde")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            !({ (*sv1.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"abcde")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            !({ (*sv1.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"xyz")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ (*sv3.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ (*sv3.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"d")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ (*sv3.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"e")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ (*sv3.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"de")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ (*sv3.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"cd")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            !({ (*sv3.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"ce")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ (*sv3.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"cde")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            !({ (*sv3.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"edc")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            !({ (*sv3.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"bcde")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            !({ (*sv3.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"abcde")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            !({ (*sv3.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"xyz")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ (*svNot.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            !({ (*svNot.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"d")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            !({ (*svNot.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"e")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            !({ (*svNot.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"de")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            !({ (*svNot.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"cd")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            !({ (*svNot.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"cde")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            !({ (*svNot.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"bcde")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            !({ (*svNot.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"abcde")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ (*svNot.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"xyz")) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            !({ (*svNot.borrow()).contains_45(Ptr::<i8>::from_string_literal(b"zyx")) }),
        )
    });
    return true;
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
