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
        std_basic_string_view_char__std_char_traits_char__::new_1({ s.offset((1) as isize) }, {
            1_u64
        }),
    ));
    let sv2: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_1({ s.offset((1) as isize) }, {
            2_u64
        }),
    ));
    let sv3: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_1({ s.offset((1) as isize) }, {
            3_u64
        }),
    ));
    let sv4: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_1({ s.offset((1) as isize) }, {
            4_u64
        }),
    ));
    let sv5: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_1({ (s).clone() }, { 5_u64 }),
    ));
    let svNot: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_1(
            { Ptr::<i8>::from_string_literal(b"xyz") },
            { 3_u64 },
        ),
    ));
    let svNot2: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_1(
            { Ptr::<i8>::from_string_literal(b"bd") },
            { 2_u64 },
        ),
    ));
    let svNot3: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_1(
            { Ptr::<i8>::from_string_literal(b"dcb") },
            { 3_u64 },
        ),
    ));
    const _: () = assert!(true, "");;
    ({
        cpp2rust_assert_fail_1(
            ({
                let ___x: std_basic_string_view_char__std_char_traits_char__ =
                    (*sv0.borrow()).clone();
                (*sv0.borrow()).contains_43(___x)
            }),
        )
    });
    ({ cpp2rust_assert_fail_1(!({ (*sv0.borrow()).contains_43((*sv1.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(({ (*sv1.borrow()).contains_43((*sv0.borrow()).clone()) })) });
    ({
        cpp2rust_assert_fail_1(
            ({
                let ___x: std_basic_string_view_char__std_char_traits_char__ =
                    (*sv1.borrow()).clone();
                (*sv1.borrow()).contains_43(___x)
            }),
        )
    });
    ({ cpp2rust_assert_fail_1(!({ (*sv1.borrow()).contains_43((*sv2.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(!({ (*sv1.borrow()).contains_43((*sv3.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(!({ (*sv1.borrow()).contains_43((*sv4.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(!({ (*sv1.borrow()).contains_43((*sv5.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(!({ (*sv1.borrow()).contains_43((*svNot.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(!({ (*sv1.borrow()).contains_43((*svNot2.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(!({ (*sv1.borrow()).contains_43((*svNot3.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(({ (*sv3.borrow()).contains_43((*sv0.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(({ (*sv3.borrow()).contains_43((*sv1.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(({ (*sv3.borrow()).contains_43((*sv2.borrow()).clone()) })) });
    ({
        cpp2rust_assert_fail_1(
            ({
                let ___x: std_basic_string_view_char__std_char_traits_char__ =
                    (*sv3.borrow()).clone();
                (*sv3.borrow()).contains_43(___x)
            }),
        )
    });
    ({ cpp2rust_assert_fail_1(!({ (*sv3.borrow()).contains_43((*sv4.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(!({ (*sv3.borrow()).contains_43((*sv5.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(!({ (*sv3.borrow()).contains_43((*svNot.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(!({ (*sv3.borrow()).contains_43((*svNot2.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(!({ (*sv3.borrow()).contains_43((*svNot3.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(({ (*sv5.borrow()).contains_43((*sv0.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(({ (*sv5.borrow()).contains_43((*sv1.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(({ (*sv5.borrow()).contains_43((*sv2.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(({ (*sv5.borrow()).contains_43((*sv3.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(({ (*sv5.borrow()).contains_43((*sv4.borrow()).clone()) })) });
    ({
        cpp2rust_assert_fail_1(
            ({
                let ___x: std_basic_string_view_char__std_char_traits_char__ =
                    (*sv5.borrow()).clone();
                (*sv5.borrow()).contains_43(___x)
            }),
        )
    });
    ({ cpp2rust_assert_fail_1(!({ (*sv5.borrow()).contains_43((*svNot.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(!({ (*sv5.borrow()).contains_43((*svNot2.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(!({ (*sv5.borrow()).contains_43((*svNot3.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(({ (*svNot.borrow()).contains_43((*sv0.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(!({ (*svNot.borrow()).contains_43((*sv1.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(!({ (*svNot.borrow()).contains_43((*sv2.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(!({ (*svNot.borrow()).contains_43((*sv3.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(!({ (*svNot.borrow()).contains_43((*sv4.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(!({ (*svNot.borrow()).contains_43((*sv5.borrow()).clone()) })) });
    ({
        cpp2rust_assert_fail_1(
            ({
                let ___x: std_basic_string_view_char__std_char_traits_char__ =
                    (*svNot.borrow()).clone();
                (*svNot.borrow()).contains_43(___x)
            }),
        )
    });
    ({ cpp2rust_assert_fail_1(!({ (*svNot.borrow()).contains_43((*svNot2.borrow()).clone()) })) });
    ({ cpp2rust_assert_fail_1(!({ (*svNot.borrow()).contains_43((*svNot3.borrow()).clone()) })) });
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
