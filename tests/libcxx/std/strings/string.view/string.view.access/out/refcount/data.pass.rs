extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(mut s: Ptr<i8>, mut len: u64) {
    let sv: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_1({ (s).clone() }, { len }),
    ));
    ({ cpp2rust_assert_fail_1((({ (*sv.borrow()).length() }) == len)) });
    ({ cpp2rust_assert_fail_1(({ ({ (*sv.borrow()).data() }) } == { (s).clone() })) });
    ({ cpp2rust_assert_fail_1((({ (*sv.borrow()).data() }) == ({ data_2(sv.as_pointer()) }))) });
}
pub fn test_3(mut s: Ptr<i32>, mut len: u64) {
    let sv: Value<std_basic_string_view_wchar_t__std_char_traits_wchar_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_wchar_t__std_char_traits_wchar_t__::new_1({ (s).clone() }, {
                len
            }),
        ));
    ({ cpp2rust_assert_fail_1((({ (*sv.borrow()).length() }) == len)) });
    ({ cpp2rust_assert_fail_1(({ ({ (*sv.borrow()).data() }) } == { (s).clone() })) });
    ({ cpp2rust_assert_fail_1((({ (*sv.borrow()).data() }) == ({ data_4(sv.as_pointer()) }))) });
}
pub fn test_5(mut s: Ptr<u16>, mut len: u64) {
    let sv: Value<std_basic_string_view_char16_t__std_char_traits_char16_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char16_t__std_char_traits_char16_t__::new_1({ (s).clone() }, {
                len
            }),
        ));
    ({ cpp2rust_assert_fail_1((({ (*sv.borrow()).length() }) == len)) });
    ({ cpp2rust_assert_fail_1(({ ({ (*sv.borrow()).data() }) } == { (s).clone() })) });
    ({ cpp2rust_assert_fail_1((({ (*sv.borrow()).data() }) == ({ data_6(sv.as_pointer()) }))) });
}
pub fn test_7(mut s: Ptr<u32>, mut len: u64) {
    let sv: Value<std_basic_string_view_char32_t__std_char_traits_char32_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char32_t__std_char_traits_char32_t__::new_1({ (s).clone() }, {
                len
            }),
        ));
    ({ cpp2rust_assert_fail_1((({ (*sv.borrow()).length() }) == len)) });
    ({ cpp2rust_assert_fail_1(({ ({ (*sv.borrow()).data() }) } == { (s).clone() })) });
    ({ cpp2rust_assert_fail_1((({ (*sv.borrow()).data() }) == ({ data_8(sv.as_pointer()) }))) });
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
    ({
        test_3(
            Ptr::<i32>::from_string_literal(&[
                65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 0 as i32,
            ]),
            5_u64,
        )
    });
    ({
        test_3(
            Ptr::<i32>::from_string_literal(&[97 as i32, 0 as i32]),
            1_u64,
        )
    });
    ({
        test_5(
            Ptr::<u16>::from_string_literal(&[
                65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 0 as u16,
            ]),
            5_u64,
        )
    });
    ({
        test_5(
            Ptr::<u16>::from_string_literal(&[97 as u16, 0 as u16]),
            1_u64,
        )
    });
    ({
        test_7(
            Ptr::<u32>::from_string_literal(&[
                65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 0 as u32,
            ]),
            5_u64,
        )
    });
    ({
        test_7(
            Ptr::<u32>::from_string_literal(&[97 as u32, 0 as u32]),
            1_u64,
        )
    });
    {
        let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"ABC");
        let sv: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
            std_basic_string_view_char__std_char_traits_char__::new_1({ (s).clone() }, { 2_u64 }),
        ));
        const _: () = assert!(true, "sv.length() == 2");;
        const _: () = assert!(true, "sv.data() == s");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
