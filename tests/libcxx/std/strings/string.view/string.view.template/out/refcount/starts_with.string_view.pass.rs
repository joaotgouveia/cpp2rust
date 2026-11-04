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
    pub fn lt(mut c1: i8, mut c2: i8) -> bool {
        return ((c1 as i32) < (c2 as i32));
    }
    pub fn compare(mut s1: Ptr<i8>, mut s2: Ptr<i8>, mut n: u64) -> i32 {
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
}
impl CppMove for constexpr_char_traits_char_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
        let sv0: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
            <std_basic_string_view_char__std_char_traits_char__>::default(),
        ));
        let sv1: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
            std_basic_string_view_char__std_char_traits_char__::new_1({ (s).clone() }, { 1_u64 }),
        ));
        let sv2: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
            std_basic_string_view_char__std_char_traits_char__::new_1({ (s).clone() }, { 2_u64 }),
        ));
        let sv3: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
            std_basic_string_view_char__std_char_traits_char__::new_1({ (s).clone() }, { 3_u64 }),
        ));
        let sv4: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
            std_basic_string_view_char__std_char_traits_char__::new_1({ (s).clone() }, { 4_u64 }),
        ));
        let sv5: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
            std_basic_string_view_char__std_char_traits_char__::new_1({ (s).clone() }, { 5_u64 }),
        ));
        let svNot: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
            RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"def") },
                { 3_u64 },
            )),
        );
        const _: () = assert!(true, "");;
        ({
            cpp2rust_assert_fail_0(
                ({
                    let ___x: std_basic_string_view_char__std_char_traits_char__ =
                        (*sv0.borrow()).clone();
                    (*sv0.borrow()).starts_with_37(___x)
                }),
            )
        });
        ({
            cpp2rust_assert_fail_0(!({ (*sv0.borrow()).starts_with_37((*sv1.borrow()).clone()) }))
        });
        ({ cpp2rust_assert_fail_0(({ (*sv1.borrow()).starts_with_37((*sv0.borrow()).clone()) })) });
        ({
            cpp2rust_assert_fail_0(
                ({
                    let ___x: std_basic_string_view_char__std_char_traits_char__ =
                        (*sv1.borrow()).clone();
                    (*sv1.borrow()).starts_with_37(___x)
                }),
            )
        });
        ({
            cpp2rust_assert_fail_0(!({ (*sv1.borrow()).starts_with_37((*sv2.borrow()).clone()) }))
        });
        ({
            cpp2rust_assert_fail_0(!({ (*sv1.borrow()).starts_with_37((*sv3.borrow()).clone()) }))
        });
        ({
            cpp2rust_assert_fail_0(!({ (*sv1.borrow()).starts_with_37((*sv4.borrow()).clone()) }))
        });
        ({
            cpp2rust_assert_fail_0(!({ (*sv1.borrow()).starts_with_37((*sv5.borrow()).clone()) }))
        });
        ({
            cpp2rust_assert_fail_0(!({ (*sv1.borrow()).starts_with_37((*svNot.borrow()).clone()) }))
        });
        ({ cpp2rust_assert_fail_0(({ (*sv2.borrow()).starts_with_37((*sv0.borrow()).clone()) })) });
        ({ cpp2rust_assert_fail_0(({ (*sv2.borrow()).starts_with_37((*sv1.borrow()).clone()) })) });
        ({
            cpp2rust_assert_fail_0(
                ({
                    let ___x: std_basic_string_view_char__std_char_traits_char__ =
                        (*sv2.borrow()).clone();
                    (*sv2.borrow()).starts_with_37(___x)
                }),
            )
        });
        ({
            cpp2rust_assert_fail_0(!({ (*sv2.borrow()).starts_with_37((*sv3.borrow()).clone()) }))
        });
        ({
            cpp2rust_assert_fail_0(!({ (*sv2.borrow()).starts_with_37((*sv4.borrow()).clone()) }))
        });
        ({
            cpp2rust_assert_fail_0(!({ (*sv2.borrow()).starts_with_37((*sv5.borrow()).clone()) }))
        });
        ({
            cpp2rust_assert_fail_0(!({ (*sv2.borrow()).starts_with_37((*svNot.borrow()).clone()) }))
        });
        ({
            cpp2rust_assert_fail_0(({ (*svNot.borrow()).starts_with_37((*sv0.borrow()).clone()) }))
        });
        ({
            cpp2rust_assert_fail_0(!({ (*svNot.borrow()).starts_with_37((*sv1.borrow()).clone()) }))
        });
        ({
            cpp2rust_assert_fail_0(!({ (*svNot.borrow()).starts_with_37((*sv2.borrow()).clone()) }))
        });
        ({
            cpp2rust_assert_fail_0(!({ (*svNot.borrow()).starts_with_37((*sv3.borrow()).clone()) }))
        });
        ({
            cpp2rust_assert_fail_0(!({ (*svNot.borrow()).starts_with_37((*sv4.borrow()).clone()) }))
        });
        ({
            cpp2rust_assert_fail_0(!({ (*svNot.borrow()).starts_with_37((*sv5.borrow()).clone()) }))
        });
        ({
            cpp2rust_assert_fail_0(
                ({
                    let ___x: std_basic_string_view_char__std_char_traits_char__ =
                        (*svNot.borrow()).clone();
                    (*svNot.borrow()).starts_with_37(___x)
                }),
            )
        });
    }
    {
        let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"abcde");
        let sv0: Value<std_basic_string_view_char__constexpr_char_traits_char__> = Rc::new(
            RefCell::new(<std_basic_string_view_char__constexpr_char_traits_char__>::default()),
        );
        let sv1: Value<std_basic_string_view_char__constexpr_char_traits_char__> =
            Rc::new(RefCell::new(
                std_basic_string_view_char__constexpr_char_traits_char__::new_1({ (s).clone() }, {
                    1_u64
                }),
            ));
        let sv2: Value<std_basic_string_view_char__constexpr_char_traits_char__> =
            Rc::new(RefCell::new(
                std_basic_string_view_char__constexpr_char_traits_char__::new_1({ (s).clone() }, {
                    2_u64
                }),
            ));
        let sv3: Value<std_basic_string_view_char__constexpr_char_traits_char__> =
            Rc::new(RefCell::new(
                std_basic_string_view_char__constexpr_char_traits_char__::new_1({ (s).clone() }, {
                    3_u64
                }),
            ));
        let sv4: Value<std_basic_string_view_char__constexpr_char_traits_char__> =
            Rc::new(RefCell::new(
                std_basic_string_view_char__constexpr_char_traits_char__::new_1({ (s).clone() }, {
                    4_u64
                }),
            ));
        let sv5: Value<std_basic_string_view_char__constexpr_char_traits_char__> =
            Rc::new(RefCell::new(
                std_basic_string_view_char__constexpr_char_traits_char__::new_1({ (s).clone() }, {
                    5_u64
                }),
            ));
        let svNot: Value<std_basic_string_view_char__constexpr_char_traits_char__> =
            Rc::new(RefCell::new(
                std_basic_string_view_char__constexpr_char_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"def") },
                    { 3_u64 },
                ),
            ));
        const _: () = assert!(true, "sv0.starts_with(sv0)");;
        const _: () = assert!(true, "!sv0.starts_with(sv1)");;
        const _: () = assert!(true, "sv1.starts_with(sv0)");;
        const _: () = assert!(true, "sv1.starts_with(sv1)");;
        const _: () = assert!(true, "!sv1.starts_with(sv2)");;
        const _: () = assert!(true, "!sv1.starts_with(sv3)");;
        const _: () = assert!(true, "!sv1.starts_with(sv4)");;
        const _: () = assert!(true, "!sv1.starts_with(sv5)");;
        const _: () = assert!(true, "!sv1.starts_with(svNot)");;
        const _: () = assert!(true, "sv2.starts_with(sv0)");;
        const _: () = assert!(true, "sv2.starts_with(sv1)");;
        const _: () = assert!(true, "sv2.starts_with(sv2)");;
        const _: () = assert!(true, "!sv2.starts_with(sv3)");;
        const _: () = assert!(true, "!sv2.starts_with(sv4)");;
        const _: () = assert!(true, "!sv2.starts_with(sv5)");;
        const _: () = assert!(true, "!sv2.starts_with(svNot)");;
        const _: () = assert!(true, "svNot.starts_with(sv0)");;
        const _: () = assert!(true, "!svNot.starts_with(sv1)");;
        const _: () = assert!(true, "!svNot.starts_with(sv2)");;
        const _: () = assert!(true, "!svNot.starts_with(sv3)");;
        const _: () = assert!(true, "!svNot.starts_with(sv4)");;
        const _: () = assert!(true, "!svNot.starts_with(sv5)");;
        const _: () = assert!(true, "svNot.starts_with(svNot)");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
