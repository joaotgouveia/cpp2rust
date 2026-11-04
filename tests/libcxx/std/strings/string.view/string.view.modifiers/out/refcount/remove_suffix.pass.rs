extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(mut s: Ptr<i8>, mut len: u64) {
    {
        let sv1: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
            std_basic_string_view_char__std_char_traits_char__::new_1({ (s).clone() }),
        ));
        ({ cpp2rust_assert_fail_1((({ (*sv1.borrow()).size() }) == len)) });
        ({ cpp2rust_assert_fail_1(({ ({ (*sv1.borrow()).data() }) } == { (s).clone() })) });
        if (len > 0_u64) {
            ({ (*sv1.borrow()).remove_suffix(1_u64) });
            ({
                cpp2rust_assert_fail_1(
                    (({ (*sv1.borrow()).size() }) == ((len).wrapping_sub(1_u64))),
                )
            });
            ({ cpp2rust_assert_fail_1(({ ({ (*sv1.borrow()).data() }) } == { (s).clone() })) });
            ({ (*sv1.borrow()).remove_suffix((len).wrapping_sub(1_u64)) });
        }
        ({ cpp2rust_assert_fail_1((({ (*sv1.borrow()).size() }) == 0_u64)) });
        ({ (*sv1.borrow()).remove_suffix(0_u64) });
        ({ cpp2rust_assert_fail_1((({ (*sv1.borrow()).size() }) == 0_u64)) });
    }
}
pub fn test_2(mut s: Ptr<i32>, mut len: u64) {
    {
        let sv1: Value<std_basic_string_view_wchar_t__std_char_traits_wchar_t__> =
            Rc::new(RefCell::new(
                std_basic_string_view_wchar_t__std_char_traits_wchar_t__::new_1({ (s).clone() }),
            ));
        ({ cpp2rust_assert_fail_1((({ (*sv1.borrow()).size() }) == len)) });
        ({ cpp2rust_assert_fail_1(({ ({ (*sv1.borrow()).data() }) } == { (s).clone() })) });
        if (len > 0_u64) {
            ({ (*sv1.borrow()).remove_suffix(1_u64) });
            ({
                cpp2rust_assert_fail_1(
                    (({ (*sv1.borrow()).size() }) == ((len).wrapping_sub(1_u64))),
                )
            });
            ({ cpp2rust_assert_fail_1(({ ({ (*sv1.borrow()).data() }) } == { (s).clone() })) });
            ({ (*sv1.borrow()).remove_suffix((len).wrapping_sub(1_u64)) });
        }
        ({ cpp2rust_assert_fail_1((({ (*sv1.borrow()).size() }) == 0_u64)) });
        ({ (*sv1.borrow()).remove_suffix(0_u64) });
        ({ cpp2rust_assert_fail_1((({ (*sv1.borrow()).size() }) == 0_u64)) });
    }
}
pub fn test_3(mut s: Ptr<u16>, mut len: u64) {
    {
        let sv1: Value<std_basic_string_view_char16_t__std_char_traits_char16_t__> =
            Rc::new(RefCell::new(
                std_basic_string_view_char16_t__std_char_traits_char16_t__::new_1({ (s).clone() }),
            ));
        ({ cpp2rust_assert_fail_1((({ (*sv1.borrow()).size() }) == len)) });
        ({ cpp2rust_assert_fail_1(({ ({ (*sv1.borrow()).data() }) } == { (s).clone() })) });
        if (len > 0_u64) {
            ({ (*sv1.borrow()).remove_suffix(1_u64) });
            ({
                cpp2rust_assert_fail_1(
                    (({ (*sv1.borrow()).size() }) == ((len).wrapping_sub(1_u64))),
                )
            });
            ({ cpp2rust_assert_fail_1(({ ({ (*sv1.borrow()).data() }) } == { (s).clone() })) });
            ({ (*sv1.borrow()).remove_suffix((len).wrapping_sub(1_u64)) });
        }
        ({ cpp2rust_assert_fail_1((({ (*sv1.borrow()).size() }) == 0_u64)) });
        ({ (*sv1.borrow()).remove_suffix(0_u64) });
        ({ cpp2rust_assert_fail_1((({ (*sv1.borrow()).size() }) == 0_u64)) });
    }
}
pub fn test_4(mut s: Ptr<u32>, mut len: u64) {
    {
        let sv1: Value<std_basic_string_view_char32_t__std_char_traits_char32_t__> =
            Rc::new(RefCell::new(
                std_basic_string_view_char32_t__std_char_traits_char32_t__::new_1({ (s).clone() }),
            ));
        ({ cpp2rust_assert_fail_1((({ (*sv1.borrow()).size() }) == len)) });
        ({ cpp2rust_assert_fail_1(({ ({ (*sv1.borrow()).data() }) } == { (s).clone() })) });
        if (len > 0_u64) {
            ({ (*sv1.borrow()).remove_suffix(1_u64) });
            ({
                cpp2rust_assert_fail_1(
                    (({ (*sv1.borrow()).size() }) == ((len).wrapping_sub(1_u64))),
                )
            });
            ({ cpp2rust_assert_fail_1(({ ({ (*sv1.borrow()).data() }) } == { (s).clone() })) });
            ({ (*sv1.borrow()).remove_suffix((len).wrapping_sub(1_u64)) });
        }
        ({ cpp2rust_assert_fail_1((({ (*sv1.borrow()).size() }) == 0_u64)) });
        ({ (*sv1.borrow()).remove_suffix(0_u64) });
        ({ cpp2rust_assert_fail_1((({ (*sv1.borrow()).size() }) == 0_u64)) });
    }
}
pub fn test_ce_5(mut n: u64, mut k: u64) -> u64 {
    let sv1: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_2(
            { Ptr::<i8>::from_string_literal(b"ABCDEFGHIJKL") },
            { n },
        ),
    ));
    ({ (*sv1.borrow()).remove_suffix(k) });
    return ({ (*sv1.borrow()).size() });
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
    ({ test_0(Ptr::<i8>::from_string_literal(b"ABCDE"), 5_u64) });
    ({ test_0(Ptr::<i8>::from_string_literal(b"a"), 1_u64) });
    ({ test_0(Ptr::<i8>::from_string_literal(b""), 0_u64) });
    ({
        test_2(
            Ptr::<i32>::from_string_literal(&[
                65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 0 as i32,
            ]),
            5_u64,
        )
    });
    ({
        test_2(
            Ptr::<i32>::from_string_literal(&[97 as i32, 0 as i32]),
            1_u64,
        )
    });
    ({ test_2(Ptr::<i32>::from_string_literal(&[0 as i32]), 0_u64) });
    ({
        test_3(
            Ptr::<u16>::from_string_literal(&[
                65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 0 as u16,
            ]),
            5_u64,
        )
    });
    ({
        test_3(
            Ptr::<u16>::from_string_literal(&[97 as u16, 0 as u16]),
            1_u64,
        )
    });
    ({ test_3(Ptr::<u16>::from_string_literal(&[0 as u16]), 0_u64) });
    ({
        test_4(
            Ptr::<u32>::from_string_literal(&[
                65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 0 as u32,
            ]),
            5_u64,
        )
    });
    ({
        test_4(
            Ptr::<u32>::from_string_literal(&[97 as u32, 0 as u32]),
            1_u64,
        )
    });
    ({ test_4(Ptr::<u32>::from_string_literal(&[0 as u32]), 0_u64) });
    {
        const _: () = assert!(true, "test_ce(5, 0) == 5");;
        const _: () = assert!(true, "test_ce(5, 1) == 4");;
        const _: () = assert!(true, "test_ce(5, 5) == 0");;
        const _: () = assert!(true, "test_ce(9, 3) == 6");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
